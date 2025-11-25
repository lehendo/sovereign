# Sovereign

**Privacy-First Screen Memory Tool**

A high-performance, local-only screen recording and search system built as an alternative to Microsoft Recall. Runs 24/7 without draining battery or hoarding RAM.

## Philosophy

> "Performance is Privacy."

All processing happens locally on your device. No cloud, no telemetry, complete control.

## Tech Stack

- **Backend**: Rust with Tauri v2
- **Frontend**: React + TypeScript + Tailwind CSS
- **Database**: SQLite (with vector search planned)
- **Screen Capture**: xcap
- **Image Processing**: WebP compression
- **OCR**: rusty-tesseract (Phase 2)
- **AI Embeddings**: fastembed-rs (Phase 2)

## Phase 1: Core Loop

**Status**: Complete

### Features Implemented

- Screen capture every 2 seconds
- Smart deduplication using perceptual hashing
- Automatic 1080p resizing for 4K displays
- High-compression WebP storage
- Tauri's AppData directory for cross-platform compatibility
- Automatic startup on app launch
- Clean React UI with status indicator

### How It Works

1. **Capture**: The app monitors your primary display every 2 seconds
2. **Smart Check**: Calculates a perceptual hash of the screen
3. **Deduplication**: Only saves screenshots when the screen has changed
4. **Compression**: Saves as .webp (85% quality) to minimize storage
5. **Resize**: Automatically resizes 4K screenshots to 1080p

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
│   │   ├── recorder.rs      # Core capture module
│   │   └── lib.rs           # Module declarations
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
└── package.json             # Frontend dependencies
```

## Development

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Xcode Command Line Tools (macOS)

### Run Development Server

```bash
npm install
npm run tauri dev
```

The app will:
- Start Vite dev server on http://localhost:1420
- Launch the Tauri window
- Begin capturing screenshots automatically
- Save to: `~/Library/Application Support/com.sovereign.app/screenshots/`

### Build for Production

```bash
npm run tauri build
```

## Storage

Screenshots are saved to the platform-specific app data directory:

- **macOS**: `~/Library/Application Support/com.sovereign.app/screenshots/`
- **Windows**: `%APPDATA%\com.sovereign.app\screenshots\`
- **Linux**: `~/.local/share/com.sovereign.app/screenshots/`

## Roadmap

### Phase 2: The Brain (OCR & Embeddings)
- [ ] Integrate rusty-tesseract for OCR
- [ ] Integrate fastembed-rs for semantic embeddings
- [ ] Pipeline: Capture → OCR → Generate Embedding

### Phase 3: The Memory (Database)
- [ ] SQLite schema implementation
- [ ] Data persistence
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

## License

MIT

