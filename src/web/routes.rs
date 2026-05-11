use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
};
use askama::Template;
use serde::{Deserialize, Serialize};

use crate::data::load_phases;
use crate::phase::Phase;

use super::templates::{
    ErrorTemplate, IndexTemplate, NoteInfo, PhaseInfo, PhaseTemplate, PhasesTemplate, TaskInfo,
};

// ============================================================================
// API Routes (JSON)
// ============================================================================

#[derive(Serialize)]
pub struct ApiPhase {
    pub id: String,
    pub name: String,
    pub description: String,
    pub priority: u32,
    pub status: String,
    pub tasks_done: usize,
    pub tasks_total: usize,
}

impl From<&Phase> for ApiPhase {
    fn from(phase: &Phase) -> Self {
        ApiPhase {
            id: phase.id.clone(),
            name: phase.name.clone(),
            description: phase.description.clone(),
            priority: phase.priority,
            status: phase.status.clone(),
            tasks_done: phase.tasks.iter().filter(|t| t.status == "done").count(),
            tasks_total: phase.tasks.len(),
        }
    }
}

/// GET /api/phases - List all phases
pub async fn api_list_phases() -> impl IntoResponse {
    match load_phases() {
        Some(phases) => {
            let api_phases: Vec<ApiPhase> = phases.iter().map(ApiPhase::from).collect();
            Json(api_phases).into_response()
        }
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Roadmap non initialisée"})),
        )
            .into_response(),
    }
}

/// GET /api/phases/:id - Get a single phase
pub async fn api_get_phase(Path(id): Path<String>) -> impl IntoResponse {
    match load_phases() {
        Some(phases) => {
            if let Some(phase) = phases.iter().find(|p| p.id == id) {
                Json(phase).into_response()
            } else {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"error": "Phase non trouvée", "id": id})),
                )
                    .into_response()
            }
        }
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Roadmap non initialisée"})),
        )
            .into_response(),
    }
}

#[derive(Serialize)]
pub struct ReportSummary {
    pub total_phases: usize,
    pub phases_done: usize,
    pub phases_in_progress: usize,
    pub phases_pending: usize,
    pub phases_blocked: usize,
    pub total_tasks: usize,
    pub tasks_done: usize,
    pub tasks_in_progress: usize,
    pub tasks_pending: usize,
    pub tasks_optional: usize,
    pub progress_percent: f32,
}

#[derive(Serialize)]
pub struct ApiReport {
    pub summary: ReportSummary,
    pub phases_in_progress: Vec<ApiPhase>,
}

/// GET /api/report - Get report data
pub async fn api_report() -> impl IntoResponse {
    match load_phases() {
        Some(phases) => {
            let phases_done = phases.iter().filter(|p| p.status == "done").count();
            let phases_in_progress_list: Vec<&Phase> =
                phases.iter().filter(|p| p.status == "in_progress").collect();
            let phases_pending = phases.iter().filter(|p| p.status == "pending").count();
            let phases_blocked = phases.iter().filter(|p| p.status == "blocked").count();

            let mut total_tasks = 0;
            let mut tasks_done = 0;
            let mut tasks_in_progress = 0;
            let mut tasks_pending = 0;
            let mut tasks_optional = 0;

            for phase in &phases {
                for task in &phase.tasks {
                    total_tasks += 1;
                    if task.optional {
                        tasks_optional += 1;
                    }
                    match task.status.as_str() {
                        "done" => tasks_done += 1,
                        "in_progress" => tasks_in_progress += 1,
                        "pending" => tasks_pending += 1,
                        _ => {}
                    }
                }
            }

            let required_tasks = tasks_done + tasks_in_progress + tasks_pending;
            let progress_percent = if required_tasks > 0 {
                (tasks_done as f32 / required_tasks as f32) * 100.0
            } else {
                0.0
            };

            let report = ApiReport {
                summary: ReportSummary {
                    total_phases: phases.len(),
                    phases_done,
                    phases_in_progress: phases_in_progress_list.len(),
                    phases_pending,
                    phases_blocked,
                    total_tasks,
                    tasks_done,
                    tasks_in_progress,
                    tasks_pending,
                    tasks_optional,
                    progress_percent,
                },
                phases_in_progress: phases_in_progress_list.iter().map(|p| ApiPhase::from(*p)).collect(),
            };

            Json(report).into_response()
        }
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Roadmap non initialisée"})),
        )
            .into_response(),
    }
}

// ============================================================================
// API: Update phase status (for drag & drop)
// ============================================================================

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

