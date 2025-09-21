# LibreChat Desktop Application - Quickstart Guide

## Overview
This guide provides step-by-step instructions for setting up the LibreChat Desktop development environment and running the application locally.

## Prerequisites

### System Requirements
- **Rust**: 1.75+ (for Tauri backend)
- **Node.js**: 18+ (for frontend)
- **Operating System**: macOS 11+, Windows 10+, or Linux (Ubuntu 20.04+)

### Development Tools
- **IDE**: VS Code with Rust Analyzer and Tauri extensions
- **Git**: For version control
- **Cargo**: Rust package manager (comes with Rust)
- **npm/yarn**: Node.js package manager

### Tauri Prerequisites

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Windows
```powershell
# Install Visual Studio C++ Build Tools
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/

# Install Rust
# Download from: https://rustup.rs/

# Install WebView2 (usually pre-installed on Windows 11)
# Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

#### Linux (Ubuntu/Debian)
```bash
# Install dependencies
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Project Setup

### 1. Initialize Project Structure
```bash
# Create project directory
mkdir librechat-desktop
cd librechat-desktop

# Initialize Tauri project
npm create tauri-app@latest . --template vanilla-ts
# or
cargo install create-tauri-app
cargo create-tauri-app --template vanilla-ts

# Project structure should look like:
# ├── src/                    # Frontend source (TypeScript/HTML/CSS)
# ├── src-tauri/             # Rust backend source
# │   ├── src/               # Rust source files
# │   ├── Cargo.toml         # Rust dependencies
# │   └── tauri.conf.json    # Tauri configuration
# ├── package.json           # Frontend dependencies
# └── README.md
```

### 2. Install Dependencies
```bash
# Install frontend dependencies
npm install

# Install additional dependencies for LibreChat integration
npm install -D @types/node

# Add React and related dependencies
npm install react react-dom @types/react @types/react-dom
npm install -D vite @vitejs/plugin-react

# Add state management and HTTP client
npm install zustand @tanstack/react-query axios

# Add UI components and styling
npm install @radix-ui/react-dialog @radix-ui/react-dropdown-menu
npm install tailwindcss @tailwindcss/typography
npm install framer-motion

# Install Rust dependencies (in src-tauri directory)
cd src-tauri
cargo add tauri-plugin-store tauri-plugin-global-shortcut tauri-plugin-system-tray
cargo add tokio serde reqwest sqlx
cargo add --features=["runtime-tokio-rustls", "sqlite"] sqlx
cd ..
```

### 3. Configure Tauri
Edit `src-tauri/tauri.conf.json`:
```json
{
  "productName": "LibreChat Desktop",
  "identifier": "com.librechat.desktop",
  "version": "0.1.0",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420"
  },
  "app": {
    "windows": [
      {
        "title": "LibreChat Desktop",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "fullscreen": false,
        "resizable": true,
        "maximized": false,
        "visible": true,
        "decorations": true,
        "alwaysOnTop": false,
        "skipTaskbar": false
      }
    ],
    "systemTray": {
      "iconPath": "icons/icon.png",
      "iconAsTemplate": true,
      "menuOnLeftClick": false
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "category": "Productivity",
    "copyright": "Copyright © 2025 LibreChat Desktop",
    "shortDescription": "Desktop wrapper for LibreChat AI platform",
    "longDescription": "A cross-platform desktop application that wraps LibreChat web interface with native features like global hotkeys, system tray integration, and offline caching."
  },
  "security": {
    "csp": null
  },
  "updater": {
    "active": false
  },
  "allowlist": {
    "all": false,
    "shell": {
      "all": false,
      "execute": false,
      "sidecar": false
    },
    "dialog": {
      "all": false,
      "ask": true,
      "confirm": true,
      "message": true,
      "open": true,
      "save": true
    },
    "fs": {
      "all": false,
      "readFile": true,
      "writeFile": true,
      "createDir": true,
      "removeDir": true,
      "removeFile": true,
      "renameFile": true,
      "exists": true
    },
    "window": {
      "all": false,
      "close": true,
      "hide": true,
      "maximize": true,
      "minimize": true,
      "setIcon": true,
      "setTitle": true,
      "show": true,
      "unmaximize": true,
      "unminimize": true
    },
    "globalShortcut": {
      "all": true
    },
    "systemTray": {
      "all": true
    },
    "clipboard": {
      "all": true
    }
  }
}
```

### 4. Set up Development Environment

#### Frontend Configuration (vite.config.ts)
```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"]
    }
  },
  envPrefix: ['VITE_', 'TAURI_ENV_*'],
})
```

#### Tailwind CSS Setup
```bash
# Initialize Tailwind
npx tailwindcss init -p

# Configure tailwind.config.js
cat > tailwind.config.js << EOF
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
EOF

# Add Tailwind to CSS
cat > src/styles.css << EOF
@tailwind base;
@tailwind components;
@tailwind utilities;
EOF
```

