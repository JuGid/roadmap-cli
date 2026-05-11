//! Export command

use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::config::Config;
use crate::phase::Phase;

pub fn cmd_export() {
    let phases_dir = Path::new(".phases");

    if !phases_dir.exists() {
        println!(
            "{} Roadmap non initialisée. Lance d'abord: {}",
            "Erreur:".red(),
            "roadmap init".yellow()
        );
        return;
    }

    let config_path = phases_dir.join("config.yml");
    let config: Config = if config_path.exists() {
        let content = fs::read_to_string(&config_path).unwrap_or_default();
        serde_yaml::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    };

    let entries = match fs::read_dir(phases_dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("{} {}", "Erreur:".red(), e);
            return;
        }
    };

    let mut phases: Vec<Phase> = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        if !filename.starts_with("phase-") || !filename.ends_with(".yml") {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let p: Phase = match serde_yaml::from_str(&content) {
            Ok(p) => p,
            Err(_) => continue,
        };

        phases.push(p);
    }

    phases.sort_by(|a, b| a.priority.cmp(&b.priority));

    let mut md = String::new();
    md.push_str(&format!("# {} - Roadmap\n\n", config.project.name));
    md.push_str(&format!("{}\n\n", config.project.description));
    md.push_str("---\n\n");
    md.push_str("## Phases\n\n");
    md.push_str("| Phase | Nom | Statut | Priorité | Progression |\n");
    md.push_str("|-------|-----|--------|----------|-------------|\n");

    for phase in &phases {
        let status_icon = match phase.status.as_str() {
            "done" => "✅",
            "in_progress" => "🔄",
            "blocked" => "🚫",
            _ => "⬜",
        };

        let total = phase.tasks.len();
        let done = phase.tasks.iter().filter(|t| t.status == "done").count();
        let progress = if total > 0 {
            format!("{}/{}", done, total)
        } else {
            "-".to_string()
        };

        md.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            phase.id, phase.name, status_icon, phase.priority, progress
        ));
    }

    md.push_str("\n---\n\n");

    for phase in &phases {
        let status_icon = match phase.status.as_str() {
            "done" => "✅",
            "in_progress" => "🔄",
            "blocked" => "🚫",
            _ => "⬜",
        };

        md.push_str(&format!("## Phase {} — {} {}\n\n", phase.id, phase.name, status_icon));

        if !phase.description.is_empty() {
            md.push_str(&format!("{}\n\n", phase.description));
        }

        if !phase.tasks.is_empty() {
            md.push_str("### Tâches\n\n");
            for task in &phase.tasks {
                if task.parent.is_some() {
                    continue;
                }

                let task_icon = match task.status.as_str() {
                    "done" => "✅",
                    "in_progress" => "🔄",
                    "blocked" => "🚫",
                    _ => "⬜",
                };

                let optional = if task.optional { " *(optionnel)*" } else { "" };
                md.push_str(&format!("- {} **{}** — {}{}\n", task_icon, task.id, task.name, optional));

                for subtask in &phase.tasks {
                    if subtask.parent.as_ref() == Some(&task.id) {
                        let sub_icon = match subtask.status.as_str() {
                            "done" => "✅",
                            "in_progress" => "🔄",
                            "blocked" => "🚫",
                            _ => "⬜",
                        };
                        md.push_str(&format!("  - {} {} — {}\n", sub_icon, subtask.id, subtask.name));
                    }
                }
            }
            md.push_str("\n");
        }

        if !phase.notes.is_empty() {
            md.push_str("### Notes\n\n");
            for note in &phase.notes {
                md.push_str(&format!("- **{}** : {}\n", note.date, note.content));
            }
            md.push_str("\n");
        }

        md.push_str("---\n\n");
    }

    let roadmap_path = Path::new(&config.export.roadmap_path);
    if let Err(e) = fs::write(roadmap_path, &md) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!("{} Roadmap exportée", "✓".green());
    println!("  Fichier: {}", config.export.roadmap_path.cyan());
    println!("  {} phases exportées", phases.len());
}
