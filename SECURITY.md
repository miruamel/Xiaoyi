# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

Please **DO NOT** file a public issue for security vulnerabilities.

Email security concerns to: miruamel@example.com (placeholder; replace with actual contact)

Include:
- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact

We aim to acknowledge reports within 48 hours and provide a fix timeline within 7 days.

## Disclosure Policy

- We follow coordinated disclosure.
- We will credit reporters in the fix commit (unless anonymity is requested).
- Critical issues are patched within 24 hours; high severity within 7 days.

## Security Best Practices

When using Xiaoyi:

- Store secrets in the encrypted vault (`XIAOYI_VAULT_KEY` env var, 32 bytes).
- Rotate API keys regularly.
- Run `cargo audit`, `npm audit`, and `pip-audit` in your CI.
- Keep dependencies up to date via Dependabot PRs.
- Never log vault contents or API keys; metadata only.