"""
# Integer Type

`int_type` defines the integer type with kind and width.

Path: `xiaoyi.domain.token.primitive.int.int_type`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `int`
- Layer 4: `int_type` — integer type definition.

@module xiaoyi.domain.token.primitive.int.int_type
@brief Integer type definition
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive.int
@see xiaoyi.domain.token.primitive.int.kind
@see xiaoyi.domain.token.primitive.int.width
"""

from __future__ import annotations
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .kind import IntKind
    from .width import IntWidth


class IntType:
    """
    Integer type with kind and width.

    @brief Integer type specification
    @group Domain
    @since 0.1.0
    """

    def __init__(self, kind: "IntKind", width: "IntWidth"):
        """
        Create integer type.

        @param kind - Signed or unsigned
        @param width - Bit width
        @since 0.1.0
        """
        self.kind = kind
        self.width = width


def create_int_type(kind: "IntKind", width: "IntWidth") -> IntType:
    """
    Create integer type.

    @param kind - Signed or unsigned
    @param width - Bit width
    @return IntType instance
    @since 0.1.0
    """
    return IntType(kind=kind, width=width)


__all__ = [
    "IntType",
    "create_int_type",
]