import { describe, it, expect } from "vitest";
import {
  SyntaxKind,
  Keyword,
  KeywordKind,
  KEYWORDS,
  keywordFromIdent,
  isKeyword,
  Operator,
  OperatorKind,
  Associativity,
  OPERATORS,
  operatorFromSymbol,
  operatorsWithPrefix,
  Delimiter,
  DelimiterKind,
  DELIMITERS,
  matchingClose,
  matchingOpen,
  isOpenDelimiter,
  isCloseDelimiter,
  isDelimiterPair,
  LiteralKind,
  Literal,
  LiteralValue,
  intLiteral,
  floatLiteral,
  stringLiteral,
  boolLiteral,
  parseLiteral,
} from "../../src/xiaoyi/domain/token/syntax";

describe("domain/token/syntax", () => {
  describe("SyntaxKind enum", () => {
    it("should have all expected values", () => {
      expect(SyntaxKind.Keyword).toBe("keyword");
      expect(SyntaxKind.Operator).toBe("operator");
      expect(SyntaxKind.Delimiter).toBe("delimiter");
      expect(SyntaxKind.Literal).toBe("literal");
      expect(SyntaxKind.Identifier).toBe("identifier");
      expect(SyntaxKind.Eof).toBe("eof");
    });
  });

  describe("Keyword module", () => {
    describe("KeywordKind enum", () => {
      it("should have all expected categories", () => {
        expect(KeywordKind.ControlFlow).toBe("control_flow");
        expect(KeywordKind.Declaration).toBe("declaration");
        expect(KeywordKind.Type).toBe("type");
        expect(KeywordKind.Module).toBe("module");
        expect(KeywordKind.Async).toBe("async");
        expect(KeywordKind.ErrorHandling).toBe("error_handling");
      });
    });

    describe("KEYWORDS array", () => {
      it("should contain all control flow keywords", () => {
        const controlFlow = KEYWORDS.filter((k) => k.kind === KeywordKind.ControlFlow);
        const texts = controlFlow.map((k) => k.text).sort();

        expect(texts).toEqual(["break", "continue", "else", "for", "if", "return", "while"]);
      });

      it("should contain all declaration keywords", () => {
        const declaration = KEYWORDS.filter((k) => k.kind === KeywordKind.Declaration);
        const texts = declaration.map((k) => k.text).sort();

        expect(texts).toEqual(["const", "enum", "fn", "let", "struct"]);
      });

      it("should contain all type keywords", () => {
        const type = KEYWORDS.filter((k) => k.kind === KeywordKind.Type);
        const texts = type.map((k) => k.text).sort();

        expect(texts).toEqual(["bool", "float", "int", "string"]);
      });

      it("should contain all module keywords", () => {
        const module = KEYWORDS.filter((k) => k.kind === KeywordKind.Module);
        const texts = module.map((k) => k.text).sort();

        expect(texts).toEqual(["mod", "pub", "use"]);
      });

      it("should contain all async keywords", () => {
        const async = KEYWORDS.filter((k) => k.kind === KeywordKind.Async);
        const texts = async.map((k) => k.text).sort();

        expect(texts).toEqual(["async", "await", "spawn"]);
      });

      it("should contain all error handling keywords", () => {
        const errorHandling = KEYWORDS.filter((k) => k.kind === KeywordKind.ErrorHandling);
        const texts = errorHandling.map((k) => k.text).sort();

        expect(texts).toEqual(["catch", "throw", "try"]);
      });

      it("should have 25 keywords total", () => {
        expect(KEYWORDS).toHaveLength(25);
      });

      it("should be frozen (immutable)", () => {
        expect(() => {
          (KEYWORDS as any).push({ text: "test", kind: KeywordKind.Type });
        }).toThrow();
      });
    });

    describe("keywordFromIdent", () => {
      it("should return keyword for valid identifier", () => {
        const result = keywordFromIdent("if");
        expect(result).toEqual({ text: "if", kind: KeywordKind.ControlFlow });
      });

      it("should return keyword for all keywords", () => {
        for (const kw of KEYWORDS) {
          const result = keywordFromIdent(kw.text);
          expect(result).toEqual(kw);
        }
      });

      it("should return undefined for non-keyword", () => {
        expect(keywordFromIdent("variable")).toBeUndefined();
        expect(keywordFromIdent("myFunction")).toBeUndefined();
        expect(keywordFromIdent("")).toBeUndefined();
      });

      it("should be case-sensitive", () => {
        expect(keywordFromIdent("IF")).toBeUndefined();
        expect(keywordFromIdent("If")).toBeUndefined();
      });
    });

    describe("isKeyword", () => {
      it("should return true for keywords", () => {
        expect(isKeyword("if")).toBe(true);
        expect(isKeyword("while")).toBe(true);
        expect(isKeyword("fn")).toBe(true);
        expect(isKeyword("int")).toBe(true);
      });

      it("should return false for non-keywords", () => {
        expect(isKeyword("variable")).toBe(false);
        expect(isKeyword("myVar")).toBe(false);
        expect(isKeyword("")).toBe(false);
      });
    });
  });

  describe("Operator module", () => {
    describe("OperatorKind enum", () => {
      it("should have expected categories", () => {
        expect(OperatorKind.Arithmetic).toBe("arithmetic");
        expect(OperatorKind.Comparison).toBe("comparison");
        expect(OperatorKind.Logical).toBe("logical");
        expect(OperatorKind.Bitwise).toBe("bitwise");
        expect(OperatorKind.Assignment).toBe("assignment");
        expect(OperatorKind.Other).toBe("other");
      });
    });

    describe("Associativity enum", () => {
      it("should have expected values", () => {
        expect(Associativity.Left).toBe("left");
        expect(Associativity.Right).toBe("right");
        expect(Associativity.None).toBe("none");
      });
    });

    describe("OPERATORS array", () => {
      it("should contain arithmetic operators", () => {
        const arithmetic = OPERATORS.filter((o) => o.kind === OperatorKind.Arithmetic);
        const symbols = arithmetic.map((o) => o.symbol).sort();

        expect(symbols).toContain("+");
        expect(symbols).toContain("-");
        expect(symbols).toContain("*");
        expect(symbols).toContain("/");
        expect(symbols).toContain("%");
      });

      it("should contain comparison operators", () => {
        const comparison = OPERATORS.filter((o) => o.kind === OperatorKind.Comparison);
        const symbols = comparison.map((o) => o.symbol).sort();

        expect(symbols).toContain("==");
        expect(symbols).toContain("!=");
        expect(symbols).toContain("<");
        expect(symbols).toContain("<=");
        expect(symbols).toContain(">");
        expect(symbols).toContain(">=");
      });

      it("should contain logical operators", () => {
        const logical = OPERATORS.filter((o) => o.kind === OperatorKind.Logical);
        const symbols = logical.map((o) => o.symbol).sort();

        expect(symbols).toContain("&&");
        expect(symbols).toContain("||");
        expect(symbols).toContain("!");
      });

      it("should contain assignment operators", () => {
        const assignment = OPERATORS.filter((o) => o.kind === OperatorKind.Assignment);
        const symbols = assignment.map((o) => o.symbol).sort();

        expect(symbols).toContain("=");
        expect(symbols).toContain("+=");
        expect(symbols).toContain("-=");
      });

      it("should be frozen (immutable)", () => {
        expect(() => {
          (OPERATORS as any).push({ symbol: "??", kind: OperatorKind.Other, precedence: 1, associativity: Associativity.None });
        }).toThrow();
      });
    });

    describe("operatorFromSymbol", () => {
      it("should find operator by exact symbol", () => {
        const plus = operatorFromSymbol("+");
        expect(plus).toBeDefined();
        expect(plus?.symbol).toBe("+");
        expect(plus?.kind).toBe(OperatorKind.Arithmetic);
      });

      it("should find multi-character operators", () => {
        const eq = operatorFromSymbol("==");
        expect(eq).toBeDefined();
        expect(eq?.symbol).toBe("==");
        expect(eq?.kind).toBe(OperatorKind.Comparison);
      });

      it("should return undefined for unknown symbol", () => {
        expect(operatorFromSymbol("???")).toBeUndefined();
        expect(operatorFromSymbol("")).toBeUndefined();
      });
    });

    describe("operatorsWithPrefix", () => {
      it("should return operators starting with prefix", () => {
        const eqOps = operatorsWithPrefix("=");
        const symbols = eqOps.map((o) => o.symbol).sort();

        expect(symbols).toContain("=");
        expect(symbols).toContain("==");
        expect(symbols).toContain("!=");
        expect(symbols).toContain("+=");
        expect(symbols).toContain("-=");
      });

      it("should return empty array for non-matching prefix", () => {
        expect(operatorsWithPrefix("@@")).toHaveLength(0);
      });
    });
  });

  describe("Delimiter module", () => {
    describe("DelimiterKind enum", () => {
      it("should have expected categories", () => {
        expect(DelimiterKind.Parenthesis).toBe("parenthesis");
        expect(DelimiterKind.Brace).toBe("brace");
        expect(DelimiterKind.Bracket).toBe("bracket");
        expect(DelimiterKind.Angle).toBe("angle");
      });
    });

    describe("DELIMITERS array", () => {
      it("should contain standard delimiters", () => {
        const pairs = DELIMITERS.map((d) => ({ open: d.open, close: d.close }));

        expect(pairs).toContainEqual({ open: "(", close: ")" });
        expect(pairs).toContainEqual({ open: "{", close: "}" });
        expect(pairs).toContainEqual({ open: "[", close: "]" });
        expect(pairs).toContainEqual({ open: "<", close: ">" });
      });

      it("should be frozen (immutable)", () => {
        expect(() => {
          (DELIMITERS as any).push({ open: "|", close: "|", kind: DelimiterKind.Other });
        }).toThrow();
      });
    });

    describe("matchingClose", () => {
      it("should return matching close for open delimiter", () => {
        expect(matchingClose("(")).toBe(")");
        expect(matchingClose("{")).toBe("}");
        expect(matchingClose("[")).toBe("]");
        expect(matchingClose("<")).toBe(">");
      });

      it("should return undefined for non-delimiter", () => {
        expect(matchingClose("x")).toBeUndefined();
        expect(matchingClose("")).toBeUndefined();
      });
    });

    describe("matchingOpen", () => {
      it("should return matching open for close delimiter", () => {
        expect(matchingOpen(")")).toBe("(");
        expect(matchingOpen("}")).toBe("{");
        expect(matchingOpen("]")).toBe("[");
        expect(matchingOpen(">")).toBe("<");
      });

      it("should return undefined for non-delimiter", () => {
        expect(matchingOpen("x")).toBeUndefined();
      });
    });

    describe("isOpenDelimiter", () => {
      it("should return true for open delimiters", () => {
        expect(isOpenDelimiter("(")).toBe(true);
        expect(isOpenDelimiter("{")).toBe(true);
        expect(isOpenDelimiter("[")).toBe(true);
        expect(isOpenDelimiter("<")).toBe(true);
      });

      it("should return false for close delimiters and others", () => {
        expect(isOpenDelimiter(")")).toBe(false);
        expect(isOpenDelimiter("}")).toBe(false);
        expect(isOpenDelimiter("x")).toBe(false);
      });
    });

    describe("isCloseDelimiter", () => {
      it("should return true for close delimiters", () => {
        expect(isCloseDelimiter(")")).toBe(true);
        expect(isCloseDelimiter("}")).toBe(true);
        expect(isCloseDelimiter("]")).toBe(true);
        expect(isCloseDelimiter(">")).toBe(true);
      });

      it("should return false for open delimiters and others", () => {
        expect(isCloseDelimiter("(")).toBe(false);
        expect(isCloseDelimiter("{")).toBe(false);
        expect(isCloseDelimiter("x")).toBe(false);
      });
    });

    describe("isDelimiterPair", () => {
      it("should return true for valid pairs", () => {
        expect(isDelimiterPair("(", ")")).toBe(true);
        expect(isDelimiterPair("{", "}")).toBe(true);
        expect(isDelimiterPair("[", "]")).toBe(true);
        expect(isDelimiterPair("<", ">")).toBe(true);
      });

      it("should return false for invalid pairs", () => {
        expect(isDelimiterPair("(", "]")).toBe(false);
        expect(isDelimiterPair("{", ")")).toBe(false);
        expect(isDelimiterPair("x", "y")).toBe(false);
      });
    });
  });

  describe("Literal module", () => {
    describe("LiteralKind enum", () => {
      it("should have expected values", () => {
        expect(LiteralKind.Integer).toBe("integer");
        expect(LiteralKind.Float).toBe("float");
        expect(LiteralKind.String).toBe("string");
        expect(LiteralKind.Boolean).toBe("boolean");
      });
    });

    describe("Literal interface", () => {
      it("should have kind, text, and value", () => {
        const lit: Literal = { kind: LiteralKind.Integer, text: "42", value: 42 };
        expect(lit.kind).toBe(LiteralKind.Integer);
        expect(lit.text).toBe("42");
        expect(lit.value).toBe(42);
      });
    });

    describe("LiteralValue type", () => {
      it("should accept number", () => {
        const val: LiteralValue = 42;
        expect(val).toBe(42);
      });

      it("should accept string", () => {
        const val: LiteralValue = "hello";
        expect(val).toBe("hello");
      });

      it("should accept boolean", () => {
        const val: LiteralValue = true;
        expect(val).toBe(true);
      });
    });

    describe("intLiteral", () => {
      it("should create integer literal", () => {
        const lit = intLiteral("42", 42);
        expect(lit).toEqual({ kind: LiteralKind.Integer, text: "42", value: 42 });
      });

      it("should handle negative integers", () => {
        const lit = intLiteral("-10", -10);
        expect(lit.value).toBe(-10);
      });
    });

    describe("floatLiteral", () => {
      it("should create float literal", () => {
        const lit = floatLiteral("3.14", 3.14);
        expect(lit).toEqual({ kind: LiteralKind.Float, text: "3.14", value: 3.14 });
      });
    });

    describe("stringLiteral", () => {
      it("should create string literal", () => {
        const lit = stringLiteral('"hello"', "hello");
        expect(lit).toEqual({ kind: LiteralKind.String, text: '"hello"', value: "hello" });
      });
    });

    describe("boolLiteral", () => {
      it("should create boolean literal", () => {
        const lit = boolLiteral("true", true);
        expect(lit).toEqual({ kind: LiteralKind.Boolean, text: "true", value: true });
      });
    });

    describe("parseLiteral", () => {
      it("should parse integer literals", () => {
        const lit = parseLiteral("42");
        expect(lit).toEqual({ kind: LiteralKind.Integer, text: "42", value: 42 });
      });

      it("should parse negative integers", () => {
        const lit = parseLiteral("-10");
        expect(lit).toEqual({ kind: LiteralKind.Integer, text: "-10", value: -10 });
      });

      it("should parse float literals", () => {
        const lit = parseLiteral("3.14");
        expect(lit).toEqual({ kind: LiteralKind.Float, text: "3.14", value: 3.14 });
      });

      it("should parse scientific notation", () => {
        const lit = parseLiteral("1e10");
        expect(lit).toEqual({ kind: LiteralKind.Float, text: "1e10", value: 1e10 });
      });

      it("should parse string literals", () => {
        const lit = parseLiteral('"hello"');
        expect(lit).toEqual({ kind: LiteralKind.String, text: '"hello"', value: "hello" });
      });

      it("should parse boolean literals", () => {
        expect(parseLiteral("true")).toEqual({ kind: LiteralKind.Boolean, text: "true", value: true });
        expect(parseLiteral("false")).toEqual({ kind: LiteralKind.Boolean, text: "false", value: false });
      });

      it("should return undefined for unrecognized input", () => {
        expect(parseLiteral("identifier")).toBeUndefined();
        expect(parseLiteral("")).toBeUndefined();
      });
    });
  });
});