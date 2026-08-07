"""
Test suite for xiaoyi.domain.token.syntax.kinds module.

@package xiaoyi.tests.domain.token.syntax
@brief Tests for SyntaxKind
@since 0.1.0
"""

import pytest
from xiaoyi.domain.token.syntax.kinds import SyntaxKind


class TestSyntaxKind:
    """Tests for SyntaxKind class."""

    def test_syntax_kind_values(self):
        """Test all syntax kind values."""
        assert SyntaxKind.KEYWORD == "keyword"
        assert SyntaxKind.OPERATOR == "operator"
        assert SyntaxKind.DELIMITER == "delimiter"
        assert SyntaxKind.LITERAL == "literal"
        assert SyntaxKind.IDENTIFIER == "identifier"
        assert SyntaxKind.EOF == "eof"

    def test_syntax_kind_is_str(self):
        """Test that SyntaxKind values are strings."""
        assert isinstance(SyntaxKind.KEYWORD, str)
        assert isinstance(SyntaxKind.OPERATOR, str)
        assert isinstance(SyntaxKind.DELIMITER, str)
        assert isinstance(SyntaxKind.LITERAL, str)
        assert isinstance(SyntaxKind.IDENTIFIER, str)
        assert isinstance(SyntaxKind.EOF, str)

    def test_syntax_kind_constants(self):
        """Test that all expected constants exist."""
        expected = {
            "KEYWORD", "OPERATOR", "DELIMITER",
            "LITERAL", "IDENTIFIER", "EOF"
        }
        for name in expected:
            assert hasattr(SyntaxKind, name)


class TestSyntaxKindUsage:
    """Tests for SyntaxKind usage patterns."""

    def test_can_be_used_as_dict_keys(self):
        """Test that SyntaxKind values can be used as dictionary keys."""
        d = {
            SyntaxKind.KEYWORD: "keyword",
            SyntaxKind.OPERATOR: "operator",
        }
        assert d[SyntaxKind.KEYWORD] == "keyword"

    def test_can_be_compared(self):
        """Test that SyntaxKind values can be compared."""
        assert SyntaxKind.KEYWORD == "keyword"
        assert SyntaxKind.OPERATOR != "keyword"
        assert SyntaxKind.EOF == "eof"

    def test_all_kinds_unique(self):
        """Test that all SyntaxKind values are unique."""
        values = [
            SyntaxKind.KEYWORD,
            SyntaxKind.OPERATOR,
            SyntaxKind.DELIMITER,
            SyntaxKind.LITERAL,
            SyntaxKind.IDENTIFIER,
            SyntaxKind.EOF,
        ]
        assert len(values) == len(set(values))