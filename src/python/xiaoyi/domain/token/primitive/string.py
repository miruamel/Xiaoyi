"""
# String Primitive

`string` provides UTF-8 string type with encoding validation.

Path: `xiaoyi.domain.token.primitive.string`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `primitive`
- Layer 3: `string`

@module xiaoyi.domain.token.primitive.string
@brief UTF-8 string primitive
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.primitive
@see xiaoyi.domain.token.primitive.int
"""

from typing import Any


#: String type alias (owned UTF-8).
String = str

#: String slice type alias (borrowed UTF-8).
Str = str


def new_string() -> String:
    """
    Create new empty string.

    @return Empty String
    @since 0.1.0
    """
    return ""


def from_string(s: Str) -> String:
    """
    Create string from string slice.

    @param s String slice
    @return Owned String
    @since 0.1.0
    """
    return s


def is_valid_utf8(data: bytes) -> bool:
    """
    Check if bytes is valid UTF-8.

    @param data Byte array
    @return True if valid UTF-8
    @since 0.1.0
    """
    try:
        data.decode("utf-8")
        return True
    except UnicodeDecodeError:
        return False


def char_len(s: Str) -> int:
    """
    Get string length in characters (code points).

    @param s String slice
    @return Character count
    @since 0.1.0
    """
    return len(s)


__all__ = [
    "String",
    "Str",
    "new_string",
    "from_string",
    "is_valid_utf8",
    "char_len",
]