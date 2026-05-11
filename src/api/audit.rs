//! Audit log API endpoint

use axum::{extract::{Extension, Path, Query}, http::StatusCode, response::{IntoResponse, Json}};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuditParams { pub limit: Option<u64> }

pub async fn get_activity(Extension(db): Extension<DatabaseConnection>, Path(slug): Path<String>, Query(params): Query<AuditParams>) -> impl IntoResponse {
    let project = match crate::db::repos::get_project_by_slug(&db, None, &slug).await {
        Ok(Some(p)) => p,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Projet non trouvé"}))).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    match crate::db::repos::get_activity(&db, project.id, params.limit.unwrap_or(50)).await {
        Ok(logs) => Json(serde_json::to_value(&logs).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
