# Xiaoyi

[![CI](https://github.com/miruamel/Xiaoyi/actions/workflows/ci.yml/badge.svg)](https://github.com/miruamel/Xiaoyi/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.12%2B-blue.svg)](https://www.python.org)
[![TypeScript](https://img.shields.io/badge/typescript-5.0%2B-blue.svg)](https://www.typescriptlang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/xiaoyi-rust.svg)](https://crates.io/crates/xiaoyi-rust)
[![PyPI](https://img.shields.io/pypi/v/xiaoyi-py.svg)](https://pypi.org/project/xiaoyi-py/)
[![npm](https://img.shields.io/npm/v/xiaoyi-ts.svg)](https://www.npmjs.com/package/xiaoyi-ts)

**Polyglot AI Agent Framework with Deep Vertical Architecture**

Xiaoyi is a multi-language (Rust, Python, TypeScript) framework for building LLM-powered agents, workflows, and orchestration systems. It features a **deep vertical architecture** with 7+ nested layers per domain, enabling fine-grained separation of concerns from token primitives to application orchestration.

## Architecture Overview

```
xiaoyi/
├── rust/          # Core runtime (performance-critical)
├── python/        # SDK & scripting (AI/ML ecosystem)
├── ts/            # TypeScript SDK (web/edge)
├── docs/          # Architecture docs & ADRs
└── .github/       # CI/CD workflows
```

### Layer Depth (7+ layers)

Each domain follows vertical layering (not flat/horizontal):

```
core/
  config/
    source/
      env/         # Layer 3
      file/
        path/      # Layer 4
        absolute/  # Layer 5
        unix/      # Layer 5
        norm/      # Layer 5
      vault/
        encrypt/   # Layer 4
        decrypt/   # Layer 4
        aes/       # Layer 5
        key/       # Layer 5

domain/
  token/
    primitive/
      int/
        kind/      # Layer 5
        width/     # Layer 5
        rep/       # Layer 5
        normalize/ # Layer 5
    syntax/
      primitive/
        int8/
          kind/    # Layer 5

workflow/
  dag/
    graph/         # Layer 2
    node/          # Layer 3
    edge/          # Layer 3
    cycle/         # Layer 3

llm/
  client/
    openai/        # Layer 2
    anthropic/     # Layer 2
    ollama/        # Layer 2

memory/
  stm/
    cache/         # Layer 2
    buffer/        # Layer 2
  ltm/
    vector/        # Layer 2
    graph/         # Layer 2

orchestration/
  pipeline/
    step/          # Layer 2
    stage/         # Layer 2
  supervisor/
    scheduler/     # Layer 2
    health/        # Layer 2
```

## Features

- **Multi-language**: Rust core + Python/TS SDKs with shared types
- **Deep vertical architecture**: 7+ layers per domain, not flat
- **LLM abstraction**: Unified client for OpenAI, Anthropic, Ollama, local
- **Workflow engine**: DAG-based execution, agents (ReAct, CoT, ToT, RAA)
- **Memory systems**: STM (cache/buffer), LTM (vector/sqlite/graph)
- **Tool registry**: REST, GraphQL, gRPC with schema validation
- **Orchestration**: Pipeline, supervisor, event bus, state machine
- **Polyglot FFI**: PyO3 (Python), NAPI (Node.js/TS) bindings

## Quick Start

### Rust
```toml
# Cargo.toml
[dependencies]
xiaoyi-rust = { git = "https://github.com/miruamel/Xiaoyi", package = "xiaoyi-rust" }
```

```rust
use xiaoyi::llm::client::{OpenAiClient, LlmClient, ChatRequest, ChatMessage, MessageRole};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAiClient::new(std::env::var("OPENAI_API_KEY")?);
    let request = ChatRequest {
        model: "gpt-4o-mini".into(),
        messages: vec![ChatMessage { role: MessageRole::User, content: "Hello!".into(), name: None }],
        temperature: Some(0.7),
        max_tokens: Some(100),
        stream: false,
    };
    let resp = client.chat(request).await?;
    println!("{}", resp.choices[0].message.content);
    Ok(())
}
```

### Python
```bash
pip install xiaoyi-py
```

```python
import os
from xiaoyi.llm.client import OpenAiClient, ChatRequest, ChatMessage, MessageRole

async def main():
    client = OpenAiClient(api_key=os.environ["OPENAI_API_KEY"])
    request = ChatRequest(
        model="gpt-4o-mini",
        messages=[ChatMessage(role=MessageRole.USER, content="Hello!")],
        temperature=0.7,
        max_tokens=100,
    )
    resp = await client.chat(request)
    print(resp.choices[0].message.content)

import asyncio
asyncio.run(main())
```

### TypeScript
```bash
npm install xiaoyi-ts
```

```typescript
import { OpenAiClient, ChatRequest, ChatMessage, MessageRole } from "xiaoyi-ts";

const client = new OpenAiClient({ apiKey: process.env.OPENAI_API_KEY! });
const request: ChatRequest = {
  model: "gpt-4o-mini",
  messages: [{ role: MessageRole.USER, content: "Hello!" }],
  temperature: 0.7,
  maxTokens: 100,
};
const resp = await client.chat(request);
console.log(resp.choices[0].message.content);
```

## Development

### Prerequisites
- Rust 1.75+
- Python 3.12+
- Node.js 20+

### Build All
```bash
# Rust
cd rust && cargo build --workspace

# Python
cd python && pip install -e ".[dev]"

# TypeScript
cd ts && npm ci && npm run build
```

### Test All
```bash
# Rust
cd rust && cargo test --workspace

# Python
cd python && pytest -v

# TypeScript
cd ts && npm run test
```

## License

Dual-licensed under **MIT OR Apache-2.0**.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.