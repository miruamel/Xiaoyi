/**
 * # Result Module
 *
 * `result` provides Result type for error handling without exceptions.
 *
 * Path: `xiaoyi.core.result`
 *
 * - Layer 0: `core`
 * - Layer 1: `result` — result/status types.
 *
 * @module core.result
 * @brief Result type for fallible operations
 * @group Core
 * @since 0.1.0
 * @author Miruamel
 * @see core.error
 * @see core.config
 */
import { XiaoyiError } from "./error";

/**
 * Result type for operations that can fail.
 *
 * @typeParam T - Success value type
 * @typeParam E - Error type (defaults to XiaoyiError)
 * @since 0.1.0
 * @group Core
 */
export type Result<T, E = XiaoyiError> =
  | { ok: true; value: T }
  | { ok: false; error: E };

/**
 * Success result constructor.
 *
 * @param value - Success value
 * @returns Result with ok = true
 * @since 0.1.0
 * @group Core
 * @example
 * ```typescript
 * const result = ok(42); // { ok: true, value: 42 }
 * ```
 */
export function ok<T>(value: T): Result<T, never> {
  return { ok: true, value };
}

/**
 * Error result constructor.
 *
 * @param error - Error value
 * @returns Result with ok = false
 * @since 0.1.0
 * @group Core
 * @example
 * ```typescript
 * const result = err(createError(ErrorKind.Config, "Missing file"));
 * ```
 */
export function err<E>(error: E): Result<never, E> {
  return { ok: false, error };
}

/**
 * Check if result is success.
 *
 * @param result - Result to check
 * @returns true if ok
 * @since 0.1.0
 * @group Core
 */
export function isOk<T, E>(result: Result<T, E>): result is { ok: true; value: T } {
  return result.ok;
}

/**
 * Check if result is error.
 *
 * @param result - Result to check
 * @returns true if error
 * @since 0.1.0
 * @group Core
 */
export function isErr<T, E>(result: Result<T, E>): result is { ok: false; error: E } {
  return !result.ok;
}

/**
 * Unwrap success value or throw.
 *
 * @param result - Result to unwrap
 * @returns Success value
 * @throws Error if result is error
 * @since 0.1.0
 * @group Core
 */
export function unwrap<T, E>(result: Result<T, E>): T {
  if (result.ok) return result.value;
  throw result.error;
}

/**
 * Unwrap error or throw.
 *
 * @param result - Result to unwrap
 * @returns Error value
 * @throws Error if result is success
 * @since 0.1.0
 * @group Core
 */
export function unwrapErr<T, E>(result: Result<T, E>): E {
  if (!result.ok) return result.error;
  throw new Error("Expected error result");
}

/**
 * Map success value.
 *
 * @param result - Result to map
 * @param fn - Mapping function
 * @returns New result with mapped value
 * @since 0.1.0
 * @group Core
 */
export function map<T, U, E>(result: Result<T, E>, fn: (value: T) => U): Result<U, E> {
  if (result.ok) return ok(fn(result.value));
  return result;
}

/**
 * Map error value.
 *
 * @param result - Result to map
 * @param fn - Mapping function
 * @returns New result with mapped error
 * @since 0.1.0
 * @group Core
 */
export function mapErr<T, E, F>(result: Result<T, E>, fn: (error: E) => F): Result<T, F> {
  if (!result.ok) return err(fn(result.error));
  return ok(result.value);
}

/**
 * Chain fallible operations.
 *
 * @param result - Result to chain
 * @param fn - Function returning new result
 * @returns Chained result
 * @since 0.1.0
 * @group Core
 */
export function andThen<T, U, E>(result: Result<T, E>, fn: (value: T) => Result<U, E>): Result<U, E> {
  if (result.ok) return fn(result.value);
  return result;
}

/**
 * Recover from error.
 *
 * @param result - Result to recover
 * @param fn - Recovery function
 * @returns Recovered result
 * @since 0.1.0
 * @group Core
 */
export function orElse<T, E, F>(result: Result<T, E>, fn: (error: E) => Result<T, F>): Result<T, F> {
  if (!result.ok) return fn(result.error);
  return ok(result.value);
}

/**
 * Convert to Promise for async compatibility.
 *
 * @param result - Result to convert
 * @returns Promise resolving to result
 * @since 0.1.0
 * @group Core
 */
export async function toPromise<T, E>(result: Result<T, E>): Promise<Result<T, E>> {
  return result;
}

// Re-export ErrorKind and XiaoyiError from error module
export type { ErrorKind, XiaoyiError } from "./error";
export { createError, isXiaoyiError } from "./error";