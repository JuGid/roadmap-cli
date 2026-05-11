//! Bug management commands

use colored::Colorize;
use crate::phase::{Bug, BugStore};
use crate::utils::{load_phases, today};

const VALID_SEVERITIES: &[&str] = &["blocking", "major", "minor"];
const VALID_STATUSES: &[&str] = &["open", "in_progress", "resolved", "wontfix"];

fn severity_icon(severity: &str) -> &'static str {
    match severity {
        "blocking" => "🔴",
        "major" => "🟠",
        "minor" => "🟡",
        _ => "⚪",
    }
}

fn status_icon(status: &str) -> &'static str {
    match status {
        "open" => "⬜",
        "in_progress" => "🔄",
        "resolved" => "✅",
        "wontfix" => "⏭️",
        _ => "⬜",
    }
}

pub fn cmd_bug_add(
    title: String,
    severity: String,
    phase: Option<String>,
    description: Option<String>,
    assignee: Option<String>,
    reported_by: Option<String>,
    target: Option<String>,
) {
    if !VALID_SEVERITIES.contains(&severity.as_str()) {
        println!(
            "{} Sévérité invalide '{}' (attendu: {})",
            "Erreur:".red(),
            severity.yellow(),
            VALID_SEVERITIES.join(", ")
        );
        return;
    }

    // Resolve target project
    let (phases_dir, target_name) = if let Some(ref target_path) = target {
        let path = std::path::Path::new(target_path);

        // Target can be a path or a project slug
        let resolved = if path.exists() && path.join(".phases").exists() {
            // Direct path: /path/to/project
            path.join(".phases").to_string_lossy().to_string()
        } else if path.join(".phases").exists() {
            path.join(".phases").to_string_lossy().to_string()
        } else {
            println!(
                "{} Projet cible '{}' non trouvé (pas de .phases/ dans ce répertoire)",
                "Erreur:".red(),
                target_path.yellow()
            );
            return;
        };

        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| target_path.clone());

        (resolved, Some(name))
    } else {
        (".phases".to_string(), None)
    };

    // Verify phase exists if specified (only for local project)
    if target.is_none() {
        if let Some(ref phase_id) = phase {
            if let Some(phases) = load_phases() {
                if !phases.iter().any(|p| p.id == *phase_id) {
                    println!("{} Phase {} non trouvée", "Erreur:".red(), phase_id.yellow());
                    return;
                }
            }
        }
    }

    // Auto-detect reported_by from current project name
    let reported_by = reported_by.or_else(|| {
        if target.is_some() {
            std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        } else {
            None
        }
    });

    let mut store = BugStore::load_from(&phases_dir);
    let id = store.next_id();

    store.bugs.push(Bug {
        id,
        title: title.clone(),
        severity: severity.clone(),
        status: String::from("open"),
        phase,
        description,
        assignee,
        reported_by: reported_by.clone(),
        resolution: None,
        created_at: today(),
        resolved_at: None,
    });

    if let Err(e) = store.save_to(&phases_dir) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    if let Some(ref name) = target_name {
        println!(
            "{} Bug #{} créé [{}] {} → {}",
            severity_icon(&severity),
            id.to_string().cyan(),
            severity.to_uppercase(),
            title,
            name.yellow()
        );
        if let Some(ref reporter) = reported_by {
            println!("  Rapporté par: {}", reporter.dimmed());
        }
    } else {
        println!(
            "{} Bug #{} créé [{}] {}",
            severity_icon(&severity),
            id.to_string().cyan(),
            severity.to_uppercase(),
            title
        );
    }
}

pub fn cmd_bug_list(severity: Option<String>, status: Option<String>, json: bool) {
    let store = BugStore::load();

    let filtered: Vec<&Bug> = store.bugs.iter().filter(|b| {
        let sev_ok = severity.as_ref().map_or(true, |s| b.severity == *s);
        let stat_ok = status.as_ref().map_or(true, |s| b.status == *s);
        sev_ok && stat_ok
    }).collect();

    if json {
        let output = serde_json::to_string_pretty(&filtered).expect("Erreur JSON");
        println!("{}", output);
        return;
    }

    if filtered.is_empty() {
        println!("Aucun bug trouvé.");
        return;
    }

    // Summary
    let open = filtered.iter().filter(|b| b.status == "open" || b.status == "in_progress").count();
    let blocking = filtered.iter().filter(|b| b.severity == "blocking" && b.status != "resolved" && b.status != "wontfix").count();

    println!();
    println!("{} {} bug(s) ({} ouvert(s), {} bloquant(s))", "🐛".bold(), filtered.len(), open, blocking);
    println!();

    for bug in &filtered {
        let phase_info = bug.phase.as_ref()
            .map(|p| format!(" — phase {}", p.dimmed()))
            .unwrap_or_default();
        let assignee_info = bug.assignee.as_ref()
            .map(|a| format!(" @{}", a.dimmed()))
            .unwrap_or_default();

        println!(
            "  {} {} #{} [{}] {}{}{}",
            status_icon(&bug.status),
            severity_icon(&bug.severity),
            bug.id.to_string().cyan(),
            bug.severity.to_uppercase(),
            bug.title,
            phase_info,
            assignee_info,
        );
    }
    println!();
}

