# Testing Sovereign on macOS

## Prerequisites Check

Before testing, ensure you have:
- Node.js 20.19+ or 22.12+ (`node --version`)
- Rust installed (`rustc --version`)
- Tesseract OCR installed (`tesseract --version`)

## About Screen Recording Permissions

### Cursor IDE Permission (The popup you're seeing)
The popup "Cursor would like to record this computer's screen and audio" is from **Cursor IDE itself**, not Sovereign. This is for Cursor's AI features (like reading your screen context). This is separate from Sovereign.

### Sovereign App Permission
When you run Sovereign for the first time, **macOS will ask Sovereign for screen recording permission**. This is the permission Sovereign needs to capture your screen.

## Testing Steps

### 1. Install Dependencies

```bash
cd /Users/arjunchatterjee/sovereign
npm install
```

### 2. (Optional) Download Embedding Model

For semantic search to work, download the model:

```bash
mkdir -p ~/.cache/huggingface/hub/models--Qdrant--all-MiniLM-L6-v2-onnx/snapshots/main
cd ~/.cache/huggingface/hub/models--Qdrant--all-MiniLM-L6-v2-onnx/snapshots/main

curl -L -o model.onnx "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/model.onnx?download=true"
curl -L -o tokenizer.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer.json?download=true"
curl -L -o config.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/config.json?download=true"
curl -L -o special_tokens_map.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/special_tokens_map.json?download=true"
curl -L -o tokenizer_config.json "https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main/tokenizer_config.json?download=true"
```

**Note:** The app works fine without embeddings (OCR only). Skip this if you just want to test basic functionality.

### 3. Run Sovereign in Development Mode

```bash
cd /Users/arjunchatterjee/sovereign
npm run tauri dev
```

**What happens:**
1. Vite dev server starts (frontend)
2. Rust backend compiles (may take 2-5 minutes first time)
3. Sovereign window opens
4. **macOS will prompt for screen recording permission** - Click "Open System Settings"

### 4. Grant Screen Recording Permission

When the macOS dialog appears:

1. Click "Open System Settings"
2. In **System Settings > Privacy & Security > Screen Recording**
3. Find "Sovereign" in the list
4. Toggle the checkbox ON
5. Click "Quit & Reopen" when prompted
6. Run `npm run tauri dev` again

### 5. Test Basic Functionality

Once the app is running:

**Backend (Check Terminal):**
- You should see: "Starting screen capture loop with OCR and embeddings..."
- Every 2 seconds: "Captured: [timestamp].webp" (if screen changed)
- Or: "No change detected" (if screen hasn't changed)

**Frontend (Check UI):**
- Header shows: "Privacy Guard Active" (green shield)
- Statistics showing: 0 frames initially
- After a few captures, thumbnails appear in the grid

### 6. Test Privacy Guard

The Privacy Guard automatically prevents capturing screenshots of sensitive windows.

**How to Test:**

1. **Open a private browser window:**
   - Chrome: Press `Cmd+Shift+N` (Incognito)
   - Safari: File > New Private Window
   - Firefox: Press `Cmd+Shift+P`

2. **Check the terminal output:**
   - You should see: `"Privacy Guard triggered: Window title contains 'Private'"`
   - The capture will be skipped
   - No screenshot will be saved

3. **Try other blacklisted apps:**
   - Open 1Password, Bitwarden, or any password manager
   - Open Tor Browser
   - Terminal should show "Privacy Guard triggered" for each

4. **Verify normal windows still work:**
   - Switch to a regular browser tab (not incognito)
   - Terminal should show normal capture messages

### 7. Test Search

After capturing a few frames:

1. Press **Cmd+K** in the Sovereign UI
2. Type a search query (e.g., "code" or "terminal")
3. Press Enter
4. Search results appear with similarity scores
5. Click a thumbnail to view full image + OCR text

### 8. Check Data Storage

Your data is stored in:

```bash
# Screenshots and database
~/Library/Application Support/com.sovereign.app/

# View database
ls -lh ~/Library/Application\ Support/com.sovereign.app/

# You should see:
# - sovereign.db (SQLite database)
# - screenshots/ (folder with .webp files)
```

## What to Look For

**Success Indicators:**
- No errors in terminal
- Thumbnails appear in UI after a few seconds
- Privacy Guard shows "Active"
- Search returns results (if embeddings installed)
- Statistics update in real-time

**Common Issues:**

**"Tesseract not found"**
- Install: `brew install tesseract`

**"No change detected" every capture**
- Normal! Move your windows around to create screen changes

**Search returns no results**
- Either: No embeddings installed (expected)
- Or: No OCR text detected (try capturing windows with text)

**Blank frames in UI**
- Check: `~/Library/Application Support/com.sovereign.app/screenshots/`
- Files should exist and be valid .webp images

## Testing Retention Policy

The app deletes data older than 14 days on startup. To test:

1. Manually set a frame's timestamp to 15 days ago in the database
2. Restart the app
3. Check terminal for: "Pruned X frames older than 14 days"

## Stop Testing

Press **Ctrl+C** in the terminal to stop the app.

## Clean Up Test Data

To remove all test data:

```bash
rm -rf ~/Library/Application\ Support/com.sovereign.app/
```

## Building for Production

To create a distributable .dmg:

```bash
npm run tauri build
```

The .dmg will be in: `src-tauri/target/release/bundle/dmg/`

## Intel Mac Specific Notes

- Intel Macs may take longer to compile Rust (3-5 minutes)
- Performance should be smooth once running
- OCR and embeddings work identically to Apple Silicon

