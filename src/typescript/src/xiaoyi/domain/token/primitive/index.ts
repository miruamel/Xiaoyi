/**
 * # Primitive Types
 *
 * `primitive` defines the fundamental primitive types: integers,
 * floats, booleans, and strings with their representations.
 *
 * Path: `xiaoyi.domain.token.primitive`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive` — primitive type system.
 * - Layer 3: `int`/`float`/`bool`/`string` — type families.
 * - Layer 4: `kind`/`width`/`rep`/`normalize` — type details.
 *
 * @module domain.token.primitive
 * @brief Fundamental primitive type definitions
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token
 * @see domain.token.primitive.int
 * @see domain.token.primitive.float
 */
export * from "./int";
export * from "./float";
export * from "./bool";
export * from "./string";

/**
 * Primitive type kind.
 *
 * @brief Classification of primitive types
 * @group Domain
 * @since 0.1.0
 */
export enum PrimitiveKind {
  /** Signed/unsigned integer. */
  Int = "int",
  /** Floating point. */
  Float = "float",
  /** Boolean. */
  Bool = "bool",
  /** UTF-8 string. */
  String = "string",
}