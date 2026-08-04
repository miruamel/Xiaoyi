"""LLM client abstraction for multiple providers.

Path: xiaoyi.llm.client

Layer hierarchy:
- 0: llm
- 1: client
- 2: openai/anthropic/local/ollama
- 3: request/response/stream

Unified interface for LLM API calls with provider-agnostic contract.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum
from typing import Any

import httpx


class MessageRole(Enum):
    SYSTEM = "system"
    USER = "user"
    ASSISTANT = "assistant"
    TOOL = "tool"


@dataclass
class ChatMessage:
    role: MessageRole
    content: str
    name: str | None = None


@dataclass
class ChatRequest:
    model: str
    messages: list[ChatMessage]
    temperature: float | None = None
    max_tokens: int | None = None
    stream: bool = False


@dataclass
class ChatChoice:
    index: int
    message: ChatMessage
    finish_reason: str | None = None


@dataclass
class Usage:
    prompt_tokens: int
    completion_tokens: int
    total_tokens: int


@dataclass
class ChatResponse:
    id: str
    model: str
    choices: list[ChatChoice]
    usage: Usage | None = None


class LlmClient(ABC):
    """Unified LLM client trait."""

    @abstractmethod
    async def chat(self, request: ChatRequest) -> ChatResponse:
        pass

    @abstractmethod
    async def chat_stream(self, request: ChatRequest) -> Any:
        pass

    @property
    @abstractmethod
    def provider_name(self) -> str:
        pass


# OpenAI-compatible client
class OpenAiClient(LlmClient):
    def __init__(self, api_key: str, base_url: str = "https://api.openai.com/v1"):
        self.api_key = api_key
        self.base_url = base_url
        self._client = httpx.AsyncClient()

    async def chat(self, request: ChatRequest) -> ChatResponse:
        url = f"{self.base_url}/chat/completions"
        headers = {"Authorization": f"Bearer {self.api_key}"}
        payload = {
            "model": request.model,
            "messages": [{"role": m.role.value, "content": m.content} for m in request.messages],
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "stream": request.stream,
        }
        resp = await self._client.post(url, headers=headers, json=payload)
        resp.raise_for_status()
        data = resp.json()
        return ChatResponse(
            id=data["id"],
            model=data["model"],
            choices=[ChatChoice(index=c["index"], message=ChatMessage(role=MessageRole(c["message"]["role"]), content=c["message"]["content"])) for c in data["choices"]],
            usage=Usage(**data["usage"]) if "usage" in data else None,
        )

    async def chat_stream(self, request: ChatRequest) -> Any:
        raise NotImplementedError("stream not yet implemented")

    @property
    def provider_name(self) -> str:
        return "openai"


# Anthropic client
class AnthropicClient(LlmClient):
    def __init__(self, api_key: str, base_url: str = "https://api.anthropic.com/v1"):
        self.api_key = api_key
        self.base_url = base_url
        self._client = httpx.AsyncClient()

    async def chat(self, request: ChatRequest) -> ChatResponse:
        raise NotImplementedError("anthropic chat not yet implemented")

    async def chat_stream(self, request: ChatRequest) -> Any:
        raise NotImplementedError("stream not yet implemented")

    @property
    def provider_name(self) -> str:
        return "anthropic"


# Ollama client
class OllamaClient(LlmClient):
    def ____(self, base_url: str = "http://localhost:11434"):
        self.base_url = base_url
        self._client = httpx.AsyncClient()

    async def chat(self, request: ChatRequest) -> ChatResponse:
        url = f"{self.base_url}/api/chat"
        payload = {
            "model": request.model,
            "messages": [{"role": m.role.value, "content": m.content} for m in request.messages],
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "stream": request.stream,
        }
        resp = await self._client.post(url, json=payload)
        resp.raise_for_status()
        data = resp.json()
        return ChatResponse(
            id=data.get("id", ""),
            model=data.get("model", ""),
            choices=[ChatChoice(index=0, message=ChatMessage(role=MessageRole.ASSISTANT, content=data.get("message", {}).get("content", "")))],
        )

    async def chat_stream(self, request: ChatRequest) -> Any:
        raise NotImplementedError("stream not yet implemented")

    @property
    def provider_name(self) -> str:
        return "ollama"