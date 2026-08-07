/**
 * # Delimiters
 *
 * `delimiter` defines all delimiter tokens (brackets, braces, parens, etc.).
 *
 * Path: `xiaoyi.domain.token.syntax.delimiter`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `syntax`
 * - Layer 3: `delimiter`
 *
 * @module domain.token.syntax.delimiter
 * @brief Delimiter tokens (brackets, braces, parens)
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.syntax
 * @see domain.token.syntax.keyword
 */
import { SyntaxKind } from "..";

/** Delimiter token. */
export interface Delimiter {
  /** Opening character. */
  readonly open: string;
  /** Closing character. */
  readonly close: string;
  /** Delimiter kind. */
  readonly kind: DelimiterKind;
}

/** Delimiter categories. */
export enum DelimiterKind {
  /** Parentheses () */
  Paren = "paren",
  /** Square brackets [] */
  Bracket = "bracket",
  /** Curly braces {} */
  Brace = "brace",
  /** Angle brackets <> */
  Angle = "angle",
}

/** All delimiters. */
export const DELIMITERS: readonly Delimiter[] = Object.freeze([
  { open: "(", close: ")", kind: DelimiterKind.Paren },
  { open: "[", close: "]", kind: DelimiterKind.Bracket },
  { open: "{", close: "}", kind: DelimiterKind.Brace },
  { open: "<", close: ">", kind: DelimiterKind.Angle },
] as const);

/**
 * Get closing delimiter for opening.
 *
 * @param open - Opening character
 * @returns Closing character if valid delimiter
 * @since 0.1.0
 */
export function matchingClose(open: string): string | undefined {
  return DELIMITERS.find((d) => d.open === open)?.close;
}

/**
 * Get opening delimiter for closing.
 *
 * @param close - Closing character
 * @returns Opening character if valid delimiter
 * @since 0.1.0
 */
export function matchingOpen(close: string): string | undefined {
  return DELIMITERS.find((d) => d.close === close)?.open;
}

/**
 * Check if character is an opening delimiter.
 *
 * @param c - Character
 * @returns true if opening delimiter
 * @since 0.1.0
 */
export function isOpenDelimiter(c: string): boolean {
  return DELIMITERS.some((d) => d.open === c);
}

/**
 * Check if character is a closing delimiter.
 *
 * @param c - Character
 * @returns true if closing delimiter
 * @since 0.1.0
 */
export function isCloseDelimiter(c: string): boolean {
  return DELIMITERS.some((d) => d.close === c);
}

/**
 * Check if characters form a valid delimiter pair.
 *
 * @param open - Opening character
 * @param close - Closing character
 * @returns true if valid pair
 * @since 0.1.0
 */
export function isDelimiterPair(open: string, close: string): boolean {
  return matchingClose(open) === close;
}