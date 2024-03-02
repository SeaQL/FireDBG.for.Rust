use std::collections::{HashMap, HashSet};

use sea_orm::{
    sea_query::{Expr, Query},
    ActiveModelTrait, ConnectOptions, ConnectionTrait, Database as SeaDatabase, DbConn, DbErr,
    EntityTrait, Schema,
};
use sea_streamer::file::AsyncFile;

use crate::entity::{
    allocation::{self, Entity as Allocation},
    breakpoint::{self, Entity as Breakpoint},
    debugger_info::{self, Entity as DebuggerInfo},
    event::{self, Entity as Event},
    file::{self, Entity as File},
    function::{self, Entity as Function},
    type_info::{self, Entity as TypeInfo},
};

#[derive(Debug)]
pub struct Database {
    path: String,
    db: Option<DbConn>,
}

impl Database {
    pub async fn create(path: String) -> Result<Self, DbErr> {
        AsyncFile::new_ow(path.parse().expect("UrlErr")) // overwrite the file
            .await
            .expect("File System Error");
        let mut opt = ConnectOptions::new(format!("sqlite://{path}?mode=rw"));
        opt.max_connections(1).sqlx_logging(false);
        let db = SeaDatabase::connect(opt).await?;
        create_tables(&db).await?;
        Ok(Self { path, db: Some(db) })
    }

    pub async fn reopen(&mut self) -> Result<(), DbErr> {
        // Close existing db, if any
        if let Some(db) = self.db.take() {
            db.close().await?; // drop it
        }
        let mut opt = ConnectOptions::new(format!("sqlite://{}", self.path));
        opt.max_connections(1).sqlx_logging(false);
        let db = SeaDatabase::connect(opt).await?;
        self.db = Some(db);
        Ok(())
    }

    pub fn db(&self) -> &DbConn {
        self.db.as_ref().expect("DB closed")
    }

    pub async fn close(&mut self) -> Result<(), DbErr> {
        // Close existing db, if any
        if let Some(db) = self.db.take() {
            db.close().await?; // drop it
        }
        Ok(())
    }
}

pub async fn create_tables(db: &DbConn) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let stmt = builder.build(&schema.create_table_from_entity(DebuggerInfo));
    log::debug!("{stmt}");
    db.execute(stmt).await?;

    let stmt = builder.build(&schema.create_table_from_entity(File));
    log::debug!("{stmt}");
    db.execute(stmt).await?;

    let stmt = builder.build(&schema.create_table_from_entity(Breakpoint));
    log::debug!("{stmt}");
    db.execute(stmt).await?;

    let stmt = builder.build(&schema.create_table_from_entity(Event));
    log::debug!("{stmt}");
    db.execute(stmt).await?;
    for stmt in schema.create_index_from_entity(Event) {
        let stmt = builder.build(&stmt);
        log::debug!("{stmt}");
        db.execute(stmt).await?;
    }

    let stmt = builder.build(&schema.create_table_from_entity(Function));
    log::debug!("{stmt}");
    db.execute(stmt).await?;

    let stmt = builder.build(&schema.create_table_from_entity(TypeInfo));
    log::debug!("{stmt}");
    db.execute(stmt).await?;

    let stmt = builder.build(&schema.create_table_from_entity(Allocation));
    log::debug!("{stmt}");
    db.execute(stmt).await?;

    Ok(())
}

pub async fn save_debugger_info(
    db: &Database,
    info: debugger_info::ActiveModel,
) -> Result<(), DbErr> {
    let res = info.save(db.db()).await?;
    log::debug!("DebuggerInfo::save: {:?}", res);
    Ok(())
}

pub async fn insert_files(
    db: &Database,
    files: impl Iterator<Item = file::ActiveModel>,
) -> Result<(), DbErr> {
    let res = File::insert_many(files)
        .on_empty_do_nothing()
        .exec(db.db())
        .await?;
    log::debug!("File::insert_many: {:?}", res);
    Ok(())
}

pub async fn insert_breakpoints(
    db: &Database,
    breakpoints: impl Iterator<Item = breakpoint::ActiveModel>,
) -> Result<(), DbErr> {
    let res = Breakpoint::insert_many(breakpoints)
        .on_empty_do_nothing()
        .exec(db.db())
        .await?;
    log::debug!("Breakpoint::insert_many: {:?}", res);
    Ok(())
}

pub async fn insert_events(
    db: &Database,
    events: impl Iterator<Item = event::ActiveModel>,
) -> Result<(), DbErr> {
    let db = db.db();
    let mut hits: HashMap<u32, usize> = Default::default();
    let mut functions: HashSet<String> = Default::default();
    let events: Vec<_> = events.collect();
    for event in events.iter() {
        if let Some(function_name) = event.function_name.as_ref() {
            functions.insert(function_name.clone());
        }
        let bp_id = event.breakpoint_id.as_ref();
        let hit = hits.entry(*bp_id).or_default();
        *hit += 1;
    }

    let start = std::time::Instant::now();
    let res = Event::insert_many(events)
        .on_empty_do_nothing()
        .exec(db)
        .await?;
    let duration = start.elapsed();
    log::debug!("Event::insert_many: {:?} in {:?}", res, duration);

    inc_hit_count(db, hits).await?;
    insert_functions(db, functions).await?;

    Ok(())
}

async fn inc_hit_count(db: &DbConn, breakpoints: HashMap<u32, usize>) -> Result<(), DbErr> {
    // we batch the updates for all the breakpoints hit by N
    let mut groups: HashMap<usize, Vec<u32>> = Default::default();
    for (bp_id, count) in breakpoints {
        let entry = groups.entry(count).or_default();
        entry.push(bp_id);
    }

    use breakpoint::Column::HitCount;

    for (count, bp_ids) in groups {
        let mut update = Query::update();
        update
            .table(Breakpoint)
            .value(HitCount, Expr::col(HitCount).add(count as u32))
            .and_where(Expr::col(breakpoint::Column::Id).is_in(bp_ids));

        let stmt = db.get_database_backend().build(&update);
        log::debug!("{stmt}");
        db.execute(stmt).await?;
    }

    Ok(())
}

async fn insert_functions(db: &DbConn, functions: HashSet<String>) -> Result<(), DbErr> {
    use sea_orm::{sea_query::OnConflict, Set};

    Function::insert_many(functions.into_iter().map(|n| function::ActiveModel {
        function_name: Set(n),
    }))
    .on_conflict(OnConflict::new().do_nothing().to_owned())
    .do_nothing()
    .exec(db)
    .await?;

    Ok(())
}

pub async fn insert_type_info(
    db: &Database,
    type_info: impl Iterator<Item = type_info::Model>,
) -> Result<(), DbErr> {
    use sea_orm::{sea_query::OnConflict, IntoActiveModel};

    TypeInfo::insert_many(type_info.map(|m| m.into_active_model()))
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .do_nothing()
        .exec(db.db())
        .await?;

    Ok(())
}

pub async fn insert_allocations(
    db: &Database,
    allocations: impl Iterator<Item = allocation::ActiveModel>,
) -> Result<(), DbErr> {
    let res = Allocation::insert_many(allocations)
        .on_empty_do_nothing()
        .exec(db.db())
        .await?;
    log::debug!("Allocation::insert_many: {:?}", res);
    Ok(())
}
