//! Organizations & Members API endpoints

use axum::{extract::{Extension, Path}, http::StatusCode, response::{IntoResponse, Json}};
use sea_orm::*;
use serde::Deserialize;
use uuid::Uuid;
use crate::auth::middleware::AuthUser;
use crate::db::entities::{organization, org_member, user};

#[derive(Deserialize)]
pub struct CreateOrgRequest { pub name: String, pub slug: String }
#[derive(Deserialize)]
pub struct InviteMemberRequest { pub email: String, pub role: Option<String> }
#[derive(Deserialize)]
pub struct UpdateRoleRequest { pub role: String }

pub async fn list_orgs(Extension(db): Extension<DatabaseConnection>, Extension(auth_user): Extension<AuthUser>) -> impl IntoResponse {
    let member_orgs: Vec<Uuid> = org_member::Entity::find()
        .filter(org_member::Column::UserId.eq(auth_user.id))
        .all(&db).await.unwrap_or_default()
        .into_iter().map(|m| m.org_id).collect();

    if member_orgs.is_empty() {
        return Json(serde_json::json!([])).into_response();
    }

    match organization::Entity::find().filter(organization::Column::Id.is_in(member_orgs)).all(&db).await {
        Ok(orgs) => Json(serde_json::to_value(&orgs).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn create_org(Extension(db): Extension<DatabaseConnection>, Extension(auth_user): Extension<AuthUser>, Json(body): Json<CreateOrgRequest>) -> impl IntoResponse {
    let org = organization::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(body.name),
        slug: Set(body.slug),
        created_at: Set(chrono::Utc::now().into()),
    };
    let org = match org.insert(&db).await {
        Ok(o) => o,
        Err(e) => return (StatusCode::CONFLICT, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    let member = org_member::ActiveModel {
        org_id: Set(org.id),
        user_id: Set(auth_user.id),
        role: Set("owner".to_string()),
        joined_at: Set(chrono::Utc::now().into()),
    };
    let _ = member.insert(&db).await;

    (StatusCode::CREATED, Json(serde_json::to_value(&org).unwrap())).into_response()
}

pub async fn list_members(Extension(db): Extension<DatabaseConnection>, Extension(auth_user): Extension<AuthUser>, Path(slug): Path<String>) -> impl IntoResponse {
    let org = match get_org_if_member(&db, &slug, auth_user.id).await { Ok(o) => o, Err(e) => return e.into_response() };

    let members = org_member::Entity::find()
        .filter(org_member::Column::OrgId.eq(org.id))
        .find_also_related(user::Entity)
        .all(&db).await;

    match members {
        Ok(list) => {
            let result: Vec<serde_json::Value> = list.into_iter().map(|(m, u)| {
                serde_json::json!({
                    "user_id": m.user_id,
                    "role": m.role,
                    "joined_at": m.joined_at,
                    "email": u.as_ref().map(|u| &u.email),
                    "name": u.as_ref().map(|u| &u.name),
                })
            }).collect();
            Json(serde_json::to_value(&result).unwrap()).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn invite_member(Extension(db): Extension<DatabaseConnection>, Extension(auth_user): Extension<AuthUser>, Path(slug): Path<String>, Json(body): Json<InviteMemberRequest>) -> impl IntoResponse {
    let org = match get_org_if_admin(&db, &slug, auth_user.id).await { Ok(o) => o, Err(e) => return e.into_response() };

    let invited = match user::Entity::find().filter(user::Column::Email.eq(&body.email)).one(&db).await {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Utilisateur non trouvé"}))).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    let role = body.role.as_deref().unwrap_or("member");
    let member = org_member::ActiveModel {
        org_id: Set(org.id),
        user_id: Set(invited.id),
        role: Set(role.to_string()),
        joined_at: Set(chrono::Utc::now().into()),
    };
    let _ = member.insert(&db).await;

    Json(serde_json::json!({"ok": true, "email": body.email, "role": role})).into_response()
}

pub async fn update_member_role(Extension(db): Extension<DatabaseConnection>, Extension(auth_user): Extension<AuthUser>, Path((slug, member_id)): Path<(String, Uuid)>, Json(body): Json<UpdateRoleRequest>) -> impl IntoResponse {
    let org = match get_org_if_admin(&db, &slug, auth_user.id).await { Ok(o) => o, Err(e) => return e.into_response() };

    let member = org_member::Entity::find()
        .filter(org_member::Column::OrgId.eq(org.id))
        .filter(org_member::Column::UserId.eq(member_id))
        .one(&db).await;

    match member {
        Ok(Some(m)) => {
            let mut active: org_member::ActiveModel = m.into();
            active.role = Set(body.role.clone());
            match active.update(&db).await {
                Ok(_) => Json(serde_json::json!({"ok": true, "role": body.role})).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
            }
        }
        _ => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Membre non trouvé"}))).into_response(),
    }
}

pub async fn remove_member(Extension(db): Extension<DatabaseConnection>, Extension(auth_user): Extension<AuthUser>, Path((slug, member_id)): Path<(String, Uuid)>) -> impl IntoResponse {
    let org = match get_org_if_admin(&db, &slug, auth_user.id).await { Ok(o) => o, Err(e) => return e.into_response() };

    let result = org_member::Entity::delete_many()
        .filter(org_member::Column::OrgId.eq(org.id))
        .filter(org_member::Column::UserId.eq(member_id))
        .exec(&db).await;

    match result {
        Ok(r) if r.rows_affected > 0 => Json(serde_json::json!({"ok": true})).into_response(),
        _ => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Membre non trouvé"}))).into_response(),
    }
}

async fn get_org_if_member(db: &DatabaseConnection, slug: &str, user_id: Uuid) -> Result<organization::Model, (StatusCode, Json<serde_json::Value>)> {
    let org = organization::Entity::find().filter(organization::Column::Slug.eq(slug)).one(db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))))?
        .ok_or((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Organisation non trouvée"}))))?;

    let is_member = org_member::Entity::find()
        .filter(org_member::Column::OrgId.eq(org.id))
        .filter(org_member::Column::UserId.eq(user_id))
        .one(db).await.unwrap_or(None).is_some();

    if !is_member { return Err((StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Accès refusé"})))); }
    Ok(org)
}

async fn get_org_if_admin(db: &DatabaseConnection, slug: &str, user_id: Uuid) -> Result<organization::Model, (StatusCode, Json<serde_json::Value>)> {
    let org = organization::Entity::find().filter(organization::Column::Slug.eq(slug)).one(db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))))?
        .ok_or((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Organisation non trouvée"}))))?;

    let member = org_member::Entity::find()
        .filter(org_member::Column::OrgId.eq(org.id))
        .filter(org_member::Column::UserId.eq(user_id))
        .one(db).await.unwrap_or(None);

    match member.map(|m| m.role.as_str() == "owner" || m.role.as_str() == "admin") {
        Some(true) => Ok(org),
        Some(false) => Err((StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Droits admin requis"})))),
        None => Err((StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Accès refusé"})))),
    }
}
