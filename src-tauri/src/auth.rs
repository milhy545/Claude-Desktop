// Authentication module
// Integrace s claude CLI authentication

use crate::error::AppError;
use crate::system::SystemOps;
use std::path::PathBuf;
use std::sync::Arc;

#[cfg(test)]
#[path = "auth_tests.rs"]
mod tests;

/// Cesta k Claude CLI session
fn get_session_path(sys: &dyn SystemOps) -> PathBuf {
    sys.home_dir()
        .expect("Nelze najít home directory")
        .join(".claude")
}

/// Zkontroluje, jestli je uživatel přihlášený
pub async fn is_authenticated(sys: &Arc<dyn SystemOps>) -> Result<bool, AppError> {
    let session_dir = get_session_path(sys.as_ref());

    // Zkontroluj, jestli existuje session directory
    if !sys.exists(&session_dir).await {
        return Ok(false);
    }

    Ok(true)
}

/// Spustí Claude CLI login proces
pub async fn login(sys: &Arc<dyn SystemOps>) -> Result<String, AppError> {
    // Spustí `claude auth login` proces
    let output = sys.run_command("claude", &["auth", "login"]).await?;

    if output.status.success() {
        Ok("Přihlášení úspěšné!".to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(AppError::Auth(format!("Přihlášení selhalo: {}", error)))
    }
}

/// Vymaže session (logout)
#[allow(dead_code)]
pub async fn logout(sys: &Arc<dyn SystemOps>) -> Result<(), AppError> {
    let session_dir = get_session_path(sys.as_ref());

    if sys.exists(&session_dir).await {
        sys.remove_dir_all(&session_dir).await?;
    }

    Ok(())
}
