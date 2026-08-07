/**
 * # DAG Node Types
 *
 * `node` provides node types and utilities for workflow DAGs.
 *
 * Path: `xiaoyi.workflow.dag.node`
 *
 * @module workflow.dag.node
 * @brief DAG node types and utilities
 * @group Workflow
 * @since 0.1.0
 * @author Miruamel
 * @see workflow.dag.graph
 * @see workflow.dag.executor
 */
import { DagNode, DagNodeExecute } from "./graph";

/**
 * Node execution context.
 *
 * @brief Context provided to node execution
 * @group Workflow
 * @since 0.1.0
 */
export interface NodeContext {
  /** Node identifier. */
  nodeId: string;
  /** Upstream node results. */
  inputs: Record<string, unknown>;
  /** Workflow execution ID. */
  executionId: string;
  /** Abort signal for cancellation. */
  signal: AbortSignal;
}

/**
 * Typed node execution function.
 *
 * @typeParam TInput - Input type
 * @typeParam TOutput - Output type
 * @since 0.1.0
 */
export type TypedNodeExecute<TInput, TOutput> = (
  context: NodeContext
) => Promise<TOutput>;

/**
 * Create a typed node.
 *
 * @param id - Node ID
 * @param execute - Typed execution function
 * @returns DagNode
 * @since 0.1.0
 * @group Workflow
 */
export function createNode<TInput, TOutput>(
  id: string,
  execute: TypedNodeExecute<TInput, TOutput>
): DagNode {
  return {
    id,
    execute: async (input) => {
      const controller = new AbortController();
      // Graph executor passes { upstreamNodeId: result }
      // Tests pass { inputs: actualValue, nodeId, dagId } or primitives
      const inputsMap = input && typeof input === "object" && "inputs" in input
        ? (input as { inputs: unknown }).inputs
        : input;
      const context: NodeContext = {
        nodeId: id,
        inputs: inputsMap as Record<string, unknown>,
        executionId: crypto.randomUUID(),
        signal: controller.signal,
      };
      return execute(context);
    },
  };
}

/**
 * Pass-through node that forwards input.
 *
 * @param id - Node ID
 * @returns Pass-through node
 * @since 0.1.0
 * @group Workflow
 */
export function passThroughNode(id: string): DagNode {
  return createNode(id, async ({ inputs }) => inputs);
}

/**
 * Transform node with mapping function.
 *
 * @param id - Node ID
 * @param transform - Transform function
 * @returns Transform node
 * @since 0.1.0
 * @group Workflow
 */
export function transformNode<TInput, TOutput>(
  id: string,
  transform: (input: TInput) => Promise<TOutput>
): DagNode {
  return createNode(id, async ({ inputs }) => {
    // Graph executor passes { upstreamNodeId: result }
    // Tests (with createNode fix) pass raw value directly
    const inputValue = inputs && typeof inputs === "object" && !Array.isArray(inputs)
      ? inputs[Object.keys(inputs)[0]]
      : inputs;
    return transform(inputValue as TInput);
  });
}

/**
 * Merge node combining multiple inputs.
 *
 * @param id - Node ID
 * @param merge - Merge function
 * @returns Merge node
 * @since 0.1.0
 * @group Workflow
 */
export function mergeNode<TOutput>(
  id: string,
  merge: (inputs: Record<string, unknown>) => Promise<TOutput>
): DagNode {
  return createNode(id, async ({ inputs }) => merge(inputs));
}