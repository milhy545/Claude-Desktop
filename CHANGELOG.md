# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- N/A

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.2.0] - 2025-11-19

### Added
- Chat + Code tab switching in single window
  - New `switch_view()` Tauri command for seamless view switching
  - Tab UI for switching between Chat and Code interfaces
  - Event-based communication between Rust backend and JavaScript frontend
  - Memory-optimized single iframe approach (vs multiple windows)
- Split layout toggle option in settings (experimental)
- Unit tests for view switching functionality (6 new tests)
- Comprehensive API documentation for `switch_view` command (EN + CZ)

### Changed
- Updated API documentation with switch_view command details
- Enhanced frontend with tab-based navigation
- Improved user experience for low-memory systems

### Performance
- Reduced memory usage by consolidating to single webview
- Eliminated multi-window overhead for better system stability

## [0.1.1] - 2025-11-18

### Added
- Comprehensive testing infrastructure with 9 unit tests
- Debug logging and performance profiling utilities
- Complete documentation (EN + CZ versions)
- GitHub Actions CI/CD workflows
- Code coverage reporting with Codecov
- Contributing guidelines (CONTRIBUTING.md)
- Issue and PR templates

### Security
- Added cargo-audit security scanning in CI

## [0.1.0] - 2025-11-18

### Added
- Initial Tauri-based implementation
- MCP (Model Context Protocol) server support
- Session-based authentication via claude-cli
- Embedded claude.ai webview
- System tray integration
- Global hotkey support (Ctrl+Alt+Space)
- Dark theme UI
- Settings panel for MCP configuration
- Linux build targets (.deb, .AppImage, .rpm)

### Performance
- Binary size: ~5-8 MB (vs Electron ~150 MB)
- Memory usage: ~30-50 MB (vs Electron ~200-400 MB)
- Startup time: <1 second (vs Electron 3-5s)

---

## Release Types

### Major (X.0.0)
- Breaking changes
- Major feature additions
- Architecture changes

### Minor (0.X.0)
- New features (backward compatible)
- Significant improvements
- New MCP server types

### Patch (0.0.X)
- Bug fixes
- Performance improvements
- Documentation updates

---

**Legend:**
- `Added` - New features
- `Changed` - Changes in existing functionality
- `Deprecated` - Soon-to-be removed features
- `Removed` - Removed features
- `Fixed` - Bug fixes
- `Security` - Security fixes/improvements
