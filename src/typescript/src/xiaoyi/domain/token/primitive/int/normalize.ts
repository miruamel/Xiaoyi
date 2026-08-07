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
import { IntKind, IntType, IntWidth } from "./kind";

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

  // Use BigInt for 32-bit and above to avoid JavaScript 32-bit signed limit
  if (bits >= 32) {
    const max = intType.kind === IntKind.Signed
      ? (1n << BigInt(bits - 1)) - 1n
      : (1n << BigInt(bits)) - 1n;
    const min = intType.kind === IntKind.Signed
      ? -(1n << BigInt(bits - 1))
      : 0n;

    const val = BigInt(value);
    if (val > max) return Number(max);
    if (val < min) return Number(min);
    return value;
  }

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

  if (bits >= 64) {
    const range = 1n << BigInt(bits);
    const val = BigInt(value);

    if (intType.kind === IntKind.Signed) {
      const half = 1n << BigInt(bits - 1);
      const wrapped = ((val + half) % range + range) % range;
      return Number(wrapped - half);
    } else {
      return Number(((val % range) + range) % range);
    }
  }

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
  if (normalized !== value) {
    throw new Error("Integer overflow");
  }
  return normalized;
}