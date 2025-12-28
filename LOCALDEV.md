# Local Development Guide

This guide will help you set up and run Sovereign locally for development.

## Prerequisites

- **Node.js**: 20.19.0+ or 22.12.0+ ([Download](https://nodejs.org/))
- **Rust**: Latest stable version ([Install via rustup](https://rustup.rs/))
- **Tauri CLI**: Will be installed automatically via npm
- **Tesseract OCR**: Required for text extraction

### Install Tesseract OCR

**macOS:**
```bash
brew install tesseract
```

**Windows (for development):**
For local development, you can either:
1. **Install Tesseract manually** (same as production users used to do):
   - Download from [UB-Mannheim](https://github.com/UB-Mannheim/tesseract/wiki)
   - Install and add to PATH
2. **Or use bundled Tesseract** (for testing the bundled experience):
   - Run `.\scripts\setup-tesseract-win.ps1` before building
   - The bundled Tesseract will be used when running the built app

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install tesseract-ocr
```

### System Dependencies

**macOS:**
- Xcode Command Line Tools: `xcode-select --install`

**Windows:**
- Microsoft Visual C++ Build Tools

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev dpkg fakeroot
```

## Setup

### Step 1: Clone the Repository

```bash
git clone https://github.com/lehendo/sovereign.git
cd sovereign
```

### Step 2: Install Node.js Dependencies

```bash
npm install
```

### Step 3: Install Rust Dependencies

Rust dependencies will be installed automatically on first build, but you can also install them manually:

```bash
cd src-tauri
cargo build
cd ..
```

### Step 4: (Optional) Enable Semantic Search

The app works with OCR-only mode by default. For semantic search (finding things by meaning, not just exact text), download the embedding model:

**macOS/Linux:**
```bash
mkdir -p ~/.cache/huggingface/hub/models--Qdrant--all-MiniLM-L6-v2-onnx/snapshots/main
cd ~/.cache/huggingface/hub/models--Qdrant--all-MiniLM-L6-v2-onnx/snapshots/main
curl -L -o model.onnx "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/model.onnx?download=true"
curl -L -o tokenizer.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer.json?download=true"
curl -L -o config.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/config.json?download=true"
curl -L -o special_tokens_map.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/special_tokens_map.json?download=true"
curl -L -o tokenizer_config.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer_config.json?download=true"
```

**Windows (PowerShell):**
```powershell
$cacheDir = "$env:LOCALAPPDATA\huggingface\hub\models--Qdrant--all-MiniLM-L6-v2-onnx\snapshots\main"
New-Item -ItemType Directory -Force -Path $cacheDir | Out-Null
Set-Location $cacheDir
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/model.onnx?download=true" -OutFile "model.onnx"
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer.json?download=true" -OutFile "tokenizer.json"
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/config.json?download=true" -OutFile "config.json"
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/special_tokens_map.json?download=true" -OutFile "special_tokens_map.json"
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer_config.json?download=true" -OutFile "tokenizer_config.json"
```

## Running in Development

**Start the development server:**
```bash
npm run tauri dev
```

This will:
- Start the Vite dev server for the frontend (http://localhost:1420)
- Build and run the Tauri app
- Enable hot-reload for frontend changes (Rust changes require app restart)

The app window will open automatically once the build completes.

## Building for Production

**Windows Build (requires Tesseract bundling):**
Before building for Windows, you must bundle Tesseract OCR:
```powershell
# Run from the repository root
.\scripts\setup-tesseract-win.ps1
```

Then build:
```bash
npm run tauri:build:windows      # Windows (x64)
```

**Build for current platform:**
```bash
npm run tauri build
```

**Build for specific platforms:**
```bash
npm run tauri:build:macos        # macOS Intel (x86_64)
npm run tauri:build:macos-arm    # macOS Apple Silicon (ARM64)
npm run tauri:build:linux        # Linux (x64)
```

**Note:** Windows builds bundle Tesseract automatically. Linux `.deb` packages declare `tesseract-ocr` as a dependency. macOS requires users to install Tesseract via Homebrew.

Built binaries will be in:
- `src-tauri/target/release/bundle/` (native builds)
- `src-tauri/target/<target-triple>/release/bundle/` (cross-compiled builds)

## Project Structure

- **Frontend**: React + TypeScript + Vite, located in `src/`
- **Backend**: Rust, located in `src-tauri/src/`
- **Database**: SQLite database created automatically in app data directory
- **Configuration**: Tauri config in `src-tauri/tauri.conf.json`

## Development Workflow

### Hot Reload

- **Frontend changes**: Automatically reload in the app (Vite HMR)
- **Rust changes**: Require app restart (stop and run `npm run tauri dev` again)

### Debugging

**Frontend:**
- Right-click in the app window → "Inspect" to open DevTools
- Console logs appear in DevTools
- React DevTools can be installed as browser extension

**Backend:**
- Rust logs appear in the terminal where you ran `npm run tauri dev`
- Use `println!()` or `eprintln!()` for debugging
- Check terminal output for errors and debug messages

### Database Location

The SQLite database is created automatically in the app data directory. See [INSTALL.md](INSTALL.md#storage-locations) for exact paths.

## Required Permissions

When running the app in development mode, you'll need to grant the same permissions as the installed app. See [INSTALL.md](INSTALL.md#step-4-launch--permissions) for detailed permission setup instructions for all platforms.

**Note:** On macOS, if running from terminal, you may need to grant permission to "Terminal" instead of "Sovereign" in System Settings.

## Troubleshooting

For common development issues and solutions, see the [Troubleshooting Guide](TROUBLESHOOTING.md). It covers:
- Tesseract OCR issues
- Build errors
- Permission problems
- App launch issues
- Hot reload problems
