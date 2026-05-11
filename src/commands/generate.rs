//! Generate man pages and shell completions

use std::fs;
use std::path::Path;
use colored::Colorize;
use clap::CommandFactory;
use clap_mangen::Man;
use clap_complete::{generate_to, Shell};

#[derive(Clone, clap::ValueEnum)]
pub enum GenerateType {
    /// Pages man
    Man,
    /// Completions shell (bash, zsh, fish)
    Completions,
    /// Tout générer
    All,
}

pub fn cmd_generate(what: GenerateType, output_dir: String) {
    let out_path = Path::new(&output_dir);

    if let Err(e) = fs::create_dir_all(out_path) {
        println!("{} Impossible de créer {}: {}", "Erreur:".red(), output_dir, e);
        return;
    }

    let cmd = crate::cli::Cli::command();

    match what {
        GenerateType::Man => generate_man_pages(&cmd, out_path),
        GenerateType::Completions => generate_completions(&cmd, out_path),
        GenerateType::All => {
            generate_man_pages(&cmd, out_path);
            generate_completions(&cmd, out_path);
        }
    }
}

fn generate_man_pages(cmd: &clap::Command, out_path: &Path) {
    let man_dir = out_path.join("man");
    if let Err(e) = fs::create_dir_all(&man_dir) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    let man = Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Vec::new();
    if let Err(e) = man.render(&mut buffer) {
        println!("{} Génération man page principale: {}", "Erreur:".red(), e);
        return;
    }

    let man_path = man_dir.join("roadmap-cli.1");
    if let Err(e) = fs::write(&man_path, &buffer) {
        println!("{} Écriture {}: {}", "Erreur:".red(), man_path.display(), e);
        return;
    }
    println!("{} {}", "✓".green(), man_path.display());

    for subcmd in cmd.get_subcommands() {
        if subcmd.is_hide_set() {
            continue;
        }

        let subcmd_name = subcmd.get_name();
        let man = Man::new(subcmd.clone());
        let mut buffer: Vec<u8> = Vec::new();

        if man.render(&mut buffer).is_ok() {
            let subman_path = man_dir.join(format!("roadmap-cli-{}.1", subcmd_name));
            if fs::write(&subman_path, &buffer).is_ok() {
                println!("{} {}", "✓".green(), subman_path.display());
            }
        }
    }

    println!();
    println!("Pour installer les man pages:");
    println!("  {} sudo cp {}/*.1 /usr/local/share/man/man1/", "→".cyan(), man_dir.display());
    println!("  {} sudo mandb", "→".cyan());
}

fn generate_completions(cmd: &clap::Command, out_path: &Path) {
    let comp_dir = out_path.join("completions");
    if let Err(e) = fs::create_dir_all(&comp_dir) {
        println!("{} {}", "Erreur:".red(), e);
        return;
    }

    let shells = [
        (Shell::Bash, "bash"),
        (Shell::Zsh, "zsh"),
        (Shell::Fish, "fish"),
        (Shell::PowerShell, "powershell"),
    ];

    for (shell, name) in shells {
        let mut cmd_clone = cmd.clone();
        match generate_to(shell, &mut cmd_clone, "roadmap", &comp_dir) {
            Ok(path) => println!("{} {} ({})", "✓".green(), path.display(), name),
            Err(e) => println!("{} Génération {}: {}", "Erreur:".red(), name, e),
        }
    }

    println!();
    println!("Pour installer les completions:");
    println!("  {} # Bash", "→".cyan());
    println!("    sudo cp {}/roadmap.bash /etc/bash_completion.d/", comp_dir.display());
    println!("  {} # Zsh", "→".cyan());
    println!("    cp {}/_roadmap ~/.zsh/completions/", comp_dir.display());
    println!("  {} # Fish", "→".cyan());
    println!("    cp {}/roadmap.fish ~/.config/fish/completions/", comp_dir.display());
}
