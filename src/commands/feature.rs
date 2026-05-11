//! Feature request management commands

use colored::Colorize;
use crate::phase::{FeatureRequest, FeatureStore};
use crate::utils::{load_phases, today};

const VALID_PRIORITIES: &[&str] = &["critical", "high", "medium", "low"];
const VALID_STATUSES: &[&str] = &["proposed", "accepted", "in_progress", "implemented", "rejected"];

fn priority_icon(priority: &str) -> &'static str {
    match priority {
        "critical" => "🔴",
        "high" => "🟠",
        "medium" => "🟡",
        "low" => "🟢",
        _ => "⚪",
    }
}

fn status_icon(status: &str) -> &'static str {
    match status {
        "proposed" => "💡",
        "accepted" => "👍",
        "in_progress" => "🔄",
        "implemented" => "✅",
        "rejected" => "❌",
        _ => "⬜",
    }
}

pub fn cmd_feature_add(
    title: String,
    priority: String,
    phase: Option<String>,
    description: Option<String>,
    assignee: Option<String>,
    requested_by: Option<String>,
    target: Option<String>,
) {
    if !VALID_PRIORITIES.contains(&priority.as_str()) {
        println!(
            "{} Priorité invalide '{}' (attendu: {})",
            "Erreur:".red(), priority.yellow(), VALID_PRIORITIES.join(", ")
        );
        return;
    }

    let (phases_dir, target_name) = if let Some(ref target_path) = target {
        let path = std::path::Path::new(target_path);
        let resolved = if path.exists() && path.join(".phases").exists() {
            path.join(".phases").to_string_lossy().to_string()
        } else {
            println!("{} Projet cible '{}' non trouvé", "Erreur:".red(), target_path.yellow());
            return;
        };
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| target_path.clone());
        (resolved, Some(name))
    } else {
        (".phases".to_string(), None)
    };

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

    let requested_by = requested_by.or_else(|| {
        if target.is_some() {
            std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        } else {
            None
        }
    });

    let mut store = FeatureStore::load_from(&phases_dir);
    let id = store.next_id();

    store.features.push(FeatureRequest {
        id,
        title: title.clone(),
        priority: priority.clone(),
        status: String::from("proposed"),
        phase,
        description,
        assignee,
        requested_by: requested_by.clone(),
        implementation: None,
        created_at: today(),
        implemented_at: None,
    });

    if let Err(e) = store.save_to(&phases_dir) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    if let Some(ref name) = target_name {
        println!(
            "{} Feature #{} créée [{}] {} → {}",
            priority_icon(&priority), id.to_string().cyan(),
            priority.to_uppercase(), title, name.yellow()
        );
        if let Some(ref requester) = requested_by {
            println!("  Demandée par: {}", requester.dimmed());
        }
    } else {
        println!(
            "{} Feature #{} créée [{}] {}",
            priority_icon(&priority), id.to_string().cyan(),
            priority.to_uppercase(), title
        );
    }
}

pub fn cmd_feature_list(priority: Option<String>, status: Option<String>, json: bool) {
    let store = FeatureStore::load();

    let filtered: Vec<&FeatureRequest> = store.features.iter().filter(|f| {
        let prio_ok = priority.as_ref().map_or(true, |p| f.priority == *p);
        let stat_ok = status.as_ref().map_or(true, |s| f.status == *s);
        prio_ok && stat_ok
    }).collect();

    if json {
        let output = serde_json::to_string_pretty(&filtered).expect("Erreur JSON");
        println!("{}", output);
        return;
    }

    if filtered.is_empty() {
        println!("Aucune feature request trouvée.");
        return;
    }

    let open = filtered.iter().filter(|f| f.status != "implemented" && f.status != "rejected").count();

    println!();
    println!("{} {} feature(s) ({} en attente)", "💡".bold(), filtered.len(), open);
    println!();

    for f in &filtered {
        let phase_info = f.phase.as_ref()
            .map(|p| format!(" — phase {}", p.dimmed()))
            .unwrap_or_default();
        let requester_info = f.requested_by.as_ref()
            .map(|r| format!(" (par {})", r.dimmed()))
            .unwrap_or_default();

        println!(
            "  {} {} #{} [{}] {}{}{}",
            status_icon(&f.status), priority_icon(&f.priority),
            f.id.to_string().cyan(), f.priority.to_uppercase(),
            f.title, phase_info, requester_info,
        );
    }
    println!();
}

