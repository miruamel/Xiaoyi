/**
 * # Float Primitives
 *
 * `float` provides floating-point types (f32, f64) with IEEE 754 compliance.
 *
 * Path: `xiaoyi.domain.token.primitive.float`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `float` — float type family.
 * - Layer 4: `f32`/`f64` — concrete types.
 *
 * @module domain.token.primitive.float
 * @brief IEEE 754 floating-point types
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive
 * @see domain.token.primitive.float.f32
 * @see domain.token.primitive.float.f64
 */
export * from "./f32";
export * from "./f64";

/**
 * Floating-point type kind.
 *
 * @brief Float precision classification
 * @group Domain
 * @since 0.1.0
 */
export enum FloatKind {
  /** 32-bit float (f32). */
  F32 = "f32",
  /** 64-bit float (f64). */
  F64 = "f64",
}