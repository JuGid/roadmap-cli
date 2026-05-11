//! Axum authentication middleware

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use uuid::Uuid;

use super::jwt::{validate_token, JwtConfig};

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
}

pub async fn auth_middleware(mut req: Request, next: Next) -> Response {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let token = match auth_header {
        Some(ref h) if h.starts_with("Bearer ") => &h[7..],
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Token manquant"})),
            )
                .into_response();
        }
    };

    let config = JwtConfig::default();
    let claims = match validate_token(token, &config) {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": e})),
            )
                .into_response();
        }
    };

    if claims.token_type != "access" {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Token de type access requis"})),
        )
            .into_response();
    }

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Token invalide"})),
            )
                .into_response();
        }
    };

    let auth_user = AuthUser {
        id: user_id,
        email: claims.email,
    };

    req.extensions_mut().insert(auth_user);
    next.run(req).await
}
