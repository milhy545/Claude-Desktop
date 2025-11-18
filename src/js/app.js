// Claude Desktop - Frontend Logic
// Komunikace s Tauri backendem

const { invoke } = window.__TAURI__.core;

// DOM Elements
let settingsModal;
let mcpConfig;
let serverList;
let authBtn;

// Initialize app
document.addEventListener('DOMContentLoaded', async () => {
    console.log('🚀 Claude Desktop (Tauri) initialized');

    // Get DOM elements
    settingsModal = document.getElementById('settingsModal');
    mcpConfig = document.getElementById('mcpConfig');
    serverList = document.getElementById('mcpServerList');
    authBtn = document.getElementById('authBtn');

    // Setup event listeners
    setupEventListeners();

    // Check authentication
    await checkAuth();

    // Load MCP servers
    await loadMcpServers();

    // Load app info
    await loadAppInfo();
});

// Event Listeners
function setupEventListeners() {
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

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
    // Ctrl+, to open settings
    if (e.ctrlKey && e.key === ',') {
        e.preventDefault();
        openSettings();
    }

    // Escape to close modal
    if (e.key === 'Escape' && !settingsModal.classList.contains('hidden')) {
        closeSettings();
    }
});

console.log('🦀 Tauri frontend loaded');
console.log('💾 Memory: ~30-50 MB (vs Electron ~200-400 MB)');
console.log('⚡ Startup: <1s (vs Electron 3-5s)');
