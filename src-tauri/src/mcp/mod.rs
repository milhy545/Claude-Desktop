// MCP (Model Context Protocol) module
// Správa MCP serverů

use crate::error::AppError;
use crate::system::SystemOps;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use crate::state::AppState;

pub mod config;
pub mod launcher;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    #[serde(skip)]
    pub process: Option<u32>, // PID running procesu
}

/// Vrátí cestu k config souboru
pub(crate) async fn get_config_path(sys: &dyn SystemOps) -> Result<PathBuf, AppError> {
    sys.config_dir()
        .ok_or(AppError::Config("Nelze najít config directory".to_string()))
        .map(|d| d.join("Claude").join("claude_desktop_config.json"))
}

/// Načte MCP konfiguraci z ~/.config/Claude/claude_desktop_config.json
pub async fn load_config(sys: &Arc<dyn SystemOps>) -> Result<String, AppError> {
    let _timer = crate::debug::PerfTimer::with_threshold("load_mcp_config", 100);

    let config_path = get_config_path(sys.as_ref()).await?;

    if !sys.exists(&config_path).await {
        // Vytvoř výchozí konfiguraci
        let default_config = r#"{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-filesystem",
        "/home/$USER/Documents"
      ]
    }
  }
}"#;
        return Ok(default_config.to_string());
    }

    sys.read_to_string(&config_path).await
}

/// Uloží MCP konfiguraci
pub async fn save_config(sys: &Arc<dyn SystemOps>, config: &str) -> Result<(), AppError> {
    let _timer = crate::debug::PerfTimer::with_threshold("save_mcp_config", 100);

    let config_path = get_config_path(sys.as_ref()).await?;

    sys.write(&config_path, config).await
}

/// Spustí MCP server
pub async fn start_server(name: &str, _state: &tauri::State<'_, AppState>) -> Result<(), AppError> {
    // TODO: Načíst konfiguraci a spustit server
    // Pro spouštění procesu budeme muset rozšířit SystemOps o spawn metodu, která vrací Child handle
    // Zatím jen log
    println!("🚀 Starting MCP server: {}", name);
    Ok(())
}

/// Zastaví MCP server
pub async fn stop_server(name: &str, _state: &tauri::State<'_, AppState>) -> Result<(), AppError> {
    // TODO: Zastavit running server
    println!("🛑 Stopping MCP server: {}", name);
    Ok(())
}

/// Parsuje config a vrátí seznam serverů
pub fn parse_config(config_json: &str) -> Result<Vec<McpServer>, AppError> {
    let config: serde_json::Value =
        serde_json::from_str(config_json).map_err(AppError::Json)?;

    let mut servers = Vec::new();

    if let Some(mcp_servers) = config.get("mcpServers").and_then(|v| v.as_object()) {
        for (name, server_config) in mcp_servers {
            if let Some(obj) = server_config.as_object() {
                let command = obj
                    .get("command")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let args = obj
                    .get("args")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| s.to_string())
                            .collect()
                    })
                    .unwrap_or_default();

                servers.push(McpServer {
                    name: name.clone(),
                    command,
                    args,
                    process: None,
                });
            }
        }
    }

    Ok(servers)
}
