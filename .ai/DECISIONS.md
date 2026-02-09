# Architectural Decisions Log
## Engineering Memory for 4DA v3

**Version:** 1.0.0
**Source:** Migrated from .claude/rules/decisions.md + CADE additions
**Purpose:** Prevent re-litigation of settled decisions

---

## How to Use This File

1. **Before proposing changes:** Check if a relevant decision exists
2. **When making new decisions:** Add to this file immediately
3. **When challenging decisions:** Note alternatives in "Considered" section

---

## Core Architecture

### AD-001: Tauri 2.0 over Electron
- **Decision:** Use Tauri 2.0 (Rust + WebView) instead of Electron
- **Rationale:** 10x smaller binary, 5x faster startup, native Rust performance for indexing
- **Considered:**
  - Electron: Rejected - too heavy for an ambient background tool
  - Flutter: Rejected - less mature desktop support, Dart learning curve
- **Date:** Project inception
- **Status:** Final

### AD-002: SQLite + sqlite-vss for Vector Storage
- **Decision:** Use SQLite with sqlite-vss extension for embeddings
- **Rationale:** No external database needed, single file, portable, sufficient for local-first app
- **Considered:**
  - Pinecone/Weaviate: Rejected - violates local-first principle, adds complexity
  - PostgreSQL + pgvector: Rejected - too heavy for desktop app
  - Qdrant: Rejected - external dependency for local-first app
- **Date:** Project inception
- **Status:** Final

### AD-003: BYOK (Bring Your Own Key) Model
- **Decision:** Users provide their own API keys, never stored remotely
- **Rationale:** Privacy-first principle, no server costs, user controls their data
- **Considered:**
  - Server-side API proxy: Rejected - privacy violation, liability
  - Free tier: Rejected - unsustainable, creates wrong incentives
- **Date:** Project inception
- **Status:** Final

---

## Embedding Strategy

### AD-004: Embedding Model Selection
- **Decision:** Use fastembed with MiniLM-L6-v2 (384 dimensions) for local embeddings
- **Rationale:**
  - Runs locally without API calls
  - Deterministic results
  - Sufficient quality for similarity search
  - Fast inference on CPU
- **Considered:**
  - OpenAI text-embedding-3-small: Good but requires API, costs money
  - Ollama embeddings: Viable fallback but slower
- **Date:** Phase 0 implementation
- **Status:** Final for Phase 0, may revisit for v2

---

## Frontend Architecture

### AD-005: React 18 + TypeScript + Tailwind
- **Decision:** Standard modern web stack
- **Rationale:** Developer familiarity, excellent tooling, Tailwind for rapid UI
- **Considered:**
  - Vue: Rejected - smaller ecosystem
  - Svelte: Rejected - less mature Tauri integration
  - Solid: Rejected - smaller community
- **Date:** Project inception
- **Status:** Final

---

## Design System

