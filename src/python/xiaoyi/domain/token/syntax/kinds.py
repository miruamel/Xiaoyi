"""
# Syntax Kinds

`kinds` defines the syntax token kind enumeration.

Path: `xiaoyi.domain.token.syntax.kinds`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `syntax`
- Layer 3: `kinds` — token kind enumeration.

@module xiaoyi.domain.token.syntax.kinds
@brief Syntax token kind enumeration
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.syntax
"""


class SyntaxKind(str):
    """
    Syntax token kind.

    @brief Classification of syntax tokens
    @group Domain
    @since 0.1.0
    """

    KEYWORD = "keyword"
    OPERATOR = "operator"
    DELIMITER = "delimiter"
    LITERAL = "literal"
    IDENTIFIER = "identifier"
    EOF = "eof"


__all__ = [
    "SyntaxKind",
]