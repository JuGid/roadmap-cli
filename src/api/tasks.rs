//! Tasks API endpoints

use axum::{extract::{Extension, Path, Query}, http::StatusCode, response::{IntoResponse, Json}};
use sea_orm::*;
use serde::Deserialize;
use crate::db::entities::{phase, task};

#[derive(Deserialize)]
pub struct TaskParams { pub status: Option<String> }

#[derive(Deserialize)]
pub struct CreateTaskRequest { pub task_id: String, pub name: String, pub description: Option<String>, pub optional: Option<bool> }

#[derive(Deserialize)]
pub struct UpdateTaskRequest { pub name: Option<String>, pub description: Option<String>, pub status: Option<String>, pub optional: Option<bool>, pub assignee_id: Option<String>, pub due_date: Option<String> }

async fn resolve_phase(db: &DatabaseConnection, slug: &str, phase_id: &str) -> Result<phase::Model, (StatusCode, Json<serde_json::Value>)> {
    let project = match crate::db::repos::get_project_by_slug(db, None, slug).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Projet non trouvé"})))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))),
    };
    match crate::db::repos::get_phase(db, project.id, phase_id).await {
        Ok(Some(p)) => Ok(p),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Phase non trouvée"})))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))),
    }
}

pub async fn list_tasks(Extension(db): Extension<DatabaseConnection>, Path((slug, phase_id)): Path<(String, String)>, Query(params): Query<TaskParams>) -> impl IntoResponse {
    let p = match resolve_phase(&db, &slug, &phase_id).await { Ok(p) => p, Err(e) => return e.into_response() };
    let mut query = task::Entity::find().filter(task::Column::PhaseId.eq(p.id));
    if let Some(status) = params.status { query = query.filter(task::Column::Status.eq(status)); }
    match query.order_by_asc(task::Column::TaskId).all(&db).await {
        Ok(t) => Json(serde_json::to_value(&t).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn create_task(Extension(db): Extension<DatabaseConnection>, Path((slug, phase_id)): Path<(String, String)>, Json(body): Json<CreateTaskRequest>) -> impl IntoResponse {
    let p = match resolve_phase(&db, &slug, &phase_id).await { Ok(p) => p, Err(e) => return e.into_response() };
    match crate::db::repos::create_task(&db, p.id, &body.task_id, &body.name, body.description.as_deref(), body.optional.unwrap_or(false)).await {
        Ok(t) => (StatusCode::CREATED, Json(serde_json::to_value(&t).unwrap())).into_response(),
        Err(e) => (StatusCode::CONFLICT, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn update_task(Extension(db): Extension<DatabaseConnection>, Path((slug, phase_id_str, task_id_str)): Path<(String, String, String)>, Json(body): Json<UpdateTaskRequest>) -> impl IntoResponse {
    let p = match resolve_phase(&db, &slug, &phase_id_str).await { Ok(p) => p, Err(e) => return e.into_response() };
    let t = match task::Entity::find()
        .filter(task::Column::PhaseId.eq(p.id))
        .filter(task::Column::TaskId.eq(&task_id_str))
        .one(&db).await
    {
        Ok(Some(t)) => t,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Tâche non trouvée"}))).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    let mut active: task::ActiveModel = t.clone().into();
    if let Some(name) = body.name { active.name = Set(name); }
    if let Some(desc) = body.description { active.description = Set(Some(desc)); }
    if let Some(opt) = body.optional { active.optional = Set(opt); }
    if let Some(status) = body.status {
        if status == "done" && t.status != "done" {
            active.completed_at = Set(Some(chrono::Utc::now().into()));
        } else if status != "done" {
            active.completed_at = Set(None);
        }
        active.status = Set(status);
    }
    if let Some(due) = body.due_date { active.due_date = Set(chrono::NaiveDate::parse_from_str(&due, "%Y-%m-%d").ok()); }
    if let Some(aid) = body.assignee_id { active.assignee_id = Set(uuid::Uuid::parse_str(&aid).ok()); }
    active.updated_at = Set(chrono::Utc::now().into());

    match active.update(&db).await {
        Ok(u) => Json(serde_json::to_value(&u).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn delete_task(Extension(db): Extension<DatabaseConnection>, Path((slug, phase_id_str, task_id_str)): Path<(String, String, String)>, ) -> impl IntoResponse {
    let p = match resolve_phase(&db, &slug, &phase_id_str).await { Ok(p) => p, Err(e) => return e.into_response() };
    let result = task::Entity::delete_many()
        .filter(task::Column::PhaseId.eq(p.id))
        .filter(task::Column::TaskId.eq(&task_id_str))
        .exec(&db).await;
    match result {
        Ok(r) if r.rows_affected > 0 => Json(serde_json::json!({"ok": true})).into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Tâche non trouvée"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
