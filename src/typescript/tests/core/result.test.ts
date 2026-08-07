import { describe, it, expect } from "vitest";
import {
  ok,
  err,
  isOk,
  isErr,
  unwrap,
  unwrapErr,
  map,
  mapErr,
  andThen,
  orElse,
  toPromise,
  Result,
  XiaoyiError,
  ErrorKind,
  createError,
} from "../../src/xiaoyi/core/result";

describe("core/result", () => {
  describe("ok / err constructors", () => {
    it("should create success result with ok()", () => {
      const result = ok(42);

      expect(isOk(result)).toBe(true);
      expect(isErr(result)).toBe(false);
      expect(result.ok).toBe(true);
      expect(result.value).toBe(42);
    });

    it("should create error result with err()", () => {
      const error = createError(ErrorKind.Runtime, "Failed");
      const result = err(error);

      expect(isOk(result)).toBe(false);
      expect(isErr(result)).toBe(true);
      expect(result.ok).toBe(false);
      expect(result.error).toBe(error);
    });

    it("should infer correct type parameters", () => {
      const success: Result<number, string> = ok(1);
      const failure: Result<number, string> = err("error");

      expect(isOk(success)).toBe(true);
      expect(isErr(failure)).toBe(true);
    });
  });

  describe("isOk / isErr type guards", () => {
    it("should narrow type correctly with isOk", () => {
      const result: Result<number, string> = ok(42);

      if (isOk(result)) {
        expect(result.value).toBe(42);
      } else {
        expect.fail("Should have narrowed to success");
      }
    });

    it("should narrow type correctly with isErr", () => {
      const result: Result<number, string> = err("error");

      if (isErr(result)) {
        expect(result.error).toBe("error");
      } else {
        expect.fail("Should have narrowed to error");
      }
    });

    it("should return false for opposite variant", () => {
      expect(isOk(err("error"))).toBe(false);
      expect(isErr(ok(42))).toBe(false);
    });
  });

  describe("unwrap / unwrapErr", () => {
    it("should unwrap success value", () => {
      const result = ok(42);
      expect(unwrap(result)).toBe(42);
    });

    it("should throw on unwrap of error", () => {
      const error = createError(ErrorKind.Config, "Missing config");
      const result = err(error);

      expect(() => unwrap(result)).toThrow(XiaoyiError);
    });

    it("should unwrap error value", () => {
      const error = createError(ErrorKind.Auth, "Unauthorized");
      const result = err(error);
      expect(unwrapErr(result)).toBe(error);
    });

    it("should throw on unwrapErr of success", () => {
      const result = ok(42);
      expect(() => unwrapErr(result)).toThrow("Expected error result");
    });
  });

  describe("map", () => {
    it("should transform success value", () => {
      const result = ok(2);
      const mapped = map(result, (x) => x * 3);

      expect(isOk(mapped)).toBe(true);
      if (isOk(mapped)) {
        expect(mapped.value).toBe(6);
      }
    });

    it("should pass through error unchanged", () => {
      const error = createError(ErrorKind.Runtime, "Fail");
      const result = err<number, XiaoyiError>(error);
      const mapped = map(result, (x) => x * 3);

      expect(isErr(mapped)).toBe(true);
      if (isErr(mapped)) {
        expect(mapped.error).toBe(error);
      }
    });

    it("should handle type changes", () => {
      const result = ok(42);
      const mapped = map(result, (x) => x.toString());

      expect(isOk(mapped)).toBe(true);
      if (isOk(mapped)) {
        expect(mapped.value).toBe("42");
        expect(typeof mapped.value).toBe("string");
      }
    });
  });

  describe("mapErr", () => {
    it("should transform error value", () => {
      const error = createError(ErrorKind.Config, "Original");
      const result = err<number, XiaoyiError>(error);
      const mapped = mapErr(result, (e) => createError(ErrorKind.Policy, "Transformed"));

      expect(isErr(mapped)).toBe(true);
      if (isErr(mapped)) {
        expect(mapped.error.message).toBe("Transformed");
        expect(mapped.error.kind).toBe(ErrorKind.Policy);
      }
    });

    it("should pass through success unchanged", () => {
      const result = ok(42);
      const mapped = mapErr(result, (e) => createError(ErrorKind.Policy, "New"));

      expect(isOk(mapped)).toBe(true);
      if (isOk(mapped)) {
        expect(mapped.value).toBe(42);
      }
    });
  });

  describe("andThen", () => {
    it("should chain successful operations", () => {
      const result = ok(2);
      const chained = andThen(result, (x) => ok(x * 3));
      const final = andThen(chained, (x) => ok(x + 1));

      expect(isOk(final)).toBe(true);
      if (isOk(final)) {
        expect(final.value).toBe(7);
      }
    });

    it("should short-circuit on first error", () => {
      const error = createError(ErrorKind.Runtime, "Failed");
      const result = ok(2);
      const chained = andThen(result, () => err(error));
      const final = andThen(chained, (x) => ok(x * 100));

      expect(isErr(final)).toBe(true);
      if (isErr(final)) {
        expect(final.error).toBe(error);
      }
    });

    it("should propagate error without calling subsequent functions", () => {
      let callCount = 0;
      const error = createError(ErrorKind.Config, "Missing");

      const result = andThen(err(error), () => {
        callCount++;
        return ok(1);
      });

      expect(callCount).toBe(0);
      expect(isErr(result)).toBe(true);
    });
  });

  describe("orElse", () => {
    it("should recover from error with alternative", () => {
      const error = createError(ErrorKind.Config, "Missing");
      const result = err<number, XiaoyiError>(error);
      const recovered = orElse(result, () => ok(42));

      expect(isOk(recovered)).toBe(true);
      if (isOk(recovered)) {
        expect(recovered.value).toBe(42);
      }
    });

    it("should pass through success unchanged", () => {
      const result = ok(10);
      const recovered = orElse(result, () => ok(42));

      expect(isOk(recovered)).toBe(true);
      if (isOk(recovered)) {
        expect(recovered.value).toBe(10);
      }
    });

    it("should chain multiple recovery attempts", () => {
      const error1 = createError(ErrorKind.Config, "Missing 1");
      const error2 = createError(ErrorKind.Auth, "Missing 2");
      const result = err<number, XiaoyiError>(error1);

      const recovered = orElse(result, () => err(error2));
      const final = orElse(recovered, () => ok(99));

      expect(isOk(final)).toBe(true);
      if (isOk(final)) {
        expect(final.value).toBe(99);
      }
    });
  });

  describe("toPromise", () => {
    it("should return Promise that resolves to result", async () => {
      const result = ok(42);
      const promise = toPromise(result);

      expect(promise).toBeInstanceOf(Promise);
      const resolved = await promise;
      expect(resolved).toBe(result);
    });

    it("should work with error result", async () => {
      const error = createError(ErrorKind.Runtime, "Fail");
      const result = err(error);
      const promise = toPromise(result);

      const resolved = await promise;
      expect(isErr(resolved)).toBe(true);
      if (isErr(resolved)) {
        expect(resolved.error).toBe(error);
      }
    });

    it("should be awaitable directly", async () => {
      const value = await toPromise(ok("hello"));
      expect(isOk(value)).toBe(true);
      if (isOk(value)) {
        expect(value.value).toBe("hello");
      }
    });
  });

  describe("complex composition", () => {
    it("should support map -> andThen -> orElse chain", () => {
      const result = ok(2);
      const processed = andThen(
        map(result, (x) => x * 5),
        (x) => (x > 5 ? ok(x + 1) : err(createError(ErrorKind.Policy, "Too small")))
      );
      const final = orElse(processed, () => ok(0));

      expect(isOk(final)).toBe(true);
      if (isOk(final)) {
        expect(final.value).toBe(11);
      }
    });

    it("should handle error path in composition", () => {
      const result = ok(1); // 1 * 5 = 5, not > 5
      const processed = andThen(
        map(result, (x) => x * 5),
        (x) => (x > 5 ? ok(x + 1) : err(createError(ErrorKind.Policy, "Too small")))
      );
      const final = orElse(processed, () => ok(0));

      expect(isOk(final)).toBe(true);
      if (isOk(final)) {
        expect(final.value).toBe(0);
      }
    });
  });
});