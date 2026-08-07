/**
 * # Integer Normalization
 *
 * `normalize` provides integer value normalization (clamping, wrapping).
 *
 * Path: `xiaoyi.domain.token.primitive.int.normalize`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `int`
 * - Layer 4: `normalize`
 *
 * @module domain.token.primitive.int.normalize
 * @brief Integer value normalization
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive.int
 * @see domain.token.primitive.int.width
 */
import { IntKind, IntType, IntWidth } from "..";

/**
 * Normalize integer value to fit within type bounds.
 *
 * @param value - Input value
 * @param intType - Target integer type
 * @returns Normalized value (clamped)
 * @since 0.1.0
 */
export function normalizeInt(value: number, intType: IntType): number {
  const bits = intType.width;
  const max = intType.kind === IntKind.Signed
    ? (1 << (bits - 1)) - 1
    : (1 << bits) - 1;
  const min = intType.kind === IntKind.Signed
    ? -(1 << (bits - 1))
    : 0;

  if (value > max) return max;
  if (value < min) return min;
  return value;
}

/**
 * Wrap integer value to fit within type bounds (modulo).
 *
 * @param value - Input value
 * @param intType - Target integer type
 * @returns Wrapped value
 * @since 0.1.0
 */
export function wrapInt(value: number, intType: IntType): number {
  const bits = intType.width;
  const range = 1 << bits;

  if (intType.kind === IntKind.Signed) {
    const half = 1 << (bits - 1);
    const wrapped = ((value + half) % range + range) % range;
    return wrapped - half;
  } else {
    return ((value % range) + range) % range;
  }
}

/**
 * Convert between integer types with overflow check.
 *
 * @param value - Source value
 * @param from - Source type
 * @param to - Target type
 * @returns Normalized value or throws on overflow
 * @since 0.1.0
 * @throws {Error} If overflow detected
 */
export function convertIntChecked(value: number, from: IntType, to: IntType): number {
  const normalized = normalizeInt(value, to);
  if (normalized !== value && from.width <= to.width) {
    throw new Error("Integer overflow");
  }
  return normalized;
}