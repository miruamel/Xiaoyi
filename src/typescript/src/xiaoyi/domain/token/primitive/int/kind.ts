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
import { IntKind, IntType, IntWidth } from "..";

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