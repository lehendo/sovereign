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
- **No encryption (Phase 1):** Screenshots are currently stored as plain .webp files
- **No access controls (Phase 1):** Any user on your system can read the screenshots

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

3. **Future Security Features (Planned):**
   - Encryption at rest (Phase 3)
   - Window blacklisting for sensitive apps (Phase 6)
   - Configurable retention policies (Phase 6)
   - User authentication for screenshot access

## Known Limitations

- Phase 1 has no privacy controls - all screen content is captured
- No password protection on stored screenshots
- No automatic deletion of sensitive content
- Timestamps in filenames could reveal user activity patterns

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

