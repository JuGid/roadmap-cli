//! Auth API endpoints

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sea_orm::*;
use serde::{Deserialize, Serialize};

use super::jwt::{create_access_token, create_refresh_token, validate_token, JwtConfig};
use super::password::{hash_password, verify_password};
use crate::db::entities::user;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
}

/// POST /api/auth/register
pub async fn register(
    Extension(db): Extension<DatabaseConnection>,
    Json(body): Json<RegisterRequest>,
) -> impl IntoResponse {
    if body.email.is_empty() || body.password.is_empty() || body.name.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Tous les champs sont requis"}))).into_response();
    }
    if body.password.len() < 8 {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Mot de passe trop court (min 8 caractères)"}))).into_response();
    }

    let exists = user::Entity::find()
        .filter(user::Column::Email.eq(&body.email))
        .one(&db).await;

    if let Ok(Some(_)) = exists {
        return (StatusCode::CONFLICT, Json(serde_json::json!({"error": "Email déjà utilisé"}))).into_response();
    }

    let password_hash = match hash_password(&body.password) {
        Ok(h) => h,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    };

    let new_user = user::ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        email: Set(body.email.clone()),
        name: Set(body.name.clone()),
        password_hash: Set(password_hash),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };

    let user_model = match new_user.insert(&db).await {
        Ok(u) => u,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    let config = JwtConfig::default();
    let access_token = create_access_token(user_model.id, &user_model.email, &config).unwrap();
    let refresh_token = create_refresh_token(user_model.id, &user_model.email, &config).unwrap();

    (StatusCode::CREATED, Json(AuthResponse {
        access_token, refresh_token,
        user: UserResponse { id: user_model.id.to_string(), email: user_model.email, name: user_model.name },
    })).into_response()
}

/// POST /api/auth/login
pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let user_model = match user::Entity::find()
        .filter(user::Column::Email.eq(&body.email))
        .one(&db).await
    {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Email ou mot de passe incorrect"}))).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    if !verify_password(&body.password, &user_model.password_hash) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Email ou mot de passe incorrect"}))).into_response();
    }

    let config = JwtConfig::default();
    let access_token = create_access_token(user_model.id, &user_model.email, &config).unwrap();
    let refresh_token = create_refresh_token(user_model.id, &user_model.email, &config).unwrap();

    Json(AuthResponse {
        access_token, refresh_token,
        user: UserResponse { id: user_model.id.to_string(), email: user_model.email, name: user_model.name },
    }).into_response()
}

/// POST /api/auth/refresh
pub async fn refresh(
    Extension(db): Extension<DatabaseConnection>,
    Json(body): Json<RefreshRequest>,
) -> impl IntoResponse {
    let config = JwtConfig::default();
    let claims = match validate_token(&body.refresh_token, &config) {
        Ok(c) => c,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": e}))).into_response(),
    };

    if claims.token_type != "refresh" {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Token de type refresh requis"}))).into_response();
    }

    let user_id = match uuid::Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Token invalide"}))).into_response(),
    };

    let user_model = match user::Entity::find_by_id(user_id).one(&db).await {
        Ok(Some(u)) => u,
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Utilisateur introuvable"}))).into_response(),
    };

    let access_token = create_access_token(user_model.id, &user_model.email, &config).unwrap();
    let refresh_token = create_refresh_token(user_model.id, &user_model.email, &config).unwrap();

    Json(AuthResponse {
        access_token, refresh_token,
        user: UserResponse { id: user_model.id.to_string(), email: user_model.email, name: user_model.name },
    }).into_response()
}
