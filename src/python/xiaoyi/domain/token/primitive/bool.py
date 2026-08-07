"""
# Boolean Primitive

`bool` provides the boolean type with true/false values.

Path: `xiaoyi.domain.token.primitive.bool`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `bool`

@module xiaoyi.domain.token.primitive.bool
@brief Boolean primitive type
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive
@see xiaoyi.domain.token.primitive.int
"""

from typing import Any


#: Boolean type alias.
Bool = bool

#: True value.
TRUE: bool = True

#: False value.
FALSE: bool = False


def bool_not(value: bool) -> bool:
    """
    Logical NOT.

    @param value Boolean value
    @return Negated value
    @since 0.1.0
    """
    return not value


def bool_and(a: bool, b: bool) -> bool:
    """
    Logical AND.

    @param a First value
    @param b Second value
    @return a and b
    @since 0.1.0
    """
    return a and b


def bool_or(a: bool, b: bool) -> bool:
    """
    Logical OR.

    @param a First value
    @param b Second value
    @return a or b
    @since 0.1.0
    """
    return a or b


__all__ = [
    "Bool",
    "TRUE",
    "FALSE",
    "bool_not",
    "bool_and",
    "bool_or",
]