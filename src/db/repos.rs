//! Repository functions using SeaORM

use sea_orm::*;
use uuid::Uuid;

use super::entities::{project, phase, task, note, organization, org_member, tag, activity_log};

// ============================================================================
// Projects
// ============================================================================

pub async fn create_project(
    db: &DatabaseConnection,
    owner_id: Uuid,
    org_id: Option<Uuid>,
    name: &str,
    slug: &str,
    description: &str,
) -> Result<project::Model, DbErr> {
    let model = project::ActiveModel {
        id: Set(Uuid::new_v4()),
        owner_id: Set(owner_id),
        org_id: Set(org_id),
        name: Set(name.to_string()),
        slug: Set(slug.to_string()),
        description: Set(description.to_string()),
        workspace_id: Set(None),
        team_id: Set(None),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };
    model.insert(db).await
}

pub async fn get_project_by_slug(
    db: &DatabaseConnection,
    org_id: Option<Uuid>,
    slug: &str,
) -> Result<Option<project::Model>, DbErr> {
    let mut query = project::Entity::find()
        .filter(project::Column::Slug.eq(slug));

    if let Some(oid) = org_id {
        query = query.filter(project::Column::OrgId.eq(oid));
    } else {
        query = query.filter(project::Column::OrgId.is_null());
    }

    query.one(db).await
}