/// POST /api/phases/:id/status - Update phase status
pub async fn api_update_phase_status(
    Path(id): Path<String>,
    Json(body): Json<UpdateStatusRequest>,
) -> impl IntoResponse {
    let valid = ["pending", "in_progress", "done", "blocked"];
    if !valid.contains(&body.status.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": format!("Statut invalide: {}", body.status)})),
        )
            .into_response();
    }

    let phases_dir = std::path::Path::new(".phases");
    let phase_file = phases_dir.join(format!("phase-{}.yml", id));

    if !phase_file.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Phase non trouvée", "id": id})),
        )
            .into_response();
    }

    let content = match std::fs::read_to_string(&phase_file) {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Erreur lecture: {}", e)})),
            )
                .into_response();
        }
    };

    let mut phase: Phase = match serde_yaml::from_str(&content) {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("YAML invalide: {}", e)})),
            )
                .into_response();
        }
    };

    let old_status = phase.status.clone();
    phase.status = body.status.clone();
    phase.updated_at = chrono::Local::now().format("%Y-%m-%d").to_string();

    let yaml = serde_yaml::to_string(&phase).expect("Erreur sérialisation");
    if let Err(e) = std::fs::write(&phase_file, yaml) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Erreur écriture: {}", e)})),
        )
            .into_response();
    }

    Json(serde_json::json!({
        "ok": true,
        "id": id,
        "old_status": old_status,
        "new_status": body.status,
    }))
    .into_response()
}

// ============================================================================
// HTML Routes (Askama templates)
// ============================================================================

/// GET / - Kanban view
pub async fn index() -> impl IntoResponse {
    match load_phases() {
        Some(phases) => {
            let phases_pending: Vec<PhaseInfo> = phases
                .iter()
                .filter(|p| p.status == "pending")
                .map(PhaseInfo::from)
                .collect();
            let phases_in_progress: Vec<PhaseInfo> = phases
                .iter()
                .filter(|p| p.status == "in_progress")
                .map(PhaseInfo::from)
                .collect();
            let phases_done: Vec<PhaseInfo> = phases
                .iter()
                .filter(|p| p.status == "done")
                .map(PhaseInfo::from)
                .collect();
            let phases_blocked: Vec<PhaseInfo> = phases
                .iter()
                .filter(|p| p.status == "blocked")
                .map(PhaseInfo::from)
                .collect();

            let template = IndexTemplate {
                title: "Roadmap - Kanban".to_string(),
                current_page: "index".to_string(),
                phases_pending,
                phases_in_progress,
                phases_done,
                phases_blocked,
            };

            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(_) => render_error(500, "Erreur de rendu").into_response(),
            }
        }
        None => render_error(503, "Roadmap non initialisée. Lancez: roadmap init").into_response(),
    }
}

/// GET /phases - List all phases
pub async fn phases_list() -> impl IntoResponse {
    match load_phases() {
        Some(phases) => {
            let phase_infos: Vec<PhaseInfo> = phases.iter().map(PhaseInfo::from).collect();

            let template = PhasesTemplate {
                title: "Roadmap - Phases".to_string(),
                current_page: "phases".to_string(),
                phases: phase_infos,
            };

            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(_) => render_error(500, "Erreur de rendu").into_response(),
            }
        }
        None => render_error(503, "Roadmap non initialisée. Lancez: roadmap init").into_response(),
    }
}

/// GET /phases/:id - Phase detail
pub async fn phase_detail(Path(id): Path<String>) -> impl IntoResponse {
    match load_phases() {
        Some(phases) => {
            if let Some(phase) = phases.iter().find(|p| p.id == id) {
                let phase_info = PhaseInfo::from(phase);
                let tasks: Vec<TaskInfo> = phase
                    .tasks
                    .iter()
                    .filter(|t| t.parent.is_none())
                    .map(TaskInfo::from)
                    .collect();
                let notes: Vec<NoteInfo> = phase
                    .notes
                    .iter()
                    .map(|n| NoteInfo {
                        date: n.date.clone(),
                        content: n.content.clone(),
                    })
                    .collect();

                let template = PhaseTemplate {
                    title: format!("Phase {} - {}", phase.id, phase.name),
                    current_page: "phases".to_string(),
                    phase: phase_info,
                    tasks,
                    notes,
                };

                match template.render() {
                    Ok(html) => Html(html).into_response(),
                    Err(_) => render_error(500, "Erreur de rendu").into_response(),
                }
            } else {
                render_error(404, &format!("Phase {} non trouvée", id)).into_response()
            }
        }
        None => render_error(503, "Roadmap non initialisée. Lancez: roadmap init").into_response(),
    }
}

fn render_error(code: u16, message: &str) -> Html<String> {
    let template = ErrorTemplate {
        title: format!("Erreur {}", code),
        current_page: "".to_string(),
        error_code: code,
        error_message: message.to_string(),
    };

    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html(format!(
            "<html><body><h1>Erreur {}</h1><p>{}</p></body></html>",
            code, message
        )),
    }
}
