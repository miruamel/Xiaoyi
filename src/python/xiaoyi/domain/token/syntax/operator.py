"""
# Operators

`operator` defines all operators in the Xiaoyi language.

Path: `xiaoyi.domain.token.syntax.operator`

- Layer 0: `domain`
- Layer 1: `token`
- Layer 2: `syntax`
- Layer 3: `operator`

@module xiaoyi.domain.token.syntax.operator
@brief Language operators
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain.token.syntax
@see xiaoyi.domain.token.syntax.keyword
"""

from dataclasses import dataclass
from enum import Enum
from .kinds import SyntaxKind


@dataclass(frozen=True)
class Operator:
    """Operator token."""

    #: Operator symbol.
    symbol: str
    #: Operator kind.
    kind: "OperatorKind"
    #: Precedence (higher = binds tighter).
    precedence: int
    #: Associativity.
    associativity: "Associativity"


class OperatorKind(str, Enum):
    """Operator categories."""

    #: Arithmetic (+, -, *, /, %)
    ARITHMETIC = "arithmetic"
    #: Comparison (==, !=, <, >, <=, >=)
    COMPARISON = "comparison"
    #: Logical (&&, ||, !)
    LOGICAL = "logical"
    #: Bitwise (&, |, ^, <<, >>)
    BITWISE = "bitwise"
    #: Assignment (=, +=, -=, etc.)
    ASSIGNMENT = "assignment"
    #: Member access (., ..)
    ACCESS = "access"


class Associativity(str, Enum):
    """Operator associativity."""

    LEFT = "left"
    RIGHT = "right"
    NONE = "none"


#: All operators ordered by precedence (highest first).
OPERATORS = tuple(
    Operator(symbol=s, kind=k, precedence=p, associativity=a)
    for s, k, p, a in [
        (".", OperatorKind.ACCESS, 100, Associativity.LEFT),
        ("..", OperatorKind.ACCESS, 100, Associativity.LEFT),
        ("*", OperatorKind.ARITHMETIC, 90, Associativity.LEFT),
        ("/", OperatorKind.ARITHMETIC, 90, Associativity.LEFT),
        ("%", OperatorKind.ARITHMETIC, 90, Associativity.LEFT),
        ("+", OperatorKind.ARITHMETIC, 80, Associativity.LEFT),
        ("-", OperatorKind.ARITHMETIC, 80, Associativity.LEFT),
        ("<<", OperatorKind.BITWISE, 70, Associativity.LEFT),
        (">>", OperatorKind.BITWISE, 70, Associativity.LEFT),
        ("<", OperatorKind.COMPARISON, 60, Associativity.NONE),
        ("<=", OperatorKind.COMPARISON, 60, Associativity.NONE),
        (">", OperatorKind.COMPARISON, 60, Associativity.NONE),
        (">=", OperatorKind.COMPARISON, 60, Associativity.NONE),
        ("==", OperatorKind.COMPARISON, 60, Associativity.NONE),
        ("!=", OperatorKind.COMPARISON, 60, Associativity.NONE),
        ("&", OperatorKind.BITWISE, 50, Associativity.LEFT),
        ("^", OperatorKind.BITWISE, 40, Associativity.LEFT),
        ("|", OperatorKind.BITWISE, 30, Associativity.LEFT),
        ("&&", OperatorKind.LOGICAL, 20, Associativity.LEFT),
        ("||", OperatorKind.LOGICAL, 10, Associativity.LEFT),
        ("=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        ("+=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        ("-=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        ("*=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        ("/=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        ("%=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        ("&=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        ("|=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        ("^=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        ("<<=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
        (">>=", OperatorKind.ASSIGNMENT, 5, Associativity.RIGHT),
    ]
)


def operator_from_symbol(symbol: str) -> Operator | None:
    """
    Look up operator by symbol.

    @param symbol Operator symbol
    @return Operator if found, None otherwise
    @since 0.1.0
    """
    for op in OPERATORS:
        if op.symbol == symbol:
            return op
    return None


def operators_with_prefix(prefix: str) -> list[Operator]:
    """
    Get all operators starting with prefix.

    @param prefix Prefix to match
    @return List of matching operators
    @since 0.1.0
    """
    return [op for op in OPERATORS if op.symbol.startswith(prefix)]


__all__ = [
    "Operator",
    "OperatorKind",
    "Associativity",
    "OPERATORS",
    "operator_from_symbol",
    "operators_with_prefix",
]