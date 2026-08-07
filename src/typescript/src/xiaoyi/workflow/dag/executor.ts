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
  /** Max concurrent nodes (for Parallel and ParallelLimited). */
  maxConcurrency?: number;
  /** Timeout per node in ms. */
  nodeTimeout?: number;
  /** Global timeout in ms. */
  timeout?: number;
  /** Continue on node error. */
  continueOnError?: boolean;
  /** Callback when node starts. */
  onNodeStart?: (nodeId: string) => void;
  /** Callback when node completes. */
  onNodeComplete?: (nodeId: string, result: NodeResult) => void;
  /** Callback when node errors. */
  onNodeError?: (nodeId: string, error: Error) => void;
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
  /** Final output - single value if one leaf node, object with all leaf outputs if multiple. */
  output?: unknown;
  /** Error if execution failed. */
  error?: Error;
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
    timeout,
    continueOnError = false,
    onNodeStart,
    onNodeComplete,
    onNodeError,
  } = options;

  const startTime = Date.now();
  const nodeResults: NodeResult[] = [];
  const results = new Map<string, unknown>();
  const errors = new Map<string, Error>();

  const order = dag.topologicalOrder();

  // Build reverse edges map (nodeId -> array of upstream nodeIds)
  const reverseEdges = new Map<string, string[]>();
  // Build forward edges map (nodeId -> array of downstream nodeIds)
  const forwardEdges = new Map<string, string[]>();
  for (const [from, tos] of dag.edges) {
    for (const to of tos) {
      const existing = reverseEdges.get(to) ?? [];
      existing.push(from);
      reverseEdges.set(to, existing);

      const existingForward = forwardEdges.get(from) ?? [];
      existingForward.push(to);
      forwardEdges.set(from, existingForward);
    }
  }

  // Find leaf nodes (nodes with no outgoing edges)
  const leafNodes = order.filter((nodeId) => !forwardEdges.has(nodeId) || forwardEdges.get(nodeId)!.length === 0);

  // Track which nodes have completed
  const completed = new Set<string>();
  let hasError = false;
  let firstError: Error | undefined;

  // Create an AbortController for global timeout
  const abortController = new AbortController();
  const { signal } = abortController;

  // Set global timeout if specified
  let globalTimeoutHandle: NodeJS.Timeout | undefined;
  if (timeout && timeout > 0) {
    globalTimeoutHandle = setTimeout(() => {
      abortController.abort(new Error("Global timeout"));
    }, timeout);
  }

  const executeNode = async (nodeId: string): Promise<unknown> => {
    const nodeStart = Date.now();
    const node = dag.nodes.get(nodeId);
    if (!node) throw new Error(`Node not found: ${nodeId}`);

    onNodeStart?.(nodeId);

    // Check if aborted before executing
    if (signal.aborted) {
      throw new Error("Execution aborted");
    }

    // Get inputs from upstream nodes
    const upstreamNodes = reverseEdges.get(nodeId) ?? [];
    let nodeInput: unknown = input;

    if (upstreamNodes.length === 1) {
      // Single upstream - pass its output directly
      nodeInput = results.get(upstreamNodes[0]);
    } else if (upstreamNodes.length > 1) {
      // Multiple upstreams - pass object with all upstream outputs
      const inputs: Record<string, unknown> = {};
      for (const from of upstreamNodes) {
        inputs[from] = results.get(from);
      }
      nodeInput = inputs;
    }
    // No upstreams - use initial input

    try {
      const promise = node.execute(nodeInput);
      const value = nodeTimeout > 0
        ? await Promise.race([
            promise,
            new Promise<never>((_, reject) =>
              setTimeout(() => reject(new Error("Node timeout")), nodeTimeout)
            ),
          ])
        : await promise;

      results.set(nodeId, value);
      completed.add(nodeId);
      const nodeResult: NodeResult = {
        nodeId,
        success: true,
        value,
        duration: Date.now() - nodeStart,
      };
      nodeResults.push(nodeResult);
      onNodeComplete?.(nodeId, nodeResult);
      return value;
    } catch (error) {
      const err = error instanceof Error ? error : new Error(String(error));
      errors.set(nodeId, err);
      completed.add(nodeId);
      const nodeResult: NodeResult = {
        nodeId,
        success: false,
        error: err,
        duration: Date.now() - nodeStart,
      };
      nodeResults.push(nodeResult);
      onNodeError?.(nodeId, err);

      if (!continueOnError) {
        hasError = true;
        if (!firstError) firstError = err;
        // Abort all remaining nodes
        abortController.abort(err);
      }
      return undefined;
    }
  };

  try {
    if (mode === ExecutionMode.Sequential) {
      // Sequential - execute one at a time in topological order
      for (const nodeId of order) {
        if (signal.aborted && !continueOnError) break;
        await executeNode(nodeId);
      }
    } else {
      // Parallel modes - use a work queue with dependency tracking
      const pending = new Set(order);
      const running = new Set<string>();
      let errorOccurred = false;

      while (pending.size > 0 || running.size > 0) {
        // Check for ready nodes
        const readyNodes: string[] = [];
        for (const nodeId of pending) {
          const upstreamNodes = reverseEdges.get(nodeId) ?? [];
          if (upstreamNodes.every((up) => completed.has(up))) {
            readyNodes.push(nodeId);
          }
        }

        // Execute ready nodes up to maxConcurrency
        for (const nodeId of readyNodes) {
          if (running.size >= maxConcurrency) break;
          if (errorOccurred && !continueOnError) break;
          if (signal.aborted && !continueOnError) break;
          pending.delete(nodeId);
          running.add(nodeId);
          executeNode(nodeId)
            .then(() => {
              running.delete(nodeId);
            })
            .catch(() => {
              running.delete(nodeId);
              if (!continueOnError) errorOccurred = true;
            });
        }

        if (running.size > 0) {
          // Wait for at least one to complete
          await new Promise<void>((resolve) => {
            const check = () => {
              if (running.size === 0 || signal.aborted) {
                resolve();
              } else {
                setTimeout(check, 10);
              }
            };
            check();
          });
        } else if (pending.size > 0) {
          // No ready nodes but pending exists
          if (signal.aborted) break;
          // Check if we're waiting for an error to propagate
          if (errorOccurred && !continueOnError) break;
          // Otherwise it's a deadlock - break with warning
          console.warn("Warning: No ready nodes but pending nodes exist");
          break;
        }
      }
    }

    const totalDuration = Date.now() - startTime;
    const success = !hasError && !signal.aborted;

    // Build output: single leaf node output or object with all leaf node outputs
    let output: unknown;
    if (success) {
      if (leafNodes.length === 1) {
        output = results.get(leafNodes[0]);
      } else if (leafNodes.length > 1) {
        const leafOutputs: unknown[] = [];
        for (const leaf of leafNodes) {
          leafOutputs.push(results.get(leaf));
        }
        output = leafOutputs;
      }
    }

    const error = hasError ? firstError : (signal.aborted ? (signal.reason instanceof Error ? signal.reason : new Error("Execution aborted")) : undefined);

    return {
      success,
      nodeResults,
      output,
      error,
      totalDuration,
    };
  } finally {
    if (globalTimeoutHandle) {
      clearTimeout(globalTimeoutHandle);
    }
  }
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
    continueOnError: false,
  };
}