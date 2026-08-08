# Evaluator & Feedback Toolchain Module

## Path
`crate::xiaoyi::evaluator`

## Module Layers
- Layer 0: `evaluator` — Evaluation toolchain layer.
- Layer 1: `sandbox` — Container sandbox for safe execution.
- Layer 2: `build` — Compilation and build verification.
- Layer 3: `test` — Unit, property, and integration testing.
- Layer 4: `analysis` — SAST, AST analysis, DAST, secret scanning.
- Layer 5: `benchmark` — Performance and cost benchmarking.
- Layer 6: `gates` — Quality gates and compliance checking.
- Layer 7: `feedback` — Feedback formulator for retry loop.

## Public Interface
- `Evaluator` — Full evaluation pipeline orchestrator.
- `SandboxResult` — Container sandbox execution outcome.
- `BuildResult` — Compilation and build outcome.
- `TestResult` — Single test execution outcome.
- `TestType` — Type of test executed (Unit, Property, Integration, SAST, DAST).
- `AnalysisFinding` — Finding from static analysis.
- `Severity` — Finding severity classification.
- `BenchmarkResult` — Performance and cost benchmark outcome.
- `TokenUsage` — LLM token consumption.
- `GateStatus` — Quality gate pass/fail status.
- `GateResult` — Single quality gate result.

## Vertical Structure
Each submodule is independent and follows deep vertical layering with strict separation of concerns.

## See Also
- `crate::builder`
- `crate::critic`
- `crate::orchestrator`

## Changelog
- 0.1.0 — Initial implementation.