# ADR 0002: Polyglot Core with FFI Bindings

## Status
Accepted

## Context
Xiaoyi needs to provide native-feeling APIs in Rust, Python, and TypeScript while sharing core logic to avoid duplication and ensure consistency.

## Decision
We implement a **Rust core with FFI bindings** approach:

1. **Rust Core** (`xiaoyi-rust`): Single source of truth for all domain logic, types, and algorithms
2. **Python Bindings** (`xiaoyi-py`): PyO3-based bindings exposing Rust types as native Python classes
3. **TypeScript Bindings** (`xiaoyi-ts`): NAPI-based bindings via `napi-rs` for Node.js/Edge runtime

### Architecture

```
┌─────────────────────────────────────────────┐
│           Application Layer                 │
├─────────────┬─────────────┬─────────────────┤
│   Rust      │   Python    │   TypeScript    │
│  (native)   │  (PyO3)     │   (NAPI)        │
├─────────────┴─────────────┴─────────────────┤
│           FFI Boundary                      │
├─────────────────────────────────────────────┤
│           Xiaoyi Core (Rust)                │
│  ┌─────┬─────┬─────┬─────┬─────┬────────┐  │
│  │Config│Domain│Workflow│LLM │Memory│Orchest│  │
│  └─────┴─────┴─────┴─────┴─────┴────────┘  │
└─────────────────────────────────────────────┘
```

### Type Sharing Strategy

- **Core Types**: Defined once in Rust, exposed via FFI
- **Serialization**: Serde (Rust) ↔ Pydantic (Python) ↔ Zod/TypeBox (TypeScript)
- **Error Handling**: Rust `Result<T, E>` → Python exceptions / TypeScript `Result` types
- **Async**: Rust `Future` → Python `asyncio` / TypeScript `Promise`

### Build Process

```bash
# Rust
cargo build --release --features python,nodejs

# Python (via maturin)
cd python && maturin develop --release -m ../rust/Cargo.toml

# TypeScript (via napi-rs)
cd typescript && npm run build
```

## Consequences

### Positive
- **Single Source of Truth**: Logic implemented once, used everywhere
- **Performance**: Rust core runs at native speed in all languages
- **Consistency**: Identical behavior across languages
- **Maintainability**: Bug fixes in core benefit all bindings

### Negative
- **Build Complexity**: Requires Rust toolchain for Python/TS builds
- **FFI Overhead**: Small crossing cost (mitigated by batching)
- **Debugging**: Cross-language debugging is harder
- **Platform Support**: Must compile for each target platform

### Mitigations
- Pre-built wheels for Python (via maturin)
- Pre-built binaries for TypeScript (via napi-rs + GitHub Actions)
- Comprehensive test suite at FFI boundary
- CI builds for all supported platforms

## Implementation Details

### PyO3 (Python)
```rust
#[pyclass]
#[derive(Clone)]
struct PyConfig {
    inner: Config,
}

#[pymethods]
impl PyConfig {
    #[new]
    fn new() -> Self { ... }
    fn get(&self, key: &str) -> Option<String> { ... }
}
```

### NAPI-RS (TypeScript)
```rust
#[napi]
impl Config {
    #[napi(constructor)]
    pub fn new() -> Self { ... }
    #[napi]
    pub fn get(&self, key: String) -> Option<String> { ... }
}
```

## Related
- ADR 0001: Deep Vertical Architecture
- ADR 0005: Unified LLM Client Abstraction