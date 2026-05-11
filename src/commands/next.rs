//! Next task command

use std::collections::HashSet;
use colored::Colorize;
use crate::phase::{Phase, Task};
use crate::utils::load_phases;

pub fn cmd_next(json: bool) {
    let phases = match load_phases() {
        Some(p) => p,
        None => return,
    };

    let done_tasks: HashSet<String> = phases
        .iter()
        .flat_map(|p| p.tasks.iter())
        .filter(|t| t.status == "done")
        .map(|t| t.id.clone())
        .collect();

    let is_blocked = |task: &Task| -> bool {
        !task.blocked_by.is_empty() && !task.blocked_by.iter().all(|id| done_tasks.contains(id))
    };

    let mut next_tasks: Vec<(&Phase, &Task)> = Vec::new();

    // Priority 1: Tasks in_progress (not blocked)
    for phase in &phases {
        for task in &phase.tasks {
            if task.status == "in_progress" && !is_blocked(task) {
                next_tasks.push((phase, task));
            }
        }
    }

    // Priority 2: Pending tasks in in_progress phases (not blocked)
    for phase in &phases {
        if phase.status == "in_progress" {
            for task in &phase.tasks {
                if task.status == "pending" && !task.optional && !is_blocked(task) {
                    next_tasks.push((phase, task));
                }
            }
        }
    }

    // Priority 3: First pending tasks in pending phases (by priority, not blocked)
    for phase in &phases {
        if phase.status == "pending" {
            for task in &phase.tasks {
                if task.status == "pending" && !task.optional && !is_blocked(task) {
                    next_tasks.push((phase, task));
                    break;
                }
            }
        }
    }

    if next_tasks.is_empty() {
        if json {
            println!("null");
        } else {
            println!("{} Aucune tâche à faire !", "✓".green());
        }
        return;
    }

    if json {
        #[derive(serde::Serialize)]
        struct NextTask {
            task_id: String,
            task_name: String,
            phase_id: String,
            phase_name: String,
            status: String,
            priority: u32,
        }

        let tasks: Vec<NextTask> = next_tasks
            .iter()
            .map(|(phase, task)| NextTask {
                task_id: task.id.clone(),
                task_name: task.name.clone(),
                phase_id: phase.id.clone(),
                phase_name: phase.name.clone(),
                status: task.status.clone(),
                priority: phase.priority,
            })
            .collect();

        let output = serde_json::to_string_pretty(&tasks).expect("Erreur JSON");
        println!("{}", output);
    } else {
        println!("{}", "Prochaines tâches:".bold());
        println!();

        for (i, (phase, task)) in next_tasks.iter().take(5).enumerate() {
            let status_icon = match task.status.as_str() {
                "in_progress" => "🔄",
                _ => "⬜",
            };

            let marker = if i == 0 { "→".green().bold() } else { " ".normal() };

            println!(
                " {} {} {} — {}",
                marker,
                status_icon,
                task.id.cyan(),
                task.name
            );
            println!(
                "      Phase {} (P{}) — {}",
                phase.id, phase.priority, phase.name.dimmed()
            );
            println!();
        }

        if next_tasks.len() > 5 {
            println!("   ... et {} autres tâches", next_tasks.len() - 5);
        }

        if let Some((_, task)) = next_tasks.first() {
            if task.status == "pending" {
                println!(
                    "Démarrer: {}",
                    format!("roadmap task start {}", task.id).yellow()
                );
            }
        }
    }
}
