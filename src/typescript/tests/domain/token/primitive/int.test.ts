import { describe, it, expect } from "vitest";
import {
  IntKind,
  IntWidth,
  IntType,
  SIGNED,
  UNSIGNED,
  W8,
  W16,
  W32,
  W64,
  W128,
  defaultWidth,
  defaultIntType,
  createIntType,
  intByteSize,
  isIntSigned,
  normalizeInt,
  wrapInt,
  convertIntChecked,
  Endianness,
  nativeEndianness,
  DEFAULT_REP,
} from "../../src/xiaoyi/domain/token/primitive/int";

describe("domain/token/primitive/int", () => {
  describe("IntKind enum", () => {
    it("should have Signed and Unsigned values", () => {
      expect(IntKind.Signed).toBe("signed");
      expect(IntKind.Unsigned).toBe("unsigned");
    });

    it("should export SIGNED and UNSIGNED constants", () => {
      expect(SIGNED).toBe(IntKind.Signed);
      expect(UNSIGNED).toBe(IntKind.Unsigned);
    });
  });

  describe("IntWidth enum", () => {
    it("should have all expected widths", () => {
      expect(IntWidth.W8).toBe(8);
      expect(IntWidth.W16).toBe(16);
      expect(IntWidth.W32).toBe(32);
      expect(IntWidth.W64).toBe(64);
      expect(IntWidth.W128).toBe(128);
    });

    it("should export width constants", () => {
      expect(W8).toBe(IntWidth.W8);
      expect(W16).toBe(IntWidth.W16);
      expect(W32).toBe(IntWidth.W32);
      expect(W64).toBe(IntWidth.W64);
      expect(W128).toBe(IntWidth.W128);
    });

    it("should have defaultWidth return W64", () => {
      expect(defaultWidth()).toBe(IntWidth.W64);
    });
  });

  describe("IntType interface", () => {
    it("should have kind and width properties", () => {
      const type: IntType = { kind: IntKind.Signed, width: IntWidth.W32 };

      expect(type.kind).toBe(IntKind.Signed);
      expect(type.width).toBe(IntWidth.W32);
    });
  });

  describe("createIntType", () => {
    it("should create IntType from kind and width", () => {
      const type = createIntType(IntKind.Unsigned, IntWidth.W16);

      expect(type.kind).toBe(IntKind.Unsigned);
      expect(type.width).toBe(IntWidth.W16);
    });
  });

  describe("intByteSize", () => {
    it("should return correct byte size", () => {
      expect(intByteSize({ kind: IntKind.Signed, width: IntWidth.W8 })).toBe(1);
      expect(intByteSize({ kind: IntKind.Unsigned, width: IntWidth.W16 })).toBe(2);
      expect(intByteSize({ kind: IntKind.Signed, width: IntWidth.W32 })).toBe(4);
      expect(intByteSize({ kind: IntKind.Unsigned, width: IntWidth.W64 })).toBe(8);
      expect(intByteSize({ kind: IntKind.Signed, width: IntWidth.W128 })).toBe(16);
    });
  });

  describe("isIntSigned", () => {
    it("should return true for signed types", () => {
      expect(isIntSigned({ kind: IntKind.Signed, width: IntWidth.W32 })).toBe(true);
    });

    it("should return false for unsigned types", () => {
      expect(isIntSigned({ kind: IntKind.Unsigned, width: IntWidth.W32 })).toBe(false);
    });
  });

  describe("defaultIntType", () => {
    it("should return signed 64-bit", () => {
      const type = defaultIntType();

      expect(type.kind).toBe(IntKind.Signed);
      expect(type.width).toBe(IntWidth.W64);
    });
  });

  describe("normalizeInt", () => {
    it("should clamp signed values to range", () => {
      const type: IntType = { kind: IntKind.Signed, width: IntWidth.W8 }; // -128 to 127

      expect(normalizeInt(0, type)).toBe(0);
      expect(normalizeInt(100, type)).toBe(100);
      expect(normalizeInt(200, type)).toBe(127); // clamped to max
      expect(normalizeInt(-200, type)).toBe(-128); // clamped to min
    });

    it("should clamp unsigned values to range", () => {
      const type: IntType = { kind: IntKind.Unsigned, width: IntWidth.W8 }; // 0 to 255

      expect(normalizeInt(0, type)).toBe(0);
      expect(normalizeInt(200, type)).toBe(200);
      expect(normalizeInt(300, type)).toBe(255); // clamped to max
      expect(normalizeInt(-50, type)).toBe(0); // clamped to min
    });

    it("should handle 16-bit ranges", () => {
      const signed16: IntType = { kind: IntKind.Signed, width: IntWidth.W16 }; // -32768 to 32767
      const unsigned16: IntType = { kind: IntKind.Unsigned, width: IntWidth.W16 }; // 0 to 65535

      expect(normalizeInt(40000, signed16)).toBe(32767);
      expect(normalizeInt(70000, unsigned16)).toBe(65535);
    });

    it("should handle 64-bit ranges", () => {
      const signed64: IntType = { kind: IntKind.Signed, width: IntWidth.W64 };
      const max = (1n << 63n) - 1n;
      const min = -(1n << 63n);

      expect(normalizeInt(Number(max) + 100, signed64)).toBe(Number(max));
      expect(normalizeInt(Number(min) - 100, signed64)).toBe(Number(min));
    });
  });

  describe("wrapInt", () => {
    it("should wrap signed values using modulo", () => {
      const type: IntType = { kind: IntKind.Signed, width: IntWidth.W8 }; // -128 to 127

      expect(wrapInt(0, type)).toBe(0);
      expect(wrapInt(127, type)).toBe(127);
      expect(wrapInt(128, type)).toBe(-128); // wraps around
      expect(wrapInt(129, type)).toBe(-127);
      expect(wrapInt(255, type)).toBe(-1);
      expect(wrapInt(256, type)).toBe(0);
    });

    it("should wrap negative signed values", () => {
      const type: IntType = { kind: IntKind.Signed, width: IntWidth.W8 };

      expect(wrapInt(-1, type)).toBe(-1);
      expect(wrapInt(-128, type)).toBe(-128);
      expect(wrapInt(-129, type)).toBe(127); // wraps around
      expect(wrapInt(-130, type)).toBe(126);
    });

    it("should wrap unsigned values using modulo", () => {
      const type: IntType = { kind: IntKind.Unsigned, width: IntWidth.W8 }; // 0 to 255

      expect(wrapInt(0, type)).toBe(0);
      expect(wrapInt(255, type)).toBe(255);
      expect(wrapInt(256, type)).toBe(0);
      expect(wrapInt(257, type)).toBe(1);
      expect(wrapInt(-1, type)).toBe(255); // wraps around
    });

    it("should handle larger widths", () => {
      const type: IntType = { kind: IntKind.Signed, width: IntWidth.W16 };

      expect(wrapInt(32768, type)).toBe(-32768);
      expect(wrapInt(65535, type)).toBe(-1);
      expect(wrapInt(65536, type)).toBe(0);
    });
  });

  describe("convertIntChecked", () => {
    it("should convert within same width", () => {
      const from: IntType = { kind: IntKind.Signed, width: IntWidth.W32 };
      const to: IntType = { kind: IntKind.Signed, width: IntWidth.W32 };

      expect(convertIntChecked(42, from, to)).toBe(42);
    });

    it("should convert signed to unsigned (widening)", () => {
      const from: IntType = { kind: IntKind.Signed, width: IntWidth.W16 };
      const to: IntType = { kind: IntKind.Unsigned, width: IntWidth.W32 };

      expect(convertIntChecked(100, from, to)).toBe(100);
    });

    it("should normalize when narrowing", () => {
      const from: IntType = { kind: IntKind.Signed, width: IntWidth.W32 };
      const to: IntType = { kind: IntKind.Signed, width: IntWidth.W8 };

      expect(convertIntChecked(100, from, to)).toBe(100);
      expect(convertIntChecked(200, from, to)).toBe(127); // clamped
    });

    it("should throw on overflow when from <= to width", () => {
      const from: IntType = { kind: IntKind.Signed, width: IntWidth.W16 };
      const to: IntType = { kind: IntKind.Signed, width: IntWidth.W8 };

      expect(() => convertIntChecked(200, from, to)).toThrow("Integer overflow");
      expect(() => convertIntChecked(-200, from, to)).toThrow("Integer overflow");
    });

    it("should not throw when from > to width and value fits", () => {
      const from: IntType = { kind: IntKind.Signed, width: IntWidth.W32 };
      const to: IntType = { kind: IntKind.Signed, width: IntWidth.W8 };

      expect(convertIntChecked(100, from, to)).toBe(100);
    });
  });

  describe("Endianness enum", () => {
    it("should have Little, Big, and Native values", () => {
      expect(Endianness.Little).toBe("little");
      expect(Endianness.Big).toBe("big");
      expect(Endianness.Native).toBe("native");
    });
  });

  describe("nativeEndianness", () => {
    it("should return Little (TypeScript runs on little-endian)", () => {
      expect(nativeEndianness()).toBe(Endianness.Little);
    });
  });

  describe("DEFAULT_REP", () => {
    it("should be tuple of Signed, W64, Little", () => {
      expect(DEFAULT_REP[0]).toBe(IntKind.Signed);
      expect(DEFAULT_REP[1]).toBe(IntWidth.W64);
      expect(DEFAULT_REP[2]).toBe(Endianness.Little);
    });
  });
});