/**
 * # Keywords
 *
 * `keyword` defines all reserved keywords in the Xiaoyi language.
 *
 * Path: `xiaoyi.domain.token.syntax.keyword`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `syntax`
 * - Layer 3: `keyword`
 *
 * @module domain.token.syntax.keyword
 * @brief Language reserved keywords
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token.syntax
 * @see domain.token.syntax.operator
 */
import { SyntaxKind } from "..";

/** Keyword token. */
export interface Keyword {
  /** Keyword text. */
  readonly text: string;
  /** Keyword kind. */
  readonly kind: KeywordKind;
}

/** Keyword categories. */
export enum KeywordKind {
  /** Control flow (if, else, while, for, return) */
  ControlFlow = "control_flow",
  /** Declaration (let, const, fn, struct, enum) */
  Declaration = "declaration",
  /** Type (int, float, bool, string) */
  Type = "type",
  /** Module (mod, use, pub) */
  Module = "module",
  /** Async (async, await, spawn) */
  Async = "async",
  /** Error handling (try, catch, throw) */
  ErrorHandling = "error_handling",
}

/** All keywords. */
export const KEYWORDS: readonly Keyword[] = Object.freeze([
  { text: "if", kind: KeywordKind.ControlFlow },
  { text: "else", kind: KeywordKind.ControlFlow },
  { text: "while", kind: KeywordKind.ControlFlow },
  { text: "for", kind: KeywordKind.ControlFlow },
  { text: "return", kind: KeywordKind.ControlFlow },
  { text: "break", kind: KeywordKind.ControlFlow },
  { text: "continue", kind: KeywordKind.ControlFlow },
  { text: "let", kind: KeywordKind.Declaration },
  { text: "const", kind: KeywordKind.Declaration },
  { text: "fn", kind: KeywordKind.Declaration },
  { text: "struct", kind: KeywordKind.Declaration },
  { text: "enum", kind: KeywordKind.Declaration },
  { text: "int", kind: KeywordKind.Type },
  { text: "float", kind: KeywordKind.Type },
  { text: "bool", kind: KeywordKind.Type },
  { text: "string", kind: KeywordKind.Type },
  { text: "mod", kind: KeywordKind.Module },
  { text: "use", kind: KeywordKind.Module },
  { text: "pub", kind: KeywordKind.Module },
  { text: "async", kind: KeywordKind.Async },
  { text: "await", kind: KeywordKind.Async },
  { text: "spawn", kind: KeywordKind.Async },
  { text: "try", kind: KeywordKind.ErrorHandling },
  { text: "catch", kind: KeywordKind.ErrorHandling },
  { text: "throw", kind: KeywordKind.ErrorHandling },
] as const);

/**
 * Check if identifier is a keyword.
 *
 * @param ident - Identifier string
 * @returns Keyword if keyword, undefined otherwise
 * @since 0.1.0
 */
export function keywordFromIdent(ident: string): Keyword | undefined {
  return KEYWORDS.find((k) => k.text === ident);
}

/**
 * Check if string is a keyword.
 *
 * @param s - String to check
 * @returns true if keyword
 * @since 0.1.0
 */
export function isKeyword(s: string): boolean {
  return keywordFromIdent(s) !== undefined;
}