"""
# Keywords

`keyword` defines all reserved keywords in the Xiaoyi language.

Path: `xiaoyi.domain.token.syntax.keyword`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `syntax`
- Layer 3: `keyword`

@module xiaoyi.domain.token.syntax.keyword
@brief Language reserved keywords
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
class Keyword:
    """Keyword token."""

    #: Keyword text.
    text: str
    #: Keyword kind.
    kind: "KeywordKind"


class KeywordKind(str, Enum):
    """Keyword categories."""

    #: Control flow (if, else, while, for, return)
    CONTROL_FLOW = "control_flow"
    #: Declaration (let, const, fn, struct, enum)
    DECLARATION = "declaration"
    #: Type (int, float, bool, string)
    TYPE = "type"
    #: Module (mod, use, pub)
    MODULE = "module"
    #: Async (async, await, spawn)
    ASYNC = "async"
    #: Error handling (try, catch, throw)
    ERROR_HANDLING = "error_handling"


#: All keywords.
KEYWORDS = tuple(
    Keyword(text=t, kind=k)
    for t, k in [
        ("if", KeywordKind.CONTROL_FLOW),
        ("else", KeywordKind.CONTROL_FLOW),
        ("while", KeywordKind.CONTROL_FLOW),
        ("for", KeywordKind.CONTROL_FLOW),
        ("return", KeywordKind.CONTROL_FLOW),
        ("break", KeywordKind.CONTROL_FLOW),
        ("continue", KeywordKind.CONTROL_FLOW),
        ("let", KeywordKind.DECLARATION),
        ("const", KeywordKind.DECLARATION),
        ("fn", KeywordKind.DECLARATION),
        ("struct", KeywordKind.DECLARATION),
        ("enum", KeywordKind.DECLARATION),
        ("int", KeywordKind.TYPE),
        ("float", KeywordKind.TYPE),
        ("bool", KeywordKind.TYPE),
        ("string", KeywordKind.TYPE),
        ("mod", KeywordKind.MODULE),
        ("use", KeywordKind.MODULE),
        ("pub", KeywordKind.MODULE),
        ("async", KeywordKind.ASYNC),
        ("await", KeywordKind.ASYNC),
        ("spawn", KeywordKind.ASYNC),
        ("try", KeywordKind.ERROR_HANDLING),
        ("catch", KeywordKind.ERROR_HANDLING),
        ("throw", KeywordKind.ERROR_HANDLING),
    ]
)


def keyword_from_ident(ident: str) -> Keyword | None:
    """
    Check if identifier is a keyword.

    @param ident Identifier string
    @return Keyword if keyword, None otherwise
    @since 0.1.0
    """
    for kw in KEYWORDS:
        if kw.text == ident:
            return kw
    return None


def is_keyword(s: str) -> bool:
    """
    Check if string is a keyword.

    @param s String to check
    @return True if keyword
    @since 0.1.0
    """
    return keyword_from_ident(s) is not None


__all__ = [
    "Keyword",
    "KeywordKind",
    "KEYWORDS",
    "keyword_from_ident",
    "is_keyword",
]