### AD-006: Matte Black Minimalism
- **Decision:** Dark theme (#0A0A0A base), minimal chrome, gold accent sparingly
- **Rationale:** Ambient tool should be visually quiet, not attention-seeking
- **Considered:**
  - Light theme: Rejected - most developers prefer dark
  - Colorful UI: Rejected - too attention-seeking for ambient tool
- **Date:** Project inception
- **Status:** Final

---

## CADE Decisions

### AD-007: Cognition Artifacts in .ai/
- **Decision:** Create dedicated `.ai/` directory for cognition artifacts separate from `.claude/`
- **Rationale:**
  - `.claude/` is for runtime state and hooks
  - `.ai/` is for truth-source documents that define agent behavior
  - Clear separation of concerns
  - `.ai/` contents are stable, `.claude/` contents are dynamic
- **Considered:**
  - Merge with .claude/: Rejected - conflates runtime and truth-source
  - Use root-level files: Rejected - clutters project root
- **Date:** CADE implementation
- **Status:** Final

### AD-008: Two-Phase Protocol Enforcement
- **Decision:** Require explicit Phase 1 (Orientation) before Phase 2 (Execution)
- **Rationale:**
  - Prevents premature coding
  - Ensures shared understanding before work begins
  - Reduces rework from misunderstood requirements
- **Date:** CADE implementation
- **Status:** Final

### AD-009: CI as Validation Authority
- **Decision:** GitHub Actions CI serves as the validation authority (not the agent)
- **Rationale:**
  - Agents cannot self-certify correctness
  - Machine verification prevents fabricated claims
  - Audit trail via CI logs
- **Considered:**
  - Agent self-validation: Rejected - agents can fabricate confidence
  - Manual review only: Rejected - not scalable
- **Date:** CADE implementation
- **Status:** Final

### AD-010: Warnings-First CI Rollout
- **Decision:** Start CI gates in warnings mode (continue-on-error: true)
- **Rationale:**
  - Allows baseline establishment
  - Prevents productivity loss during tuning
  - Graduate to blocking after patterns understood
- **Date:** CADE implementation
- **Status:** Active - will transition to blocking mode

### AD-011: Frontend Test Infrastructure First
- **Decision:** Set up Vitest infrastructure without writing extensive tests initially
- **Rationale:**
  - Gets gates in place
  - Allows incremental test addition
  - Doesn't derail main CADE implementation
- **Date:** CADE implementation
- **Status:** Final

---

## Void Engine

### AD-012: Void Engine - Heartbeat is Production, Universe is Experimental
- **Decision:** The Void Engine heartbeat (Phase 1: ambient signal indicator) is a production feature. The 3D universe (Phase 2: Three.js spatial visualization) is classified as experimental and should not receive further investment until the core product loop (signals, briefings, feedback) is mature.
- **Rationale:**
  - The heartbeat communicates real system state (scanning, idle, stale, error, discoveries) through a 48px ambient glow. It fits the "quiet ambient tool" design philosophy perfectly.
  - The 3D universe contradicts 4DA's core value proposition. 4DA delivers what matters to you - it doesn't ask you to explore a cloud of dots. The universe is discovery mode; 4DA is delivery mode.
  - Johnson-Lindenstrauss random projection from 384-dim to 3D doesn't produce human-interpretable clusters. Users see dots but can't form a mental model.
  - Three.js bundle (~908KB) is 2.5x the rest of the app (353KB) for a rarely-used feature.
  - Particle relevance scores were never populated (all 0.0), making the visualization purely spatial with no relevance dimension.
  - The `void_get_neighbors` command rebuilds the entire universe per query instead of using cached data or the existing sqlite-vec KNN.
- **Considered:**
  - Remove universe entirely: Rejected - the code is clean, well-tested, and costs nothing when not loaded (React.lazy code-split). Keeping it as opt-in experimental preserves optionality.
  - Invest in fixing universe (relevance scores, 2D mode, caching): Deferred - signals, briefings, and feedback loop have higher ROI for user value.
  - Make universe the primary view: Rejected - antithetical to 4DA's ambient delivery philosophy.
- **Date:** 2026-02-09
- **Status:** Final

### AD-013: Void Engine Signal Architecture
- **Decision:** Void signals are change-driven (emit only when values differ), not timer-driven. Frontend interpolates locally at 30fps.
- **Rationale:**
  - Zero CPU cost when nothing changes (most of the time for an ambient tool)
  - Signal emissions are hooked into real backend events (fetch start, analysis complete, error, ACE scan)
  - Frontend RAF loop with cancelled flag prevents memory leaks on unmount
- **Date:** 2026-02-09
- **Status:** Final

---

## Rejected Alternatives (Reference)

| ID | Alternative | Reason for Rejection |
|----|-------------|---------------------|
| REJ-001 | Electron | Too heavy for ambient background tool |
| REJ-002 | External Vector DB | Violates local-first principle |
| REJ-003 | Server-side API keys | Privacy violation, liability |
| REJ-004 | Agent self-certification | Agents can fabricate confidence |
| REJ-005 | Light theme | Most developers prefer dark |
| REJ-006 | 3D universe as primary view | Contradicts ambient delivery philosophy; discovery mode vs delivery mode |
| REJ-007 | Remove universe code entirely | Code is clean, code-split, costs nothing when unused; preserves optionality |

---

## Pending Decisions

*Decisions under active consideration*

| ID | Topic | Options | Status |
|----|-------|---------|--------|
| - | (None currently) | - | - |

---

## Decision Template

When adding a new decision:

```markdown
### AD-NNN: [Short Title]
- **Decision:** [What was decided]
- **Rationale:** [Why this choice was made]
- **Considered:**
  - [Alternative 1]: [Why rejected]
  - [Alternative 2]: [Why rejected]
- **Date:** [When decided]
- **Status:** [Final/Active/Superseded]
```

---

*Decisions are made once and referenced often. Re-litigation requires new evidence.*
