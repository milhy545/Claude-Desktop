# Přispívání do Claude Desktop

Děkujeme za zájem přispět do Claude Desktop! 🎉

Tento dokument poskytuje pokyny pro přispívání do projektu.

## Obsah

- [Kodex chování](#kodex-chování)
- [Začínáme](#začínáme)
- [Vývojový workflow](#vývojový-workflow)
- [Proces pull requestu](#proces-pull-requestu)
- [Standardy kódu](#standardy-kódu)
- [Požadavky na testování](#požadavky-na-testování)
- [Dokumentace](#dokumentace)

## Kodex chování

### Náš závazek

Zavazujeme se poskytovat vstřícnou a inspirující komunitu pro všechny.

**Slibujeme:**
- Být respektoví a inkluzivní
- Vítat nováčky
- Přijímat konstruktivní kritiku
- Zaměřovat se na to, co je nejlepší pro komunitu

**NETOLERUJEME:**
- Obtěžování nebo diskriminaci
- Trolling nebo podněcující komentáře
- Osobní útoky
- Spam nebo off-topic diskuze

## Začínáme

### Požadavky

Před přispíváním se ujisti, že máš:

```bash
# Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (18+)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20

# Tauri závislosti (Ubuntu/Debian)
sudo apt install libwebkit2gtk-4.1-dev build-essential libssl-dev libgtk-3-dev
```

Viz [DEVELOPMENT.cz.md](docs/DEVELOPMENT.cz.md) pro kompletní instalační instrukce.

### Fork a Clone

1. Forkni repozitář na GitHubu
2. Naklonuj svůj fork:
   ```bash
   git clone https://github.com/TVOJE_JMENO/Claude-Desktop.git
   cd Claude-Desktop
   ```
3. Přidej upstream remote:
   ```bash
   git remote add upstream https://github.com/milhy545/Claude-Desktop.git
   ```

## Vývojový workflow

### 1. Vytvoř branch

```bash
# Aktualizuj svůj fork
git checkout main
git pull upstream main

# Vytvoř feature branch
git checkout -b feature/super-funkce

# Nebo pro opravu chyby
git checkout -b fix/popis-chyby
```

### 2. Proved změny

```bash
# Spusť vývojový server
npm run dev

# Proved změny
# Edituj soubory v src-tauri/src/ nebo src/

# Otestuj změny
cd src-tauri
cargo test
cargo clippy
cargo fmt
```

### 3. Commitni změny

Používej [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git add .
git commit -m "feat: přidání super funkce"
```

**Typy commitů:**
- `feat:` - Nová funkce
- `fix:` - Oprava chyby
- `docs:` - Změny v dokumentaci
- `style:` - Změny stylu kódu (formátování)
- `refactor:` - Refaktoring kódu
- `test:` - Přidání nebo změny testů
- `chore:` - Údržbové úkoly
- `perf:` - Vylepšení výkonu

**Příklady:**
```bash
git commit -m "feat: přidání auto-restartu MCP serverů"
git commit -m "fix: oprava memory leaku v auth modulu"
git commit -m "docs: aktualizace instalačních instrukcí"
```

### 4. Pushni změny

```bash
git push origin feature/super-funkce
```

## Proces pull requestu

### Před odesláním

**Povinné kontroly:**

```bash
# 1. Spusť testy
cd src-tauri
cargo test

# 2. Spusť linter
cargo clippy -- -D warnings

# 3. Naformátuj kód
cargo fmt

# 4. Sestavení projektu
cd ..
npm run build
```

Všechny musí projít! ✅

### Vytvoření PR

1. Jdi na svůj fork na GitHubu
2. Klikni na "Pull Request"
3. Vyber:
   - Base: `milhy545/Claude-Desktop` `main`
   - Compare: `tvůj-fork` `feature/super-funkce`
4. Vyplň PR template
5. Odešli!

### Pokyny pro PR

**Dobrý PR:**
- ✅ Jasný název a popis
- ✅ Odkazuje na číslo issue (`Fixes #123`)
- ✅ Malé, zaměřené změny
- ✅ Testy přiloženy
- ✅ Dokumentace aktualizována
- ✅ Všechny CI kontroly prošly

**Špatný PR:**
- ❌ Vágní popis
- ❌ Obrovské změny (1000+ řádků)
- ❌ Žádné testy
- ❌ Breaking changes bez diskuze
- ❌ Selhávající CI kontroly

### Proces review

1. **Automatické kontroly** běží (CI, testy, linting)
2. **Maintainer review** (může požadovat změny)
3. **Ty ošetříš** feedback
4. **Schválení** a merge! 🎉

**Časová osa:**
- Počáteční review: 1-3 dny
- Follow-up reviews: 1-2 dny

## Standardy kódu

### Rust kód

**Dodržuj [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):**

```rust
// ✅ Dobré
pub fn load_config() -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("Nepodařilo se načíst config: {}", e))
}

// ❌ Špatné
pub fn load_config() -> String {
    std::fs::read_to_string(path).unwrap()
}
```

**Pojmenování:**
- `snake_case` pro funkce a proměnné
- `PascalCase` pro typy a struktury
- `UPPER_SNAKE_CASE` pro konstanty

**Zpracování chyb:**
- Používej `Result<T, E>` pro operace, které mohou selhat
- Poskytuj popisné chybové zprávy
- Nepoužívej `.unwrap()` v library kódu

**Dokumentace:**
```rust
/// Načte MCP konfiguraci z disku.
///
/// # Returns
/// JSON konfigurace jako string
///
/// # Errors
/// Vrací chybu, pokud soubor nelze přečíst
pub fn load_config() -> Result<String, String> {
    // ...
}
```

### JavaScript/CSS kód

**JavaScript:**
```javascript
// ✅ Dobré
async function loadConfig() {
    try {
        const config = await invoke('load_mcp_config');
        return JSON.parse(config);
    } catch (error) {
        console.error('Nepodařilo se načíst config:', error);
        throw error;
    }
}

// ❌ Špatné
function loadConfig() {
    return invoke('load_mcp_config');
}
```

**CSS:**
- Používej CSS proměnné pro theming
- Dodržuj BEM konvenci pojmenování
- Udržuj selektory specifické, ale ne příliš vnořené

## Požadavky na testování

### Unit testy jsou povinné

Pro **všechny nové funkce** a **opravy chyb**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funkce_funguje() {
        let result = moje_funkce();
        assert!(result.is_ok());
    }

    #[test]
    fn test_funkce_zpracovava_chyby() {
        let result = moje_funkce_se_spatnym_vstupem();
        assert!(result.is_err());
    }
}
```

### Pokrytí testy

- **Minimum:** 70% pokrytí
- **Cíl:** 80%+ pokrytí
- Spuštění: `cargo tarpaulin`

### Manuální testování

Před odesláním otestuj:
1. ✅ Build uspěje
2. ✅ Aplikace se spustí
3. ✅ Funkce funguje jak očekáváno
4. ✅ Žádné chyby v konzoli
5. ✅ Žádné regrese

## Dokumentace

### Aktualizuj dokumentaci

Pokud měníš funkcionalitu, aktualizuj:

1. **Komentáře v kódu** - Rust docstringy
2. **API docs** - `docs/API.md` a `docs/API.cz.md`
3. **Uživatelské docs** - `README.md` pokud user-facing
4. **Development docs** - `docs/DEVELOPMENT.md` pokud potřeba

### České překlady

**Důležité:** Aktualizuj české verze!

Pokud upravuješ:
- `docs/TESTING.md` → Také aktualizuj `docs/TESTING.cz.md`
- `docs/DEVELOPMENT.md` → Také aktualizuj `docs/DEVELOPMENT.cz.md`
- `docs/API.md` → Také aktualizuj `docs/API.cz.md`

Obě verze by měly mít stejné informace.

## Hlášení problémů

### Bug reporty

Použij [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md):

**Zahrň:**
- OS a verzi
- Kroky k reprodukci
- Očekávané vs skutečné chování
- Chybové zprávy
- Screenshoty pokud relevantní

### Feature requesty

Použij [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md):

**Zahrň:**
- Popis problému
- Navrhované řešení
- Alternativní řešení
- Dodatečný kontext

## Získání pomoci

**Zasekl ses?** Jsme tu, abychom pomohli!

- 💬 [GitHub Discussions](https://github.com/milhy545/Claude-Desktop/discussions)
- 🐛 [GitHub Issues](https://github.com/milhy545/Claude-Desktop/issues)
- 📖 [Dokumentace](docs/)

## Uznání

Přispěvatelé budou:
- Uvedeni v `CONTRIBUTORS.md`
- Zmíněni v release notes
- Ocenění v projektu

Děkujeme za přispívání! 🙏

---

**Otázky?** Otevři [diskuzi](https://github.com/milhy545/Claude-Desktop/discussions) a pomůžeme!
