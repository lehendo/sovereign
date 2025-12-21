# Sovereign

<div align="center">

**Your Digital Memory. Not Microsoft's.**

A privacy-first, local-only alternative to Microsoft Recall. Search everything you've seen on your screen without sending a single pixel to the cloud.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)](https://github.com/lehendo/sovereign/releases/latest)

**[🌐 Visit Landing Page](https://lehendo.github.io/sovereign/)** • **[📥 Download Latest Release](https://github.com/lehendo/sovereign/releases/latest)** • **[🔒 Security Policy](SECURITY.md)**

</div>

---

## Why Sovereign?

**Microsoft Recall** sends your screen data to the cloud. **Sovereign** keeps everything on your device.

| Feature | Microsoft Recall | Sovereign |
|---------|-----------------|-----------|
| **Data Storage** | Cloud (Microsoft servers) | 100% Local (your device) |
| **Privacy** | Data sent to Microsoft | Zero network requests |
| **Open Source** | ❌ Closed source | ✅ Open source (MIT) |
| **Cross-Platform** | Windows only | ✅ macOS, Windows, Linux |
| **Privacy Guards** | Manual configuration | ✅ Automatic blacklist |
| **Performance** | Unknown | ✅ <1% CPU, minimal battery |
| **Cost** | Requires Windows 11+ | ✅ Free, no requirements |

### Key Advantages

- **100% Local** - No cloud, no telemetry, no data collection
- **High Performance** - Written in Rust, uses <1% CPU, minimal battery drain
- **Privacy Guards** - Automatically skips recording sensitive windows (password managers, incognito mode)
- **Semantic Search** - Find anything you've seen using natural language
- **Smart Deduplication** - Only saves when your screen actually changes
- **Auto-Updates** - Built-in updater keeps you secure
- **Auto-Cleanup** - Automatically deletes data older than 14 days

## Key Features

### 🔍 Semantic Search
Search your screen history using natural language. Find that email, code snippet, or conversation you saw yesterday - even if you don't remember the exact words.

### 🛡️ Privacy First
- **Automatic Blacklist**: Skips recording when you open password managers (Bitwarden, 1Password, KeePass, LastPass), incognito windows, or Tor Browser
- **Local-Only**: All processing happens on your device. Zero network requests after initial setup
- **Auto-Deletion**: Data older than 14 days is automatically removed

### ⚡ Performance Optimized
- **Smart Capture**: Only saves screenshots when your screen actually changes (perceptual hashing)
- **Efficient Storage**: High-compression WebP format, automatic 1080p resizing for 4K displays
- **Low Resource Usage**: Runs in background with minimal CPU and memory footprint

### Modern Interface
- Dark mode UI with Cmd+K search (macOS) / Ctrl+K (Windows/Linux)
- Timeline slider to navigate your history
- Real-time frame updates and live statistics
- Full-screen viewer with extracted text display

## Important Security Notice

**This software captures ALL visible content on your screen, including sensitive information.**

- Screenshots are stored **UNENCRYPTED** on your local disk
- Anyone with physical or remote access to your computer can view screenshots
- Use at your own risk and ensure your device is properly secured

See [SECURITY.md](SECURITY.md) for detailed security considerations.

## How It Works

1. **Privacy Check** - Automatically detects and skips sensitive windows
2. **Smart Capture** - Takes a screenshot every 2 seconds, but only saves when the screen changes
3. **Text Extraction** - Uses Tesseract OCR to extract all visible text
4. **Semantic Indexing** - Generates AI embeddings for natural language search (optional)
5. **Storage** - Saves compressed screenshots and metadata locally
6. **Search** - Query in plain English to find anything you've seen
7. **Auto-Cleanup** - Removes data older than 14 days automatically

## Quick Start

### Step 1: Download

Download the latest release from [GitHub Releases](https://github.com/lehendo/sovereign/releases/latest):

- **macOS**: Download `.dmg` file → Open → Drag to Applications folder
- **Windows**: Download `.exe` installer → Run and follow prompts
- **Linux**: Download `.deb` or `.AppImage` file → Install/run

### Step 2: Install Tesseract OCR

**Required for text extraction:**

   **macOS:**
   ```bash
   brew install tesseract
   ```

   **Windows:**
   Download from: https://github.com/UB-Mannheim/tesseract/wiki  
   Add Tesseract to your PATH.

   **Linux (Ubuntu/Debian):**
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

Run the app and grant required permissions when prompted (see below).

### Required Permissions

#### macOS

When the app first runs, macOS will prompt for permissions:

1. **Screen Recording Permission** (Required):
   - Click "Open System Settings" when prompted
   - Go to **System Settings > Privacy & Security > Screen Recording**
   - Enable "Sovereign"
   - Restart the app

2. **Accessibility Permission** (Required for Privacy Guard):
   - Go to **System Settings > Privacy & Security > Accessibility**
   - Enable "Sovereign"
   - This allows the app to detect active window titles for blacklist checking
   - Restart the app after enabling

#### Windows

Windows may prompt for permissions on first run:

1. **Screen Recording Permission** (Required):
   - Windows 10/11 will typically prompt automatically when the app first attempts screen capture
   - If prompted, click "Yes" to allow screen recording
   - You may need to grant permission in **Settings > Privacy > Screen recording** (Windows 11)

2. **Window Detection** (For Privacy Guard):
   - No special permissions required
   - Uses PowerShell with Win32 API (works automatically)

#### Linux

Linux requirements depend on your desktop environment:

1. **Screen Recording Permission** (Required):
   - **X11**: Usually works automatically, but may require X11 permissions
   - **Wayland**: May require specific permissions depending on your compositor
     - **GNOME**: May need to grant permission in Settings
     - **KDE**: Usually works automatically
   - If screen capture fails, ensure your user has access to the X server or Wayland session

2. **Window Detection** (For Privacy Guard):
   - Requires `xdotool` to be installed:
     ```bash
     # Ubuntu/Debian
     sudo apt-get install xdotool
     
     # Fedora
     sudo dnf install xdotool
     
     # Arch Linux
     sudo pacman -S xdotool
     ```
   - **X11**: Works automatically once xdotool is installed
   - **Wayland**: Window detection may be limited (xdotool primarily supports X11)

### Auto-Updates

Sovereign includes built-in auto-update functionality:

- Click **"Check for Updates"** in the sidebar to manually check
- When a new version is available, the app will download, verify, and install automatically
- Updates are cryptographically signed for security
- You'll be notified when updates are ready to install

## Storage

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

## Troubleshooting

### "Tesseract not found"
- Make sure Tesseract is installed (see Installation instructions above)
- On Windows, you may need to add Tesseract to your PATH or set the `TESSDATA_PREFIX` environment variable
- Restart the app after installing Tesseract

### "Model cache not found"
- The app works perfectly with OCR-only mode (text search without semantic search)
- To enable semantic search with embeddings, follow the manual download instructions in the Installation section above
- Model files are loaded completely offline (no network requests)
- Total download size: ~90MB (one-time download)

### High CPU usage
- Normal during OCR processing (CPU-intensive by design)
- The app captures and processes screenshots every 2 seconds
- CPU usage should stabilize after initial processing

### OCR returns wrong text
- OCR accuracy depends on screen content quality
- Works best with clear, high-contrast text
- Small, blurry, or stylized text may not be recognized accurately
- This is a limitation of OCR technology, not a bug

### Privacy Guard not working

**macOS:**
- Ensure both Screen Recording and Accessibility permissions are enabled (see Required Permissions above)
- Go to **System Settings > Privacy & Security > Accessibility** and enable "Sovereign"
- Restart the app after enabling Accessibility permission
- Privacy Guard requires both permissions to function

**Windows:**
- Window detection should work automatically
- If Privacy Guard fails, ensure the app has necessary permissions
- Try restarting the app

**Linux:**
- Ensure `xdotool` is installed (see Required Permissions above)
- On X11: Window detection should work automatically once xdotool is installed
- On Wayland: Window detection may be limited (xdotool primarily supports X11)

## FAQ

**Q: How is this different from Microsoft Recall?**  
A: Sovereign runs 100% locally on your device. Microsoft Recall sends your data to the cloud. Sovereign gives you complete control and privacy.

**Q: Does it work offline?**  
A: Yes! After the initial setup (downloading Tesseract and optionally the embedding model), everything works completely offline. No internet connection required.

**Q: How much disk space does it use?**  
A: Depends on your usage, but screenshots are highly compressed (WebP format). The app automatically deletes data older than 14 days.

**Q: Can I change the retention period?**  
A: Currently fixed at 14 days. This ensures privacy while maintaining useful search history.

**Q: Does it slow down my computer?**  
A: No. Written in Rust for maximum performance. Uses <1% CPU and minimal memory. Designed to run continuously without impacting system performance.

**Q: What if I don't install the embedding model?**  
A: The app works perfectly fine! You'll have OCR text search (exact text matching). Semantic search (finding by meaning) requires the optional model download.

**Q: Is my data encrypted?**  
A: No. Screenshots are stored unencrypted on your local disk. Anyone with access to your computer can view them. This is by design for performance, but ensure your device is properly secured.

## System Requirements

> **Note**: This app is actively tested on macOS Intel. Other platforms are built and should work, but may have platform-specific issues. Please report any problems you encounter.

### macOS
- **macOS 10.13 (High Sierra) or later**
- **Intel (x86_64)**: ✅ Tested and working
- **Apple Silicon (ARM64)**: ✅ Built and tested (M1, M2, M3, and later chips)
- **Tesseract OCR**: Required (install via `brew install tesseract`)

### Windows
- **Windows 10** (version 1809 or later, x64) - ⚠️ Built but limited testing
- **Windows 11** (all versions, x64) - ⚠️ Built but limited testing
- **Tesseract OCR**: Required (download from [UB-Mannheim](https://github.com/UB-Mannheim/tesseract/wiki))

### Linux
- **Ubuntu**: 20.04 LTS or later - ⚠️ Built but limited testing
- **Debian**: 11 (Bullseye) or later - ⚠️ Built but limited testing
- **Fedora**: 34 or later - ⚠️ Built but limited testing
- **Arch Linux**: Latest - ⚠️ Built but limited testing
- **Other distributions**: Any modern distribution with WebKitGTK 2.0+ support
- **Desktop Environment**: X11 or Wayland (GNOME, KDE, etc.)
- **Tesseract OCR**: Required (install via package manager)
- **xdotool**: Required for Privacy Guard (install via package manager)

### All Platforms
- **Disk Space**: ~100MB for app + ~90MB for optional embedding model
- **RAM**: 512MB minimum (2GB+ recommended)
- **Internet**: Required only for initial download and optional embedding model

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details.
