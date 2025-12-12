#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{is_authenticated, login};
    use crate::error::AppError;
    use crate::mocks::MockSystemOps;
    use crate::system::SystemOps;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_is_authenticated_false() {
        let mock = MockSystemOps::new();
        let sys: Arc<dyn SystemOps> = Arc::new(mock);

        let result = is_authenticated(&sys).await.unwrap();
        assert_eq!(result, false);
    }

    #[tokio::test]
    async fn test_is_authenticated_true() {
        let mock = MockSystemOps::new().with_file("/home/mockuser/.claude", "dir placeholder");

        let sys: Arc<dyn SystemOps> = Arc::new(mock);

        let result = is_authenticated(&sys).await.unwrap();
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn test_login_success() {
        let mock = MockSystemOps::new().with_command_output("claude", true, "", "");
        let sys: Arc<dyn SystemOps> = Arc::new(mock);

        let result = login(&sys).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Přihlášení úspěšné!");
    }

    #[tokio::test]
    async fn test_login_failure() {
        let mock = MockSystemOps::new().with_command_output("claude", false, "", "Auth error");
        let sys: Arc<dyn SystemOps> = Arc::new(mock);

        let result = login(&sys).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Auth(msg) => assert!(msg.contains("Auth error")),
            _ => panic!("Unexpected error type"),
        }
    }
}
