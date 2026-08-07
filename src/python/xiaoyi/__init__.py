"""
# Xiaoyi Python Package

`xiaoyi` is a polyglot AI agent framework with Rust core and Python/TypeScript bindings.

## Architecture

- **Core**: Configuration, error handling, result types
- **Domain**: Token primitives and syntax definitions
- **LLM**: Language model client abstractions
- **Workflow**: DAG-based workflow execution
- **Memory**: Short-term and long-term memory management
- **Builder**: AST building and code generation
- **Orchestrator**: Agent orchestration and policy
- **Gateway**: API and CLI interfaces
- **Lexer**: Lexical analysis and tokenization

@package xiaoyi
@brief Polyglot AI Agent Framework
@group Framework
@since 0.1.0
@author Miruamel
@see xiaoyi.core
@see xiaoyi.domain
@see xiaoyi.llm
@see xiaoyi.workflow
@see xiaoyi.memory
"""

from .core import *
from .domain import *

__version__ = "0.1.0"
__author__ = "Miruamel"
__all__ = [
    "core",
    "domain",
]