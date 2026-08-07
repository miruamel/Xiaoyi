/**
 * # Integer Primitives
 *
 * `int` provides signed and unsigned integer types with configurable
 * width, representation, and normalization.
 *
 * Path: `xiaoyi.domain.token.primitive.int`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `int` — integer type family.
 * - Layer 4: `kind`/`width`/`rep`/`normalize` — details.
 *
 * @module domain.token.primitive.int
 * @brief Integer primitive types with width and representation
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive
 * @see domain.token.primitive.int.kind
 * @see domain.token.primitive.int.width
 */
export * from "./kind";
export * from "./width";
export * from "./rep";
export * from "./normalize";

/**
 * Integer signedness.
 *
 * @brief Signed or unsigned classification
 * @group Domain
 * @since 0.1.0
 */
export enum IntKind {
  /** Signed integer (two's complement). */
  Signed = "signed",
  /** Unsigned integer. */
  Unsigned = "unsigned",
}

/**
 * Integer bit width.
 *
 * @brief Supported integer widths
 * @group Domain
 * @since 0.1.0
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

/**
 * Integer type with configurable signedness and width.
 *
 * @brief Parameterized integer type
 * @group Domain
 * @since 0.1.0
 * @see IntKind
 * @see IntWidth
 */
export interface IntType {
  /** Signed or unsigned. */
  kind: IntKind;
  /** Bit width. */
  width: IntWidth;
}

/**
 * Create new integer type.
 *
 * @param kind - Signedness
 * @param width - Bit width
 * @returns IntType instance
 * @since 0.1.0
 */
export function createIntType(kind: IntKind, width: IntWidth): IntType {
  return { kind, width };
}

/**
 * Get size in bytes.
 *
 * @param type - Integer type
 * @returns Byte size
 * @since 0.1.0
 */
export function intByteSize(type: IntType): number {
  return type.width / 8;
}

/**
 * Check if signed.
 *
 * @param type - Integer type
 * @returns true if signed
 * @since 0.1.0
 */
export function isIntSigned(type: IntType): boolean {
  return type.kind === IntKind.Signed;
}