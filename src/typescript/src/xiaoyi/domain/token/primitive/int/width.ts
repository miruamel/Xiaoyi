/**
 * # Integer Width
 *
 * `width` defines supported integer bit widths.
 *
 * Path: `xiaoyi.domain.token.primitive.int.width`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `int`
 * - Layer 4: `width`
 *
 * @module domain.token.primitive.int.width
 * @brief Integer bit width definitions
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive.int
 * @see domain.token.primitive.int.kind
 */
export enum IntWidth {
  /** 8-bit. */
  W8 = 8,
  /** 16-bit. */
  W16 = 16,
  /** 32-bit. */
  W32 = 32,
  /** 64-bit. */
  W64 = 64,
  /** 128-bit. */
  W128 = 128,
}

/** 8-bit integer width. */
export const W8: IntWidth = IntWidth.W8;

/** 16-bit integer width. */
export const W16: IntWidth = IntWidth.W16;

/** 32-bit integer width. */
export const W32: IntWidth = IntWidth.W32;

/** 64-bit integer width. */
export const W64: IntWidth = IntWidth.W64;

/** 128-bit integer width. */
export const W128: IntWidth = IntWidth.W128;

/**
 * Get default width (64-bit).
 *
 * @returns Default IntWidth
 * @since 0.1.0
 */
export function defaultWidth(): IntWidth {
  return IntWidth.W64;
}