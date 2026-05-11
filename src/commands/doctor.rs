//! Doctor command - validates YAML integrity

use std::fs;
use std::path::Path;
use std::collections::HashSet;
use colored::Colorize;
use crate::phase::Phase;

const VALID_STATUSES: &[&str] = &["pending", "in_progress", "done", "blocked"];

pub fn cmd_doctor() {
    let phases_dir = Path::new(".phases");

    if !phases_dir.exists() {
        println!(
            "{} Roadmap non initialisée. Lance d'abord: {}",
            "Erreur:".red(),
            "roadmap init".yellow()
        );
        return;
    }

    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut phase_count = 0;
    let mut task_count = 0;
    let mut all_phase_ids: HashSet<String> = HashSet::new();
    let mut all_task_ids: HashSet<String> = HashSet::new();
    let mut phases: Vec<Phase> = Vec::new();

    let entries = match fs::read_dir(phases_dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("{} Impossible de lire .phases/: {}", "Erreur:".red(), e);
            return;
        }
    };

    let mut phase_files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_str().unwrap_or("");
            name.starts_with("phase-") && name.ends_with(".yml")
        })
        .collect();
    phase_files.sort_by_key(|e| e.file_name());

    for entry in &phase_files {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();

        // Test parsing YAML
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                errors.push(format!("{}: impossible de lire le fichier: {}", filename, e));
                continue;
            }
        };

        let phase: Phase = match serde_yaml::from_str(&content) {
            Ok(p) => p,
            Err(e) => {
                errors.push(format!("{}: YAML invalide: {}", filename, e));
                continue;
            }
        };

        phase_count += 1;

        // Verify file name matches phase ID
        let expected_filename = format!("phase-{}.yml", phase.id);
        if filename != expected_filename {
            errors.push(format!(
                "{}: le nom du fichier ne correspond pas à l'ID de la phase (id: {}, attendu: {})",
                filename, phase.id, expected_filename
            ));
        }

        // Check duplicate phase IDs
        if !all_phase_ids.insert(phase.id.clone()) {
            errors.push(format!("{}: ID de phase dupliqué: {}", filename, phase.id));
        }

        // Validate phase status
        if !VALID_STATUSES.contains(&phase.status.as_str()) {
            errors.push(format!(
                "Phase {}: statut invalide '{}' (attendu: {})",
                phase.id, phase.status, VALID_STATUSES.join(", ")
            ));
        }

        // Validate parent reference
        if let Some(ref parent_id) = phase.parent {
            let parent_file = phases_dir.join(format!("phase-{}.yml", parent_id));
            if !parent_file.exists() {
                errors.push(format!(
                    "Phase {}: phase parente '{}' n'existe pas",
                    phase.id, parent_id
                ));
            }
        }

        // Validate tasks
        let mut phase_task_ids: HashSet<String> = HashSet::new();
        for task in &phase.tasks {
            task_count += 1;

            // Check task ID prefix matches phase ID
            if !task.id.starts_with(&format!("{}.", phase.id)) {
                errors.push(format!(
                    "Phase {}: tâche '{}' n'a pas le bon préfixe (attendu: {}.X)",
                    phase.id, task.id, phase.id
                ));
            }

            // Check duplicate task IDs within phase
            if !phase_task_ids.insert(task.id.clone()) {
                errors.push(format!(
                    "Phase {}: ID de tâche dupliqué: {}",
                    phase.id, task.id
                ));
            }

            // Check global duplicate task IDs
            if !all_task_ids.insert(task.id.clone()) {
                errors.push(format!(
                    "ID de tâche dupliqué globalement: {}",
                    task.id
                ));
            }

            // Validate task status
            if !VALID_STATUSES.contains(&task.status.as_str()) {
                errors.push(format!(
                    "Tâche {}: statut invalide '{}' (attendu: {})",
                    task.id, task.status, VALID_STATUSES.join(", ")
                ));
            }

            // Validate parent task reference
            if let Some(ref parent_id) = task.parent {
                if !phase.tasks.iter().any(|t| t.id == *parent_id) {
                    errors.push(format!(
                        "Tâche {}: tâche parente '{}' n'existe pas dans la phase {}",
                        task.id, parent_id, phase.id
                    ));
                }
            }

            // Check done tasks have completed_at
            if task.status == "done" && task.completed_at.is_none() {
                warnings.push(format!(
                    "Tâche {}: statut 'done' mais pas de date completed_at",
                    task.id
                ));
            }
        }

        phases.push(phase);
    }

    // Validate cross-references: blocks/blocked_by
    for phase in &phases {
        for task in &phase.tasks {
            for blocked_id in &task.blocks {
                if !all_task_ids.contains(blocked_id) {
                    errors.push(format!(
                        "Tâche {}: bloque '{}' qui n'existe pas",
                        task.id, blocked_id
                    ));
                }
            }
            for blocker_id in &task.blocked_by {
                if !all_task_ids.contains(blocker_id) {
                    errors.push(format!(
                        "Tâche {}: bloquée par '{}' qui n'existe pas",
                        task.id, blocker_id
                    ));
                }
            }
        }

        // Validate depends_on references
        for dep_id in &phase.depends_on {
            if !all_phase_ids.contains(dep_id) {
                errors.push(format!(
                    "Phase {}: dépend de '{}' qui n'existe pas",
                    phase.id, dep_id
                ));
            }
        }
    }

    // Validate bugs
    let bug_store = crate::phase::BugStore::load();
    let bug_count = bug_store.bugs.len();
    for bug in &bug_store.bugs {
        if let Some(ref phase_id) = bug.phase {
            if !all_phase_ids.contains(phase_id) {
                errors.push(format!(
                    "Bug #{}: phase '{}' n'existe pas",
                    bug.id, phase_id
                ));
            }
        }
        let valid_severities = ["blocking", "major", "minor"];
        if !valid_severities.contains(&bug.severity.as_str()) {
            errors.push(format!(
                "Bug #{}: sévérité invalide '{}'",
                bug.id, bug.severity
            ));
        }
        let valid_statuses = ["open", "in_progress", "resolved", "wontfix"];
        if !valid_statuses.contains(&bug.status.as_str()) {
            errors.push(format!(
                "Bug #{}: statut invalide '{}'",
                bug.id, bug.status
            ));
        }
    }

    // Print results
    println!();
    println!("{}", "🩺 Diagnostic roadmap".bold());
    println!();
    println!(
        "  Fichiers analysés: {} phases, {} tâches, {} bugs",
        phase_count.to_string().cyan(),
        task_count.to_string().cyan(),
        bug_count.to_string().cyan()
    );
    println!();

    if errors.is_empty() && warnings.is_empty() {
        println!("  {} Aucun problème détecté", "✅".green());
    } else {
        if !errors.is_empty() {
            println!("  {} {} erreur(s):", "❌".red(), errors.len());
            for error in &errors {
                println!("    {} {}", "•".red(), error);
            }
            println!();
        }

        if !warnings.is_empty() {
            println!("  {} {} avertissement(s):", "⚠️".yellow(), warnings.len());
            for warning in &warnings {
                println!("    {} {}", "•".yellow(), warning);
            }
            println!();
        }
    }

    println!();
}
