# Sovereign

**Privacy-First Screen Memory Tool**

A high-performance, local-only screen recording and search system built as an alternative to Microsoft Recall. Runs 24/7 without draining battery or hoarding RAM.

## Philosophy

> "Performance is Privacy."

All processing happens locally on your device. No cloud, no telemetry, complete control.

## Security Warning

**IMPORTANT: This software captures ALL visible content on your screen, including sensitive information.**

- Screenshots are stored UNENCRYPTED on your local disk
- No privacy filters or blacklisting (yet - planned for Phase 6)
- Anyone with access to your computer can view screenshots
- Use at your own risk

See [SECURITY.md](SECURITY.md) for detailed security considerations before using this software.

## Tech Stack

- **Backend**: Rust with Tauri v2
- **Frontend**: React + TypeScript + Tailwind CSS
- **Database**: SQLite (with vector search planned)
- **Screen Capture**: xcap
- **Image Processing**: WebP compression
- **OCR**: rusty-tesseract
- **AI Embeddings**: fastembed-rs

## Features

### Phase 1: Core Loop (Complete)

- Screen capture every 2 seconds
- Smart deduplication using perceptual hashing
- Automatic 1080p resizing for 4K displays
- High-compression WebP storage
- Cross-platform path handling with Tauri's AppData
- Automatic startup on app launch
- Clean React UI with status indicator

### Phase 2: OCR & Embeddings (Complete)

- OCR text extraction using Tesseract
- Image preprocessing for optimal recognition
- Fastembed integration for 384-dimensional vectors (completely offline)
- Full pipeline: Capture → Resize → OCR → Embedding
- Graceful degradation if embedding model unavailable (app continues with OCR only)

**Embedding Model Setup**: The app loads embedding models completely offline from cache. To enable embeddings, manually download the model files once (see Installation section below).

### How It Works

1. **Capture**: The app monitors your primary display every 2 seconds
2. **Smart Check**: Calculates a perceptual hash of the screen
3. **Deduplication**: Only saves screenshots when the screen has changed
4. **OCR**: Extracts text from the image using Tesseract
5. **Embedding**: Generates semantic vectors for search (when model available)
6. **Storage**: Saves as .webp (85% quality) to minimize disk usage

## Installation

### Prerequisites

**1. Node.js and Rust**
- Node.js 18-21 (if using Node 22+, you may need to update Vite)
- Rust 1.70+
- Xcode Command Line Tools (macOS)

**2. Tesseract OCR**

Required for text extraction.

#### macOS
```bash
brew install tesseract
```

#### Windows
Download from: https://github.com/UB-Mannheim/tesseract/wiki

Add Tesseract to your PATH.

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install tesseract-ocr
```

#### Verify Installation
```bash
tesseract --version
```

You should see version 4.0 or higher.

### Setup

1. Clone the repository:
```bash
git clone https://github.com/lehendo/sovereign.git
cd sovereign
```

2. Install dependencies:
```bash
npm install
```

3. (Optional) Download embedding model for semantic search:

**Note**: The app works perfectly without embeddings (OCR only). Embeddings enable future semantic search features (Phase 4).

#### Linux / macOS

```bash
# Create cache directory
mkdir -p ~/.cache/huggingface/hub/models--Qdrant--all-MiniLM-L6-v2-onnx/snapshots/main
cd ~/.cache/huggingface/hub/models--Qdrant--all-MiniLM-L6-v2-onnx/snapshots/main

# Download model files (90MB total)
curl -L -o model.onnx "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/model.onnx?download=true"
curl -L -o tokenizer.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer.json?download=true"
curl -L -o config.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/config.json?download=true"
curl -L -o special_tokens_map.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/special_tokens_map.json?download=true"
curl -L -o tokenizer_config.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer_config.json?download=true"
```

#### Windows (PowerShell)

```powershell
# The app looks in %LOCALAPPDATA% on Windows
$cacheDir = "$env:LOCALAPPDATA\huggingface\hub\models--Qdrant--all-MiniLM-L6-v2-onnx\snapshots\main"
New-Item -ItemType Directory -Force -Path $cacheDir | Out-Null
Set-Location $cacheDir

# Download model files (90MB total)
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/model.onnx?download=true" -OutFile "model.onnx"
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer.json?download=true" -OutFile "tokenizer.json"
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/config.json?download=true" -OutFile "config.json"
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/special_tokens_map.json?download=true" -OutFile "special_tokens_map.json"
Invoke-WebRequest -Uri "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer_config.json?download=true" -OutFile "tokenizer_config.json"

Write-Host "Model files downloaded to: $cacheDir"
```

4. Run development server:
```bash
npm run tauri dev
```

### Build for Production

```bash
npm run tauri build
```

The compiled application will be in `src-tauri/target/release/`.

## Storage

Screenshots are saved to the platform-specific app data directory:

- **macOS**: `~/Library/Application Support/com.sovereign.app/screenshots/`
- **Windows**: `%APPDATA%\com.sovereign.app\screenshots\` (e.g., `C:\Users\YourName\AppData\Roaming\com.sovereign.app\screenshots\`)
- **Linux**: `~/.local/share/com.sovereign.app/screenshots/`

## Troubleshooting

### Blank white window on startup
- **Cause**: Node.js version incompatibility with Vite
- **Solution**: Using Node.js 18-21. If on Node 22+, run `npm install vite@latest`
- **Note**: Current configuration supports Node.js 21 and below

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

### npm audit warnings
- esbuild vulnerability (GHSA-67mh-4wv8-2f99) only affects dev server
- Does not impact production builds
- Can be safely ignored for development

## Roadmap

### Phase 3: The Memory (Database)
- [ ] SQLite schema implementation with vector support
- [ ] Store OCR text and embeddings
- [ ] Data persistence layer
- [ ] Encryption at rest

### Phase 4: The Recall (Search Logic)
- [ ] Natural language search
- [ ] Vector similarity matching
- [ ] Top-N results

### Phase 5: The Face (UI Construction)
- [ ] Timeline slider
- [ ] Search bar (cmd+k style)
- [ ] Grid view for results
- [ ] Full screenshot viewer

### Phase 6: Privacy Guards
- [ ] Window blacklist (incognito, password managers)
- [ ] Configurable retention policy
- [ ] Auto-delete old data

## Project Structure

```
sovereign/
├── src/                      # React frontend
│   ├── App.tsx              # Main UI component
│   ├── main.tsx             # React entry point
│   └── index.css            # Tailwind styles
├── src-tauri/
│   ├── src/
│   │   ├── main.rs          # Tauri app + auto-start logic
│   │   ├── recorder.rs      # Core capture + OCR + embeddings
│   │   └── lib.rs           # Module declarations
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
└── package.json             # Frontend dependencies
```

## License

MIT
