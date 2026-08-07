/**
 * # Integer Representation
 *
 * `rep` defines integer representation details (endianness, encoding).
 *
 * Path: `xiaoyi.domain.token.primitive.int.rep`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `int`
 * - Layer 4: `rep`
 *
 * @module domain.token.primitive.int.rep
 * @brief Integer representation details
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive.int
 * @see domain.token.primitive.int.kind
 */
import { IntKind, IntWidth } from "..";

/** Integer endianness. */
export enum Endianness {
  /** Little-endian (least significant byte first). */
  Little = "little",
  /** Big-endian (most significant byte first). */
  Big = "big",
  /** Native endianness. */
  Native = "native",
}

/** Get native endianness. */
export function nativeEndianness(): Endianness {
  // TypeScript runs on little-endian in practice
  return Endianness.Little;
}

/** Default integer representation: signed, 64-bit, little-endian. */
export const DEFAULT_REP: [IntKind, IntWidth, Endianness] = [
  IntKind.Signed,
  IntWidth.W64,
  Endianness.Little,
];