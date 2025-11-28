# Sovereign

**Privacy-First Screen Memory Desktop Application**

A high-performance, local-only screen recording and search system. Runs continuously without draining battery or consuming excessive memory.

**[Visit the Landing Page →](https://lehendo.github.io/sovereign/)**

## Philosophy

> "Performance is Privacy."

All processing happens locally on your device. No cloud, no telemetry, complete control.

## Security Warning

**IMPORTANT: This software captures ALL visible content on your screen, including sensitive information.**

- Screenshots are stored UNENCRYPTED on your local disk
- Anyone with access to your computer can view screenshots
- Use at your own risk

See [SECURITY.md](SECURITY.md) for detailed security considerations before using this software.

## Tech Stack

- **Backend**: Rust with Tauri v2
- **Frontend**: React + TypeScript + Tailwind CSS
- **Database**: SQLite with vector search
- **Screen Capture**: xcap
- **Image Processing**: WebP compression
- **OCR**: rusty-tesseract
- **AI Embeddings**: fastembed-rs

## Features

### Screen Capture

- Automatic capture every 2 seconds
- Smart deduplication using perceptual hashing
- Automatic 1080p resizing for 4K displays
- High-compression WebP storage
- Cross-platform path handling
- Only saves screenshots when the screen has changed

### Text Extraction & Search

- OCR text extraction using Tesseract
- Image preprocessing for optimal recognition
- Semantic search using 384-dimensional vector embeddings
- Natural language queries with cosine similarity ranking
- Top-20 results with similarity scores
- Completely offline - no network requests required

### Privacy Guards

- **Window Blacklist**: Automatically skips recording when sensitive applications are detected
  - Password managers: Bitwarden, 1Password, KeePass, LastPass
  - Private browsing: Incognito, InPrivate, Private Browsing
  - Privacy tools: Tor Browser
  - Uses native system commands for maximum stability
- **Auto-Deletion**: Retention policy automatically removes data older than 14 days
  - Deletes database records (frames, OCR text, embeddings)
  - Removes image files from disk
  - Runs on app startup
- **Privacy Status UI**: Shield icon indicator showing active protection

### User Interface

- Modern dark-mode UI with Tailwind CSS
- Cmd+K style search bar with keyboard shortcuts
- Timeline slider for navigating history
- Responsive masonry grid layout
- Full-screen modal viewer for images + OCR text
- Real-time frame updates
- Live statistics dashboard

## How It Works

1. **Privacy Check**: Checks if the active window is on the blacklist
2. **Capture**: Monitors your primary display every 2 seconds
3. **Smart Check**: Calculates a perceptual hash of the screen
4. **Deduplication**: Only saves screenshots when the screen has changed
5. **OCR**: Extracts text from the image using Tesseract
6. **Embedding**: Generates semantic 384-dimensional vectors
7. **Storage**: Saves .webp images + metadata/text/vectors to SQLite
8. **Search**: Query in natural language, get ranked results by semantic similarity
9. **Auto-Cleanup**: Removes data older than 14 days on startup

## Installation

### Quick Start (End Users)

1. **Download** the latest release from [GitHub Releases](https://github.com/lehendo/sovereign/releases/latest)
   - **macOS**: Download `.dmg` file, open and drag to Applications
   - **Windows**: Download `.exe` installer and run
   - **Linux**: Download `.deb` or `.AppImage` file

2. **Install Tesseract OCR** (Required for text extraction):

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

3. **(Optional) Download Embedding Model** for semantic search:
   
   The app works without embeddings (OCR-only mode). For semantic search, download the model:

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

4. **Run the app** and grant required permissions when prompted.

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

### Development (Build from Source)

For contributors and developers who want to build from source:

**Prerequisites:**
- **Node.js 20.19+ or 22.12+** (LTS versions recommended)
- **Rust 1.70+**
- **Xcode Command Line Tools** (macOS)
- **Tesseract OCR** (see Quick Start section above)

**Setup:**

1. Clone the repository:
```bash
git clone https://github.com/lehendo/sovereign.git
cd sovereign
```

2. Install dependencies:
```bash
npm install
```

3. (Optional) Download embedding model (see Quick Start section above)

4. Run development server:
```bash
npm run tauri dev
```

**Build for Production:**

```bash
npm run tauri build
```

The compiled application will be in `src-tauri/target/release/`.

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

The database contains:
- `frames` table: Timestamps, image paths, perceptual hashes
- `ocr_text` table: Extracted text from each frame
- `embeddings` table: 384-dimensional vectors (serialized as binary blobs)

## Troubleshooting

### Blank white window on startup
- **Cause**: Node.js version too old
- **Solution**: Upgrade to Node.js 20.19+ or 22.12+ (LTS versions)
  - Download from: https://nodejs.org/
  - After upgrading: `rm -rf node_modules package-lock.json && npm install`
- **Note**: Node 21 and earlier are End-of-Life and not supported

### "Tesseract not found"
- Make sure Tesseract is installed and in your PATH
- On Windows, you may need to set `TESSDATA_PREFIX` environment variable

### "Model cache not found"
- The app works perfectly with OCR only
- To enable embeddings, follow the manual download instructions above
- Model files are loaded completely offline (no network requests)
- Total download size: ~90MB (one-time)

### High CPU usage
- Normal during OCR processing (CPU-intensive by design)
- Consider increasing capture interval if needed

### OCR returns wrong text
- OCR accuracy depends on screen content quality
- Works best with clear, high-contrast text
- Small/blurry text may not be recognized accurately

### Privacy Guard not working

**macOS:**
- Ensure Accessibility permission is enabled (see Required Permissions above)
- Check terminal output for "[Privacy Guard] Failed to get active window" messages
- Restart the app after enabling Accessibility permission
- Privacy Guard requires both Screen Recording and Accessibility permissions

**Windows:**
- Window detection should work automatically
- If Privacy Guard fails, check that PowerShell is available and working
- Ensure the app has necessary permissions to query window information

**Linux:**
- Ensure `xdotool` is installed (see Required Permissions above)
- On X11: Window detection should work automatically once xdotool is installed
- On Wayland: Window detection may be limited (xdotool primarily supports X11)
- Check terminal output for error messages related to xdotool

## API Reference

### Tauri Commands (Callable from Frontend)

```typescript
// Search frames by semantic similarity
await invoke<SearchResult[]>('search_frames', { query: 'code review' });

// Get recent frames for timeline
await invoke<FrameMetadata[]>('get_recent_frames', { limit: 50 });

// Get database statistics
await invoke<DatabaseStats>('get_database_stats');
```

## Future Enhancements

- User-configurable blacklist
- Per-window privacy settings
- Encrypted storage option
- Export/import data
- Advanced search filters
- Activity timeline visualization
- Password protection for stored screenshots
- Per-window granular control

## Project Structure

```
sovereign/
├── src/                      # React frontend
│   ├── App.tsx              # Main UI component + TanStack Query
│   ├── components/          # UI components
│   │   ├── SearchBar.tsx    # Cmd+K search (keyboard shortcuts)
│   │   ├── Timeline.tsx     # History slider
│   │   ├── Grid.tsx         # Masonry layout for frames
│   │   └── Modal.tsx        # Full image viewer
│   ├── types.ts             # TypeScript definitions
│   ├── main.tsx             # React entry point
│   └── index.css            # Tailwind styles
├── src-tauri/
│   ├── src/
│   │   ├── main.rs          # Tauri app + commands + retention
│   │   ├── recorder.rs      # Capture + OCR + embeddings + blacklist
│   │   ├── database.rs      # SQLite persistence + pruning
│   │   ├── search.rs        # Cosine similarity search
│   │   ├── commands.rs      # Tauri command handlers
│   │   └── lib.rs           # Module declarations
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
└── package.json             # Frontend dependencies
```

## License

MIT
