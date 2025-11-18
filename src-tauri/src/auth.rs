// Authentication module
// Integrace s claude CLI authentication

use std::process::Command;
use std::path::PathBuf;
use dirs::home_dir;

/// Cesta k Claude CLI session
fn get_session_path() -> PathBuf {
    home_dir()
        .expect("Nelze najít home directory")
        .join(".claude")
}

/// Zkontroluje, jestli je uživatel přihlášený
pub fn is_authenticated() -> Result<bool, String> {
    let session_dir = get_session_path();

    // Zkontroluj, jestli existuje session directory
    if !session_dir.exists() {
        return Ok(false);
    }

    // Zkontroluj, jestli jsou nějaké session soubory
    let has_session = session_dir.read_dir()
        .map(|mut dir| dir.any(|_| true))
        .unwrap_or(false);

    Ok(has_session)
}

/// Spustí Claude CLI login proces
pub async fn login() -> Result<String, String> {
    // Spustí `claude auth login` proces
    let output = Command::new("claude")
        .arg("auth")
        .arg("login")
        .output()
        .map_err(|e| format!("Nepodařilo se spustit claude CLI: {}", e))?;

    if output.status.success() {
        Ok("Přihlášení úspěšné!".to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Přihlášení selhalo: {}", error))
    }
}

/// Načte session token (pokud existuje)
pub fn get_session_token() -> Option<String> {
    // TODO: Implementovat načítání tokenu z ~/.claude/
    // Zatím placeholder
    None
}

/// Vymaže session (logout)
pub fn logout() -> Result<(), String> {
    let session_dir = get_session_path();

    if session_dir.exists() {
        std::fs::remove_dir_all(&session_dir)
            .map_err(|e| format!("Nepodařilo se smazat session: {}", e))?;
    }

    Ok(())
}
