// Voice module - Conversation history and voice settings management
// Handles storage of conversations and user preferences for voice features

use crate::error::AppError;
use crate::system::SystemOps;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[cfg(test)]
#[path = "voice_tests.rs"]
mod tests;

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
async fn get_voice_dir(sys: &dyn SystemOps) -> Result<PathBuf, AppError> {
    let config_dir = sys
        .config_dir()
        .ok_or(AppError::Config("Cannot find config directory".to_string()))?;

    let voice_dir = config_dir.join("Claude").join("voice");

    if !sys.exists(&voice_dir).await {
        sys.create_dir_all(&voice_dir).await?;
    }

    Ok(voice_dir)
}

/// Get path to conversations file
async fn get_conversations_path(sys: &dyn SystemOps) -> Result<PathBuf, AppError> {
    Ok(get_voice_dir(sys).await?.join("conversations.json"))
}

/// Get path to voice settings file
async fn get_settings_path(sys: &dyn SystemOps) -> Result<PathBuf, AppError> {
    Ok(get_voice_dir(sys).await?.join("voice_settings.json"))
}

/// Load all conversations from file
pub async fn load_conversations(
    sys: &Arc<dyn SystemOps>,
) -> Result<Vec<ConversationEntry>, AppError> {
    let path = get_conversations_path(sys.as_ref()).await?;

    if !sys.exists(&path).await {
        return Ok(Vec::new());
    }

    let content = sys.read_to_string(&path).await?;

    let conversations: Vec<ConversationEntry> =
        serde_json::from_str(&content).map_err(AppError::Json)?;

    Ok(conversations)
}

/// Save conversation entry
pub async fn save_conversation(
    sys: &Arc<dyn SystemOps>,
    entry: ConversationEntry,
) -> Result<(), AppError> {
    let mut conversations = load_conversations(sys).await?;

    // Add new entry
    conversations.push(entry);

    // Load settings to get history limit
    let settings = load_voice_settings(sys).await?;

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
    let path = get_conversations_path(sys.as_ref()).await?;
    let json = serde_json::to_string_pretty(&conversations).map_err(AppError::Json)?;

    sys.write(&path, &json).await?;

    log::info!("💾 Saved conversation entry: {}", conversations.len());
    Ok(())
}

/// Clear all conversations
pub async fn clear_conversations(sys: &Arc<dyn SystemOps>) -> Result<(), AppError> {
    let path = get_conversations_path(sys.as_ref()).await?;

    if sys.exists(&path).await {
        sys.remove_file(&path).await?;
    }

    log::info!("🗑️  Cleared conversation history");
    Ok(())
}

/// Load voice settings
pub async fn load_voice_settings(sys: &Arc<dyn SystemOps>) -> Result<VoiceSettings, AppError> {
    let path = get_settings_path(sys.as_ref()).await?;

    if !sys.exists(&path).await {
        // Return defaults if file doesn't exist
        return Ok(VoiceSettings::default());
    }

    let content = sys.read_to_string(&path).await?;

    let settings: VoiceSettings = serde_json::from_str(&content).map_err(AppError::Json)?;

    Ok(settings)
}

/// Save voice settings
pub async fn save_voice_settings(
    sys: &Arc<dyn SystemOps>,
    settings: &VoiceSettings,
) -> Result<(), AppError> {
    let path = get_settings_path(sys.as_ref()).await?;

    let json = serde_json::to_string_pretty(settings).map_err(AppError::Json)?;

    sys.write(&path, &json).await?;

    log::info!("💾 Saved voice settings");
    Ok(())
}
