// Claude Desktop - Frontend Logic
// Komunikace s Tauri backendem

(function() {
    const { invoke } = window.__TAURI__.core;
    const { listen } = window.__TAURI__.event;

    // DOM Elements
    let settingsModal;
    let mcpConfig;
    let serverList;
    let authBtn;
    let chatTab;
    let codeTab;
    let chatView;
    let codeView;
    let chatFrame;
    let codeFrame;
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

        chatView = document.getElementById('chatView');
        codeView = document.getElementById('codeView');
        chatFrame = document.getElementById('chatFrame');
        codeFrame = document.getElementById('codeFrame');

        console.log('DOM Elements loaded:', {
            chatTab: !!chatTab,
            codeTab: !!codeTab,
            chatView: !!chatView,
            codeView: !!codeView,
            chatFrame: !!chatFrame,
            codeFrame: !!codeFrame
        });

        // Setup event listeners
        setupEventListeners();

        // Listen for view change events from Rust (např. klávesové zkratky)
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
        if (chatTab) chatTab.addEventListener('click', () => requestSwitchView('chat'));
        if (codeTab) codeTab.addEventListener('click', () => requestSwitchView('code'));

        // Settings button
        const settingsBtn = document.getElementById('settingsBtn');
        if (settingsBtn) settingsBtn.addEventListener('click', openSettings);

        // Close modal
        const closeBtn = document.querySelector('.close-btn');
        if (closeBtn) closeBtn.addEventListener('click', closeSettings);

        // Close modal on outside click
        if (settingsModal) {
            settingsModal.addEventListener('click', (e) => {
                if (e.target === settingsModal) {
                    closeSettings();
                }
            });
        }

        // Auth button
        if (authBtn) authBtn.addEventListener('click', handleLogin);

        // Save config button
        const saveConfigBtn = document.getElementById('saveConfigBtn');
        if (saveConfigBtn) saveConfigBtn.addEventListener('click', saveConfig);

        // Open config dir
        const openConfigDirBtn = document.getElementById('openConfigDirBtn');
        if (openConfigDirBtn) openConfigDirBtn.addEventListener('click', openConfigDir);

        // Add server button
        const addServerBtn = document.getElementById('addServerBtn');
        if (addServerBtn) {
            addServerBtn.addEventListener('click', () => {
                openSettings();
            });
        }

        // Voice controls setup (if present)
        setupVoiceControls();
    }

    function setupVoiceControls() {
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

    // Request View Switch (calls Backend)
    async function requestSwitchView(view) {
        if (currentView === view) return;

        try {
            // Call Rust backend to emit event (to keep logic consistent with shortcuts)
            // Alternatively, we could just call applyViewSwitch(view) directly if no Rust side effect is needed
            await invoke('switch_view', { view });
            // The actual switch happens in listenForViewChanges when the event comes back
            // But to make it snappy, we can do optimistic update
            applyViewSwitch(view);
        } catch (error) {
            console.error('Failed to switch view:', error);
        }
    }

    // Apply View Switch (UI changes)
    function applyViewSwitch(view) {
        currentView = view;

        // Update Tabs
        if (view === 'chat') {
            if (chatTab) chatTab.classList.add('active');
            if (codeTab) codeTab.classList.remove('active');

            if (chatView) {
                chatView.classList.remove('hidden');
                chatView.classList.add('active');
            }
            if (chatFrame) {
                chatFrame.classList.remove('hidden');
                chatFrame.classList.add('active');
            }

            if (codeView) {
                codeView.classList.add('hidden');
                codeView.classList.remove('active');
            }
            if (codeFrame) {
                codeFrame.classList.add('hidden');
                codeFrame.classList.remove('active');
            }
        } else {
            if (chatTab) chatTab.classList.remove('active');
            if (codeTab) codeTab.classList.add('active');

            if (chatView) {
                chatView.classList.add('hidden');
                chatView.classList.remove('active');
            }
            if (chatFrame) {
                chatFrame.classList.add('hidden');
                chatFrame.classList.remove('active');
            }

            if (codeView) {
                codeView.classList.remove('hidden');
                codeView.classList.add('active');
            }
            if (codeFrame) {
                codeFrame.classList.remove('hidden');
                codeFrame.classList.add('active');
            }
        }

        console.log(`🔄 Switched to ${view}`);
    }

    // Listen for view change events from Rust
    async function listenForViewChanges() {
        await listen('switch-tab', (event) => {
            const view = event.payload;
            console.log(`📡 Received switch-tab event: ${view}`);
            applyViewSwitch(view);
        });
    }

    // Authentication
    async function checkAuth() {
        try {
            const isAuth = await invoke('check_auth');
            if (isAuth && authBtn) {
                authBtn.textContent = '✓ Přihlášen';
                authBtn.classList.add('btn-success');
            } else if (authBtn) {
                authBtn.textContent = 'Přihlásit se';
                authBtn.classList.remove('btn-success');
            }
        } catch (error) {
            console.error('Auth check failed:', error);
        }
    }

    async function handleLogin() {
        try {
            if (authBtn) {
                authBtn.textContent = 'Přihlašuji...';
                authBtn.disabled = true;
            }

            const result = await invoke('login');
            console.log('Login result:', result);

            await checkAuth();
        } catch (error) {
            console.error('Login failed:', error);
            alert('Přihlášení selhalo: ' + error);
        } finally {
            if (authBtn) authBtn.disabled = false;
        }
    }

    // MCP Servers
    async function loadMcpServers() {
        try {
            const servers = await invoke('get_mcp_servers');
            displayServers(servers);
        } catch (error) {
            console.error('Failed to load MCP servers:', error);
            if (serverList) serverList.innerHTML = '<p class="loading">Chyba načítání serverů</p>';
        }
    }

    function displayServers(servers) {
        if (!serverList) return;

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
        if (settingsModal) settingsModal.classList.remove('hidden');

        // Load MCP config
        try {
            const config = await invoke('load_mcp_config');
            if (mcpConfig) mcpConfig.value = config;
        } catch (error) {
            console.error('Failed to load config:', error);
            if (mcpConfig) mcpConfig.value = '// Chyba načítání konfigurace';
        }

        // Load voice settings
        await loadVoiceSettings();
    }

    function closeSettings() {
        if (settingsModal) settingsModal.classList.add('hidden');
    }

    async function saveConfig() {
        try {
            const config = mcpConfig ? mcpConfig.value : '';
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
            const verEl = document.getElementById('appVersion');
            if (verEl) verEl.textContent = version;

            const systemInfo = await invoke('get_system_info');
            const sysEl = document.getElementById('systemInfo');
            if (sysEl) sysEl.textContent = systemInfo;
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
            const langEl = document.getElementById('inputLanguage');
            if (langEl) langEl.value = settings.input_language;

            const speedEl = document.getElementById('outputSpeed');
            if (speedEl) speedEl.value = settings.output_speed;

            const speedValEl = document.getElementById('speedValue');
            if (speedValEl) speedValEl.textContent = settings.output_speed + 'x';

            const autoPlayEl = document.getElementById('autoPlay');
            if (autoPlayEl) autoPlayEl.checked = settings.auto_play;

            const historyEl = document.getElementById('historyLimit');
            if (historyEl) historyEl.value = settings.history_limit;
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
        if (e.key === 'Escape' && settingsModal && !settingsModal.classList.contains('hidden')) {
            closeSettings();
        }
    });

    console.log('🦀 Tauri frontend loaded');
    console.log('💾 Memory: ~30-50 MB (vs Electron ~200-400 MB)');
})();