pub fn cmd_feature_show(id: u32) {
    let store = FeatureStore::load();

    let f = match store.features.iter().find(|f| f.id == id) {
        Some(f) => f,
        None => { println!("{} Feature #{} non trouvée", "Erreur:".red(), id); return; }
    };

    println!();
    println!(
        "{} {} Feature #{} — {}",
        status_icon(&f.status), priority_icon(&f.priority),
        f.id.to_string().cyan().bold(), f.title.bold()
    );
    println!();
    println!("  Priorité:     {} {}", priority_icon(&f.priority), f.priority.to_uppercase());
    println!("  Statut:       {} {}", status_icon(&f.status), f.status);
    if let Some(ref phase) = f.phase { println!("  Phase:        {}", phase); }
    if let Some(ref assignee) = f.assignee { println!("  Assignée à:   {}", assignee); }
    if let Some(ref requester) = f.requested_by { println!("  Demandée par: {}", requester); }
    println!("  Créée le:     {}", f.created_at);
    if let Some(ref implemented) = f.implemented_at { println!("  Implémentée:  {}", implemented); }
    if let Some(ref desc) = f.description {
        println!();
        println!("  {}", "Description:".bold());
        println!("  {}", desc);
    }
    if let Some(ref impl_note) = f.implementation {
        println!();
        println!("  {}", "Implémentation:".bold());
        println!("  {}", impl_note);
    }
    println!();
}

pub fn cmd_feature_implement(id: u32, implementation: Option<String>, commit: Option<String>) {
    let mut store = FeatureStore::load();

    let f = match store.features.iter_mut().find(|f| f.id == id) {
        Some(f) => f,
        None => { println!("{} Feature #{} non trouvée", "Erreur:".red(), id); return; }
    };

    f.status = String::from("implemented");
    f.implemented_at = Some(today());

    let mut parts = Vec::new();
    if let Some(desc) = implementation { parts.push(desc); }
    if let Some(c) = commit { parts.push(format!("commit {}", c)); }
    if !parts.is_empty() { f.implementation = Some(parts.join(" — ")); }

    let title = f.title.clone();

    if let Err(e) = store.save() {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!("{} Feature #{} implémentée: {}", "✅".green(), id.to_string().cyan(), title);
}

pub fn cmd_feature_update(id: u32, status: Option<String>, priority: Option<String>, assignee: Option<String>, title: Option<String>, description: Option<String>, phase: Option<String>) {
    let mut store = FeatureStore::load();

    let f = match store.features.iter_mut().find(|f| f.id == id) {
        Some(f) => f,
        None => { println!("{} Feature #{} non trouvée", "Erreur:".red(), id); return; }
    };

    let mut changes = Vec::new();

    if let Some(t) = title {
        changes.push(format!("title: {}", t));
        f.title = t;
    }
    if let Some(d) = description {
        changes.push("description".to_string());
        f.description = Some(d);
    }
    if let Some(p) = phase {
        changes.push(format!("phase: {}", p));
        f.phase = Some(p);
    }
    if let Some(s) = status {
        if !VALID_STATUSES.contains(&s.as_str()) {
            println!("{} Statut invalide '{}'", "Erreur:".red(), s); return;
        }
        changes.push(format!("status: {}", s));
        f.status = s;
    }
    if let Some(p) = priority {
        if !VALID_PRIORITIES.contains(&p.as_str()) {
            println!("{} Priorité invalide '{}'", "Erreur:".red(), p); return;
        }
        changes.push(format!("priority: {}", p));
        f.priority = p;
    }
    if let Some(a) = assignee {
        changes.push(format!("assignee: {}", a));
        f.assignee = Some(a);
    }

    if changes.is_empty() {
        println!("{}", "Rien à modifier".yellow());
        return;
    }

    if let Err(e) = store.save() {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!("{} Feature #{} modifiée: {}", "✓".green(), id.to_string().cyan(), changes.join(", "));
}
