use xiaoyi::{
    LlmClient, ChatRequest, ChatMessage, MessageRole, ChatResponse, ChatChoice, Usage,
    AgentBuilder, ConfigBuilder, ConfigSource, Result
};
use futures::stream;
use std::pin::Pin;
use std::collections::HashMap;
use serde_json::Value;

/// Mock LLM client for demonstration.
struct MockClient;

/// Simple in-memory config source for demonstration.
struct MockConfigSource {
    data: HashMap<String, Value>,
}

impl MockConfigSource {
    fn new() -> Self {
        let mut data = HashMap::new();
        data.insert("llm.provider".to_string(), Value::String("mock".to_string()));
        data.insert("llm.model".to_string(), Value::String("mock-model".to_string()));
        data.insert("llm.temperature".to_string(), Value::Number(serde_json::Number::from_f64(0.7).unwrap()));
        Self { data }
    }
}

impl ConfigSource for MockConfigSource {
    fn load(&self) -> Result<HashMap<String, Value>> {
        Ok(self.data.clone())
    }

    fn clone_box(&self) -> Box<dyn ConfigSource> {
        Box::new(self.clone())
    }
}

impl Clone for MockConfigSource {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

impl std::fmt::Debug for MockConfigSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockConfigSource").finish()
    }
}

#[async_trait::async_trait]
impl LlmClient for MockClient {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let response = ChatResponse {
            id: "mock-response".into(),
            model: request.model.clone(),
            choices: vec![ChatChoice {
                index: 0,
                message: ChatMessage {
                    role: MessageRole::Assistant,
                    content: format!("Mock response to: {}", request.messages.last().map(|m| &m.content).unwrap_or(&"".to_string())),
                    name: None,
                },
                finish_reason: Some("stop".into()),
            }],
            usage: Some(Usage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            }),
        };
        Ok(response)
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> Result<Pin<Box<dyn futures::Stream<Item = Result<ChatResponse>> + Send + Unpin>>> {
        // For mock, just return a stream with one response
        let response = self.chat(request).await?;
        let stream = stream::iter(vec![Ok(response)]);
        Ok(Box::pin(stream))
    }

    fn provider_name(&self) -> &'static str {
        "mock"
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration using mock source
    let config = ConfigBuilder::new()
        .add_source(MockConfigSource::new())
        .build()?;

    // Create LLM client (using mock for demonstration)
    let client = MockClient;

    // Build agent
    let agent = AgentBuilder::new(config.clone())
        .name("basic-agent")
        .model("mock-model")
        .build()?;

    println!("Agent created: {}", agent.name);
    println!("Model: {}", agent.model);

    // Simple chat
    let request = ChatRequest {
        model: "mock-model".to_string(),
        messages: vec![
            ChatMessage {
                role: MessageRole::System,
                content: "You are a helpful AI assistant.".to_string(),
                name: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: "Hello! What is Xiaoyi?".to_string(),
                name: None,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(500),
        stream: false,
    };

    let response = client.chat(request).await?;
    println!("Response: {}", response.choices[0].message.content);

    Ok(())
}