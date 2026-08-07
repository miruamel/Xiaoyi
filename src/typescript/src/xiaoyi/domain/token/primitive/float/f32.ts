/**
 * # 32-bit Float (f32)
 *
 * `f32` provides IEEE 754 single-precision floating-point type.
 *
 * Path: `xiaoyi.domain.token.primitive.float.f32`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `float`
 * - Layer 4: `f32`
 *
 * @module domain.token.primitive.float.f32
 * @brief IEEE 754 single-precision float
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive.float
 * @see domain.token.primitive.float.f64
 */

/** 32-bit float type alias. */
export type F32 = number;

/** f32 bit pattern. */
export type F32Bits = number;

/** f32 constants. */
export namespace F32Consts {
  /** Positive infinity. */
  export const INFINITY = Infinity;
  /** Negative infinity. */
  export const NEG_INFINITY = -Infinity;
  /** Not a Number. */
  export const NAN = NaN;
  /** Minimum positive normal value. */
  export const MIN_POSITIVE = 1.175494351e-38;
  /** Maximum finite value. */
  export const MAX = 3.402823466e+38;
  /** Minimum finite value. */
  export const MIN = -3.402823466e+38;
  /** Epsilon (difference between 1.0 and next representable). */
  export const EPSILON = 1.19209290e-7;
}

/**
 * Check if value is finite.
 *
 * @param value - f32 value
 * @returns true if finite
 * @since 0.1.0
 */
export function isF32Finite(value: number): boolean {
  return Number.isFinite(value);
}

/**
 * Check if value is NaN.
 *
 * @param value - f32 value
 * @returns true if NaN
 * @since 0.1.0
 */
export function isF32NaN(value: number): boolean {
  return Number.isNaN(value);
}

/**
 * Check if value is infinite.
 *
 * @param value - f32 value
 * @returns true if infinite
 * @since 0.1.0
 */
export function isF32Infinite(value: number): boolean {
  return !Number.isFinite(value) && !Number.isNaN(value);
}