/**
 * # STM Window
 *
 * `window` provides sliding window management for STM history.
 *
 * Path: `xiaoyi.memory.stm.window`
 *
 * @module memory.stm.window
 * @brief Sliding window for conversation history
 * @group Memory
 * @since 0.1.0
 * @author Miruamel
 * @see memory.stm
 * @see memory.stm.context
 * @see memory.stm.cache
 */
import { StmEntry } from "..";

/**
 * Window configuration.
 *
 * @brief Sliding window configuration
 * @group Memory
 * @since 0.1.0
 */
export interface WindowConfig {
  /** Window size (number of entries). */
  size: number;
  /** Step size for sliding. */
  step?: number;
}

/**
 * Sliding window over STM entries.
 *
 * @brief Sliding window for conversation history
 * @group Memory
 * @since 0.1.0
 * @threadsafe
 * @example
 * ```typescript
 * const window = new SlidingWindow(entries, { size: 10, step: 5 });
 * for (const chunk of window) {
 *   // Process chunk of 10 entries
 * }
 * ```
 */
export class SlidingWindow {
  private readonly entries: StmEntry[];
  private readonly config: Required<WindowConfig>;
  private position = 0;

  /**
   * Create sliding window.
   *
   * @param entries - STM entries
   * @param config - Window config
   * @since 0.1.0
   */
  constructor(entries: StmEntry[], config: WindowConfig) {
    this.entries = entries;
    this.config = {
      size: config.size,
      step: config.step ?? config.size,
    };
  }

  /**
   * Get current window.
   *
   * @returns Current window entries
   * @since 0.1.0
   */
  current(): StmEntry[] {
    const end = this.position + this.config.size;
    return this.entries.slice(this.position, end);
  }

  /**
   * Move window forward.
   *
   * @returns true if moved, false if at end
   * @since 0.1.0
   */
  next(): boolean {
    if (this.position + this.config.step >= this.entries.length) {
      return false;
    }
    this.position += this.config.step;
    return true;
  }

  /**
   * Move window backward.
   *
   * @returns true if moved, false if at start
   * @since 0.1.0
   */
  prev(): boolean {
    if (this.position === 0) return false;
    this.position = Math.max(0, this.position - this.config.step);
    return true;
  }

  /**
   * Reset to start.
   *
   * @since 0.1.0
   */
  reset(): void {
    this.position = 0;
  }

  /**
   * Check if at end.
   *
   * @returns true if at end
   * @since 0.1.0
   */
  isAtEnd(): boolean {
    return this.position + this.config.size >= this.entries.length;
  }

  /**
   * Get window position.
   *
   * @returns Current position
   * @since 0.1.0
   */
  getPosition(): number {
    return this.position;
  }

  /**
   * Get total windows.
   *
   * @returns Number of windows
   * @since 0.1.0
   */
  totalWindows(): number {
    if (this.entries.length <= this.config.size) return 1;
    return Math.ceil((this.entries.length - this.config.size) / this.config.step) + 1;
  }

  /**
   * Iterate all windows.
   *
   * @returns Iterator of windows
   * @since 0.1.0
   */
  *[Symbol.iterator](): Iterator<StmEntry[]> {
    this.reset();
    while (true) {
      yield this.current();
      if (!this.next()) break;
    }
  }
}

/**
 * Create sliding window from STM store.
 *
 * @param store - STM store
 * @param config - Window config
 * @returns Sliding window
 * @since 0.1.0
 * @group Memory
 */
export function createWindow(
  store: { getAll(): StmEntry[] },
  config: WindowConfig
): SlidingWindow {
  return new SlidingWindow(store.getAll(), config);
}