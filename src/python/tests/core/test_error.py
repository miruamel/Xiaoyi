"""
Test suite for xiaoyi.core.error module.

@package xiaoyi.tests.core
@brief Tests for ErrorKind, XiaoyiError, create_error, is_xiaoyi_error
@since 0.1.0
"""

import pytest
from xiaoyi.core.error import (
    ErrorKind,
    XiaoyiError,
    create_error,
    is_xiaoyi_error,
)


class TestErrorKind:
    """Tests for ErrorKind enum."""

    def test_error_kind_values(self):
        """Test that all expected error kinds exist."""
        assert ErrorKind.SYNTAX == "syntax"
        assert ErrorKind.PARSE == "parse"
        assert ErrorKind.RUNTIME == "runtime"
        assert ErrorKind.IO == "io"
        assert ErrorKind.AUTH == "auth"
        assert ErrorKind.POLICY == "policy"
        assert ErrorKind.LLM == "llm"
        assert ErrorKind.MEMORY == "memory"
        assert ErrorKind.TOOL == "tool"
        assert ErrorKind.WORKFLOW == "workflow"
        assert ErrorKind.CONFIG == "config"
        assert ErrorKind.STATE == "state"

    def test_error_kind_is_str_enum(self):
        """Test that ErrorKind is a string enum."""
        assert isinstance(ErrorKind.SYNTAX, str)
        assert ErrorKind.SYNTAX == "syntax"

    def test_error_kind_iteration(self):
        """Test that all error kinds can be iterated."""
        kinds = list(ErrorKind)
        assert len(kinds) == 12


class TestXiaoyiError:
    """Tests for XiaoyiError class."""

    def test_create_xiaoyi_error(self):
        """Test creating a XiaoyiError instance."""
        error = XiaoyiError(
            kind=ErrorKind.CONFIG,
            message="Failed to load config",
            meta={"path": "./config.toml"}
        )
        assert error.kind == ErrorKind.CONFIG
        assert error.message == "Failed to load config"
        assert error.meta == {"path": "./config.toml"}

    def test_create_xiaoyi_error_empty_meta(self):
        """Test creating XiaoyiError with default empty meta."""
        error = XiaoyiError(kind=ErrorKind.RUNTIME, message="Runtime error")
        assert error.meta == {}

    def test_str_representation(self):
        """Test string representation of XiaoyiError."""
        error = XiaoyiError(kind=ErrorKind.AUTH, message="Invalid token")
        assert str(error) == "[auth] Invalid token"

    def test_with_meta(self):
        """Test adding metadata to error."""
        error = XiaoyiError(kind=ErrorKind.IO, message="File not found")
        error.with_meta("path", "/tmp/test.txt")
        assert error.meta == {"path": "/tmp/test.txt"}

    def test_with_meta_chaining(self):
        """Test chaining with_meta calls."""
        error = (
            XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
            .with_meta("key1", "value1")
            .with_meta("key2", "value2")
        )
        assert error.meta == {"key1": "value1", "key2": "value2"}

    def test_is_exception(self):
        """Test that XiaoyiError is an Exception."""
        error = XiaoyiError(kind=ErrorKind.RUNTIME, message="Test")
        assert isinstance(error, Exception)

    def test_raise_xiaoyi_error(self):
        """Test raising and catching XiaoyiError."""
        with pytest.raises(XiaoyiError) as exc_info:
            raise XiaoyiError(kind=ErrorKind.LLM, message="LLM error")
        assert exc_info.value.kind == ErrorKind.LLM


class TestCreateError:
    """Tests for create_error function."""

    def test_create_error_basic(self):
        """Test basic error creation."""
        error = create_error(ErrorKind.CONFIG, "Config load failed")
        assert isinstance(error, XiaoyiError)
        assert error.kind == ErrorKind.CONFIG
        assert error.message == "Config load failed"
        assert error.meta == {}

    def test_create_error_with_meta(self):
        """Test error creation with metadata."""
        error = create_error(
            ErrorKind.PARSE,
            "Parse failed",
            {"line": "10", "column": "5"}
        )
        assert error.meta == {"line": "10", "column": "5"}

    def test_create_error_none_meta(self):
        """Test error creation with explicit None meta."""
        error = create_error(ErrorKind.RUNTIME, "Runtime error", None)
        assert error.meta == {}


class TestIsXiaoyiError:
    """Tests for is_xiaoyi_error function."""

    def test_is_xiaoyi_error_true(self):
        """Test that XiaoyiError returns True."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Test")
        assert is_xiaoyi_error(error) is True

    def test_is_xiaoyi_error_false_exception(self):
        """Test that regular Exception returns False."""
        assert is_xiaoyi_error(ValueError("test")) is False

    def test_is_xiaoyi_error_false_none(self):
        """Test that None returns False."""
        assert is_xiaoyi_error(None) is False

    def test_is_xiaoyi_error_false_string(self):
        """Test that string returns False."""
        assert is_xiaoyi_error("error message") is False

    def test_is_xiaoyi_error_false_dict(self):
        """Test that dict returns False."""
        assert is_xiaoyi_error({"error": "test"}) is False