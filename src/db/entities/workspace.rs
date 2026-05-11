use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workspaces")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::organization::Entity", from = "Column::OrgId", to = "super::organization::Column::Id")]
    Organization,
    #[sea_orm(has_many = "super::workspace_member::Entity")]
    Members,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef { Relation::Organization.def() }
}
impl Related<super::workspace_member::Entity> for Entity {
    fn to() -> RelationDef { Relation::Members.def() }
}

impl ActiveModelBehavior for ActiveModel {}
