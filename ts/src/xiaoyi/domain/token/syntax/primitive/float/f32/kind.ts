/**
 * Syntax primitive float token kind: F32.
 *
 * Layer hierarchy:
 * - 1 syntax
 * - 2 primitive
 * - 3 float
 * - 4 f32
 * - 5 kind
 */

export type F32Kind = 'F32_LITERAL' | 'F32_VAR' | 'F32_CAST';

export const F32_KIND: readonly F32Kind[] = [
  'F32_LITERAL',
  'F32_VAR',
  'F32_CAST',
];

export function labelF32Kind(kind: F32Kind): string {
  return kind;
}
