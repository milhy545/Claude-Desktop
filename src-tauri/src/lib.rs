// Library interface for Tauri build system
// This file is required by Tauri's build process

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Re-export modules for Tauri
pub mod auth;
pub mod debug;
pub mod mcp;
pub mod voice;
