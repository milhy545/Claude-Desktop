// Voice features - Speech-to-Text and Text-to-Speech
// Handles voice input (dictation) and voice output (reading responses)

const { invoke } = window.__TAURI__.core;

class VoiceManager {
    constructor() {
        this.recognition = null;
        this.synthesis = window.speechSynthesis;
        this.isListening = false;
        this.isSpeaking = false;
        this.settings = null;
        this.currentUtterance = null;

        this.initializeSpeechRecognition();
        this.loadSettings();
    }

    // Initialize Speech Recognition (STT)
    initializeSpeechRecognition() {
        if (!('webkitSpeechRecognition' in window) && !('SpeechRecognition' in window)) {
            console.warn('⚠️ Speech Recognition not supported in this browser');
            return;
        }

        const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
        this.recognition = new SpeechRecognition();

        this.recognition.continuous = false;
        this.recognition.interimResults = true;
        this.recognition.maxAlternatives = 1;

        this.recognition.onstart = () => {
            this.isListening = true;
            this.updateMicButton('listening');
            log('🎤 Voice recognition started');
        };

        this.recognition.onresult = (event) => {
            let interimTranscript = '';
            let finalTranscript = '';

            for (let i = event.resultIndex; i < event.results.length; i++) {
                const transcript = event.results[i][0].transcript;
                if (event.results[i].isFinal) {
                    finalTranscript += transcript;
                } else {
                    interimTranscript += transcript;
                }
            }

            if (finalTranscript) {
                this.handleTranscript(finalTranscript);
                log('✅ Recognized: ' + finalTranscript);
            }
        };

        this.recognition.onerror = (event) => {
            console.error('❌ Speech recognition error:', event.error);
            this.isListening = false;
            this.updateMicButton('idle');

            if (event.error === 'no-speech') {
                showNotification('Nebyl zachycen žádný hlas', 'warning');
            } else if (event.error === 'not-allowed') {
                showNotification('Přístup k mikrofonu byl zamítnut', 'error');
            } else {
                showNotification('Chyba rozpoznávání hlasu: ' + event.error, 'error');
            }
        };

        this.recognition.onend = () => {
            this.isListening = false;
            this.updateMicButton('idle');
            log('🛑 Voice recognition stopped');
        };
    }

    // Load voice settings
    async loadSettings() {
        try {
            this.settings = await invoke('get_voice_settings');

            // Set language for recognition
            if (this.recognition) {
                this.recognition.lang = this.settings.input_language;
            }

            log('⚙️ Voice settings loaded:', this.settings);
        } catch (error) {
            console.error('Failed to load voice settings:', error);
            // Use defaults if loading fails
            this.settings = {
                input_language: 'cs-CZ',
                output_voice: 'default',
                output_speed: 1.0,
                auto_play: false,
                history_limit: 100
            };
        }
    }

    // Start listening
    startListening() {
        if (!this.recognition) {
            showNotification('Rozpoznávání hlasu není podporováno', 'error');
            return;
        }

        if (this.isListening) {
            this.stopListening();
            return;
        }

        try {
            this.recognition.lang = this.settings.input_language;
            this.recognition.start();
        } catch (error) {
            console.error('Failed to start recognition:', error);
            showNotification('Nepodařilo se spustit rozpoznávání hlasu', 'error');
        }
    }

    // Stop listening
    stopListening() {
        if (this.recognition && this.isListening) {
            this.recognition.stop();
        }
    }

    // Handle recognized transcript
    handleTranscript(text) {
        const textarea = document.getElementById('chatInput');
        if (textarea) {
            // Append to existing text or replace
            if (textarea.value.trim()) {
                textarea.value += ' ' + text;
            } else {
                textarea.value = text;
            }

            // Trigger input event for any listeners
            textarea.dispatchEvent(new Event('input'));
        }
    }

    // Speak text (TTS)
    speak(text, options = {}) {
        if (this.isSpeaking) {
            this.stopSpeaking();
        }

        if (!this.synthesis) {
            showNotification('Syntéza řeči není podporována', 'error');
            return;
        }

        this.currentUtterance = new SpeechSynthesisUtterance(text);

        // Apply settings
        this.currentUtterance.lang = this.settings.input_language;
        this.currentUtterance.rate = options.rate || this.settings.output_speed;
        this.currentUtterance.pitch = options.pitch || 1.0;
        this.currentUtterance.volume = options.volume || 1.0;

        // Select voice if specified
        if (this.settings.output_voice && this.settings.output_voice !== 'default') {
            const voices = this.synthesis.getVoices();
            const selectedVoice = voices.find(v => v.name === this.settings.output_voice);
            if (selectedVoice) {
                this.currentUtterance.voice = selectedVoice;
            }
        }

        this.currentUtterance.onstart = () => {
            this.isSpeaking = true;
            log('🔊 Started speaking');
        };

        this.currentUtterance.onend = () => {
            this.isSpeaking = false;
            log('🔇 Finished speaking');
        };

        this.currentUtterance.onerror = (event) => {
            console.error('Speech synthesis error:', event);
            this.isSpeaking = false;
            showNotification('Chyba přehrávání hlasu', 'error');
        };

        this.synthesis.speak(this.currentUtterance);
    }

