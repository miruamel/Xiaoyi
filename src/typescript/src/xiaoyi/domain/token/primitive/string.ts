/**
 * # String Primitive
 *
 * `string` provides UTF-8 string type with encoding validation.
 *
 * Path: `xiaoyi.domain.token.primitive.string`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `string`
 *
 * @module domain.token.primitive.string
 * @brief UTF-8 string primitive
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive
 * @see domain.token.primitive.int
 */

/** String type alias (owned UTF-8). */
export type String = string;

/** String slice type alias (borrowed UTF-8). */
export type Str = string;

/**
 * Create new empty string.
 *
 * @returns Empty String
 * @since 0.1.0
 */
export function newString(): String {
  return "";
}

/**
 * Create string from string slice.
 *
 * @param s - String slice
 * @returns Owned String
 * @since 0.1.0
 */
export function fromString(s: Str): String {
  return s;
}

/**
 * Check if string is valid UTF-8.
 *
 * @param bytes - Byte array
 * @returns true if valid UTF-8
 * @since 0.1.0
 */
export function isValidUtf8(bytes: Uint8Array): boolean {
  try {
    new TextDecoder("utf-8", { fatal: true }).decode(bytes);
    return true;
  } catch {
    return false;
  }
}

/**
 * Get string length in characters (code points).
 *
 * @param s - String slice
 * @returns Character count
 * @since 0.1.0
 */
export function charLen(s: Str): number {
  return [...s].length;
}