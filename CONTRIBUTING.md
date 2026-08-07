# Contributing to Xiaoyi

Thank you for your interest in contributing to Xiaoyi. This document covers the development workflow, style conventions, and review process.

## Code of Conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md). By participating, you agree to its terms.

## Quick Start

```bash
git clone https://github.com/miruamel/Xiaoyi
cd Xiaoyi
```

### Rust

```bash
cd src/rust
cargo build --features python,nodejs
cargo test --features python,nodejs
cargo clippy --features python,nodejs -- -D warnings
cargo fmt --all
```

### TypeScript

```bash
cd src/typescript
npm ci
npm run build
npm test
npm run lint
```

### Python

```bash
cd src/python
pip install -e ".[dev]"
pytest
ruff check xiaoyi/
ruff format xiaoyi/
mypy xiaoyi/
```

## Project Layout

Xiaoyi is a polyglot project. The Rust crate (`src/rust/`) is the canonical implementation. TypeScript and Python bindings mirror the Rust module hierarchy.

- `src/rust/` — Core implementation + PyO3 (Python) and napi-rs (Node) bindings
- `src/typescript/` — TypeScript facade consuming the Node addon
- `src/python/` — Python package consuming the PyO3 module

When you change a public API in Rust, mirror it in TypeScript and Python. When you add a new module, add it to all three language trees.

## Coding Style

### General

- Every public item carries Doxygen-style documentation (`@module`, `@brief`, `@group`, `@param`, `@return`, `@since`, `@author`, `@example`).
- Use semantic types wherever possible.
- Prefer composition over inheritance.
- Keep modules small and focused.

### Rust

- 2024 edition, MSRV 1.82.
- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings`; warnings are errors.
- Use `tracing` for logging, not `println!`.
- All public functions return `Result<T, XiaoyiError>` or a more specific error.

### TypeScript

- Strict mode (`"strict": true`).
- All exports carry JSDoc with Doxygen tags.
- Prefer `readonly` for immutable data.
- Use discriminated unions for sum types (e.g., `Result<T, E>`).

### Python

- Python 3.10+.
- All public items carry docstrings with Doxygen tags.
- Use `from __future__ import annotations` at module top.
- Type hints are mandatory for new code.

## Pull Request Process

1. Fork the repository and create a branch from `main`.
2. Make your changes in logically-scoped commits.
3. Ensure all tests pass and linters are clean.
4. Update documentation to reflect any API changes.
5. Open a pull request using the provided template.
6. Wait for CI to pass and a maintainer review.

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(scope): add new token classification
fix(rust): handle empty input in lexer
docs(readme): clarify installation steps
test(python): add tests for vault decryption
chore(ci): bump action versions
```

## Release Process

Maintainers cut releases via GitHub Actions when a tag matching `v*.*.*` is pushed. The release workflow builds all three languages, publishes to npm and PyPI, and creates a GitHub Release with notes.

## Questions?

Open a [Discussion](https://github.com/miruamel/Xiaoyi/discussions) or join the development chat.