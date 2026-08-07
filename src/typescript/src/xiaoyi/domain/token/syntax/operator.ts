/**
 * # Operators
 *
 * `operator` defines all operators with precedence and associativity.
 *
 * Path: `xiaoyi.domain.token.syntax.operator`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `syntax`
 * - Layer 3: `operator`
 *
 * @module domain.token.syntax.operator
 * @brief Operators with precedence
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.syntax
 * @see domain.token.syntax.keyword
 */
import { SyntaxKind } from "..";

/** Operator token. */
export interface Operator {
  /** Operator symbol. */
  readonly symbol: string;
  /** Operator kind. */
  readonly kind: OperatorKind;
  /** Precedence (higher = tighter binding). */
  readonly precedence: number;
  /** Associativity. */
  readonly associativity: Associativity;
}

/** Operator categories. */
export enum OperatorKind {
  /** Arithmetic (+, -, *, /, %) */
  Arithmetic = "arithmetic",
  /** Comparison (==, !=, <, >, <=, >=) */
  Comparison = "comparison",
  /** Logical (&&, ||, !) */
  Logical = "logical",
  /** Bitwise (&, |, ^, ~, <<, >>) */
  Bitwise = "bitwise",
  /** Assignment (=, +=, -=, *=, /=, %=) */
  Assignment = "assignment",
  /** Member access (., .., ?.) */
  MemberAccess = "member_access",
  /** Call/Index ((), []) */
  CallIndex = "call_index",
}

/** Associativity. */
export enum Associativity {
  /** Left associative (a + b + c = (a + b) + c) */
  Left = "left",
  /** Right associative (a = b = c = a = (b = c)) */
  Right = "right",
  /** Non-associative (a < b < c is invalid) */
  None = "none",
}

/** All operators ordered by precedence (highest first). */
export const OPERATORS: readonly Operator[] = Object.freeze([
  // Member access / call / index (highest precedence)
  { symbol: ".", kind: OperatorKind.MemberAccess, precedence: 15, associativity: Associativity.Left },
  { symbol: "..", kind: OperatorKind.MemberAccess, precedence: 15, associativity: Associativity.Left },
  { symbol: "?.", kind: OperatorKind.MemberAccess, precedence: 15, associativity: Associativity.Left },
  { symbol: "()", kind: OperatorKind.CallIndex, precedence: 15, associativity: Associativity.Left },
  { symbol: "[]", kind: OperatorKind.CallIndex, precedence: 15, associativity: Associativity.Left },
  // Unary
  { symbol: "-", kind: OperatorKind.Arithmetic, precedence: 14, associativity: Associativity.Right },
  { symbol: "!", kind: OperatorKind.Logical, precedence: 14, associativity: Associativity.Right },
  { symbol: "~", kind: OperatorKind.Bitwise, precedence: 14, associativity: Associativity.Right },
  // Multiplicative
  { symbol: "*", kind: OperatorKind.Arithmetic, precedence: 13, associativity: Associativity.Left },
  { symbol: "/", kind: OperatorKind.Arithmetic, precedence: 13, associativity: Associativity.Left },
  { symbol: "%", kind: OperatorKind.Arithmetic, precedence: 13, associativity: Associativity.Left },
  // Additive
  { symbol: "+", kind: OperatorKind.Arithmetic, precedence: 12, associativity: Associativity.Left },
  { symbol: "-", kind: OperatorKind.Arithmetic, precedence: 12, associativity: Associativity.Left },
  // Shift
  { symbol: "<<", kind: OperatorKind.Bitwise, precedence: 11, associativity: Associativity.Left },
  { symbol: ">>", kind: OperatorKind.Bitwise, precedence: 11, associativity: Associativity.Left },
  // Comparison
  { symbol: "<", kind: OperatorKind.Comparison, precedence: 10, associativity: Associativity.None },
  { symbol: ">", kind: OperatorKind.Comparison, precedence: 10, associativity: Associativity.None },
  { symbol: "<=", kind: OperatorKind.Comparison, precedence: 10, associativity: Associativity.None },
  { symbol: ">=", kind: OperatorKind.Comparison, precedence: 10, associativity: Associativity.None },
  // Equality
  { symbol: "==", kind: OperatorKind.Comparison, precedence: 9, associativity: Associativity.Left },
  { symbol: "!=", kind: OperatorKind.Comparison, precedence: 9, associativity: Associativity.Left },
  // Bitwise AND
  { symbol: "&", kind: OperatorKind.Bitwise, precedence: 8, associativity: Associativity.Left },
  // Bitwise XOR
  { symbol: "^", kind: OperatorKind.Bitwise, precedence: 7, associativity: Associativity.Left },
  // Bitwise OR
  { symbol: "|", kind: OperatorKind.Bitwise, precedence: 6, associativity: Associativity.Left },
  // Logical AND
  { symbol: "&&", kind: OperatorKind.Logical, precedence: 5, associativity: Associativity.Left },
  // Logical OR
  { symbol: "||", kind: OperatorKind.Logical, precedence: 4, associativity: Associativity.Left },
  // Assignment (lowest, right-associative)
  { symbol: "=", kind: OperatorKind.Assignment, precedence: 3, associativity: Associativity.Right },
  { symbol: "+=", kind: OperatorKind.Assignment, precedence: 3, associativity: Associativity.Right },
  { symbol: "-=", kind: OperatorKind.Assignment, precedence: 3, associativity: Associativity.Right },
  { symbol: "*=", kind: OperatorKind.Assignment, precedence: 3, associativity: Associativity.Right },
  { symbol: "/=", kind: OperatorKind.Assignment, precedence: 3, associativity: Associativity.Right },
  { symbol: "%=", kind: OperatorKind.Assignment, precedence: 3, associativity: Associativity.Right },
] as const);

/**
 * Find operator by symbol.
 *
 * @param sym - Operator symbol
 * @returns Operator if found, undefined otherwise
 * @since 0.1.0
 */
export function operatorFromSymbol(sym: string): Operator | undefined {
  return OPERATORS.find((op) => op.symbol === sym);
}

/**
 * Get all operators starting with prefix.
 *
 * @param prefix - Prefix string
 * @returns Matching operators
 * @since 0.1.0
 */
export function operatorsWithPrefix(prefix: string): readonly Operator[] {
  return OPERATORS.filter((op) => op.symbol.startsWith(prefix));
}