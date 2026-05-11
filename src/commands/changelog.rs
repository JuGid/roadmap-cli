//! Changelog command

use std::process::Command;
use std::collections::HashMap;
use colored::Colorize;
use regex::Regex;

pub fn cmd_changelog(limit: usize, from: Option<String>, to: Option<String>, format: String) {
    let range = match (&from, &to) {
        (Some(f), Some(t)) => format!("{}..{}", f, t),
        (Some(f), None) => format!("{}..HEAD", f),
        (None, Some(t)) => format!("HEAD~{}..{}", limit, t),
        (None, None) => format!("-{}", limit),
    };

    let output = Command::new("git")
        .args(["log", &range, "--pretty=format:%H|%h|%s|%an|%aI|%D", "--no-merges"])
        .output();

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            println!("{} Erreur git: {}", "Erreur:".red(), e);
            return;
        }
    };

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("{} {}", "Erreur git:".red(), err);
        return;
    }

    let log = String::from_utf8_lossy(&output.stdout);

    if log.trim().is_empty() {
        println!("{} Aucun commit trouvé", "ℹ".blue());
        return;
    }

    #[derive(Debug, serde::Serialize)]
    struct Commit {
        hash: String,
        short_hash: String,
        message: String,
        author: String,
        date: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        commit_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
    }

    let conv_re = Regex::new(r"^(\w+)(?:\(([^)]+)\))?(!)?:\s*(.+)$").unwrap();
    let tag_re = Regex::new(r"tag: ([^,\)]+)").unwrap();

    let mut commits: Vec<Commit> = Vec::new();

    for line in log.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 5 {
            continue;
        }

        let hash = parts[0].to_string();
        let short_hash = parts[1].to_string();
        let message = parts[2].to_string();
        let author = parts[3].to_string();
        let date = parts[4].to_string();
        let refs = if parts.len() > 5 { parts[5] } else { "" };

        let tag = tag_re.captures(refs).map(|c| c.get(1).unwrap().as_str().to_string());

        let (commit_type, scope) = if let Some(caps) = conv_re.captures(&message) {
            (
                Some(caps.get(1).unwrap().as_str().to_string()),
                caps.get(2).map(|m| m.as_str().to_string()),
            )
        } else {
            (None, None)
        };

        commits.push(Commit {
            hash,
            short_hash,
            message,
            author,
            date,
            tag,
            commit_type,
            scope,
        });
    }

    if format == "json" {
        let json = serde_json::to_string_pretty(&commits).expect("Erreur JSON");
        println!("{}", json);
        return;
    }

    // Markdown output
    println!("# Changelog\n");

    let has_conv = commits.iter().any(|c| c.commit_type.is_some());

    if has_conv {
        let mut by_type: HashMap<String, Vec<&Commit>> = HashMap::new();

        for commit in &commits {
            let type_key = commit.commit_type.as_deref().unwrap_or("other").to_string();
            by_type.entry(type_key).or_default().push(commit);
        }

        let type_order = ["feat", "fix", "perf", "refactor", "docs", "test", "chore", "other"];
        let type_labels: HashMap<&str, &str> = [
            ("feat", "✨ Nouvelles fonctionnalités"),
            ("fix", "🐛 Corrections de bugs"),
            ("perf", "⚡ Performances"),
            ("refactor", "♻️ Refactoring"),
            ("docs", "📚 Documentation"),
            ("test", "🧪 Tests"),
            ("chore", "🔧 Maintenance"),
            ("other", "📝 Autres"),
        ].into_iter().collect();

        for type_key in &type_order {
            if let Some(commits) = by_type.get(*type_key) {
                let label = type_labels.get(type_key).unwrap_or(type_key);
                println!("## {}\n", label);

                for commit in commits {
                    let scope = commit.scope.as_ref().map(|s| format!("**{}**: ", s)).unwrap_or_default();
                    let msg = conv_re.captures(&commit.message)
                        .and_then(|c| c.get(4))
                        .map(|m| m.as_str())
                        .unwrap_or(&commit.message);

                    let tag_badge = commit.tag.as_ref().map(|t| format!(" `{}`", t)).unwrap_or_default();

                    println!("- {}{}{} ({})", scope, msg, tag_badge, commit.short_hash);
                }
                println!();
            }
        }
    } else {
        let mut current_date = String::new();

        for commit in &commits {
            let date = commit.date.split('T').next().unwrap_or(&commit.date);

            if date != current_date {
                if !current_date.is_empty() {
                    println!();
                }
                println!("## {}\n", date);
                current_date = date.to_string();
            }

            let tag_badge = commit.tag.as_ref().map(|t| format!(" `{}`", t)).unwrap_or_default();
            println!("- {}{} ({})", commit.message, tag_badge, commit.short_hash);
        }
    }
}
