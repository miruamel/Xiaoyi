/**
 * # 64-bit Float (f64)
 *
 * `f64` provides IEEE 754 double-precision floating-point type.
 *
 * Path: `xiaoyi.domain.token.primitive.float.f64`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `float`
 * - Layer 4: `f64`
 *
 * @module domain.token.primitive.float.f64
 * @brief IEEE 754 double-precision float
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive.float
 * @see domain.token.primitive.float.f32
 */

/** 64-bit float type alias. */
export type F64 = number;

/** f64 bit pattern. */
export type F64Bits = bigint;

/** f64 constants. */
export namespace F64Consts {
  /** Positive infinity. */
  export const INFINITY = Infinity;
  /** Negative infinity. */
  export const NEG_INFINITY = -Infinity;
  /** Not a Number. */
  export const NAN = NaN;
  /** Minimum positive normal value. */
  export const MIN_POSITIVE = 2.2250738585072014e-308;
  /** Maximum finite value. */
  export const MAX = 1.7976931348623157e+308;
  /** Minimum finite value. */
  export const MIN = -1.7976931348623157e+308;
  /** Epsilon (difference between 1.0 and next representable). */
  export const EPSILON = 2.220446049250313e-16;
}

/**
 * Check if value is finite.
 *
 * @param value - f64 value
 * @returns true if finite
 * @since 0.1.0
 */
export function isF64Finite(value: number): boolean {
  return Number.isFinite(value);
}

/**
 * Check if value is NaN.
 *
 * @param value - f64 value
 * @returns true if NaN
 * @since 0.1.0
 */
export function isF64NaN(value: number): boolean {
  return Number.isNaN(value);
}

/**
 * Check if value is infinite.
 *
 * @param value - f64 value
 * @returns true if infinite
 * @since 0.1.0
 */
export function isF64Infinite(value: number): boolean {
  return !Number.isFinite(value) && !Number.isNaN(value);
}