# How to Create a GitHub Release (v1.0.0)

## Step 1: Update Version Numbers ✅

Version numbers have been updated to `1.0.0` in:
- `package.json`
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`

## Step 2: Commit and Push Changes

```bash
# Stage all changes
git add .

# Commit with a release message
git commit -m "Release v1.0.0"

# Push to GitHub
git push origin main
```

## Step 3: Build the Application

**For macOS (current platform):**
```bash
npm run tauri build
```

This will create:
- `src-tauri/target/release/bundle/dmg/sovereign_1.0.0_x64.dmg` (or `_aarch64.dmg` for Apple Silicon)

**For Windows and Linux:**
You'll need to build on those platforms, or use GitHub Actions CI/CD (recommended for future releases).

For now, you can:
1. Build macOS version now
2. Create the release with just macOS
3. Add Windows/Linux builds later when you have access to those platforms

## Step 4: Create a Git Tag

```bash
# Create an annotated tag
git tag -a v1.0.0 -m "Release v1.0.0"

# Push the tag to GitHub
git push origin v1.0.0
```

## Step 5: Create GitHub Release

### Option A: Via GitHub Website (Easiest)

1. Go to: https://github.com/lehendo/sovereign/releases
2. Click **"Draft a new release"**
3. Fill in:
   - **Tag version**: `v1.0.0` (select from dropdown or type)
   - **Release title**: `v1.0.0 - First Public Release`
   - **Description**: Copy from template below
4. **Attach binaries**: 
   - Click "Attach binaries by dropping them here or selecting them"
   - Upload your `.dmg` file from `src-tauri/target/release/bundle/dmg/`
5. Check **"Set as the latest release"**
6. Click **"Publish release"**

### Option B: Via GitHub CLI (if installed)

```bash
gh release create v1.0.0 \
  --title "v1.0.0 - First Public Release" \
  --notes "First public release of Sovereign" \
  src-tauri/target/release/bundle/dmg/sovereign_1.0.0_*.dmg
```

## Release Notes Template

```markdown
# v1.0.0 - First Public Release

## Features

- Privacy-first screen memory with local-only processing
- Automatic screen capture every 2 seconds with smart deduplication
- OCR text extraction using Tesseract
- Semantic search using local AI embeddings (optional)
- Privacy Guard: Automatic blacklist for sensitive applications
- Auto-deletion: 14-day retention policy
- Modern dark-mode UI with timeline and search

## Installation

Download the appropriate file for your platform:
- **macOS**: `.dmg` file (Apple Silicon & Intel)
- **Windows**: Coming soon
- **Linux**: Coming soon

## Requirements

- Tesseract OCR (required for text extraction)
- Optional: Embedding model for semantic search (see README)

## Security

⚠️ **Important**: This software captures all visible screen content. Screenshots are stored unencrypted on your local disk. See [SECURITY.md](SECURITY.md) for details.

## Full Changelog

See [README.md](README.md) for complete documentation.
```

## Step 6: Update Landing Page (Optional)

Once the release is live, you can update the landing page to use direct download links:

```html
href="https://github.com/lehendo/sovereign/releases/download/v1.0.0/sovereign_1.0.0_x64.dmg"
```

## Future Releases

For future releases, consider setting up GitHub Actions to automatically build for all platforms. This way you can build Windows and Linux binaries without needing those machines.

## Troubleshooting

- **Tag already exists?**: Delete it with `git tag -d v1.0.0 && git push origin :refs/tags/v1.0.0`
- **Build fails?**: Make sure all dependencies are installed (`npm install`, Rust toolchain)
- **Can't find .dmg file?**: Check `src-tauri/target/release/bundle/dmg/` directory

