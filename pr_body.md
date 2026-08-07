## Summary

This PR applies the **Doxygen Universal** documentation standard across all three languages in the Xiaoyi project (Rust, TypeScript, Python).

### Changes

**Rust** (`src/rust/src/xiaoyi/`):
- 9 modules documented with `@module`, `@brief`, `@group`, `@since`, `@author`, `@see`
- Core modules: `core`, `builder`, `domain`, `gateway`, `lexer`, `llm`, `memory`, `orchestrator`, `workflow`
- Item-level tags: `@param`, `@return`, `@throw`, `@example`, `@security`, `@threadsafe`
- Deep vertical architecture: 7+ nested layers per domain

**TypeScript** (`src/typescript/src/xiaoyi/`):
- JSDoc with Doxygen tags across all modules
- `core/error`, `core/result`, `core/config/*`, `workflow/dag/*`, `memory/stm/*`, `domain/token/*`
- Full hierarchical structure with `@module`, `@group`, `@example`, `@security` (vault)

**Python** (`src/python/src/xiaoyi/`):
- Docstrings with Doxygen tags across all modules
- `core/error`, `core/result`, `core/config/*`, `domain/token/primitive/*`, `domain/token/syntax/*`
- 5-layer primitive hierarchy (int→kind/width/rep/normalize, float/f32/f64, bool, string)
- 4 syntax categories (keyword, operator, delimiter, literal)

### Verification
- Rust: `cargo check --features python,nodejs` ✅ (18 minor warnings only)
- TypeScript: `npm run build` ✅ compiles cleanly
- Python: All core modules import successfully ✅
- Tests: Fixed LRU cache test type mismatches

### Architecture
- Rust-core + bindings: Single Rust implementation with PyO3 (Python) and napi-rs (TypeScript/Node) bindings
- Unified src/ structure: `src/typescript/` and `src/python/` with mirrored module hierarchies
- Rust Cargo features: `python`, `nodejs` for conditional compilation

### Related
- Updates architecture documentation
- Adds .gitignore for build artifacts