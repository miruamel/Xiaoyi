import { describe, it, expect } from "vitest";
import {
  F32,
  F64,
  F32Bits,
  F64Bits,
  F32Consts,
  F64Consts,
  FloatKind,
  isF32Finite,
  isF32NaN,
  isF32Infinite,
  isF64Finite,
  isF64NaN,
  isF64Infinite,
} from "@xiaoyi/xiaoyi/domain/token/primitive/float";

describe("domain/token/primitive/float", () => {
  describe("FloatKind enum", () => {
    it("should have F32 and F64 values", () => {
      expect(FloatKind.F32).toBe("f32");
      expect(FloatKind.F64).toBe("f64");
    });
  });

  describe("F32 type alias", () => {
    it("should be number type", () => {
      const value: F32 = 3.14;
      expect(typeof value).toBe("number");
    });
  });

  describe("F64 type alias", () => {
    it("should be number type", () => {
      const value: F64 = 3.14;
      expect(typeof value).toBe("number");
    });
  });

  describe("F32Bits type alias", () => {
    it("should be number type", () => {
      const bits: F32Bits = 0x40490fdb;
      expect(typeof bits).toBe("number");
    });
  });

  describe("F64Bits type alias", () => {
    it("should be bigint type", () => {
      const bits: F64Bits = 0x400921fb54442d18n;
      expect(typeof bits).toBe("bigint");
    });
  });

  describe("F32Consts", () => {
    it("should have expected constants", () => {
      expect(F32Consts.INFINITY).toBe(Infinity);
      expect(F32Consts.NEG_INFINITY).toBe(-Infinity);
      expect(F32Consts.NAN).toBe(NaN);
    });

    it("should have MIN_POSITIVE close to 1.175494351e-38", () => {
      expect(F32Consts.MIN_POSITIVE).toBeCloseTo(1.175494351e-38);
    });

    it("should have MAX close to 3.402823466e+38", () => {
      expect(F32Consts.MAX).toBeCloseTo(3.402823466e+38);
    });

    it("should have MIN close to -3.402823466e+38", () => {
      expect(F32Consts.MIN).toBeCloseTo(-3.402823466e+38);
    });

    it("should have EPSILON close to 1.19209290e-7", () => {
      expect(F32Consts.EPSILON).toBeCloseTo(1.19209290e-7);
    });
  });

  describe("F64Consts", () => {
    it("should have expected constants", () => {
      expect(F64Consts.INFINITY).toBe(Infinity);
      expect(F64Consts.NEG_INFINITY).toBe(-Infinity);
      expect(F64Consts.NAN).toBe(NaN);
    });

    it("should have MIN_POSITIVE close to 2.2250738585072014e-308", () => {
      expect(F64Consts.MIN_POSITIVE).toBeCloseTo(2.2250738585072014e-308);
    });

    it("should have MAX close to 1.7976931348623157e+308", () => {
      expect(F64Consts.MAX).toBeCloseTo(1.7976931348623157e+308);
    });

    it("should have MIN close to -1.7976931348623157e+308", () => {
      expect(F64Consts.MIN).toBeCloseTo(-1.7976931348623157e+308);
    });

    it("should have EPSILON close to 2.220446049250313e-16", () => {
      expect(F64Consts.EPSILON).toBeCloseTo(2.220446049250313e-16);
    });
  });

  describe("isF32Finite", () => {
    it("should return true for finite numbers", () => {
      expect(isF32Finite(0)).toBe(true);
      expect(isF32Finite(1)).toBe(true);
      expect(isF32Finite(-1)).toBe(true);
      expect(isF32Finite(1.5)).toBe(true);
      expect(isF32Finite(F32Consts.MAX)).toBe(true);
      expect(isF32Finite(F32Consts.MIN)).toBe(true);
    });

    it("should return false for Infinity", () => {
      expect(isF32Finite(Infinity)).toBe(false);
      expect(isF32Finite(-Infinity)).toBe(false);
      expect(isF32Finite(F32Consts.INFINITY)).toBe(false);
      expect(isF32Finite(F32Consts.NEG_INFINITY)).toBe(false);
    });

    it("should return false for NaN", () => {
      expect(isF32Finite(NaN)).toBe(false);
      expect(isF32Finite(F32Consts.NAN)).toBe(false);
    });
  });

  describe("isF64Finite", () => {
    it("should return true for finite numbers", () => {
      expect(isF64Finite(0)).toBe(true);
      expect(isF64Finite(1)).toBe(true);
      expect(isF64Finite(-1)).toBe(true);
      expect(isF64Finite(1.5)).toBe(true);
      expect(isF64Finite(F64Consts.MAX)).toBe(true);
      expect(isF64Finite(F64Consts.MIN)).toBe(true);
    });

    it("should return false for Infinity", () => {
      expect(isF64Finite(Infinity)).toBe(false);
      expect(isF64Finite(-Infinity)).toBe(false);
      expect(isF64Finite(F64Consts.INFINITY)).toBe(false);
      expect(isF64Finite(F64Consts.NEG_INFINITY)).toBe(false);
    });

    it("should return false for NaN", () => {
      expect(isF64Finite(NaN)).toBe(false);
      expect(isF64Finite(F64Consts.NAN)).toBe(false);
    });
  });

  describe("isF32NaN", () => {
    it("should return true for NaN", () => {
      expect(isF32NaN(NaN)).toBe(true);
      expect(isF32NaN(F32Consts.NAN)).toBe(true);
      expect(isF32NaN(0 / 0)).toBe(true);
    });

    it("should return false for non-NaN values", () => {
      expect(isF32NaN(0)).toBe(false);
      expect(isF32NaN(1)).toBe(false);
      expect(isF32NaN(Infinity)).toBe(false);
      expect(isF32NaN(-Infinity)).toBe(false);
    });
  });

  describe("isF64NaN", () => {
    it("should return true for NaN", () => {
      expect(isF64NaN(NaN)).toBe(true);
      expect(isF64NaN(F64Consts.NAN)).toBe(true);
      expect(isF64NaN(0 / 0)).toBe(true);
    });

    it("should return false for non-NaN values", () => {
      expect(isF64NaN(0)).toBe(false);
      expect(isF64NaN(1)).toBe(false);
      expect(isF64NaN(Infinity)).toBe(false);
      expect(isF64NaN(-Infinity)).toBe(false);
    });
  });

  describe("isF32Infinite", () => {
    it("should return true for Infinity", () => {
      expect(isF32Infinite(Infinity)).toBe(true);
      expect(isF32Infinite(F32Consts.INFINITY)).toBe(true);
    });

    it("should return true for -Infinity", () => {
      expect(isF32Infinite(-Infinity)).toBe(true);
      expect(isF32Infinite(F32Consts.NEG_INFINITY)).toBe(true);
    });

    it("should return false for finite numbers", () => {
      expect(isF32Infinite(0)).toBe(false);
      expect(isF32Infinite(1)).toBe(false);
      expect(isF32Infinite(F32Consts.MAX)).toBe(false);
    });

    it("should return false for NaN", () => {
      expect(isF32Infinite(NaN)).toBe(false);
      expect(isF32Infinite(F32Consts.NAN)).toBe(false);
    });
  });

  describe("isF64Infinite", () => {
    it("should return true for Infinity", () => {
      expect(isF64Infinite(Infinity)).toBe(true);
      expect(isF64Infinite(F64Consts.INFINITY)).toBe(true);
    });

    it("should return true for -Infinity", () => {
      expect(isF64Infinite(-Infinity)).toBe(true);
      expect(isF64Infinite(F64Consts.NEG_INFINITY)).toBe(true);
    });

    it("should return false for finite numbers", () => {
      expect(isF64Infinite(0)).toBe(false);
      expect(isF64Infinite(1)).toBe(false);
      expect(isF64Infinite(F64Consts.MAX)).toBe(false);
    });

    it("should return false for NaN", () => {
      expect(isF64Infinite(NaN)).toBe(false);
      expect(isF64Infinite(F64Consts.NAN)).toBe(false);
    });
  });

  describe("edge cases and IEEE 754 behavior", () => {
    it("should handle subnormal numbers as finite", () => {
      const subnormal = Number.MIN_VALUE; // smallest positive subnormal
      expect(isF32Finite(subnormal)).toBe(true);
      expect(isF64Finite(subnormal)).toBe(true);
    });

    it("should handle very large finite numbers", () => {
      expect(isF32Finite(1e38)).toBe(true);
      expect(isF64Finite(1e308)).toBe(true);
    });

    it("should distinguish NaN from Infinity", () => {
      expect(isF32NaN(Infinity)).toBe(false);
      expect(isF32Infinite(NaN)).toBe(false);
      expect(isF64NaN(Infinity)).toBe(false);
      expect(isF64Infinite(NaN)).toBe(false);
    });

    it("should work with negative zero", () => {
      expect(isF32Finite(-0)).toBe(true);
      expect(isF32NaN(-0)).toBe(false);
      expect(isF32Infinite(-0)).toBe(false);
      expect(isF64Finite(-0)).toBe(true);
    });

    it("should handle Number.MAX_VALUE and Number.MIN_VALUE for f64", () => {
      expect(isF64Finite(Number.MAX_VALUE)).toBe(true);
      expect(isF64Finite(Number.MIN_VALUE)).toBe(true);
    });
  });
});