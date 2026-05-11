//! Projects API endpoints

use axum::{extract::{Extension, Path}, http::StatusCode, response::{IntoResponse, Json}};
use sea_orm::*;
use serde::Deserialize;

use crate::auth::middleware::AuthUser;
use crate::db::entities::project;

#[derive(Deserialize)]
pub struct CreateProjectRequest { pub name: String, pub slug: String, pub description: Option<String> }

#[derive(Deserialize)]
pub struct UpdateProjectRequest { pub name: Option<String>, pub description: Option<String> }

pub async fn list_projects(Extension(db): Extension<DatabaseConnection>, Extension(user): Extension<AuthUser>) -> impl IntoResponse {
    match crate::db::repos::list_projects_for_user(&db, user.id).await {
        Ok(projects) => Json(serde_json::to_value(&projects).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn create_project(Extension(db): Extension<DatabaseConnection>, Extension(user): Extension<AuthUser>, Json(body): Json<CreateProjectRequest>) -> impl IntoResponse {
    match crate::db::repos::create_project(&db, user.id, None, &body.name, &body.slug, body.description.as_deref().unwrap_or("")).await {
        Ok(p) => (StatusCode::CREATED, Json(serde_json::to_value(&p).unwrap())).into_response(),
        Err(e) => (StatusCode::CONFLICT, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn get_project(Extension(db): Extension<DatabaseConnection>, Path(slug): Path<String>) -> impl IntoResponse {
    match crate::db::repos::get_project_by_slug(&db, None, &slug).await {
        Ok(Some(p)) => Json(serde_json::to_value(&p).unwrap()).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Projet non trouvé"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn update_project(Extension(db): Extension<DatabaseConnection>, Path(slug): Path<String>, Json(body): Json<UpdateProjectRequest>) -> impl IntoResponse {
    let p = match crate::db::repos::get_project_by_slug(&db, None, &slug).await {
        Ok(Some(p)) => p,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Projet non trouvé"}))).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    let mut active: project::ActiveModel = p.clone().into();
    if let Some(name) = body.name { active.name = Set(name); }
    if let Some(desc) = body.description { active.description = Set(desc); }
    active.updated_at = Set(chrono::Utc::now().into());
    match active.update(&db).await {
        Ok(u) => Json(serde_json::to_value(&u).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn delete_project(Extension(db): Extension<DatabaseConnection>, Path(slug): Path<String>) -> impl IntoResponse {
    let result = project::Entity::delete_many()
        .filter(project::Column::Slug.eq(&slug))
        .filter(project::Column::OrgId.is_null())
        .exec(&db).await;
    match result {
        Ok(r) if r.rows_affected > 0 => Json(serde_json::json!({"ok": true})).into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Projet non trouvé"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
