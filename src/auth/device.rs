//! Device authorization flow for CLI login
//!
//! Flow:
//! 1. CLI calls POST /api/auth/device — gets a device_code + user_code
//! 2. CLI opens browser to /auth/cli?code=<user_code>
//! 3. User logs in and approves
//! 4. App calls POST /api/auth/device/approve with user_code + access token
//! 5. CLI polls GET /api/auth/device/<device_code> until approved
//! 6. CLI receives access_token + refresh_token

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeviceStore {
    pub pending: Arc<RwLock<HashMap<String, DeviceRequest>>>,
}

#[derive(Clone, Debug)]
pub struct DeviceRequest {
    pub device_code: String,
    pub user_code: String,
    pub status: DeviceStatus,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub user_email: Option<String>,
    pub user_name: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum DeviceStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "expired")]
    Expired,
}

impl DeviceStore {
    pub fn new() -> Self {
        DeviceStore {
            pending: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Serialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_url: String,
    pub expires_in: u64,
    pub interval: u64,
}

/// POST /api/auth/device — CLI requests a device code
pub async fn request_device_code(
    Extension(store): Extension<DeviceStore>,
) -> impl IntoResponse {
    let device_code = Uuid::new_v4().to_string();
    let user_code = generate_user_code();

    let request = DeviceRequest {
        device_code: device_code.clone(),
        user_code: user_code.clone(),
        status: DeviceStatus::Pending,
        access_token: None,
        refresh_token: None,
        user_email: None,
        user_name: None,
        created_at: chrono::Utc::now(),
    };

    store.pending.write().await.insert(device_code.clone(), request);

    // Frontend URL (separate from API)
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    Json(DeviceCodeResponse {
        device_code,
        user_code: user_code.clone(),
        verification_url: format!("{}/auth/cli?code={}", frontend_url, user_code),
        expires_in: 300,
        interval: 2,
    })
    .into_response()
}

#[derive(Deserialize)]
pub struct ApproveRequest {
    pub user_code: String,
    pub access_token: String,
    pub refresh_token: String,
    pub user_email: String,
    pub user_name: String,
}

/// POST /api/auth/device/approve — Web app approves a device code
pub async fn approve_device(
    Extension(store): Extension<DeviceStore>,
    Json(body): Json<ApproveRequest>,
) -> impl IntoResponse {
    let mut pending = store.pending.write().await;

    let entry = pending.values_mut().find(|r| r.user_code == body.user_code);

    match entry {
        Some(req) => {
            req.status = DeviceStatus::Approved;
            req.access_token = Some(body.access_token);
            req.refresh_token = Some(body.refresh_token);
            req.user_email = Some(body.user_email);
            req.user_name = Some(body.user_name);
            Json(serde_json::json!({"ok": true})).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Code non trouvé"})),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
pub struct DenyRequest {
    pub user_code: String,
}

/// POST /api/auth/device/deny — Web app denies a device code
pub async fn deny_device(
    Extension(store): Extension<DeviceStore>,
    Json(body): Json<DenyRequest>,
) -> impl IntoResponse {
    let mut pending = store.pending.write().await;

    let entry = pending.values_mut().find(|r| r.user_code == body.user_code);

    match entry {
        Some(req) => {
            req.status = DeviceStatus::Denied;
            Json(serde_json::json!({"ok": true})).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Code non trouvé"})),
        )
            .into_response(),
    }
}

/// GET /api/auth/device/:device_code — CLI polls for approval
pub async fn poll_device(
    Extension(store): Extension<DeviceStore>,
    Path(device_code): Path<String>,
) -> impl IntoResponse {
    let pending = store.pending.read().await;

    match pending.get(&device_code) {
        Some(req) => {
            // Check expiry (5 min)
            let elapsed = chrono::Utc::now() - req.created_at;
            if elapsed.num_seconds() > 300 {
                return (
                    StatusCode::GONE,
                    Json(serde_json::json!({"status": "expired"})),
                )
                    .into_response();
            }

            match req.status {
                DeviceStatus::Pending => (
                    StatusCode::ACCEPTED,
                    Json(serde_json::json!({"status": "pending"})),
                )
                    .into_response(),
                DeviceStatus::Approved => Json(serde_json::json!({
                    "status": "approved",
                    "access_token": req.access_token,
                    "refresh_token": req.refresh_token,
                    "user_email": req.user_email,
                    "user_name": req.user_name,
                }))
                .into_response(),
                DeviceStatus::Denied => (
                    StatusCode::FORBIDDEN,
                    Json(serde_json::json!({"status": "denied"})),
                )
                    .into_response(),
                DeviceStatus::Expired => (
                    StatusCode::GONE,
                    Json(serde_json::json!({"status": "expired"})),
                )
                    .into_response(),
            }
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Device code non trouvé"})),
        )
            .into_response(),
    }
}

fn generate_user_code() -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
    let code = (ts % 900000) + 100000;
    format!("{}", code)
}
