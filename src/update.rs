use std::env;
use std::fs;
use std::path::PathBuf;

use colored::Colorize;
use flate2::read::GzDecoder;
use semver::Version;
use tar::Archive;

const GITHUB_REPO: &str = "Siovos/roadmap-cli";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, serde::Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
    html_url: String,
}

#[derive(Debug, serde::Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

/// Get the asset name for the current platform
fn get_platform_asset_name() -> Option<&'static str> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    match (os, arch) {
        ("macos", "x86_64") => Some("roadmap-cli-darwin-x86_64.tar.gz"),
        ("macos", "aarch64") => Some("roadmap-cli-darwin-arm64.tar.gz"),
        ("linux", "x86_64") => Some("roadmap-cli-linux-x86_64.tar.gz"),
        _ => None,
    }
}

/// Fetch the latest release info from GitHub
fn fetch_latest_release() -> Result<GitHubRelease, String> {
    let url = format!(
        "https://api.github.com/repos/{}/releases/latest",
        GITHUB_REPO
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent("roadmap-cli")
        .build()
        .map_err(|e| format!("Erreur création client HTTP: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .map_err(|e| format!("Erreur requête GitHub: {}", e))?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err("Aucune release trouvée sur GitHub".to_string());
    }

    if !response.status().is_success() {
        return Err(format!("Erreur GitHub API: {}", response.status()));
    }

    response
        .json::<GitHubRelease>()
        .map_err(|e| format!("Erreur parsing JSON: {}", e))
}

/// Parse version from tag (removes 'v' prefix if present)
fn parse_version(tag: &str) -> Result<Version, String> {
    let version_str = tag.strip_prefix('v').unwrap_or(tag);
    Version::parse(version_str).map_err(|e| format!("Version invalide '{}': {}", tag, e))
}

/// Get the path to the current executable
fn get_current_exe_path() -> Result<PathBuf, String> {
    env::current_exe().map_err(|e| format!("Impossible de trouver l'exécutable actuel: {}", e))
}

/// Check for updates and return info about latest version
pub fn check_for_update() -> Result<Option<(String, String)>, String> {
    let release = fetch_latest_release()?;

    let current = parse_version(CURRENT_VERSION)?;
    let latest = parse_version(&release.tag_name)?;

    if latest > current {
        Ok(Some((release.tag_name, release.html_url)))
    } else {
        Ok(None)
    }
}

/// Download and install the update
pub fn perform_update() -> Result<(), String> {
    let release = fetch_latest_release()?;

    let current = parse_version(CURRENT_VERSION)?;
    let latest = parse_version(&release.tag_name)?;

    if latest <= current {
        println!(
            "{} Vous utilisez déjà la dernière version ({})",
            "✓".green(),
            CURRENT_VERSION
        );
        return Ok(());
    }

    println!(
        "Nouvelle version disponible: {} → {}",
        CURRENT_VERSION.dimmed(),
        release.tag_name.green()
    );

    // Find the right asset for this platform
    let asset_name = get_platform_asset_name().ok_or_else(|| {
        format!(
            "Plateforme non supportée: {} {}",
            env::consts::OS,
            env::consts::ARCH
        )
    })?;

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| format!("Asset '{}' non trouvé dans la release", asset_name))?;

    println!("Téléchargement de {}...", asset.name);

    // Download the archive
    let client = reqwest::blocking::Client::builder()
        .user_agent("roadmap-cli")
        .build()
        .map_err(|e| format!("Erreur création client HTTP: {}", e))?;

    let response = client
        .get(&asset.browser_download_url)
        .send()
        .map_err(|e| format!("Erreur téléchargement: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Erreur téléchargement: {}", response.status()));
    }

    let bytes = response
        .bytes()
        .map_err(|e| format!("Erreur lecture réponse: {}", e))?;

    println!("Extraction...");

    // Extract to temp directory
    let temp_dir =
        tempfile::tempdir().map_err(|e| format!("Erreur création répertoire temp: {}", e))?;

    let tar_gz = GzDecoder::new(&bytes[..]);
    let mut archive = Archive::new(tar_gz);
    archive
        .unpack(temp_dir.path())
        .map_err(|e| format!("Erreur extraction archive: {}", e))?;

    let new_binary = temp_dir.path().join("roadmap-cli");
    if !new_binary.exists() {
        return Err("Binaire 'roadmap-cli' non trouvé dans l'archive".to_string());
    }

    // Get current exe path
    let current_exe = get_current_exe_path()?;

    println!("Installation...");

    // On Unix, we can replace the running binary
    // Create backup first
    let backup_path = current_exe.with_extension("old");
    if backup_path.exists() {
        fs::remove_file(&backup_path).ok();
    }

    // Rename current to backup
    fs::rename(&current_exe, &backup_path)
        .map_err(|e| format!("Erreur sauvegarde ancien binaire: {} (essayez avec sudo)", e))?;

    // Copy new binary
    match fs::copy(&new_binary, &current_exe) {
        Ok(_) => {
            // Set executable permissions on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&current_exe)
                    .map_err(|e| format!("Erreur lecture permissions: {}", e))?
                    .permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&current_exe, perms)
                    .map_err(|e| format!("Erreur définition permissions: {}", e))?;
            }

            // Remove backup
            fs::remove_file(&backup_path).ok();

            println!(
                "\n{} Mise à jour réussie ! Version {} installée.",
                "✓".green(),
                release.tag_name.green()
            );
            println!(
                "  Relancez {} pour utiliser la nouvelle version.",
                "roadmap".cyan()
            );

            Ok(())
        }
        Err(e) => {
            // Restore backup
            fs::rename(&backup_path, &current_exe).ok();
            Err(format!(
                "Erreur copie nouveau binaire: {} (essayez avec sudo)",
                e
            ))
        }
    }
}

/// Main entry point for update command
pub fn cmd_update(check_only: bool) {
    if check_only {
        println!("Vérification des mises à jour...");

        match check_for_update() {
            Ok(Some((version, url))) => {
                println!(
                    "\n{} Nouvelle version disponible: {}",
                    "→".yellow(),
                    version.green()
                );
                println!("  Version actuelle: {}", CURRENT_VERSION);
                println!("  Release: {}", url.cyan());
                println!(
                    "\n  Lancez {} pour mettre à jour.",
                    "roadmap update".yellow()
                );
            }
            Ok(None) => {
                println!(
                    "\n{} Vous utilisez la dernière version ({})",
                    "✓".green(),
                    CURRENT_VERSION
                );
            }
            Err(e) => {
                println!("{} {}", "Erreur:".red(), e);
            }
        }
    } else {
        println!("Mise à jour de roadmap-cli...\n");

        match perform_update() {
            Ok(()) => {}
            Err(e) => {
                println!("{} {}", "Erreur:".red(), e);
            }
        }
    }
}
