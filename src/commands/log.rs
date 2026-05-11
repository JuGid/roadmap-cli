//! Log command - shows history of completed tasks

use colored::Colorize;
use crate::utils::load_phases;

pub fn cmd_log(limit: usize, json: bool) {
    let phases = match load_phases() {
        Some(p) => p,
        None => return,
    };

    // Collect all tasks with a completed_at date
    let mut events: Vec<serde_json::Value> = Vec::new();

    for phase in &phases {
        // Phase completion (inferred from all tasks done)
        let all_done = !phase.tasks.is_empty()
            && phase.tasks.iter().filter(|t| !t.optional).all(|t| t.status == "done");

        if phase.status == "done" {
            // Use the latest completed_at from tasks, or updated_at
            let date = phase.tasks.iter()
                .filter_map(|t| t.completed_at.as_ref())
                .max()
                .unwrap_or(&phase.updated_at);

            events.push(serde_json::json!({
                "type": "phase_done",
                "date": date,
                "phase_id": phase.id,
                "phase_name": phase.name,
            }));
        } else if all_done && !phase.tasks.is_empty() {
            // Edge case: all tasks done but phase not marked done
            let date = phase.tasks.iter()
                .filter_map(|t| t.completed_at.as_ref())
                .max()
                .unwrap_or(&phase.updated_at);

            events.push(serde_json::json!({
                "type": "phase_all_tasks_done",
                "date": date,
                "phase_id": phase.id,
                "phase_name": phase.name,
            }));
        }

        for task in &phase.tasks {
            if let Some(ref completed) = task.completed_at {
                events.push(serde_json::json!({
                    "type": "task_done",
                    "date": completed,
                    "task_id": task.id,
                    "task_name": task.name,
                    "phase_id": phase.id,
                    "phase_name": phase.name,
                }));
            }
        }

        for note in &phase.notes {
            events.push(serde_json::json!({
                "type": "note",
                "date": note.date,
                "phase_id": phase.id,
                "phase_name": phase.name,
                "content": note.content,
            }));
        }
    }

    // Bug events
    let bug_store = crate::phase::BugStore::load();
    for bug in &bug_store.bugs {
        events.push(serde_json::json!({
            "type": "bug_created",
            "date": bug.created_at,
            "bug_id": bug.id,
            "bug_title": bug.title,
            "severity": bug.severity,
        }));
        if let Some(ref resolved) = bug.resolved_at {
            events.push(serde_json::json!({
                "type": "bug_resolved",
                "date": resolved,
                "bug_id": bug.id,
                "bug_title": bug.title,
                "resolution": bug.resolution,
            }));
        }
    }

    // Sort by date descending
    events.sort_by(|a, b| {
        let da = a["date"].as_str().unwrap_or("");
        let db = b["date"].as_str().unwrap_or("");
        db.cmp(da)
    });

    events.truncate(limit);

    if json {
        let output = serde_json::to_string_pretty(&events).expect("Erreur JSON");
        println!("{}", output);
        return;
    }

    if events.is_empty() {
        println!("Aucun historique trouvé.");
        return;
    }

    println!("{}", "📜 Historique".bold());
    println!();

    let mut current_date = String::new();

    for event in &events {
        let date = event["date"].as_str().unwrap_or("?");

        if date != current_date {
            if !current_date.is_empty() {
                println!();
            }
            println!("  {}", date.bold());
            current_date = date.to_string();
        }

        match event["type"].as_str().unwrap() {
            "phase_done" => {
                println!(
                    "    {} Phase {} — {} terminée",
                    "✅",
                    event["phase_id"].as_str().unwrap().cyan(),
                    event["phase_name"].as_str().unwrap()
                );
            }
            "phase_all_tasks_done" => {
                println!(
                    "    {} Phase {} — {} (toutes tâches terminées)",
                    "🏁",
                    event["phase_id"].as_str().unwrap().cyan(),
                    event["phase_name"].as_str().unwrap()
                );
            }
            "task_done" => {
                println!(
                    "    {} {} — {}",
                    "✓".green(),
                    event["task_id"].as_str().unwrap().cyan(),
                    event["task_name"].as_str().unwrap()
                );
            }
            "note" => {
                let content = event["content"].as_str().unwrap();
                let truncated = if content.len() > 80 {
                    format!("{}...", &content[..80])
                } else {
                    content.to_string()
                };
                println!(
                    "    {} Phase {} — {}",
                    "📝",
                    event["phase_id"].as_str().unwrap().cyan(),
                    truncated.dimmed()
                );
            }
            "bug_created" => {
                let sev = match event["severity"].as_str().unwrap_or("") {
                    "blocking" => "🔴",
                    "major" => "🟠",
                    _ => "🟡",
                };
                println!(
                    "    {} {} Bug #{} — {}",
                    "🐛",
                    sev,
                    event["bug_id"],
                    event["bug_title"].as_str().unwrap()
                );
            }
            "bug_resolved" => {
                println!(
                    "    {} Bug #{} résolu — {}",
                    "🔧".green(),
                    event["bug_id"],
                    event["bug_title"].as_str().unwrap()
                );
            }
            _ => {}
        }
    }

    println!();
}
