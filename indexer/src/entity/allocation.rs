use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "allocation")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Integer")]
    pub id: i64,
    pub action: String,
    /// Sadly SQLite does not support u64
    pub address: i64,
    pub type_name: String,
}

#[derive(Debug, Copy, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
