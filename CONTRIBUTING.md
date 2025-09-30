# Contributing

Welcome — this project is a prototype and we welcome improvements. Please follow these basic guidelines.

## Developer workflow
- Use feature branches and open PRs against `main`.
- Keep changes small and focused. Add tests for new behavior.

## Local checks (recommended)
- Rust: `cargo fmt && cargo clippy -- -D warnings`
- JS/TS: `pnpm --prefix frontend lint` (add ESLint if missing)
- Security: run `cargo audit` and `pnpm audit` before opening PRs

## Pre-commit
- Consider installing `pre-commit` hooks to prevent accidental commits of secrets and to run formatters. Example checks:
  - `detect-secrets` or `git-secrets` to scan for high-entropy strings
  - `cargo fmt` and `pnpm format`

## Release / Deployment notes
- Do not use dev Vault tokens, admin passwords, or local-only credentials in production.
