"""
# Syntax Tokens

`syntax` provides syntax-level tokens (keywords, operators, delimiters)
for the Xiaoyi language parser.

Path: `xiaoyi.domain.token.syntax`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `syntax` — syntax token definitions.
- Layer 3: `keyword`/`operator`/`delimiter`/`literal` — token categories.

@package xiaoyi.domain.token.syntax
@brief Syntax-level tokens for parsing
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token
@see xiaoyi.domain.token.primitive
@see xiaoyi.lexer
"""

from .kinds import *
from .keyword import *
from .operator import *
from .delimiter import *
from .literal import *

__all__ = [
    "SyntaxKind",
    "Keyword",
    "KeywordKind",
    "KEYWORDS",
    "keyword_from_ident",
    "is_keyword",
    "Operator",
    "OperatorKind",
    "Associativity",
    "OPERATORS",
    "operator_from_symbol",
    "operators_with_prefix",
    "Delimiter",
    "DelimiterKind",
    "DELIMITERS",
    "matching_close",
    "matching_open",
    "is_open_delimiter",
    "is_close_delimiter",
    "is_delimiter_pair",
    "LiteralKind",
    "Literal",
    "LiteralValue",
    "int_literal",
    "float_literal",
    "string_literal",
    "bool_literal",
    "parse_literal",
]