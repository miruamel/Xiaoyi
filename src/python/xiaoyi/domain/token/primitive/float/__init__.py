"""
# Float Primitives

`float` provides floating-point types (f32, f64) with IEEE 754 compliance.

Path: `xiaoyi.domain.token.primitive.float`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `float` — float type family.
- Layer 4: `f32`/`f64` — concrete types.

@package xiaoyi.domain.token.primitive.float
@brief IEEE 754 floating-point types
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive
@see xiaoyi.domain.token.primitive.float.f32
@see xiaoyi.domain.token.primitive.float.f64
"""

from .f32 import *
from .f64 import *

__all__ = [
    "FloatKind",
    "F32",
    "F64",
    "F32Consts",
    "F64Consts",
    "is_f32_finite",
    "is_f32_nan",
    "is_f32_infinite",
    "is_f64_finite",
    "is_f64_nan",
    "is_f64_infinite",
]


class FloatKind(str):
    """
    Floating-point type kind.

    @brief Float precision classification
    @group Domain
    @since 0.1.0
    """

    F32 = "f32"
    F64 = "f64"


__all__ = [
    "FloatKind",
]