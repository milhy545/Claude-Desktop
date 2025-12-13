use crate::error::AppError;
use std::path::{Path, PathBuf};
use std::process::Output;
use tokio::fs;
use tokio::process::Command;

/// Abstraktní rozhraní pro systémové operace (Filesystem, Process, Environment)
/// Umožňuje snadné mockování v testech.
#[async_trait::async_trait]
pub trait SystemOps: Send + Sync {
    /// Načtení obsahu souboru jako string
    async fn read_to_string(&self, path: &Path) -> Result<String, AppError>;

    /// Zápis stringu do souboru
    async fn write(&self, path: &Path, content: &str) -> Result<(), AppError>;

    /// Vytvoření adresáře (včetně rodičů)
    async fn create_dir_all(&self, path: &Path) -> Result<(), AppError>;

    /// Kontrola existence cesty
    async fn exists(&self, path: &Path) -> bool;

    /// Smazání souboru
    async fn remove_file(&self, path: &Path) -> Result<(), AppError>;

    /// Smazání adresáře
    async fn remove_dir_all(&self, path: &Path) -> Result<(), AppError>;

    /// Získání domovského adresáře
    fn home_dir(&self) -> Option<PathBuf>;

    /// Získání konfiguračního adresáře
    fn config_dir(&self) -> Option<PathBuf>;

    /// Spuštění příkazu a čekání na výsledek
    async fn run_command(&self, command: &str, args: &[&str]) -> Result<Output, AppError>;
}

/// Skutečná implementace využívající tokio a std
pub struct RealSystemOps;

#[async_trait::async_trait]
impl SystemOps for RealSystemOps {
    async fn read_to_string(&self, path: &Path) -> Result<String, AppError> {
        fs::read_to_string(path).await.map_err(AppError::Io)
    }

    async fn write(&self, path: &Path, content: &str) -> Result<(), AppError> {
        // Ensure parent dir exists
        if let Some(parent) = path.parent() {
            if !self.exists(parent).await {
                self.create_dir_all(parent).await?;
            }
        }
        fs::write(path, content).await.map_err(AppError::Io)
    }

    async fn create_dir_all(&self, path: &Path) -> Result<(), AppError> {
        fs::create_dir_all(path).await.map_err(AppError::Io)
    }

    async fn exists(&self, path: &Path) -> bool {
        fs::try_exists(path).await.unwrap_or(false)
    }

    async fn remove_file(&self, path: &Path) -> Result<(), AppError> {
        fs::remove_file(path).await.map_err(AppError::Io)
    }

    async fn remove_dir_all(&self, path: &Path) -> Result<(), AppError> {
        fs::remove_dir_all(path).await.map_err(AppError::Io)
    }

    fn home_dir(&self) -> Option<PathBuf> {
        dirs::home_dir()
    }

    fn config_dir(&self) -> Option<PathBuf> {
        dirs::config_dir()
    }

    async fn run_command(&self, command: &str, args: &[&str]) -> Result<Output, AppError> {
        Command::new(command)
            .args(args)
            .output()
            .await
            .map_err(AppError::Io)
    }
}
