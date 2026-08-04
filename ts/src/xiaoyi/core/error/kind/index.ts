/**
 * Layer 0 - Foundation / Core Error Kind
 *
 * Path: xiaoyi.core.error.kind
 *
 * Layer hierarchy:
 * - 0: core — foundational cross-cutting types.
 * - 1: error — unified exception taxonomy.
 * - 2: kind — categorical failure model.
 */

export type ErrorKind =
  | 'syntax'
  | 'parse'
  | 'runtime'
  | 'io'
  | 'auth'
  | 'policy'
  | 'llm'
  | 'memory'
  | 'tool'
  | 'workflow'
  | 'config'
  | 'state';

export const ERROR_KIND: readonly ErrorKind[] = [
  'syntax',
  'parse',
  'runtime',
  'io',
  'auth',
  'policy',
  'llm',
  'memory',
  'tool',
  'workflow',
  'config',
  'state',
];
