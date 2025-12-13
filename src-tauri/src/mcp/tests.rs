#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::{load_config, parse_config, save_config};
    use crate::mocks::MockSystemOps;
    use crate::system::SystemOps;
    use std::path::PathBuf;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_load_default_config() {
        let mock = MockSystemOps::new();
        let sys: Arc<dyn SystemOps> = Arc::new(mock);

        let config = load_config(&sys).await.unwrap();
        assert!(config.contains("mcpServers"));
        assert!(config.contains("filesystem"));
    }

    #[tokio::test]
    async fn test_load_existing_config() {
        let expected_json = r#"{"mcpServers": {"test": {"command": "echo", "args": ["hello"]}}}"#;
        let config_path = "/home/mockuser/.config/Claude/claude_desktop_config.json";

        let mock = MockSystemOps::new().with_file(config_path, expected_json);
        let sys: Arc<dyn SystemOps> = Arc::new(mock);

        let config = load_config(&sys).await.unwrap();
        assert_eq!(config, expected_json);
    }

    #[tokio::test]
    async fn test_save_config() {
        let mock = MockSystemOps::new();
        let sys: Arc<dyn SystemOps> = Arc::new(mock);

        let config_data = r#"{"test": true}"#;
        save_config(&sys, config_data).await.unwrap();

        let path = PathBuf::from("/home/mockuser/.config/Claude/claude_desktop_config.json");
        let saved = sys.read_to_string(&path).await.unwrap();
        assert_eq!(saved, config_data);
    }

    #[test]
    fn test_parse_config() {
        let json = r#"{
            "mcpServers": {
                "py": {
                    "command": "python3",
                    "args": ["server.py"]
                }
            }
        }"#;

        let servers = parse_config(json).unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].name, "py");
        assert_eq!(servers[0].command, "python3");
        assert_eq!(servers[0].args[0], "server.py");
    }
}
