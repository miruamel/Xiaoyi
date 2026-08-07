/**
 * # STM Context
 *
 * `context` provides conversation context management for STM.
 *
 * Path: `xiaoyi.memory.stm.context`
 *
 * @module memory.stm.context
 * @brief Conversation context management
 * @group Memory
 * @since 0.1.0
 * @author Miruamel
 * @see memory.stm
 * @see memory.stm.cache
 * @see memory.stm.window
 */
import { StmEntry, StmStore, StmConfig } from "..";

/**
 * Context window configuration.
 *
 * @brief Configuration for context window
 * @group Memory
 * @since 0.1.0
 */
export interface ContextWindowConfig {
  /** Maximum tokens in context. */
  maxTokens: number;
  /** Reserve tokens for response. */
  reserveTokens: number;
  /** System prompt tokens. */
  systemTokens?: number;
}

/**
 * Conversation context builder.
 *
 * @brief Build optimized context for LLM
 * @group Memory
 * @since 0.1.0
 * @threadsafe
 * @example
 * ```typescript
 * const context = new ContextBuilder(stmStore, { maxTokens: 4096 });
 * const messages = context.build();
 * ```
 */
export class ContextBuilder {
  private readonly store: StmStore;
  private readonly config: ContextWindowConfig;
  private readonly tokenEstimator: (text: string) => number;

  /**
   * Create context builder.
   *
   * @param store - STM store
   * @param config - Context window config
   * @param tokenEstimator - Optional token estimation function
   * @since 0.1.0
   */
  constructor(
    store: StmStore,
    config: ContextWindowConfig,
    tokenEstimator?: (text: string) => number
  ) {
    this.store = store;
    this.config = config;
    this.tokenEstimator = tokenEstimator ?? this.defaultTokenEstimator;
  }

  /**
   * Default token estimator (~4 chars per token).
   *
   * @param text - Text to estimate
   * @returns Estimated tokens
   * @since 0.1.0
   */
  private defaultTokenEstimator(text: string): number {
    return Math.ceil(text.length / 4);
  }

  /**
   * Build context messages for LLM.
   *
   * @param systemPrompt - Optional system prompt
   * @returns Messages array for LLM
   * @since 0.1.0
   */
  build(systemPrompt?: string): Array<{ role: string; content: string }> {
    const messages: Array<{ role: string; content: string }> = [];
    let tokenCount = 0;

    // Add system prompt if provided
    if (systemPrompt) {
      const sysTokens = this.tokenEstimator(systemPrompt);
      if (sysTokens <= this.config.maxTokens - this.config.reserveTokens) {
        messages.push({ role: "system", content: systemPrompt });
        tokenCount += sysTokens;
      }
    }

    // Get recent entries and add from newest to oldest
    const entries = this.store.getAll().reverse();
    const availableTokens = this.config.maxTokens - this.config.reserveTokens - tokenCount;

    for (const entry of entries) {
      const entryTokens = this.tokenEstimator(entry.content);
      if (tokenCount + entryTokens > availableTokens) break;

      messages.unshift({
        role: entry.role,
        content: entry.content,
      });
      tokenCount += entryTokens;
    }

    return messages;
  }

  /**
   * Get estimated token count for current context.
   *
   * @param systemPrompt - Optional system prompt
   * @returns Estimated tokens
   * @since 0.1.0
   */
  estimateTokens(systemPrompt?: string): number {
    let tokens = systemPrompt ? this.tokenEstimator(systemPrompt) : 0;
    for (const entry of this.store.getAll()) {
      tokens += this.tokenEstimator(entry.content);
    }
    return tokens;
  }
}

/**
 * Default context window configuration.
 *
 * @returns Default context config
 * @since 0.1.0
 * @group Memory
 */
export function defaultContextConfig(): ContextWindowConfig {
  return {
    maxTokens: 4096,
    reserveTokens: 512,
    systemTokens: 256,
  };
}