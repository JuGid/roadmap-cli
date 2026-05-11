use std::fs;
use std::path::Path;

use crate::phase::Phase;

/// Load all phases from the .phases directory
/// Returns None if the roadmap is not initialized or on error
pub fn load_phases() -> Option<Vec<Phase>> {
    load_phases_from(Path::new(".phases"))
}

/// Load all phases from a specific directory
pub fn load_phases_from(phases_dir: &Path) -> Option<Vec<Phase>> {
    if !phases_dir.exists() {
        return None;
    }

    let entries = match fs::read_dir(phases_dir) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    let mut phases: Vec<Phase> = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        let filename = match path.file_name().and_then(|f| f.to_str()) {
            Some(f) => f,
            None => continue,
        };

        if !filename.starts_with("phase-") || !filename.ends_with(".yml") {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let phase: Phase = match serde_yaml::from_str(&content) {
            Ok(p) => p,
            Err(_) => continue,
        };

        phases.push(phase);
    }

    phases.sort_by(|a, b| a.priority.cmp(&b.priority));
    Some(phases)
}
