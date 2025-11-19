// Voice module - Conversation history and voice settings management
// Handles storage of conversations and user preferences for voice features

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEntry {
    pub id: String,
    pub timestamp: i64,
    pub user_input: String,
    pub assistant_response: String,
    pub voice_used: bool,
    pub played_back: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    pub input_language: String,
    pub output_voice: String,
    pub output_speed: f32,
    pub auto_play: bool,
    pub history_limit: usize,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self {
            input_language: "cs-CZ".to_string(),
            output_voice: "default".to_string(),
            output_speed: 1.0,
            auto_play: false,
            history_limit: 100,
        }
    }
}

/// Get path to voice data directory
fn get_voice_dir() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir()
        .ok_or("Cannot find config directory")?;

    let voice_dir = config_dir.join("Claude").join("voice");

    std::fs::create_dir_all(&voice_dir)
        .map_err(|e| format!("Failed to create voice directory: {}", e))?;

    Ok(voice_dir)
}

/// Get path to conversations file
fn get_conversations_path() -> Result<PathBuf, String> {
    Ok(get_voice_dir()?.join("conversations.json"))
}

/// Get path to voice settings file
fn get_settings_path() -> Result<PathBuf, String> {
    Ok(get_voice_dir()?.join("voice_settings.json"))
}

/// Load all conversations from file
pub fn load_conversations() -> Result<Vec<ConversationEntry>, String> {
    let path = get_conversations_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read conversations: {}", e))?;

    let conversations: Vec<ConversationEntry> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse conversations: {}", e))?;

    Ok(conversations)
}

/// Save conversation entry
pub fn save_conversation(entry: ConversationEntry) -> Result<(), String> {
    let mut conversations = load_conversations()?;

    // Add new entry
    conversations.push(entry);

    // Load settings to get history limit
    let settings = load_voice_settings()?;

    // Keep only the most recent entries
    if conversations.len() > settings.history_limit {
        conversations = conversations
            .into_iter()
            .rev()
            .take(settings.history_limit)
            .rev()
            .collect();
    }

    // Save to file
    let path = get_conversations_path()?;
    let json = serde_json::to_string_pretty(&conversations)
        .map_err(|e| format!("Failed to serialize conversations: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("Failed to write conversations: {}", e))?;

    log::info!("💾 Saved conversation entry: {}", conversations.len());
    Ok(())
}

/// Clear all conversations
pub fn clear_conversations() -> Result<(), String> {
    let path = get_conversations_path()?;

    if path.exists() {
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to delete conversations: {}", e))?;
    }

    log::info!("🗑️  Cleared conversation history");
    Ok(())
}

/// Load voice settings
pub fn load_voice_settings() -> Result<VoiceSettings, String> {
    let path = get_settings_path()?;

    if !path.exists() {
        // Return defaults if file doesn't exist
        return Ok(VoiceSettings::default());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read voice settings: {}", e))?;

    let settings: VoiceSettings = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse voice settings: {}", e))?;

    Ok(settings)
}

/// Save voice settings
pub fn save_voice_settings(settings: &VoiceSettings) -> Result<(), String> {
    let path = get_settings_path()?;

    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize voice settings: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("Failed to write voice settings: {}", e))?;

    log::info!("💾 Saved voice settings");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_default_voice_settings() {
        let settings = VoiceSettings::default();
        assert_eq!(settings.input_language, "cs-CZ");
        assert_eq!(settings.output_speed, 1.0);
        assert_eq!(settings.auto_play, false);
        assert_eq!(settings.history_limit, 100);
    }

    #[test]
    fn test_conversation_entry_creation() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let entry = ConversationEntry {
            id: "test-123".to_string(),
            timestamp,
            user_input: "Test input".to_string(),
            assistant_response: "Test response".to_string(),
            voice_used: true,
            played_back: false,
        };

        assert_eq!(entry.id, "test-123");
        assert!(entry.voice_used);
        assert!(!entry.played_back);
    }

    #[test]
    fn test_voice_settings_serialization() {
        let settings = VoiceSettings {
            input_language: "en-US".to_string(),
            output_voice: "Google US English".to_string(),
            output_speed: 1.5,
            auto_play: true,
            history_limit: 50,
        };

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: VoiceSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.input_language, "en-US");
        assert_eq!(deserialized.output_speed, 1.5);
        assert_eq!(deserialized.history_limit, 50);
    }

    #[test]
    fn test_get_voice_dir() {
        let result = get_voice_dir();
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("Claude/voice"));
    }
}
