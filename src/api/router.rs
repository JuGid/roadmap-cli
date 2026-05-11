//! API v1 router with auth middleware

use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};

use super::{projects, phases, tasks, orgs, audit};
use crate::auth::middleware::auth_middleware;

pub fn api_v1_router() -> Router {
    Router::new()
        // Projects
        .route("/projects", get(projects::list_projects).post(projects::create_project))
        .route("/projects/{slug}", get(projects::get_project).put(projects::update_project).delete(projects::delete_project))
        // Phases
        .route("/projects/{slug}/phases", get(phases::list_phases).post(phases::create_phase))
        .route("/projects/{slug}/phases/{phase_id}", get(phases::get_phase).put(phases::update_phase).delete(phases::delete_phase))
        // Tasks
        .route("/projects/{slug}/phases/{phase_id}/tasks", get(tasks::list_tasks).post(tasks::create_task))
        .route("/projects/{slug}/phases/{phase_id}/tasks/{task_id}", put(tasks::update_task).delete(tasks::delete_task))
        // Activity log
        .route("/projects/{slug}/activity", get(audit::get_activity))
        // Organizations
        .route("/orgs", get(orgs::list_orgs).post(orgs::create_org))
        .route("/orgs/{slug}/members", get(orgs::list_members).post(orgs::invite_member))
        .route("/orgs/{slug}/members/{user_id}", put(orgs::update_member_role).delete(orgs::remove_member))
        // Auth middleware on all routes
        .layer(middleware::from_fn(auth_middleware))
}
