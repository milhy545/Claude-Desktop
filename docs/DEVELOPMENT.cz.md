# Průvodce vývojem

Kompletní průvodce vývojem Claude Desktop (Tauri Edition).

## Obsah

- [Nastavení vývojového prostředí](#nastavení-vývojového-prostředí)
- [Struktura projektu](#struktura-projektu)
- [Vývojový workflow](#vývojový-workflow)
- [Debugování](#debugování)
- [Build proces](#build-proces)
- [Přispívání](#přispívání)

## Nastavení vývojového prostředí

### Systémové požadavky

- **OS:** Linux (Ubuntu 20.04+, Debian, Fedora, Arch)
- **RAM:** 4GB minimum, 8GB doporučeno
- **Disk:** 2GB volného místa pro závislosti

### Instalace závislostí

#### 1. Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Ověření:
```bash
rustc --version  # Mělo by být 1.70+
cargo --version
```

#### 2. Node.js

```bash
# Přes nvm (doporučeno)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20

# Nebo přes apt
sudo apt install nodejs npm
```

Ověření:
```bash
node --version  # Mělo by být 18+
npm --version
```

#### 3. Tauri závislosti (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf
```

#### 4. Tauri CLI

```bash
npm install -g @tauri-apps/cli
# Nebo použij project-local verzi (doporučeno)
npm install
```

### Klonování a nastavení

```bash
git clone https://github.com/milhy545/Claude-Desktop.git
cd Claude-Desktop

# Instalace frontend závislostí
npm install

# Stažení Rust závislostí
cd src-tauri
cargo fetch
cd ..
```

## Struktura projektu

```
Claude-Desktop/
├── docs/                      # Dokumentace
│   ├── TESTING.md            # Průvodce testováním
│   ├── DEVELOPMENT.md        # Tento soubor
│   └── API.md                # API dokumentace
│
├── src-tauri/                # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs          # Vstupní bod, Tauri commands
│   │   ├── auth.rs          # Autentizační modul
│   │   ├── mcp/             # Správa MCP serverů
│   │   │   ├── mod.rs       # Config loader, parser
│   │   │   ├── config.rs    # Path utility
│   │   │   └── launcher.rs  # Server launcher
│   │   └── debug.rs         # Logování, profilování
│   ├── Cargo.toml           # Rust závislosti
│   ├── tauri.conf.json      # Tauri konfigurace
│   └── build.rs             # Build skript
│
├── src/                      # Frontend (Web)
│   ├── index.html           # Hlavní UI
│   ├── styles/
│   │   └── main.css         # Styling (tmavý režim)
│   └── js/
│       └── app.js           # Frontend logika, Tauri bridge
│
├── package.json             # Frontend závislosti, skripty
├── .gitignore
├── README.md
└── CLAUDE.md                # Průvodce pro AI asistenta
```

### Klíčové soubory

| Soubor | Účel |
|--------|------|
| `src-tauri/src/main.rs` | Vstupní bod Tauri app, command handlery |
| `src-tauri/tauri.conf.json` | Nastavení okna, bundle config |
| `src/index.html` | Hlavní UI aplikace |
| `src/js/app.js` | Frontend<->Backend komunikace |

## Vývojový workflow

### 1. Spuštění dev serveru

```bash
npm run dev
```

To provede:
- Spustí Tauri dev server
- Zapne hot-reload pro frontend
- Přebuildí Rust při změnách
- Otevře okno aplikace

### 2. Provádění změn

**Rust Backend:**
```bash
# Edituj soubory v src-tauri/src/
nvim src-tauri/src/auth.rs

# Tauri automaticky přebuduje při uložení
```

**Frontend:**
```bash
# Edituj soubory v src/
nvim src/index.html
nvim src/styles/main.css
nvim src/js/app.js

# Prohlížeč se automaticky obnoví při uložení
```

### 3. Testování změn

```bash
# Spuštění Rust testů
cd src-tauri
cargo test

# Spuštění linteru
cargo clippy

# Formátování kódu
cargo fmt
```

### 4. Commit změn

```bash
git add .
git commit -m "feat: přidání nové funkce"
```

Dodržuj [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` - Nová funkce
- `fix:` - Oprava chyby
- `docs:` - Dokumentace
- `refactor:` - Refaktoring kódu
- `test:` - Testy
- `chore:` - Údržba

## Debugování

### Zapnutí debug logů

Vývojový režim automaticky zapíná debug logování:

```bash
npm run dev

# Zobrazí se:
# 🐛 Debug logging enabled
# 🦀 Claude Desktop (Tauri) v0.1.0
# ⏱️ Starting: initialization
# ✅ Finished: initialization (0.05s)
```

### Rust Debugger (rust-lldb)

```bash
# Instalace lldb
sudo apt install lldb

# Build debug verze
cd src-tauri
cargo build

# Debug
rust-lldb target/debug/claude-desktop
(lldb) breakpoint set --name main
(lldb) run
```

### Frontend debugování

```bash
# Otevři DevTools v Tauri okně
npm run dev

# V okně aplikace:
# Pravé tlačítko > Inspect Element
# Nebo: Ctrl+Shift+I (pokud povoleno v tauri.conf.json)
```

### Profilování výkonu

```rust
use crate::debug::PerfTimer;

#[tauri::command]
fn pomalý_příkaz() -> Result<String, String> {
    let _timer = PerfTimer::new("pomalý_příkaz");

    // Tvůj kód zde
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok("Hotovo".to_string())
    // Timer automaticky loguje při drop:
    // ✅ Finished: pomalý_příkaz (1.00s)
}
```

### Běžné problémy

**Problém: "Failed to load native addon"**
```bash
# Přebuduj Rust moduly
cd src-tauri
cargo clean
cargo build
```

**Problém: "webkit2gtk not found"**
```bash
# Instaluj WebKit závislosti
sudo apt install libwebkit2gtk-4.1-dev
```

**Problém: "Permission denied" při buildu**
```bash
# Oprav oprávnění
chmod +x ./scripts/*.sh
```

## Build proces

### Vývojový build

```bash
npm run dev
```

### Produkční build

```bash
# Build všech formátů
npm run build

# Build konkrétního formátu
npm run build:deb        # .deb balíček
npm run build:appimage   # .AppImage
npm run build:rpm        # .rpm balíček
```

### Build výstup

```
src-tauri/target/release/
├── claude-desktop                    # Binární executable
└── bundle/
    ├── deb/
    │   └── claude-desktop_0.1.0_amd64.deb
    ├── appimage/
    │   └── claude-desktop_0.1.0_amd64.AppImage
    └── rpm/
        └── claude-desktop-0.1.0-1.x86_64.rpm
```

### Optimalizace buildu

Aktuální nastavení (`Cargo.toml`):
```toml
[profile.release]
panic = "abort"        # Menší binárka
codegen-units = 1      # Lepší optimalizace
lto = true             # Link-time optimizace
opt-level = "z"        # Optimalizace pro velikost
strip = true           # Odstranit debug symboly
```

**Velikost binárky:** ~5-8 MB (vs Electron ~150 MB)

### Cross-kompilace

Build pro různé architektury:

```bash
# Instalace cross-kompilačních nástrojů
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Build pro ARM64
cargo build --release --target aarch64-unknown-linux-gnu
```

## Přispívání

### Před odesláním PR

1. **Spusť testy**
   ```bash
   cd src-tauri
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

2. **Aktualizuj dokumentaci**
   - Aktualizuj relevantní `.md` soubory
   - Přidej docstringy k novým funkcím
   - Vytvoř `.cz.md` verze pro českou dokumentaci

3. **Dodržuj konvence**
   - Rust: Dodržuj [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
   - Git: Používej [Conventional Commits](https://www.conventionalcommits.org/)
   - Styl kódu: Spusť `cargo fmt`

4. **Testuj na Linuxu**
   - Ubuntu/Debian (apt-based)
   - Fedora (rpm-based)
   - Arch (pacman-based)

## Užitečné příkazy

```bash
# Vývoj
npm run dev                  # Spuštění dev serveru
npm run build               # Build produkce

# Testování
cargo test                  # Spuštění všech testů
cargo test auth::tests      # Spuštění konkrétních testů
cargo test -- --nocapture   # Zobrazení print výpisů

# Kvalita kódu
cargo clippy                # Lint Rust kódu
cargo fmt                   # Formátování Rust kódu
cargo check                 # Kontrola bez buildu

# Závislosti
cargo update                # Aktualizace Rust závislostí
npm update                  # Aktualizace Node závislostí

# Úklid
cargo clean                 # Vyčištění Rust buildu
rm -rf node_modules         # Vyčištění Node modulů
```

## Zdroje

- [Tauri dokumentace](https://tauri.app/)
- [Rust kniha](https://doc.rust-lang.org/book/)
- [WebKitGTK](https://webkitgtk.org/)
- [MCP protokol](https://modelcontextprotocol.io/)

---

**Poslední aktualizace:** 2025-11-18
**Správce:** milhy545
