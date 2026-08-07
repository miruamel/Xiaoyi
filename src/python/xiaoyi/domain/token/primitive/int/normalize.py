"""
# Integer Normalization

`normalize` provides integer value normalization (clamping, wrapping).

Path: `xiaoyi.domain.token.primitive.int.normalize`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `int`
- Layer 4: `normalize`

@module xiaoyi.domain.token.primitive.int.normalize
@brief Integer value normalization
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive.int
@see xiaoyi.domain.token.primitive.int.width
"""

from .int_type import IntType, create_int_type
from .kind import IntKind
from .width import IntWidth


def int_byte_size(int_type: IntType) -> int:
    """
    Get size in bytes.

    @param int_type Integer type
    @return Size in bytes
    @since 0.1.0
    """
    return int_type.width // 8


def is_int_signed(int_type: IntType) -> bool:
    """
    Check if signed.

    @param int_type Integer type
    @return True if signed
    @since 0.1.0
    """
    return int_type.kind == IntKind.SIGNED


def normalize_int(value: int, int_type: IntType) -> int:
    """
    Normalize integer value to fit within type bounds.

    @param value Input value
    @param int_type Target integer type
    @return Normalized value (clamped)
    @since 0.1.0
    """
    bits = int_type.width
    if int_type.kind == IntKind.SIGNED:
        max_val = (1 << (bits - 1)) - 1
        min_val = -(1 << (bits - 1))
    else:
        max_val = (1 << bits) - 1
        min_val = 0

    if value > max_val:
        return max_val
    if value < min_val:
        return min_val
    return value


def wrap_int(value: int, int_type: IntType) -> int:
    """
    Wrap integer value to fit within type bounds (modulo).

    @param value Input value
    @param int_type Target integer type
    @return Wrapped value
    @since 0.1.0
    """
    bits = int_type.width
    range_val = 1 << bits

    if int_type.kind == IntKind.SIGNED:
        half = 1 << (bits - 1)
        wrapped = ((value + half) % range_val + range_val) % range_val
        return wrapped - half
    else:
        return ((value % range_val) + range_val) % range_val


def convert_int_checked(value: int, from_type: IntType, to_type: IntType) -> int:
    """
    Convert between integer types with overflow check.

    @param value Source value
    @param from_type Source type
    @param to_type Target type
    @return Normalized value
    @raises ValueError If overflow detected
    @since 0.1.0
    """
    normalized = normalize_int(value, to_type)
    if normalized != value and from_type.width <= to_type.width:
        raise ValueError("Integer overflow")
    return normalized


__all__ = [
    "IntType",
    "create_int_type",
    "int_byte_size",
    "is_int_signed",
    "normalize_int",
    "wrap_int",
    "convert_int_checked",
]