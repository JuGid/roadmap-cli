//! Search command - find keywords across phases and tasks

use colored::Colorize;
use crate::utils::{load_phases, get_status_icon};

pub fn cmd_search(query: String, json: bool) {
    let phases = match load_phases() {
        Some(p) => p,
        None => return,
    };

    let query_lower = query.to_lowercase();
    let mut results: Vec<serde_json::Value> = Vec::new();

    for phase in &phases {
        let phase_matches = phase.name.to_lowercase().contains(&query_lower)
            || phase.description.to_lowercase().contains(&query_lower);

        if phase_matches {
            results.push(serde_json::json!({
                "type": "phase",
                "id": phase.id,
                "name": phase.name,
                "status": phase.status,
                "priority": phase.priority,
            }));
        }

        for task in &phase.tasks {
            let task_matches = task.name.to_lowercase().contains(&query_lower)
                || task.description.as_ref().is_some_and(|d| d.to_lowercase().contains(&query_lower))
                || task.tags.iter().any(|t| t.to_lowercase().contains(&query_lower));

            if task_matches {
                results.push(serde_json::json!({
                    "type": "task",
                    "id": task.id,
                    "name": task.name,
                    "status": task.status,
                    "phase_id": phase.id,
                    "phase_name": phase.name,
                    "optional": task.optional,
                    "tags": task.tags,
                }));
            }
        }

        for note in &phase.notes {
            if note.content.to_lowercase().contains(&query_lower) {
                results.push(serde_json::json!({
                    "type": "note",
                    "phase_id": phase.id,
                    "phase_name": phase.name,
                    "date": note.date,
                    "content": note.content,
                }));
            }
        }
    }

    if json {
        let output = serde_json::to_string_pretty(&results).expect("Erreur JSON");
        println!("{}", output);
        return;
    }

    if results.is_empty() {
        println!("Aucun résultat pour {}", query.yellow());
        return;
    }

    println!(
        "{} {} résultat(s) pour {}:",
        "🔍",
        results.len().to_string().cyan(),
        query.yellow().bold()
    );
    println!();

    for item in &results {
        match item["type"].as_str().unwrap() {
            "phase" => {
                let icon = get_status_icon(item["status"].as_str().unwrap_or("pending"));
                println!(
                    "  {} Phase {} — {} (P{})",
                    icon,
                    item["id"].as_str().unwrap().cyan(),
                    item["name"].as_str().unwrap(),
                    item["priority"].as_u64().unwrap()
                );
            }
            "task" => {
                let icon = get_status_icon(item["status"].as_str().unwrap_or("pending"));
                let optional = if item["optional"].as_bool().unwrap_or(false) { " (opt)" } else { "" };
                println!(
                    "  {} {} — {}{}",
                    icon,
                    item["id"].as_str().unwrap().cyan(),
                    item["name"].as_str().unwrap(),
                    optional
                );
                println!(
                    "      Phase {} — {}",
                    item["phase_id"].as_str().unwrap().dimmed(),
                    item["phase_name"].as_str().unwrap().dimmed()
                );
            }
            "note" => {
                println!(
                    "  {} Note dans phase {} ({})",
                    "📝",
                    item["phase_id"].as_str().unwrap().cyan(),
                    item["date"].as_str().unwrap().dimmed()
                );
                let content = item["content"].as_str().unwrap();
                let truncated = if content.len() > 100 {
                    format!("{}...", &content[..100])
                } else {
                    content.to_string()
                };
                println!("      {}", truncated.dimmed());
            }
            _ => {}
        }
    }
}