pub fn cmd_bug_show(id: u32) {
    let store = BugStore::load();

    let bug = match store.bugs.iter().find(|b| b.id == id) {
        Some(b) => b,
        None => {
            println!("{} Bug #{} non trouvé", "Erreur:".red(), id);
            return;
        }
    };

    println!();
    println!(
        "{} {} Bug #{} — {}",
        status_icon(&bug.status),
        severity_icon(&bug.severity),
        bug.id.to_string().cyan().bold(),
        bug.title.bold()
    );
    println!();
    println!("  Sévérité:    {} {}", severity_icon(&bug.severity), bug.severity.to_uppercase());
    println!("  Statut:      {} {}", status_icon(&bug.status), bug.status);
    if let Some(ref phase) = bug.phase {
        println!("  Phase:       {}", phase);
    }
    if let Some(ref assignee) = bug.assignee {
        println!("  Assigné à:   {}", assignee);
    }
    if let Some(ref reporter) = bug.reported_by {
        println!("  Rapporté par: {}", reporter);
    }
    println!("  Créé le:     {}", bug.created_at);
    if let Some(ref resolved) = bug.resolved_at {
        println!("  Résolu le:   {}", resolved);
    }
    if let Some(ref desc) = bug.description {
        println!();
        println!("  {}", "Description:".bold());
        println!("  {}", desc);
    }
    if let Some(ref resolution) = bug.resolution {
        println!();
        println!("  {}", "Résolution:".bold());
        println!("  {}", resolution);
    }
    println!();
}

pub fn cmd_bug_resolve(id: u32, resolution: Option<String>, commit: Option<String>) {
    let mut store = BugStore::load();

    let bug = match store.bugs.iter_mut().find(|b| b.id == id) {
        Some(b) => b,
        None => {
            println!("{} Bug #{} non trouvé", "Erreur:".red(), id);
            return;
        }
    };

    bug.status = String::from("resolved");
    bug.resolved_at = Some(today());

    let mut res_parts = Vec::new();
    if let Some(desc) = resolution {
        res_parts.push(desc);
    }
    if let Some(c) = commit {
        res_parts.push(format!("commit {}", c));
    }
    if !res_parts.is_empty() {
        bug.resolution = Some(res_parts.join(" — "));
    }

    let title = bug.title.clone();

    if let Err(e) = store.save() {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!("{} Bug #{} résolu: {}", "✅".green(), id.to_string().cyan(), title);
}

pub fn cmd_bug_update(id: u32, status: Option<String>, severity: Option<String>, assignee: Option<String>, title: Option<String>, description: Option<String>, phase: Option<String>) {
    let mut store = BugStore::load();

    let bug = match store.bugs.iter_mut().find(|b| b.id == id) {
        Some(b) => b,
        None => {
            println!("{} Bug #{} non trouvé", "Erreur:".red(), id);
            return;
        }
    };

    let mut changes = Vec::new();

    if let Some(t) = title {
        changes.push(format!("title: {}", t));
        bug.title = t;
    }
    if let Some(d) = description {
        changes.push("description".to_string());
        bug.description = Some(d);
    }
    if let Some(p) = phase {
        changes.push(format!("phase: {}", p));
        bug.phase = Some(p);
    }
    if let Some(s) = status {
        if !VALID_STATUSES.contains(&s.as_str()) {
            println!("{} Statut invalide '{}'", "Erreur:".red(), s);
            return;
        }
        changes.push(format!("status: {}", s));
        bug.status = s;
    }
    if let Some(s) = severity {
        if !VALID_SEVERITIES.contains(&s.as_str()) {
            println!("{} Sévérité invalide '{}'", "Erreur:".red(), s);
            return;
        }
        changes.push(format!("severity: {}", s));
        bug.severity = s;
    }
    if let Some(a) = assignee {
        changes.push(format!("assignee: {}", a));
        bug.assignee = Some(a);
    }

    if changes.is_empty() {
        println!("{}", "Rien à modifier".yellow());
        return;
    }

    if let Err(e) = store.save() {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!("{} Bug #{} modifié: {}", "✓".green(), id.to_string().cyan(), changes.join(", "));
}
