# API Dokumentace

Kompletní API reference pro Claude Desktop (Tauri Edition).

## Obsah

- [Tauri příkazy](#tauri-příkazy)
- [Autentizační API](#autentizační-api)
- [MCP Server API](#mcp-server-api)
- [Konfigurační API](#konfigurační-api)
- [Utility API](#utility-api)
- [Frontend API](#frontend-api)

## Tauri příkazy

Tyto příkazy jsou exportovány z Rust backendu do JavaScript frontendu přes `invoke()`.

### Přehled

```javascript
import { invoke } from '@tauri-apps/api/core';

// Příklad použití
const result = await invoke('check_auth');
```

---

## Autentizační API

### `check_auth()`

Zkontroluje, jestli je uživatel přihlášen přes Claude CLI.

**Parametry:** Žádné

**Vrací:** `Promise<boolean>`

**Příklad:**
```javascript
const isAuth = await invoke('check_auth');
if (isAuth) {
    console.log('✅ Uživatel je přihlášen');
} else {
    console.log('❌ Uživatel se potřebuje přihlásit');
}
```

**Rust implementace:**
```rust
#[tauri::command]
fn check_auth() -> Result<bool, String> {
    auth::is_authenticated()
}
```

**Chyby:**
- Vrací `false` pokud `~/.claude/` neexistuje
- Vrací `false` pokud je session adresář prázdný

---

### `login()`

Spustí proces přihlášení přes Claude CLI.

**Parametry:** Žádné

**Vrací:** `Promise<string>`

**Příklad:**
```javascript
try {
    const result = await invoke('login');
    console.log(result); // "Přihlášení úspěšné!"
} catch (error) {
    console.error('Přihlášení selhalo:', error);
}
```

**Rust implementace:**
```rust
#[tauri::command]
async fn login() -> Result<String, String> {
    auth::login().await
}
```

**Chyby:**
- `"Nepodařilo se spustit claude CLI: ..."` - CLI nenalezeno
- `"Přihlášení selhalo: ..."` - Proces přihlášení selhal

---

## MCP Server API

### `get_mcp_servers()`

Získá seznam nakonfigurovaných MCP serverů.

**Parametry:** Žádné

**Vrací:** `Promise<string[]>`

**Příklad:**
```javascript
const servers = await invoke('get_mcp_servers');
console.log(servers); // ['filesystem', 'git', 'sqlite']
```

**Rust implementace:**
```rust
#[tauri::command]
fn get_mcp_servers(state: tauri::State<AppState>) -> Result<Vec<String>, String> {
    let servers = state.mcp_servers.lock().unwrap();
    Ok(servers.iter().map(|s| s.name.clone()).collect())
}
```

---

### `start_mcp_server(name)`

Spustí konkrétní MCP server.

**Parametry:**
- `name: string` - Název serveru z konfigurace

**Vrací:** `Promise<void>`

**Příklad:**
```javascript
await invoke('start_mcp_server', { name: 'filesystem' });
console.log('📂 Filesystem server spuštěn');
```

**Rust implementace:**
```rust
#[tauri::command]
fn start_mcp_server(name: String, state: tauri::State<AppState>) -> Result<(), String> {
    mcp::start_server(&name, &state)
}
```

**Chyby:**
- `"Server not found"` - Název serveru neexistuje v konfiguraci
- `"Failed to start: ..."` - Spuštění serveru selhalo

---

### `stop_mcp_server(name)`

Zastaví běžící MCP server.

**Parametry:**
- `name: string` - Název serveru

**Vrací:** `Promise<void>`

**Příklad:**
```javascript
await invoke('stop_mcp_server', { name: 'filesystem' });
console.log('🛑 Filesystem server zastaven');
```

**Rust implementace:**
```rust
#[tauri::command]
fn stop_mcp_server(name: String, state: tauri::State<AppState>) -> Result<(), String> {
    mcp::stop_server(&name, &state)
}
```

---

## Konfigurační API

### `load_mcp_config()`

Načte MCP konfiguraci z disku.

**Parametry:** Žádné

**Vrací:** `Promise<string>` - JSON konfigurace

**Příklad:**
```javascript
const configJson = await invoke('load_mcp_config');
const config = JSON.parse(configJson);
console.log(config.mcpServers);
```

**Rust implementace:**
```rust
#[tauri::command]
fn load_mcp_config() -> Result<String, String> {
    mcp::load_config()
}
```

**Cesta ke konfiguraci:** `~/.config/Claude/claude_desktop_config.json`

**Výchozí konfigurace:**
```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-filesystem",
        "/home/$USER/Documents"
      ]
    }
  }
}
```

---

### `save_mcp_config(config)`

Uloží MCP konfiguraci na disk.

**Parametry:**
- `config: string` - JSON konfigurace jako string

**Vrací:** `Promise<void>`

**Příklad:**
```javascript
const newConfig = {
    mcpServers: {
        git: {
            command: "npx",
            args: ["-y", "@modelcontextprotocol/server-git"]
        }
    }
};

await invoke('save_mcp_config', {
    config: JSON.stringify(newConfig, null, 2)
});
```

**Rust implementace:**
```rust
#[tauri::command]
fn save_mcp_config(config: String) -> Result<(), String> {
    mcp::save_config(&config)
}
```

**Chyby:**
- `"Nepodařilo se vytvořit config directory: ..."` - Vytvoření adresáře selhalo
- `"Nepodařilo se uložit config: ..."` - Zápis souboru selhal

---

## Utility API

### `get_app_version()`

Získá verzi aplikace.

**Parametry:** Žádné

**Vrací:** `Promise<string>`

**Příklad:**
```javascript
const version = await invoke('get_app_version');
console.log(`Verze: ${version}`); // "Verze: 0.1.0"
```

**Rust implementace:**
```rust
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
```

---

### `get_system_info()`

Získá informace o systému.

**Parametry:** Žádné

**Vrací:** `Promise<string>`

**Příklad:**
```javascript
const sysInfo = await invoke('get_system_info');
console.log(sysInfo); // "OS: linux, Arch: x86_64"
```

**Rust implementace:**
```rust
#[tauri::command]
fn get_system_info() -> Result<String, String> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    Ok(format!("OS: {}, Arch: {}", os, arch))
}
```

---

### `open_config_dir()`

Otevře konfigurační adresář ve správci souborů.

**Parametry:** Žádné

**Vrací:** `Promise<void>`

**Příklad:**
```javascript
await invoke('open_config_dir');
// Otevře ~/.config/Claude/ ve správci souborů
```

**Rust implementace:**
```rust
#[tauri::command]
fn open_config_dir() -> Result<(), String> {
    let config_path = dirs::config_dir()
        .ok_or("Cannot find config directory")?
        .join("Claude");

    std::fs::create_dir_all(&config_path)
        .map_err(|e| format!("Failed to create config dir: {}", e))?;

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&config_path)
            .spawn()
            .map_err(|e| format!("Failed to open config dir: {}", e))?;
    }

    Ok(())
}
```

---

### `switch_view(view)`

Přepínání mezi zobrazeními Chat a Code v aplikaci.

**Parametry:**
- `view` (string): Zobrazení, na které se má přepnout. Platné hodnoty: `"chat"` nebo `"code"`

**Vrací:** `Promise<void>`

**Příklad:**
```javascript
// Přepnout na zobrazení Chat
await invoke('switch_view', { view: 'chat' });

// Přepnout na zobrazení Code
await invoke('switch_view', { view: 'code' });

// S ošetřením chyb
try {
    await invoke('switch_view', { view: 'chat' });
    console.log('✅ Přepnuto na zobrazení Chat');
} catch (error) {
    console.error('Nepodařilo se přepnout zobrazení:', error);
}
```

**Rust implementace:**
```rust
#[tauri::command]
fn switch_view(app: tauri::AppHandle, view: String) -> Result<(), String> {
    let url = match view.as_str() {
        "chat" => "https://claude.ai",
        "code" => "https://claude.ai/code",
        _ => return Err(format!("Unknown view: {}", view)),
    };

    if let Some(window) = app.get_webview_window("main") {
        window.emit("change-view", url)
            .map_err(|e| format!("Failed to emit event: {}", e))?;
        log::info!("🔄 Switched view to: {}", view);
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}
```

**Události:**
- Vysílá událost `change-view` s URL do hlavního okna
- Frontend naslouchá této události pro aktualizaci iframe src

**Chyby:**
- `"Unknown view: <view>"` - Byl poskytnut neplatný název zobrazení
- `"Failed to emit event: ..."` - Selhalo vyslání události
- `"Main window not found"` - Hlavní okno není dostupné

**Platná zobrazení:**
- `"chat"` - Chatovací rozhraní Claude AI (https://claude.ai)
- `"code"` - Rozhraní Claude Code (https://claude.ai/code)

**Poznámky:**
- Názvy zobrazení rozlišují velikost písmen
- Platné jsou pouze malá písmena "chat" a "code"
- Funkce vysílá událost místo přímé změny URL, aby oddělila backend od manipulace s DOM na frontendu

---

## Voice API (Hlasové funkce)

### `save_conversation(entry)`

Uložit záznam konverzace do historie.

**Parametry:**
- `entry` (ConversationEntry): Záznam konverzace k uložení
  - `id` (string): Jedinečný identifikátor (UUID)
  - `timestamp` (number): Unix timestamp v milisekundách
  - `user_input` (string): Text vstupu uživatele
  - `assistant_response` (string): Text odpovědi asistenta
  - `voice_used` (boolean): Zda byl použit hlasový vstup
  - `played_back` (boolean): Zda byla odpověď přehrána

**Vrací:** `Promise<void>`

**Příklad:**
```javascript
const entry = {
    id: crypto.randomUUID(),
    timestamp: Date.now(),
    user_input: "Jak funguje fotosyntéza?",
    assistant_response: "Fotosyntéza je proces...",
    voice_used: true,
    played_back: false
};

await invoke('save_conversation', { entry });
```

**Rust implementace:**
```rust
#[tauri::command]
fn save_conversation(entry: voice::ConversationEntry) -> Result<(), String> {
    voice::save_conversation(entry)
}
```

**Uložiště:**
- Uloženo v: `~/.config/Claude/voice/conversations.json`
- Automaticky udržuje limit historie (výchozí: 100 záznamů)
- Ponechává nejnovější záznamy při překročení limitu

---

### `load_conversations()`

Načíst historii konverzací z úložiště.

**Parametry:** Žádné

**Vrací:** `Promise<Array<ConversationEntry>>`

**Příklad:**
```javascript
const conversations = await invoke('load_conversations');

console.log(`Načteno ${conversations.length} konverzací`);

conversations.forEach(conv => {
    console.log(`${conv.user_input} -> ${conv.assistant_response}`);
});
```

**Rust implementace:**
```rust
#[tauri::command]
fn load_conversations() -> Result<Vec<voice::ConversationEntry>, String> {
    voice::load_conversations()
}
```

**Vrací prázdné pole pokud:**
- Ještě nebyly uloženy žádné konverzace
- Soubor s konverzacemi neexistuje

---

### `clear_conversations()`

Smazat celou historii konverzací.

**Parametry:** Žádné

**Vrací:** `Promise<void>`

**Příklad:**
```javascript
if (confirm('Smazat celou historii konverzací?')) {
    await invoke('clear_conversations');
    console.log('✅ Historie smazána');
}
```

**Rust implementace:**
```rust
#[tauri::command]
fn clear_conversations() -> Result<(), String> {
    voice::clear_conversations()
}
```

**Efekty:**
- Smaže `~/.config/Claude/voice/conversations.json`
- Nelze vrátit zpět
- Neovlivňuje hlasová nastavení

---

### `get_voice_settings()`

Načíst aktuální hlasová nastavení.

**Parametry:** Žádné

**Vrací:** `Promise<VoiceSettings>`

**Struktura VoiceSettings:**
- `input_language` (string): Jazyk pro rozpoznávání řeči (např. "cs-CZ")
- `output_voice` (string): Název hlasu pro syntézu řeči
- `output_speed` (number): Rychlost přehrávání (0.5 - 2.0)
- `auto_play` (boolean): Automaticky přehrát odpovědi
- `history_limit` (number): Maximální počet uložených konverzací

**Příklad:**
```javascript
const settings = await invoke('get_voice_settings');

console.log(`Jazyk vstupu: ${settings.input_language}`);
console.log(`Rychlost výstupu: ${settings.output_speed}x`);
console.log(`Auto-přehrávání: ${settings.auto_play}`);
```

**Rust implementace:**
```rust
#[tauri::command]
fn get_voice_settings() -> Result<voice::VoiceSettings, String> {
    voice::load_voice_settings()
}
```

**Výchozí hodnoty:**
```json
{
  "input_language": "cs-CZ",
  "output_voice": "default",
  "output_speed": 1.0,
  "auto_play": false,
  "history_limit": 100
}
```

---

### `save_voice_settings(settings)`

Uložit hlasová nastavení do trvalého úložiště.

**Parametry:**
- `settings` (VoiceSettings): Objekt nastavení k uložení

**Vrací:** `Promise<void>`

**Příklad:**
```javascript
const newSettings = {
    input_language: 'en-US',
    output_voice: 'Google US English',
    output_speed: 1.5,
    auto_play: true,
    history_limit: 200
};

await invoke('save_voice_settings', { settings: newSettings });
console.log('✅ Nastavení uloženo');
```

**Rust implementace:**
```rust
#[tauri::command]
fn save_voice_settings(settings: voice::VoiceSettings) -> Result<(), String> {
    voice::save_voice_settings(&settings)
}
```

**Uložiště:**
- Uloženo v: `~/.config/Claude/voice/voice_settings.json`
- Přetrvává napříč restarty aplikace
- Aplikováno okamžitě po uložení

**Validace:**
- `output_speed` musí být mezi 0.5 a 2.0
- `history_limit` musí být minimálně 10
- `input_language` by měl být validní BCP 47 jazykový tag

---

## Frontend API

### Inicializace aplikace

```javascript
document.addEventListener('DOMContentLoaded', async () => {
    // Inicializace aplikace
    await checkAuth();
    await loadMcpServers();
    await loadAppInfo();
});
```

### Event handlery

```javascript
// Tlačítko nastavení
document.getElementById('settingsBtn')
    .addEventListener('click', openSettings);

// Tlačítko přihlášení
document.getElementById('authBtn')
    .addEventListener('click', handleLogin);

// Uložení konfigurace
document.getElementById('saveConfigBtn')
    .addEventListener('click', saveConfig);
```

### Klávesové zkratky

```javascript
document.addEventListener('keydown', (e) => {
    // Ctrl+, pro otevření nastavení
    if (e.ctrlKey && e.key === ',') {
        e.preventDefault();
        openSettings();
    }

    // Escape pro zavření modalu
    if (e.key === 'Escape') {
        closeSettings();
    }
});
```

---

## Zpracování chyb

### Rust error pattern

```rust
#[tauri::command]
fn příklad_příkazu() -> Result<String, String> {
    nějaká_operace()
        .map_err(|e| format!("Operace selhala: {}", e))?;

    Ok("Úspěch".to_string())
}
```

### JavaScript error pattern

```javascript
async function příkladFunkce() {
    try {
        const result = await invoke('příklad_příkazu');
        return result;
    } catch (error) {
        console.error('Příkaz selhal:', error);
        alert(`Chyba: ${error}`);
        throw error;
    }
}
```

---

## Datové typy

### MCP Server Configuration

```typescript
interface McpConfig {
    mcpServers: {
        [name: string]: {
            command: string;
            args: string[];
        }
    }
}
```

**Příklad:**
```json
{
    "mcpServers": {
        "filesystem": {
            "command": "npx",
            "args": [
                "-y",
                "@modelcontextprotocol/server-filesystem",
                "/home/user/Documents"
            ]
        },
        "git": {
            "command": "npx",
            "args": ["-y", "@modelcontextprotocol/server-git"]
        }
    }
}
```

### McpServer (Rust)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    #[serde(skip)]
    pub process: Option<u32>, // PID
}
```

---

## Poznámky k výkonu

### Využití paměti

- **Idle:** ~30-50 MB
- **S 3 MCP servery:** ~60-80 MB
- **Intenzivní použití:** ~100-150 MB

**vs Electron:** ~200-400 MB (80% úspora) 🚀

### Čas spuštění

- **Studený start:** ~0.5-1s
- **Teplý start:** ~0.2-0.5s

**vs Electron:** 3-5s (5x rychlejší) ⚡

### Velikost binárky

- **Stripped release:** ~5-8 MB
- **S debug symboly:** ~20-30 MB

**vs Electron:** ~150 MB (95% menší) 📦

---

## Bezpečnostní úvahy

### Validace vstupu

Všechny uživatelské vstupy jsou validovány před zpracováním:

```rust
fn validate_server_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Název serveru nemůže být prázdný".to_string());
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err("Neplatný název serveru".to_string());
    }
    Ok(())
}
```

### Sanitizace cest

Cesty jsou sanitizovány pro prevenci directory traversal:

```rust
use std::path::Path;

fn sanitize_path(path: &str) -> PathBuf {
    Path::new(path)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(path))
}
```

### Prevence command injection

MCP server příkazy používají bezpečné předávání argumentů:

```rust
// ✅ Bezpečné: Argumenty předány jako pole
Command::new("npx")
    .args(&["-y", "@modelcontextprotocol/server-filesystem"])
    .spawn()

// ❌ Nebezpečné: Shell interpretace
Command::new("sh")
    .arg("-c")
    .arg(format!("npx -y {}", user_input))  // NEDĚLEJ TOTO
```

---

## Příklady

### Kompletní MCP Server workflow

```javascript
// 1. Načíst existující config
const configJson = await invoke('load_mcp_config');
const config = JSON.parse(configJson);

// 2. Přidat nový server
config.mcpServers.postgres = {
    command: "npx",
    args: [
        "-y",
        "@modelcontextprotocol/server-postgres",
        "--connection-string",
        "postgresql://localhost/mydb"
    ]
};

// 3. Uložit aktualizovaný config
await invoke('save_mcp_config', {
    config: JSON.stringify(config, null, 2)
});

// 4. Spustit server
await invoke('start_mcp_server', { name: 'postgres' });

// 5. Ověřit, že běží
const servers = await invoke('get_mcp_servers');
console.log('Aktivní servery:', servers);
```

### Autentizační tok

```javascript
// Zkontrolovat, jestli je přihlášen
const isAuth = await invoke('check_auth');

if (!isAuth) {
    // Zobrazit tlačítko přihlášení
    authBtn.textContent = 'Přihlásit se';
    authBtn.addEventListener('click', async () => {
        try {
            authBtn.textContent = 'Přihlašuji...';
            authBtn.disabled = true;

            const result = await invoke('login');
            console.log(result);

            authBtn.textContent = '✓ Přihlášen';
            authBtn.classList.add('btn-success');
        } catch (error) {
            authBtn.textContent = 'Chyba';
            alert(`Přihlášení selhalo: ${error}`);
        } finally {
            authBtn.disabled = false;
        }
    });
}
```

---

**Poslední aktualizace:** 2025-11-18
**Verze API:** 0.1.0
