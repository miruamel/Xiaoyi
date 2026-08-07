# ADR 0004: STM/LTM Memory Architecture

## Status
Accepted

## Context
AI agents require different memory systems for different timescales and access patterns:
- **Short-term**: Conversation context, working memory (milliseconds to hours)
- **Long-term**: Knowledge, facts, episodic memory (days to years)

## Decision
We implement a **dual-memory architecture** with separate Short-Term Memory (STM) and Long-Term Memory (LTM) systems, each optimized for its access patterns.

### STM (Short-Term Memory)

**Characteristics**: Low latency, high throughput, ephemeral, capacity-bounded

**Components**:
- **Cache**: LRU eviction with TTL support (`LruCache<K, V>`)
- **Buffer**: Ring buffer for sequential context (`RingBuffer<T>`)
- **Session**: Conversation-scoped key-value store

**API**:
```rust
// Cache with TTL and LRU
let cache = LruCache::new(1000); // capacity
cache.insert("key", value, ttl_seconds)?;
let value = cache.get("key")?;

// Ring buffer for context window
let mut buffer = RingBuffer::new(4096); // token limit
buffer.push(message);
let context = buffer.iter().collect();
```

### LTM (Long-Term Memory)

**Characteristics**: Durable, queryable, large capacity, semantic search

**Components**:
- **Vector Store**: Embedding-based semantic search (`VectorStore`)
- **Graph Store**: Knowledge graph with entities/relations (`GraphStore`)
- **SQLite Store**: Structured relational memory (`SqliteStore`)

**API**:
```rust
// Vector similarity search
let store = VectorStore::new(embedding_model)?;
store.upsert(vec![("doc1", embedding, metadata)])?;
let results = store.search(query_embedding, top_k)?;

// Knowledge graph
let graph = GraphStore::new()?;
graph.add_entity(Entity::new("person", "Alice"))?;
graph.add_relation("Alice", "knows", "Bob")?;
let neighbors = graph.neighbors("Alice")?;
```

### Memory Consolidation

Background process promotes important STM entries to LTM:

```rust
// Consolidation policy
struct ConsolidationPolicy {
    importance_threshold: f32,
    min_access_count: u32,
    max_age: Duration,
}

// Background task
async fn consolidate(stm: &StmCache, ltm: &LtmStore, policy: &ConsolidationPolicy) {
    for entry in stm.iter() {
        if policy.should_consolidate(entry) {
            ltm.upsert(entry.to_ltm_format()).await?;
            stm.remove(entry.key);
        }
    }
}
```

## Consequences

### Positive
- **Separation of Concerns**: Each memory type optimized for its use case
- **Performance**: STM operations are O(1), LTM uses efficient indexes
- **Scalability**: LTM can grow independently of STM capacity
- **Flexibility**: Multiple LTM backends for different needs

### Negative
- **Complexity**: Two memory systems to maintain
- **Consistency**: Synchronization between STM/LTM
- **Consolidation Logic**: Heuristics for promotion are imperfect

### Mitigations
- Clear APIs with explicit consolidation triggers
- Event-driven consolidation (not periodic)
- Configurable policies per agent type
- Comprehensive tests for memory behaviors

## Implementation Layers (Vertical)

```
memory/
  stm/
    cache/         # Layer 2: LRU cache with TTL
    buffer/        # Layer 2: Ring buffer for context
    session/       # Layer 3: Session-scoped storage
  ltm/
    vector/        # Layer 2: Embedding-based search
      index/       # Layer 3: HNSW/IVF index
      embed/       # Layer 3: Embedding generation
    graph/         # Layer 2: Knowledge graph
      entity/      # Layer 3: Entity management
      relation/    # Layer 3: Relation management
      query/       # Layer 3: Graph traversal
    sqlite/        # Layer 2: Relational storage
      schema/      # Layer 3: Schema definitions
      migration/   # Layer 3: Schema migrations
```

## Related
- ADR 0001: Deep Vertical Architecture
- ADR 0003: DAG-based Workflow Engine