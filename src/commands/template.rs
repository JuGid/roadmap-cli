//! Templates command - create phases from predefined templates

use colored::Colorize;
use crate::phase::{Phase, Task};
use crate::utils::{save_phase, today};

pub fn cmd_template(name: String, phase_id: String, list: bool) {
    if list {
        print_templates();
        return;
    }

    let template = match get_template(&name) {
        Some(t) => t,
        None => {
            println!("{} Template '{}' non trouvé", "Erreur:".red(), name.yellow());
            println!();
            print_templates();
            return;
        }
    };

    let phase_file = format!(".phases/phase-{}.yml", phase_id);
    if std::path::Path::new(&phase_file).exists() {
        println!(
            "{} La phase {} existe déjà",
            "Erreur:".red(),
            phase_id.yellow()
        );
        return;
    }

    let now = today();
    let mut phase = Phase::new(phase_id.clone(), template.name.to_string());
    phase.description = template.description.to_string();

    for (i, task_name) in template.tasks.iter().enumerate() {
        phase.tasks.push(Task {
            id: format!("{}.{}", phase_id, i + 1),
            name: task_name.to_string(),
            description: None,
            status: String::from("pending"),
            parent: None,
            workflow_stage: None,
            optional: false,
            completed_at: None,
            blocks: Vec::new(),
            blocked_by: Vec::new(),
            files: Vec::new(),
            tags: Vec::new(),
            assignee: None,
            due: None,
        });
    }

    phase.created_at = now.clone();
    phase.updated_at = now;

    if let Err(e) = save_phase(&phase) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    println!(
        "{} Phase {} créée depuis le template '{}' ({} tâches)",
        "✓".green(),
        phase_id.cyan(),
        name.cyan(),
        template.tasks.len()
    );
}

struct TemplateDef {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    tasks: Vec<&'static str>,
}

fn get_template(name: &str) -> Option<TemplateDef> {
    let templates = get_all_templates();
    templates.into_iter().find(|t| t.id == name)
}

fn get_all_templates() -> Vec<TemplateDef> {
    vec![
        TemplateDef {
            id: "feature",
            name: "Nouvelle fonctionnalité",
            description: "Développement d'une nouvelle fonctionnalité",
            tasks: vec![
                "Analyse et spécifications",
                "Conception technique",
                "Implémentation",
                "Tests unitaires",
                "Tests d'intégration",
                "Documentation",
                "Code review",
                "Déploiement",
            ],
        },
        TemplateDef {
            id: "bug",
            name: "Correction de bug",
            description: "Investigation et correction d'un bug",
            tasks: vec![
                "Reproduction du bug",
                "Investigation root cause",
                "Fix",
                "Tests de non-régression",
                "Code review",
                "Déploiement",
            ],
        },
        TemplateDef {
            id: "api",
            name: "Endpoint API",
            description: "Création d'un nouvel endpoint API",
            tasks: vec![
                "Spécification OpenAPI",
                "Implémentation route",
                "Validation & erreurs",
                "Tests unitaires",
                "Tests d'intégration",
                "Documentation API",
                "Rate limiting",
            ],
        },
        TemplateDef {
            id: "infra",
            name: "Infrastructure",
            description: "Mise en place ou modification d'infrastructure",
            tasks: vec![
                "Audit de l'existant",
                "Architecture cible",
                "Setup environnement",
                "Configuration",
                "Monitoring & alertes",
                "Documentation runbook",
                "Tests de charge",
            ],
        },
        TemplateDef {
            id: "release",
            name: "Release",
            description: "Préparation et publication d'une release",
            tasks: vec![
                "Feature freeze",
                "Tests de régression",
                "Changelog",
                "Bump version",
                "Build & artefacts",
                "Déploiement staging",
                "Validation QA",
                "Déploiement production",
                "Communication",
            ],
        },
        TemplateDef {
            id: "onboarding",
            name: "Onboarding développeur",
            description: "Parcours d'intégration d'un nouveau développeur",
            tasks: vec![
                "Setup poste de travail",
                "Accès repos & outils",
                "Lecture documentation",
                "Premier build local",
                "Premier ticket simple",
                "Présentation architecture",
                "Premier code review",
            ],
        },
    ]
}

fn print_templates() {
    println!("{}", "📋 Templates disponibles:".bold());
    println!();

    for template in get_all_templates() {
        println!(
            "  {} — {} ({} tâches)",
            template.id.cyan().bold(),
            template.name,
            template.tasks.len()
        );
        println!("      {}", template.description.dimmed());
    }

    println!();
    println!(
        "Usage: {}",
        "roadmap template <nom> <phase-id>".yellow()
    );
}
