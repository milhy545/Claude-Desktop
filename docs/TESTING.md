# Testing Guide

This document describes how to test Claude Desktop (Tauri Edition).

## Table of Contents

- [Running Tests](#running-tests)
- [Unit Tests](#unit-tests)
- [Integration Tests](#integration-tests)
- [Debug Mode](#debug-mode)
- [Performance Testing](#performance-testing)

## Running Tests

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
cd Claude-Desktop
npm install
cargo fetch
```

### Run All Tests

```bash
# Rust backend tests
cd src-tauri
cargo test

# Verbose output
cargo test -- --nocapture

# Specific test
cargo test test_name
```

### Run Tests with Coverage

```bash
# Install tarpaulin for coverage
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --out Html
```

## Unit Tests

### Authentication Module Tests

Located in: `src-tauri/src/auth.rs`

```bash
cargo test auth::tests
```

**Tests:**
- `test_get_session_path` - Verifies session path contains `.claude`
- `test_is_authenticated_returns_result` - Checks auth function returns Ok
- `test_logout_no_panic` - Ensures logout doesn't panic

### MCP Module Tests

Located in: `src-tauri/src/mcp/tests.rs`

```bash
cargo test mcp::tests
```

**Tests:**
- `test_parse_config_empty` - Empty config parsing
- `test_parse_config_with_servers` - Valid config with servers
- `test_parse_config_invalid_json` - Invalid JSON handling
- `test_get_config_path` - Config path validation

### Debug Module Tests

Located in: `src-tauri/src/debug.rs`

```bash
cargo test debug::tests
```

**Tests:**
- `test_init_logging` - Logging initialization
- `test_perf_timer` - Performance timer functionality

## Integration Tests

### Manual Integration Testing

1. **Build the Application**
   ```bash
   npm run dev
   ```

2. **Test Authentication Flow**
   - Click "Přihlásit se" button
   - Verify login process starts
   - Check `~/.claude/` for session files

3. **Test MCP Configuration**
   - Open Settings (⚙️ button)
   - Edit MCP config
   - Save and verify file at `~/.config/Claude/claude_desktop_config.json`

4. **Test Embedded WebView**
   - Verify claude.ai loads in iframe
   - Test chat functionality
   - Check for console errors

5. **Test System Integration**
   - Press `Ctrl+Alt+Space` global hotkey
   - Verify window shows/focuses
   - Check system tray icon

## Debug Mode

### Enable Debug Logging

Debug logging is automatically enabled in development builds:

```bash
# Development mode (debug logs enabled)
npm run dev

# See logs in terminal:
# 🐛 Debug logging enabled
# 🦀 Claude Desktop (Tauri) v0.1.0
# 📦 OS: linux x86_64
```

### Log Levels

```rust
log::error!("Critical error");
log::warn!("Warning message");
log::info!("Info message");
log::debug!("Debug message");
```

### Performance Profiling

Use `PerfTimer` for measuring execution time:

```rust
use crate::debug::PerfTimer;

fn slow_function() {
    let _timer = PerfTimer::new("slow_function");
    // Your code here
    // On drop, timer logs: "✅ Finished: slow_function (1.23s)"
}
```

## Performance Testing

### Memory Usage

```bash
# Build release version
npm run build

# Run and check memory
./src-tauri/target/release/claude-desktop &
ps aux | grep claude-desktop

# Expected: ~30-50 MB RSS
```

### Binary Size

```bash
# Check build output
ls -lh src-tauri/target/release/claude-desktop

# Expected: ~5-8 MB (stripped)
```

### Startup Time

```bash
# Measure startup
time ./src-tauri/target/release/claude-desktop

# Expected: <1 second
```

## Automated Testing Script

Create `scripts/test.sh`:

```bash
#!/bin/bash
set -e

echo "🧪 Running Rust tests..."
cd src-tauri
cargo test --all

echo "📊 Checking code coverage..."
cargo tarpaulin --out Stdout

echo "🔍 Running clippy (linter)..."
cargo clippy -- -D warnings

echo "📝 Checking formatting..."
cargo fmt -- --check

echo "✅ All tests passed!"
```

Make executable and run:

```bash
chmod +x scripts/test.sh
./scripts/test.sh
```

## Continuous Integration

Example `.github/workflows/test.yml`:

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential
      - name: Run tests
        run: cd src-tauri && cargo test
```

## Troubleshooting

### Tests Fail on CI

- Ensure all dependencies are installed
- Check Rust version compatibility
- Verify test isolation (no shared state)

### Permission Errors

```bash
# Give execute permission
chmod +x ./scripts/test.sh

# Run with proper permissions
sudo ./scripts/test.sh  # If needed
```

### Slow Test Execution

```bash
# Run tests in parallel
cargo test -- --test-threads=4

# Skip expensive tests in development
cargo test --lib
```

## Best Practices

1. **Write tests for all public APIs**
2. **Use descriptive test names** (`test_function_does_what`)
3. **Keep tests isolated** (no shared state)
4. **Mock external dependencies** (filesystem, network)
5. **Test edge cases** (empty inputs, errors, etc.)
6. **Run tests before committing**

## Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tauri Testing](https://tauri.app/v1/guides/testing/)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)

---

**Last Updated:** 2025-11-18
