/**
 * # Short-Term Memory (STM)
 *
 * `stm` provides short-term memory management for agent conversations.
 *
 * Path: `xiaoyi.memory.stm`
 *
 * - Layer 0: `memory`
 * - Layer 1: `stm` — short-term memory.
 * - Layer 2: `cache` — message/state caching.
 * - Layer 3: `context` — conversation context management.
 * - Layer 4: `window` — sliding window for history.
 *
 * @module memory.stm
 * @brief Short-term memory for agent conversations
 * @group Memory
 * @since 0.1.0
 * @author Miruamel
 * @see memory
 * @see memory.stm.cache
 * @see memory.stm.context
 */
export * from "./cache";
export * from "./context";
export * from "./window";

/**
 * STM entry representing a conversation message.
 *
 * @brief Message entry in short-term memory
 * @group Memory
 * @since 0.1.0
 */
export interface StmEntry {
  /** Unique entry ID. */
  id: string;
  /** Role (user, assistant, system, tool). */
  role: "user" | "assistant" | "system" | "tool";
  /** Message content. */
  content: string;
  /** Optional metadata. */
  metadata?: Record<string, unknown>;
  /** Timestamp. */
  timestamp: number;
}

/**
 * Short-term memory configuration.
 *
 * @brief Configuration for STM behavior
 * @group Memory
 * @since 0.1.0
 */
export interface StmConfig {
  /** Maximum entries to retain. */
  maxEntries: number;
  /** Maximum total characters. */
  maxChars: number;
  /** Time-to-live in ms. */
  ttl?: number;
}

/**
 * Short-term memory store.
 *
 * @brief In-memory conversation history
 * @group Memory
 * @since 0.1.0
 * @threadsafe
 * @example
 * ```typescript
 * const stm = new StmStore({ maxEntries: 100, maxChars: 50000 });
 * stm.add({ role: "user", content: "Hello" });
 * const history = stm.getRecent(10);
 * ```
 */
export class StmStore {
  private entries: StmEntry[] = [];
  private readonly config: Required<StmConfig>;

  /**
   * Create STM store.
   *
   * @param config - STM configuration
   * @since 0.1.0
   */
  constructor(config: StmConfig) {
    this.config = {
      maxEntries: config.maxEntries,
      maxChars: config.maxChars,
      ttl: config.ttl ?? 3600000, // 1 hour default
    };
  }

  /**
   * Add entry to memory.
   *
   * @param entry - Entry to add (without id/timestamp)
   * @returns Added entry with generated id and timestamp
   * @since 0.1.0
   */
  add(entry: Omit<StmEntry, "id" | "timestamp">): StmEntry {
    const newEntry: StmEntry = {
      ...entry,
      id: crypto.randomUUID(),
      timestamp: Date.now(),
    };

    this.entries.push(newEntry);
    this.prune();
    return newEntry;
  }

  /**
   * Get recent entries.
   *
   * @param count - Number of recent entries
   * @returns Recent entries (oldest first)
   * @since 0.1.0
   */
  getRecent(count: number): StmEntry[] {
    return this.entries.slice(-count);
  }

  /**
   * Get all entries.
   *
   * @returns All entries (oldest first)
   * @since 0.1.0
   */
  getAll(): StmEntry[] {
    return [...this.entries];
  }

  /**
   * Get entries within time range.
   *
   * @param since - Start timestamp
   * @param until - End timestamp
   * @returns Matching entries
   * @since 0.1.0
   */
  getRange(since: number, until: number = Date.now()): StmEntry[] {
    return this.entries.filter((e) => e.timestamp >= since && e.timestamp <= until);
  }

  /**
   * Clear all entries.
   *
   * @since 0.1.0
   */
  clear(): void {
    this.entries = [];
  }

  /**
   * Get current entry count.
   *
   * @returns Number of entries
   * @since 0.1.0
   */
  size(): number {
    return this.entries.length;
  }

  /**
   * Get total character count.
   *
   * @returns Total characters
   * @since 0.1.0
   */
  charCount(): number {
    return this.entries.reduce((sum, e) => sum + e.content.length, 0);
  }

  /**
   * Prune old entries based on config.
   *
   * @since 0.1.0
   */
  private prune(): void {
    const now = Date.now();

    // Remove expired entries
    if (this.config.ttl > 0) {
      const cutoff = now - this.config.ttl;
      this.entries = this.entries.filter((e) => e.timestamp > cutoff);
    }

    // Remove oldest entries if over maxEntries
    while (this.entries.length > this.config.maxEntries) {
      this.entries.shift();
    }

    // Remove oldest entries if over maxChars
    while (this.charCount() > this.config.maxChars && this.entries.length > 0) {
      this.entries.shift();
    }
  }
}

/**
 * Default STM configuration.
 *
 * @returns Default STM config
 * @since 0.1.0
 * @group Memory
 */
export function defaultStmConfig(): StmConfig {
  return {
    maxEntries: 100,
    maxChars: 50000,
    ttl: 3600000,
  };
}