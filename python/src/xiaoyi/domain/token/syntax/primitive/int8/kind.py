"""Syntax primitive integer token kind: INT8.

Layer hierarchy:
- 1 syntax
- 2 primitive
- 3 int8
- 4 kind

Concrete variant taxonomy for INT8 syntax nodes before rendering.
"""

from __future__ import annotations

from enum import StrEnum


class Int8Kind(StrEnum):
    """Classification of INT8 token kinds."""

    LITERAL = "INT8_LITERAL"
    VARIABLE = "INT8_VAR"
    CAST = "INT8_CAST"

    def label(self) -> str:
        """Return the token label for parser diagnostics."""
        return self.value
