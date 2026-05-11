use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "notes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub phase_id: Uuid,
    pub content: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::phase::Entity", from = "Column::PhaseId", to = "super::phase::Column::Id")]
    Phase,
}

impl Related<super::phase::Entity> for Entity {
    fn to() -> RelationDef { Relation::Phase.def() }
}

impl ActiveModelBehavior for ActiveModel {}
