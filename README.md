# 🦀 Claude Desktop - Tauri Edition

**Lightweight, fast Linux desktop client for Claude AI** - Built with Tauri (Rust + Web) instead of Electron.

## ⚡ Why Tauri?

| Feature | Tauri (This project) | Electron (Official) |
|---------|---------------------|---------------------|
| **Memory (idle)** | ~30-50 MB 🚀 | ~200-400 MB 🐌 |
| **Binary size** | ~5-8 MB 📦 | ~150 MB 📦📦📦 |
| **Startup time** | <1 second ⚡ | 3-5 seconds 🐌 |
| **CPU usage** | Low 💚 | High 🔥 |

## ✨ Features

- ✅ **Native Linux performance** - No Electron bloat
- ✅ **MCP Server support** - Full Model Context Protocol integration
- ✅ **Session authentication** - Uses your Claude subscription (no API keys)
- ✅ **System tray** - Quick access from taskbar
- ✅ **Global hotkey** - `Ctrl+Alt+Space` to open
- ✅ **Dark theme** - Easy on the eyes
- ✅ **Config UI** - Manage MCP servers through GUI

## 📦 Installation

### Download Pre-built Packages

**Coming soon!** Check [Releases](https://github.com/milhy545/Claude-Desktop/releases) for:
- `.deb` (Ubuntu, Debian, Mint)
- `.AppImage` (Universal Linux)
- `.rpm` (Fedora, RHEL)

### Build from Source

#### Prerequisites

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Tauri dependencies (Ubuntu/Debian)
sudo apt install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Node.js (for frontend tooling)
# Install via nvm, apt, or snap
```

#### Build

```bash
# Clone repository
git clone https://github.com/milhy545/Claude-Desktop.git
cd Claude-Desktop

# Install frontend dependencies
npm install

# Build (choose one)
npm run build              # All formats
npm run build:deb          # .deb package
npm run build:appimage     # .AppImage
npm run build:rpm          # .rpm package

# Output: src-tauri/target/release/bundle/
```

#### Development

```bash
npm run dev
```

## 🔧 Configuration

### MCP Servers

Config file: `~/.config/Claude/claude_desktop_config.json`

Example:

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-filesystem",
        "/home/user/Documents",
        "/home/user/Projects"
      ]
    },
    "git": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-git"]
    },
    "sqlite": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-sqlite",
        "--db-path",
        "/home/user/my-database.db"
      ]
    }
  }
}
```

**Edit via GUI:** Open app → Settings (⚙️) → Edit config

### Authentication

Uses [claude-cli-authentication](https://github.com/milhy545/claude-cli-authentication) for session management.

Session stored in: `~/.claude/`

## ⌨️ Keyboard Shortcuts

- `Ctrl+Alt+Space` - Show/focus window (global)
- `Ctrl+,` - Open settings
- `Esc` - Close modal/settings

## 🏗️ Architecture

```
Rust Backend (Tauri)
├── Authentication (claude CLI integration)
├── MCP Server Manager (launch/stop npx/python/binary)
├── Config Parser (JSON)
└── System Integration (tray, hotkeys)

Web Frontend
├── Embedded claude.ai (iframe)
├── Settings UI
└── Server management UI
```

## 🤝 Contributing

Contributions welcome! This is a community project.

1. Fork the repo
2. Create feature branch: `git checkout -b feature/amazing`
3. Commit: `git commit -m 'feat: add amazing feature'`
4. Push: `git push origin feature/amazing`
5. Open PR

## 📄 License

MIT License - See [LICENSE](LICENSE)

## 🙏 Credits

- **Anthropic** - For Claude AI
- **Tauri** - For the amazing framework
- **Community** - For Electron alternatives inspiration

## 🐛 Known Issues

- First launch might be slow (npx downloads MCP servers)
- WebKitGTK might have some rendering quirks vs Chromium
- Some claude.ai features might not work in iframe

## 🗺️ Roadmap

- [ ] Auto-update mechanism
- [ ] Desktop Extensions (.mcpb) support
- [ ] Custom themes
- [ ] Multi-window support
- [ ] Notification system
- [ ] Offline mode

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/milhy545/Claude-Desktop/issues)
- **Discussions:** [GitHub Discussions](https://github.com/milhy545/Claude-Desktop/discussions)

---

**Made with 🦀 Rust and ❤️ for Linux users**

*"Protože Linux uživatelé si zaslouží stejně dobrý desktop klient jako Mac/Windows - ale rychlejší!"*

