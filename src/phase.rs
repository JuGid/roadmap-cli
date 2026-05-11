use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_priority")]
    pub priority: u32,
    #[serde(default = "default_status")]
    pub status: String,
    pub parent: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    pub workflow: Option<PhaseWorkflow>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub notes: Vec<Note>,
}

fn default_priority() -> u32 { 10 }
fn default_status() -> String { String::from("pending") }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseWorkflow {
    pub enabled: bool,
    pub stages: Vec<WorkflowStage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStage {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default = "default_status")]
    pub status: String,
    pub parent: Option<String>,
    pub workflow_stage: Option<String>,
    #[serde(default)]
    pub optional: bool,
    pub completed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocked_by: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub date: String,
    pub content: String,
}

// ============================================================================
// Bugs / Incidents
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct BugStore {
    #[serde(default)]
    pub bugs: Vec<Bug>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bug {
    pub id: u32,
    pub title: String,
    pub severity: String,
    #[serde(default = "default_bug_status")]
    pub status: String,
    pub phase: Option<String>,
    pub description: Option<String>,
    pub assignee: Option<String>,
    pub reported_by: Option<String>,
    pub resolution: Option<String>,
    pub created_at: String,
    pub resolved_at: Option<String>,
}

fn default_bug_status() -> String { String::from("open") }

impl BugStore {
    pub fn load() -> Self {
        Self::load_from(".phases")
    }

    pub fn load_from(phases_dir: &str) -> Self {
        let path = std::path::Path::new(phases_dir).join("bugs.yml");
        if !path.exists() {
            return BugStore { bugs: Vec::new() };
        }
        let content = std::fs::read_to_string(path).unwrap_or_default();
        serde_yaml::from_str(&content).unwrap_or(BugStore { bugs: Vec::new() })
    }

    pub fn save(&self) -> Result<(), String> {
        self.save_to(".phases")
    }

    pub fn save_to(&self, phases_dir: &str) -> Result<(), String> {
        let path = std::path::Path::new(phases_dir).join("bugs.yml");
        let content = serde_yaml::to_string(self)
            .map_err(|e| format!("Erreur sérialisation: {}", e))?;
        std::fs::write(path, content)
            .map_err(|e| format!("Erreur écriture: {}", e))?;
        Ok(())
    }

    pub fn next_id(&self) -> u32 {
        self.bugs.iter().map(|b| b.id).max().unwrap_or(0) + 1
    }
}

// ============================================================================
// Feature Requests
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureStore {
    #[serde(default)]
    pub features: Vec<FeatureRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureRequest {
    pub id: u32,
    pub title: String,
    pub priority: String,
    #[serde(default = "default_feature_status")]
    pub status: String,
    pub phase: Option<String>,
    pub description: Option<String>,
    pub assignee: Option<String>,
    pub requested_by: Option<String>,
    pub implementation: Option<String>,
    pub created_at: String,
    pub implemented_at: Option<String>,
}

fn default_feature_status() -> String { String::from("proposed") }

impl FeatureStore {
    pub fn load() -> Self {
        Self::load_from(".phases")
    }

    pub fn load_from(phases_dir: &str) -> Self {
        let path = std::path::Path::new(phases_dir).join("features.yml");
        if !path.exists() {
            return FeatureStore { features: Vec::new() };
        }
        let content = std::fs::read_to_string(path).unwrap_or_default();
        serde_yaml::from_str(&content).unwrap_or(FeatureStore { features: Vec::new() })
    }

    pub fn save(&self) -> Result<(), String> {
        self.save_to(".phases")
    }

    pub fn save_to(&self, phases_dir: &str) -> Result<(), String> {
        let path = std::path::Path::new(phases_dir).join("features.yml");
        let content = serde_yaml::to_string(self)
            .map_err(|e| format!("Erreur sérialisation: {}", e))?;
        std::fs::write(path, content)
            .map_err(|e| format!("Erreur écriture: {}", e))?;
        Ok(())
    }

    pub fn next_id(&self) -> u32 {
        self.features.iter().map(|f| f.id).max().unwrap_or(0) + 1
    }
}

impl Phase {
    pub fn new(id: String, name: String) -> Self {
        let now = chrono::Local::now().format("%Y-%m-%d").to_string();

        Phase {
            id,
            name,
            description: String::new(),
            priority: 10,
            status: String::from("pending"),
            parent: None,
            created_at: now.clone(),
            updated_at: now,
            workflow: None,
            depends_on: Vec::new(),
            tasks: Vec::new(),
            notes: Vec::new(),
        }
    }
}
