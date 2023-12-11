use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "debugger_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub debugger: String,
    pub version: String,
    pub workspace_root: String,
    pub package_name: String,
    pub target: String,
    /// Json
    pub arguments: String,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Copy, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
