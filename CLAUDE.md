# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

LibreChat Desktop is a cross-platform native desktop wrapper for LibreChat web UI built with Tauri v2. It provides native desktop features like global hotkeys, system tray integration, drag & drop support, offline caching, and quick capture overlay for seamless AI-powered desktop workflow integration.

## Architecture

**Backend**: Rust 1.75+ with Tauri v2.x framework
**Frontend**: TypeScript/React with Vite build system
**Target Platforms**: macOS (primary), Windows 10+, Linux (Ubuntu 20.04+)
**Performance Goals**: <3s startup, <100ms UI interactions, <50MB package size

## Project Structure

```
src/                     # Frontend source (TypeScript/React)
src-tauri/              # Rust backend source
├── src/                # Rust source files
├── Cargo.toml          # Rust dependencies
└── tauri.conf.json     # Tauri configuration
tests/                  # Test files
├── contract/           # API contract tests
├── integration/        # Integration tests
└── unit/              # Unit tests
package.json           # Frontend dependencies
```

## Key Dependencies

**Rust Backend**:
- `tauri` v2.x - Application framework
- `tokio` - Async runtime
- `serde` - Serialization
- `reqwest` - HTTP client
- `sqlx` - Database ORM with SQLite
- `tauri-plugin-store` - Local storage
- `tauri-plugin-global-shortcut` - Global hotkeys
- `tauri-plugin-system-tray` - System tray integration

**Frontend**:
- `react` + `react-dom` - UI framework
- `@tanstack/react-query` - Data fetching
- `zustand` - State management
- `axios` - HTTP client
- `@radix-ui/*` - UI components
- `tailwindcss` - Styling
- `framer-motion` - Animations

## Development Commands

```bash
# Start development server
npm run tauri dev

# Build for production
npm run tauri build

# Run tests
cargo test                    # Rust tests
npm run test                  # Frontend tests

# Code quality
cargo fmt && cargo clippy     # Rust formatting/linting
npm run format && npm run lint # Frontend formatting/linting

# Install dependencies
npm install                   # Frontend deps
cd src-tauri && cargo build   # Rust deps
```

## Core Features to Implement

### Primary Features
- **LibreChat Integration**: Connect to LibreChat API without server modifications
- **System Tray**: Quick access menu and notifications
- **Global Hotkeys**: Configurable shortcuts to summon app from anywhere
- **Quick Capture**: Floating mini-window for rapid AI queries
- **Drag & Drop**: File, screenshot, and text snippet handling
- **Offline Caching**: Encrypted local storage of conversations (SQLite)

### UI Features
- **Tabbed Interface**: Multiple conversations like browser tabs
- **Split View**: Side-by-side conversation comparison
- **Pin/Star Chats**: Priority conversation management
- **Rich Previews**: Inline code, image, table, and chart rendering
- **Multi-window**: Standard, docking, and floating modes

### Technical Features
- **Authentication**: JWT-based auth with OAuth/LDAP support
- **Encryption**: OS keychain-managed encryption for cached data
- **Cross-platform**: Native features optimized per platform
- **Performance**: Memory usage <500MB, fast startup/interactions

## Testing Strategy

**Test-Driven Development**: Write tests before implementation
- **Contract Tests**: API endpoint validation (auth, conversations, messages, files)
- **Unit Tests**: Individual component testing (Rust + React)
- **Integration Tests**: End-to-end user scenarios
- **Performance Tests**: Startup time, memory usage, package size validation

## API Integration

LibreChat REST API endpoints to integrate:
- `/api/auth` - Authentication
- `/api/config` - Configuration
- `/api/convos` - Conversations
- `/api/messages` - Messages
- `/api/files` - File uploads
- `/api/mcp` - MCP server integration

## Security Considerations

- Encrypted local storage using OS keychain
- HTTPS enforcement with HTTP warnings
- Secure credential storage
- Input validation and sanitization
- Minimal privilege principle for system access

## Performance Requirements

- **Startup Time**: <3 seconds
- **UI Responsiveness**: <100ms for interactions
- **API Response**: <200ms for LibreChat calls
- **Package Size**: <50MB total
- **Memory Usage**: <500MB during normal operation

## Development Workflow

1. **Setup**: Follow quickstart.md for environment setup
2. **TDD Approach**: Write tests → implement → refactor
3. **Code Quality**: Run linting/formatting before commits
4. **Testing**: Validate all functionality before PR
5. **Documentation**: Update relevant docs with changes

## Configuration Files

- `src-tauri/tauri.conf.json` - Tauri app configuration
- `vite.config.ts` - Frontend build configuration
- `tailwind.config.js` - Styling configuration
- `Cargo.toml` - Rust dependencies and metadata
- `package.json` - Frontend dependencies and scripts

## Key Implementation Notes

- No LibreChat server modifications allowed
- Maintain cross-platform compatibility
- Prioritize macOS native features
- Use encrypted storage for all cached data
- Implement graceful offline/online state handling
- Follow Tauri security best practices
- Keep bundle size optimized for quick downloads

## Troubleshooting

- **Build Issues**: Ensure Rust 1.75+, Node.js 18+, and platform prerequisites installed
- **Dev Server**: Use `npm run tauri dev` for hot reload
- **Platform Specific**: Check Tauri docs for macOS/Windows/Linux requirements
- **Performance**: Use Tauri bundle analyzer and memory profiling tools