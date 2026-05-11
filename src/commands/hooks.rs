//! Git hooks management - auto-export on commit

use std::fs;
use std::path::Path;
use colored::Colorize;

const HOOK_MARKER: &str = "# roadmap-cli auto-export";

const HOOK_SCRIPT: &str = r#"# roadmap-cli auto-export
if command -v roadmap >/dev/null 2>&1; then
    if [ -d ".phases" ]; then
        roadmap export 2>/dev/null
        git add ROADMAP.md 2>/dev/null
    fi
fi
"#;

pub fn cmd_hooks(install: bool, uninstall: bool) {
    if !install && !uninstall {
        // Show status
        let hook_path = Path::new(".git/hooks/pre-commit");
        if hook_path.exists() {
            let content = fs::read_to_string(hook_path).unwrap_or_default();
            if content.contains(HOOK_MARKER) {
                println!("{} Git hook installé (pre-commit: auto-export)", "✅".green());
            } else {
                println!("{} Git hook non installé", "⬜");
            }
        } else {
            println!("{} Git hook non installé", "⬜");
        }
        println!();
        println!("  Installer:   {}", "roadmap hooks --install".yellow());
        println!("  Désinstaller: {}", "roadmap hooks --uninstall".yellow());
        return;
    }

    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        println!("{} Pas un repo git", "Erreur:".red());
        return;
    }

    let hooks_dir = git_dir.join("hooks");
    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir).expect("Impossible de créer .git/hooks");
    }

    let hook_path = hooks_dir.join("pre-commit");

    if install {
        let existing = if hook_path.exists() {
            fs::read_to_string(&hook_path).unwrap_or_default()
        } else {
            String::new()
        };

        if existing.contains(HOOK_MARKER) {
            println!("{} Hook déjà installé", "ℹ".blue());
            return;
        }

        let new_content = if existing.is_empty() {
            format!("#!/bin/sh\n{}", HOOK_SCRIPT)
        } else {
            format!("{}\n{}", existing, HOOK_SCRIPT)
        };

        fs::write(&hook_path, new_content).expect("Erreur écriture hook");

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(0o755);
            fs::set_permissions(&hook_path, perms).expect("Erreur permissions");
        }

        println!("{} Hook pre-commit installé", "✓".green());
        println!("  Le ROADMAP.md sera auto-généré à chaque commit");
    }

    if uninstall {
        if !hook_path.exists() {
            println!("{} Aucun hook à désinstaller", "ℹ".blue());
            return;
        }

        let content = fs::read_to_string(&hook_path).unwrap_or_default();
        if !content.contains(HOOK_MARKER) {
            println!("{} Hook roadmap non trouvé dans pre-commit", "ℹ".blue());
            return;
        }

        // Remove our section
        let new_content: String = content
            .lines()
            .collect::<Vec<_>>()
            .split(|line| line.contains(HOOK_MARKER))
            .next()
            .unwrap_or(&[])
            .join("\n");

        let trimmed = new_content.trim();
        if trimmed.is_empty() || trimmed == "#!/bin/sh" {
            fs::remove_file(&hook_path).expect("Erreur suppression hook");
        } else {
            fs::write(&hook_path, format!("{}\n", trimmed)).expect("Erreur écriture hook");
        }

        println!("{} Hook pre-commit désinstallé", "✓".green());
    }
}
