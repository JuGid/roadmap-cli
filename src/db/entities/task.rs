use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub phase_id: Uuid,
    pub task_id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub optional: bool,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<Date>,
    pub parent_task_id: Option<Uuid>,
    pub workflow_stage: Option<String>,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::phase::Entity", from = "Column::PhaseId", to = "super::phase::Column::Id")]
    Phase,
    #[sea_orm(belongs_to = "super::user::Entity", from = "Column::AssigneeId", to = "super::user::Column::Id")]
    Assignee,
}

impl Related<super::phase::Entity> for Entity {
    fn to() -> RelationDef { Relation::Phase.def() }
}
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef { Relation::Assignee.def() }
}

impl ActiveModelBehavior for ActiveModel {}
