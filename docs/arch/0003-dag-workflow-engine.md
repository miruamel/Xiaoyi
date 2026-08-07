# ADR 0003: DAG-based Workflow Engine

## Status
Accepted

## Context
AI agent workflows require complex execution patterns: parallel branches, conditional execution, retries, human-in-the-loop, and dynamic graph modification. Traditional linear pipelines are insufficient.

## Decision
We implement a **Directed Acyclic Graph (DAG) based workflow engine** with the following characteristics:

### Core Concepts

1. **Nodes**: Units of work (LLM calls, tool invocations, sub-workflows, conditions)
2. **Edges**: Dependencies and data flow between nodes
3. **Graph**: Complete workflow definition with validation
4. **Executor**: Runtime that schedules and executes nodes topologically

### Node Types

```rust
enum NodeKind {
    LlmCall,           // LLM inference
    ToolCall,          // External tool invocation
    SubWorkflow,       // Nested workflow
    Condition,         // Branch based on condition
    Parallel,          // Fork/join parallel execution
    HumanInTheLoop,    // Wait for human input
    Transform,         // Data transformation
    Start,             // Entry point
    End,               // Exit point
}
```

### Edge Types

```rust
enum EdgeKind {
    Data,      // Pass output as input
    Control,   // Execution dependency
    Conditional, // Execute based on condition
    Error,     // Error handling path
}
```

### Execution Model

1. **Topological Sort**: Determine execution order
2. **Cycle Detection**: Reject cyclic graphs at build time
3. **Parallel Execution**: Independent branches run concurrently
4. **Checkpointing**: Save state for resumability
5. **Observability**: Event stream for monitoring

### API Design

```rust
// Builder pattern for graph construction
let graph = DagGraph::builder()
    .add_node("llm1", NodeKind::LlmCall, config1)
    .add_node("tool1", NodeKind::ToolCall, config2)
    .add_edge("llm1", "tool1", EdgeKind::Data)
    .build()?;

// Execute with context
let result = executor.execute(graph, context).await?;
```

## Consequences

### Positive
- **Flexibility**: Supports arbitrary workflow patterns
- **Composability**: Workflows as nodes enable recursion
- **Observability**: Full visibility into execution graph
- **Resumability**: Checkpoint/restart from any node
- **Testability**: Graph structure can be validated statically

### Negative
- **Complexity**: More complex than linear pipelines
- **Overhead**: Graph scheduling adds latency
- **Debugging**: Non-linear execution harder to trace

### Mitigations
- Visual graph debugger (planned)
- Comprehensive logging and tracing
- Simple API for common patterns (sequential, parallel, branch)
- Cycle detection at build time

## Implementation Layers (Vertical)

```
workflow/
  dag/
    graph/         # Layer 2: Graph structure & validation
    node/          # Layer 3: Node definitions & metadata
    edge/          # Layer 3: Edge types & data flow
    cycle/         # Layer 3: Cycle detection algorithms
    executor/      # Layer 2: Runtime execution engine
    scheduler/     # Layer 3: Topological scheduling
    checkpoint/    # Layer 3: State persistence
    events/        # Layer 3: Execution event stream
```

## Related
- ADR 0001: Deep Vertical Architecture
- ADR 0004: STM/LTM Memory Architecture