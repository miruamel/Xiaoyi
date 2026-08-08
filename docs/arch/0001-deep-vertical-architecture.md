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

### Implemented Layer Structure

```
core/
  config/                    # Layer 1: Configuration domain
    source/                  # Layer 2: Source abstraction
      env/                   # Layer 3: Environment variables
      file/                  # Layer 3: File-based config
      vault/                 # Layer 3: Secret management
        aes/                 # Layer 4: AES implementation
        decrypt/             # Layer 4: Decryption
        encrypt/             # Layer 4: Encryption
        key/                 # Layer 4: Key management
  error/                     # Layer 1: Error domain
    context/                 # Layer 2: Error context helpers
  result/                    # Layer 1: Result domain
    handler/                 # Layer 2: Status handlers
    status/                  # Layer 2: Status codes

domain/
  token/                     # Layer 1: Token domain
    primitive/               # Layer 2: Primitive types
      bool/                  # Layer 3: Boolean
      bytes/                 # Layer 3: Bytes/base64
      float/                 # Layer 3: Float
      int/                   # Layer 3: Integer
      string/                # Layer 3: String
    syntax/                  # Layer 2: Syntax tokens
      keyword/               # Layer 3: Keywords
      operator/              # Layer 3: Operators
      punctuation/           # Layer 3: Punctuation

workflow/
  dag/                       # Layer 1: Workflow domain
    graph/                   # Layer 2: Graph model
    node/                    # Layer 2: Node model
    edge/                    # Layer 2: Edge model

memory/
  stm/                       # Layer 1: Short-term memory
    cache/                   # Layer 2: LRU cache
  ltm/                       # Layer 1: Long-term memory
    vector/                  # Layer 2: Vector store
    graph/                   # Layer 2: Knowledge graph

utils/                       # Shared vertical utilities
  env/                       # Environment helpers
  fs/                        # Filesystem helpers
  id/                        # ID generation
  json/                      # JSON helpers
  math/                      # Math helpers
  net/                       # HTTP client config
  retry/                     # Retry configuration
  string/                    # String helpers
  time/                      # Time helpers
  validation/                # Input validation

builder/                     # Layer 1: Builder domain
  ast/                       # Layer 2: AST model
  codegen/                   # Layer 2: Code generation
  formatter/                 # Layer 2: Formatting
  template/                  # Layer 2: Templates
  validator/                 # Layer 2: Validation

orchestrator/                # Layer 1: Orchestration domain
  loop_/                     # Layer 2: Agent loop
  monitor/                   # Layer 2: Execution monitoring
  planner/                   # Layer 2: Goal decomposition
  policy/                    # Layer 2: Decision policies
  recovery/                  # Layer 2: Error recovery

critic/                      # Layer 1: Review domain
  aggregator/                # Layer 2: Meta-critic aggregation
  cache/                     # Layer 2: Semantic cache
    embedding/               # Layer 3: Embedding helper
  large_llm/                 # Layer 2: Large LLM critics
  model_router/              # Layer 2: Model routing
  rules/                     # Layer 2: Fast-path rules
    security/                # Layer 3: Security rules
    style/                   # Layer 3: Style rules
  small_llm/                 # Layer 2: Small LLM critics

evaluator/                   # Layer 1: Evaluation domain
  analysis/                  # Layer 2: Static analysis
  benchmark/                 # Layer 2: Benchmarks
  build/                     # Layer 2: Build checks
    compiler/                # Layer 3: Compiler config
  feedback/                  # Layer 2: Feedback formulation
  gates/                     # Layer 2: Quality gates
  sandbox/                   # Layer 2: Sandbox execution
    container/               # Layer 3: Container config
  test/                      # Layer 2: Test runners
    property/                # Layer 3: Property test config

gateway/                     # Layer 1: Gateway domain
  api/                       # Layer 2: REST/GraphQL API
  cli/                       # Layer 2: Command-line interface
  middleware/                # Layer 2: Middleware
    auth/                    # Layer 3: Authentication
    ratelimit/               # Layer 3: Rate limiting
  route/                     # Layer 2: Routing
  web/                       # Layer 2: Web UI server

knowledge/                   # Layer 1: Knowledge domain
  graph/                     # Layer 2: Knowledge graph
    repo/                    # Layer 3: Git-native repo
      commit/                # Layer 4: Commit snapshots
      scanner/               # Layer 4: Repo scanner
  retrieval/                 # Layer 2: Retrieval
    ranker/                  # Layer 3: Result ranking
  tools/                     # Layer 2: Tool registry
    openapi/                 # Layer 3: OpenAPI schema
      schema/                # Layer 4: Schema parsing
      schema_store/          # Layer 4: Schema storage
    registry/                # Layer 3: Plugin registry
      discovery/             # Layer 4: Tool discovery
  vector/                    # Layer 2: Vector store
    store/                   # Layer 3: Storage backend
      in_memory/             # Layer 4: In-memory store
      remote/                # Layer 4: Remote store

llm/                         # Layer 1: LLM abstraction
  client/                    # Layer 2: Client contract
    anthropic/               # Layer 3: Anthropic provider
    ollama/                  # Layer 3: Ollama provider
    openai/                  # Layer 3: OpenAI provider
    retry/                   # Layer 3: Retry policy
  prompt/                    # Layer 2: Prompt utilities
    template/                # Layer 3: Prompt templating

monitoring/                  # Layer 1: Monitoring domain
  alerts/                    # Layer 2: Alerting
    alert/                   # Layer 3: Alert model
    channel/                 # Layer 3: Delivery channel
    notifier/                # Layer 3: Notifier impl
    rule/                    # Layer 3: Rule engine
  finops/                    # Layer 2: Cost tracking
    budget/                  # Layer 3: Budget model
    cost/                    # Layer 3: Cost estimate
    tracker/                 # Layer 3: Cost tracker
  metrics/                   # Layer 2: Metrics
    counter/                 # Layer 3: Counter metric
    gauge/                   # Layer 3: Gauge metric
    histogram/               # Layer 3: Histogram metric
    registry/                # Layer 3: Metric registry
  tracing/                   # Layer 2: Distributed tracing
    context/                 # Layer 3: Trace context
    exporter/                # Layer 3: Span exporters
      jaeger/                # Layer 4: Jaeger backend
      otlp/                  # Layer 4: OTLP backend
    sampler/                 # Layer 3: Sampling strategy
    span/                    # Layer 3: Span model

resilience/                  # Layer 1: Resilience domain
  bulkhead/                  # Layer 2: Bulkhead isolation
  circuit_breaker/           # Layer 2: Circuit breaker
  fallback/                  # Layer 2: Fallback routing
  health/                    # Layer 2: Health checks
  retry/                     # Layer 2: Retry/backoff
  timeout/                   # Layer 2: Timeout handling
  semaphore/                 # Layer 2: Concurrency limiter
  isolation/                 # Layer 2: Failure isolation
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
