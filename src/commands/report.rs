//! Report command

use colored::Colorize;
use crate::phase::Phase;
use crate::utils::load_phases;

pub fn cmd_report(json: bool) {
    let phases = match load_phases() {
        Some(p) => p,
        None => return,
    };

    if phases.is_empty() {
        if json {
            println!("{{}}");
        } else {
            println!("Aucune phase trouvée.");
        }
        return;
    }

    #[derive(serde::Serialize)]
    struct TaskInfo {
        id: String,
        name: String,
        phase_id: String,
        phase_name: String,
        status: String,
        optional: bool,
        workflow_stage: Option<String>,
    }

    let mut all_tasks: Vec<TaskInfo> = Vec::new();
    let mut phases_in_progress: Vec<&Phase> = Vec::new();
    let mut phases_pending: Vec<&Phase> = Vec::new();
    let mut phases_done: Vec<&Phase> = Vec::new();
    let mut phases_blocked: Vec<&Phase> = Vec::new();

    for phase in &phases {
        match phase.status.as_str() {
            "in_progress" => phases_in_progress.push(phase),
            "done" => phases_done.push(phase),
            "blocked" => phases_blocked.push(phase),
            _ => phases_pending.push(phase),
        }

        for task in &phase.tasks {
            all_tasks.push(TaskInfo {
                id: task.id.clone(),
                name: task.name.clone(),
                phase_id: phase.id.clone(),
                phase_name: phase.name.clone(),
                status: task.status.clone(),
                optional: task.optional,
                workflow_stage: task.workflow_stage.clone(),
            });
        }
    }

    let tasks_done: Vec<_> = all_tasks.iter().filter(|t| t.status == "done").collect();
    let tasks_in_progress: Vec<_> = all_tasks.iter().filter(|t| t.status == "in_progress").collect();
    let tasks_pending: Vec<_> = all_tasks.iter().filter(|t| t.status == "pending" && !t.optional).collect();
    let tasks_optional: Vec<_> = all_tasks.iter().filter(|t| t.optional && t.status != "done").collect();
    let tasks_blocked: Vec<_> = all_tasks.iter().filter(|t| t.status == "blocked").collect();

    if json {
        #[derive(serde::Serialize)]
        struct Report {
            summary: Summary,
            phases_in_progress: Vec<PhaseInfo>,
            next_tasks: Vec<TaskInfo>,
            optional_tasks: Vec<TaskInfo>,
            blocked: Blocked,
        }

        #[derive(serde::Serialize)]
        struct Summary {
            total_phases: usize,
            phases_done: usize,
            phases_in_progress: usize,
            phases_pending: usize,
            phases_blocked: usize,
            total_tasks: usize,
            tasks_done: usize,
            tasks_in_progress: usize,
            tasks_pending: usize,
            tasks_optional: usize,
            tasks_blocked: usize,
            progress_percent: f32,
        }

        #[derive(serde::Serialize)]
        struct PhaseInfo {
            id: String,
            name: String,
            priority: u32,
            progress: String,
        }

        #[derive(serde::Serialize)]
        struct Blocked {
            phases: Vec<String>,
            tasks: Vec<TaskInfo>,
        }

        let total_required = tasks_done.len() + tasks_in_progress.len() + tasks_pending.len() + tasks_blocked.len();
        let progress_percent = if total_required > 0 {
            (tasks_done.len() as f32 / total_required as f32) * 100.0
        } else {
            0.0
        };

        let report = Report {
            summary: Summary {
                total_phases: phases.len(),
                phases_done: phases_done.len(),
                phases_in_progress: phases_in_progress.len(),
                phases_pending: phases_pending.len(),
                phases_blocked: phases_blocked.len(),
                total_tasks: all_tasks.len(),
                tasks_done: tasks_done.len(),
                tasks_in_progress: tasks_in_progress.len(),
                tasks_pending: tasks_pending.len(),
                tasks_optional: tasks_optional.len(),
                tasks_blocked: tasks_blocked.len(),
                progress_percent,
            },
            phases_in_progress: phases_in_progress
                .iter()
                .map(|p| {
                    let total = p.tasks.len();
                    let done = p.tasks.iter().filter(|t| t.status == "done").count();
                    PhaseInfo {
                        id: p.id.clone(),
                        name: p.name.clone(),
                        priority: p.priority,
                        progress: format!("{}/{}", done, total),
                    }
                })
                .collect(),
            next_tasks: tasks_pending
                .iter()
                .take(10)
                .map(|t| TaskInfo {
                    id: t.id.clone(),
                    name: t.name.clone(),
                    phase_id: t.phase_id.clone(),
                    phase_name: t.phase_name.clone(),
                    status: t.status.clone(),
                    optional: t.optional,
                    workflow_stage: t.workflow_stage.clone(),
                })
                .collect(),
            optional_tasks: tasks_optional
                .iter()
                .map(|t| TaskInfo {
                    id: t.id.clone(),
                    name: t.name.clone(),
                    phase_id: t.phase_id.clone(),
                    phase_name: t.phase_name.clone(),
                    status: t.status.clone(),
                    optional: t.optional,
                    workflow_stage: t.workflow_stage.clone(),
                })
                .collect(),
            blocked: Blocked {
                phases: phases_blocked.iter().map(|p| format!("{} — {}", p.id, p.name)).collect(),
                tasks: tasks_blocked
                    .iter()
                    .map(|t| TaskInfo {
                        id: t.id.clone(),
                        name: t.name.clone(),
                        phase_id: t.phase_id.clone(),
                        phase_name: t.phase_name.clone(),
                        status: t.status.clone(),
                        optional: t.optional,
                        workflow_stage: t.workflow_stage.clone(),
                    })
                    .collect(),
            },
        };

        let output = serde_json::to_string_pretty(&report).expect("Erreur sérialisation JSON");
        println!("{}", output);
    } else {
        println!();
        println!("{}", "═══════════════════════════════════════════════════════════".dimmed());
        println!("{}", "                    📊 RAPPORT DE PROGRESSION              ".bold());
        println!("{}", "═══════════════════════════════════════════════════════════".dimmed());
        println!();

        let total_required = tasks_done.len() + tasks_in_progress.len() + tasks_pending.len() + tasks_blocked.len();
        let progress_percent = if total_required > 0 {
            (tasks_done.len() as f32 / total_required as f32) * 100.0
        } else {
            0.0
        };

        println!("{}", "📈 RÉSUMÉ".bold().cyan());
        println!("   Phases: {} terminées / {} total",
            phases_done.len().to_string().green(),
            phases.len()
        );
        println!("   Tâches: {} / {} ({:.0}%)",
            tasks_done.len().to_string().green(),
            total_required,
            progress_percent
        );

        let bar_width = 30;
        let filled = ((progress_percent / 100.0) * bar_width as f32) as usize;
        let bar = format!(
            "[{}{}]",
            "█".repeat(filled).green(),
            "░".repeat(bar_width - filled).dimmed()
        );
        println!("   {}", bar);
        println!();

        if !phases_in_progress.is_empty() {
            println!("{}", "🔄 PHASES EN COURS".bold().yellow());
            for phase in &phases_in_progress {
                let total = phase.tasks.len();
                let done = phase.tasks.iter().filter(|t| t.status == "done").count();
                println!(
                    "   [P{}] {} — {} ({}/{})",
                    phase.priority,
                    phase.id.cyan(),
                    phase.name,
                    done,
                    total
                );
            }
            println!();
        }

        if !tasks_in_progress.is_empty() {
            println!("{}", "⚡ EN COURS MAINTENANT".bold().yellow());
            for task in &tasks_in_progress {
                let stage = task.workflow_stage.as_ref().map(|s| format!(" [{}]", s)).unwrap_or_default();
                println!(
                    "   {} {} — {}{}",
                    task.id.cyan(),
                    task.name,
                    task.phase_name.dimmed(),
                    stage.dimmed()
                );
            }
            println!();
        }

        if !tasks_pending.is_empty() {
            println!("{}", "📋 PROCHAINES TÂCHES".bold());
            for task in tasks_pending.iter().take(10) {
                println!(
                    "   ⬜ {} {} — {}",
                    task.id.cyan(),
                    task.name,
                    task.phase_name.dimmed()
                );
            }
            if tasks_pending.len() > 10 {
                println!("   ... et {} autres", tasks_pending.len() - 10);
            }
            println!();
        }

        if !tasks_optional.is_empty() {
            println!("{}", "💡 OPTIONNEL (mise de côté)".bold().blue());
            for task in &tasks_optional {
                println!(
                    "   ◇ {} {} — {}",
                    task.id.cyan(),
                    task.name,
                    task.phase_name.dimmed()
                );
            }
            println!();
        }

        if !phases_blocked.is_empty() || !tasks_blocked.is_empty() {
            println!("{}", "🚫 BLOQUÉ".bold().red());
            for phase in &phases_blocked {
                println!("   Phase {} — {}", phase.id.red(), phase.name);
            }
            for task in &tasks_blocked {
                println!(
                    "   {} {} — {}",
                    task.id.red(),
                    task.name,
                    task.phase_name.dimmed()
                );
            }
            println!();
        }

        // Bugs section
        {
            let bug_store = crate::phase::BugStore::load();
            let open_bugs: Vec<_> = bug_store.bugs.iter()
                .filter(|b| b.status == "open" || b.status == "in_progress")
                .collect();
            if !open_bugs.is_empty() {
                let blocking = open_bugs.iter().filter(|b| b.severity == "blocking").count();
                let major = open_bugs.iter().filter(|b| b.severity == "major").count();
                let minor = open_bugs.iter().filter(|b| b.severity == "minor").count();

                println!("{}", "🐛 BUGS".bold().red());
                println!(
                    "   {} bloquant(s), {} majeur(s), {} mineur(s)",
                    blocking, major, minor
                );
                for bug in open_bugs.iter().take(5) {
                    let icon = match bug.severity.as_str() {
                        "blocking" => "🔴",
                        "major" => "🟠",
                        _ => "🟡",
                    };
                    let status_icon = match bug.status.as_str() {
                        "in_progress" => "🔄",
                        _ => "⬜",
                    };
                    let phase_info = bug.phase.as_ref()
                        .map(|p| format!(" — phase {}", p))
                        .unwrap_or_default();
                    println!(
                        "   {} {} #{} {}{}",
                        status_icon, icon,
                        bug.id.to_string().cyan(),
                        bug.title,
                        phase_info.dimmed()
                    );
                }
                if open_bugs.len() > 5 {
                    println!("   ... et {} autres", open_bugs.len() - 5);
                }
                println!();
            }
        }

        // Features section
        {
            let store = crate::phase::FeatureStore::load();
            let open_features: Vec<_> = store.features.iter()
                .filter(|f| f.status == "proposed" || f.status == "accepted" || f.status == "in_progress")
                .collect();
            if !open_features.is_empty() {
                println!("{}", "💡 FEATURES DEMANDÉES".bold().blue());
                for f in open_features.iter().take(5) {
                    let icon = match f.priority.as_str() {
                        "critical" => "🔴",
                        "high" => "🟠",
                        "medium" => "🟡",
                        _ => "🟢",
                    };
                    let status_icon = match f.status.as_str() {
                        "accepted" => "👍",
                        "in_progress" => "🔄",
                        _ => "💡",
                    };
                    let requester = f.requested_by.as_ref()
                        .map(|r| format!(" (par {})", r))
                        .unwrap_or_default();
                    println!(
                        "   {} {} #{} {}{}",
                        status_icon, icon,
                        f.id.to_string().cyan(),
                        f.title,
                        requester.dimmed()
                    );
                }
                if open_features.len() > 5 {
                    println!("   ... et {} autres", open_features.len() - 5);
                }
                println!();
            }
        }

        if !phases_pending.is_empty() {
            println!("{}", "📅 PHASES À VENIR".bold().dimmed());
            for phase in phases_pending.iter().take(5) {
                println!(
                    "   [P{}] {} — {}",
                    phase.priority,
                    phase.id,
                    phase.name
                );
            }
            if phases_pending.len() > 5 {
                println!("   ... et {} autres", phases_pending.len() - 5);
            }
            println!();
        }

        println!("{}", "═══════════════════════════════════════════════════════════".dimmed());
    }
}
