// MCP Config helper

pub fn expand_path(path: &str) -> String {
    // Expandne $USER, ~, atd.
    path.replace("$USER", &std::env::var("USER").unwrap_or_default())
        .replace("~", &dirs::home_dir().unwrap().to_string_lossy())
}
