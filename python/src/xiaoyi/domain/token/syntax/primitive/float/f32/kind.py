"""Syntax primitive float token kind: F32.

Layer hierarchy:
- 1 syntax
- 2 primitive
- 3 float
- 4 f32
- 5 kind
"""

from __future__ import annotations

from enum import StrEnum


class F32Kind(StrEnum):
    """Classification of F32 token kinds."""

    LITERAL = "F32_LITERAL"
    VARIABLE = "F32_VAR"
    CAST = "F32_CAST"

    def label(self) -> str:
        """Return the token label used in diagnostics."""
        return self.value
