/**
 * # Literals
 *
 * `literal` defines literal token types (numbers, strings, booleans).
 *
 * Path: `xiaoyi.domain.token.syntax.literal`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `syntax`
 * - Layer 3: `literal`
 *
 * @module domain.token.syntax.literal
 * @brief Literal token definitions
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.syntax
 * @see domain.token.primitive
 */
import { SyntaxKind } from "..";

/** Literal token kind. */
export enum LiteralKind {
  /** Integer literal. */
  Integer = "integer",
  /** Float literal. */
  Float = "float",
  /** String literal. */
  String = "string",
  /** Boolean literal. */
  Boolean = "boolean",
}

/** Literal token. */
export interface Literal {
  /** Literal kind. */
  readonly kind: LiteralKind;
  /** Raw text. */
  readonly text: string;
  /** Parsed value. */
  readonly value: LiteralValue;
}

/** Literal value union. */
export type LiteralValue = number | string | boolean;

/**
 * Create integer literal.
 *
 * @param text - Raw text
 * @param value - Parsed value
 * @returns Integer literal
 * @since 0.1.0
 */
export function intLiteral(text: string, value: number): Literal {
  return { kind: LiteralKind.Integer, text, value };
}

/**
 * Create float literal.
 *
 * @param text - Raw text
 * @param value - Parsed value
 * @returns Float literal
 * @since 0.1.0
 */
export function floatLiteral(text: string, value: number): Literal {
  return { kind: LiteralKind.Float, text, value };
}

/**
 * Create string literal.
 *
 * @param text - Raw text (with quotes)
 * @param value - Parsed value (without quotes)
 * @returns String literal
 * @since 0.1.0
 */
export function stringLiteral(text: string, value: string): Literal {
  return { kind: LiteralKind.String, text, value };
}

/**
 * Create boolean literal.
 *
 * @param text - Raw text (true/false)
 * @param value - Parsed value
 * @returns Boolean literal
 * @since 0.1.0
 */
export function boolLiteral(text: string, value: boolean): Literal {
  return { kind: LiteralKind.Boolean, text, value };
}

/**
 * Parse literal from text.
 *
 * @param text - Raw literal text
 * @returns Parsed literal or undefined if invalid
 * @since 0.1.0
 */
export function parseLiteral(text: string): Literal | undefined {
  // Empty string
  if (text === "") return undefined;

  // Boolean
  if (text === "true") return boolLiteral(text, true);
  if (text === "false") return boolLiteral(text, false);

  // String (quoted)
  if ((text.startsWith('"') && text.endsWith('"')) ||
      (text.startsWith("'") && text.endsWith("'"))) {
    return stringLiteral(text, text.slice(1, -1));
  }

  // Number - check for scientific notation first
  const hasExponent = /[eE]/.test(text);
  const num = Number(text);
  if (!Number.isNaN(num)) {
    if (hasExponent || !Number.isInteger(num)) {
      return floatLiteral(text, num);
    }
    return intLiteral(text, num);
  }

  return undefined;
}