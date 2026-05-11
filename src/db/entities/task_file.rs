use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "task_files")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub task_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub file_path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::task::Entity", from = "Column::TaskId", to = "super::task::Column::Id")]
    Task,
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef { Relation::Task.def() }
}

impl ActiveModelBehavior for ActiveModel {}
