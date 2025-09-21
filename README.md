# LibreChat Desktop

A cross-platform native desktop wrapper for LibreChat web UI built with Tauri v2. Provides native desktop features like global hotkeys, system tray integration, drag & drop support, offline caching, and quick capture overlay for seamless AI-powered desktop workflow integration.

## 🏗️ Project Methodology

This project follows the [Spec Kit](https://github.com/github/spec-kit) methodology for structured software development:

- **📋 Specification-First**: Complete feature specifications before implementation
- **🧪 Test-Driven Development**: Contract tests → Implementation → Validation
- **📚 Comprehensive Documentation**: Research, data models, API contracts, and quickstart guides
- **⚡ Task-Driven Execution**: 75 structured tasks with clear dependencies and parallel execution opportunities

## 📁 Project Structure

```
specs/001-build-an-application/     # Complete feature specification
├── plan.md                         # Implementation plan & tech stack
├── research.md                     # Technical research & decisions  
├── data-model.md                   # 8 core entities & database schema
├── contracts/                      # OpenAPI specs for LibreChat integration
│   ├── auth-api.yaml              # Authentication endpoints
│   ├── conversations-api.yaml     # Conversation management
│   ├── messages-api.yaml          # Message operations
│   └── files-api.yaml             # File upload/management
├── quickstart.md                   # Development setup guide
└── tasks.md                        # 75 structured implementation tasks
```

## 🚀 Tech Stack

- **Backend**: Rust 1.75+ with Tauri v2.x framework
- **Frontend**: TypeScript/React with Vite build system  
- **Storage**: SQLite with encryption via Tauri store plugin
- **Target Platforms**: macOS (primary), Windows 10+, Linux (Ubuntu 20.04+)

## 🎯 Key Features

- **LibreChat Integration**: Connect to LibreChat API without server modifications
- **System Tray**: Quick access menu and notifications
- **Global Hotkeys**: Configurable shortcuts to summon app from anywhere
- **Quick Capture**: Floating mini-window for rapid AI queries
- **Drag & Drop**: File, screenshot, and text snippet handling
- **Offline Caching**: Encrypted local storage of conversations

## 🛠️ Development

See [`specs/001-build-an-application/quickstart.md`](specs/001-build-an-application/quickstart.md) for detailed setup instructions.

## 📖 Spec Kit Integration

This project demonstrates Spec Kit principles:
1. **Specification completeness** before coding
2. **Contract-driven development** with failing tests first
3. **Structured task execution** with clear dependencies
4. **Documentation-first approach** for long-term maintainability