    // Stop speaking
    stopSpeaking() {
        if (this.synthesis && this.isSpeaking) {
            this.synthesis.cancel();
            this.isSpeaking = false;
        }
    }

    // Pause speaking
    pauseSpeaking() {
        if (this.synthesis && this.isSpeaking) {
            this.synthesis.pause();
        }
    }

    // Resume speaking
    resumeSpeaking() {
        if (this.synthesis && this.synthesis.paused) {
            this.synthesis.resume();
        }
    }

    // Get available voices
    getVoices() {
        if (!this.synthesis) return [];
        return this.synthesis.getVoices();
    }

    // Save conversation entry
    async saveConversation(userInput, assistantResponse, voiceUsed = false) {
        try {
            const entry = {
                id: crypto.randomUUID(),
                timestamp: Date.now(),
                user_input: userInput,
                assistant_response: assistantResponse,
                voice_used: voiceUsed,
                played_back: false
            };

            await invoke('save_conversation', { entry });
            log('💾 Conversation saved');
        } catch (error) {
            console.error('Failed to save conversation:', error);
        }
    }

    // Load conversation history
    async loadConversations() {
        try {
            const conversations = await invoke('load_conversations');
            log(`📚 Loaded ${conversations.length} conversations`);
            return conversations;
        } catch (error) {
            console.error('Failed to load conversations:', error);
            return [];
        }
    }

    // Clear conversation history
    async clearConversations() {
        try {
            await invoke('clear_conversations');
            log('🗑️ Conversation history cleared');
            showNotification('Historie konverzací smazána', 'success');
        } catch (error) {
            console.error('Failed to clear conversations:', error);
            showNotification('Nepodařilo se smazat historii', 'error');
        }
    }

    // Update voice settings
    async updateSettings(newSettings) {
        try {
            await invoke('save_voice_settings', { settings: newSettings });
            this.settings = newSettings;

            // Update recognition language
            if (this.recognition) {
                this.recognition.lang = newSettings.input_language;
            }

            log('⚙️ Voice settings updated');
            showNotification('Nastavení uloženo', 'success');
        } catch (error) {
            console.error('Failed to save voice settings:', error);
            showNotification('Nepodařilo se uložit nastavení', 'error');
        }
    }

    // Update microphone button state
    updateMicButton(state) {
        const micButton = document.getElementById('micButton');
        if (!micButton) return;

        micButton.className = 'voice-btn mic-btn';

        switch (state) {
            case 'listening':
                micButton.classList.add('listening');
                micButton.title = 'Nahrávám... (klikněte pro zastavení)';
                break;
            case 'processing':
                micButton.classList.add('processing');
                micButton.title = 'Zpracovávám...';
                break;
            case 'error':
                micButton.classList.add('error');
                micButton.title = 'Chyba - zkuste to znovu';
                setTimeout(() => {
                    this.updateMicButton('idle');
                }, 2000);
                break;
            default: // idle
                micButton.title = 'Klikněte pro diktování';
        }
    }
}

// Utility functions
function log(message) {
    if (typeof console !== 'undefined') {
        console.log(`[Voice] ${message}`);
    }
}

function showNotification(message, type = 'info') {
    // Simple notification implementation
    const notification = document.createElement('div');
    notification.className = `notification notification-${type}`;
    notification.textContent = message;
    notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        padding: 12px 20px;
        background: ${type === 'error' ? '#e74c3c' : type === 'warning' ? '#f39c12' : type === 'success' ? '#27ae60' : '#3498db'};
        color: white;
        border-radius: 6px;
        z-index: 10000;
        box-shadow: 0 4px 8px rgba(0,0,0,0.3);
        animation: slideIn 0.3s ease-out;
    `;

    document.body.appendChild(notification);

    setTimeout(() => {
        notification.style.animation = 'slideOut 0.3s ease-in';
        setTimeout(() => {
            document.body.removeChild(notification);
        }, 300);
    }, 3000);
}

// Export instance
window.voiceManager = new VoiceManager();
