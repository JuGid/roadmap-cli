//! JWT token generation and validation

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // user_id
    pub email: String,
    pub exp: usize,         // expiration timestamp
    pub iat: usize,         // issued at
    pub token_type: String, // "access" or "refresh"
}

pub struct JwtConfig {
    pub secret: String,
    pub access_expiry_hours: i64,
    pub refresh_expiry_days: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "roadmap-dev-secret-change-me".to_string()),
            access_expiry_hours: 1,
            refresh_expiry_days: 30,
        }
    }
}

pub fn create_access_token(user_id: Uuid, email: &str, config: &JwtConfig) -> Result<String, String> {
    let now = Utc::now();
    let exp = now + Duration::hours(config.access_expiry_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_type: "access".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| format!("Erreur JWT: {}", e))
}

pub fn create_refresh_token(user_id: Uuid, email: &str, config: &JwtConfig) -> Result<String, String> {
    let now = Utc::now();
    let exp = now + Duration::days(config.refresh_expiry_days);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_type: "refresh".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| format!("Erreur JWT: {}", e))
}

pub fn validate_token(token: &str, config: &JwtConfig) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Token invalide: {}", e))
}
