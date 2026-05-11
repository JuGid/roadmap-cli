use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "phases")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub project_id: Uuid,
    pub phase_id: String,
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub status: String,
    pub parent_phase_id: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::project::Entity", from = "Column::ProjectId", to = "super::project::Column::Id")]
    Project,
    #[sea_orm(has_many = "super::task::Entity")]
    Tasks,
    #[sea_orm(has_many = "super::note::Entity")]
    Notes,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef { Relation::Project.def() }
}
impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef { Relation::Tasks.def() }
}
impl Related<super::note::Entity> for Entity {
    fn to() -> RelationDef { Relation::Notes.def() }
}

impl ActiveModelBehavior for ActiveModel {}
