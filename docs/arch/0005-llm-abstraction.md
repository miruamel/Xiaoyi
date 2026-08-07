# ADR 0005: Unified LLM Client Abstraction

## Status
Accepted

## Context
Xiaoyi needs to support multiple LLM providers (OpenAI, Anthropic, Ollama, local models) with a consistent API across all three languages (Rust, Python, TypeScript). Each provider has different:
- Authentication methods
- Request/response formats
- Streaming protocols
- Model capabilities
- Rate limits and error handling

## Decision
We implement a **unified LLM client abstraction** with a trait/interface-based design:

### Core Abstraction

```rust
// Rust trait
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    async fn stream(&self, request: ChatRequest) -> Result<Stream<ChatChunk>>;
    async fn embed(&self, request: EmbedRequest) -> Result<EmbedResponse>;
    fn models(&self) -> Vec<ModelInfo>;
    fn capabilities(&self) -> ClientCapabilities;
}
```

### Provider Implementations

| Provider | Auth | Streaming | Tools | Vision | Embeddings |
|----------|------|-----------|-------|--------|------------|
| OpenAI | Bearer | SSE | ✅ | ✅ | ✅ |
| Anthropic | Bearer | SSE | ✅ | ✅ | ❌ |
| Ollama | None | SSE | ✅ | ❌ | ✅ |
| Local (llama.cpp) | None | WebSocket | ✅ | ❌ | ✅ |

### Request/Response Types (Shared)

```rust
// Unified across all providers
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    stream: bool,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
    response_format: Option<ResponseFormat>,
}

struct ChatMessage {
    role: MessageRole,      // System, User, Assistant, Tool
    content: String,
    name: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
    tool_call_id: Option<String>,
}

struct ChatResponse {
    id: String,
    model: String,
    choices: Vec<ChatChoice>,
    usage: Usage,
    created: u64,
}
```

### Factory Pattern

```rust
// Provider-agnostic client creation
let client = LlmClientFactory::create(
    Provider::OpenAI,
    LlmConfig {
        api_key: env::var("OPENAI_API_KEY")?,
        base_url: None,
        organization: None,
        timeout: Duration::from_secs(60),
        max_retries: 3,
    }
)?;
```

### Streaming Abstraction

```rust
// Unified streaming across providers
let mut stream = client.stream(request).await?;
while let Some(chunk) = stream.next().await {
    match chunk {
        ChatChunk::Content(delta) => print!("{}", delta),
        ChatChunk::ToolCall(call) => handle_tool(call),
        ChatChunk::Done(usage) => println!("Usage: {:?}", usage),
        ChatChunk::Error(e) => return Err(e),
    }
}
```

## Consequences

### Positive
- **Provider Agnostic**: Swap providers without code changes
- **Consistent API**: Same interface in Rust/Python/TypeScript
- **Testability**: Mock trait for unit testing
- **Extensibility**: New providers implement the trait

### Negative
- **Least Common Denominator**: Advanced provider features need extensions
- **Abstraction Overhead**: Small performance cost
- **Feature Parity**: Not all providers support all features

### Mitigations
- Extension traits for provider-specific features
- Capability negotiation at runtime
- Feature flags for optional functionality
- Clear documentation of provider limitations

## Implementation Layers (Vertical)

```
llm/
  client/
    openai/        # Layer 2: OpenAI implementation
      auth/        # Layer 3: Bearer token auth
      chat/        # Layer 3: Chat completion
      stream/      # Layer 3: SSE streaming
      embed/       # Layer 3: Embeddings
      models/      # Layer 3: Model listing
    anthropic/     # Layer 2: Anthropic implementation
      auth/        # Layer 3: Bearer token auth
      chat/        # Layer 3: Messages API
      stream/      # Layer 3: SSE streaming
    ollama/        # Layer 2: Ollama implementation
      chat/        # Layer 3: Chat API
      stream/      # Layer 3: SSE streaming
      embed/       # Layer 3: Embeddings
    local/         # Layer 2: Local model (llama.cpp)
      server/      # Layer 3: Embedded server
      chat/        # Layer 3: WebSocket chat
    factory/       # Layer 2: Client factory
    traits/        # Layer 2: Core traits
    types/         # Layer 2: Shared request/response types
    retry/         # Layer 3: Retry policies
    rate_limit/    # Layer 3: Rate limiting
```

## Related
- ADR 0001: Deep Vertical Architecture
- ADR 0002: Polyglot Core with FFI Bindings