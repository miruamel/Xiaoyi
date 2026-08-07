"""
# Integer Primitives

`int` provides signed and unsigned integer types with configurable
width, representation, and normalization.

Path: `xiaoyi.domain.token.primitive.int`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `int` — integer type family.
- Layer 4: `kind`/`width`/`rep`/`normalize` — details.

@package xiaoyi.domain.token.primitive.int
@brief Integer primitive types with width and representation
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive
@see xiaoyi.domain.token.primitive.int.kind
@see xiaoyi.domain.token.primitive.int.width
"""

from .kind import *
from .width import *
from .rep import *
from .normalize import *
from .int_type import *

__all__ = [
    "IntKind",
    "IntWidth",
    "IntType",
    "create_int_type",
    "int_byte_size",
    "is_int_signed",
    "SIGNED",
    "UNSIGNED",
    "default_int_type",
    "W8",
    "W16",
    "W32",
    "W64",
    "W128",
    "default_width",
    "Endianness",
    "native_endianness",
    "DEFAULT_REP",
    "normalize_int",
    "wrap_int",
    "convert_int_checked",
]