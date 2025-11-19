// Claude Desktop - Frontend Logic
// Komunikace s Tauri backendem

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// DOM Elements
let settingsModal;
let mcpConfig;
let serverList;
let authBtn;
let chatTab;
let codeTab;
let claudeIframe;
let currentView = 'chat';

// Initialize app
document.addEventListener('DOMContentLoaded', async () => {
    console.log('🚀 Claude Desktop (Tauri) initialized');

    // Get DOM elements
    settingsModal = document.getElementById('settingsModal');
    mcpConfig = document.getElementById('mcpConfig');
    serverList = document.getElementById('mcpServerList');
    authBtn = document.getElementById('authBtn');
    chatTab = document.getElementById('chatTab');
    codeTab = document.getElementById('codeTab');
    claudeIframe = document.querySelector('#claudeWebview iframe');

    // Setup event listeners
    setupEventListeners();

    // Listen for view change events from Rust
    await listenForViewChanges();

    // Check authentication
    await checkAuth();

    // Load MCP servers
    await loadMcpServers();

    // Load app info
    await loadAppInfo();
});

// Event Listeners
function setupEventListeners() {
    // Tab switching
    chatTab.addEventListener('click', () => switchView('chat'));
    codeTab.addEventListener('click', () => switchView('code'));

    // Settings button
    document.getElementById('settingsBtn').addEventListener('click', openSettings);

    // Close modal
    document.querySelector('.close-btn').addEventListener('click', closeSettings);

    // Close modal on outside click
    settingsModal.addEventListener('click', (e) => {
        if (e.target === settingsModal) {
            closeSettings();
        }
    });

    // Auth button
    authBtn.addEventListener('click', handleLogin);

    // Save config button
    document.getElementById('saveConfigBtn').addEventListener('click', saveConfig);

    // Open config dir
    document.getElementById('openConfigDirBtn').addEventListener('click', openConfigDir);

    // Add server button
    document.getElementById('addServerBtn').addEventListener('click', () => {
        openSettings();
    });

    // Voice controls
    const micButton = document.getElementById('micButton');
    if (micButton) {
        micButton.addEventListener('click', () => {
            if (window.voiceManager) {
                window.voiceManager.startListening();
            }
        });
    }

    const saveVoiceSettingsBtn = document.getElementById('saveVoiceSettingsBtn');
    if (saveVoiceSettingsBtn) {
        saveVoiceSettingsBtn.addEventListener('click', saveVoiceSettings);
    }

    const clearHistoryBtn = document.getElementById('clearHistoryBtn');
    if (clearHistoryBtn) {
        clearHistoryBtn.addEventListener('click', clearVoiceHistory);
    }

    const outputSpeed = document.getElementById('outputSpeed');
    const speedValue = document.getElementById('speedValue');
    if (outputSpeed && speedValue) {
        outputSpeed.addEventListener('input', (e) => {
            speedValue.textContent = e.target.value + 'x';
        });
    }
}

// View Switching
async function switchView(view) {
    if (currentView === view) return;

    try {
        // Call Rust backend to emit event
        await invoke('switch_view', { view });

        // Update UI
        currentView = view;
        updateTabStates();

        console.log(`🔄 Switched to ${view}`);
    } catch (error) {
        console.error('Failed to switch view:', error);
    }
}

function updateTabStates() {
    if (currentView === 'chat') {
        chatTab.classList.add('active');
        codeTab.classList.remove('active');
    } else {
        chatTab.classList.remove('active');
        codeTab.classList.add('active');
    }
}

// Listen for view change events from Rust
async function listenForViewChanges() {
    await listen('change-view', (event) => {
        const url = event.payload;
        console.log(`📡 Received view change event: ${url}`);

        // Change iframe URL
        if (claudeIframe) {
            claudeIframe.src = url;
        }
    });
}

// Authentication
async function checkAuth() {
    try {
        const isAuth = await invoke('check_auth');
        if (isAuth) {
            authBtn.textContent = '✓ Přihlášen';
            authBtn.classList.add('btn-success');
        } else {
            authBtn.textContent = 'Přihlásit se';
            authBtn.classList.remove('btn-success');
        }
    } catch (error) {
        console.error('Auth check failed:', error);
    }
}

async function handleLogin() {
    try {
        authBtn.textContent = 'Přihlašuji...';
        authBtn.disabled = true;

        const result = await invoke('login');
        console.log('Login result:', result);

        await checkAuth();
    } catch (error) {
        console.error('Login failed:', error);
        alert('Přihlášení selhalo: ' + error);
    } finally {
        authBtn.disabled = false;
    }
}

// MCP Servers
async function loadMcpServers() {
    try {
        const servers = await invoke('get_mcp_servers');
        displayServers(servers);
    } catch (error) {
        console.error('Failed to load MCP servers:', error);
        serverList.innerHTML = '<p class="loading">Chyba načítání serverů</p>';
    }
}

