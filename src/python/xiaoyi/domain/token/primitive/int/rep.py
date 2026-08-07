"""
# Integer Representation

`rep` defines integer representation details (endianness, encoding).

Path: `xiaoyi.domain.token.primitive.int.rep`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `int`
- Layer 4: `rep`

@module xiaoyi.domain.token.primitive.int.rep
@brief Integer representation details
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive.int
@see xiaoyi.domain.token.primitive.int.kind
"""

from enum import Enum
from .kind import IntKind
from .width import IntWidth


class Endianness(str, Enum):
    """Integer endianness."""

    #: Little-endian (least significant byte first).
    LITTLE = "little"
    #: Big-endian (most significant byte first).
    BIG = "big"
    #: Native endianness.
    NATIVE = "native"


def native_endianness() -> Endianness:
    """
    Get native endianness.

    @return Native endianness
    @since 0.1.0
    """
    # Python runs on little-endian in practice
    return Endianness.LITTLE


#: Default integer representation: signed, 64-bit, little-endian.
DEFAULT_REP = (IntKind.SIGNED, IntWidth.W64, Endianness.LITTLE)


__all__ = [
    "Endianness",
    "native_endianness",
    "DEFAULT_REP",
]