# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.0.0 (2026-08-08)


### Features

* add comprehensive release automation ([8196633](https://github.com/miruamel/Xiaoyi/commit/8196633f397184e99365b3e29001b798b4f8d309))
* add dev scripts and expand xiaoyi::utils (json, retry, validation) ([af8569a](https://github.com/miruamel/Xiaoyi/commit/af8569a008105e97fd254b066f40a1234cc632ed))
* add example applications for all three languages ([a03cc9d](https://github.com/miruamel/Xiaoyi/commit/a03cc9d6f16ed871c6eead85fdaf9888a444347e))
* add llm provider stubs for deeply vertical client layer ([458fe03](https://github.com/miruamel/Xiaoyi/commit/458fe0377fa117a90230e1099503c8d89a1c899a))
* complete CI/CD infrastructure and Rust test suite ([b0e0645](https://github.com/miruamel/Xiaoyi/commit/b0e0645d72400b1090e991cbe9a3136b6ea53459))
* comprehensive CI/CD, testing, and repo infrastructure ([151ae65](https://github.com/miruamel/Xiaoyi/commit/151ae654d254c191a2c177717580687fc57562b4))
* **critic:** Implement Layer 4 - AI Cascadic Critic Plant ([020eb19](https://github.com/miruamel/Xiaoyi/commit/020eb191be822481f7b82d943efd8dd64743d773))
* deepen cache, vault, gateway, llm, and provider vertical modules ([826c16c](https://github.com/miruamel/Xiaoyi/commit/826c16ceaf7ef28de39900b6c61e8d9efa338c12))
* deepen critic/knowledge vertical modules ([b21a123](https://github.com/miruamel/Xiaoyi/commit/b21a123ce58195424893cffd41580e089d59d83c))
* deepen domain, builder, evaluator, gateway, knowledge, and llm vertical ([2ef518a](https://github.com/miruamel/Xiaoyi/commit/2ef518a477e3d450e7ad9220eea9ce32f9cd98d4))
* deepen evaluator/tracing vertical structure ([a1e97b3](https://github.com/miruamel/Xiaoyi/commit/a1e97b3b3da098973e2de0235bff6449cc96f1a7))
* deepen gateway/llm vertical modules ([096a0c0](https://github.com/miruamel/Xiaoyi/commit/096a0c07bbd1fbb019646ca6b2a126f0cb69265a))
* deepen monitoring, knowledge, and registry submodules ([1efdaf4](https://github.com/miruamel/Xiaoyi/commit/1efdaf4eb09f02978ef636d9e1467b7b43916507))
* deepen monitoring/knowledge vertical modules ([49181e7](https://github.com/miruamel/Xiaoyi/commit/49181e74722154f2eaa2034dfe47d68dd7d908a7))
* deepen monitoring/resilience vertical modules ([0fea76a](https://github.com/miruamel/Xiaoyi/commit/0fea76ae51a3167ff718ed396c8567b7cf927cf0))
* deepen utils, resilience, monitoring, and workflow vertical modules ([24c121e](https://github.com/miruamel/Xiaoyi/commit/24c121e92740bc5fe908e4424c9ca186cb70f036))
* deepen vertical architecture across orchestrator/builder/core/domain ([8e82baa](https://github.com/miruamel/Xiaoyi/commit/8e82baab33683efa8143c7022c6c8582007f9d5f))
* deepen vertical architecture with core/domain submodules ([a031436](https://github.com/miruamel/Xiaoyi/commit/a0314363fb6be57294defa1cf4bed91feebfaaef))
* extend deep vertical branches under core, memory, workflow ([ed4b468](https://github.com/miruamel/Xiaoyi/commit/ed4b4682eddf0ff6a383fb670e756c880ccb7bc3))
* extend gateway, llm, knowledge, resilience deep vertical ([096a6d4](https://github.com/miruamel/Xiaoyi/commit/096a6d42723b1e150e69874732923729e4545015))
* **knowledge:** implement Layer 8 Knowledge module + public API wiring ([ab7357d](https://github.com/miruamel/Xiaoyi/commit/ab7357d3cc8ffb18d8d2456b2a72772594bee4c8))
* **monitoring:** Layer 9 Monitoring + integrate uncommitted layer fixes ([c90237c](https://github.com/miruamel/Xiaoyi/commit/c90237ce324df4c481e3f3ce86d344fccaa10b35))
* **orchestrator:** integrate Layer 9 loop with monitoring/critic/evaluator/resilience ([84007ce](https://github.com/miruamel/Xiaoyi/commit/84007ce35dff8412987fbdd849c6443fdd4e3ea6))
* **utils:** add deeply vertical utils submodules (env, fs, math, net) ([b389a1f](https://github.com/miruamel/Xiaoyi/commit/b389a1fd20b91c01ab03603384417e0148440a7e))
* **utils:** add xiaoyi::utils module with id/time/string helpers ([4403f53](https://github.com/miruamel/Xiaoyi/commit/4403f53f9c2644b2b9d534a81dc3df739ac6354c))


### Bug Fixes

* add missing config source modules and fix .gitignore ([e5613ae](https://github.com/miruamel/Xiaoyi/commit/e5613aeddc9340a6692bea4f90f7b58c475807a8))
* add MIT and Apache-2.0 license files for GitHub detection ([541655f](https://github.com/miruamel/Xiaoyi/commit/541655f5c63458f59338e8151fced4e9e813f1d8))
* align cache module API with python_bindings wrapper ([02f3ebf](https://github.com/miruamel/Xiaoyi/commit/02f3ebf21fa0a5046e9636ffb7f127d677086e98))
* declare dual MIT/Apache-2.0 license in Cargo.toml ([3697626](https://github.com/miruamel/Xiaoyi/commit/36976260ed8f752954affaa3bb6a4f3eabf8b911))
* env source parser - only parse JSON objects/arrays ([930c4c2](https://github.com/miruamel/Xiaoyi/commit/930c4c20eb6a51373987dbbff68a1392f9c88eec))
* executor multiple leaf output & timeout error ([712f12e](https://github.com/miruamel/Xiaoyi/commit/712f12edca9e135d1bc91db2e23fe72c02ef05c5))
* Resolve test compilation errors; fix cache.rs imports ([549642e](https://github.com/miruamel/Xiaoyi/commit/549642e4a6e5aa94e45f6e94beb3f552dcb49f8d))
* restore LICENSE file ([a3d967a](https://github.com/miruamel/Xiaoyi/commit/a3d967aec98a97eaeedd1500de32df42a9ec5aca))
* update package-lock.json & format Rust code ([cae8050](https://github.com/miruamel/Xiaoyi/commit/cae80500c58729712d7619f48b176986c2f7c3e8))
* use full Apache 2.0 license text for GitHub detection ([0f66b58](https://github.com/miruamel/Xiaoyi/commit/0f66b58419b6eadc8a064cd451089de603a0f665))
* use npm install instead of npm ci for TypeScript docs ([43bb673](https://github.com/miruamel/Xiaoyi/commit/43bb673526d92f718a63b8040851d07ef352ea2c))

## [Unreleased]

### Added
- Comprehensive CI/CD pipeline (`.github/workflows/ci.yml`)
- Dependabot configuration for Rust, npm, pip, and GitHub Actions
- Pre-commit hooks for rustfmt, clippy, eslint, ruff, mypy, markdownlint, yamllint
- Test suites for Rust (cargo test), TypeScript (vitest), Python (pytest)
- Issue and PR templates
- Security policy, contributing guide, code of conduct
- pyproject.toml for Python (ruff, mypy, pytest config)
- vitest.config.ts and ESLint config for TypeScript
- clippy.toml for Rust

### Changed
- Migrated to unified `src/{rust,typescript,python}/` layout
- Applied Doxygen Universal documentation standard across all three languages

## [0.1.0] - 2025-08-07

### Added
- Initial release
- Rust core with PyO3 (Python) and napi-rs (Node) bindings
- Layered configuration system (defaults < file < env < vault)
- AES-256-GCM encrypted vault for secrets
- LLM client abstraction (async-openai)
- Workflow DAG executor with cycle detection
- LRU cache with TTL for short-term memory
- Lexer with token classification (primitive + syntax kinds)
- Agent builder, orchestrator, gateway (CLI/API/Web)

[Unreleased]: https://github.com/miruamel/Xiaoyi/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/miruamel/Xiaoyi/releases/tag/v0.1.0
