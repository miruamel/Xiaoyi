/**
 * # DAG Module
 *
 * `dag` provides DAG builder, nodes, and executor for workflow execution.
 *
 * Path: `xiaoyi.workflow.dag`
 *
 * - Layer 0: `workflow`
 * - Layer 1: `dag` — DAG builder, nodes, and executor.
 * - Layer 2: `graph`/`node`/`executor` — DAG components.
 *
 * @module workflow.dag
 * @brief DAG-based workflow execution
 * @group Workflow
 * @since 0.1.0
 * @author Miruamel
 * @see workflow.dag.graph
 * @see workflow.dag.node
 * @see workflow.dag.executor
 */
export * from "./graph";
export * from "./node";
export * from "./executor";

/**
 * DAG builder.
 *
 * @brief Build directed acyclic graphs
 * @group Workflow
 * @since 0.1.0
 */
export { DagBuilder } from "./graph";

/**
 * DAG.
 *
 * @brief Directed acyclic graph
 * @group Workflow
 * @since 0.1.0
 */
export { Dag } from "./graph";

/**
 * DAG node.
 *
 * @brief Node in a DAG
 * @group Workflow
 * DAG node.
…
 * @since 0.1.0
 * @group Workflow
 */
export type { DagNode } from "./graph";

/**
 * Node context.
…
 * @since 0.1.0
 * @group Workflow
 */
export type { NodeContext } from "./node";
/**
 * Create a node.
 *
 * @param fn - Node function
 * @returns Node creator
 * @since 0.1.0
 * @group Workflow
 */
export { createNode } from "./node";

/**
 * Pass-through node.
 *
 * @returns Pass-through node
 * @since 0.1.0
 * @group Workflow
 */
export { passThroughNode } from "./node";

/**
 * Transform node.
 *
 * @param fn - Transform function
 * @returns Transform node
 * @since 0.1.0
 * @group Workflow
 */
export { transformNode } from "./node";

/**
 * Merge node.
 *
 * @param fn - Merge function
 * @returns Merge node
 * @since 0.1.0
 * @group Workflow
 */
export { mergeNode } from "./node";

/**
 * Execution mode.
 *
 * @brief How nodes execute
 * @group Workflow
 * @since 0.1.0
 */
export type { ExecutionMode } from "./executor";

/**
 * Executor options.
 *
 * @brief Configuration for workflow execution
 * @group Workflow
 * @since 0.1.0
 */
export type { ExecutorOptions } from "./executor";

/**
 * Node result.
 *
 * @brief Result of executing a single node
 * @group Workflow
 * @since 0.1.0
 */
export type { NodeResult } from "./executor";

/**
 * DAG execution result.
 *
 * @brief Result of executing entire DAG
 * @group Workflow
 * @since 0.1.0
 */
export type { DagExecutionResult } from "./executor";

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
export { executeDag } from "./executor";