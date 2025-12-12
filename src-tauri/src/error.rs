use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO chyba: {0}")]
    Io(#[from] std::io::Error),

    #[error("Chyba autentizace: {0}")]
    Auth(String),

    #[error("MCP chyba: {0}")]
    Mcp(String),

    #[error("Voice chyba: {0}")]
    Voice(String),

    #[error("Konfigurační chyba: {0}")]
    Config(String),

    #[error("Neznámá chyba: {0}")]
    Unknown(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("JSON chyba: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Tauri chyba: {0}")]
    Tauri(String),
}

// Implementace pro serializaci do frontendu
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

// Helper pro převod string chyb
impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Unknown(s)
    }
}
