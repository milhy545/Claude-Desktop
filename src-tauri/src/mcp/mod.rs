// MCP (Model Context Protocol) module
// Správa MCP serverů

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Child, Command};
use dirs::config_dir;

pub mod config;
pub mod launcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    #[serde(skip)]
    pub process: Option<u32>, // PID running procesu
}

/// Načte MCP konfiguraci z ~/.config/Claude/claude_desktop_config.json
pub fn load_config() -> Result<String, String> {
    let config_path = get_config_path();

    if !config_path.exists() {
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

    std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Nepodařilo se načíst config: {}", e))
}

/// Uloží MCP konfiguraci
pub fn save_config(config: &str) -> Result<(), String> {
    let config_path = get_config_path();

    // Vytvoř parent directory, pokud neexistuje
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Nepodařilo se vytvořit config directory: {}", e))?;
    }

    std::fs::write(&config_path, config)
        .map_err(|e| format!("Nepodařilo se uložit config: {}", e))
}

/// Vrátí cestu k config souboru
fn get_config_path() -> PathBuf {
    config_dir()
        .expect("Nelze najít config directory")
        .join("Claude")
        .join("claude_desktop_config.json")
}

/// Spustí MCP server
pub fn start_server(name: &str, state: &tauri::State<crate::AppState>) -> Result<(), String> {
    // TODO: Načíst konfiguraci a spustit server
    println!("🚀 Starting MCP server: {}", name);
    Ok(())
}

/// Zastaví MCP server
pub fn stop_server(name: &str, state: &tauri::State<crate::AppState>) -> Result<(), String> {
    // TODO: Zastavit running server
    println!("🛑 Stopping MCP server: {}", name);
    Ok(())
}

/// Parsuje config a vrátí seznam serverů
pub fn parse_config(config_json: &str) -> Result<Vec<McpServer>, String> {
    let config: serde_json::Value = serde_json::from_str(config_json)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    let mut servers = Vec::new();

    if let Some(mcp_servers) = config.get("mcpServers").and_then(|v| v.as_object()) {
        for (name, server_config) in mcp_servers {
            if let Some(obj) = server_config.as_object() {
                let command = obj.get("command")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let args = obj.get("args")
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
