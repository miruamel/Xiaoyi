/**
 * # Error Module
 *
 * `error` provides error types and handling for the Xiaoyi framework.
 *
 * Path: `xiaoyi.core.error`
 *
 * - Layer 0: `core`
 * - Layer 1: `error` — error types and handling.
 *
 * @module core.error
 * @brief Error types and handling
 * @group Core
 * @since 0.1.0
 * @author Miruamel
 * @see core.config
 * @see core.result
 */

/**
 * Error kind classification.
 *
 * @brief Classification of error types
 * @group Core
 * @since 0.1.0
 */
export enum ErrorKind {
  /** Syntax error during parsing or compilation. */
  Syntax = "syntax",
  /** Parse error for structured data (JSON, TOML, etc.). */
  Parse = "parse",
  /** Runtime execution error. */
  Runtime = "runtime",
  /** I/O error (file, network, etc.). */
  Io = "io",
  /** Authentication/authorization failure. */
  Auth = "auth",
  /** Policy violation (rate limit, quota, etc.). */
  Policy = "policy",
  /** LLM provider error. */
  Llm = "llm",
  /** Memory system error (STM/LTM). */
  Memory = "memory",
  /** Tool execution error. */
  Tool = "tool",
  /** Workflow DAG execution error. */
  Workflow = "workflow",
  /** Configuration error. */
  Config = "config",
  /** State management error. */
  State = "state",
}

/**
 * Structured error with metadata for recovery decisions.
 *
 * @brief Structured error with context for error handling
 * @group Core
 * @since 0.1.0
 * @see ErrorKind
 */
export interface XiaoyiError extends Error {
  /** Error kind classification. */
  kind: ErrorKind;
  /** Additional metadata for error recovery. */
  meta: Record<string, string>;
}

/**
 * Create a new XiaoyiError.
 *
 * @param kind - Error kind
 * @param message - Error message
 * @param meta - Optional metadata
 * @returns New XiaoyiError instance
 * @since 0.1.0
 * @example
 * ```typescript
 * const error = createError(ErrorKind.Config, "Failed to load config", { path: "./config.toml" });
 * ```
 */
export function createError(kind: ErrorKind, message: string, meta: Record<string, string> = {}): XiaoyiError {
  const error = new Error(message) as XiaoyiError;
  error.name = "XiaoyiError";
  error.kind = kind;
  error.meta = meta;
  return error;
}

/**
 * Check if an error is a XiaoyiError.
 *
 * @param error - Error to check
 * @returns true if XiaoyiError
 * @since 0.1.0
 */
export function isXiaoyiError(error: unknown): error is XiaoyiError {
  return error instanceof Error && "kind" in error && "meta" in error;
}