"""
# Literals

`literal` defines all literal types in the Xiaoyi language.

Path: `xiaoyi.domain.token.syntax.literal`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `syntax`
- Layer 3: `literal`

@module xiaoyi.domain.token.syntax.literal
@brief Language literals
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.syntax
@see xiaoyi.domain.token.syntax.primitive
"""

from dataclasses import dataclass
from enum import Enum
from typing import Union
from .kinds import SyntaxKind


@dataclass(frozen=True)
class Literal:
    """Literal token."""

    #: Literal kind.
    kind: "LiteralKind"
    #: Raw source text.
    raw: str
    #: Parsed value.
    value: "LiteralValue"


class LiteralKind(str, Enum):
    """Literal categories."""

    #: Integer literal.
    INTEGER = "integer"
    #: Float literal.
    FLOAT = "float"
    #: String literal.
    STRING = "string"
    #: Boolean literal.
    BOOLEAN = "boolean"
    #: Null literal.
    NULL = "null"


#: Union of all literal value types.
LiteralValue = Union[int, float, str, bool, None]


def int_literal(value: int, raw: str | None = None) -> Literal:
    """
    Create integer literal.

    @param value Integer value
    @param raw Raw source text (defaults to str(value))
    @return Literal
    @since 0.1.0
    """
    return Literal(kind=LiteralKind.INTEGER, raw=raw or str(value), value=value)


def float_literal(value: float, raw: str | None = None) -> Literal:
    """
    Create float literal.

    @param value Float value
    @param raw Raw source text (defaults to str(value))
    @return Literal
    @since 0.1.0
    """
    return Literal(kind=LiteralKind.FLOAT, raw=raw or str(value), value=value)


def string_literal(value: str, raw: str | None = None) -> Literal:
    """
    Create string literal.

    @param value String value (unescaped)
    @param raw Raw source text (defaults to quoted value)
    @return Literal
    @since 0.1.0
    """
    return Literal(kind=LiteralKind.STRING, raw=raw or f'"{value}"', value=value)


def bool_literal(value: bool, raw: str | None = None) -> Literal:
    """
    Create boolean literal.

    @param value Boolean value
    @param raw Raw source text (defaults to "true"/"false")
    @return Literal
    @since 0.1.0
    """
    return Literal(kind=LiteralKind.BOOLEAN, raw=raw or ("true" if value else "false"), value=value)


def parse_literal(raw: str) -> Literal:
    """
    Parse literal from raw source text.

    @param raw Raw source text
    @return Parsed literal
    @throws ValueError If cannot parse
    @since 0.1.0
    """
    # Boolean
    if raw == "true":
        return bool_literal(True, raw)
    if raw == "false":
        return bool_literal(False, raw)

    # Null
    if raw == "null":
        return Literal(kind=LiteralKind.NULL, raw=raw, value=None)

    # Integer
    try:
        return int_literal(int(raw), raw)
    except ValueError:
        pass

    # Float
    try:
        return float_literal(float(raw), raw)
    except ValueError:
        pass

    # String (quoted)
    if len(raw) >= 2 and raw[0] == '"' and raw[-1] == '"':
        return string_literal(raw[1:-1], raw)
    if len(raw) >= 2 and raw[0] == "'" and raw[-1] == "'":
        return string_literal(raw[1:-1], raw)

    raise ValueError(f"Cannot parse literal: {raw}")


__all__ = [
    "LiteralKind",
    "Literal",
    "LiteralValue",
    "int_literal",
    "float_literal",
    "string_literal",
    "bool_literal",
    "parse_literal",
]