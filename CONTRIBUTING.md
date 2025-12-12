# Contributing to Claude Desktop

Thank you for your interest in contributing to Claude Desktop! 🎉

This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Documentation](#documentation)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all.

**We pledge to:**
- Be respectful and inclusive
- Welcome newcomers
- Accept constructive criticism
- Focus on what's best for the community

**We do NOT tolerate:**
- Harassment or discrimination
- Trolling or inflammatory comments
- Personal attacks
- Spam or off-topic discussions

## Getting Started

### Prerequisites

Before contributing, make sure you have:

```bash
# Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (18+)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20

# Tauri dependencies (Ubuntu/Debian)
sudo apt install libwebkit2gtk-4.1-dev build-essential libssl-dev libgtk-3-dev
```

See [DEVELOPMENT.md](docs/DEVELOPMENT.md) for complete setup instructions.

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Claude-Desktop.git
   cd Claude-Desktop
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/milhy545/Claude-Desktop.git
   ```

## Development Workflow

### 1. Create a Branch

```bash
# Update your fork
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/amazing-feature

# Or for bug fixes
git checkout -b fix/bug-description
```

### 2. Make Changes

```bash
# Start development server
npm run dev

# Make your changes
# Edit files in src-tauri/src/ or src/

# Test your changes
cd src-tauri
cargo test
cargo clippy
cargo fmt
```

### 3. Commit Changes

Use [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git add .
git commit -m "feat: add amazing feature"
```

**Commit types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting)
- `refactor:` - Code refactoring
- `test:` - Test additions or changes
- `chore:` - Maintenance tasks
- `perf:` - Performance improvements

**Examples:**
```bash
git commit -m "feat: add MCP server auto-restart"
git commit -m "fix: resolve memory leak in auth module"
git commit -m "docs: update installation instructions"
```

### 4. Push Changes

```bash
git push origin feature/amazing-feature
```

## Pull Request Process

### Before Submitting

**Required checks:**

```bash
# 1. Run tests
cd src-tauri
cargo test

# 2. Run linter
cargo clippy -- -D warnings

# 3. Format code
cargo fmt

# 4. Build project
cd ..
npm run build
```

All must pass! ✅

### Creating the PR

1. Go to your fork on GitHub
2. Click "Pull Request"
3. Select:
   - Base: `milhy545/Claude-Desktop` `main`
   - Compare: `your-fork` `feature/amazing-feature`
4. Fill out the PR template
5. Submit!

### PR Guidelines

**Good PR:**
- ✅ Clear title and description
- ✅ References issue number (`Fixes #123`)
- ✅ Small, focused changes
- ✅ Tests included
- ✅ Documentation updated
- ✅ All CI checks pass

**Bad PR:**
- ❌ Vague description
- ❌ Massive changes (1000+ lines)
- ❌ No tests
- ❌ Breaking changes without discussion
- ❌ Failing CI checks

### Review Process

1. **Automated checks** run (CI, tests, linting)
2. **Maintainer review** (may request changes)
3. **You address** feedback
4. **Approval** and merge! 🎉

**Timeline:**
- Initial review: 1-3 days
- Follow-up reviews: 1-2 days

## Coding Standards

### Rust Code

**Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):**

```rust
// ✅ Good
pub fn load_config() -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to load config: {}", e))
}

// ❌ Bad
pub fn load_config() -> String {
    std::fs::read_to_string(path).unwrap()
}
```

**Naming:**
- `snake_case` for functions and variables
- `PascalCase` for types and structs
- `UPPER_SNAKE_CASE` for constants

**Error handling:**
- Use `Result<T, E>` for fallible operations
- Provide descriptive error messages
- Don't use `.unwrap()` in library code

**Documentation:**
```rust
/// Loads MCP configuration from disk.
///
/// # Returns
/// JSON configuration as string
///
/// # Errors
/// Returns error if file cannot be read
pub fn load_config() -> Result<String, String> {
    // ...
}
```

### JavaScript/CSS Code

**JavaScript:**
```javascript
// ✅ Good
async function loadConfig() {
    try {
        const config = await invoke('load_mcp_config');
        return JSON.parse(config);
    } catch (error) {
        console.error('Failed to load config:', error);
        throw error;
    }
}

// ❌ Bad
function loadConfig() {
    return invoke('load_mcp_config');
}
```

**CSS:**
- Use CSS variables for theming
- Follow BEM naming convention
- Keep selectors specific but not overly nested

## Testing Requirements

### Unit Tests Required

For **all new features** and **bug fixes**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_works() {
        let result = my_function();
        assert!(result.is_ok());
    }

    #[test]
    fn test_feature_handles_errors() {
        let result = my_function_with_bad_input();
        assert!(result.is_err());
    }
}
```

### Test Coverage

- **Minimum:** 70% coverage
- **Target:** 80%+ coverage
- Run: `cargo tarpaulin`

### Manual Testing

Before submitting, test:
1. ✅ Build succeeds
2. ✅ Application starts
3. ✅ Feature works as expected
4. ✅ No console errors
5. ✅ No regressions

## Documentation

### Update Documentation

If you change functionality, update:

1. **Code comments** - Rust docstrings
2. **API docs** - `docs/API.md` and `docs/API.cz.md`
3. **User docs** - `README.md` if user-facing
4. **Development docs** - `docs/DEVELOPMENT.md` if needed

### Czech Translations

**Important:** Update Czech versions!

If you modify:
- `docs/TESTING.md` → Also update `docs/TESTING.cz.md`
- `docs/DEVELOPMENT.md` → Also update `docs/DEVELOPMENT.cz.md`
- `docs/API.md` → Also update `docs/API.cz.md`

Both versions should have the same information.

## Issue Reporting

### Bug Reports

Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md):

**Include:**
- OS and version
- Steps to reproduce
- Expected vs actual behavior
- Error messages
- Screenshots if applicable

### Feature Requests

Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md):

**Include:**
- Problem description
- Proposed solution
- Alternative solutions
- Additional context

## Getting Help

**Stuck?** We're here to help!

- 💬 [GitHub Discussions](https://github.com/milhy545/Claude-Desktop/discussions)
- 🐛 [GitHub Issues](https://github.com/milhy545/Claude-Desktop/issues)
- 📖 [Documentation](docs/)

## Recognition

Contributors will be:
- Listed in `CONTRIBUTORS.md`
- Mentioned in release notes
- Credited in the project

Thank you for contributing! 🙏

---

**Questions?** Open a [discussion](https://github.com/milhy545/Claude-Desktop/discussions) and we'll help!
