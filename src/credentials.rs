//! Credentials storage (~/.roadmap/credentials.json)

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub server: String,
    pub access_token: String,
    pub refresh_token: String,
    pub user_email: String,
    pub user_name: String,
}

fn credentials_path() -> PathBuf {
    let home = dirs_next().unwrap_or_else(|| PathBuf::from("."));
    let dir = home.join(".roadmap");
    dir
}

fn credentials_file() -> PathBuf {
    credentials_path().join("credentials.json")
}

fn dirs_next() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(PathBuf::from)
}

pub fn save(creds: &Credentials) -> Result<(), String> {
    let dir = credentials_path();
    fs::create_dir_all(&dir).map_err(|e| format!("Erreur création ~/.roadmap: {}", e))?;

    let json = serde_json::to_string_pretty(creds)
        .map_err(|e| format!("Erreur sérialisation: {}", e))?;

    fs::write(credentials_file(), json)
        .map_err(|e| format!("Erreur écriture credentials: {}", e))?;

    // Restrict permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o600);
        let _ = fs::set_permissions(credentials_file(), perms);
    }

    Ok(())
}

pub fn load() -> Option<Credentials> {
    let file = credentials_file();
    if !file.exists() {
        return None;
    }

    let content = fs::read_to_string(file).ok()?;
    serde_json::from_str(&content).ok()
}

pub fn clear() -> Result<(), String> {
    let file = credentials_file();
    if file.exists() {
        fs::remove_file(file).map_err(|e| format!("Erreur suppression: {}", e))?;
    }
    Ok(())
}
