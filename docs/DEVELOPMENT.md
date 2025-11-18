# Development Guide

Complete guide for developing Claude Desktop (Tauri Edition).

## Table of Contents

- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Debugging](#debugging)
- [Build Process](#build-process)
- [Contributing](#contributing)

## Development Setup

### System Requirements

- **OS:** Linux (Ubuntu 20.04+, Debian, Fedora, Arch)
- **RAM:** 4GB minimum, 8GB recommended
- **Disk:** 2GB free space for dependencies

### Install Dependencies

#### 1. Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify:
```bash
rustc --version  # Should be 1.70+
cargo --version
```

#### 2. Node.js

```bash
# Via nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20

# Or via apt
sudo apt install nodejs npm
```

Verify:
```bash
node --version  # Should be 18+
npm --version
```

#### 3. Tauri Dependencies (Ubuntu/Debian)

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
# Or use project-local version (recommended)
npm install
```

### Clone and Setup

```bash
git clone https://github.com/milhy545/Claude-Desktop.git
cd Claude-Desktop

# Install frontend dependencies
npm install

# Fetch Rust dependencies
cd src-tauri
cargo fetch
cd ..
```

## Project Structure

```
Claude-Desktop/
├── docs/                      # Documentation
│   ├── TESTING.md            # Testing guide
│   ├── DEVELOPMENT.md        # This file
│   └── API.md                # API documentation
│
├── src-tauri/                # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs          # Entry point, Tauri commands
│   │   ├── auth.rs          # Authentication module
│   │   ├── mcp/             # MCP server management
│   │   │   ├── mod.rs       # Config loader, parser
│   │   │   ├── config.rs    # Path utilities
│   │   │   └── launcher.rs  # Server launcher
│   │   └── debug.rs         # Logging, profiling
│   ├── Cargo.toml           # Rust dependencies
│   ├── tauri.conf.json      # Tauri configuration
│   └── build.rs             # Build script
│
├── src/                      # Frontend (Web)
│   ├── index.html           # Main UI
│   ├── styles/
│   │   └── main.css         # Styling (dark theme)
│   └── js/
│       └── app.js           # Frontend logic, Tauri bridge
│
├── package.json             # Frontend dependencies, scripts
├── .gitignore
├── README.md
└── CLAUDE.md                # AI assistant guide
```

### Key Files

| File | Purpose |
|------|---------|
| `src-tauri/src/main.rs` | Tauri app entry point, command handlers |
| `src-tauri/tauri.conf.json` | Window settings, bundle config |
| `src/index.html` | Main application UI |
| `src/js/app.js` | Frontend<->Backend communication |

## Development Workflow

### 1. Start Development Server

```bash
npm run dev
```

This will:
- Start Tauri dev server
- Enable hot-reload for frontend
- Rebuild Rust on changes
- Open application window

### 2. Make Changes

**Rust Backend:**
```bash
# Edit files in src-tauri/src/
nvim src-tauri/src/auth.rs

# Tauri will auto-rebuild on save
```

**Frontend:**
```bash
# Edit files in src/
nvim src/index.html
nvim src/styles/main.css
nvim src/js/app.js

# Browser will auto-refresh on save
```

### 3. Test Changes

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

### 4. Commit Changes

```bash
git add .
git commit -m "feat: add new feature"
```

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `refactor:` - Code refactoring
- `test:` - Tests
- `chore:` - Maintenance

## Debugging

### Enable Debug Logs

Development mode automatically enables debug logging:

```bash
npm run dev

# You'll see:
# 🐛 Debug logging enabled
# 🦀 Claude Desktop (Tauri) v0.1.0
# ⏱️ Starting: initialization
# ✅ Finished: initialization (0.05s)
```

### Rust Debugger (rust-lldb)

```bash
# Install lldb
sudo apt install lldb

# Build debug version
cd src-tauri
cargo build

# Debug
rust-lldb target/debug/claude-desktop
(lldb) breakpoint set --name main
(lldb) run
```

### Frontend Debugging

```bash
# Open DevTools in Tauri window
npm run dev

# In the app window:
# Right-click > Inspect Element
# Or: Ctrl+Shift+I (if enabled in tauri.conf.json)
```

### Performance Profiling

```rust
use crate::debug::PerfTimer;

#[tauri::command]
fn slow_command() -> Result<String, String> {
    let _timer = PerfTimer::new("slow_command");

    // Your code here
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok("Done".to_string())
    // Timer automatically logs on drop:
    // ✅ Finished: slow_command (1.00s)
}
```

### Common Issues

**Issue: "Failed to load native addon"**
```bash
# Rebuild Rust modules
cd src-tauri
cargo clean
cargo build
```

**Issue: "webkit2gtk not found"**
```bash
# Install WebKit dependencies
sudo apt install libwebkit2gtk-4.1-dev
```

**Issue: "Permission denied" on build**
```bash
# Fix permissions
chmod +x ./scripts/*.sh
```

## Build Process

### Development Build

```bash
npm run dev
```

### Production Build

```bash
# Build all formats
npm run build

# Build specific format
npm run build:deb        # .deb package
npm run build:appimage   # .AppImage
npm run build:rpm        # .rpm package
```

### Build Output

```
src-tauri/target/release/
├── claude-desktop                    # Binary executable
└── bundle/
    ├── deb/
    │   └── claude-desktop_0.1.0_amd64.deb
    ├── appimage/
    │   └── claude-desktop_0.1.0_amd64.AppImage
    └── rpm/
        └── claude-desktop-0.1.0-1.x86_64.rpm
```

### Build Optimization

Current settings (`Cargo.toml`):
```toml
[profile.release]
panic = "abort"        # Smaller binary
codegen-units = 1      # Better optimization
lto = true             # Link-time optimization
opt-level = "z"        # Optimize for size
strip = true           # Remove debug symbols
```

**Binary size:** ~5-8 MB (vs Electron ~150 MB)

### Cross-Compilation

Build for different architectures:

```bash
# Install cross-compilation tools
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Build for ARM64
cargo build --release --target aarch64-unknown-linux-gnu
```

## Contributing

### Before Submitting PR

1. **Run tests**
   ```bash
   cd src-tauri
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

2. **Update documentation**
   - Update relevant `.md` files
   - Add docstrings to new functions
   - Create `.cz.md` versions for Czech docs

3. **Follow conventions**
   - Rust: Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
   - Git: Use [Conventional Commits](https://www.conventionalcommits.org/)
   - Code style: Run `cargo fmt`

4. **Test on Linux**
   - Ubuntu/Debian (apt-based)
   - Fedora (rpm-based)
   - Arch (pacman-based)

### Development Tips

**Rust Best Practices:**
```rust
// ✅ Good: Descriptive names, error handling
pub fn load_config() -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to load config: {}", e))
}

// ❌ Bad: Panic, unclear errors
pub fn load_config() -> String {
    std::fs::read_to_string(path).unwrap()
}
```

**Frontend Best Practices:**
```javascript
// ✅ Good: Async/await, error handling
async function loadConfig() {
    try {
        const config = await invoke('load_mcp_config');
        return config;
    } catch (error) {
        console.error('Failed to load config:', error);
        throw error;
    }
}

// ❌ Bad: No error handling
function loadConfig() {
    return invoke('load_mcp_config');
}
```

### Hot Reload Issues

If hot reload stops working:

```bash
# Kill dev server
Ctrl+C

# Clean build cache
cd src-tauri
cargo clean

# Restart
cd ..
npm run dev
```

## Useful Commands

```bash
# Development
npm run dev                  # Start dev server
npm run build               # Build production

# Testing
cargo test                  # Run all tests
cargo test auth::tests      # Run specific tests
cargo test -- --nocapture   # See print statements

# Code Quality
cargo clippy                # Lint Rust code
cargo fmt                   # Format Rust code
cargo check                 # Check without building

# Dependencies
cargo update                # Update Rust deps
npm update                  # Update Node deps

# Cleaning
cargo clean                 # Clean Rust build
rm -rf node_modules         # Clean Node modules
```

## Resources

- [Tauri Documentation](https://tauri.app/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WebKitGTK](https://webkitgtk.org/)
- [MCP Protocol](https://modelcontextprotocol.io/)

---

**Last Updated:** 2025-11-18
**Maintainer:** milhy545
