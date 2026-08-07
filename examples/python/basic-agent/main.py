import os
import asyncio
from xiaoyi.llm.client import OpenAiClient, ChatRequest, ChatMessage, MessageRole
from xiaoyi.builder import AgentBuilder, AgentHandle
from xiaoyi.core.config import Config

async def main():
    # Load configuration
    config = Config.builder() \
        .set("llm.provider", "openai") \
        .set("llm.model", "gpt-4o-mini") \
        .set("llm.temperature", 0.7) \
        .build()

    # Create LLM client
    api_key = os.environ["OPENAI_API_KEY"]
    client = OpenAiClient(api_key=api_key)

    # Build agent
    agent = AgentBuilder(config) \
        .name("basic-agent") \
        .model("gpt-4o-mini") \
        .system_prompt("You are a helpful AI assistant.") \
        .build()

    print(f"Agent created: {agent.name()}")
    print(f"Model: {agent.model()}")

    # Simple chat
    request = ChatRequest(
        model="gpt-4o-mini",
        messages=[
            ChatMessage(
                role=MessageRole.SYSTEM,
                content="You are a helpful AI assistant."
            ),
            ChatMessage(
                role=MessageRole.USER,
                content="Hello! What is Xiaoyi?"
            ),
        ],
        temperature=0.7,
        max_tokens=500,
        stream=False,
    )

    response = await client.chat(request)
    print(f"Response: {response.choices[0].message.content}")

if __name__ == "__main__":
    asyncio.run(main())