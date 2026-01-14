# Sovereign

<div align="center">

**Your Digital Memory. Not Microsoft's.**

A privacy-first, local-only alternative to Microsoft Recall. Search everything you've seen on your screen without sending a single pixel to the cloud.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)](https://github.com/lehendo/sovereign/releases/latest)

**[Visit Landing Page](https://lehendo.github.io/sovereign/)** • **[Download Latest Release](https://github.com/lehendo/sovereign/releases/latest)** • **[Security Policy](SECURITY.md)**

</div>

---

## Demo

<!-- To make the video show inline on GitHub:
1. Go to GitHub and edit this README.md
2. Delete the placeholder line below
3. Drag and drop sovereigndemo.mp4 directly into the editor at this location
4. GitHub will auto-upload and generate the proper video embed code
-->


https://github.com/user-attachments/assets/09d26adc-23b4-497f-87a5-a3178b25ad6b

## Why Sovereign?

**Microsoft Recall** sends your screen data to the cloud. **Sovereign** keeps everything on your device.

| Feature | Microsoft Recall | Sovereign |
|---------|-----------------|-----------|
| **Data Storage** | Cloud (Microsoft servers) | 100% Local (your device) |
| **Privacy** | Data sent to Microsoft | Zero network requests |
| **Open Source** | Closed source | Open source (MIT) |
| **Cross-Platform** | Windows only | macOS, Windows, Linux |
| **Performance** | Unknown | <1% CPU, minimal battery |
| **Cost** | Requires Windows 11+ | Free, no requirements |

## Features

- **100% Local** - No cloud, no telemetry, no data collection
- **High Performance** - Written in Rust, uses <1% CPU, minimal battery drain
- **Semantic Search** - Find anything you've seen using natural language
- **Smart Deduplication** - Only saves when your screen actually changes
- **Easy Updates** - Download the latest version directly from GitHub releases
- **Auto-Cleanup** - Automatically deletes data older than 14 days
- **Multi-Monitor Support** - Captures all connected displays
- **Modern UI** - Dark mode with Cmd+K/Ctrl+K search, timeline slider, full-screen viewer

## Getting Started

### Option 1: Use Pre-built App (Recommended)

1. **[Download](https://github.com/lehendo/sovereign/releases/latest)** the latest release for your platform
2. **Install Tesseract OCR** (required for text extraction):
   - macOS: `brew install tesseract` (required)
   - Windows: Bundled with the app (no installation needed)
   - Linux: Auto-installed with `.deb` package (manual install needed for `.AppImage`)
3. **Launch** the app and grant screen recording permission
4. **Optional**: [Enable semantic search](INSTALL.md#step-3-optional-enable-semantic-search) for natural language queries

**[Full Installation Guide →](INSTALL.md)**

### Option 2: Build from Source

1. **Clone** the repository: `git clone https://github.com/lehendo/sovereign.git`
2. **Install dependencies** (Node.js, Rust, Tesseract, system libraries)
3. **Run** `npm run tauri dev` to start development

**[Local Development Guide →](LOCALDEV.md)**

## Important Security Notice

**This software captures ALL visible content on your screen, including sensitive information.**

- Screenshots are stored **UNENCRYPTED** on your local disk
- Anyone with physical or remote access to your computer can view screenshots
- Use at your own risk and ensure your device is properly secured

See [SECURITY.md](SECURITY.md) for detailed security considerations.

## How It Works

1. **Smart Capture** - Takes a screenshot every 30 seconds, but only saves when the screen changes
2. **Multi-Monitor** - Automatically detects and captures all connected displays
3. **Text Extraction** - Uses Tesseract OCR to extract all visible text
4. **Semantic Indexing** - Generates AI embeddings for natural language search (optional)
5. **Storage** - Saves compressed screenshots and metadata locally
6. **Search** - Query in plain English to find anything you've seen
7. **Auto-Cleanup** - Removes data older than 14 days automatically

## Troubleshooting

If you encounter issues, please consult the [Troubleshooting Guide](TROUBLESHOOTING.md) or [open a GitHub Issue](https://github.com/lehendo/sovereign/issues).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an issue on this GitHub repository.

Before contributing, please:
- Check existing issues and pull requests to avoid duplicates
- Follow the existing code style and conventions
- Test your changes thoroughly
- Update documentation if needed

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

<div align="center">

Made with ❤️ for privacy-conscious users

</div>
