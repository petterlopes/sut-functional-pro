# Security Policy

This file describes the security and disclosure policies for this repository and operational guidance for handling secrets and vulnerabilities.

## Reporting a Vulnerability
- If you discover a security issue in this project, please report it privately. If this repository is in an organization, use their security contact or private issue tracker.
- For public forks, avoid posting PoCs in public channels. Share a minimal reproduction privately with maintainers.

## Responsible Disclosure
- Give maintainers 90 days to respond and remediate for critical issues unless otherwise agreed. If the issue is actively being exploited, consider faster disclosure and coordinating with the maintainers.

## Handling Secrets
- Do not commit secrets (API keys, private keys, credentials) to Git history.
- Use the Vault integration for runtime secrets. For development, `deploy/vault` runs a dev Vault instance — treat the token `root` as ephemeral and not for production use.
- If secrets are accidentally committed, rotate them immediately and remove them from Git history using tools such as `git filter-repo`.

## Dependency and Supply-chain Security
- Run `cargo audit` for Rust dependencies and `pnpm audit`/`npm audit` for frontend dependencies.
- Consider pinning direct dependencies and enabling Dependabot for automatic security PRs.

## Operational Hardening Checklist
- Ensure TLS is enforced in front of Keycloak and API in production.
- Ensure Vault runs in production mode with TLS and secure storage backends.
- Enforce minimum log retention for audit trails and secure storage of logs.
- Configure rate-limiting on authentication and ingestion endpoints.

## Contact
- Use the project's internal security contact or the repository owner for private reports.
