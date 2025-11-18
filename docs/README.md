# Documentation / Dokumentace

Complete documentation for Claude Desktop (Tauri Edition).

Kompletní dokumentace pro Claude Desktop (Tauri Edition).

## 📚 English Documentation

| Document | Description |
|----------|-------------|
| [TESTING.md](TESTING.md) | Testing guide - how to run tests, debug, and measure performance |
| [DEVELOPMENT.md](DEVELOPMENT.md) | Development guide - setup, workflow, build process |
| [API.md](API.md) | API reference - all Tauri commands and their usage |

## 🇨🇿 Česká dokumentace

| Dokument | Popis |
|----------|-------|
| [TESTING.cz.md](TESTING.cz.md) | Průvodce testováním - jak spouštět testy, debugovat a měřit výkon |
| [DEVELOPMENT.cz.md](DEVELOPMENT.cz.md) | Průvodce vývojem - nastavení, workflow, build proces |
| [API.cz.md](API.cz.md) | API reference - všechny Tauri příkazy a jejich použití |

## Quick Links / Rychlé odkazy

### For Users / Pro uživatele

- [README.md](../README.md) - Main project README / Hlavní README projektu
- [Installation](../README.md#installation) - How to install / Jak nainstalovat
- [Configuration](../README.md#configuration) - MCP server setup / Nastavení MCP serverů

### For Developers / Pro vývojáře

- [Development Setup](DEVELOPMENT.md#development-setup) - Get started developing / Začni vyvíjet
- [Project Structure](DEVELOPMENT.md#project-structure) - Understand the codebase / Pochop kódovou základnu
- [API Commands](API.md#tauri-commands) - Available commands / Dostupné příkazy
- [Testing](TESTING.md#running-tests) - Run tests / Spusť testy

### For Contributors / Pro přispěvatele

- [Contributing Guide](../README.md#contributing) - How to contribute / Jak přispět
- [Code Style](DEVELOPMENT.md#contributing) - Coding standards / Standardy kódu
- [Testing Requirements](TESTING.md#best-practices) - Test guidelines / Pokyny pro testování

## Document Conventions / Konvence dokumentů

### English Documents (.md)

- Written in English
- Technical, precise language
- Code examples in comments use English
- Aimed at international contributors

### Czech Documents (.cz.md)

- Written in Czech (Čeština)
- Same content as English version
- Code examples in comments use Czech
- Aimed at Czech-speaking users

## Contributing to Documentation / Přispívání do dokumentace

When updating documentation / Při aktualizaci dokumentace:

1. **Update English version first** (.md)
   - Make changes to the English document
   - Ensure accuracy and clarity

2. **Update Czech version** (.cz.md)
   - Translate the changes to Czech
   - Maintain the same structure
   - Keep code examples consistent

3. **Keep them in sync**
   - Both versions should have the same information
   - Update both in the same commit

Example commit message:
```
docs: update API documentation for new MCP commands

- Added start_mcp_server and stop_mcp_server commands
- Updated configuration examples
- Fixed typos in testing guide

Updated both EN and CZ versions.
```

## Documentation Structure / Struktura dokumentace

```
docs/
├── README.md              # This file / Tento soubor
├── TESTING.md             # Testing guide (EN)
├── TESTING.cz.md          # Průvodce testováním (CZ)
├── DEVELOPMENT.md         # Development guide (EN)
├── DEVELOPMENT.cz.md      # Průvodce vývojem (CZ)
├── API.md                 # API reference (EN)
└── API.cz.md              # API reference (CZ)
```

## Need Help? / Potřebuješ pomoc?

- **English:** Open an issue on [GitHub](https://github.com/milhy545/Claude-Desktop/issues)
- **Čeština:** Otevři issue na [GitHubu](https://github.com/milhy545/Claude-Desktop/issues)

---

**Last Updated / Poslední aktualizace:** 2025-11-18
