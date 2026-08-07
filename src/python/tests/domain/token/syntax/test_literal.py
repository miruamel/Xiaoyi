"""
Test suite for xiaoyi.domain.token.syntax.literal module.

@package xiaoyi.tests.domain.token.syntax
@brief Tests for Literal, LiteralKind, literal factories, parse_literal
@since 0.1.0
"""

import pytest
from xiaoyi.domain.token.syntax.literal import (
    Literal,
    LiteralKind,
    LiteralValue,
    int_literal,
    float_literal,
    string_literal,
    bool_literal,
    parse_literal,
)


class TestLiteral:
    """Tests for Literal dataclass."""

    def test_literal_creation(self):
        """Test creating a Literal instance."""
        lit = Literal(kind=LiteralKind.INTEGER, raw="42", value=42)
        assert lit.kind == LiteralKind.INTEGER
        assert lit.raw == "42"
        assert lit.value == 42

    def test_literal_frozen(self):
        """Test that Literal is frozen (immutable)."""
        lit = Literal(kind=LiteralKind.INTEGER, raw="42", value=42)
        with pytest.raises(AttributeError):
            lit.value = 100


class TestLiteralKind:
    """Tests for LiteralKind enum."""

    def test_literal_kind_values(self):
        """Test all literal kinds exist."""
        assert LiteralKind.INTEGER == "integer"
        assert LiteralKind.FLOAT == "float"
        assert LiteralKind.STRING == "string"
        assert LiteralKind.BOOLEAN == "boolean"
        assert LiteralKind.NULL == "null"

    def test_literal_kind_iteration(self):
        """Test that all literal kinds can be iterated."""
        kinds = list(LiteralKind)
        assert len(kinds) == 5


class TestIntLiteral:
    """Tests for int_literal function."""

    def test_int_literal_basic(self):
        """Test creating integer literal."""
        lit = int_literal(42)
        assert lit.kind == LiteralKind.INTEGER
        assert lit.raw == "42"
        assert lit.value == 42

    def test_int_literal_negative(self):
        """Test creating negative integer literal."""
        lit = int_literal(-42)
        assert lit.value == -42
        assert lit.raw == "-42"

    def test_int_literal_zero(self):
        """Test creating zero integer literal."""
        lit = int_literal(0)
        assert lit.value == 0
        assert lit.raw == "0"

    def test_int_literal_custom_raw(self):
        """Test creating integer literal with custom raw."""
        lit = int_literal(42, raw="0x2A")
        assert lit.value == 42
        assert lit.raw == "0x2A"


class TestFloatLiteral:
    """Tests for float_literal function."""

    def test_float_literal_basic(self):
        """Test creating float literal."""
        lit = float_literal(3.14)
        assert lit.kind == LiteralKind.FLOAT
        assert lit.raw == "3.14"
        assert lit.value == 3.14

    def test_float_literal_negative(self):
        """Test creating negative float literal."""
        lit = float_literal(-3.14)
        assert lit.value == -3.14
        assert lit.raw == "-3.14"

    def test_float_literal_scientific(self):
        """Test creating float literal with scientific notation."""
        lit = float_literal(1.5e-10, raw="1.5e-10")
        assert lit.value == 1.5e-10
        assert lit.raw == "1.5e-10"

    def test_float_literal_custom_raw(self):
        """Test creating float literal with custom raw."""
        lit = float_literal(3.14, raw="3.1400")
        assert lit.value == 3.14
        assert lit.raw == "3.1400"


class TestStringLiteral:
    """Tests for string_literal function."""

    def test_string_literal_basic(self):
        """Test creating string literal."""
        lit = string_literal("hello")
        assert lit.kind == LiteralKind.STRING
        assert lit.raw == '"hello"'
        assert lit.value == "hello"

    def test_string_literal_empty(self):
        """Test creating empty string literal."""
        lit = string_literal("")
        assert lit.value == ""
        assert lit.raw == '""'

    def test_string_literal_with_escapes(self):
        """Test creating string literal with escape sequences."""
        lit = string_literal("hello\nworld", raw='"hello\\nworld"')
        assert lit.value == "hello\nworld"
        assert lit.raw == '"hello\\nworld"'

    def test_string_literal_custom_raw(self):
        """Test creating string literal with custom raw."""
        lit = string_literal("hello", raw="'hello'")
        assert lit.value == "hello"
        assert lit.raw == "'hello'"


