import { describe, it, expect, vi } from "vitest";
import {
  createNode,
  passThroughNode,
  transformNode,
  mergeNode,
  DagNode,
  NodeContext,
  TypedNodeExecute,
} from "@xiaoyi/xiaoyi/workflow/dag/node";

describe("workflow/dag/node", () => {
  describe("createNode", () => {
    it("should create typed node with id and execute function", () => {
      const execute: TypedNodeExecute<string, number> = async ({ inputs }) => inputs.length;

      const node = createNode("test-node", execute);

      expect(node).toBeDefined();
      expect(node.id).toBe("test-node");
      expect(node.execute).toBeDefined();
      expect(typeof node.execute).toBe("function");
    });

    it("should create node with dependencies", () => {
      const execute: TypedNodeExecute<unknown, string> = async () => "result";
      const node = createNode("dependent-node", execute);

      expect(node.dependencies).toBeUndefined(); // createNode doesn't set deps
    });

    it("should execute node with context", async () => {
      const execute: TypedNodeExecute<string, number> = async ({ inputs }) => inputs.length;
      const node = createNode("len-node", execute);

      const context: NodeContext = { inputs: "hello", nodeId: "len-node", dagId: "test" };
      const result = await node.execute(context);

      expect(result).toBe(5);
    });

    it("should infer generic types correctly", () => {
      const execute: TypedNodeExecute<{ a: number }, string> = async ({ inputs }) => `value: ${inputs.a}`;
      const node = createNode("typed", execute);

      expect(node.id).toBe("typed");
    });
  });

  describe("passThroughNode", () => {
    it("should create node that returns inputs", () => {
      const node = passThroughNode("passthrough");

      expect(node.id).toBe("passthrough");
      expect(node.execute).toBeDefined();
    });

    it("should pass through any input type", async () => {
      const node = passThroughNode("passthrough");

      const stringResult = await node.execute({ inputs: "hello", nodeId: "passthrough", dagId: "test" });
      expect(stringResult).toBe("hello");

      const numberResult = await node.execute({ inputs: 42, nodeId: "passthrough", dagId: "test" });
      expect(numberResult).toBe(42);

      const objectResult = await node.execute({ inputs: { a: 1 }, nodeId: "passthrough", dagId: "test" });
      expect(objectResult).toEqual({ a: 1 });
    });
  });

  describe("transformNode", () => {
    it("should create node with transform function", () => {
      const transform = async (input: number) => input * 2;
      const node = transformNode("double", transform);

      expect(node.id).toBe("double");
      expect(node.execute).toBeDefined();
    });

    it("should apply transform to input", async () => {
      const transform = async (input: string) => input.toUpperCase();
      const node = transformNode("uppercase", transform);

      const result = await node.execute({ inputs: "hello", nodeId: "uppercase", dagId: "test" });
      expect(result).toBe("HELLO");
    });

    it("should handle async transform", async () => {
      const transform = async (input: number) => {
        await Promise.resolve();
        return input + 10;
      };
      const node = transformNode("add10", transform);

      const result = await node.execute({ inputs: 5, nodeId: "add10", dagId: "test" });
      expect(result).toBe(15);
    });

    it("should infer types from transform function", () => {
      const transform = async (input: boolean) => !input;
      const node = transformNode("not", transform);

      expect(node.id).toBe("not");
    });
  });

  describe("mergeNode", () => {
    it("should create node with merge function", () => {
      const merge = async (inputs: Record<string, unknown>) => Object.values(inputs);
      const node = mergeNode("merge", merge);

      expect(node.id).toBe("merge");
      expect(node.execute).toBeDefined();
    });

    it("should merge multiple inputs", async () => {
      const merge = async (inputs: Record<string, unknown>) => {
        return { combined: Object.keys(inputs).sort() };
      };
      const node = mergeNode("combiner", merge);

      const result = await node.execute({
        inputs: { a: 1, b: 2, c: 3 },
        nodeId: "combiner",
        dagId: "test",
      });
      expect(result).toEqual({ combined: ["a", "b", "c"] });
    });

    it("should handle empty inputs", async () => {
      const merge = async (inputs: Record<string, unknown>) => ({ count: Object.keys(inputs).length });
      const node = mergeNode("counter", merge);

      const result = await node.execute({ inputs: {}, nodeId: "counter", dagId: "test" });
      expect(result).toEqual({ count: 0 });
    });

    it("should infer output type from merge function", () => {
      const merge = async (inputs: Record<string, unknown>) => inputs["primary"] as string;
      const node = mergeNode<string>("extractor", merge);

      expect(node.id).toBe("extractor");
    });
  });

  describe("NodeContext interface", () => {
    it("should have required properties", () => {
      const context: NodeContext = {
        inputs: { key: "value" },
        nodeId: "test-node",
        dagId: "test-dag",
      };

      expect(context.inputs).toEqual({ key: "value" });
      expect(context.nodeId).toBe("test-node");
      expect(context.dagId).toBe("test-dag");
    });
  });

  describe("TypedNodeExecute type", () => {
    it("should accept context and return typed output", async () => {
      const execute: TypedNodeExecute<number, string> = async ({ inputs }) => `number: ${inputs}`;

      const result = await execute({ inputs: 42, nodeId: "test", dagId: "dag" });
      expect(result).toBe("number: 42");
    });

    it("should work with complex input/output types", async () => {
      interface Input { items: string[] }
      interface Output { count: number; first: string }

      const execute: TypedNodeExecute<Input, Output> = async ({ inputs }) => ({
        count: inputs.items.length,
        first: inputs.items[0] || "",
      });

      const result = await execute({ inputs: { items: ["a", "b", "c"] }, nodeId: "test", dagId: "dag" });
      expect(result).toEqual({ count: 3, first: "a" });
    });
  });

  describe("node composition", () => {
    it("should create different node types for different purposes", () => {
      const passThrough = passThroughNode("pt");
      const transform = transformNode("tx", async (x: number) => x * 2);
      const merge = mergeNode("mg", async (inputs) => Object.values(inputs));

      expect(passThrough.id).toBe("pt");
      expect(transform.id).toBe("tx");
      expect(merge.id).toBe("mg");
    });

    it("should all implement DagNode interface", () => {
      const nodes = [
        passThroughNode("1"),
        transformNode("2", async (x) => x),
        mergeNode("3", async (x) => x),
        createNode("4", async ({ inputs }) => inputs),
      ];

      for (const node of nodes) {
        expect(node.id).toBeDefined();
        expect(typeof node.execute).toBe("function");
      }
    });
  });
});