//! Phases API endpoints

use axum::{extract::{Extension, Path, Query}, http::StatusCode, response::{IntoResponse, Json}};
use sea_orm::*;
use serde::Deserialize;
use crate::db::entities::phase;

#[derive(Deserialize)]
pub struct PhaseParams { pub status: Option<String> }

#[derive(Deserialize)]
pub struct CreatePhaseRequest { pub phase_id: String, pub name: String, pub description: Option<String>, pub priority: Option<i32> }

#[derive(Deserialize)]
pub struct UpdatePhaseRequest { pub name: Option<String>, pub description: Option<String>, pub priority: Option<i32>, pub status: Option<String> }

async fn resolve_project(db: &DatabaseConnection, slug: &str) -> Result<uuid::Uuid, (StatusCode, Json<serde_json::Value>)> {
    match crate::db::repos::get_project_by_slug(db, None, slug).await {
        Ok(Some(p)) => Ok(p.id),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Projet non trouvé"})))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))),
    }
}

pub async fn list_phases(Extension(db): Extension<DatabaseConnection>, Path(slug): Path<String>, Query(params): Query<PhaseParams>) -> impl IntoResponse {
    let project_id = match resolve_project(&db, &slug).await { Ok(id) => id, Err(e) => return e.into_response() };
    let mut query = phase::Entity::find().filter(phase::Column::ProjectId.eq(project_id));
    if let Some(status) = params.status { query = query.filter(phase::Column::Status.eq(status)); }
    match query.order_by_asc(phase::Column::Priority).all(&db).await {
        Ok(p) => Json(serde_json::to_value(&p).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn create_phase(Extension(db): Extension<DatabaseConnection>, Path(slug): Path<String>, Json(body): Json<CreatePhaseRequest>) -> impl IntoResponse {
    let project_id = match resolve_project(&db, &slug).await { Ok(id) => id, Err(e) => return e.into_response() };
    match crate::db::repos::create_phase(&db, project_id, &body.phase_id, &body.name, body.description.as_deref().unwrap_or(""), body.priority.unwrap_or(10)).await {
        Ok(p) => (StatusCode::CREATED, Json(serde_json::to_value(&p).unwrap())).into_response(),
        Err(e) => (StatusCode::CONFLICT, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn get_phase(Extension(db): Extension<DatabaseConnection>, Path((slug, phase_id)): Path<(String, String)>) -> impl IntoResponse {
    let project_id = match resolve_project(&db, &slug).await { Ok(id) => id, Err(e) => return e.into_response() };
    match crate::db::repos::get_phase(&db, project_id, &phase_id).await {
        Ok(Some(p)) => Json(serde_json::to_value(&p).unwrap()).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Phase non trouvée"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn update_phase(Extension(db): Extension<DatabaseConnection>, Path((slug, phase_id)): Path<(String, String)>, Json(body): Json<UpdatePhaseRequest>) -> impl IntoResponse {
    let project_id = match resolve_project(&db, &slug).await { Ok(id) => id, Err(e) => return e.into_response() };
    let p = match crate::db::repos::get_phase(&db, project_id, &phase_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Phase non trouvée"}))).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    let mut active: phase::ActiveModel = p.into();
    if let Some(name) = body.name { active.name = Set(name); }
    if let Some(desc) = body.description { active.description = Set(desc); }
    if let Some(prio) = body.priority { active.priority = Set(prio); }
    if let Some(status) = body.status { active.status = Set(status); }
    active.updated_at = Set(chrono::Utc::now().into());
    match active.update(&db).await {
        Ok(u) => Json(serde_json::to_value(&u).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn delete_phase(Extension(db): Extension<DatabaseConnection>, Path((slug, phase_id)): Path<(String, String)>) -> impl IntoResponse {
    let project_id = match resolve_project(&db, &slug).await { Ok(id) => id, Err(e) => return e.into_response() };
    let result = phase::Entity::delete_many()
        .filter(phase::Column::ProjectId.eq(project_id))
        .filter(phase::Column::PhaseId.eq(&phase_id))
        .exec(&db).await;
    match result {
        Ok(r) if r.rows_affected > 0 => Json(serde_json::json!({"ok": true})).into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Phase non trouvée"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
