"""
# Integer Kind

`kind` defines signed vs unsigned integer classification.

Path: `xiaoyi.domain.token.primitive.int.kind`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `int`
- Layer 4: `kind`

@module xiaoyi.domain.token.primitive.int.kind
@brief Integer signedness classification
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive.int
@see xiaoyi.domain.token.primitive.int.width
"""

from enum import Enum
from .int_type import IntType
from .width import IntWidth


class IntKind(str, Enum):
    """Integer signedness."""

    #: Signed integer (two's complement representation).
    SIGNED = "signed"
    #: Unsigned integer.
    UNSIGNED = "unsigned"


#: Signed integer (two's complement representation).
SIGNED = IntKind.SIGNED

#: Unsigned integer.
UNSIGNED = IntKind.UNSIGNED


def default_int_type() -> IntType:
    """
    Get default integer type (signed 64-bit).

    @return Default IntType
    @since 0.1.0
    """
    return IntType(kind=IntKind.SIGNED, width=IntWidth.W64)


__all__ = [
    "IntKind",
    "SIGNED",
    "UNSIGNED",
    "default_int_type",
]