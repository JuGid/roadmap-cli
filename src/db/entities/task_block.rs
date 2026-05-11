use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "task_blocks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub blocker_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub blocked_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
