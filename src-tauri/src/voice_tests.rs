#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::MockSystemOps;
    use crate::system::SystemOps;
    use crate::voice::{
        load_conversations, save_conversation, save_voice_settings, ConversationEntry,
        VoiceSettings,
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn test_save_and_load_conversation() {
        let mock = MockSystemOps::new();
        let sys: Arc<dyn SystemOps> = Arc::new(mock);

        let entry = ConversationEntry {
            id: "1".to_string(),
            timestamp: 100,
            user_input: "Hi".to_string(),
            assistant_response: "Hello".to_string(),
            voice_used: true,
            played_back: true,
        };

        save_conversation(&sys, entry.clone()).await.unwrap();

        let loaded = load_conversations(&sys).await.unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, "1");
    }

    #[tokio::test]
    async fn test_history_limit() {
        let mock = MockSystemOps::new();
        let sys: Arc<dyn SystemOps> = Arc::new(mock);

        // Set limit to 2
        let settings = VoiceSettings {
            history_limit: 2,
            ..Default::default()
        };
        save_voice_settings(&sys, &settings).await.unwrap();

        // Add 3 entries
        for i in 1..=3 {
            save_conversation(
                &sys,
                ConversationEntry {
                    id: i.to_string(),
                    timestamp: i,
                    user_input: format!("Q{}", i),
                    assistant_response: format!("A{}", i),
                    voice_used: false,
                    played_back: false,
                },
            )
            .await
            .unwrap();
        }

        let loaded = load_conversations(&sys).await.unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, "2"); // Oldest should be dropped (1 dropped, 2 and 3 remain)
        assert_eq!(loaded[1].id, "3");
    }
}
