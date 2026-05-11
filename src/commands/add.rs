//! Add and edit phases

use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::phase::Phase;
use crate::utils::{load_phases, save_phase, today};

pub fn cmd_add(id: String, name: String, parent: Option<String>, depends_on: Option<Vec<String>>) {
    let phases_dir = Path::new(".phases");

    if !phases_dir.exists() {
        println!(
            "{} Roadmap non initialisée. Lance d'abord: {}",
            "Erreur:".red(),
            "roadmap init".yellow()
        );
        return;
    }

    let phase_file = phases_dir.join(format!("phase-{}.yml", id));
    if phase_file.exists() {
        println!(
            "{} La phase {} existe déjà",
            "Erreur:".red(),
            id.yellow()
        );
        return;
    }

    if let Some(ref parent_id) = parent {
        let parent_file = phases_dir.join(format!("phase-{}.yml", parent_id));
        if !parent_file.exists() {
            println!(
                "{} La phase parente {} n'existe pas",
                "Erreur:".red(),
                parent_id.yellow()
            );
            return;
        }
    }

    let mut phase = Phase::new(id.clone(), name.clone());
    phase.parent = parent.clone();
    phase.depends_on = depends_on.unwrap_or_default();

    let yaml = serde_yaml::to_string(&phase).expect("Erreur sérialisation");
    if let Err(e) = fs::write(&phase_file, yaml) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    if let Some(parent_id) = parent {
        println!(
            "{} Phase {} créée (sous-phase de {})",
            "✓".green(),
            id.cyan(),
            parent_id.cyan()
        );
    } else {
        println!("{} Phase {} créée", "✓".green(), id.cyan());
    }
    println!("  Fichier: {}", format!(".phases/phase-{}.yml", id).cyan());
}

pub fn cmd_edit(id: String, name: Option<String>, description: Option<String>, depends_on: Option<Vec<String>>) {
    let mut phases = match load_phases() {
        Some(p) => p,
        None => return,
    };

    let phase = match phases.iter_mut().find(|p| p.id == id) {
        Some(p) => p,
        None => {
            println!("{} Phase {} non trouvée", "Erreur:".red(), id.yellow());
            return;
        }
    };

    let mut modified = false;

    if let Some(new_name) = name {
        phase.name = new_name;
        modified = true;
    }

    if let Some(new_desc) = description {
        phase.description = new_desc;
        modified = true;
    }

    if let Some(deps) = depends_on {
        phase.depends_on = deps;
        modified = true;
    }

    if !modified {
        println!("{}", "Rien à modifier. Utilisez --name, --description ou --depends-on".yellow());
        return;
    }

    phase.updated_at = today();

    if let Err(e) = save_phase(phase) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!("{} Phase {} modifiée", "✓".green(), id.cyan());
}

pub fn cmd_priority(id: String, priority: u32) {
    let mut phases = match load_phases() {
        Some(p) => p,
        None => return,
    };

    let phase = match phases.iter_mut().find(|p| p.id == id) {
        Some(p) => p,
        None => {
            println!("{} Phase {} non trouvée", "Erreur:".red(), id.yellow());
            return;
        }
    };

    let old_priority = phase.priority;
    phase.priority = priority;
    phase.updated_at = today();

    if let Err(e) = save_phase(phase) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!(
        "{} Priorité de {} changée: {} → {}",
        "✓".green(),
        id.cyan(),
        old_priority,
        priority
    );
}

pub fn cmd_note(id: String, content: String) {
    let mut phases = match load_phases() {
        Some(p) => p,
        None => return,
    };

    let phase = match phases.iter_mut().find(|p| p.id == id) {
        Some(p) => p,
        None => {
            println!("{} Phase {} non trouvée", "Erreur:".red(), id.yellow());
            return;
        }
    };

    phase.notes.push(crate::phase::Note {
        date: today(),
        content: content.clone(),
    });
    phase.updated_at = today();

    if let Err(e) = save_phase(phase) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!("{} Note ajoutée à {}", "✓".green(), id.cyan());
}
