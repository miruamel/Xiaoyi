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
export { IntKind } from "./kind";
export { IntWidth } from "./width";
export * from "./rep";
export * from "./normalize";
export * from "./kind";
export * from "./width";