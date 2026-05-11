//! Initialize a new roadmap project

use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::config::Config;

pub fn cmd_init() {
    let phases_dir = Path::new(".phases");

    if phases_dir.exists() {
        println!("{}", "Erreur: .phases/ existe déjà".red());
        return;
    }

    if let Err(e) = fs::create_dir(phases_dir) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    let config = Config::default();
    let yaml = serde_yaml::to_string(&config).expect("Erreur sérialisation");

    let config_path = phases_dir.join("config.yml");
    if let Err(e) = fs::write(&config_path, yaml) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!("{}", "✓ Roadmap initialisée !".green());
    println!("  Créé: {}", ".phases/config.yml".cyan());
    println!(
        "\n  Prochaine étape: {} pour créer une phase",
        "roadmap add <id> <nom>".yellow()
    );
}
