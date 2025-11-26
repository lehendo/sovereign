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
- Use at your own risk - this is early-stage software

### Data Privacy

- **All data stays local:** No telemetry, no cloud uploads, no network requests for data
- **Storage location:** Screenshots are saved to your OS's application data directory
- **No encryption:** Screenshots are currently stored as plain .webp files
- **No access controls:** Any user on your system can read the screenshots
- **Privacy Guards (Phase 6 - Partial):** 
  - ⚠️ Window blacklist currently disabled due to crash on Intel Macs
  - ✅ Auto-deletion removes data older than 14 days (fully functional)
  - See "Privacy Guards" section below for details

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
   - Window blacklisting for sensitive apps (Phase 6 - Active)
   - Auto-deletion after 14 days (Phase 6 - Active)
   - Privacy status indicator in UI (Phase 6 - Active)

4. **Future Security Features (Planned):**
   - Encryption at rest
   - User-configurable blacklist
   - Per-window privacy settings
   - User authentication for screenshot access

## Privacy Guards (Phase 6 - Partially Active)

**Window Blacklist - ⚠️ Currently Disabled:**
The window detection feature is **temporarily disabled** due to a critical crash bug:

- **Issue**: The `active-win-pos-rs` library (v0.8.4) causes a `null pointer dereference` panic on Intel-based Macs
- **Impact**: The panic cannot be caught or recovered from, causing the entire application to crash
- **Root Cause**: Bug in the library's native macOS (AppKit) bindings when accessing window information
- **Status**: Code is present but commented out in `src-tauri/src/recorder.rs` (lines 295-299)
- **Workaround**: Manually quit the app when working with sensitive data (password managers, private browsing)
- **Future**: Will be re-enabled when a stable cross-platform alternative is found

**Planned Blacklist Terms** (when re-enabled):
- Password Managers: Bitwarden, 1Password, KeePass, LastPass
- Private Browsing: Incognito, InPrivate, Private Browsing
- Privacy Tools: Tor Browser

**Auto-Deletion Policy - ✅ Fully Functional:**
- Data older than 14 days is automatically deleted on app startup
- Includes: Database records, OCR text, embeddings, and image files
- Helps maintain privacy and manage storage space
- Verified working on macOS, Windows, and Linux

**Privacy Status UI:**
- Green shield icon displayed in UI (indicates privacy features present)
- Statistics show current retention policy (14 days)
- Visible confirmation that auto-deletion is active

## Known Limitations

- **Window blacklist disabled** on Intel Macs due to `active-win-pos-rs` crash bug
- No password protection on stored screenshots
- No encryption at rest
- Blacklist configuration is hardcoded (user configuration planned for future)
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

