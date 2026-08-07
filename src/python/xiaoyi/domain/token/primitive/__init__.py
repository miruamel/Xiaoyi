"""
# Primitive Types

`primitive` defines the fundamental primitive types: integers,
floats, booleans, and strings with their representations.

Path: `xiaoyi.domain.token.primitive`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive` — primitive type system.
- Layer 3: `int`/`float`/`bool`/`string` — type families.
- Layer 4: `kind`/`width`/`rep`/`normalize` — type details.

@package xiaoyi.domain.token.primitive
@brief Fundamental primitive type definitions
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token
@see xiaoyi.domain.token.primitive.int
@see xiaoyi.domain.token.primitive.float
"""

from .int import *
from .float import *
from .bool import *
from .string import *

__all__ = [
    "PrimitiveKind",
    "IntKind",
    "IntWidth",
    "IntType",
    "FloatKind",
    "Bool",
    "TRUE",
    "FALSE",
    "bool_not",
    "bool_and",
    "bool_or",
    "String",
    "Str",
    "new_string",
    "from_string",
    "is_valid_utf8",
    "char_len",
]


class PrimitiveKind(str):
    """
    Primitive type kind.

    @brief Classification of primitive types
    @group Domain
    @since 0.1.0
    """

    INT = "int"
    FLOAT = "float"
    BOOL = "bool"
    STRING = "string"


__all__ = [
    "PrimitiveKind",
]