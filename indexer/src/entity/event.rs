use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Integer")]
    pub id: i64,
    pub breakpoint_id: u32,
    /// Sadly SQLite does not support u64
    pub thread_id: i64,
    #[sea_orm(indexed)]
    pub frame_id: i64,
    pub parent_frame_id: Option<i64>,
    pub stack_pointer: Option<i64>,
    pub function_name: Option<String>,
    pub event_type: EventType,
    pub timestamp: TimeDateTimeWithTimeZone,
    /// Json containing `locals`, `arguments`, or `return_value` depending on event type
    pub data: String,
    /// A pretty printed version of data
    pub pretty: String,
    /// If any local, argument or return value is of `Err` type
    pub is_error: bool,
}

#[derive(Debug, Copy, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::breakpoint::Entity",
        from = "Column::BreakpointId",
        to = "super::breakpoint::Column::Id"
    )]
    Breakpoint,
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Breakpoint.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, DeriveDisplay)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(Some(1))",
    enum_name = "event_type"
)]
pub enum EventType {
    #[sea_orm(string_value = "B")]
    Breakpoint,
    #[sea_orm(string_value = "P")]
    Panic,
    #[sea_orm(string_value = "F")]
    FunctionCall,
    #[sea_orm(string_value = "R")]
    FunctionReturn,
    #[sea_orm(string_value = "AF")]
    FutureEnter,
    #[sea_orm(string_value = "AR")]
    FutureExit,
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
