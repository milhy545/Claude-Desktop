# Repository Guidelines

Contributor quickstart for Claude Desktop (Tauri edition). Keep changes small, well-tested, and aligned with the structure below.

## Project Structure & Module Organization
- `src/`: Web UI (single-page `index.html`, logic in `js/app.js` + `js/voice.js`, styling in `styles/main.css`). Keep UI assets here.
- `src-tauri/`: Rust backend and Tauri config (`Cargo.toml`, `tauri.conf.json`, `src/` modules, MCP logic under `src/mcp`). Tests live in `src-tauri/src/mcp/tests.rs`.
- `docs/` and root markdown files: user-facing documentation; update when behavior changes. Build artifacts land in `src-tauri/target/`.

## Build, Test, and Development Commands
- `npm install`: install frontend + Tauri CLI deps (needs Rust toolchain present).
- `npm run dev`: start Tauri dev server with live reload.
- `npm run build`: full release bundle; use `build:deb`, `build:appimage`, or `build:rpm` for specific targets. Outputs to `src-tauri/target/release/bundle/`.
- `npm run tauri info` (optional): confirm toolchain health.
- `cargo test` (inside `src-tauri` or via `npm run tauri test` if added): run Rust unit tests, including MCP config parsing.

## Coding Style & Naming Conventions
- JavaScript/HTML: 4-space indent, prefer `const`/`let`, async/await for Tauri invocations, keep DOM IDs stable (`chatTab`, `mcpServerList`, etc.). Avoid inline scripts beyond initialization.
- Rust: Rust 2021, run `cargo fmt` before commits; small modules per feature (e.g., MCP helpers). Use `Result<T>` returns and `?` for propagation.
- CSS: leverage existing custom properties in `styles/main.css`; keep dark palette unless feature requires change.
- Naming: follow conventional commits (`feat:`, `fix:`, `chore:`, `style:`) seen in history.

## Testing Guidelines
- Rust: `cargo test` must pass; add focused unit tests under `src-tauri/src/**/` mirroring module structure.
- UI: no automated UI tests yet; smoke-test via `npm run dev` for tab switching, auth button states, MCP list rendering, and voice controls.
- When adding features touching auth or MCP, validate config parsing with malformed JSON and missing fields.

## Commit & Pull Request Guidelines
- Commits: concise scope, conventional prefix, formatted code. Include why the change is needed if not obvious.
- PRs: describe behavior change, link issues, list manual test steps, and add screenshots/GIFs for UI tweaks (tabs, modals, voice buttons). Note platform tested (e.g., Ubuntu 22.04).
- Keep PRs small; highlight any migration steps (config changes, new env requirements).

## Security & Configuration Tips
- User config: `~/.config/Claude/claude_desktop_config.json`; session data in `~/.claude/`. Never commit real configs or tokens.
- MCP servers often run via `npx`; warn reviewers when new external binaries are invoked. Document required permissions and paths in PR descriptions.
