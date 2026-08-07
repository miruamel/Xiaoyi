"""
# Error Module

`error` provides error types and handling for the Xiaoyi framework.

Path: `xiaoyi.core.error`

- Layer 0: `core`
- Layer 1: `error` — error types and handling.

@module xiaoyi.core.error
@brief Error types and handling
@group Core
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config
@see xiaoyi.core.result
"""

from enum import Enum
from typing import Any, Dict, Optional
from dataclasses import dataclass, field


class ErrorKind(str, Enum):
    """
    Error kind classification.

    @brief Classification of error types
    @group Core
    @since 0.1.0
    """

    #: Syntax error during parsing or compilation.
    SYNTAX = "syntax"
    #: Parse error for structured data (JSON, TOML, etc.).
    PARSE = "parse"
    #: Runtime execution error.
    RUNTIME = "runtime"
    #: I/O error (file, network, etc.).
    IO = "io"
    #: Authentication/authorization failure.
    AUTH = "auth"
    #: Policy violation (rate limit, quota, etc.).
    POLICY = "policy"
    #: LLM provider error.
    LLM = "llm"
    #: Memory system error (STM/LTM).
    MEMORY = "memory"
    #: Tool execution error.
    TOOL = "tool"
    #: Workflow DAG execution error.
    WORKFLOW = "workflow"
    #: Configuration error.
    CONFIG = "config"
    #: State management error.
    STATE = "state"


@dataclass
class XiaoyiError(Exception):
    """
    Structured error with metadata for recovery decisions.

    @brief Structured error with context for error handling
    @group Core
    @since 0.1.0
    @see ErrorKind

    @example
    ```python
    error = create_error(ErrorKind.CONFIG, "Failed to load config", {"path": "./config.toml"})
    ```
    """

    kind: ErrorKind
    message: str
    meta: Dict[str, str] = field(default_factory=dict)

    def __str__(self) -> str:
        return f"[{self.kind.value}] {self.message}"

    def with_meta(self, key: str, value: str) -> "XiaoyiError":
        """Add metadata to error."""
        self.meta[key] = value
        return self


def create_error(
    kind: ErrorKind,
    message: str,
    meta: Optional[Dict[str, str]] = None
) -> XiaoyiError:
    """
    Create a new XiaoyiError.

    @param kind Error kind
    @param message Error message
    @param meta Optional metadata
    @return New XiaoyiError instance
    @since 0.1.0
    @example
    ```python
    error = create_error(ErrorKind.CONFIG, "Failed to load config", {"path": "./config.toml"})
    ```
    """
    return XiaoyiError(kind=kind, message=message, meta=meta or {})


def is_xiaoyi_error(error: Any) -> bool:
    """
    Check if an error is a XiaoyiError.

    @param error Error to check
    @return True if XiaoyiError
    @since 0.1.0
    """
    return isinstance(error, XiaoyiError)


__all__ = [
    "ErrorKind",
    "XiaoyiError",
    "create_error",
    "is_xiaoyi_error",
]