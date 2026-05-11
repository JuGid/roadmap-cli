//! Context command for LLM

use crate::utils::load_phases;

pub fn cmd_context(include_done: bool, phase_filter: Option<String>) {
    let phases = match load_phases() {
        Some(p) => p,
        None => return,
    };

    // If --phase is specified, show focused context for that phase only
    if let Some(ref phase_id) = phase_filter {
        cmd_context_phase(&phases, phase_id, include_done);
        return;
    }

    println!("# Contexte Roadmap\n");

    // Summary
    let total_phases = phases.len();
    let phases_done = phases.iter().filter(|p| p.status == "done").count();
    let phases_in_progress: Vec<_> = phases.iter().filter(|p| p.status == "in_progress").collect();

    let mut total_tasks = 0;
    let mut tasks_done = 0;
    let mut tasks_in_progress = 0;

    for phase in &phases {
        for task in &phase.tasks {
            total_tasks += 1;
            match task.status.as_str() {
                "done" => tasks_done += 1,
                "in_progress" => tasks_in_progress += 1,
                _ => {}
            }
        }
    }

    println!("## Résumé\n");
    println!("- Phases: {}/{} terminées", phases_done, total_phases);
    println!("- Tâches: {}/{} terminées", tasks_done, total_tasks);
    println!("- En cours: {} tâches", tasks_in_progress);
    println!();

    // Current focus
    if !phases_in_progress.is_empty() {
        println!("## Focus actuel\n");
        for phase in &phases_in_progress {
            println!("### Phase {} — {}\n", phase.id, phase.name);

            if !phase.description.is_empty() {
                println!("{}\n", phase.description);
            }

            let in_progress: Vec<_> = phase.tasks.iter().filter(|t| t.status == "in_progress").collect();
            if !in_progress.is_empty() {
                println!("**En cours:**");
                for task in in_progress {
                    println!("- [{}] {}", task.id, task.name);
                    if !task.files.is_empty() {
                        println!("  Fichiers: {}", task.files.join(", "));
                    }
                }
                println!();
            }

            let pending: Vec<_> = phase.tasks.iter().filter(|t| t.status == "pending" && !t.optional).collect();
            if !pending.is_empty() {
                println!("**À faire:**");
                for task in pending {
                    println!("- [{}] {}", task.id, task.name);
                    if !task.files.is_empty() {
                        println!("  Fichiers: {}", task.files.join(", "));
                    }
                }
                println!();
            }

            if !phase.notes.is_empty() {
                println!("**Notes récentes:**");
                for note in phase.notes.iter().rev().take(3) {
                    println!("- {}: {}", note.date, note.content);
                }
                println!();
            }
        }
    }

    // Upcoming phases
    let pending_phases: Vec<_> = phases.iter().filter(|p| p.status == "pending").take(3).collect();
    if !pending_phases.is_empty() {
        println!("## Phases à venir\n");
        for phase in pending_phases {
            let task_count = phase.tasks.len();
            println!("- **Phase {}** (P{}) — {} ({} tâches)", phase.id, phase.priority, phase.name, task_count);
        }
        println!();
    }

    // Completed phases
    if include_done {
        let done_phases: Vec<_> = phases.iter().filter(|p| p.status == "done").collect();
        if !done_phases.is_empty() {
            println!("## Phases terminées\n");
            for phase in done_phases {
                println!("- ✅ Phase {} — {}", phase.id, phase.name);
            }
            println!();
        }
    }

    // Action items
    println!("## Actions suggérées\n");
    println!("```bash");
    println!("# Voir la prochaine tâche");
    println!("roadmap next");
    println!();
    println!("# Démarrer une tâche");
    println!("roadmap task start <id>");
    println!();
    println!("# Marquer comme terminée");
    println!("roadmap task done <id>");
    println!("```");
}

fn cmd_context_phase(phases: &[crate::phase::Phase], phase_id: &str, include_done: bool) {
    let phase = match phases.iter().find(|p| p.id == phase_id) {
        Some(p) => p,
        None => {
            println!("Phase {} non trouvée", phase_id);
            return;
        }
    };

    println!("# Contexte — Phase {} — {}\n", phase.id, phase.name);

    if !phase.description.is_empty() {
        println!("{}\n", phase.description);
    }

    println!("- Statut: {}", phase.status);
    println!("- Priorité: {}", phase.priority);
    if !phase.depends_on.is_empty() {
        println!("- Dépend de: {}", phase.depends_on.join(", "));
    }

    let total = phase.tasks.len();
    let done = phase.tasks.iter().filter(|t| t.status == "done").count();
    println!("- Progression: {}/{} tâches", done, total);
    println!();

    let in_progress: Vec<_> = phase.tasks.iter().filter(|t| t.status == "in_progress").collect();
    if !in_progress.is_empty() {
        println!("## En cours\n");
        for task in &in_progress {
            print_task_detail(task);
        }
    }

    let pending: Vec<_> = phase.tasks.iter().filter(|t| t.status == "pending" && !t.optional).collect();
    if !pending.is_empty() {
        println!("## À faire\n");
        for task in &pending {
            print_task_detail(task);
        }
    }

    let optional: Vec<_> = phase.tasks.iter().filter(|t| t.optional && t.status != "done").collect();
    if !optional.is_empty() {
        println!("## Optionnel\n");
        for task in &optional {
            print_task_detail(task);
        }
    }

    let blocked: Vec<_> = phase.tasks.iter().filter(|t| t.status == "blocked").collect();
    if !blocked.is_empty() {
        println!("## Bloqué\n");
        for task in &blocked {
            print_task_detail(task);
        }
    }

    if include_done {
        let done_tasks: Vec<_> = phase.tasks.iter().filter(|t| t.status == "done").collect();
        if !done_tasks.is_empty() {
            println!("## Terminé\n");
            for task in &done_tasks {
                println!("- [{}] ✅ {}", task.id, task.name);
            }
            println!();
        }
    }

    if !phase.notes.is_empty() {
        println!("## Notes\n");
        for note in &phase.notes {
            println!("- {}: {}", note.date, note.content);
        }
        println!();
    }
}

fn print_task_detail(task: &crate::phase::Task) {
    println!("- [{}] {}", task.id, task.name);
    if let Some(ref desc) = task.description {
        println!("  {}", desc);
    }
    if !task.files.is_empty() {
        println!("  Fichiers: {}", task.files.join(", "));
    }
    if !task.tags.is_empty() {
        println!("  Tags: {}", task.tags.join(", "));
    }
    if !task.blocked_by.is_empty() {
        println!("  Bloqué par: {}", task.blocked_by.join(", "));
    }
}
