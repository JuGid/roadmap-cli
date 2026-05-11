//! Database module — SeaORM

pub mod entities;
pub mod migration;
pub mod repos;

use sea_orm::{Database, DatabaseConnection};

pub async fn create_pool(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect(database_url).await
}
