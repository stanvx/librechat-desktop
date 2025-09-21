# Tasks: LibreChat Desktop Application

**Input**: Design documents from `/Users/trentstanton/Dev/librechat-desktop/specs/001-build-an-application/`
**Prerequisites**: plan.md (✓), research.md (✓), data-model.md (✓), contracts/ (✓), quickstart.md (✓)

## Feature Overview
Cross-platform native desktop wrapper for LibreChat web UI using Tauri v2, prioritizing macOS with features like global hotkeys, system tray integration, drag & drop support, offline caching, tabbed interface, and quick capture overlay.

## Tech Stack Summary
- **Backend**: Rust 1.75+ with Tauri v2.x framework
- **Frontend**: TypeScript/React with Vite build system
- **Storage**: SQLite with encryption via Tauri store plugin
- **Target Platforms**: macOS (primary), Windows 10+, Linux (Ubuntu 20.04+)

## Path Conventions
- **Project Structure**: Single Tauri project
- **Rust Backend**: `src-tauri/src/` 
- **Frontend**: `src/`
- **Tests**: `tests/contract/`, `tests/integration/`, `tests/unit/`
- **Configuration**: `src-tauri/tauri.conf.json`, `package.json`

## Phase 3.1: Setup
- [X] T001 Create Tauri project structure with React frontend
- [X] T002 Initialize Rust dependencies (tokio, serde, reqwest, sqlx, tauri-plugins)
- [X] T003 [P] Configure frontend dependencies (React, TypeScript, Tailwind, Zustand, React Query)
- [X] T004 [P] Configure development environment (Vite, ESLint, Prettier, Rust clippy)
- [X] T005 [P] Set up Tauri configuration in src-tauri/tauri.conf.json

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Contract Tests
- [X] T006 [P] Contract test auth API endpoints in src-tauri/tests/contract/test_auth_api.rs
- [X] T007 [P] Contract test conversations API endpoints in src-tauri/tests/contract/test_conversations_api.rs  
- [X] T008 [P] Contract test messages API endpoints in src-tauri/tests/contract/test_messages_api.rs
- [X] T009 [P] Contract test files API endpoints in src-tauri/tests/contract/test_files_api.rs

### Integration Tests
- [X] T010 [P] Integration test LibreChat connection in src-tauri/tests/integration/test_connection.rs
- [X] T011 [P] Integration test authentication flow in src-tauri/tests/integration/test_auth_flow.rs
- [X] T012 [P] Integration test file upload/drag & drop in src-tauri/tests/integration/test_file_handling.rs
- [X] T013 [P] Integration test system tray functionality in src-tauri/tests/integration/test_system_tray.rs
- [X] T014 [P] Integration test global hotkeys in src-tauri/tests/integration/test_global_hotkeys.rs
- [X] T015 [P] Integration test quick capture overlay in src-tauri/tests/integration/test_quick_capture.rs
- [X] T016 [P] Integration test offline caching in src-tauri/tests/integration/test_offline_cache.rs
- [X] T017 [P] Integration test multi-window support in src-tauri/tests/integration/test_multi_window.rs

## Phase 3.3: Core Models (ONLY after tests are failing)
- [X] T018 [P] Conversation model in src-tauri/src/models/conversation.rs
- [X] T019 [P] Message model in src-tauri/src/models/message.rs
- [X] T020 [P] EncryptedCache model in src-tauri/src/models/encrypted_cache.rs
- [X] T021 [P] MessageQueue model in src-tauri/src/models/message_queue.rs
- [X] T022 [P] UserPreferences model in src-tauri/src/models/user_preferences.rs
- [X] T023 [P] ServerConfiguration model in src-tauri/src/models/server_configuration.rs
- [X] T024 [P] DroppedFile model in src-tauri/src/models/dropped_file.rs
- [X] T025 [P] QuickCapture model in src-tauri/src/models/quick_capture.rs

