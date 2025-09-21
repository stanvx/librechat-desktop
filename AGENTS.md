# Repository Guidelines

## Project Structure & Module Organization
LibreChat Desktop is currently driven by Spec Kit material in `specs/001-build-an-application/`; keep plan, contracts, and tasks up to date before branching. When the Tauri scaffold is in place, store the Vite + React UI under `src/`, Rust commands under `src-tauri/src/`, and runtime configuration in `src-tauri/tauri.conf.json`. Frontend assets belong in `src/assets/`, shared TypeScript utilities in `src/lib/`, and Rust helpers in `src-tauri/src/lib/`. Co-locate tests with the code they exercise (`Component.test.tsx`, `mod.rs` unit blocks) and reserve `src-tauri/tests/` for integration suites.

## Build, Test, and Development Commands
Use `npm run tauri dev` for the full desktop experience; pair `npm run dev` (frontend) with `cargo tauri dev` only when isolating layers. `npm run build` produces the web bundle consumed by Tauri, while `npm run tauri build` emits installers in `src-tauri/target/release/bundle/`. Run `npm run lint`, `npm run format`, `cargo fmt`, and `cargo clippy` before opening a PR to keep both toolchains clean.

## Coding Style & Naming Conventions
TypeScript follows Prettier defaults (2-space indent, single quotes, trailing commas) enforced via ESLint; name React components and files in PascalCase, hooks in camelCase, and shared constants in SCREAMING_SNAKE_CASE. Mirror OpenAPI schema names when creating DTOs. Rust code adheres to rustfmt, snake_case modules and functions, CamelCase types, and errs on explicit `Result` returns for Tauri commands.

## Testing Guidelines
Vitest with Testing Library covers UI logic; add `*.test.tsx` or `*.test.ts` files beside the code and run `npm run test` (append `--watch` during development). Rust logic relies on `cargo test` with `tokio::test` for async cases; keep integration flows in `src-tauri/tests/` and match contract expectations from `specs/001-build-an-application/contracts/`. Target meaningful coverage on data transformations and LibreChat API adapters, and document new fixtures in the spec.

## Commit & Pull Request Guidelines
Commits follow Conventional Commits (`feat`, `fix`, `docs`, `chore`) as seen in history; reference the related Spec Kit task or spec section in the body. Keep PRs focused, describe user-visible changes, list validation commands, and attach screenshots or recordings for UI updates. Confirm specs remain accurate, ensure linters/tests pass locally, and request review once CI succeeds.

## Security & Configuration Tips
Never commit secrets; store local overrides in `.env` with defaults such as `VITE_DEFAULT_SERVER_URL`. Review `tauri.conf.json` before release to confirm only required API scopes are enabled and file system access is scoped. Validate desktop bundles on macOS, Windows, and Linux sandboxes prior to tagging a release.
