/**
 * # Memory Module
 *
 * `memory` provides short-term memory (STM) with LRU cache and sliding window.
 *
 * Path: `xiaoyi.memory`
 *
 * - Layer 0: `memory`
 * - Layer 1: `stm` — Short-term memory with LRU cache and sliding window.
 *
 * @module memory
 * @brief Short-term memory with LRU cache
 * @group Memory
 * @since 0.1.0
 * @author Miruamel
 * @see memory.stm
 * @see memory.stm.cache
 * @see memory.stm.context
 * @see memory.stm.window
 */
export * from "./stm";

/**
 * STM entry.
 *
 * @brief Single entry in short-term memory
 * @group Memory
 * @since 0.1.0
 */
export type { StmEntry } from "./stm";

/**
 * STM configuration.
 *
 * @brief Configuration for short-term memory
 * @group Memory
 * @since 0.1.0
 */
export type { StmConfig } from "./stm";

/**
 * STM store interface.
 *
 * @brief Interface for STM implementations
 * @group Memory
 * @since 0.1.0
 */
export type { StmStore } from "./stm";

/**
 * LRU cache.
 *
 * @brief Least recently used cache
 * @group Memory
 * @since 0.1.0
 */
export { LruCache } from "./stm/cache";

/**
 * Context builder.
 *
 * @brief Build conversation context from STM
 * @group Memory
 * @since 0.1.0
 */
export { ContextBuilder } from "./stm/context";

/**
 * Sliding window.
 *
 * @brief Sliding window over STM entries
 * @group Memory
 * @since 0.1.0
 */
export { SlidingWindow } from "./stm/window";