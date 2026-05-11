use askama::Template;
use crate::phase::{Phase, Task};

/// Phase info for display in templates
pub struct PhaseInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub priority: u32,
    pub status: String,
    pub tasks_done: usize,
    pub tasks_total: usize,
}

impl From<&Phase> for PhaseInfo {
    fn from(phase: &Phase) -> Self {
        let tasks_done = phase.tasks.iter().filter(|t| t.status == "done").count();
        let tasks_total = phase.tasks.len();

        PhaseInfo {
            id: phase.id.clone(),
            name: phase.name.clone(),
            description: phase.description.clone(),
            priority: phase.priority,
            status: phase.status.clone(),
            tasks_done,
            tasks_total,
        }
    }
}

/// Task info for display in templates
pub struct TaskInfo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub optional: bool,
    pub workflow_stage: Option<String>,
}

impl From<&Task> for TaskInfo {
    fn from(task: &Task) -> Self {
        TaskInfo {
            id: task.id.clone(),
            name: task.name.clone(),
            status: task.status.clone(),
            optional: task.optional,
            workflow_stage: task.workflow_stage.clone(),
        }
    }
}

/// Kanban index page
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub current_page: String,
    pub phases_pending: Vec<PhaseInfo>,
    pub phases_in_progress: Vec<PhaseInfo>,
    pub phases_done: Vec<PhaseInfo>,
    pub phases_blocked: Vec<PhaseInfo>,
}

/// Phases list page
#[derive(Template)]
#[template(path = "phases.html")]
pub struct PhasesTemplate {
    pub title: String,
    pub current_page: String,
    pub phases: Vec<PhaseInfo>,
}

/// Single phase detail page
#[derive(Template)]
#[template(path = "phase.html")]
pub struct PhaseTemplate {
    pub title: String,
    pub current_page: String,
    pub phase: PhaseInfo,
    pub tasks: Vec<TaskInfo>,
    pub notes: Vec<NoteInfo>,
}

pub struct NoteInfo {
    pub date: String,
    pub content: String,
}

/// Error page (404, etc.)
#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub title: String,
    pub current_page: String,
    pub error_code: u16,
    pub error_message: String,
}
