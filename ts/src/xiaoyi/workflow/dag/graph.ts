/**
 * Workflow DAG graph structure.
 *
 * Path: xiaoyi.workflow.dag.graph
 *
 * Layer hierarchy:
 * - 0: workflow
 * - 1: dag
 * - 2: graph
 * - 3: node/edge/cycle
 *
 * Directed acyclic graph for workflow execution ordering.
 */

export type NodeId = string;

export type NodeKind = "task" | "agent" | "chain" | "conditional" | "parallel";

export type EdgeKind = "sequential" | "conditional" | "parallel";

export interface DagNode {
  id: NodeId;
  label: string;
  kind: NodeKind;
  metadata: Record<string, string>;
}

export interface DagEdge {
  from: NodeId;
  to: NodeId;
  kind: EdgeKind;
}

export class DagGraph {
  private nodes = new Map<NodeId, DagNode>();
  private edges: DagEdge[] = [];
  private adjacency = new Map<NodeId, NodeId[]>();
  private reverseAdjacency = new Map<NodeId, NodeId[]>();

  addNode(node: DagNode): void {
    this.nodes.set(node.id, node);
    this.adjacency.set(node.id, []);
    this.reverseAdjacency.set(node.id, []);
  }

  addEdge(edge: DagEdge): void {
    if (!this.nodes.has(edge.from) || !this.nodes.has(edge.to)) {
      throw new Error("source or target node not found");
    }
    this.edges.push(edge);
    this.adjacency.get(edge.from)!.push(edge.to);
    this.reverseAdjacency.get(edge.to)!.push(edge.from);
  }

  topologicalOrder(): NodeId[] {
    // Kahn's algorithm
    const inDegree = new Map<NodeId, number>();
    for (const [nid, preds] of this.reverseAdjacency) {
      inDegree.set(nid, preds.length);
    }

    const queue: NodeId[] = [];
    for (const [nid, deg] of inDegree) {
      if (deg === 0) queue.push(nid);
    }

    const order: NodeId[] = [];
    while (queue.length > 0) {
      const nid = queue.shift()!;
      order.push(nid);
      for (const succ of this.adjacency.get(nid) ?? []) {
        const deg = inDegree.get(succ)! - 1;
        inDegree.set(succ, deg);
        if (deg === 0) queue.push(succ);
      }
    }

    if (order.length !== this.nodes.size) {
      throw new Error("graph contains cycles");
    }
    return order;
  }

  getNode(id: NodeId): DagNode | undefined {
    return this.nodes.get(id);
  }

  getNodes(): DagNode[] {
    return Array.from(this.nodes.values());
  }
}