# ADR 0001: Deep Vertical Architecture

## Status
Accepted

## Context
Xiaoyi is a polyglot AI agent framework that needs to support multiple languages (Rust, Python, TypeScript) while maintaining a consistent architecture across all implementations. Traditional horizontal/layered architectures lead to:

- Tight coupling between unrelated concerns
- Difficulty in testing individual layers
- Inconsistent abstractions across languages
- Poor separation of domain logic from infrastructure

## Decision
We adopt a **deep vertical architecture** where each domain (core, domain, workflow, llm, memory, orchestration) is organized as a deep hierarchy of nested modules (7+ layers), rather than flat horizontal layers.

### Principles

1. **Vertical Slicing**: Each domain is a complete vertical slice from primitives to application-level abstractions
2. **Layer Depth**: Minimum 7 layers per domain to ensure fine-grained separation
3. **Domain Isolation**: Domains communicate only through well-defined interfaces
4. **Cross-Language Consistency**: Same vertical structure mirrored in all three languages

### Layer Structure Example (Core Config)

```
core/
  config/                    # Layer 1: Domain entry point
    source/                  # Layer 2: Source abstraction
      env/                   # Layer 3: Environment variables
      file/                  # Layer 3: File-based config
        path/                # Layer 4: Path resolution
        absolute/            # Layer 5: Absolute path handling
        unix/                # Layer 5: Unix path semantics
        norm/                # Layer 5: Path normalization
      vault/                 # Layer 3: Secret management
        encrypt/             # Layer 4: Encryption
        decrypt/             # Layer 4: Decryption
        aes/                 # Layer 5: AES implementation
        key/                 # Layer 5: Key management
```

## Consequences

### Positive
- **Testability**: Each layer can be tested in isolation
- **Replaceability**: Individual layers can be swapped without affecting others
- **Clarity**: Clear ownership and responsibility boundaries
- **Extensibility**: New implementations fit naturally into existing layers
- **Cross-Language Parity**: Same mental model across Rust/Python/TypeScript

### Negative
- **Verbosity**: More modules and files to navigate
- **Indirection**: More hops to reach functionality
- **Learning Curve**: Developers must understand vertical vs horizontal thinking

### Mitigations
- Comprehensive documentation and diagrams
- Code generation for boilerplate layers
- IDE tooling for navigation
- Consistent naming conventions

## Implementation Notes

- Rust: Uses `mod` system with `pub use` re-exports at each layer
- Python: Uses package hierarchy with `__init__.py` re-exports
- TypeScript: Uses directory structure with barrel exports (`index.ts`)

## Related
- ADR 0002: Polyglot Core with FFI Bindings
- ADR 0003: DAG-based Workflow Engine