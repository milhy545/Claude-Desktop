# Průvodce testováním

Tento dokument popisuje, jak testovat Claude Desktop (Tauri Edition).

## Obsah

- [Spouštění testů](#spouštění-testů)
- [Unit testy](#unit-testy)
- [Integrační testy](#integrační-testy)
- [Debug režim](#debug-režim)
- [Testování výkonu](#testování-výkonu)

## Spouštění testů

### Požadavky

```bash
# Instalace Rustu (pokud ještě není nainstalován)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalace závislostí
cd Claude-Desktop
npm install
cargo fetch
```

### Spuštění všech testů

```bash
# Rust backend testy
cd src-tauri
cargo test

# Podrobný výstup
cargo test -- --nocapture

# Konkrétní test
cargo test název_testu
```

### Spuštění testů s pokrytím

```bash
# Instalace tarpaulin pro pokrytí
cargo install cargo-tarpaulin

# Spuštění s pokrytím
cargo tarpaulin --out Html
```

## Unit testy

### Testy autentizačního modulu

Umístění: `src-tauri/src/auth.rs`

```bash
cargo test auth::tests
```

**Testy:**
- `test_get_session_path` - Ověří, že cesta k session obsahuje `.claude`
- `test_is_authenticated_returns_result` - Zkontroluje, že auth funkce vrací Ok
- `test_logout_no_panic` - Zajistí, že logout nezpůsobí panic

### Testy MCP modulu

Umístění: `src-tauri/src/mcp/tests.rs`

```bash
cargo test mcp::tests
```

**Testy:**
- `test_parse_config_empty` - Parsování prázdné konfigurace
- `test_parse_config_with_servers` - Platná konfigurace se servery
- `test_parse_config_invalid_json` - Zpracování neplatného JSON
- `test_get_config_path` - Validace cesty ke konfiguraci

### Testy debug modulu

Umístění: `src-tauri/src/debug.rs`

```bash
cargo test debug::tests
```

**Testy:**
- `test_init_logging` - Inicializace logování
- `test_perf_timer` - Funkcionalita měření výkonu

## Integrační testy

### Manuální integrační testování

1. **Sestavení aplikace**
   ```bash
   npm run dev
   ```

2. **Test autentizačního toku**
   - Klikni na tlačítko "Přihlásit se"
   - Ověř, že proces přihlášení začne
   - Zkontroluj `~/.claude/` pro session soubory

3. **Test MCP konfigurace**
   - Otevři nastavení (tlačítko ⚙️)
   - Uprav MCP config
   - Ulož a ověř soubor na `~/.config/Claude/claude_desktop_config.json`

4. **Test embedded webview**
   - Ověř, že se claude.ai načte v iframe
   - Otestuj chat funkcionalitu
   - Zkontroluj konzoli na chyby

5. **Test systémové integrace**
   - Stiskni globální klávesovou zkratku `Ctrl+Alt+Space`
   - Ověř zobrazení/fokus okna
   - Zkontroluj ikonu v system tray

## Debug režim

### Zapnutí debug logování

Debug logování je automaticky zapnuto ve vývojových buildech:

```bash
# Vývojový režim (debug logy zapnuté)
npm run dev

# Logy se zobrazí v terminálu:
# 🐛 Debug logging enabled
# 🦀 Claude Desktop (Tauri) v0.1.0
# 📦 OS: linux x86_64
```

### Úrovně logování

```rust
log::error!("Kritická chyba");
log::warn!("Varovná zpráva");
log::info!("Informační zpráva");
log::debug!("Debug zpráva");
```

### Profilování výkonu

Použij `PerfTimer` pro měření času vykonávání:

```rust
use crate::debug::PerfTimer;

fn pomalá_funkce() {
    let _timer = PerfTimer::new("pomalá_funkce");
    // Tvůj kód zde
    // Při drop timer zaloguje: "✅ Finished: pomalá_funkce (1.23s)"
}
```

## Testování výkonu

### Využití paměti

```bash
# Sestavení release verze
npm run build

# Spuštění a kontrola paměti
./src-tauri/target/release/claude-desktop &
ps aux | grep claude-desktop

# Očekáváno: ~30-50 MB RSS
```

### Velikost binárky

```bash
# Kontrola build výstupu
ls -lh src-tauri/target/release/claude-desktop

# Očekáváno: ~5-8 MB (stripped)
```

### Čas spuštění

```bash
# Měření spuštění
time ./src-tauri/target/release/claude-desktop

# Očekáváno: <1 sekunda
```

## Automatizovaný testovací skript

Vytvoř `scripts/test.sh`:

```bash
#!/bin/bash
set -e

echo "🧪 Spouštím Rust testy..."
cd src-tauri
cargo test --all

echo "📊 Kontrola pokrytí kódu..."
cargo tarpaulin --out Stdout

echo "🔍 Spouštím clippy (linter)..."
cargo clippy -- -D warnings

echo "📝 Kontrola formátování..."
cargo fmt -- --check

echo "✅ Všechny testy prošly!"
```

Nastav jako spustitelný a spusť:

```bash
chmod +x scripts/test.sh
./scripts/test.sh
```

## Kontinuální integrace

Příklad `.github/workflows/test.yml`:

```yaml
name: Testy

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Instalace závislostí
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential
      - name: Spuštění testů
        run: cd src-tauri && cargo test
```

## Řešení problémů

### Testy selhávají na CI

- Ujisti se, že jsou všechny závislosti nainstalovány
- Zkontroluj kompatibilitu verze Rustu
- Ověř izolaci testů (bez sdíleného stavu)

### Chyby oprávnění

```bash
# Dej právo spouštění
chmod +x ./scripts/test.sh

# Spusť s řádnými oprávněními
sudo ./scripts/test.sh  # Pokud je potřeba
```

### Pomalé vykonávání testů

```bash
# Spusť testy paralelně
cargo test -- --test-threads=4

# Přeskoč nákladné testy ve vývoji
cargo test --lib
```

## Doporučené postupy

1. **Piš testy pro všechny veřejné API**
2. **Používej popisné názvy testů** (`test_funkce_dělá_co`)
3. **Udržuj testy izolované** (bez sdíleného stavu)
4. **Mockuj externí závislosti** (filesystem, síť)
5. **Testuj okrajové případy** (prázdné vstupy, chyby, atd.)
6. **Spouštěj testy před commitem**

## Zdroje

- [Průvodce testováním v Rustu](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Testování v Tauri](https://tauri.app/v1/guides/testing/)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)

---

**Poslední aktualizace:** 2025-11-18
