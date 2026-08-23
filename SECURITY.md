# Security Policy

## Supported Versions

Security updates are actively maintained for the latest stable release of AURA.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## Reporting a Vulnerability

The AURA engineering team takes security and reliability seriously. If you discover a security vulnerability, memory safety issue, or potential compromise in AURA, please report it responsibly.

### How to Report
**Do not file a public GitHub issue for security vulnerabilities.**

Instead, please send a report to the maintainers via:
- **Email**: `security@grevix.ai`
- **Private Advisory**: Use GitHub's [Private Vulnerability Reporting](https://github.com/Grevix/aura/security/advisories/new) feature.

### What to Include
To help us triage and resolve the issue quickly, please include:
1. Description of the vulnerability and its potential impact.
2. Steps to reproduce the issue (including sample GGUF artifacts, CLI commands, or environment configurations).
3. Affected platform(s) (Linux, Windows, macOS).
4. Proposed fix or mitigation (if available).

### Response Timeline
- **Acknowledgement**: Within 48 hours of report submission.
- **Triage & Assessment**: Within 5 business days.
- **Fix & Disclosure**: Critical vulnerabilities will be patched in a security release within 14 days, followed by a coordinated public advisory.

## Security Practices in AURA

- **No Memory Safety Compromises**: All crates compile with strict Clippy lints (`-D warnings`).
- **Automated Dependency Audits**: Continuous Integration runs `cargo audit` to block known advisory vulnerabilities (`RUSTSEC`).
- **Strict License & Security Gate**: `cargo deny` validates license cleanliness and banned dependencies on every pull request.
