// Claude Desktop - Tauri Edition
// Lightweight Linux desktop client for Claude AI

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::sync::Mutex;

// Moduly
mod auth;
mod mcp;
mod debug;

// Globální stav aplikace
struct AppState {
    session: Mutex<Option<String>>,
    mcp_servers: Mutex<Vec<mcp::McpServer>>,
}

// Tauri commands (volané z JavaScriptu)
#[tauri::command]
fn check_auth() -> Result<bool, String> {
    auth::is_authenticated()
}

#[tauri::command]
async fn login() -> Result<String, String> {
    auth::login().await
}

#[tauri::command]
fn get_mcp_servers(state: tauri::State<AppState>) -> Result<Vec<String>, String> {
    let servers = state.mcp_servers.lock().unwrap();
    Ok(servers.iter().map(|s| s.name.clone()).collect())
}

#[tauri::command]
fn start_mcp_server(name: String, state: tauri::State<AppState>) -> Result<(), String> {
    mcp::start_server(&name, &state)
}

#[tauri::command]
fn stop_mcp_server(name: String, state: tauri::State<AppState>) -> Result<(), String> {
    mcp::stop_server(&name, &state)
}

#[tauri::command]
fn load_mcp_config() -> Result<String, String> {
    mcp::load_config()
}

#[tauri::command]
fn save_mcp_config(config: String) -> Result<(), String> {
    mcp::save_config(&config)
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn get_system_info() -> Result<String, String> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    Ok(format!("OS: {}, Arch: {}", os, arch))
}

#[tauri::command]
fn open_config_dir() -> Result<(), String> {
    use std::process::Command;

    let config_path = dirs::config_dir()
        .ok_or("Cannot find config directory")?
        .join("Claude");

    std::fs::create_dir_all(&config_path)
        .map_err(|e| format!("Failed to create config dir: {}", e))?;

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&config_path)
            .spawn()
            .map_err(|e| format!("Failed to open config dir: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
fn switch_view(app: tauri::AppHandle, view: String) -> Result<(), String> {
    use tauri::Manager;

    let url = match view.as_str() {
        "chat" => "https://claude.ai",
        "code" => "https://claude.ai/code",
        _ => return Err(format!("Unknown view: {}", view)),
    };

    // Get the main window
    if let Some(window) = app.get_webview_window("main") {
        // Emit event to change iframe URL
        window.emit("change-view", url)
            .map_err(|e| format!("Failed to emit event: {}", e))?;

        log::info!("🔄 Switched view to: {}", view);
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

fn main() {
    // Inicializace loggingu
    debug::init_logging();
    debug::log_system_info();

    // Inicializace aplikace
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            session: Mutex::new(None),
            mcp_servers: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            check_auth,
            login,
            get_mcp_servers,
            start_mcp_server,
            stop_mcp_server,
            load_mcp_config,
            save_mcp_config,
            get_app_version,
            get_system_info,
            open_config_dir,
            switch_view,
        ])
        .setup(|app| {
            // Inicializace system tray
            let tray = app.tray_by_id("main").unwrap();

            // Global hotkey: Ctrl+Alt+Space
            #[cfg(target_os = "linux")]
            {
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
                let handle = app.handle();
                app.global_shortcut().register(
                    Code::Space,
                    Modifiers::CONTROL | Modifiers::ALT,
                    move |_app, _shortcut, event| {
                        if event.state == ShortcutState::Pressed {
                            if let Some(window) = handle.get_webview_window("main") {
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                        }
                    },
                )?;
            }

            println!("🦀 Claude Desktop (Tauri) started!");
            println!("📦 Memory footprint: ~30-50 MB (vs Electron ~200-400 MB)");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
