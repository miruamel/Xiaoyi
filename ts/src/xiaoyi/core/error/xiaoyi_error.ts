/**
 * Layer 0 - Foundation / Core Error
 *
 * Structured error value with kind, message, and metadata map.
 * TypeScript consumers can map kinds to recoverable / fatal decisions.
 *
 * {@see ErrorKind}
 */

export interface XiaoyiErrorInput {
  kind: string;
  message: string;
  meta?: Record<string, string>;
}

export class XiaoyiError extends Error {
  readonly kind: string;

  readonly meta: Record<string, string>;

  constructor(input: XiaoyiErrorInput) {
    super(`[${input.kind}] ${input.message}`);
    this.kind = input.kind;
    this.meta = input.meta ?? {};
    Object.setPrototypeOf(this, XiaoyiError.prototype);
  }
}
