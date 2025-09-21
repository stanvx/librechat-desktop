# Repository Guidelines

## Project Structure & Module Organization
LibreChat Desktop mirrors the Spec Kit in `specs/001-build-an-application/`; update plans, contracts, and tasks before branching. Host the React UI under `src/`, shared TypeScript utilities in `src/lib/`, and static assets in `src/assets/`. Tauri lives in `src-tauri/` with runtime config in `src-tauri/tauri.conf.json`, Rust commands in `src-tauri/src/`, shared helpers in `src-tauri/src/lib/`, and desktop integrations under `src-tauri/tests/` for integration cases. Co-locate Vitest suites next to the code (`Component.test.tsx`) and Rust unit tests inside the relevant `mod.rs` files.

## Build, Test, and Development Commands
Run `npm run tauri dev` for the full desktop shell, or pair `npm run dev` with `cargo tauri dev` when isolating frontend and Rust layers. `npm run build` prepares the Vite bundle consumed by Tauri, while `npm run tauri build` emits installers in `src-tauri/target/release/bundle/`. Keep both toolchains clean with `npm run lint`, `npm run format`, `cargo fmt`, and `cargo clippy`.

## Coding Style & Naming Conventions
TypeScript follows Prettier defaults: 2-space indent, single quotes, trailing commas. Use PascalCase for components and files, camelCase for hooks, and SCREAMING_SNAKE_CASE for shared constants. Mirror OpenAPI schema names when defining DTOs. Rust modules and functions stay snake_case, types CamelCase, with explicit `Result` returns for Tauri commands.

## Testing Guidelines
Use Vitest with Testing Library for UI logic via `npm run test` (append `--watch` during iterations). Rust code relies on `cargo test`, reserving `tokio::test` for async flows. Target meaningful coverage on data transforms and LibreChat API adapters, documenting new fixtures in the spec folder.

## Commit & Pull Request Guidelines
Commits use Conventional Commits (`feat`, `fix`, `docs`, `chore`) and reference the relevant Spec Kit task or section. PRs should describe user-visible changes, list validation commands, and include screenshots or recordings for UI updates. Confirm specs stay accurate and ensure linters/tests pass before requesting review.

## Security & Configuration Tips
Never commit secrets; local overrides live in `.env` (e.g., `VITE_DEFAULT_SERVER_URL`). Review `src-tauri/tauri.conf.json` before release to ensure only required API scopes and file access are enabled. Validate desktop bundles on macOS, Windows, and Linux sandboxes prior to tagging a release.
