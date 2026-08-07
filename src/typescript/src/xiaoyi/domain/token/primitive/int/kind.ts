/**
 * # Integer Kind
 *
 * `kind` defines signed vs unsigned integer classification.
 *
 * Path: `xiaoyi.domain.token.primitive.int.kind`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `int`
 * - Layer 4: `kind`
 *
 * @module domain.token.primitive.int.kind
 * @brief Integer signedness classification
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive.int
 * @see domain.token.primitive.int.width
 */
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

/** Signed integer (two's complement representation). */
export const SIGNED: IntKind = IntKind.Signed;

/** Unsigned integer. */
export const UNSIGNED: IntKind = IntKind.Unsigned;

/**
 * Get default integer type (signed 64-bit).
 *
 * @returns Default IntType
 * @since 0.1.0
 */
export function defaultIntType(): IntType {
  return { kind: IntKind.Signed, width: IntWidth.W64 };
}