class TestBoolLiteral:
    """Tests for bool_literal function."""

    def test_bool_literal_true(self):
        """Test creating true boolean literal."""
        lit = bool_literal(True)
        assert lit.kind == LiteralKind.BOOLEAN
        assert lit.raw == "true"
        assert lit.value is True

    def test_bool_literal_false(self):
        """Test creating false boolean literal."""
        lit = bool_literal(False)
        assert lit.kind == LiteralKind.BOOLEAN
        assert lit.raw == "false"
        assert lit.value is False

    def test_bool_literal_custom_raw(self):
        """Test creating boolean literal with custom raw."""
        lit = bool_literal(True, raw="TRUE")
        assert lit.value is True
        assert lit.raw == "TRUE"


class TestParseLiteral:
    """Tests for parse_literal function."""

    def test_parse_integer(self):
        """Test parsing integer literals."""
        lit = parse_literal("42")
        assert lit.kind == LiteralKind.INTEGER
        assert lit.value == 42
        assert lit.raw == "42"

    def test_parse_negative_integer(self):
        """Test parsing negative integer literals."""
        lit = parse_literal("-42")
        assert lit.kind == LiteralKind.INTEGER
        assert lit.value == -42

    def test_parse_float(self):
        """Test parsing float literals."""
        lit = parse_literal("3.14")
        assert lit.kind == LiteralKind.FLOAT
        assert lit.value == 3.14

    def test_parse_scientific_float(self):
        """Test parsing scientific notation floats."""
        lit = parse_literal("1.5e-10")
        assert lit.kind == LiteralKind.FLOAT
        assert lit.value == 1.5e-10

    def test_parse_string_double_quotes(self):
        """Test parsing double-quoted strings."""
        lit = parse_literal('"hello world"')
        assert lit.kind == LiteralKind.STRING
        assert lit.value == "hello world"

    def test_parse_string_single_quotes(self):
        """Test parsing single-quoted strings."""
        lit = parse_literal("'hello world'")
        assert lit.kind == LiteralKind.STRING
        assert lit.value == "hello world"

    def test_parse_string_empty(self):
        """Test parsing empty strings."""
        lit = parse_literal('""')
        assert lit.value == ""
        lit = parse_literal("''")
        assert lit.value == ""

    def test_parse_bool_true(self):
        """Test parsing true boolean."""
        lit = parse_literal("true")
        assert lit.kind == LiteralKind.BOOLEAN
        assert lit.value is True

    def test_parse_bool_false(self):
        """Test parsing false boolean."""
        lit = parse_literal("false")
        assert lit.kind == LiteralKind.BOOLEAN
        assert lit.value is False

    def test_parse_null(self):
        """Test parsing null literal."""
        lit = parse_literal("null")
        assert lit.kind == LiteralKind.NULL
        assert lit.value is None
        assert lit.raw == "null"

    def test_parse_invalid_raises(self):
        """Test parsing invalid literal raises ValueError."""
        with pytest.raises(ValueError, match="Cannot parse literal"):
            parse_literal("invalid")

        with pytest.raises(ValueError):
            parse_literal("")

        with pytest.raises(ValueError):
            parse_literal("123abc")


class TestLiteralIntegration:
    """Integration tests for literal module."""

    def test_literal_value_types(self):
        """Test that LiteralValue covers all value types."""
        # This is a type annotation test - verify the union works
        values: list[LiteralValue] = [42, 3.14, "hello", True, None]
        assert len(values) == 5

    def test_all_literal_factories_produce_correct_kind(self):
        """Test that all factory functions produce correct kinds."""
        assert int_literal(1).kind == LiteralKind.INTEGER
        assert float_literal(1.0).kind == LiteralKind.FLOAT
        assert string_literal("x").kind == LiteralKind.STRING
        assert bool_literal(True).kind == LiteralKind.BOOLEAN

    def test_parse_literal_roundtrip(self):
        """Test parsing literals created by factories."""
        test_cases = [
            int_literal(42),
            float_literal(3.14),
            string_literal("hello"),
            bool_literal(True),
        ]
        for lit in test_cases:
            parsed = parse_literal(lit.raw)
            assert parsed.kind == lit.kind
            assert parsed.value == lit.value