### 5. Basic Application Structure

#### Frontend Entry Point (src/main.tsx)
```typescript
import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './styles.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
```

#### Basic App Component (src/App.tsx)
```typescript
import React, { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

function App() {
  const [connected, setConnected] = useState(false)
  const [serverUrl, setServerUrl] = useState('http://localhost:3080')

  const testConnection = async () => {
    try {
      const result = await invoke('test_librechat_connection', { url: serverUrl })
      setConnected(result as boolean)
    } catch (error) {
      console.error('Connection test failed:', error)
      setConnected(false)
    }
  }

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">LibreChat Desktop</h1>
      
      <div className="mb-4">
        <input
          type="text"
          value={serverUrl}
          onChange={(e) => setServerUrl(e.target.value)}
          placeholder="LibreChat Server URL"
          className="border rounded px-3 py-2 w-full"
        />
        <button
          onClick={testConnection}
          className="mt-2 bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
        >
          Test Connection
        </button>
      </div>

      <div className={`p-3 rounded ${connected ? 'bg-green-100' : 'bg-red-100'}`}>
        Status: {connected ? 'Connected' : 'Disconnected'}
      </div>
    </div>
  )
}

export default App
```

#### Basic Rust Backend (src-tauri/src/main.rs)
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
async fn test_librechat_connection(url: String) -> Result<bool, String> {
    let client = reqwest::Client::new();
    
    match client.get(&format!("{}/api/config", url))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await 
    {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Ok(false),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_system_tray::Builder::new().build())
        .invoke_handler(tauri::generate_handler![test_librechat_connection])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Development Workflow

### 1. Start Development Server
```bash
# Start in development mode
npm run tauri dev

# Alternative: start frontend and backend separately
# Terminal 1: Start frontend dev server
npm run dev

# Terminal 2: Start Tauri development
npm run tauri dev
```

### 2. Build for Production
```bash
# Build the application
npm run tauri build

# Output will be in src-tauri/target/release/bundle/
```

### 3. Testing Setup
```bash
# Add testing dependencies
npm install -D vitest @testing-library/react @testing-library/jest-dom
npm install -D jsdom happy-dom

# Add Rust testing framework
cd src-tauri
cargo add --dev tokio-test
```

## Environment Configuration

### 1. Environment Variables
Create `.env` file:
```env
# Development
VITE_DEFAULT_SERVER_URL=http://localhost:3080
VITE_APP_NAME=LibreChat Desktop
VITE_APP_VERSION=0.1.0

# Tauri specific
TAURI_ENV_DEBUG=true
```

### 2. LibreChat Server Setup
Ensure you have a LibreChat server running:
```bash
# Using Docker (recommended for development)
git clone https://github.com/danny-avila/LibreChat.git
cd LibreChat
docker-compose up -d

# Server will be available at http://localhost:3080
```

## Development Tips

### Hot Reload
- Frontend changes trigger automatic reload
- Rust changes require manual restart of dev server
- Use `cargo-watch` for automatic Rust rebuilds:
```bash
cargo install cargo-watch
# Then in src-tauri directory:
cargo watch -x run
```

### Debugging
- Frontend: Use browser dev tools (auto-opened in dev mode)
- Rust: Use `println!` or proper logging with `log` crate
- Enable Tauri developer tools in development builds

### Code Style
```bash
# Frontend formatting
npm run format  # Prettier
npm run lint    # ESLint

# Rust formatting
cd src-tauri
cargo fmt
cargo clippy
```

## Next Steps

1. **Set up LibreChat API Integration**: Implement authentication flow
2. **Add System Tray**: Implement tray menu and notifications
3. **Global Hotkeys**: Set up keyboard shortcuts
4. **Offline Storage**: Implement SQLite database for caching
5. **File Upload**: Add drag & drop functionality
6. **Multi-window Support**: Implement quick capture overlay

## Troubleshooting

### Common Issues

1. **Rust compilation errors**: Ensure Rust toolchain is up to date
2. **Frontend build fails**: Check Node.js version and dependencies
3. **Tauri dev server won't start**: Verify all prerequisites are installed
4. **Permission errors**: Check system permissions for file access

### Platform-Specific Issues

#### macOS
- May need to approve app in Security & Privacy settings
- Ensure Xcode Command Line Tools are installed

#### Windows
- Install Visual Studio Build Tools if compilation fails
- WebView2 runtime required for app to run

#### Linux
- Install all webkit and development dependencies
- May need to install additional system libraries

For detailed troubleshooting, see the [Tauri documentation](https://tauri.app/guides/debugging/).

## Resources

- [Tauri Documentation](https://tauri.app/)
- [LibreChat Repository](https://github.com/danny-avila/LibreChat)
- [React Documentation](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tailwind CSS](https://tailwindcss.com/)