// MCP Module Tests

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_parse_config_empty() {
        let config = r#"{
            "mcpServers": {}
        }"#;

        let result = parse_config(config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_config_with_servers() {
        let config = r#"{
            "mcpServers": {
                "filesystem": {
                    "command": "npx",
                    "args": ["-y", "@modelcontextprotocol/server-filesystem"]
                }
            }
        }"#;

        let result = parse_config(config);
        assert!(result.is_ok());

        let servers = result.unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].name, "filesystem");
        assert_eq!(servers[0].command, "npx");
    }

    #[test]
    fn test_parse_config_invalid_json() {
        let config = "{ invalid json }";
        let result = parse_config(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_config_path() {
        let path = super::super::get_config_path();
        assert!(path.to_string_lossy().contains("Claude"));
        assert!(path.to_string_lossy().contains("claude_desktop_config.json"));
    }
}
