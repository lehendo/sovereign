# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability, please report it by opening a [GitHub Security Advisory](https://github.com/lehendo/sovereign/security/advisories/new).

**Please do not open public issues for security vulnerabilities.**

## Important Security Notice

**This software captures ALL visible content on your screen, including sensitive information.**

### What This Means

- **Screen Recording Access:** The app requires screen recording permissions and can capture everything visible on your display
- **Unencrypted Storage:** Screenshots are stored as plain .webp files on your local disk
- **No Access Controls:** Anyone with access to your computer can view captured screenshots

### Data Storage

- **Location:** Screenshots saved to your OS application data directory
  - macOS: `~/Library/Application Support/com.sovereign.app/screenshots/`
  - Windows: `%APPDATA%\com.sovereign.app\screenshots\`
  - Linux: `~/.local/share/com.sovereign.app/screenshots/`
- **Auto-Deletion:** Data older than 14 days is automatically removed
- **Local Only:** No data is sent to the cloud or any external servers

### Recommendations

**Do NOT use this software if you:**
   - Work with classified or highly sensitive information
   - Handle financial data, healthcare records, or legal documents
   - Are in a shared computer environment

**Best Practices:**
   - Keep your computer physically secure
   - Use full disk encryption on your OS
   - Be aware of what's on your screen when the app is running
   - Regularly review and delete old screenshots if needed

## Updates

- Releases are cryptographically signed
- Security patches are released promptly after discovery
- Keep the app updated to the latest version

## License

This software is provided "AS IS" without warranty of any kind. See LICENSE file for details.
