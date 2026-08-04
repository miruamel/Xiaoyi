/**
 * Syntax primitive integer token kind: INT8.
 *
 * Layer hierarchy:
 * - 1 syntax
 * - 2 primitive
 * - 3 int8
 * - 4 kind
 *
 * Concrete variant taxonomy for INT8 syntax nodes before rendering.
 */

export type Int8Kind = 'INT8_LITERAL' | 'INT8_VAR' | 'INT8_CAST';

export const INT8_KIND: readonly Int8Kind[] = [
  'INT8_LITERAL',
  'INT8_VAR',
  'INT8_CAST',
];

export function labelInt8Kind(kind: Int8Kind): string {
  return kind;
}
