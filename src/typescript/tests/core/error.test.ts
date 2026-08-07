import { describe, it, expect } from "vitest";
import { ErrorKind, XiaoyiError, createError, isXiaoyiError } from "@xiaoyi/xiaoyi/core/error";

describe("core/error", () => {
  describe("ErrorKind enum", () => {
    it("should have all expected error kinds", () => {
      expect(ErrorKind.Syntax).toBe("syntax");
      expect(ErrorKind.Parse).toBe("parse");
      expect(ErrorKind.Runtime).toBe("runtime");
      expect(ErrorKind.Io).toBe("io");
      expect(ErrorKind.Auth).toBe("auth");
      expect(ErrorKind.Policy).toBe("policy");
      expect(ErrorKind.Llm).toBe("llm");
      expect(ErrorKind.Memory).toBe("memory");
      expect(ErrorKind.Tool).toBe("tool");
      expect(ErrorKind.Workflow).toBe("workflow");
      expect(ErrorKind.Config).toBe("config");
      expect(ErrorKind.State).toBe("state");
    });

    it("should have exactly 12 error kinds", () => {
      const keys = Object.keys(ErrorKind).filter((k) => isNaN(Number(k)));
      expect(keys).toHaveLength(12);
    });
  });

  describe("createError", () => {
    it("should create a XiaoyiError with kind, message, and empty meta by default", () => {
      const error = createError(ErrorKind.Config, "Failed to load config");

      expect(error).toBeInstanceOf(Error);
      expect(error.name).toBe("XiaoyiError");
      expect(error.kind).toBe(ErrorKind.Config);
      expect(error.message).toBe("Failed to load config");
      expect(error.meta).toEqual({});
    });

    it("should create a XiaoyiError with custom metadata", () => {
      const meta = { path: "./config.toml", line: "42" };
      const error = createError(ErrorKind.Parse, "Invalid TOML", meta);

      expect(error.kind).toBe(ErrorKind.Parse);
      expect(error.message).toBe("Invalid TOML");
      expect(error.meta).toEqual(meta);
    });

    it("should create distinct error instances", () => {
      const error1 = createError(ErrorKind.Runtime, "Error 1");
      const error2 = createError(ErrorKind.Runtime, "Error 2");

      expect(error1).not.toBe(error2);
      expect(error1.message).not.toBe(error2.message);
    });
  });

  describe("isXiaoyiError", () => {
    it("should return true for XiaoyiError instances", () => {
      const error = createError(ErrorKind.Auth, "Unauthorized");
      expect(isXiaoyiError(error)).toBe(true);
    });

    it("should return false for plain Error instances", () => {
      const error = new Error("Plain error");
      expect(isXiaoyiError(error)).toBe(false);
    });

    it("should return false for non-error values", () => {
      expect(isXiaoyiError(null)).toBe(false);
      expect(isXiaoyiError(undefined)).toBe(false);
      expect(isXiaoyiError("string")).toBe(false);
      expect(isXiaoyiError(123)).toBe(false);
      expect(isXiaoyiError({})).toBe(false);
      expect(isXiaoyiError({ kind: ErrorKind.Config, meta: {} })).toBe(false);
    });

    it("should correctly narrow type in conditional", () => {
      const unknownError: unknown = createError(ErrorKind.Memory, "OOM");

      if (isXiaoyiError(unknownError)) {
        expect(unknownError.kind).toBe(ErrorKind.Memory);
        expect(unknownError.meta).toBeDefined();
      } else {
        expect.fail("Should have narrowed to XiaoyiError");
      }
    });
  });

  describe("XiaoyiError interface", () => {
    it("should have required properties: kind and meta", () => {
      const error = createError(ErrorKind.Tool, "Tool failed", { tool: "calculator" });

      expect("kind" in error).toBe(true);
      expect("meta" in error).toBe(true);
      expect(typeof error.kind).toBe("string");
      expect(typeof error.meta).toBe("object");
    });
    it("should preserve Error prototype chain", () => {
      const error = createError(ErrorKind.Llm, "Model unavailable");
      expect(error instanceof Error).toBe(true);
      // XiaoyiError is a TypeScript interface, not a class - no instanceof check
      expect(isXiaoyiError(error)).toBe(true);
    });
  });
});