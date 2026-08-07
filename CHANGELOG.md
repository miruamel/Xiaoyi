# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive CI/CD pipeline (`.github/workflows/ci.yml`)
- Dependabot configuration for Rust, npm, pip, and GitHub Actions
- Pre-commit hooks for rustfmt, clippy, eslint, ruff, mypy, markdownlint, yamllint
- Test suites for Rust (cargo test), TypeScript (vitest), Python (pytest)
- Issue and PR templates
- Security policy, contributing guide, code of conduct
- pyproject.toml for Python (ruff, mypy, pytest config)
- vitest.config.ts and ESLint config for TypeScript
- clippy.toml for Rust

### Changed
- Migrated to unified `src/{rust,typescript,python}/` layout
- Applied Doxygen Universal documentation standard across all three languages

## [0.1.0] - 2025-08-07

### Added
- Initial release
- Rust core with PyO3 (Python) and napi-rs (Node) bindings
- Layered configuration system (defaults < file < env < vault)
- AES-256-GCM encrypted vault for secrets
- LLM client abstraction (async-openai)
- Workflow DAG executor with cycle detection
- LRU cache with TTL for short-term memory
- Lexer with token classification (primitive + syntax kinds)
- Agent builder, orchestrator, gateway (CLI/API/Web)

[Unreleased]: https://github.com/miruamel/Xiaoyi/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/miruamel/Xiaoyi/releases/tag/v0.1.0