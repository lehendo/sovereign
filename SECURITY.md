# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in Sovereign, please report it by:
- Opening a GitHub Security Advisory (preferred)
- Emailing the maintainer directly

**Please do not open public issues for security vulnerabilities.**

## Security Considerations

### Screen Capture Permissions

Sovereign requires screen recording permissions to function. This grants the application access to:
- All visible content on your primary display
- Text, images, and any sensitive information displayed on screen
- Passwords, banking information, private messages if visible when captured

**Important:**
- Screenshots are stored unencrypted on your local disk
- Anyone with access to your computer can view captured screenshots
- Use at your own risk

### Data Privacy

- **All data stays local:** No telemetry, no cloud uploads, no network requests for data
- **Storage location:** Screenshots are saved to your OS's application data directory
- **No encryption:** Screenshots are currently stored as plain .webp files
- **No access controls:** Any user on your system can read the screenshots
- **Privacy Guards Active:**
  - Window blacklist prevents capture of password managers and private browsing
  - Auto-deletion removes data older than 14 days
  - See "Privacy Guards" section below

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

3. **Active Privacy Features:**
   - Window blacklisting for sensitive apps
   - Auto-deletion after 14 days
   - Privacy status indicator in UI

4. **Future Security Features (Planned):**
   - Encryption at rest
   - User-configurable blacklist
   - Per-window privacy settings
   - User authentication for screenshot access

## Privacy Guards

**Window Blacklist - Fully Functional:**
The app automatically skips recording when it detects sensitive applications:

- **Protected Applications:**
  - Password Managers: Bitwarden, 1Password, KeePass, LastPass
  - Private Browsing: Incognito, InPrivate, Private Browsing
  - Privacy Tools: Tor Browser

- **Implementation:**
  - Uses native system commands for maximum stability (AppleScript on macOS, PowerShell on Windows, xdotool on Linux)
  - No external FFI libraries that could crash
  - Matches against both application name and window title
  - Case-insensitive matching

- **Behavior:**
  - When a blacklisted window is detected, the capture is skipped
  - Terminal logs: "Privacy Guard triggered: Window title contains '[term]'"
  - No screenshot saved, no database entry created
  - Completely transparent to the user

**Auto-Deletion Policy - Fully Functional:**
- Data older than 14 days is automatically deleted on app startup
- Includes: Database records, OCR text, embeddings, and image files
- Helps maintain privacy and manage storage space
- Verified working on macOS, Windows, and Linux

**Privacy Status UI:**
- Shield icon indicates Privacy Guard is active
- Statistics show current retention policy (14 days)
- Visible confirmation that protections are in place

## Known Limitations

- No password protection on stored screenshots
- No encryption at rest
- Blacklist configuration is hardcoded (user configuration planned for future)
- Requires AppleScript permissions on macOS (automatically granted)
- Timestamps in filenames could reveal user activity patterns
- No per-window granular control (future feature)

## Dependencies

We use automated dependency scanning and will address security vulnerabilities promptly:
- npm dependencies are checked with `npm audit`
- Rust dependencies follow security best practices
- All dependencies are pinned to specific versions

## Security Updates

Security patches will be released as soon as possible after discovery. Users are strongly encouraged to:
- Watch this repository for security updates
- Keep Sovereign updated to the latest version
- Review the changelog before updating

## License

This software is provided "AS IS" without warranty of any kind. See LICENSE file for details.
