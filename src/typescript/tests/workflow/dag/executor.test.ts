import { describe, it, expect, vi } from "vitest";
import { executeDag, ExecutionMode, ExecutorOptions, NodeResult, DagExecutionResult, defaultExecutorOptions } from "@xiaoyi/xiaoyi/workflow/dag/executor";
import { DagBuilder, Dag, DagNodeExecute } from "@xiaoyi/xiaoyi/workflow/dag";

describe("workflow/dag/executor", () => {
  describe("ExecutionMode enum", () => {
    it("should have expected values", () => {
      expect(ExecutionMode.Sequential).toBe("sequential");
      expect(ExecutionMode.Parallel).toBe("parallel");
    });
  });

  describe("defaultExecutorOptions", () => {
    it("should return default options", () => {
      const options = defaultExecutorOptions();

      expect(options.mode).toBe(ExecutionMode.Sequential);
      expect(options.maxConcurrency).toBe(4);
      expect(options.timeout).toBeUndefined();
      expect(options.onNodeStart).toBeUndefined();
      expect(options.onNodeComplete).toBeUndefined();
      expect(options.onNodeError).toBeUndefined();
    });
  });

  describe("ExecutorOptions interface", () => {
    it("should accept all optional properties", () => {
      const options: ExecutorOptions = {
        mode: ExecutionMode.Parallel,
        maxConcurrency: 8,
        timeout: 5000,
        onNodeStart: vi.fn(),
        onNodeComplete: vi.fn(),
        onNodeError: vi.fn(),
      };

      expect(options.mode).toBe(ExecutionMode.Parallel);
      expect(options.maxConcurrency).toBe(8);
      expect(options.timeout).toBe(5000);
    });

    it("should allow partial options", () => {
      const options: ExecutorOptions = { mode: ExecutionMode.Parallel };

      expect(options.mode).toBe(ExecutionMode.Parallel);
      expect(options.maxConcurrency).toBeUndefined();
    });
  });

  describe("NodeResult interface", () => {
    it("should have required properties for success", () => {
      const result: NodeResult = {
        nodeId: "node1",
        success: true,
        output: { value: 42 },
        duration: 100,
      };

      expect(result.nodeId).toBe("node1");
      expect(result.success).toBe(true);
      expect(result.output).toEqual({ value: 42 });
      expect(result.duration).toBe(100);
      expect(result.error).toBeUndefined();
    });

    it("should have required properties for error", () => {
      const result: NodeResult = {
        nodeId: "node1",
        success: false,
        error: new Error("Failed"),
        duration: 50,
      };

      expect(result.nodeId).toBe("node1");
      expect(result.success).toBe(false);
      expect(result.error).toBeInstanceOf(Error);
      expect(result.output).toBeUndefined();
    });
  });

  describe("DagExecutionResult interface", () => {
    it("should have required properties", () => {
      const result: DagExecutionResult = {
        success: true,
        output: { final: "result" },
        nodeResults: [],
        totalDuration: 1000,
      };

      expect(result.success).toBe(true);
      expect(result.output).toEqual({ final: "result" });
      expect(Array.isArray(result.nodeResults)).toBe(true);
      expect(result.totalDuration).toBe(1000);
      expect(result.error).toBeUndefined();
    });

    it("should include error on failure", () => {
      const result: DagExecutionResult = {
        success: false,
        error: new Error("DAG failed"),
        nodeResults: [],
        totalDuration: 500,
      };

      expect(result.success).toBe(false);
      expect(result.error).toBeInstanceOf(Error);
    });
  });

  describe("executeDag - sequential mode", () => {
    it("should execute nodes in dependency order", async () => {
      const executeFn: DagNodeExecute = async (input) => {
        await Promise.resolve(); // simulate async work
        return input;
      };

      const dag = new DagBuilder()
        .addNode("a", executeFn)
        .addNode("b", executeFn)
        .addNode("c", executeFn)
        .addEdge("a", "b")
        .addEdge("b", "c")
        .build();

      const result = await executeDag(dag, "initial", { mode: ExecutionMode.Sequential });

      expect(result.success).toBe(true);
      expect(result.nodeResults).toHaveLength(3);
      expect(result.nodeResults[0].nodeId).toBe("a");
      expect(result.nodeResults[1].nodeId).toBe("b");
      expect(result.nodeResults[2].nodeId).toBe("c");
    });

    it("should pass output from one node to next", async () => {
      const dag = new DagBuilder()
        .addNode("step1", async (input: string) => `${input}-step1`)
        .addNode("step2", async (input: string) => `${input}-step2`)
        .addEdge("step1", "step2")
        .build();

      const result = await executeDag(dag, "start", { mode: ExecutionMode.Sequential });

      expect(result.success).toBe(true);
      expect(result.output).toBe("start-step1-step2");
    });

    it("should collect node results with duration", async () => {
      const dag = new DagBuilder()
        .addNode("fast", async (input) => input)
        .addNode("slow", async (input) => {
          await new Promise((r) => setTimeout(r, 10));
          return input;
        })
        .addEdge("fast", "slow")
        .build();

      const result = await executeDag(dag, "test", { mode: ExecutionMode.Sequential });

      expect(result.nodeResults).toHaveLength(2);
      expect(result.nodeResults[0].duration).toBeLessThan(result.nodeResults[1].duration);
    });
  });

  describe("executeDag - parallel mode", () => {
    it("should execute independent nodes in parallel", async () => {
      const durations: number[] = [];

      const createTimedNode = (name: string, delay: number): DagNodeExecute => {
        return async (input) => {
          const start = Date.now();
          await new Promise((r) => setTimeout(r, delay));
          durations.push(Date.now() - start);
          return { [name]: input };
        };
      };

      const dag = new DagBuilder()
        .addNode("a", createTimedNode("a", 20))
        .addNode("b", createTimedNode("b", 20))
        .addNode("c", createTimedNode("c", 20))
        .build();

      const result = await executeDag(dag, "input", { mode: ExecutionMode.Parallel, maxConcurrency: 3 });

      expect(result.success).toBe(true);
      expect(result.nodeResults).toHaveLength(3);
      // Parallel execution should take ~20ms total, not 60ms
      expect(result.totalDuration).toBeLessThan(100);
    });

    it("should respect maxConcurrency limit", async () => {
      const runningCount = { current: 0, max: 0 };

      const createNode = (name: string): DagNodeExecute => {
        return async (input) => {
          runningCount.current++;
          runningCount.max = Math.max(runningCount.max, runningCount.current);
          await new Promise((r) => setTimeout(r, 30));
          runningCount.current--;
          return { [name]: input };
        };
      };

      const dag = new DagBuilder()
        .addNode("1", createNode("1"))
        .addNode("2", createNode("2"))
        .addNode("3", createNode("3"))
        .addNode("4", createNode("4"))
        .build();

      await executeDag(dag, "input", { mode: ExecutionMode.Parallel, maxConcurrency: 2 });

      expect(runningCount.max).toBeLessThanOrEqual(2);
    });

    it("should handle mixed dependencies in parallel mode", async () => {
      const dag = new DagBuilder()
        .addNode("a", async (input) => `${input}-a`)
        .addNode("b", async (input) => `${input}-b`)
        .addNode("c", async (input) => `${input}-c`)
        .addEdge("a", "c")
        .build();

      const result = await executeDag(dag, "start", { mode: ExecutionMode.Parallel });

      expect(result.success).toBe(true);
      // a and b run in parallel, c waits for a
      expect(result.output).toContain("start-a-c");
      expect(result.output).toContain("start-b");
    });
  });

  describe("executeDag - error handling", () => {
    it("should stop on node error in sequential mode", async () => {
      const dag = new DagBuilder()
        .addNode("success", async (input) => input)
        .addNode("fail", async () => { throw new Error("Node failed"); })
        .addNode("after-fail", async (input) => input)
        .addEdge("success", "fail")
        .addEdge("fail", "after-fail")
        .build();

      const result = await executeDag(dag, "input", { mode: ExecutionMode.Sequential });

      expect(result.success).toBe(false);
      expect(result.error).toBeInstanceOf(Error);
      expect(result.error?.message).toBe("Node failed");
      expect(result.nodeResults).toHaveLength(2); // only success and fail executed
    });

    it("should stop on node error in parallel mode", async () => {
      const dag = new DagBuilder()
        .addNode("a", async (input) => input)
        .addNode("b", async () => { throw new Error("Parallel fail"); })
        .addNode("c", async (input) => input)
        .build();

      const result = await executeDag(dag, "input", { mode: ExecutionMode.Parallel });

      expect(result.success).toBe(false);
      expect(result.error?.message).toBe("Parallel fail");
    });

    it("should call onNodeError callback", async () => {
      const onError = vi.fn();
      const dag = new DagBuilder()
        .addNode("fail", async () => { throw new Error("Callback test"); })
        .build();

      await executeDag(dag, "input", { onNodeError: onError });

      expect(onError).toHaveBeenCalledWith("fail", expect.any(Error));
    });

    it("should call onNodeStart and onNodeComplete callbacks", async () => {
      const onStart = vi.fn();
      const onComplete = vi.fn();
      const dag = new DagBuilder()
        .addNode("node1", async (input) => input)
        .addNode("node2", async (input) => input)
        .addEdge("node1", "node2")
        .build();

      await executeDag(dag, "input", { onNodeStart: onStart, onNodeComplete: onComplete });

      expect(onStart).toHaveBeenCalledTimes(2);
      expect(onComplete).toHaveBeenCalledTimes(2);
      expect(onStart).toHaveBeenNthCalledWith(1, "node1");
      expect(onComplete).toHaveBeenNthCalledWith(1, "node1", expect.any(Object));
    });
  });

  describe("executeDag - timeout", () => {
    it("should timeout long-running nodes", async () => {
      const dag = new DagBuilder()
        .addNode("slow", async (input) => {
          await new Promise((r) => setTimeout(r, 100));
          return input;
        })
        .build();

      const result = await executeDag(dag, "input", { timeout: 10 });

      expect(result.success).toBe(false);
      expect(result.error?.message).toContain("timeout");
    });

    it("should complete within timeout for fast nodes", async () => {
      const dag = new DagBuilder()
        .addNode("fast", async (input) => input)
        .build();

      const result = await executeDag(dag, "input", { timeout: 1000 });

      expect(result.success).toBe(true);
    });
  });

  describe("executeDag - complex scenarios", () => {
    it("should handle diamond dependency pattern", async () => {
      const dag = new DagBuilder()
        .addNode("start", async (input) => input)
        .addNode("left", async (input) => `${input}-left`)
        .addNode("right", async (input) => `${input}-right`)
        .addNode("merge", async (input) => `${input}-merged`)
        .addEdge("start", "left")
        .addEdge("start", "right")
        .addEdge("left", "merge")
        .addEdge("right", "merge")
        .build();

      const result = await executeDag(dag, "root", { mode: ExecutionMode.Parallel });

      expect(result.success).toBe(true);
      // left and right run in parallel after start, merge waits for both
    });

    it("should return final output from last node", async () => {
      const dag = new DagBuilder()
        .addNode("final", async (input) => ({ result: input * 2 }))
        .build();

      const result = await executeDag(dag, 21);

      expect(result.success).toBe(true);
      expect(result.output).toEqual({ result: 42 });
    });
  });
});