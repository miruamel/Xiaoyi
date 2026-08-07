# Xiaoyi Autonomous Vertical Architecture

**Unified concept merging: Autonomous Agent Loop + Deep Vertical Layers**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         XIAOYI AUTONOMOUS VERTICAL AGENT                     │
│                    "Autonomous Software Engineer"                            │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 1: USER INTERFACE GATEWAY / 用户接口网关                              │
│  ─────────────────────────────────────────────────────────────────────────  │
│  CLI • Web • API  →  HTTPS  →  Session & State  →  Rate Limit & Quota       │
│  Cost Estimator  →  Token Auth  →  RBAC  →  Request Validation              │
│  Input Validation  →  Sanitizer & PII Redactor  →  Intent Router            │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 2: ORCHESTRATOR (AUTONOMOUS LOOP) / 编排调度                          │
│  ─────────────────────────────────────────────────────────────────────────  │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  AUTONOMOUS AGENT LOOP (Non-HITL)                                    │   │
│  │  ─────────────────────────────────────────────────────────────────  │   │
│  │  Task Decomposer  →  Semantic Cache (Vector)  →  DAG Builder        │   │
│  │       │                                                                 │   │
│  │       ▼                                                                │   │
│  │  Cycle & Conflict Detector  ──(cycle?)──→  DAG Rebuild                │   │
│  │       │                                                                 │   │
│  │       ▼                                                                │   │
│  │  Dynamic Budget & Token Allocator  →  Parallel Execution Engine       │   │
│  │       │                        (Async State Store / Redis)            │   │
│  │       ▼                                                                │   │
│  │  ─────────────────────────────────────────────────────────────────    │   │
│  │  ERROR CONTEXT FORMULATOR → RETRY / REDESIGN (auto)                    │   │
│  │  ─────────────────────────────────────────────────────────────────    │   │
│  │  NO HITL GATE • NO MANUAL APPROVAL • AUTO RECOVERY                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 3: BUILDER AGENT / 构建网关                                           │
│  ─────────────────────────────────────────────────────────────────────────  │
│  Input Sanitizer (AST)  →  PII Redaction (Diff-aware)  →  AST Sifter       │
│  XSS Filter  →  Prompt Cache Manager (Prefix KV)  →  Context Window Mgr     │
│  Multi-File Spec Planner  →  Code Generator & AST Patcher                   │
│  Tool Call Validator (Anti-Injection)  →  Self-Verification & Syntax Check  │
│  Code Formatter & Dependency Mapper                                          │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 4: AI CASCADIC CRITIC PLANT / 级联评审网关                            │
│  ─────────────────────────────────────────────────────────────────────────  │
│  ⚡ Fast-Path Rules Engine (Linters/Regex)                                  │
│       │                                                                      │
│       ▼                                                                      │
│  ⏳ Small-LLM Critics (Style, Doc, Accessibility)                           │
│       │                                                                      │
│       ▼                                                                      │
│  ⚙️ Stage 2 Large-LLM Critics (Security, Architecture, Complex Logic)       │
│       │                                                                      │
│       ▼                                                                      │
│  Model Router: Heavy (GPT-4/Claude) │ Light (Llama) │ Micro (Phi-3)        │
│       │                                                                      │
│       ▼                                                                      │
│  Meta-Critic Aggregator (Weighted Pareto Frontier) → JSON Extractor         │
│       │                                                                      │
│       ▼                                                                      │
│  Semantic Cache with Vector DB + Embedding Similarity                       │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 5: EVALUATOR & FEEDBACK TOOLCHAIN / 评估工具链                        │
│  ─────────────────────────────────────────────────────────────────────────  │
│  Container Sandbox  →  Compilation/Build  →  Unit & Property Testing        │
│  Integration Testing (SAST + AST + Rule Engine)  →  DAST & Secret Scanning  │
│  Perf & Cost Benchmarking  →  Quality Gates & Compliance                    │
│  Feedback Formulator (Creates Precise Error Prompts for Retry Loop)         │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    ▼                               ▼
┌─────────────────────────────────┐ ┌─────────────────────────────────┐
│  LAYER 6A: DEPLOYMENT           │ │  LAYER 6B: GIT CHECKPOINT       │
│  / 部署集成                      │ │  / Git 检查点                    │
│  ─────────────────────────────  │ │  ─────────────────────────────  │
│  Git PR/MR Generator            │ │  Auto-commit on Stable          │
│  CI/CD Pipeline Trigger         │ │  Tag: xiaoyi/stable/<timestamp> │
│  Webhook & Slack Notifier       │ │  Rollback: Auto-revert on Fail  │
│  Alert System                   │ │  Feature Branch Expansion       │
└─────────────────────────────────┘ └─────────────────────────────────┘
                    │                               │
                    └───────────────┬───────────────┘
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 7: RESILIENCE & FALLBACK / 容错模式 (Cross-Cutting)                  │
│  ─────────────────────────────────────────────────────────────────────────  │
│  Circuit Breaker  →  Fallback Model Routing (Heavy→Light)                   │
│  Exponential Backoff Retry  →  Dead Letter Queue                            │
│  Error Classification  →  Recovery Strategies                               │
│  Graceful Degradation Engine                                                │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 8: KNOWLEDGE & TOOLS MODULE / 知识库                                 │
│  ─────────────────────────────────────────────────────────────────────────  │
│  🗃️ Vector DB (RAG Codebase)   🐙 Git-Native Repo & AST Graph               │
│  🔌 API & Tool Plugin Registry   📜 OpenAPI & Schema Store                  │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER 9: MONITORING & FINOPS LAYER / 监控层 (Cross-Cutting)                │
│  ─────────────────────────────────────────────────────────────────────────  │
│  Distributed Tracing (OpenTelemetry)  •  Token & Cost FinOps Tracker        │
│  LLM Quality Metrics (Pass@k, Hallucination Rate)  •  Real-time Alerting    │
│  Optimization Suggestions                                                 │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  TOOLS RUNTIME (Available to All Layers)                                    │
│  ─────────────────────────────────────────────────────────────────────────  │
│  Bash  •  Edit  •  Read  •  Write  •  Glob  •  Grep  •  WebFetch           │
│  WebSearch  •  Summarize  •  Skill  •  Task  •  Agent                      │
│  AskUserQuestion: DISABLED (Non-HITL)                                      │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  STABILITY CRITERIA (Definition of Stable)                                  │
│  ─────────────────────────────────────────────────────────────────────────  │
│  ✓ Build passes          ✓ Tests pass          ✓ App starts                │
│  ✓ No critical errors    ✓ Git checkpoint saved  ✓ Task marked stable     │
└─────────────────────────────────────────────────────────────────────────────┘

Network: Unrestricted but observable
