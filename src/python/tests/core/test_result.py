"""
Test suite for xiaoyi.core.result module.

@package xiaoyi.tests.core
@brief Tests for Result type and operations
@since 0.1.0
"""

import pytest
from xiaoyi.core.result import (
    Ok,
    Err,
    Result,
    ok,
    err,
    is_ok,
    is_err,
    unwrap,
    unwrap_err,
    map,
    map_err,
    and_then,
    or_else,
    to_awaitable,
)
from xiaoyi.core.error import XiaoyiError, ErrorKind


class TestOk:
    """Tests for Ok variant."""

    def test_ok_creation(self):
        """Test creating Ok with value."""
        result = Ok(value=42)
        assert result.value == 42

    def test_ok_via_function(self):
        """Test creating Ok via ok() function."""
        result = ok(42)
        assert result.value == 42
        assert isinstance(result, Ok)

    def test_ok_is_ok(self):
        """Test is_ok returns True for Ok."""
        assert is_ok(ok(42)) is True

    def test_ok_is_not_err(self):
        """Test is_err returns False for Ok."""
        assert is_err(ok(42)) is False


class TestErr:
    """Tests for Err variant."""

    def test_err_creation(self):
        """Test creating Err with error."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        result = Err(error=error)
        assert result.error == error

    def test_err_via_function(self):
        """Test creating Err via err() function."""
        error = XiaoyiError(kind=ErrorKind.RUNTIME, message="Runtime error")
        result = err(error)
        assert result.error == error
        assert isinstance(result, Err)

    def test_err_is_err(self):
        """Test is_err returns True for Err."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        assert is_err(err(error)) is True

    def test_err_is_not_ok(self):
        """Test is_ok returns False for Err."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        assert is_ok(err(error)) is False


class TestUnwrap:
    """Tests for unwrap function."""

    def test_unwrap_ok(self):
        """Test unwrapping Ok returns value."""
        assert unwrap(ok(42)) == 42
        assert unwrap(ok("hello")) == "hello"
        assert unwrap(ok([1, 2, 3])) == [1, 2, 3]

    def test_unwrap_err_raises(self):
        """Test unwrapping Err raises the error."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        with pytest.raises(XiaoyiError) as exc_info:
            unwrap(err(error))
        assert exc_info.value == error


class TestUnwrapErr:
    """Tests for unwrap_err function."""

    def test_unwrap_err_err(self):
        """Test unwrapping Err returns error."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        assert unwrap_err(err(error)) == error

    def test_unwrap_err_ok_raises(self):
        """Test unwrapping Ok raises ValueError."""
        with pytest.raises(ValueError, match="Expected error result"):
            unwrap_err(ok(42))


class TestMap:
    """Tests for map function."""

    def test_map_ok(self):
        """Test mapping Ok transforms value."""
        result = map(ok(42), lambda x: x * 2)
        assert isinstance(result, Ok)
        assert result.value == 84

    def test_map_ok_chaining(self):
        """Test chaining map operations."""
        result = map(ok(2), lambda x: x + 1)
        result = map(result, lambda x: x * 3)
        assert result.value == 9

    def test_map_err_unchanged(self):
        """Test mapping Err leaves it unchanged."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        result = map(err(error), lambda x: x * 2)
        assert isinstance(result, Err)
        assert result.error == error

    def test_map_with_different_types(self):
        """Test mapping changes the value type."""
        result = map(ok(42), lambda x: str(x))
        assert result.value == "42"


class TestMapErr:
    """Tests for map_err function."""

    def test_map_err_err(self):
        """Test mapping Err transforms error."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        new_error = XiaoyiError(kind=ErrorKind.RUNTIME, message="Runtime error")
        result = map_err(err(error), lambda _: new_error)
        assert isinstance(result, Err)
        assert result.error == new_error

    def test_map_err_ok_unchanged(self):
        """Test mapping Ok leaves it unchanged."""
        result = map_err(ok(42), lambda e: XiaoyiError(kind=ErrorKind.RUNTIME, message="New"))
        assert isinstance(result, Ok)
        assert result.value == 42


class TestAndThen:
    """Tests for and_then function."""

    def test_and_then_ok(self):
        """Test and_then on Ok chains operations."""
        result = and_then(ok(42), lambda x: ok(x * 2))
        assert isinstance(result, Ok)
        assert result.value == 84

    def test_and_then_ok_returns_err(self):
        """Test and_then when function returns Err."""
        error = XiaoyiError(kind=ErrorKind.RUNTIME, message="Runtime error")
        result = and_then(ok(42), lambda _: err(error))
        assert isinstance(result, Err)
        assert result.error == error

    def test_and_then_err_unchanged(self):
        """Test and_then on Err leaves it unchanged."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        result = and_then(err(error), lambda x: ok(x * 2))
        assert isinstance(result, Err)
        assert result.error == error


class TestOrElse:
    """Tests for or_else function."""

    def test_or_else_err(self):
        """Test or_else on Err provides fallback."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        result = or_else(err(error), lambda _: ok(99))
        assert isinstance(result, Ok)
        assert result.value == 99

    def test_or_else_err_returns_err(self):
        """Test or_else when fallback returns Err."""
        error1 = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        error2 = XiaoyiError(kind=ErrorKind.RUNTIME, message="Runtime error")
        result = or_else(err(error1), lambda _: err(error2))
        assert isinstance(result, Err)
        assert result.error == error2

    def test_or_else_ok_unchanged(self):
        """Test or_else on Ok leaves it unchanged."""
        result = or_else(ok(42), lambda _: ok(99))
        assert isinstance(result, Ok)
        assert result.value == 42


class TestToAwaitable:
    """Tests for to_awaitable function."""

    @pytest.mark.asyncio
    async def test_to_awaitable_ok(self):
        """Test to_awaitable on Ok."""
        result = await to_awaitable(ok(42))
        assert isinstance(result, Ok)
        assert result.value == 42

    @pytest.mark.asyncio
    async def test_to_awaitable_err(self):
        """Test to_awaitable on Err."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        result = await to_awaitable(err(error))
        assert isinstance(result, Err)
        assert result.error == error


class TestResultIntegration:
    """Integration tests for Result operations."""

    def test_complex_chain(self):
        """Test complex operation chain."""
        result = (
            ok(10)
            |> (lambda r: and_then(r, lambda x: ok(x + 5)))
            |> (lambda r: map(r, lambda x: x * 2))
            |> (lambda r: or_else(r, lambda _: ok(0)))
        )
        # Using function composition instead of pipe operator
        result = ok(10)
        result = and_then(result, lambda x: ok(x + 5))
        result = map(result, lambda x: x * 2)
        result = or_else(result, lambda _: ok(0))
        assert result.value == 30

    def test_error_short_circuit(self):
        """Test that errors short-circuit the chain."""
        error = XiaoyiError(kind=ErrorKind.CONFIG, message="Config error")
        result = err(error)
        result = and_then(result, lambda x: ok(x + 5))
        result = map(result, lambda x: x * 2)
        result = or_else(result, lambda _: ok(99))
        assert result.value == 99