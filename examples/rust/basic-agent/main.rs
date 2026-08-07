use xiaoyi::llm::client::{OpenAiClient, LlmClient, ChatRequest, ChatMessage, MessageRole};
use xiaoyi::builder::{AgentBuilder, AgentHandle};
use xiaoyi::Config;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::builder()
        .set("llm.provider", "openai")
        .set("llm.model", "gpt-4o-mini")
        .set("llm.temperature", 0.7)
        .build()?;

    // Create LLM client
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = OpenAiClient::new(api_key);

    // Build agent
    let agent = AgentBuilder::new(config.clone())
        .name("basic-agent")
        .model("gpt-4o-mini")
        .system_prompt("You are a helpful AI assistant.")
        .build()?;

    println!("Agent created: {}", agent.name());
    println!("Model: {}", agent.model());

    // Simple chat
    let request = ChatRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            ChatMessage {
                role: MessageRole::System,
                content: "You are a helpful AI assistant.".to_string(),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: "Hello! What is Xiaoyi?".to_string(),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(500),
        stream: false,
        tools: None,
        tool_choice: None,
        response_format: None,
    };

    let response = client.chat(request).await?;
    println!("Response: {}", response.choices[0].message.content);

    Ok(())
}