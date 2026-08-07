"""
Test suite for xiaoyi.domain.token.primitive.string module.

@package xiaoyi.tests.domain.token.primitive
@brief Tests for String, Str, new_string, from_string, is_valid_utf8, char_len
@since 0.1.0
"""

import pytest
from xiaoyi.domain.token.primitive.string import (
    String,
    Str,
    new_string,
    from_string,
    is_valid_utf8,
    char_len,
)


class TestStringTypeAliases:
    """Tests for String and Str type aliases."""

    def test_string_is_str(self):
        """Test that String is str."""
        assert String is str

    def test_str_is_str(self):
        """Test that Str is str."""
        assert Str is str

    def test_string_usage(self):
        """Test using String type alias."""
        s: String = "hello"
        assert s == "hello"

    def test_str_usage(self):
        """Test using Str type alias."""
        s: Str = "hello"
        assert s == "hello"


class TestNewString:
    """Tests for new_string function."""

    def test_new_string_returns_empty(self):
        """Test that new_string returns empty string."""
        s = new_string()
        assert s == ""
        assert isinstance(s, str)

    def test_new_string_multiple_calls(self):
        """Test multiple calls return independent strings."""
        s1 = new_string()
        s2 = new_string()
        assert s1 == ""
        assert s2 == ""
        # They are both empty strings, but should be independent
        s1 += "a"
        assert s2 == ""


class TestFromString:
    """Tests for from_string function."""

    def test_from_string_basic(self):
        """Test creating string from string slice."""
        s = from_string("hello")
        assert s == "hello"
        assert isinstance(s, str)

    def test_from_string_empty(self):
        """Test creating string from empty slice."""
        s = from_string("")
        assert s == ""

    def test_from_string_unicode(self):
        """Test creating string with unicode."""
        s = from_string("Hello, 世界! 🌍")
        assert s == "Hello, 世界! 🌍"


class TestIsValidUtf8:
    """Tests for is_valid_utf8 function."""

    def test_is_valid_utf8_valid(self):
        """Test valid UTF-8 returns True."""
        assert is_valid_utf8(b"hello") is True
        assert is_valid_utf8("hello".encode("utf-8")) is True
        assert is_valid_utf8("Hello, 世界! 🌍".encode("utf-8")) is True
        assert is_valid_utf8(b"") is True

    def test_is_valid_utf8_invalid(self):
        """Test invalid UTF-8 returns False."""
        # Invalid UTF-8 sequence
        assert is_valid_utf8(b"\xff\xfe") is False
        assert is_valid_utf8(b"\x80\x81") is False
        assert is_valid_utf8(b"\xc0\xc1") is False

    def test_is_valid_utf8_partial(self):
        """Test partial UTF-8 sequences return False."""
        # Incomplete multi-byte sequence
        assert is_valid_utf8(b"\xe2\x82") is False  # Incomplete euro sign


class TestCharLen:
    """Tests for char_len function."""

    def test_char_len_ascii(self):
        """Test character length for ASCII strings."""
        assert char_len("hello") == 5
        assert char_len("") == 0
        assert char_len("a") == 1

    def test_char_len_unicode(self):
        """Test character length for Unicode strings."""
        assert char_len("世界") == 2  # 2 Chinese characters
        assert char_len("🌍") == 1   # 1 emoji
        assert char_len("Hello, 世界! 🌍") == 13  # 13 code points

    def test_char_len_combining_chars(self):
        """Test character length with combining characters."""
        # e + combining acute accent = 2 code points, 1 grapheme
        assert char_len("e\u0301") == 2

    def test_char_len_emoji_sequences(self):
        """Test character length for emoji sequences."""
        # Family emoji = multiple code points
        assert char_len("👨‍👩‍👧‍👦") >= 1  # Multiple code points


class TestStringIntegration:
    """Integration tests for string module."""

    def test_roundtrip_new_from(self):
        """Test new_string -> from_string roundtrip."""
        s1 = new_string()
        s2 = from_string(s1)
        assert s2 == ""

    def test_char_len_matches_python_len(self):
        """Test char_len matches Python's len for strings."""
        test_strings = [
            "",
            "a",
            "hello",
            "世界",
            "🌍",
            "Hello, 世界! 🌍",
        ]
        for s in test_strings:
            assert char_len(s) == len(s)

    def test_is_valid_utf8_roundtrip(self):
        """Test that valid strings encode to valid UTF-8."""
        test_strings = [
            "",
            "hello",
            "世界",
            "🌍",
            "Hello, 世界! 🌍",
        ]
        for s in test_strings:
            encoded = s.encode("utf-8")
            assert is_valid_utf8(encoded) is True