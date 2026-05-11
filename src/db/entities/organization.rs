use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "organizations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(unique)]
    pub slug: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::org_member::Entity")]
    Members,
    #[sea_orm(has_many = "super::workspace::Entity")]
    Workspaces,
    #[sea_orm(has_many = "super::team::Entity")]
    Teams,
    #[sea_orm(has_many = "super::project::Entity")]
    Projects,
}

impl Related<super::org_member::Entity> for Entity {
    fn to() -> RelationDef { Relation::Members.def() }
}
impl Related<super::workspace::Entity> for Entity {
    fn to() -> RelationDef { Relation::Workspaces.def() }
}
impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef { Relation::Teams.def() }
}
impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef { Relation::Projects.def() }
}

impl ActiveModelBehavior for ActiveModel {}
