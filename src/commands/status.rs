//! Status command

use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::phase::Phase;
use crate::utils::today;

pub fn cmd_status(id: String, status: String) {
    let phases_dir = Path::new(".phases");

    if !phases_dir.exists() {
        println!(
            "{} Roadmap non initialisée. Lance d'abord: {}",
            "Erreur:".red(),
            "roadmap init".yellow()
        );
        return;
    }

    let valid_statuses = ["pending", "in_progress", "done", "blocked"];
    if !valid_statuses.contains(&status.as_str()) {
        println!(
            "{} Statut invalide. Valeurs possibles: {}",
            "Erreur:".red(),
            valid_statuses.join(", ").yellow()
        );
        return;
    }

    let phase_file = phases_dir.join(format!("phase-{}.yml", id));
    if !phase_file.exists() {
        println!("{} Phase {} non trouvée", "Erreur:".red(), id.yellow());
        return;
    }

    let content = match fs::read_to_string(&phase_file) {
        Ok(c) => c,
        Err(e) => {
            println!("{} {}", "Erreur:".red(), e);
            return;
        }
    };

    let mut phase: Phase = match serde_yaml::from_str(&content) {
        Ok(p) => p,
        Err(e) => {
            println!("{} YAML invalide: {}", "Erreur:".red(), e);
            return;
        }
    };

    let old_status = phase.status.clone();
    phase.status = status.clone();
    phase.updated_at = today();

    let yaml = serde_yaml::to_string(&phase).expect("Erreur sérialisation");
    if let Err(e) = fs::write(&phase_file, yaml) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    let status_icon = match status.as_str() {
        "done" => "✅",
        "in_progress" => "🔄",
        "blocked" => "🚫",
        _ => "⬜",
    };

    println!(
        "{} Phase {} : {} → {}",
        status_icon,
        id.cyan(),
        old_status.dimmed(),
        status.green()
    );
}
