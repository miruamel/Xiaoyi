/**
 * # Boolean Primitive
 *
 * `bool` provides the boolean type with true/false values.
 *
 * Path: `xiaoyi.domain.token.primitive.bool`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `primitive`
 * - Layer 3: `bool`
 *
 * @module domain.token.primitive.bool
 * @brief Boolean primitive type
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.primitive
 * @see domain.token.primitive.int
 */

/** Boolean type alias. */
export type Bool = boolean;

/** True value. */
export const TRUE: boolean = true;

/** False value. */
export const FALSE: boolean = false;

/**
 * Logical NOT.
 *
 * @param value - Boolean value
 * @returns Negated value
 * @since 0.1.0
 */
export function boolNot(value: boolean): boolean {
  return !value;
}

/**
 * Logical AND.
 *
 * @param a - First value
 * @param b - Second value
 * @returns a && b
 * @since 0.1.0
 */
export function boolAnd(a: boolean, b: boolean): boolean {
  return a && b;
}

/**
 * Logical OR.
 *
 * @param a - First value
 * @param b - Second value
 * @returns a || b
 * @since 0.1.0
 */
export function boolOr(a: boolean, b: boolean): boolean {
  return a || b;
}