"""
# Delimiters

`delimiter` defines all delimiters (brackets, parentheses, etc.) in the Xiaoyi language.

Path: `xiaoyi.domain.token.syntax.delimiter`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `syntax`
- Layer 3: `delimiter`

@module xiaoyi.domain.token.syntax.delimiter
@brief Language delimiters
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.syntax
@see xiaoyi.domain.token.syntax.operator
"""

from dataclasses import dataclass
from enum import Enum
from .kinds import SyntaxKind


@dataclass(frozen=True)
class Delimiter:
    """Delimiter token."""

    #: Delimiter text.
    text: str
    #: Delimiter kind.
    kind: "DelimiterKind"


class DelimiterKind(str, Enum):
    """Delimiter categories."""

    #: Opening delimiter.
    OPEN = "open"
    #: Closing delimiter.
    CLOSE = "close"


#: All delimiters.
DELIMITERS = tuple(
    Delimiter(text=t, kind=k)
    for t, k in [
        ("(", DelimiterKind.OPEN),
        (")", DelimiterKind.CLOSE),
        ("[", DelimiterKind.OPEN),
        ("]", DelimiterKind.CLOSE),
        ("{", DelimiterKind.OPEN),
        ("}", DelimiterKind.CLOSE),
        ("<", DelimiterKind.OPEN),
        (">", DelimiterKind.CLOSE),
    ]
)


def matching_close(open_delim: str) -> str | None:
    """
    Get matching closing delimiter.

    @param open_delim Opening delimiter
    @return Closing delimiter or None
    @since 0.1.0
    """
    pairs = {"(": ")", "[": "]", "{": "}", "<": ">"}
    return pairs.get(open_delim)


def matching_open(close_delim: str) -> str | None:
    """
    Get matching opening delimiter.

    @param close_delim Closing delimiter
    @return Opening delimiter or None
    @since 0.1.0
    """
    pairs = {")": "(", "]": "[", "}": "{", ">": "<"}
    return pairs.get(close_delim)


def is_open_delimiter(d: str) -> bool:
    """
    Check if string is an opening delimiter.

    @param d String to check
    @return True if opening delimiter
    @since 0.1.0
    """
    return d in {"(", "[", "{", "<"}


def is_close_delimiter(d: str) -> bool:
    """
    Check if string is a closing delimiter.

    @param d String to check
    @return True if closing delimiter
    @since 0.1.0
    """
    return d in {")", "]", "}", ">"}


def is_delimiter_pair(open_d: str, close_d: str) -> bool:
    """
    Check if two delimiters form a matching pair.

    @param open_d Opening delimiter
    @param close_d Closing delimiter
    @return True if matching pair
    @since 0.1.0
    """
    return matching_close(open_d) == close_d


__all__ = [
    "Delimiter",
    "DelimiterKind",
    "DELIMITERS",
    "matching_close",
    "matching_open",
    "is_open_delimiter",
    "is_close_delimiter",
    "is_delimiter_pair",
]