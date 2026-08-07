import { describe, it, expect, vi } from "vitest";
import { DagBuilder, Dag, DagNode, DagNodeExecute } from "../../src/xiaoyi/workflow/dag";

describe("workflow/dag/graph", () => {
  describe("DagBuilder", () => {
    it("should create builder instance", () => {
      const builder = new DagBuilder();
      expect(builder).toBeInstanceOf(DagBuilder);
    });

    it("should add nodes with addNode", () => {
      const builder = new DagBuilder();
      const executeFn: DagNodeExecute = async (input) => input;

      const result = builder.addNode("node1", executeFn);

      expect(result).toBe(builder); // chaining
    });

    it("should add edges with addEdge", () => {
      const builder = new DagBuilder();
      const executeFn: DagNodeExecute = async (input) => input;

      builder.addNode("node1", executeFn).addNode("node2", executeFn);
      const result = builder.addEdge("node1", "node2");

      expect(result).toBe(builder); // chaining
    });

    it("should build Dag with nodes and edges", () => {
      const builder = new DagBuilder();
      const executeFn: DagNodeExecute = async (input) => input;

      builder.addNode("node1", executeFn).addNode("node2", executeFn).addEdge("node1", "node2");
      const dag = builder.build();

      expect(dag).toBeInstanceOf(Dag);
    });

    it("should support full chaining", () => {
      const executeFn: DagNodeExecute = async (input) => input;

      const dag = new DagBuilder()
        .addNode("a", executeFn)
        .addNode("b", executeFn)
        .addNode("c", executeFn)
        .addEdge("a", "b")
        .addEdge("b", "c")
        .build();

      expect(dag).toBeInstanceOf(Dag);
    });
  });

  describe("DagNode interface", () => {
    it("should have required properties", () => {
      const node: DagNode = {
        id: "test-node",
        execute: async (input) => input,
        dependencies: ["dep1", "dep2"],
      };

      expect(node.id).toBe("test-node");
      expect(node.dependencies).toEqual(["dep1", "dep2"]);
      expect(typeof node.execute).toBe("function");
    });

    it("should allow optional dependencies", () => {
      const node: DagNode = {
        id: "test-node",
        execute: async (input) => input,
      };

      expect(node.id).toBe("test-node");
      expect(node.dependencies).toBeUndefined();
    });
  });

  describe("Dag class", () => {
    it("should store nodes", () => {
      const executeFn: DagNodeExecute = async (input) => input;
      const dag = new DagBuilder().addNode("node1", executeFn).build();

      const nodes = dag.getNodes();
      expect(nodes).toHaveLength(1);
      expect(nodes[0].id).toBe("node1");
    });

    it("should store edges", () => {
      const executeFn: DagNodeExecute = async (input) => input;
      const dag = new DagBuilder()
        .addNode("node1", executeFn)
        .addNode("node2", executeFn)
        .addEdge("node1", "node2")
        .build();

      const nodes = dag.getNodes();
      expect(nodes[1].dependencies).toContain("node1");
    });

    it("should return correct node count", () => {
      const executeFn: DagNodeExecute = async (input) => input;
      const dag = new DagBuilder()
        .addNode("a", executeFn)
        .addNode("b", executeFn)
        .addNode("c", executeFn)
        .build();

      expect(dag.getNodes()).toHaveLength(3);
    });

    it("should get node by id", () => {
      const executeFn: DagNodeExecute = async (input) => input;
      const dag = new DagBuilder().addNode("target", executeFn).build();

      const node = dag.getNode("target");
      expect(node).toBeDefined();
      expect(node?.id).toBe("target");
    });

    it("should return undefined for non-existent node", () => {
      const executeFn: DagNodeExecute = async (input) => input;
      const dag = new DagBuilder().addNode("existing", executeFn).build();

      const node = dag.getNode("nonexistent");
      expect(node).toBeUndefined();
    });

    it("should validate DAG (no cycles) on build", () => {
      const executeFn: DagNodeExecute = async (input) => input;
      const builder = new DagBuilder()
        .addNode("a", executeFn)
        .addNode("b", executeFn)
        .addEdge("a", "b")
        .addEdge("b", "a"); // Creates cycle

      expect(() => builder.build()).toThrow();
    });

    it("should validate all edge targets exist", () => {
      const executeFn: DagNodeExecute = async (input) => input;
      const builder = new DagBuilder()
        .addNode("a", executeFn)
        .addEdge("a", "nonexistent");

      expect(() => builder.build()).toThrow();
    });

    it("should validate all edge sources exist", () => {
      const executeFn: DagNodeExecute = async (input) => input;
      const builder = new DagBuilder()
        .addNode("b", executeFn)
        .addEdge("nonexistent", "b");

      expect(() => builder.build()).toThrow();
    });
  });

  describe("DagNodeExecute type", () => {
    it("should accept async function", async () => {
      const execute: DagNodeExecute = async (input) => {
        return { processed: input };
      };

      const result = await execute("test");
      expect(result).toEqual({ processed: "test" });
    });

    it("should handle Promise return", async () => {
      const execute: DagNodeExecute = async (input) => {
        return Promise.resolve(input * 2);
      };

      const result = await execute(5);
      expect(result).toBe(10);
    });
  });
});