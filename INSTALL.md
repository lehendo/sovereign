# Installation Guide

## Quick Install

### Step 1: Download

Download the latest release from [GitHub Releases](https://github.com/lehendo/sovereign/releases/latest):

- **macOS**: Download `.dmg` file → Open → Drag to Applications folder
  - **First Launch**: If you see "cannot be opened because the developer cannot be verified":
    1. Right-click the app in Applications → Select "Open"
    2. Click "Open" in the dialog that appears
    3. Alternatively: System Settings → Privacy & Security → Scroll down → Click "Open Anyway" next to Sovereign
- **Windows**: Download `.msi` installer → Run and follow prompts
  - **First Launch**: If Windows SmartScreen blocks the app:
    1. Click "More info" on the warning screen
    2. Click "Run anyway" button
    3. The installer will proceed normally
- **Linux**: Download `.deb` or `.AppImage` file → Install/run
  - **AppImage**: Make executable with `chmod +x Sovereign_*.AppImage` before running

### Step 2: Install Tesseract OCR

**Required for text extraction:**

**macOS:**
```bash
brew install tesseract
```

**Windows:**
**Tesseract is bundled with the app** - No installation needed! The Windows installer includes Tesseract OCR automatically.

**Linux:**
**Tesseract is auto-installed** - When you install the `.deb` package, Tesseract OCR is automatically installed as a dependency. No manual steps required.

**Note:** If you're using the `.AppImage` on Linux, you'll need to install Tesseract manually:
```bash
sudo apt-get update
sudo apt-get install tesseract-ocr
```

### Step 3: (Optional) Enable Semantic Search

The app works perfectly with OCR-only mode. For semantic search (finding things by meaning, not just exact text), download the embedding model:

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

### Step 4: Launch & Permissions

Run the app and grant required permissions when prompted.

#### macOS

1. **Screen Recording Permission** (Required):
   - Click "Open System Settings" when prompted
   - Go to **System Settings > Privacy & Security > Screen Recording**
   - Enable "Sovereign" (or "Terminal" if running from terminal)
   - Restart the app

#### Windows

1. **Screen Recording Permission** (Required):
   - Windows 10/11 will typically prompt automatically when the app first attempts screen capture
   - If prompted, click "Yes" to allow screen recording
   - You may need to grant permission in **Settings > Privacy > Screen recording** (Windows 11)

#### Linux

1. **Screen Recording Permission** (Required):
   - **X11**: Usually works automatically, but may require X11 permissions
   - **Wayland**: May require specific permissions depending on your compositor
     - **GNOME**: May need to grant permission in Settings
     - **KDE**: Usually works automatically
   - If screen capture fails, ensure your user has access to the X server or Wayland session

## System Requirements

### macOS
- **macOS 10.13 (High Sierra) or later**
- **Intel (x86_64)** or **Apple Silicon (ARM64)** (M1 or later chips)
- **Tesseract OCR**: Required (install via `brew install tesseract`)

### Windows
- **Windows 10** (version 1809 or later, x64)
- **Windows 11** (all versions, x64)
- **Tesseract OCR**: Bundled with the app (no installation needed)

### Linux
- **Ubuntu**: 20.04 LTS or later
- **Debian**: 11 (Bullseye) or later
- **Fedora**: 34 or later
- **Arch Linux**: Latest
- **Other distributions**: Any modern distribution with WebKitGTK 2.0+ support
- **Desktop Environment**: X11 or Wayland (GNOME, KDE, etc.)
- **Tesseract OCR**: Auto-installed with `.deb` package (manual install required for `.AppImage`)

### All Platforms
- **Disk Space**: ~100MB for app + ~90MB for optional embedding model
- **RAM**: 512MB minimum (2GB+ recommended)
- **Internet**: Required only for initial download and optional embedding model

## Storage Locations

### Screenshots
Screenshots are saved to the platform-specific app data directory:

- **macOS**: `~/Library/Application Support/com.sovereign.app/screenshots/`
- **Windows**: `%APPDATA%\com.sovereign.app\screenshots\` (e.g., `C:\Users\YourName\AppData\Roaming\com.sovereign.app\screenshots\`)
- **Linux**: `~/.local/share/com.sovereign.app/screenshots/`

### Database
All metadata, OCR text, and embeddings are stored in SQLite:

- **macOS**: `~/Library/Application Support/com.sovereign.app/sovereign.db`
- **Windows**: `%APPDATA%\com.sovereign.app\sovereign.db`
- **Linux**: `~/.local/share/com.sovereign.app/sovereign.db`

## Updates

To update Sovereign to the latest version:

1. Visit the **[GitHub Releases](https://github.com/lehendo/sovereign/releases/latest)** page or the **[Landing Page](https://lehendo.github.io/sovereign/)**
2. Download the latest installer for your platform
3. Install the new version (it will replace your existing installation)
4. Your data (screenshots and database) will be preserved

**Note**: The app includes quick links to GitHub releases and the landing page in the sidebar for easy access to updates.

## Troubleshooting

For common development issues and solutions, see the [Troubleshooting Guide](TROUBLESHOOTING.md). It covers:
- Tesseract OCR issues
- Build errors
- Permission problems
- App launch issues
- Hot reload problems

