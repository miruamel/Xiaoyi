/**
 * # Xiaoyi TypeScript Core
 *
 * `xiaoyi` provides the TypeScript implementation of the Xiaoyi AI Agent Framework.
 *
 * Path: `xiaoyi`
 *
 * - Layer 0: `domain` — domain primitives.
 * - Layer 1: `core` — configuration, error handling, result types.
 * - Layer 2: `llm` — LLM client abstraction.
 * - Layer 3: `workflow` — DAG-based workflow execution.
 * - Layer 4: `memory` — Short-term memory (STM) with LRU cache.
 * - Layer 5: `builder`/`orchestrator`/`gateway`/`lexer` — composition layer.
 *
 * @module xiaoyi
 * @brief Xiaoyi AI Agent Framework - TypeScript
 * @group Xiaoyi
 * @since 0.1.0
 * @author Miruamel
 * @see https://github.com/miruamel/Xiaoyi
 */
export * from "./domain";
export * from "./core";
export * from "./workflow";
export * from "./memory";