# CLAUDE.md - AI Assistant Guide for Claude Desktop

## Repository Overview

**Project Name:** Claude Desktop
**Repository:** milhy545/Claude-Desktop
**Status:** Initial Development Phase
**Purpose:** Desktop application for Claude AI assistant

This document provides comprehensive guidance for AI assistants working on this codebase, including structure, conventions, and development workflows.

---

## Current Repository State

### Structure
```
Claude-Desktop/
├── .git/                 # Git version control
├── .gitattributes       # Git line ending configuration
├── README.md            # Project documentation
└── CLAUDE.md            # This file - AI assistant guide
```

### Key Files
- **README.md** - Main project documentation (currently minimal)
- **.gitattributes** - Configured for auto LF normalization
- **Git Branch:** `claude/claude-md-mi47dbk11n54regr-01PSZjVh1vaM6R1fDzdJM3nG`

---

## Development Workflows

### Git Workflow

#### Branch Strategy
- **Development Branch:** `claude/claude-md-mi47dbk11n54regr-01PSZjVh1vaM6R1fDzdJM3nG`
- All development work should be done on this branch
- Branch naming convention: `claude/` prefix with session identifier suffix

#### Commit Guidelines
- Use clear, descriptive commit messages
- Follow conventional commits format:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation changes
  - `refactor:` for code refactoring
  - `test:` for test additions/changes
  - `chore:` for maintenance tasks

#### Push Protocol
```bash
# Always push with upstream tracking
git push -u origin claude/claude-md-mi47dbk11n54regr-01PSZjVh1vaM6R1fDzdJM3nG

# Retry logic on network failures:
# - Retry up to 4 times
# - Exponential backoff: 2s, 4s, 8s, 16s
```

#### Fetch/Pull Strategy
```bash
# Fetch specific branches
git fetch origin <branch-name>

# Pull with branch specification
git pull origin <branch-name>

# Apply same retry logic as push operations
```

### Code Review Process
- Ensure code is tested before committing
- Check for security vulnerabilities (OWASP Top 10)
- Verify no secrets are committed (.env, credentials, etc.)
- Run linters and formatters if configured

---

## Code Conventions

### File Organization
```
Recommended structure (to be established):

src/
├── main/              # Main process code
├── renderer/          # Renderer process code
├── shared/            # Shared utilities and types
├── assets/            # Static assets
└── tests/             # Test files

config/                # Configuration files
docs/                  # Additional documentation
scripts/               # Build and utility scripts
```

### Naming Conventions
- **Files:** kebab-case (e.g., `user-settings.ts`)
- **Classes:** PascalCase (e.g., `UserSettings`)
- **Functions/Variables:** camelCase (e.g., `getUserSettings`)
- **Constants:** UPPER_SNAKE_CASE (e.g., `MAX_RETRY_COUNT`)
- **Interfaces/Types:** PascalCase with 'I' prefix for interfaces (e.g., `IUserConfig`)

### Code Style
- Use consistent indentation (2 or 4 spaces)
- Maximum line length: 100 characters
- Use meaningful variable and function names
- Add comments for complex logic
- Document public APIs with JSDoc/TSDoc

---

## Security Best Practices

### Critical Security Checks
1. **Input Validation:** Always validate and sanitize user inputs
2. **SQL Injection:** Use parameterized queries
3. **XSS Prevention:** Escape output, use Content Security Policy
4. **Command Injection:** Avoid shell command execution with user input
5. **Dependency Security:** Regularly update dependencies
6. **Secrets Management:** Never commit secrets, use environment variables
7. **Authentication:** Implement proper authentication/authorization
8. **HTTPS:** Use secure connections for all network requests

### Files to Never Commit
- `.env`, `.env.local`, `.env.*`
- `credentials.json`, `secrets.json`
- Private keys (`.pem`, `.key`)
- `config.local.*`
- Database files with sensitive data

---

## AI Assistant Guidelines

### Task Execution Protocol

1. **Planning Phase**
   - Use TodoWrite tool for multi-step tasks (3+ steps)
   - Break complex tasks into manageable subtasks
   - Mark tasks as in_progress/completed in real-time

2. **Research Phase**
   - Use Task tool with subagent_type=Explore for codebase exploration
   - Read relevant files before making changes
   - Use Grep/Glob for targeted searches

3. **Implementation Phase**
   - Read files before editing (required for Edit/Write tools)
   - Prefer Edit over Write for existing files
   - Test changes before committing
   - Check for security vulnerabilities

4. **Commit Phase**
   - Run `git status` to review changes
   - Run `git diff` to verify modifications
   - Create descriptive commit messages
   - Only commit when explicitly requested

### Tool Usage Best Practices

**File Operations:**
- **Read:** For viewing file contents
- **Edit:** For modifying existing files
- **Write:** Only for new files
- **Glob:** For finding files by pattern
- **Grep:** For searching file contents

**Code Search:**
- Use Task tool with Explore agent for broad codebase questions
- Use Grep for specific keyword searches
- Use Glob for finding files by name pattern

**Parallel Execution:**
- Run independent operations in parallel (single message, multiple tools)
- Run dependent operations sequentially
- Never use placeholders or guess parameters

**Communication:**
- Output text directly to user (not via echo/bash)
- Reference code with `file_path:line_number` format
- Use markdown for formatting
- Keep responses concise and actionable

### Common Patterns

