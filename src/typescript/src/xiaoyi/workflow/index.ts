/**
 * # Workflow Module
 *
 * `workflow` provides DAG-based workflow execution.
 *
 * Path: `xiaoyi.workflow`
 *
 * - Layer 0: `workflow`
 * - Layer 1: `dag` — DAG builder, nodes, and executor.
 *
 * @module workflow
 * @brief DAG-based workflow execution
 * @group Workflow
 * @since 0.1.0
 * @author Miruamel
 * @see workflow.dag
 * @see workflow.dag.graph
 * @see workflow.dag.node
 * @see workflow.dag.executor
 */
export * from "./dag";

/**
 * Execution mode.
 *
 * @brief How nodes execute
 * @group Workflow
 * @since 0.1.0
 */
export type { ExecutionMode } from "./dag/executor";

/**
 * Executor options.
 *
 * @brief Configuration for workflow execution
 * @group Workflow
 * @since 0.1.0
 */
export type { ExecutorOptions } from "./dag/executor";

/**
 * Node result.
 *
 * @brief Result of executing a single node
 * @group Workflow
 * @since 0.1.0
 */
export type { NodeResult } from "./dag/executor";

/**
 * DAG execution result.
 *
 * @brief Result of executing entire DAG
 * @group Workflow
 * @since 0.1.0
 */
export type { DagExecutionResult } from "./dag/executor";

/**
 * Execute a DAG.
 *
 * @param dag - DAG to execute
 * @param options - Execution options
 * @returns Execution result
 * @since 0.1.0
 * @group Workflow
 * @example
 * ```typescript
 * const result = await executeDag(dag, { mode: "parallel" });
 * ```
 */
export { executeDag } from "./dag/executor";