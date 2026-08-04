# Layer 0 - Foundation / Core Error Kind
# Path: xiaoyi.core.error.kind
#
# Layer hierarchy:
# - 0: core — foundational cross-cutting types.
# - 1: error — unified exception taxonomy.
# - 2: kind — categorical failure model.

from enum import StrEnum


class ErrorKind(StrEnum):
    """Unified error kind used across Xiaoyi runtime layers."""

    SYNTAX = "syntax"
    PARSE = "parse"
    RUNTIME = "runtime"
    IO = "io"
    AUTH = "auth"
    POLICY = "policy"
    LLM = "llm"
    MEMORY = "memory"
    TOOL = "tool"
    WORKFLOW = "workflow"
    CONFIG = "config"
    STATE = "state"
