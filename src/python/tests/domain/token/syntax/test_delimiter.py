"""
Test suite for xiaoyi.domain.token.syntax.delimiter module.

@package xiaoyi.tests.domain.token.syntax
@brief Tests for Delimiter, DelimiterKind, matching functions
@since 0.1.0
"""

import pytest
from xiaoyi.domain.token.syntax.delimiter import (
    Delimiter,
    DelimiterKind,
    DELIMITERS,
    matching_close,
    matching_open,
    is_open_delimiter,
    is_close_delimiter,
    is_delimiter_pair,
)


class TestDelimiter:
    """Tests for Delimiter dataclass."""

    def test_delimiter_creation(self):
        """Test creating a Delimiter instance."""
        delim = Delimiter(text="(", kind=DelimiterKind.OPEN)
        assert delim.text == "("
        assert delim.kind == DelimiterKind.OPEN

    def test_delimiter_frozen(self):
        """Test that Delimiter is frozen (immutable)."""
        delim = Delimiter(text="(", kind=DelimiterKind.OPEN)
        with pytest.raises(AttributeError):
            delim.text = ")"


class TestDelimiterKind:
    """Tests for DelimiterKind enum."""

    def test_delimiter_kind_values(self):
        """Test all delimiter kinds exist."""
        assert DelimiterKind.OPEN == "open"
        assert DelimiterKind.CLOSE == "close"

    def test_delimiter_kind_iteration(self):
        """Test that all delimiter kinds can be iterated."""
        kinds = list(DelimiterKind)
        assert len(kinds) == 2


class TestDELIMITERS:
    """Tests for DELIMITERS tuple."""

    def test_delimiters_not_empty(self):
        """Test that DELIMITERS is not empty."""
        assert len(DELIMITERS) > 0

    def test_delimiters_are_delimiter_instances(self):
        """Test that all DELIMITERS are Delimiter instances."""
        for delim in DELIMITERS:
            assert isinstance(delim, Delimiter)

    def test_expected_delimiters_exist(self):
        """Test that expected delimiters exist."""
        texts = {d.text for d in DELIMITERS}
        expected = {"(", ")", "[", "]", "{", "}", "<", ">"}
        assert expected.issubset(texts)

    def test_open_delimiters(self):
        """Test open delimiters."""
        opens = [d for d in DELIMITERS if d.kind == DelimiterKind.OPEN]
        texts = {d.text for d in opens}
        assert {"(", "[", "{", "<"}.issubset(texts)

    def test_close_delimiters(self):
        """Test close delimiters."""
        closes = [d for d in DELIMITERS if d.kind == DelimiterKind.CLOSE]
        texts = {d.text for d in closes}
        assert {")", "]", "}", ">"}.issubset(texts)


class TestMatchingClose:
    """Tests for matching_close function."""

    def test_matching_close_valid(self):
        """Test getting matching close for open delimiters."""
        assert matching_close("(") == ")"
        assert matching_close("[") == "]"
        assert matching_close("{") == "}"
        assert matching_close("<") == ">"

    def test_matching_close_invalid(self):
        """Test getting matching close for invalid input returns None."""
        assert matching_close(")") is None
        assert matching_close("]") is None
        assert matching_close("}") is None
        assert matching_close(">") is None
        assert matching_close("invalid") is None
        assert matching_close("") is None


class TestMatchingOpen:
    """Tests for matching_open function."""

    def test_matching_open_valid(self):
        """Test getting matching open for close delimiters."""
        assert matching_open(")") == "("
        assert matching_open("]") == "["
        assert matching_open("}") == "{"
        assert matching_open(">") == "<"

    def test_matching_open_invalid(self):
        """Test getting matching open for invalid input returns None."""
        assert matching_open("(") is None
        assert matching_open("[") is None
        assert matching_open("{") is None
        assert matching_open("<") is None
        assert matching_open("invalid") is None
        assert matching_open("") is None


class TestIsOpenDelimiter:
    """Tests for is_open_delimiter function."""

    def test_is_open_delimiter_true(self):
        """Test is_open_delimiter returns True for open delimiters."""
        assert is_open_delimiter("(") is True
        assert is_open_delimiter("[") is True
        assert is_open_delimiter("{") is True
        assert is_open_delimiter("<") is True

    def test_is_open_delimiter_false(self):
        """Test is_open_delimiter returns False for close/other."""
        assert is_open_delimiter(")") is False
        assert is_open_delimiter("]") is False
        assert is_open_delimiter("}") is False
        assert is_open_delimiter(">") is False
        assert is_open_delimiter("invalid") is False
        assert is_open_delimiter("") is False


class TestIsCloseDelimiter:
    """Tests for is_close_delimiter function."""

    def test_is_close_delimiter_true(self):
        """Test is_close_delimiter returns True for close delimiters."""
        assert is_close_delimiter(")") is True
        assert is_close_delimiter("]") is True
        assert is_close_delimiter("}") is True
        assert is_close_delimiter(">") is True

    def test_is_close_delimiter_false(self):
        """Test is_close_delimiter returns False for open/other."""
        assert is_close_delimiter("(") is False
        assert is_close_delimiter("[") is False
        assert is_close_delimiter("{") is False
        assert is_close_delimiter("<") is False
        assert is_close_delimiter("invalid") is False
        assert is_close_delimiter("") is False


class TestIsDelimiterPair:
    """Tests for is_delimiter_pair function."""

    def test_is_delimiter_pair_true(self):
        """Test is_delimiter_pair returns True for matching pairs."""
        assert is_delimiter_pair("(", ")") is True
        assert is_delimiter_pair("[", "]") is True
        assert is_delimiter_pair("{", "}") is True
        assert is_delimiter_pair("<", ">") is True

    def test_is_delimiter_pair_false(self):
        """Test is_delimiter_pair returns False for non-matching."""
        assert is_delimiter_pair("(", "]") is False
        assert is_delimiter_pair("[", ")") is False
        assert is_delimiter_pair("{", ">") is False
        assert is_delimiter_pair("<", "}") is False
        assert is_delimiter_pair(")", "(") is False  # Wrong order
        assert is_delimiter_pair("invalid", "invalid") is False
        assert is_delimiter_pair("", "") is False


class TestDelimiterIntegration:
    """Integration tests for delimiter module."""

    def test_all_delimiters_have_valid_kinds(self):
        """Test that all delimiters have valid kinds."""
        for delim in DELIMITERS:
            assert delim.kind in DelimiterKind

    def test_matching_functions_consistent(self):
        """Test that matching functions are consistent."""
        for delim in DELIMITERS:
            if delim.kind == DelimiterKind.OPEN:
                close = matching_close(delim.text)
                assert close is not None
                assert matching_open(close) == delim.text
                assert is_delimiter_pair(delim.text, close) is True

            elif delim.kind == DelimiterKind.CLOSE:
                open_ = matching_open(delim.text)
                assert open_ is not None
                assert matching_close(open_) == delim.text
                assert is_delimiter_pair(open_, delim.text) is True

    def test_is_open_close_consistent(self):
        """Test is_open_delimiter and is_close_delimiter are consistent."""
        for delim in DELIMITERS:
            if delim.kind == DelimiterKind.OPEN:
                assert is_open_delimiter(delim.text) is True
                assert is_close_delimiter(delim.text) is False
            else:
                assert is_open_delimiter(delim.text) is False
                assert is_close_delimiter(delim.text) is True