## Phase 3.4: Database Setup
- [X] T026 SQLite database initialization in src-tauri/src/database/mod.rs
- [X] T027 Database migrations for all models in src-tauri/src/database/migrations.rs
- [X] T028 [P] Encryption setup with OS keychain integration in src-tauri/src/database/encryption.rs

## Phase 3.5: API Client Services
- [X] T029 [P] LibreChat API client base in src-tauri/src/services/api_client.rs
- [X] T030 [P] Authentication service in src-tauri/src/services/auth_service.rs
- [X] T031 [P] Conversations service in src-tauri/src/services/conversations_service.rs
- [X] T032 [P] Messages service in src-tauri/src/services/messages_service.rs
- [X] T033 [P] Files service in src-tauri/src/services/files_service.rs

## Phase 3.6: Core Backend Services
- [X] T034 [P] Offline storage service in src-tauri/src/services/storage_service.rs
- [X] T035 [P] Sync service for online/offline state in src-tauri/src/services/sync_service.rs
- [X] T036 [P] Cache management service in src-tauri/src/services/cache_service.rs
- [X] T037 [P] Preferences service in src-tauri/src/services/preferences_service.rs

## Phase 3.7: Tauri Commands (Backend-Frontend Bridge)
- [ ] T038 Authentication commands (login, logout, refresh) in src-tauri/src/commands/auth.rs
- [ ] T039 Conversation commands (CRUD operations) in src-tauri/src/commands/conversations.rs
- [ ] T040 Message commands (send, edit, delete) in src-tauri/src/commands/messages.rs
- [ ] T041 File commands (upload, download, delete) in src-tauri/src/commands/files.rs
- [ ] T042 [P] System commands (tray, hotkeys, windows) in src-tauri/src/commands/system.rs
- [ ] T043 [P] Preferences commands in src-tauri/src/commands/preferences.rs

## Phase 3.8: Frontend Core Components
- [ ] T044 [P] Authentication components in src/components/auth/
- [ ] T045 [P] Conversation list component in src/components/conversations/ConversationList.tsx
- [ ] T046 [P] Message display component in src/components/messages/MessageDisplay.tsx
- [ ] T047 [P] Chat input component in src/components/chat/ChatInput.tsx
- [ ] T048 [P] File upload component in src/components/files/FileUpload.tsx

## Phase 3.9: Frontend State Management
- [ ] T049 [P] Authentication store in src/stores/authStore.ts
- [ ] T050 [P] Conversations store in src/stores/conversationsStore.ts  
- [ ] T051 [P] Messages store in src/stores/messagesStore.ts
- [ ] T052 [P] Preferences store in src/stores/preferencesStore.ts
- [ ] T053 [P] UI state store in src/stores/uiStore.ts

## Phase 3.10: Desktop Features
- [ ] T054 System tray implementation in src-tauri/src/system/tray.rs
- [ ] T055 Global hotkeys implementation in src-tauri/src/system/hotkeys.rs
- [ ] T056 Quick capture overlay window in src/components/QuickCapture.tsx
- [ ] T057 Drag & drop handler in src-tauri/src/system/drag_drop.rs
- [ ] T058 [P] Multi-window management in src-tauri/src/system/windows.rs

## Phase 3.11: Main Application Integration
- [ ] T059 Main Tauri application setup in src-tauri/src/main.rs
- [ ] T060 Frontend app integration and routing in src/App.tsx
- [ ] T061 Error handling and logging throughout application
- [ ] T062 Application lifecycle management (startup, shutdown)

## Phase 3.12: End-to-End Integration
- [ ] T063 Connect all authentication flows (frontend ↔ backend ↔ LibreChat API)
- [ ] T064 Connect conversation management (CRUD operations with offline sync)
- [ ] T065 Connect message streaming (real-time message display)
- [ ] T066 Connect file handling (upload, drag & drop, display)
- [ ] T067 Integrate offline/online state management

