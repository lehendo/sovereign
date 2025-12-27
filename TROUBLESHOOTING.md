# Troubleshooting Guide

Common issues and solutions for Sovereign.

## Installation Issues

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

## Tesseract OCR Issues

### "Tesseract not found" or OCR not working

**macOS:**
1. Verify Tesseract is installed:
   ```bash
   tesseract --version
   ```
2. If not installed:
   ```bash
   brew install tesseract
   ```
3. Restart the app after installing

**Windows:**
Tesseract is bundled with the Windows installer, so it should work automatically. If you're experiencing OCR issues:

1. **Verify the app was installed correctly**: Reinstall the `.msi` package if needed
2. **Check app logs**: Look for Tesseract-related errors in the terminal or system logs
3. **If bundled Tesseract is missing**: This indicates a build/packaging issue. Please [open a GitHub Issue](https://github.com/lehendo/sovereign/issues) with details

**Note:** If you're building from source on Windows, you need to run `.\scripts\setup-tesseract-win.ps1` before building to bundle Tesseract.

**Linux:**
If you installed via `.deb` package, Tesseract should be auto-installed. If you're using `.AppImage` or experiencing issues:

1. **For .deb installations**: Tesseract should already be installed. If not, reinstall the package:
   ```bash
   sudo apt-get install --reinstall tesseract-ocr
   ```

2. **For .AppImage or manual installations**: Install Tesseract:
   ```bash
   # Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install tesseract-ocr
   
   # Fedora
   sudo dnf install tesseract
   
   # Arch Linux
   sudo pacman -S tesseract
   ```

3. Verify installation:
   ```bash
   tesseract --version
   ```

4. Restart the app

## Permission Issues

### Screen capture not working (macOS)

1. Go to **System Settings → Privacy & Security → Screen Recording**
2. Ensure "Sovereign" (or "Terminal" if running from terminal) is enabled
3. If the app was already running, quit and restart it
4. The app will prompt you to grant permission on first launch

### Screen capture not working (Windows)

1. Go to **Settings → Privacy → Screen recording** (Windows 11)
2. Ensure screen recording is enabled for apps
3. If the app was already running, quit and restart it
4. Windows should prompt automatically when the app first attempts screen capture

### Screen capture not working (Linux)

**X11:**
- Usually works automatically
- If not, ensure your user has access to the X server
- Try running: `xhost +local:` (temporary fix, not recommended for security)

**Wayland:**
- **GNOME**: Go to Settings → Privacy → Screen Sharing, enable for apps
- **KDE**: Usually works automatically
- May need to restart the app after granting permissions

## App Functionality Issues

### No screenshots are being captured

1. **Check permissions**: Ensure screen recording permission is granted (see above)
2. **Check Tesseract**: Verify Tesseract is installed and accessible
3. **Check app logs**: Look for error messages in the terminal (if running from source) or system logs
4. **Restart the app**: Sometimes a restart resolves permission or initialization issues

### Search not working

1. **OCR-only mode**: Basic text search should work if Tesseract is installed
2. **Semantic search**: If semantic search isn't working:
   - Verify the embedding model files are downloaded (see [INSTALL.md](INSTALL.md))
   - Check that files are in the correct Hugging Face cache directory
   - Restart the app after downloading the model

### App crashes or freezes

1. **Check system resources**: Ensure you have enough RAM and disk space
2. **Check logs**: Look for error messages in terminal or system logs
3. **Clear database**: If the database is corrupted, you can delete it:
   - macOS: `~/Library/Application Support/com.sovereign.app/sovereign.db`
   - Windows: `%APPDATA%\com.sovereign.app\sovereign.db`
   - Linux: `~/.local/share/com.sovereign.app/sovereign.db`
4. **Reinstall**: As a last resort, uninstall and reinstall the app

### High CPU or memory usage

1. **Normal behavior**: The app uses minimal resources (<1% CPU typically)
2. **If unusually high**:
   - Check if many screenshots are being processed
   - Restart the app
   - Check for background processes that might be interfering

## Development Issues

### Build errors

1. **Check dependencies**: Ensure all system dependencies are installed (see [LOCALDEV.md](LOCALDEV.md))
2. **Check versions**: Verify Node.js (20.19.0+ or 22.12.0+) and Rust (latest stable)
3. **Clean build**: Try cleaning the build:
   ```bash
   cd src-tauri
   cargo clean
   cd ..
   npm run tauri build
   ```

### Hot reload not working

1. **Frontend changes**: Should reload automatically via Vite HMR
   - Check terminal for Vite errors
   - Ensure the dev server is running
2. **Rust changes**: Always require a full app restart
   - Stop the app (Ctrl+C)
   - Run `npm run tauri dev` again

## Getting Help

If you're still experiencing issues:

1. **Check existing issues**: Search [GitHub Issues](https://github.com/lehendo/sovereign/issues) for similar problems
2. **Create a new issue**: Include:
   - Your operating system and version
   - Steps to reproduce the problem
   - Error messages (if any)
   - App version

For security vulnerabilities, please use [GitHub Security Advisories](https://github.com/lehendo/sovereign/security/advisories/new) instead of opening a public issue.
