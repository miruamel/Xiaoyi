"""
Test suite for xiaoyi.domain.token.syntax.keyword module.

@package xiaoyi.tests.domain.token.syntax
@brief Tests for Keyword, KeywordKind, KEYWORDS, keyword_from_ident, is_keyword
@since 0.1.0
"""

import pytest
from xiaoyi.domain.token.syntax.keyword import (
    Keyword,
    KeywordKind,
    KEYWORDS,
    keyword_from_ident,
    is_keyword,
)


class TestKeyword:
    """Tests for Keyword dataclass."""

    def test_keyword_creation(self):
        """Test creating a Keyword instance."""
        kw = Keyword(text="if", kind=KeywordKind.CONTROL_FLOW)
        assert kw.text == "if"
        assert kw.kind == KeywordKind.CONTROL_FLOW

    def test_keyword_frozen(self):
        """Test that Keyword is frozen (immutable)."""
        kw = Keyword(text="if", kind=KeywordKind.CONTROL_FLOW)
        with pytest.raises(AttributeError):
            kw.text = "else"


class TestKeywordKind:
    """Tests for KeywordKind enum."""

    def test_keyword_kind_values(self):
        """Test all expected keyword kinds exist."""
        assert KeywordKind.CONTROL_FLOW == "control_flow"
        assert KeywordKind.DECLARATION == "declaration"
        assert KeywordKind.TYPE == "type"
        assert KeywordKind.MODULE == "module"
        assert KeywordKind.ASYNC == "async"
        assert KeywordKind.ERROR_HANDLING == "error_handling"

    def test_keyword_kind_iteration(self):
        """Test that all keyword kinds can be iterated."""
        kinds = list(KeywordKind)
        assert len(kinds) == 6


class TestKEYWORDS:
    """Tests for KEYWORDS tuple."""

    def test_keywords_not_empty(self):
        """Test that KEYWORDS is not empty."""
        assert len(KEYWORDS) > 0

    def test_keywords_are_keyword_instances(self):
        """Test that all KEYWORDS are Keyword instances."""
        for kw in KEYWORDS:
            assert isinstance(kw, Keyword)

    def test_keywords_have_unique_text(self):
        """Test that all keywords have unique text."""
        texts = [kw.text for kw in KEYWORDS]
        assert len(texts) == len(set(texts))

    def test_expected_keywords_exist(self):
        """Test that expected language keywords exist."""
        keyword_texts = {kw.text for kw in KEYWORDS}
        expected = {
            "if", "else", "while", "for", "return",
            "let", "const", "fn", "struct", "enum",
            "int", "float", "bool", "string",
            "mod", "use", "pub",
            "async", "await", "spawn",
            "try", "catch", "throw",
        }
        assert expected.issubset(keyword_texts)


class TestKeywordFromIdent:
    """Tests for keyword_from_ident function."""

    def test_keyword_from_ident_valid(self):
        """Test getting keyword from valid identifier."""
        kw = keyword_from_ident("if")
        assert kw is not None
        assert kw.text == "if"
        assert kw.kind == KeywordKind.CONTROL_FLOW

    def test_keyword_from_ident_invalid(self):
        """Test getting keyword from invalid identifier returns None."""
        assert keyword_from_ident("not_a_keyword") is None
        assert keyword_from_ident("") is None
        assert keyword_from_ident("variable_name") is None

    def test_keyword_from_ident_case_sensitive(self):
        """Test that keyword lookup is case-sensitive."""
        assert keyword_from_ident("IF") is None
        assert keyword_from_ident("If") is None
        assert keyword_from_ident("if") is not None


class TestIsKeyword:
    """Tests for is_keyword function."""

    def test_is_keyword_true(self):
        """Test is_keyword returns True for keywords."""
        assert is_keyword("if") is True
        assert is_keyword("else") is True
        assert is_keyword("fn") is True
        assert is_keyword("async") is True

    def test_is_keyword_false(self):
        """Test is_keyword returns False for non-keywords."""
        assert is_keyword("variable") is False
        assert is_keyword("my_function") is False
        assert is_keyword("") is False
        assert is_keyword("IF") is False  # Case sensitive

    def test_is_keyword_consistency(self):
        """Test is_keyword is consistent with keyword_from_ident."""
        for kw in KEYWORDS:
            assert is_keyword(kw.text) is True
            assert keyword_from_ident(kw.text) is not None

        # Non-keywords
        assert is_keyword("notakeyword") is False
        assert keyword_from_ident("notakeyword") is None


class TestKeywordIntegration:
    """Integration tests for keyword module."""

    def test_all_keywords_have_kinds(self):
        """Test that all keywords have valid kinds."""
        for kw in KEYWORDS:
            assert kw.kind in KeywordKind

    def test_control_flow_keywords(self):
        """Test control flow keywords."""
        control_flow = [kw for kw in KEYWORDS if kw.kind == KeywordKind.CONTROL_FLOW]
        texts = {kw.text for kw in control_flow}
        assert {"if", "else", "while", "for", "return"}.issubset(texts)

    def test_declaration_keywords(self):
        """Test declaration keywords."""
        declaration = [kw for kw in KEYWORDS if kw.kind == KeywordKind.DECLARATION]
        texts = {kw.text for kw in declaration}
        assert {"let", "const", "fn", "struct", "enum"}.issubset(texts)

    def test_type_keywords(self):
        """Test type keywords."""
        types = [kw for kw in KEYWORDS if kw.kind == KeywordKind.TYPE]
        texts = {kw.text for kw in types}
        assert {"int", "float", "bool", "string"}.issubset(texts)