## Phase 3.13: Polish & Performance
- [ ] T068 [P] Unit tests for models in tests/unit/models/
- [ ] T069 [P] Unit tests for services in tests/unit/services/
- [ ] T070 [P] Unit tests for frontend components in tests/unit/components/
- [ ] T071 Performance optimization (startup time <3s, UI <100ms)
- [ ] T072 Memory usage optimization (<500MB target)
- [ ] T073 Bundle size optimization (<50MB target)
- [ ] T074 [P] Update README.md and documentation
- [ ] T075 Run complete manual testing scenarios from quickstart.md

## Dependencies

### Setup Dependencies
- T001 → T002-T005 (project must exist before configuration)
- T002-T005 can run in parallel

### Test Dependencies  
- T006-T017 can run in parallel (different test files)
- All tests (T006-T017) must complete before any implementation

### Model Dependencies
- T018-T025 can run in parallel (different model files)
- T026 → T027 → T028 (database setup sequence)

### Service Dependencies
- T029-T033 can run in parallel (different API client files)
- T034-T037 can run in parallel (different backend service files)
- T029 (API client base) → T030-T033 (specific API services)

### Command Dependencies
- T038-T043 depend on corresponding services being complete
- T042-T043 can run in parallel (different command files)

### Frontend Dependencies
- T044-T048 can run in parallel (different component directories)
- T049-T053 can run in parallel (different store files)

### Desktop Features Dependencies
- T054-T058 depend on T042 (system commands)
- T058 can run in parallel with T054-T057

### Integration Dependencies
- T059-T062 require all models, services, and commands complete
- T063-T067 require T059-T062 complete

### Polish Dependencies
- T068-T070 can run in parallel (different test directories)
- T071-T073 require integration complete
- T074-T075 can run in parallel

## Parallel Execution Examples

### Phase 3.2 - All Contract Tests Together:
```bash
Task: "Contract test auth API endpoints in src-tauri/tests/contract/test_auth_api.rs"
Task: "Contract test conversations API endpoints in src-tauri/tests/contract/test_conversations_api.rs"  
Task: "Contract test messages API endpoints in src-tauri/tests/contract/test_messages_api.rs"
Task: "Contract test files API endpoints in src-tauri/tests/contract/test_files_api.rs"
```

### Phase 3.3 - All Models Together:
```bash
Task: "Conversation model in src-tauri/src/models/conversation.rs"
Task: "Message model in src-tauri/src/models/message.rs"
Task: "EncryptedCache model in src-tauri/src/models/encrypted_cache.rs"
Task: "MessageQueue model in src-tauri/src/models/message_queue.rs"
Task: "UserPreferences model in src-tauri/src/models/user_preferences.rs"
Task: "ServerConfiguration model in src-tauri/src/models/server_configuration.rs"
Task: "DroppedFile model in src-tauri/src/models/dropped_file.rs"
Task: "QuickCapture model in src-tauri/src/models/quick_capture.rs"
```

### Phase 3.8 - Frontend Components Together:
```bash
Task: "Authentication components in src/components/auth/"
Task: "Conversation list component in src/components/conversations/ConversationList.tsx"
Task: "Message display component in src/components/messages/MessageDisplay.tsx"
Task: "Chat input component in src/components/chat/ChatInput.tsx"
Task: "File upload component in src/components/files/FileUpload.tsx"
```

## Notes
- [P] tasks = different files, no dependencies, can run in parallel
- All contract tests must fail before starting implementation
- Tauri-specific structure: src-tauri/ for Rust backend, src/ for React frontend
- Focus on macOS native features while maintaining cross-platform compatibility
- Maintain <3s startup, <100ms UI interactions, <50MB bundle size targets
- No LibreChat server modifications allowed - API client only

## Validation Checklist
- [x] All 4 contracts have corresponding test tasks (T006-T009)
- [x] All 8 entities have model creation tasks (T018-T025)
- [x] All tests come before implementation (T006-T017 → T018+)
- [x] Parallel tasks truly independent (different files, no shared dependencies)
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] TDD workflow: Tests → Models → Services → Integration → Polish
- [x] Performance targets integrated (T071-T073)
- [x] Desktop features covered (system tray, hotkeys, drag & drop, multi-window)
- [x] API integration complete (auth, conversations, messages, files)