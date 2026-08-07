/**
 * # DAG Executor
 *
 * `executor` provides execution strategies for workflow DAGs.
 *
 * Path: `xiaoyi.workflow.dag.executor`
 *
 * @module workflow.dag.executor
 * @brief DAG execution strategies
 * @group Workflow
 * @since 0.1.0
 * @author Miruamel
 * @see workflow.dag.graph
 * @see workflow.dag.node
 */
import { Dag, DagNode } from "./graph";

/**
 * Execution mode for DAG.
 *
 * @brief How to execute DAG nodes
 * @group Workflow
 * @since 0.1.0
 */
export enum ExecutionMode {
  /** Sequential execution (default). */
  Sequential = "sequential",
  /** Parallel execution where possible. */
  Parallel = "parallel",
  /** Parallel with limited concurrency. */
  ParallelLimited = "parallel-limited",
}

/**
 * Executor options.
 *
 * @brief Configuration for DAG executor
 * @group Workflow
 * @since 0.1.0
 */
export interface ExecutorOptions {
  /** Execution mode. */
  mode?: ExecutionMode;
  /** Max concurrent nodes (for ParallelLimited). */
  maxConcurrency?: number;
  /** Timeout per node in ms. */
  nodeTimeout?: number;
  /** Global timeout in ms. */
  timeout?: number;
  /** Continue on node error. */
  continueOnError?: boolean;
}

/**
 * Node execution result.
 *
 * @brief Result of executing a node
 * @group Workflow
 * @since 0.1.0
 */
export interface NodeResult {
  /** Node ID. */
  nodeId: string;
  /** Success flag. */
  success: boolean;
  /** Output value (if success). */
  value?: unknown;
  /** Error (if failed). */
  error?: Error;
  /** Execution time in ms. */
  duration: number;
}

/**
 * DAG execution result.
 *
 * @brief Complete DAG execution result
 * @group Workflow
 * @since 0.1.0
 */
export interface DagExecutionResult {
  /** Overall success. */
  success: boolean;
  /** Node results. */
  nodeResults: NodeResult[];
  /** Final output. */
  output?: unknown;
  /** Total execution time in ms. */
  totalDuration: number;
}

/**
 * Execute DAG with configurable strategy.
 *
 * @param dag - DAG to execute
 * @param input - Initial input
 * @param options - Execution options
 * @returns Execution result
 * @since 0.1.0
 * @group Workflow
 * @example
 * ```typescript
 * const result = await executeDag(dag, input, { mode: ExecutionMode.Parallel, maxConcurrency: 4 });
 * ```
 */
export async function executeDag(
  dag: Dag,
  input: unknown,
  options: ExecutorOptions = {}
): Promise<DagExecutionResult> {
  const {
    mode = ExecutionMode.Sequential,
    maxConcurrency = 4,
    nodeTimeout = 30000,
    timeout = 300000,
    continueOnError = false,
  } = options;

  const startTime = Date.now();
  const nodeResults: NodeResult[] = [];
  const results = new Map<string, unknown>();
  const errors = new Map<string, Error>();

  const order = dag.topologicalOrder();

  const executeNode = async (nodeId: string): Promise<unknown> => {
    const nodeStart = Date.now();
    const node = dag.nodes.get(nodeId);
    if (!node) throw new Error(`Node not found: ${nodeId}`);

    // Get inputs from upstream
    const inputs: Record<string, unknown> = {};
    for (const [from, tos] of dag.edges) {
      if (tos.includes(nodeId)) {
        inputs[from] = results.get(from);
      }
    }

    try {
      const promise = node.execute(inputs);
      const value = nodeTimeout > 0
        ? await Promise.race([
            promise,
            new Promise<never>((_, reject) =>
              setTimeout(() => reject(new Error("Node timeout")), nodeTimeout)
            ),
          ])
        : await promise;

      results.set(nodeId, value);
      nodeResults.push({
        nodeId,
        success: true,
        value,
        duration: Date.now() - nodeStart,
      });
      return value;
    } catch (error) {
      const err = error instanceof Error ? error : new Error(String(error));
      errors.set(nodeId, err);
      nodeResults.push({
        nodeId,
        success: false,
        error: err,
        duration: Date.now() - nodeStart,
      });

      if (!continueOnError) {
        throw err;
      }
      return undefined;
    }
  };

  if (mode === ExecutionMode.Parallel) {
    // Execute all nodes in parallel (respecting dependencies)
    await Promise.all(order.map(executeNode));
  } else if (mode === ExecutionMode.ParallelLimited) {
    // Execute with concurrency limit
    const queue = [...order];
    const running = new Set<Promise<void>>();

    while (queue.length > 0 || running.size > 0) {
      while (queue.length > 0 && running.size < maxConcurrency) {
        const nodeId = queue.shift()!;
        const promise = executeNode(nodeId).then(() => {});
        running.add(promise);
        promise.finally(() => running.delete(promise));
      }
      if (running.size > 0) {
        await Promise.race(running);
      }
    }
  } else {
    // Sequential
    for (const nodeId of order) {
      await executeNode(nodeId);
    }
  }

  const totalDuration = Date.now() - startTime;
  const success = errors.size === 0;
  const output = success ? results.get(order[order.length - 1]) : undefined;

  return {
    success,
    nodeResults,
    output,
    totalDuration,
  };
}

/**
 * Create default executor options.
 *
 * @returns Default executor options
 * @since 0.1.0
 * @group Workflow
 */
export function defaultExecutorOptions(): ExecutorOptions {
  return {
    mode: ExecutionMode.Sequential,
    maxConcurrency: 4,
    nodeTimeout: 30000,
    timeout: 300000,
    continueOnError: false,
  };
}