// MCP Server Launcher
// Spouští npx, python, nebo binary MCP servery

use std::process::{Command, Child, Stdio};

pub enum ServerType {
    NodeJs,  // npx
    Python,  // python
    Binary,  // executable
}

pub fn launch_server(
    server_type: ServerType,
    command: &str,
    args: &[String],
) -> Result<Child, String> {
    let child = match server_type {
        ServerType::NodeJs => {
            Command::new(command)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to start Node.js server: {}", e))?
        }
        ServerType::Python => {
            Command::new("python3")
                .arg(command)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to start Python server: {}", e))?
        }
        ServerType::Binary => {
            Command::new(command)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to start binary server: {}", e))?
        }
    };

    Ok(child)
}

pub fn detect_server_type(command: &str) -> ServerType {
    match command {
        "npx" | "node" => ServerType::NodeJs,
        "python" | "python3" => ServerType::Python,
        _ => ServerType::Binary,
    }
}
