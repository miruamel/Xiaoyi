"""
# Result Module

`result` provides Result type for error handling without exceptions.

Path: `xiaoyi.core.result`

- Layer 0: `core`
- Layer 1: `result` — result/status types.

@module xiaoyi.core.result
@brief Result type for fallible operations
@group Core
@since 0.1.0
@author Miruamel
@see xiaoyi.core.error
@see xiaoyi.core.config
"""

from typing import TypeVar, Generic, Callable, Awaitable, Union
from dataclasses import dataclass
from .error import XiaoyiError

T = TypeVar("T")
U = TypeVar("U")
E = TypeVar("E", bound=XiaoyiError)
F = TypeVar("F", bound=XiaoyiError)


@dataclass
class Ok(Generic[T]):
    """Success result."""
    value: T


@dataclass
class Err(Generic[E]):
    """Error result."""
    error: E


Result = Union[Ok[T], Err[E]]
"""Result type for operations that can fail."""


def ok(value: T) -> Ok[T]:
    """
    Success result constructor.

    @param value Success value
    @return Result with ok = True
    @since 0.1.0
    @group Core
    @example
    ```python
    result = ok(42)  # Ok(value=42)
    ```
    """
    return Ok(value=value)


def err(error: E) -> Err[E]:
    """
    Error result constructor.

    @param error Error value
    @return Result with ok = False
    @since 0.1.0
    @group Core
    @example
    ```python
    result = err(create_error(ErrorKind.CONFIG, "Missing file"))
    ```
    """
    return Err(error=error)


def is_ok(result: Result[T, E]) -> bool:
    """
    Check if result is success.

    @param result Result to check
    @return True if ok
    @since 0.1.0
    @group Core
    """
    return isinstance(result, Ok)


def is_err(result: Result[T, E]) -> bool:
    """
    Check if result is error.

    @param result Result to check
    @return True if error
    @since 0.1.0
    @group Core
    """
    return isinstance(result, Err)


def unwrap(result: Result[T, E]) -> T:
    """
    Unwrap success value or raise.

    @param result Result to unwrap
    @return Success value
    @raises Exception if result is error
    @since 0.1.0
    @group Core
    """
    if isinstance(result, Ok):
        return result.value
    raise result.error


def unwrap_err(result: Result[T, E]) -> E:
    """
    Unwrap error or raise.

    @param result Result to unwrap
    @return Error value
    @raises Exception if result is success
    @since 0.1.0
    @group Core
    """
    if isinstance(result, Err):
        return result.error
    raise ValueError("Expected error result")


def map(result: Result[T, E], fn: Callable[[T], U]) -> Result[U, E]:
    """
    Map success value.

    @param result Result to map
    @param fn Mapping function
    @return New result with mapped value
    @since 0.1.0
    @group Core
    """
    if isinstance(result, Ok):
        return Ok(value=fn(result.value))
    return result


def map_err(result: Result[T, E], fn: Callable[[E], F]) -> Result[T, F]:
    """
    Map error value.

    @param result Result to map
    @param fn Mapping function
    @return New result with mapped error
    @since 0.1.0
    @group Core
    """
    if isinstance(result, Err):
        return Err(error=fn(result.error))
    return Ok(value=result.value)


def and_then(result: Result[T, E], fn: Callable[[T], Result[U, E]]) -> Result[U, E]:
    """
    Chain fallible operations.

    @param result Result to chain
    @param fn Function returning new result
    @return Chained result
    @since 0.1.0
    @group Core
    """
    if isinstance(result, Ok):
        return fn(result.value)
    return result


def or_else(result: Result[T, E], fn: Callable[[E], Result[T, F]]) -> Result[T, F]:
    """
    Recover from error.

    @param result Result to recover
    @param fn Recovery function
    @return Recovered result
    @since 0.1.0
    @group Core
    """
    if isinstance(result, Err):
        return fn(result.error)
    return Ok(value=result.value)


async def to_awaitable(result: Result[T, E]) -> Result[T, E]:
    """
    Convert to awaitable for async compatibility.

    @param result Result to convert
    @return Awaitable result
    @since 0.1.0
    @group Core
    """
    return result


__all__ = [
    "Result",
    "Ok",
    "Err",
    "ok",
    "err",
    "is_ok",
    "is_err",
    "unwrap",
    "unwrap_err",
    "map",
    "map_err",
    "and_then",
    "or_else",
    "to_awaitable",
]