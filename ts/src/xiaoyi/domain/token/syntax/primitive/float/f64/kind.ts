/**
 * Syntax primitive float token kind: F64.
 *
 * Layer hierarchy:
 * - 1 syntax
 * - 2 primitive
 * - 3 float
 * - 4 f64
 * - 5 kind
 */

export type F64Kind = 'F64_LITERAL' | 'F64_VAR' | 'F64_CAST';

export const F64_KIND: readonly F64Kind[] = [
  'F64_LITERAL',
  'F64_VAR',
  'F64_CAST',
];

export function labelF64Kind(kind: F64Kind): string {
  return kind;
}
