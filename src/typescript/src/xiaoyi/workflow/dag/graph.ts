/**
 * # Workflow DAG Module
 *
 * `dag` provides Directed Acyclic Graph execution for workflows.
 *
 * Path: `xiaoyi.workflow.dag`
 *
 * - Layer 0: `workflow`
 * - Layer 1: `dag` — DAG execution engine.
 * - Layer 2: `graph` — graph structure and operations.
 * - Layer 3: `node` — node types and execution.
 * - Layer 4: `executor` — DAG execution logic.
 *
 * @module workflow.dag
 * @brief DAG-based workflow execution
 * @group Workflow
 * @since 0.1.0
 * @author Miruamel
 * @see workflow
 * @see workflow.dag.graph
 * @see workflow.dag.executor
 */
export * from "./graph";
export * from "./node";
export * from "./executor";

/**
 * Workflow DAG builder.
 *
 * @brief Build and execute DAG workflows
 * @group Workflow
 * @since 0.1.0
 * @example
 * ```typescript
 * const dag = new DagBuilder()
 *   .addNode("step1", async () => { ... })
 *   .addNode("step2", async () => { ... })
 *   .addEdge("step1", "step2")
 *   .build();
 * await dag.execute(input);
 * ```
 */
export class DagBuilder {
  private nodes: Map<string, DagNode> = new Map();
  private edges: Map<string, string[]> = new Map();

  /**
   * Add a node to the DAG.
   *
   * @param id - Node identifier
   * @param execute - Node execution function
   * @returns this (for chaining)
   * @since 0.1.0
   */
  addNode(id: string, execute: DagNodeExecute): this {
    this.nodes.set(id, { id, execute });
    this.edges.set(id, []);
    return this;
  }

  /**
   * Add a directed edge between nodes.
   *
   * @param from - Source node ID
   * @param to - Target node ID
   * @returns this (for chaining)
   * @since 0.1.0
   */
  addEdge(from: string, to: string): this {
    const edges = this.edges.get(from) ?? [];
    edges.push(to);
    this.edges.set(from, edges);
    return this;
  }

  /**
   * Build the DAG.
   *
   * @returns Executable DAG
   * @since 0.1.0
   */
  build(): Dag {
    return new Dag(this.nodes, this.edges);
  }
}

/**
 * DAG node definition.
 *
 * @brief Node in a workflow DAG
 * @group Workflow
 * @since 0.1.0
 */
export interface DagNode {
  /** Node identifier. */
  id: string;
  /** Execution function. */
  execute: DagNodeExecute;
}

/**
 * DAG node execution function.
 *
 * @param input - Input data from upstream nodes
 * @returns Output data for downstream nodes
 * @since 0.1.0
 */
export type DagNodeExecute = (input: unknown) => Promise<unknown>;

/**
 * Directed Acyclic Graph for workflow execution.
 *
 * @brief Executable workflow DAG
 * @group Workflow
 * @since 0.1.0
 */
export class Dag {
  constructor(
    public readonly nodes: Map<string, DagNode>,
    public readonly edges: Map<string, string[]>
  ) {}

  /**
   * Execute the DAG with input.
   *
   * @param input - Initial input
   * @returns Final output
   * @since 0.1.0
   */
  async execute(input: unknown): Promise<unknown> {
    const results = new Map<string, unknown>();
    const visited = new Set<string>();

    const executeNode = async (nodeId: string): Promise<unknown> => {
      if (visited.has(nodeId)) return results.get(nodeId);
      visited.add(nodeId);

      const node = this.nodes.get(nodeId);
      if (!node) throw new Error(`Node not found: ${nodeId}`);

      // Get inputs from upstream nodes
      const inputs: Record<string, unknown> = {};
      for (const [from, tos] of this.edges) {
        if (tos.includes(nodeId)) {
          inputs[from] = await executeNode(from);
        }
      }

      const output = await node.execute(inputs);
      results.set(nodeId, output);
      return output;
    };

    // Execute all nodes (handles disconnected components)
    for (const nodeId of this.nodes.keys()) {
      await executeNode(nodeId);
    }

    // Return last node's result or all results
    const nodeIds = Array.from(this.nodes.keys());
    return nodeIds.length === 1
      ? results.get(nodeIds[0])
      : Object.fromEntries(results);
  }

  /**
   * Get topological order of nodes.
   *
   * @returns Node IDs in topological order
   * @since 0.1.0
   */
  topologicalOrder(): string[] {
    const visited = new Set<string>();
    const order: string[] = [];

    const visit = (nodeId: string) => {
      if (visited.has(nodeId)) return;
      visited.add(nodeId);
      for (const to of this.edges.get(nodeId) ?? []) {
        visit(to);
      }
      order.push(nodeId);
    };

    for (const nodeId of this.nodes.keys()) {
      visit(nodeId);
    }

    return order.reverse();
  }
}