function displayServers(servers) {
    if (servers.length === 0) {
        serverList.innerHTML = '<p class="loading">Žádné servery</p>';
        return;
    }

    serverList.innerHTML = servers.map(server => `
        <div class="server-item">
            <span class="server-name">${server}</span>
            <div class="server-status"></div>
        </div>
    `).join('');
}

// Settings
async function openSettings() {
    settingsModal.classList.remove('hidden');

    // Load MCP config
    try {
        const config = await invoke('load_mcp_config');
        mcpConfig.value = config;
    } catch (error) {
        console.error('Failed to load config:', error);
        mcpConfig.value = '// Chyba načítání konfigurace';
    }
}

function closeSettings() {
    settingsModal.classList.add('hidden');
}

async function saveConfig() {
    try {
        const config = mcpConfig.value;
        await invoke('save_mcp_config', { config });
        alert('Konfigurace uložena! Restartujte aplikaci pro aplikování změn.');
    } catch (error) {
        console.error('Failed to save config:', error);
        alert('Chyba ukládání: ' + error);
    }
}

async function openConfigDir() {
    try {
        await invoke('open_config_dir');
    } catch (error) {
        console.error('Failed to open config dir:', error);
        alert('Chyba otevírání složky: ' + error);
    }
}

// App Info
async function loadAppInfo() {
    try {
        const version = await invoke('get_app_version');
        document.getElementById('appVersion').textContent = version;

        const systemInfo = await invoke('get_system_info');
        document.getElementById('systemInfo').textContent = systemInfo;
    } catch (error) {
        console.error('Failed to load app info:', error);
    }
}

// Voice Settings
async function loadVoiceSettings() {
    if (!window.voiceManager) return;

    // Load voices into dropdown
    const voiceSelect = document.getElementById('outputVoice');
    if (voiceSelect) {
        const voices = window.voiceManager.getVoices();

        // Wait for voices to load (sometimes delayed)
        if (voices.length === 0) {
            window.speechSynthesis.addEventListener('voiceschanged', () => {
                const updatedVoices = window.voiceManager.getVoices();
                populateVoiceOptions(voiceSelect, updatedVoices);
            }, { once: true });
        } else {
            populateVoiceOptions(voiceSelect, voices);
        }
    }

    // Load settings from Rust backend
    const settings = window.voiceManager.settings;
    if (settings) {
        document.getElementById('inputLanguage').value = settings.input_language;
        document.getElementById('outputSpeed').value = settings.output_speed;
        document.getElementById('speedValue').textContent = settings.output_speed + 'x';
        document.getElementById('autoPlay').checked = settings.auto_play;
        document.getElementById('historyLimit').value = settings.history_limit;
    }
}

function populateVoiceOptions(selectElement, voices) {
    // Clear existing options except default
    selectElement.innerHTML = '<option value="default">Výchozí systémový hlas</option>';

    // Add voice options
    voices.forEach(voice => {
        const option = document.createElement('option');
        option.value = voice.name;
        option.textContent = `${voice.name} (${voice.lang})`;
        selectElement.appendChild(option);
    });

    // Select current voice if set
    if (window.voiceManager.settings && window.voiceManager.settings.output_voice) {
        selectElement.value = window.voiceManager.settings.output_voice;
    }
}

async function saveVoiceSettings() {
    if (!window.voiceManager) return;

    const settings = {
        input_language: document.getElementById('inputLanguage').value,
        output_voice: document.getElementById('outputVoice').value,
        output_speed: parseFloat(document.getElementById('outputSpeed').value),
        auto_play: document.getElementById('autoPlay').checked,
        history_limit: parseInt(document.getElementById('historyLimit').value)
    };

    await window.voiceManager.updateSettings(settings);
}

async function clearVoiceHistory() {
    if (!window.voiceManager) return;

    if (confirm('Opravdu chcete smazat celou historii konverzací?')) {
        await window.voiceManager.clearConversations();
    }
}

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
    // Ctrl+, to open settings
    if (e.ctrlKey && e.key === ',') {
        e.preventDefault();
        openSettings();
    }

    // Ctrl+M to toggle microphone
    if (e.ctrlKey && e.key === 'm') {
        e.preventDefault();
        if (window.voiceManager) {
            window.voiceManager.startListening();
        }
    }

    // Escape to close modal
    if (e.key === 'Escape' && !settingsModal.classList.contains('hidden')) {
        closeSettings();
    }
});

// Load voice settings when opening settings modal
const originalOpenSettings = openSettings;
openSettings = async function() {
    await originalOpenSettings();
    await loadVoiceSettings();
};

console.log('🦀 Tauri frontend loaded');
console.log('💾 Memory: ~30-50 MB (vs Electron ~200-400 MB)');
console.log('⚡ Startup: <1s (vs Electron 3-5s)');
console.log('🎤 Voice features enabled');
