"""
Test suite for xiaoyi.domain.token.syntax.operator module.

@package xiaoyi.tests.domain.token.syntax
@brief Tests for Operator, OperatorKind, Associativity, OPERATORS, operator_from_symbol
@since 0.1.0
"""

import pytest
from xiaoyi.domain.token.syntax.operator import (
    Operator,
    OperatorKind,
    Associativity,
    OPERATORS,
    operator_from_symbol,
    operators_with_prefix,
)


class TestOperator:
    """Tests for Operator dataclass."""

    def test_operator_creation(self):
        """Test creating an Operator instance."""
        op = Operator(
            symbol="+",
            kind=OperatorKind.ARITHMETIC,
            precedence=10,
            associativity=Associativity.LEFT
        )
        assert op.symbol == "+"
        assert op.kind == OperatorKind.ARITHMETIC
        assert op.precedence == 10
        assert op.associativity == Associativity.LEFT

    def test_operator_frozen(self):
        """Test that Operator is frozen (immutable)."""
        op = Operator(symbol="+", kind=OperatorKind.ARITHMETIC, precedence=10, associativity=Associativity.LEFT)
        with pytest.raises(AttributeError):
            op.symbol = "-"


class TestOperatorKind:
    """Tests for OperatorKind enum."""

    def test_operator_kind_values(self):
        """Test all expected operator kinds exist."""
        assert OperatorKind.ARITHMETIC == "arithmetic"
        assert OperatorKind.COMPARISON == "comparison"
        assert OperatorKind.LOGICAL == "logical"
        assert OperatorKind.BITWISE == "bitwise"
        assert OperatorKind.ASSIGNMENT == "assignment"
        assert OperatorKind.ACCESS == "access"

    def test_operator_kind_iteration(self):
        """Test that all operator kinds can be iterated."""
        kinds = list(OperatorKind)
        assert len(kinds) == 6


class TestAssociativity:
    """Tests for Associativity enum."""

    def test_associativity_values(self):
        """Test all associativity values exist."""
        assert Associativity.LEFT == "left"
        assert Associativity.RIGHT == "right"
        assert Associativity.NONE == "none"

    def test_associativity_iteration(self):
        """Test that all associativity values can be iterated."""
        assoc = list(Associativity)
        assert len(assoc) == 3


class TestOPERATORS:
    """Tests for OPERATORS tuple."""

    def test_operators_not_empty(self):
        """Test that OPERATORS is not empty."""
        assert len(OPERATORS) > 0

    def test_operators_are_operator_instances(self):
        """Test that all OPERATORS are Operator instances."""
        for op in OPERATORS:
            assert isinstance(op, Operator)

    def test_operators_ordered_by_precedence(self):
        """Test that operators are ordered by precedence (highest first)."""
        for i in range(len(OPERATORS) - 1):
            assert OPERATORS[i].precedence >= OPERATORS[i + 1].precedence

    def test_expected_operators_exist(self):
        """Test that expected operators exist."""
        symbols = {op.symbol for op in OPERATORS}
        expected = {
            "+", "-", "*", "/", "%",       # Arithmetic
            "==", "!=", "<", ">", "<=", ">=",  # Comparison
            "&&", "||", "!",               # Logical
            "&", "|", "^", "<<", ">>",     # Bitwise
            "=", "+=", "-=", "*=", "/=",   # Assignment
            ".", "..",                     # Access
        }
        assert expected.issubset(symbols)

    def test_arithmetic_operators(self):
        """Test arithmetic operators have correct properties."""
        arithmetic = [op for op in OPERATORS if op.kind == OperatorKind.ARITHMETIC]
        symbols = {op.symbol for op in arithmetic}
        assert {"+", "-", "*", "/", "%"}.issubset(symbols)
        for op in arithmetic:
            assert op.associativity == Associativity.LEFT

    def test_comparison_operators(self):
        """Test comparison operators have correct properties."""
        comparison = [op for op in OPERATORS if op.kind == OperatorKind.COMPARISON]
        symbols = {op.symbol for op in comparison}
        assert {"==", "!=", "<", ">", "<=", ">="}.issubset(symbols)
        for op in comparison:
            assert op.associativity == Associativity.NONE

    def test_logical_operators(self):
        """Test logical operators have correct properties."""
        logical = [op for op in OPERATORS if op.kind == OperatorKind.LOGICAL]
        symbols = {op.symbol for op in logical}
        assert {"&&", "||", "!"}.issubset(symbols)

    def test_assignment_operators(self):
        """Test assignment operators have correct properties."""
        assignment = [op for op in OPERATORS if op.kind == OperatorKind.ASSIGNMENT]
        symbols = {op.symbol for op in assignment}
        assert {"=", "+=", "-=", "*=", "/="}.issubset(symbols)
        for op in assignment:
            assert op.associativity == Associativity.RIGHT


class TestOperatorFromSymbol:
    """Tests for operator_from_symbol function."""

    def test_operator_from_symbol_valid(self):
        """Test getting operator from valid symbol."""
        op = operator_from_symbol("+")
        assert op is not None
        assert op.symbol == "+"
        assert op.kind == OperatorKind.ARITHMETIC

    def test_operator_from_symbol_invalid(self):
        """Test getting operator from invalid symbol returns None."""
        assert operator_from_symbol("not_an_operator") is None
        assert operator_from_symbol("") is None

    def test_operator_from_symbol_multi_char(self):
        """Test getting multi-character operators."""
        op = operator_from_symbol("==")
        assert op is not None
        assert op.symbol == "=="

        op = operator_from_symbol("+=")
        assert op is not None
        assert op.symbol == "+="


class TestOperatorsWithPrefix:
    """Tests for operators_with_prefix function."""

    def test_operators_with_prefix_empty(self):
        """Test getting operators with empty prefix returns all."""
        all_ops = operators_with_prefix("")
        assert len(all_ops) == len(OPERATORS)

    def test_operators_with_prefix_plus(self):
        """Test getting operators starting with +."""
        ops = operators_with_prefix("+")
        symbols = {op.symbol for op in ops}
        assert "+" in symbols
        assert "+=" in symbols

    def test_operators_with_prefix_equal(self):
        """Test getting operators starting with =."""
        ops = operators_with_prefix("=")
        symbols = {op.symbol for op in ops}
        assert "=" in symbols
        assert "==" in symbols
        assert "+=" in symbols
        assert "-=" in symbols
        assert "*=" in symbols
        assert "/=" in symbols

    def test_operators_with_prefix_invalid(self):
        """Test getting operators with non-matching prefix returns empty."""
        ops = operators_with_prefix("invalid")
        assert len(ops) == 0


class TestOperatorIntegration:
    """Integration tests for operator module."""

    def test_all_operators_have_valid_kinds(self):
        """Test that all operators have valid kinds."""
        for op in OPERATORS:
            assert op.kind in OperatorKind
            assert op.associativity in Associativity
            assert op.precedence > 0

    def test_precedence_order(self):
        """Test that precedence ordering makes sense."""
        # Higher precedence = binds tighter
        # Access should have highest precedence
        access_ops = [op for op in OPERATORS if op.kind == OperatorKind.ACCESS]
        max_access_prec = max(op.precedence for op in access_ops)

        # Assignment should have lowest precedence
        assign_ops = [op for op in OPERATORS if op.kind == OperatorKind.ASSIGNMENT]
        min_assign_prec = min(op.precedence for op in assign_ops)

        assert max_access_prec > min_assign_prec

    def test_unique_symbols(self):
        """Test that all operator symbols are unique."""
        symbols = [op.symbol for op in OPERATORS]
        assert len(symbols) == len(set(symbols))