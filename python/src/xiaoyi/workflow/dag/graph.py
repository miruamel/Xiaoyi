"""Workflow DAG graph structure.

Path: xiaoyi.workflow.dag.graph

Layer hierarchy:
- 0: workflow
- 1: dag
- 2: graph
- 3: node/edge/cycle

Directed acyclic graph for workflow execution ordering.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class NodeKind(Enum):
    TASK = "task"
    AGENT = "agent"
    CHAIN = "chain"
    CONDITIONAL = "conditional"
    PARALLEL = "parallel"


class EdgeKind(Enum):
    SEQUENTIAL = "sequential"
    CONDITIONAL = "conditional"
    PARALLEL = "parallel"


@dataclass
class DagNode:
    id: str
    label: str
    kind: NodeKind
    metadata: dict[str, str] = field(default_factory=dict)


@dataclass
class DagEdge:
    from_id: str
    to_id: str
    kind: EdgeKind


class DagGraph:
    """Directed acyclic graph for workflow execution."""

    def __init__(self) -> None:
        self._nodes: dict[str, DagNode] = {}
        self._edges: list[DagEdge] = []
        self._adjacency: dict[str, list[str]] = {}
        self._reverse_adjacency: dict[str, list[str]] = {}

    def add_node(self, node: DagNode) -> None:
        self._nodes[node.id] = node
        self._adjacency.setdefault(node.id, [])
        self._reverse_adjacency.setdefault(node.id, [])

    def add_edge(self, edge: DagEdge) -> None:
        if edge.from_id not in self._nodes or edge.to_id not in self._nodes:
            raise ValueError("source or target node not found")
        self._edges.append(edge)
        self._adjacency[edge.from_id].append(edge.to_id)
        self._reverse_adjacency[edge.to_id].append(edge.from_id)

    def topological_order(self) -> list[str]:
        """Return nodes in topological order (Kahn's algorithm)."""
        from collections import deque

        in_degree = {nid: len(self._reverse_adjacency[nid]) for nid in self._nodes}
        queue = deque([nid for nid, deg in in_degree.items() if deg == 0])
        order = []

        while queue:
            nid = queue.popleft()
            order.append(nid)
            for succ in self._adjacency[nid]:
                in_degree[succ] -= 1
                if in_degree[succ] == 0:
                    queue.append(succ)

        if len(order) != len(self._nodes):
            raise ValueError("graph contains cycles")

        return order

    def get_node(self, node_id: str) -> DagNode | None:
        return self._nodes.get(node_id)

    def nodes(self) -> list[DagNode]:
        return list(self._nodes.values())