"""
# 32-bit Float (f32)

`f32` provides IEEE 754 single-precision floating-point type.

Path: `xiaoyi.domain.token.primitive.float.f32`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `float`
- Layer 4: `f32`

@module xiaoyi.domain.token.primitive.float.f32
@brief IEEE 754 single-precision float
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive.float
@see xiaoyi.domain.token.primitive.float.f64
"""

import math


#: 32-bit float type alias.
F32 = float

#: f32 bit pattern.
F32Bits = int


class F32Consts:
    """f32 constants."""

    #: Positive infinity.
    INFINITY = float("inf")
    #: Negative infinity.
    NEG_INFINITY = float("-inf")
    #: Not a Number.
    NAN = float("nan")
    #: Minimum positive normal value.
    MIN_POSITIVE = 1.175494351e-38
    #: Maximum finite value.
    MAX = 3.402823466e38
    #: Minimum finite value.
    MIN = -3.402823466e38
    #: Epsilon (difference between 1.0 and next representable).
    EPSILON = 1.19209290e-7


def is_f32_finite(value: float) -> bool:
    """
    Check if value is finite.

    @param value f32 value
    @return True if finite
    @since 0.1.0
    """
    return math.isfinite(value)


def is_f32_nan(value: float) -> bool:
    """
    Check if value is NaN.

    @param value f32 value
    @return True if NaN
    @since 0.1.0
    """
    return math.isnan(value)


def is_f32_infinite(value: float) -> bool:
    """
    Check if value is infinite.

    @param value f32 value
    @return True if infinite
    @since 0.1.0
    """
    return math.isinf(value)


__all__ = [
    "F32",
    "F32Bits",
    "F32Consts",
    "is_f32_finite",
    "is_f32_nan",
    "is_f32_infinite",
]