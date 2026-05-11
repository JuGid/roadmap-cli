//! Scan command for TODO/FIXME

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;
use colored::Colorize;
use regex::Regex;
use crate::phase::{Phase, Task};
use crate::utils::today;

pub fn cmd_scan(pattern: String, create: bool, target_phase: Option<String>, include_hidden: bool) {
    let markers = ["TODO", "FIXME", "HACK", "BUG", "XXX", "OPTIMIZE", "REFACTOR"];
    let pattern_str = format!(r"(?i)\b({})\b[:\s]*(.+?)$", markers.join("|"));
    let re = Regex::new(&pattern_str).expect("Invalid regex");

    let glob_pattern = if pattern.starts_with('/') || pattern.starts_with('.') {
        pattern.clone()
    } else {
        format!("./{}", pattern)
    };

    let entries = match glob::glob(&glob_pattern) {
        Ok(paths) => paths,
        Err(e) => {
            println!("{} Pattern invalide: {}", "Erreur:".red(), e);
            return;
        }
    };

    #[derive(Debug)]
    struct Finding {
        file: String,
        line: usize,
        marker: String,
        content: String,
    }

    let mut findings: Vec<Finding> = Vec::new();

    for entry in entries {
        let path = match entry {
            Ok(p) => p,
            Err(_) => continue,
        };

        if !include_hidden {
            let path_str = path.to_string_lossy();
            if path_str.contains("/.") || path_str.contains("\\.") {
                continue;
            }
        }

        if !path.is_file() {
            continue;
        }

        let ext = path.extension().map(|e| e.to_string_lossy().to_lowercase());
        let binary_exts = ["exe", "dll", "so", "dylib", "bin", "png", "jpg", "jpeg", "gif", "ico", "woff", "woff2", "ttf", "eot"];
        if let Some(ref e) = ext {
            if binary_exts.contains(&e.as_str()) {
                continue;
            }
        }

        let file = match fs::File::open(&path) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let reader = BufReader::new(file);

        for (line_num, line_result) in reader.lines().enumerate() {
            let line = match line_result {
                Ok(l) => l,
                Err(_) => continue,
            };

            if let Some(captures) = re.captures(&line) {
                let marker = captures.get(1).map(|m| m.as_str().to_uppercase()).unwrap_or_default();
                let content = captures.get(2).map(|m| m.as_str().trim().to_string()).unwrap_or_default();

                if content.is_empty() || content.len() < 3 {
                    continue;
                }

                findings.push(Finding {
                    file: path.to_string_lossy().to_string(),
                    line: line_num + 1,
                    marker,
                    content,
                });
            }
        }
    }

    if findings.is_empty() {
        println!("{} Aucun TODO/FIXME trouvé", "✓".green());
        return;
    }

    println!();
    println!("{}", format!("📝 {} résultats trouvés", findings.len()).bold());
    println!();

    let mut by_marker: HashMap<String, Vec<&Finding>> = HashMap::new();
    for finding in &findings {
        by_marker.entry(finding.marker.clone()).or_default().push(finding);
    }

    for (marker, items) in &by_marker {
        let icon = match marker.as_str() {
            "TODO" => "📋",
            "FIXME" => "🔧",
            "HACK" => "⚠️",
            "BUG" => "🐛",
            "XXX" => "❌",
            "OPTIMIZE" => "⚡",
            "REFACTOR" => "🔄",
            _ => "📌",
        };

        println!("{} {} ({})", icon, marker.cyan().bold(), items.len());

        for item in items {
            let short_file = item.file.strip_prefix("./").unwrap_or(&item.file);
            println!(
                "   {}:{} — {}",
                short_file.dimmed(),
                item.line.to_string().dimmed(),
                item.content
            );
        }
        println!();
    }

    if create {
        let phase_id = match target_phase {
            Some(id) => id,
            None => {
                println!(
                    "{} Spécifie la phase cible avec {}",
                    "Erreur:".red(),
                    "--phase <id>".yellow()
                );
                return;
            }
        };

        let phases_dir = Path::new(".phases");
        let phase_file = phases_dir.join(format!("phase-{}.yml", phase_id));

        if !phase_file.exists() {
            println!("{} Phase {} non trouvée", "Erreur:".red(), phase_id.yellow());
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

        let existing_task_count = phase.tasks.iter().filter(|t| t.parent.is_none()).count();
        let mut created = 0;

        for (i, finding) in findings.iter().enumerate() {
            let already_exists = phase.tasks.iter().any(|t| {
                t.name.to_lowercase().contains(&finding.content.to_lowercase())
                    || finding.content.to_lowercase().contains(&t.name.to_lowercase())
            });

            if already_exists {
                continue;
            }

            let task_id = format!("{}.{}", phase_id, existing_task_count + i + 1);

            let task_name = if finding.content.len() > 80 {
                format!("{}...", &finding.content[..77])
            } else {
                finding.content.clone()
            };

            let short_file = finding.file.strip_prefix("./").unwrap_or(&finding.file);

            let task = Task {
                id: task_id.clone(),
                name: format!("[{}] {}", finding.marker, task_name),
                description: Some(format!(
                    "Trouvé dans {}:{}",
                    short_file, finding.line
                )),
                status: String::from("pending"),
                parent: None,
                workflow_stage: None,
                optional: finding.marker != "FIXME" && finding.marker != "BUG",
                completed_at: None,
                blocks: Vec::new(),
                blocked_by: Vec::new(),
                files: vec![short_file.to_string()],
                tags: Vec::new(),
                assignee: None,
                due: None,
            };

            phase.tasks.push(task);
            created += 1;
        }

        if created == 0 {
            println!("{} Toutes les tâches existent déjà", "ℹ".blue());
            return;
        }

        phase.updated_at = today();

        let yaml = serde_yaml::to_string(&phase).expect("Erreur sérialisation");
        if let Err(e) = fs::write(&phase_file, yaml) {
            println!("{} {}", "Erreur:".red(), e);
            return;
        }

        println!(
            "{} {} tâches créées dans la phase {}",
            "✓".green(),
            created,
            phase_id.cyan()
        );
    } else {
        println!(
            "Pour créer des tâches: {}",
            format!("roadmap scan --glob \"{}\" --create --phase <id>", pattern).yellow()
        );
    }
}
