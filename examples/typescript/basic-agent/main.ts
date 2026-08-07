import { OpenAiClient, ChatRequest, ChatMessage, MessageRole } from "xiaoyi-ts";
import { AgentBuilder, AgentHandle } from "xiaoyi-ts";
import { Config } from "xiaoyi-ts";

async function main() {
    // Load configuration
    const config = Config.builder()
        .set("llm.provider", "openai")
        .set("llm.model", "gpt-4o-mini")
        .set("llm.temperature", 0.7)
        .build();

    // Create LLM client
    const apiKey = process.env.OPENAI_API_KEY!;
    const client = new OpenAiClient({ apiKey });

    // Build agent
    const agent = new AgentBuilder(config)
        .name("basic-agent")
        .model("gpt-4o-mini")
        .systemPrompt("You are a helpful AI assistant.")
        .build();

    console.log(`Agent created: ${agent.name()}`);
    console.log(`Model: ${agent.model()}`);

    // Simple chat
    const request: ChatRequest = {
        model: "gpt-4o-mini",
        messages: [
            {
                role: MessageRole.SYSTEM,
                content: "You are a helpful AI assistant.",
            },
            {
                role: MessageRole.USER,
                content: "Hello! What is Xiaoyi?",
            },
        ],
        temperature: 0.7,
        maxTokens: 500,
        stream: false,
    };

    const response = await client.chat(request);
    console.log(`Response: ${response.choices[0].message.content}`);
}

main().catch(console.error);