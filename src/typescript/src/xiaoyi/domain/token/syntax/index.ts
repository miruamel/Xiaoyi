/**
 * # Syntax Tokens
 *
 * `syntax` provides syntax-level tokens (keywords, operators, delimiters)
 * for the Xiaoyi language parser.
 *
 * Path: `xiaoyi.domain.token.syntax`
 *
 * - Layer 0: `domain`
 * - Layer 1: `token`
 * - Layer 2: `syntax` — syntax token definitions.
 * - Layer 3: `keyword`/`operator`/`delimiter`/`literal` — token categories.
 *
 * @module domain.token.syntax
 * @brief Syntax-level tokens for parsing
 * @group Domain
 * @since 0.1.0
 * @author Miruamel
 * @see domain.token
 * @see domain.token.primitive
 * @see lexer
 */
export * from "./keyword";
export * from "./operator";
export * from "./delimiter";
export * from "./literal";

/**
 * Syntax token kind.
 *
 * @brief Classification of syntax tokens
 * @group Domain
 * @since 0.1.0
 */
export enum SyntaxKind {
  /** Keyword (if, else, while, etc.) */
  Keyword = "keyword",
  /** Operator (+, -, *, /, etc.) */
  Operator = "operator",
  /** Delimiter ((), {}, [], etc.) */
  Delimiter = "delimiter",
  /** Literal (number, string, bool) */
  Literal = "literal",
  /** Identifier. */
  Identifier = "identifier",
  /** End of input. */
  Eof = "eof",
}