**Exploring Codebase:**
```
User: "Where are errors handled?"
Assistant: [Uses Task tool with subagent_type=Explore]
```

**Reading Multiple Files:**
```
User: "Check the config files"
Assistant: [Reads multiple files in parallel in one message]
```

**Making Changes:**
```
1. Read file(s)
2. Plan changes with TodoWrite
3. Edit file(s)
4. Verify changes
5. Commit when requested
```

---

## Development Environment

### Prerequisites
*To be established as project develops:*
- Node.js version
- Package manager (npm/yarn/pnpm)
- Build tools
- Testing framework

### Setup Instructions
*To be documented:*
```bash
# Clone repository
git clone <repo-url>
cd Claude-Desktop

# Install dependencies
# npm install

# Run development server
# npm run dev

# Build for production
# npm run build
```

### Environment Variables
*To be documented as needed:*
```
EXAMPLE_API_KEY=your_api_key_here
EXAMPLE_ENV=development
```

---

## Testing Strategy

### Test Types
*To be implemented:*
- **Unit Tests:** Test individual functions/components
- **Integration Tests:** Test component interactions
- **E2E Tests:** Test complete user workflows
- **Security Tests:** Test for vulnerabilities

### Running Tests
```bash
# To be established
# npm test
# npm run test:unit
# npm run test:integration
# npm run test:e2e
```

---

## Build and Deployment

### Build Process
*To be documented:*
```bash
# Development build
# npm run build:dev

# Production build
# npm run build:prod

# Platform-specific builds
# npm run build:mac
# npm run build:windows
# npm run build:linux
```

### Release Process
1. Update version in package.json
2. Update CHANGELOG.md
3. Run full test suite
4. Build for all platforms
5. Create git tag
6. Push to remote
7. Create GitHub release

---

## Project Dependencies

### Current Dependencies
*None yet - to be added as project develops*

### Dependency Management
- Keep dependencies up to date
- Review security advisories
- Use lock files (package-lock.json/yarn.lock)
- Audit dependencies regularly: `npm audit`

---

## Documentation Standards

### Code Documentation
- Use JSDoc/TSDoc for functions and classes
- Document complex algorithms
- Include usage examples
- Document edge cases and assumptions

### README Updates
- Keep README.md synchronized with project state
- Include setup instructions
- Document features and usage
- Add screenshots/demos when available

### CLAUDE.md Maintenance
- Update this file as conventions change
- Document new patterns and practices
- Add project-specific guidelines
- Keep structure section current

---

## Common Tasks Reference

### For AI Assistants Working on This Repo

**Adding a New Feature:**
1. Create TodoWrite plan
2. Research existing code patterns
3. Implement feature following conventions
4. Add tests
5. Update documentation
6. Commit with descriptive message

**Fixing a Bug:**
1. Reproduce the issue
2. Locate the problematic code
3. Write test to verify fix
4. Implement fix
5. Verify test passes
6. Commit with fix: prefix

**Refactoring Code:**
1. Understand current implementation
2. Plan refactoring approach
3. Ensure tests exist
4. Refactor incrementally
5. Verify tests still pass
6. Commit with refactor: prefix

**Updating Documentation:**
1. Identify outdated sections
2. Research current state
3. Update documentation
4. Verify accuracy
5. Commit with docs: prefix

---

## Troubleshooting

### Common Issues

**Git Push Fails (403 Error):**
- Verify branch name starts with `claude/`
- Verify branch name ends with session ID
- Check network connection
- Retry with exponential backoff

**Merge Conflicts:**
1. Fetch latest changes
2. Review conflicting files
3. Resolve conflicts manually
4. Test after resolution
5. Commit resolution

**Build Failures:**
- Check dependency installation
- Verify environment variables
- Review error logs
- Check for breaking changes

---

## Project Roadmap

### Phase 1: Foundation (Current)
- [ ] Establish project structure
- [ ] Set up build system
- [ ] Configure development environment
- [ ] Implement basic UI framework

### Phase 2: Core Features
- [ ] User authentication
- [ ] Chat interface
- [ ] Settings management
- [ ] API integration

### Phase 3: Advanced Features
- [ ] Conversation history
- [ ] File handling
- [ ] Plugin system
- [ ] Themes/customization

### Phase 4: Polish
- [ ] Performance optimization
- [ ] Comprehensive testing
- [ ] Documentation completion
- [ ] Production release

---

## Resources

### Helpful Links
- [Git Best Practices](https://git-scm.com/book/en/v2)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Semantic Versioning](https://semver.org/)

### Contact
- **Repository Owner:** milhy545
- **Issues:** Use GitHub Issues for bug reports and feature requests

---

## Changelog

### 2025-11-18
- Initial CLAUDE.md creation
- Established development workflows
- Documented AI assistant guidelines
- Set up project structure recommendations

---

**Last Updated:** 2025-11-18
**Document Version:** 1.0.0
**Maintained By:** AI Assistants working on this codebase

---

## Notes for AI Assistants

- **Always read files before editing them**
- **Use TodoWrite for complex tasks**
- **Commit only when explicitly requested**
- **Keep this document updated as the project evolves**
- **Follow security best practices rigorously**
- **Test before committing**
- **Use parallel tool calls when possible**
- **Be concise and actionable in responses**
- **Never commit secrets or credentials**
- **Reference code with file:line format**

This is a living document. As the project grows and patterns emerge, update this guide to reflect the actual state and conventions of the codebase.
