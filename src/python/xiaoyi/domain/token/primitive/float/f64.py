"""
# 64-bit Float (f64)

`f64` provides IEEE 754 double-precision floating-point type.

Path: `xiaoyi.domain.token.primitive.float.f64`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `float`
- Layer 4: `f64`

@module xiaoyi.domain.token.primitive.float.f64
@brief IEEE 754 double-precision float
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive.float
@see xiaoyi.domain.token.primitive.float.f32
"""

import math


#: 64-bit float type alias.
F64 = float

#: f64 bit pattern.
F64Bits = int


class F64Consts:
    """f64 constants."""

    #: Positive infinity.
    INFINITY = float("inf")
    #: Negative infinity.
    NEG_INFINITY = float("-inf")
    #: Not a Number.
    NAN = float("nan")
    #: Minimum positive normal value.
    MIN_POSITIVE = 2.2250738585072014e-308
    #: Maximum finite value.
    MAX = 1.7976931348623157e308
    #: Minimum finite value.
    MIN = -1.7976931348623157e308
    #: Epsilon (difference between 1.0 and next representable).
    EPSILON = 2.220446049250313e-16


def is_f64_finite(value: float) -> bool:
    """
    Check if value is finite.

    @param value f64 value
    @return True if finite
    @since 0.1.0
    """
    return math.isfinite(value)


def is_f64_nan(value: float) -> bool:
    """
    Check if value is NaN.

    @param value f64 value
    @return True if NaN
    @since 0.1.0
    """
    return math.isnan(value)


def is_f64_infinite(value: float) -> bool:
    """
    Check if value is infinite.

    @param value f64 value
    @return True if infinite
    @since 0.1.0
    """
    return math.isinf(value)


__all__ = [
    "F64",
    "F64Bits",
    "F64Consts",
    "is_f64_finite",
    "is_f64_nan",
    "is_f64_infinite",
]