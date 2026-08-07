"""
# Integer Width

`width` defines supported integer bit widths.

Path: `xiaoyi.domain.token.primitive.int.width`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `int`
- Layer 4: `width`

@module xiaoyi.domain.token.primitive.int.width
@brief Integer bit width definitions
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive.int
@see xiaoyi.domain.token.primitive.int.kind
"""

from enum import Enum


class IntWidth(int, Enum):
    """Integer bit width."""

    #: 8-bit integer width.
    W8 = 8
    #: 16-bit integer width.
    W16 = 16
    #: 32-bit integer width.
    W32 = 32
    #: 64-bit integer width.
    W64 = 64
    #: 128-bit integer width.
    W128 = 128


#: 8-bit integer width.
W8 = IntWidth.W8

#: 16-bit integer width.
W16 = IntWidth.W16

#: 32-bit integer width.
W32 = IntWidth.W32

#: 64-bit integer width.
W64 = IntWidth.W64

#: 128-bit integer width.
W128 = IntWidth.W128


def default_width() -> IntWidth:
    """
    Get default width (64-bit).

    @return Default IntWidth
    @since 0.1.0
    """
    return IntWidth.W64


__all__ = [
    "IntWidth",
    "W8",
    "W16",
    "W32",
    "W64",
    "W128",
    "default_width",
]