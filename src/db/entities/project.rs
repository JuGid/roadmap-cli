use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub owner_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub workspace_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity", from = "Column::OwnerId", to = "super::user::Column::Id")]
    Owner,
    #[sea_orm(belongs_to = "super::organization::Entity", from = "Column::OrgId", to = "super::organization::Column::Id")]
    Organization,
    #[sea_orm(belongs_to = "super::workspace::Entity", from = "Column::WorkspaceId", to = "super::workspace::Column::Id")]
    Workspace,
    #[sea_orm(belongs_to = "super::team::Entity", from = "Column::TeamId", to = "super::team::Column::Id")]
    Team,
    #[sea_orm(has_many = "super::phase::Entity")]
    Phases,
    #[sea_orm(has_many = "super::tag::Entity")]
    Tags,
    #[sea_orm(has_many = "super::activity_log::Entity")]
    ActivityLogs,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef { Relation::Owner.def() }
}
impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef { Relation::Organization.def() }
}
impl Related<super::phase::Entity> for Entity {
    fn to() -> RelationDef { Relation::Phases.def() }
}
impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef { Relation::Tags.def() }
}

impl ActiveModelBehavior for ActiveModel {}
