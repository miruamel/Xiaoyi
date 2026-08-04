"""Syntax primitive integer token kind: INT16.

Layer hierarchy:
- 1 syntax
- 2 primitive
- 3 int16
- 4 kind
"""

from __future__ import annotations

from enum import StrEnum


class Int16Kind(StrEnum):
    """Classification of INT16 token kinds."""

    LITERAL = "INT16_LITERAL"
    VARIABLE = "INT16_VAR"
    CAST = "INT16_CAST"

    def label(self) -> str:
        return self.value
