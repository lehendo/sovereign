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
| **Privacy Guards** | Manual configuration | ✅ Automatic blacklist (currently disabled) |
| **Performance** | Unknown | ✅ <1% CPU, minimal battery |
| **Cost** | Requires Windows 11+ | ✅ Free, no requirements |

### Key Advantages

- **100% Local** - No cloud, no telemetry, no data collection
- **High Performance** - Written in Rust, uses <1% CPU, minimal battery drain
- **Semantic Search** - Find anything you've seen using natural language
- **Smart Deduplication** - Only saves when your screen actually changes
- **Auto-Updates** - Built-in updater keeps you secure
- **Auto-Cleanup** - Automatically deletes data older than 14 days
- **Multi-Monitor Support** - Captures all connected displays

## Key Features

### Semantic Search
Search your screen history using natural language. Find that email, code snippet, or conversation you saw yesterday - even if you don't remember the exact words.

### Privacy First
- **Local-Only**: All processing happens on your device. Zero network requests after initial setup
- **Auto-Deletion**: Data older than 14 days is automatically removed
- **Privacy Guards**: Currently disabled but code preserved for future use (see [Security](#security-considerations))

### Performance Optimized
- **Smart Capture**: Only saves screenshots when your screen actually changes (perceptual hashing)
- **Efficient Storage**: High-compression WebP format, automatic resizing for large displays (8K+)
- **Low Resource Usage**: Runs in background with minimal CPU and memory footprint
- **Multi-Monitor**: Automatically captures and combines all connected displays

### Modern Interface
- Dark mode UI with Cmd+K search (macOS) / Ctrl+K (Windows/Linux)
- Timeline slider to navigate your history
- Real-time frame updates (auto-refreshes when new screenshots are captured)
- Full-screen viewer with zoom and pan capabilities
- Extracted text display with OCR results

## Important Security Notice

**This software captures ALL visible content on your screen, including sensitive information.**

- Screenshots are stored **UNENCRYPTED** on your local disk
- Anyone with physical or remote access to your computer can view screenshots
- Use at your own risk and ensure your device is properly secured
- **Privacy Guard is currently disabled** - all windows are captured regardless of content

See [SECURITY.md](SECURITY.md) for detailed security considerations.

## How It Works

1. **Smart Capture** - Takes a screenshot every 2 seconds, but only saves when the screen changes
2. **Multi-Monitor** - Automatically detects and captures all connected displays
3. **Text Extraction** - Uses Tesseract OCR to extract all visible text
4. **Semantic Indexing** - Generates AI embeddings for natural language search (optional)
5. **Storage** - Saves compressed screenshots and metadata locally
6. **Search** - Query in plain English to find anything you've seen
7. **Auto-Cleanup** - Removes data older than 14 days automatically

## Quick Start

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

## Local Development / Building from Source

If you prefer to build and run the app locally instead of downloading pre-built binaries:

### Prerequisites

- **Node.js**: 20.19.0+ or 22.12.0+ ([Download](https://nodejs.org/))
- **Rust**: Latest stable version ([Install via rustup](https://rustup.rs/))
- **Tauri CLI**: Will be installed automatically via npm
- **Tesseract OCR**: Required (see installation instructions above)
- **System Dependencies**:
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Windows**: Microsoft Visual C++ Build Tools
  - **Linux**: `libwebkit2gtk-4.0-dev`, `build-essential`, `curl`, `wget`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`

### Installation Steps

1. **Clone the repository:**
   ```bash
   git clone https://github.com/lehendo/sovereign.git
   cd sovereign
   ```

2. **Install Node.js dependencies:**
   ```bash
   npm install
   ```

3. **Install Rust dependencies** (automatic on first build):
   ```bash
   cd src-tauri
   cargo build
   cd ..
   ```

4. **Run in development mode:**
   ```bash
   npm run tauri dev
   ```
   This will:
   - Start the Vite dev server for the frontend
   - Build and run the Tauri app
   - Enable hot-reload for both frontend and backend changes

5. **Build for production:**
   ```bash
   # Build for current platform
   npm run tauri build
   
   # Or build for specific platforms:
   npm run tauri:build:macos        # macOS Intel
   npm run tauri:build:macos-arm    # macOS Apple Silicon
   npm run tauri:build:windows      # Windows
   npm run tauri:build:linux        # Linux
   ```

   Built binaries will be in `src-tauri/target/release/bundle/`

### Development Notes

- **Frontend**: React + TypeScript + Vite, located in `src/`
- **Backend**: Rust, located in `src-tauri/src/`
- **Hot Reload**: Frontend changes reload automatically, Rust changes require app restart
- **Debugging**: 
  - Frontend: Use browser DevTools (right-click → Inspect)
  - Backend: Check terminal output for Rust logs
- **Database**: SQLite database is created automatically in app data directory

### Required Permissions

#### macOS

When the app first runs, macOS will prompt for permissions:

1. **Screen Recording Permission** (Required):
   - Click "Open System Settings" when prompted
   - Go to **System Settings > Privacy & Security > Screen Recording**
   - Enable "Sovereign" (or "Terminal" if running from terminal)
   - Restart the app

#### Windows

Windows may prompt for permissions on first run:

1. **Screen Recording Permission** (Required):
   - Windows 10/11 will typically prompt automatically when the app first attempts screen capture
   - If prompted, click "Yes" to allow screen recording
   - You may need to grant permission in **Settings > Privacy > Screen recording** (Windows 11)

#### Linux

Linux requirements depend on your desktop environment:

1. **Screen Recording Permission** (Required):
   - **X11**: Usually works automatically, but may require X11 permissions
   - **Wayland**: May require specific permissions depending on your compositor
     - **GNOME**: May need to grant permission in Settings
     - **KDE**: Usually works automatically
   - If screen capture fails, ensure your user has access to the X server or Wayland session

### Auto-Updates

Sovereign includes built-in auto-update functionality:

- Click **"Check for Updates"** in the sidebar to manually check
- When a new version is available, the app will download, verify, and install automatically
- Updates are cryptographically signed for security
- You'll be notified when updates are ready to install

**Note**: Auto-updates require `latest.json` to be present in GitHub releases. If you're building from source, you'll need to generate this file manually or disable auto-updates.

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

## Security Considerations

### Current Security Status

✅ **Secure:**
- No hardcoded credentials or API keys
- No network requests for data (only for updates and optional model download)
- All data stored locally
- Proper permission scoping in Tauri capabilities
- Input validation on search queries (max 1000 characters)
- SQL injection protection via parameterized queries (rusqlite)

⚠️ **Security Notes:**
- **Privacy Guard is currently disabled** - All windows are captured regardless of content
- Screenshots stored unencrypted on disk
- Asset protocol scope is permissive (`$APPDATA/**`) - necessary for app functionality
- Console logging present (debug info only, no sensitive data)
- No authentication required to view screenshots

### Data Privacy

- **All data stays local:** No telemetry, no cloud uploads, no network requests for data
- **Storage location:** Screenshots are saved to your OS's application data directory
- **No encryption:** Screenshots are currently stored as plain .webp files
- **No access controls:** Any user on your system can read the screenshots
- **Auto-deletion:** Data older than 14 days is automatically removed

### Recommendations

1. **Do NOT use this software if you:**
   - Work with classified or highly sensitive information
   - Handle financial data, healthcare records, or legal documents
   - Are in a shared computer environment

2. **Best Practices:**
   - Regularly review and delete old screenshots
   - Be aware of what's on your screen when the app is running
   - Keep your computer physically secure
   - Use full disk encryption on your OS

## Troubleshooting

### "Sovereign cannot be opened because the developer cannot be verified" (macOS)

This is macOS Gatekeeper blocking unsigned apps. To fix:

**Method 1 (Recommended):**
1. Right-click the Sovereign app in Applications folder
2. Select "Open" from the context menu
3. Click "Open" in the security dialog
4. The app will now open normally (you only need to do this once)

**Method 2:**
1. Go to **System Settings → Privacy & Security**
2. Scroll down to find a message about Sovereign being blocked
3. Click **"Open Anyway"** button
4. Confirm by clicking "Open" in the dialog

**Why this happens:** Sovereign is open-source and not code-signed with an Apple Developer certificate (which costs $99/year). The app is safe to use - you can verify the source code on GitHub.

### "Windows protected your PC" / SmartScreen Warning (Windows)

Windows SmartScreen may block unsigned installers. To fix:

1. When you see "Windows protected your PC" warning, click **"More info"**
2. Click **"Run anyway"** button
3. The installer will proceed normally

**Why this happens:** Sovereign is not code-signed with a Windows code signing certificate (which costs ~$200-400/year). The app is safe to use - you can verify the source code on GitHub.

### AppImage won't run (Linux)

If the AppImage file won't execute:

1. Make it executable:
   ```bash
   chmod +x Sovereign_*.AppImage
   ```
2. Then run it:
   ```bash
   ./Sovereign_*.AppImage
   ```

**Note:** Linux doesn't have code signing requirements like macOS/Windows. This is just a file permissions issue.

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

### Images not loading (403 errors)
- The app uses Tauri's asset protocol to serve images
- If you see 403 errors in the console, the app will automatically fall back to reading files directly
- This is expected behavior and images should still display correctly

### Auto-refresh not working
- The app uses Tauri events to automatically refresh when new screenshots are captured
- If auto-refresh isn't working, check the browser console for event listener setup messages
- The app will still poll every 5 seconds as a fallback

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

**Q: Why is Privacy Guard disabled?**  
A: Privacy Guard functionality is currently commented out but preserved in the codebase. It can be re-enabled in future versions. Currently, all windows are captured regardless of content.

**Q: Does it work with multiple monitors?**  
A: Yes! The app automatically detects and captures all connected displays, combining them into a single screenshot.

## System Requirements

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

### All Platforms
- **Disk Space**: ~100MB for app + ~90MB for optional embedding model
- **RAM**: 512MB minimum (2GB+ recommended)
- **Internet**: Required only for initial download and optional embedding model

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details.
