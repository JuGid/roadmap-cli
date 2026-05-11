//! Show phase details

use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::phase::{Phase, Task};

pub fn cmd_show(id: String, json: bool) {
    let phases_dir = Path::new(".phases");

    if !phases_dir.exists() {
        if json {
            println!(r#"{{"error": "Roadmap non initialisée"}}"#);
        } else {
            println!(
                "{} Roadmap non initialisée. Lance d'abord: {}",
                "Erreur:".red(),
                "roadmap init".yellow()
            );
        }
        return;
    }

    let phase_file = phases_dir.join(format!("phase-{}.yml", id));
    if !phase_file.exists() {
        if json {
            println!(r#"{{"error": "Phase non trouvée", "id": "{}"}}"#, id);
        } else {
            println!("{} Phase {} non trouvée", "Erreur:".red(), id.yellow());
        }
        return;
    }

    let content = match fs::read_to_string(&phase_file) {
        Ok(c) => c,
        Err(e) => {
            if json {
                println!(r#"{{"error": "{}"}}"#, e);
            } else {
                println!("{} {}", "Erreur:".red(), e);
            }
            return;
        }
    };

    let phase: Phase = match serde_yaml::from_str(&content) {
        Ok(p) => p,
        Err(e) => {
            if json {
                println!(r#"{{"error": "YAML invalide: {}"}}"#, e);
            } else {
                println!("{} YAML invalide: {}", "Erreur:".red(), e);
            }
            return;
        }
    };

    if json {
        let output = serde_json::to_string_pretty(&phase).expect("Erreur sérialisation JSON");
        println!("{}", output);
        return;
    }

    let status_icon = match phase.status.as_str() {
        "done" => "✅",
        "in_progress" => "🔄",
        "blocked" => "🚫",
        _ => "⬜",
    };

    println!();
    println!(
        "{} {} - {}",
        status_icon,
        format!("Phase {}", phase.id).cyan().bold(),
        phase.name.bold()
    );

    if !phase.description.is_empty() {
        println!("  {}", phase.description.dimmed());
    }

    println!();
    println!("  Priorité:   {}", phase.priority);
    println!("  Statut:     {}", phase.status);
    if let Some(ref parent) = phase.parent {
        println!("  Parent:     {}", parent);
    }
    if !phase.depends_on.is_empty() {
        println!("  Dépend de:  {}", phase.depends_on.join(", "));
    }
    println!("  Créée le:   {}", phase.created_at);
    println!("  Modifiée:   {}", phase.updated_at);

    if !phase.tasks.is_empty() {
        println!();
        println!("  {}:", "Tâches".bold());
        print_tasks(&phase.tasks, None, 4);
    }

    if !phase.notes.is_empty() {
        println!();
        println!("  {}:", "Notes".bold());
        for note in &phase.notes {
            println!("    {} - {}", note.date.dimmed(), note.content);
        }
    }

    // Bugs liés à cette phase
    let bug_store = crate::phase::BugStore::load();
    let phase_bugs: Vec<_> = bug_store.bugs.iter()
        .filter(|b| b.phase.as_ref() == Some(&id))
        .collect();
    if !phase_bugs.is_empty() {
        println!();
        println!("  {}:", "Bugs".bold());
        for bug in phase_bugs {
            let sev_icon = match bug.severity.as_str() {
                "blocking" => "🔴",
                "major" => "🟠",
                _ => "🟡",
            };
            let status_icon = match bug.status.as_str() {
                "resolved" => "✅",
                "in_progress" => "🔄",
                "wontfix" => "⏭️",
                _ => "⬜",
            };
            println!(
                "    {} {} #{} [{}] {}",
                status_icon, sev_icon,
                bug.id.to_string().cyan(),
                bug.status,
                bug.title
            );
        }
    }

    println!();
}

fn print_tasks(tasks: &[Task], parent: Option<&str>, indent: usize) {
    let spaces = " ".repeat(indent);

    for task in tasks {
        let task_parent = task.parent.as_deref();
        if task_parent != parent {
            continue;
        }

        let task_icon = match task.status.as_str() {
            "done" => "✅",
            "in_progress" => "🔄",
            "blocked" => "🚫",
            _ => "⬜",
        };

        let optional_tag = if task.optional {
            " (optionnel)".dimmed().to_string()
        } else {
            String::new()
        };

        let stage_info = match &task.workflow_stage {
            Some(stage) => format!(" [{}]", stage).dimmed().to_string(),
            None => String::new(),
        };

        let blocked_info = if !task.blocked_by.is_empty() {
            format!(" (bloqué par: {})", task.blocked_by.join(", ")).red().to_string()
        } else {
            String::new()
        };

        let blocks_info = if !task.blocks.is_empty() {
            format!(" → bloque: {}", task.blocks.join(", ")).yellow().to_string()
        } else {
            String::new()
        };

        let files_info = if !task.files.is_empty() {
            format!(" 📁 {}", task.files.join(", ")).dimmed().to_string()
        } else {
            String::new()
        };

        let tags_info = if !task.tags.is_empty() {
            format!(" 🏷️ {}", task.tags.join(", ")).dimmed().to_string()
        } else {
            String::new()
        };

        let assignee_info = match &task.assignee {
            Some(a) => format!(" @{}", a).dimmed().to_string(),
            None => String::new(),
        };

        let due_info = match &task.due {
            Some(d) => {
                let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                if d.as_str() < today.as_str() && task.status != "done" {
                    format!(" ⚠️ {}", d).red().to_string()
                } else {
                    format!(" 📅 {}", d).dimmed().to_string()
                }
            }
            None => String::new(),
        };

        println!(
            "{}{} {} - {}{}{}{}{}{}{}",
            spaces, task_icon, task.id.cyan(), task.name, stage_info, optional_tag, assignee_info, due_info, blocked_info, blocks_info
        );

        if !task.files.is_empty() {
            println!("{}      {}", spaces, files_info);
        }
        if !task.tags.is_empty() {
            println!("{}      {}", spaces, tags_info);
        }

        print_tasks(tasks, Some(&task.id), indent + 2);
    }
}