pub async fn list_projects_for_user(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<project::Model>, DbErr> {
    // Projects owned by user OR in orgs where user is member
    let org_ids: Vec<Uuid> = org_member::Entity::find()
        .filter(org_member::Column::UserId.eq(user_id))
        .all(db)
        .await?
        .into_iter()
        .map(|m| m.org_id)
        .collect();

    let mut condition = Condition::any()
        .add(project::Column::OwnerId.eq(user_id));

    if !org_ids.is_empty() {
        condition = condition.add(project::Column::OrgId.is_in(org_ids));
    }

    project::Entity::find()
        .filter(condition)
        .order_by_desc(project::Column::UpdatedAt)
        .all(db)
        .await
}

// ============================================================================
// Phases
// ============================================================================

pub async fn create_phase(
    db: &DatabaseConnection,
    project_id: Uuid,
    phase_id: &str,
    name: &str,
    description: &str,
    priority: i32,
) -> Result<phase::Model, DbErr> {
    let model = phase::ActiveModel {
        id: Set(Uuid::new_v4()),
        project_id: Set(project_id),
        phase_id: Set(phase_id.to_string()),
        name: Set(name.to_string()),
        description: Set(description.to_string()),
        priority: Set(priority),
        status: Set("pending".to_string()),
        parent_phase_id: Set(None),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };
    model.insert(db).await
}

pub async fn list_phases(
    db: &DatabaseConnection,
    project_id: Uuid,
) -> Result<Vec<phase::Model>, DbErr> {
    phase::Entity::find()
        .filter(phase::Column::ProjectId.eq(project_id))
        .order_by_asc(phase::Column::Priority)
        .order_by_asc(phase::Column::PhaseId)
        .all(db)
        .await
}

pub async fn get_phase(
    db: &DatabaseConnection,
    project_id: Uuid,
    phase_id: &str,
) -> Result<Option<phase::Model>, DbErr> {
    phase::Entity::find()
        .filter(phase::Column::ProjectId.eq(project_id))
        .filter(phase::Column::PhaseId.eq(phase_id))
        .one(db)
        .await
}

pub async fn update_phase_status(
    db: &DatabaseConnection,
    id: Uuid,
    status: &str,
) -> Result<phase::Model, DbErr> {
    let phase = phase::Entity::find_by_id(id).one(db).await?
        .ok_or(DbErr::RecordNotFound("Phase not found".to_string()))?;

    let mut active: phase::ActiveModel = phase.into();
    active.status = Set(status.to_string());
    active.updated_at = Set(chrono::Utc::now().into());
    active.update(db).await
}

// ============================================================================
// Tasks
// ============================================================================

pub async fn create_task(
    db: &DatabaseConnection,
    phase_id: Uuid,
    task_id: &str,
    name: &str,
    description: Option<&str>,
    optional: bool,
) -> Result<task::Model, DbErr> {
    let model = task::ActiveModel {
        id: Set(Uuid::new_v4()),
        phase_id: Set(phase_id),
        task_id: Set(task_id.to_string()),
        name: Set(name.to_string()),
        description: Set(description.map(|s| s.to_string())),
        status: Set("pending".to_string()),
        optional: Set(optional),
        assignee_id: Set(None),
        due_date: Set(None),
        parent_task_id: Set(None),
        workflow_stage: Set(None),
        completed_at: Set(None),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };
    model.insert(db).await
}

pub async fn list_tasks(
    db: &DatabaseConnection,
    phase_id: Uuid,
) -> Result<Vec<task::Model>, DbErr> {
    task::Entity::find()
        .filter(task::Column::PhaseId.eq(phase_id))
        .order_by_asc(task::Column::TaskId)
        .all(db)
        .await
}

pub async fn update_task_status(
    db: &DatabaseConnection,
    id: Uuid,
    status: &str,
) -> Result<task::Model, DbErr> {
    let t = task::Entity::find_by_id(id).one(db).await?
        .ok_or(DbErr::RecordNotFound("Task not found".to_string()))?;

    let completed_at = if status == "done" {
        Some(chrono::Utc::now().fixed_offset())
    } else {
        None
    };

    let mut active: task::ActiveModel = t.into();
    active.status = Set(status.to_string());
    active.completed_at = Set(completed_at);
    active.updated_at = Set(chrono::Utc::now().into());
    active.update(db).await
}

// ============================================================================
// Notes
// ============================================================================

pub async fn create_note(
    db: &DatabaseConnection,
    phase_id: Uuid,
    content: &str,
) -> Result<note::Model, DbErr> {
    let model = note::ActiveModel {
        id: Set(Uuid::new_v4()),
        phase_id: Set(phase_id),
        content: Set(content.to_string()),
        created_at: Set(chrono::Utc::now().into()),
    };
    model.insert(db).await
}

pub async fn list_notes(
    db: &DatabaseConnection,
    phase_id: Uuid,
) -> Result<Vec<note::Model>, DbErr> {
    note::Entity::find()
        .filter(note::Column::PhaseId.eq(phase_id))
        .order_by_desc(note::Column::CreatedAt)
        .all(db)
        .await
}

// ============================================================================
// Tags
// ============================================================================

pub async fn get_or_create_tag(
    db: &DatabaseConnection,
    project_id: Uuid,
    name: &str,
) -> Result<tag::Model, DbErr> {
    let existing = tag::Entity::find()
        .filter(tag::Column::ProjectId.eq(project_id))
        .filter(tag::Column::Name.eq(name))
        .one(db)
        .await?;

    if let Some(t) = existing {
        return Ok(t);
    }

    let model = tag::ActiveModel {
        id: Set(Uuid::new_v4()),
        project_id: Set(project_id),
        name: Set(name.to_string()),
    };
    model.insert(db).await
}

// ============================================================================
// Activity Log
// ============================================================================

pub async fn log_activity(
    db: &DatabaseConnection,
    project_id: Uuid,
    user_id: Option<Uuid>,
    action: &str,
    target_type: Option<&str>,
    target_id: Option<Uuid>,
    metadata: Option<serde_json::Value>,
) -> Result<(), DbErr> {
    let model = activity_log::ActiveModel {
        id: Set(Uuid::new_v4()),
        project_id: Set(project_id),
        user_id: Set(user_id),
        action: Set(action.to_string()),
        target_type: Set(target_type.map(|s| s.to_string())),
        target_id: Set(target_id),
        metadata: Set(metadata),
        created_at: Set(chrono::Utc::now().into()),
    };
    model.insert(db).await?;
    Ok(())
}

pub async fn get_activity(
    db: &DatabaseConnection,
    project_id: Uuid,
    limit: u64,
) -> Result<Vec<activity_log::Model>, DbErr> {
    activity_log::Entity::find()
        .filter(activity_log::Column::ProjectId.eq(project_id))
        .order_by_desc(activity_log::Column::CreatedAt)
        .limit(limit)
        .all(db)
        .await
}
