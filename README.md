# Xiaoyi

[![CI](https://github.com/miruamel/Xiaoyi/actions/workflows/ci.yml/badge.svg)](https://github.com/miruamel/Xiaoyi/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.99--nightly-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.12%2B-blue.svg)](https://www.python.org)
[![TypeScript](https://img.shields.io/badge/typescript-5.0%2B-blue.svg)](https://www.typescriptlang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](LICENSE)

**Xiaoyi is an open-source AI agent framework for building autonomous agents.**

It lets you build agents that plan, review, evaluate, and recover from errors with minimal supervision. A Rust core handles execution, with Python and TypeScript SDKs for scripting and integration.

## What you can build

- Autonomous coding agents that write, review, and iterate on code
- Workflow pipelines with DAG-based task orchestration
- Agents that use multiple LLM providers (OpenAI, Anthropic, Ollama)
- Systems with persistent memory, tool use, and quality gates

## Quick start

### Rust

```toml
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
        messages: vec![ChatMessage {
            role: MessageRole::User,
            content: "Hello!".into(),
            name: None,
        }],
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
pip install -U pip maturin
cd src/rust && maturin develop --release -m Cargo.toml
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
cd src/typescript && npm ci && npm run build
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

- Rust 1.99 nightly
- Python 3.12+
- Node.js 20+

### Build

```bash
cd src/rust && cargo build --features python
```

### Docs

```bash
cd website && npm install && npm start
```

## License

Dual-licensed under **MIT OR Apache-2.0**.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
