# Voice Features Design Document

## Overview
Adding speech-to-text (voice dictation) and text-to-speech (response playback) capabilities to Claude Desktop for improved accessibility and hands-free operation.

## User Requirements
- **Voice Input**: Speak instead of typing to reduce keyboard strain
- **Response Playback**: Listen to responses instead of reading
- **Memory**: Store conversation history for playback

## Technical Design

### 1. Speech-to-Text (Diktování)
**Technology**: Web Speech API - SpeechRecognition
**Browser Support**: Chrome, Edge (WebKit-based browsers on Linux)

**Features**:
- Real-time voice recognition
- Language support: Czech (cs-CZ) and English (en-US)
- Continuous recognition mode
- Visual feedback (microphone active state)
- Append to existing text or replace

**Implementation**:
```javascript
const recognition = new webkitSpeechRecognition();
recognition.lang = 'cs-CZ';
recognition.continuous = false;
recognition.interimResults = true;
```

### 2. Text-to-Speech (Přehrávání)
**Technology**: Web Speech API - SpeechSynthesis
**Browser Support**: All modern browsers

**Features**:
- Read assistant responses aloud
- Voice selection (male/female, different accents)
- Playback speed control (0.5x - 2.0x)
- Pause/resume/stop controls
- Highlight current word being spoken (optional)

**Implementation**:
```javascript
const utterance = new SpeechSynthesisUtterance(text);
utterance.lang = 'cs-CZ';
utterance.rate = 1.0;
speechSynthesis.speak(utterance);
```

### 3. Conversation History
**Storage**: localStorage + optional Rust backend

**Data Structure**:
```json
{
  "conversations": [
    {
      "id": "uuid",
      "timestamp": 1637012345678,
      "user_input": "Jak funguje fotosyntéza?",
      "assistant_response": "Fotosyntéza je proces...",
      "voice_used": false,
      "played_back": true
    }
  ]
}
```

## UI Design

### Voice Input Button
- Location: Next to chat input field
- Icon: 🎤 (microphone)
- States:
  - Idle: Gray microphone
  - Listening: Red pulsing microphone
  - Processing: Yellow microphone with spinner
  - Error: Red microphone with X

### Voice Output Controls
- Location: Below each assistant response
- Buttons:
  - ▶️ Play - Start reading response
  - ⏸️ Pause - Pause playback
  - ⏹️ Stop - Stop playback
  - 🔊 Volume slider
  - ⚡ Speed selector (0.5x, 0.75x, 1x, 1.25x, 1.5x, 2x)

### Settings Panel
New section: "Voice Settings"
- Input language: Dropdown (Czech, English)
- Output voice: Dropdown (list of available voices)
- Output speed: Slider (0.5x - 2.0x)
- Auto-play responses: Checkbox
- Conversation history limit: Number input (default: 100)

## Implementation Plan

### Phase 1: Core Voice Features
1. Add SpeechRecognition for voice input
2. Add SpeechSynthesis for response playback
3. Basic UI controls (mic button, play button)

### Phase 2: Enhanced Controls
1. Voice selection dropdown
2. Speed control slider
3. Visual feedback improvements
4. Error handling and fallbacks

### Phase 3: Conversation History
1. localStorage implementation
2. History viewer UI
3. Search/filter history
4. Export history (JSON, TXT)

### Phase 4: Advanced Features
1. Auto-play mode (automatically read responses)
2. Voice commands ("start dictation", "read that again")
3. Keyboard shortcuts (Ctrl+M for mic, Ctrl+Shift+P for play)
4. Multi-language auto-detection

## API Changes

### New Tauri Commands

```rust
// Save conversation entry
#[tauri::command]
fn save_conversation(entry: ConversationEntry) -> Result<(), String>

// Load conversation history
#[tauri::command]
fn load_conversations(limit: usize) -> Result<Vec<ConversationEntry>, String>

// Clear conversation history
#[tauri::command]
fn clear_conversations() -> Result<(), String>

// Get voice settings
#[tauri::command]
fn get_voice_settings() -> Result<VoiceSettings, String>

// Save voice settings
#[tauri::command]
fn save_voice_settings(settings: VoiceSettings) -> Result<(), String>
```

### Data Structures

```rust
#[derive(Serialize, Deserialize)]
struct ConversationEntry {
    id: String,
    timestamp: i64,
    user_input: String,
    assistant_response: String,
    voice_used: bool,
    played_back: bool,
}

#[derive(Serialize, Deserialize)]
struct VoiceSettings {
    input_language: String,
    output_voice: String,
    output_speed: f32,
    auto_play: bool,
    history_limit: usize,
}
```

## Browser Compatibility

### SpeechRecognition
- ✅ Chrome/Chromium (prefixed: webkitSpeechRecognition)
- ✅ Edge
- ⚠️ Firefox (limited support)
- ✅ WebKitGTK (used by Tauri on Linux)

### SpeechSynthesis
- ✅ Chrome/Chromium
- ✅ Edge
- ✅ Firefox
- ✅ WebKitGTK

## Privacy Considerations

1. **Local Processing**: Web Speech API processes on-device when possible
2. **Data Storage**: Conversation history stored locally only
3. **No Cloud Upload**: Voice data never sent to third-party servers (except browser's speech service)
4. **User Control**: Clear history button, opt-in for history saving

## Testing Strategy

### Unit Tests
- Voice settings save/load
- Conversation history CRUD operations
- Language/voice selection logic

### Integration Tests
- SpeechRecognition start/stop
- SpeechSynthesis playback
- UI state changes

### Manual Testing
- Test with different voices
- Test with different speeds
- Test language switching
- Test on different Linux distros

## Accessibility Benefits

1. **Reduced Physical Strain**: Less typing reduces RSI risk
2. **Multitasking**: Listen while doing other tasks
3. **Visual Impairment Support**: Full audio interface
4. **Learning Assistance**: Hear proper pronunciation
5. **Fatigue Reduction**: Less screen time for reading

## Performance Metrics

- Voice recognition latency: < 500ms
- TTS initialization: < 200ms
- History load time: < 100ms for 100 entries
- Memory overhead: < 10 MB for voice features

## Future Enhancements

1. Offline voice models (using Vosk or Mozilla DeepSpeech)
2. Custom wake word ("Hey Claude")
3. Voice biometrics for user identification
4. Emotion detection in voice
5. Voice translation (speak Czech, transcribe to English)

---

**Version**: 0.3.0
**Author**: Claude Desktop Team
**Date**: 2025-11-19
