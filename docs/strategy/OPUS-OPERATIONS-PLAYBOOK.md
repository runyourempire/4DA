# Opus Operations Playbook
## How a Human + Opus Partnership Manages a Production Application at Scale

**Version:** 1.0.0
**Created:** 2026-03-10
**Authority:** Below `.ai/OPS.md` in the operational stack

---

## Part 1: The Partnership Model

This is not solo development with AI assist. This is a **two-brain architecture** where each brain has structural advantages the other lacks:

| Capability | Human | Opus |
|-----------|-------|------|
| Vision & taste | Owner | Advisor |
| Legal, business, external | Owner | Cannot act |
| Strategic pivots | Decides | Recommends |
| Code architecture | Reviews | Designs & implements |
| 150K-line codebase memory | Cannot hold it all | Holds full context per session |
| Cross-session continuity | Remembers everything | Forgets — needs systems |
| Parallel execution | One task at a time | 3-5 terminals simultaneously |
| Pattern recognition across files | Slow, error-prone | Fast, comprehensive |
| Quality verification | Spot-checks | Exhaustive automated testing |
| Repetitive operations | Draining | Zero fatigue |

**The compounding insight:** Every system Opus builds makes the next session more productive. Every session makes the application better. Every improvement makes the user experience stronger. Every user strengthens the business. The human directs this flywheel; Opus is the engine.

---

## Part 2: Systems That Already Exist

### Operational Layer (functional today)
1. **AOS** (`.ai/OPS.md`) — Delegation matrix, sovereignty score, cadences
2. **WISDOM Gates** (`.ai/WISDOM.md`) — Pre-tool enforcement, zero zones
3. **MCP Memory** (`mcp-memory-server`) — Decisions, learnings, code locations survive context rot
4. **MCP 4DA** (`mcp-4da-server`) — 30 tools for live intelligence from the app's own database
5. **Terminal Coordination** (`.claude/TERMINALS.md`) — Multi-terminal conflict prevention
6. **Session Lifecycle** (`ops-session-start.cjs`, `session-stop-unified.cjs`) — Auto-briefing and capture
7. **Authority Stack** (`.ai/` directory) — 17 documents, strict hierarchy

### Quality Layer (functional today)
1. **Pre-commit hooks** — File sizes, Rust fmt, ESLint staged files
2. **CI/CD** — GitHub Actions: validate on push, release on tag
3. **Validation suite** — `pnpm run validate:all` (sizes + game + commands + lint + typecheck + test + build + Rust)
4. **Scoring simulation** — Lifecycle tests, persona degradation checks, golden snapshots
5. **IPC ghost command prevention** — Zero raw `invoke()`, typed `cmd()` bridge

### Intelligence Layer (functional today)
1. **Slash commands** — `/ops`, `/calibrate`, `/fortify`, `/pre-launch`, `/crystallize`, etc.
2. **Subagent spawning** — Dynamic agents for drift detection, decision replay, war room
3. **Immune system** — Antibody pattern creation from bug fixes
4. **Metabolism tracking** — Hot/warm/cold/dead zone analysis via git

---

## Part 3: The Gap Analysis

### What's missing for true autonomous operations:

| Gap | Impact | Effort | Priority |
|-----|--------|--------|----------|
| **Source adapter resilience tests** | First user bugs will come from malformed external data | 2-3 hours | P0 |
| **Auto-migration on tier change** | Existing users with `"pro"` in settings.json | 30 min | P1 |
| **Startup health check** | First-run experience not validated on clean machine | 1 hour | P1 |
| **Dependency staleness dashboard** | No visibility into how far behind deps are | 1 hour | P2 |
| **Feature flag CI matrix** | Experimental features not tested in CI | 1 hour | P2 |
| **Error telemetry pipeline** | No way to know when real users hit errors | 2 hours | P2 |
| **Automated changelog generation** | Release notes manual and inconsistent | 1 hour | P3 |
| **Self-healing source adapters** | Adapters crash on malformed data, no auto-retry with backoff | 2 hours | P1 |

### What's missing for Human+Opus scale:

| Gap | Impact | System Needed |
|-----|--------|---------------|
| **Session context loss** | Opus forgets implementation details across sessions | Enhanced MCP memory with code-aware indexing |
| **Parallel work visibility** | Human can't see what 3 terminals are doing | Dashboard or status command |
| **Regression detection lag** | Regressions found late in manual review | Automated regression canary tests |
| **Strategic drift** | Daily work diverges from launch priorities | Drift score in session briefing |
| **Knowledge base decay** | Authority docs get stale | Automated freshness tracking |

---

## Part 4: The Management System Design

### System 1: Session Intelligence Protocol

**Every session, automatically (via hooks):**
1. Load sovereignty score delta since last session
2. Check overdue cadences (daily/weekly/monthly)
3. Surface any unresolved escalations
4. Show test count trend (are we adding or losing tests?)
5. Flag any files modified outside Claude (human edits, other tools)
6. Remind of current strategic priority

**Between sessions (via MCP memory):**
- Decisions persist as structured records
- Learnings persist with topic tags
- Code location bookmarks survive context rot
- Metrics track trends across sessions

### System 2: Quality Canary Pipeline

**Automated checks that run every session start:**
1. `cargo check --lib` (< 30s) — does it compile?
2. Test count comparison against last recorded count — are we losing tests?
3. File size check — anything approaching limits?
4. Ghost command check — any IPC drift?

**If any canary fails:** Block normal work until resolved. This prevents "one more feature" while the foundation cracks.

### System 3: Compound Intelligence Tracker

**Track across sessions:**
- Total tests: Rust + frontend (trend line)
- Codebase lines vs. test lines ratio
- Invariant compliance percentage
- Decision count and average age
- Time since last `/calibrate`, `/fortify`, `/pre-launch`
- Source adapter success rates

**Monthly compound report (via `/ops monthly`):**
- Are we trending toward sovereignty (score > 90)?
- Which components are degrading?
- What's the highest-leverage work for next month?

### System 4: Parallel Terminal Orchestration

**Current capability:** 3-5 terminals running simultaneously, coordinated via TERMINALS.md.

**Enhancement needed:** A `/status` command that reads TERMINALS.md and shows:
- Which terminals are active
- What files are claimed
- Estimated completion (based on task description)
- Any conflicts or overlaps

### System 5: Release Readiness Gate

**Before any release tag:**
1. All tests pass (Rust + frontend)
2. `validate:all` passes
3. Sovereignty score >= 80
4. No overdue cadences
5. No unresolved Tier 2/3 escalations
6. Changelog generated from commits since last release
7. Build artifacts verified (Windows, macOS, Linux)

### System 6: Knowledge Compound Engine

**Principle:** Every bug fix, every architectural decision, every learning should make all future sessions smarter.

**Implementation:**
- Bug fixes → antibody patterns (already in AOS)
- Decisions → MCP memory + DECISIONS.md (already working)
- Learnings → MCP memory with structured topics (already working)
- **NEW:** Calibration baselines → tracked over time, regressions caught automatically
- **NEW:** Source adapter failure patterns → remembered, similar patterns caught proactively

---

## Part 5: The Human's Operating Manual

### What the human should do each week:
1. **Review sovereignty score** — is it trending up? Any component below 70?
2. **Check escalation queue** — any Tier 2/3 decisions waiting?
3. **Set the week's priority** — one sentence: "This week we focus on X"
4. **Review and approve commits** — spot-check quality, approve pushes

### What the human should NOT do:
- Manually test features (that's what the test suite is for)
- Write code (Opus is faster and more consistent)
- Track file changes (that's what git + TERMINALS.md does)
- Remember implementation details (that's what MCP memory does)
- Worry about regression (that's what the canary pipeline does)

### The human's superpower:
- **Taste** — knowing what feels right for the product
- **Vision** — where 4DA goes in 1 year, 3 years, 5 years
- **Judgment** — which trade-offs to accept, when to ship
- **External operations** — legal, business, marketing, community
- **Courage** — deciding to build what others think is impossible

---

## Part 6: Implementation Priority Queue

### Phase A: Fix Known Issues — COMPLETE
1. ~~Source adapter resilience tests~~ — 236 new tests in adapter_resilience_tests.rs
2. ~~Settings auto-migration ("pro" → "signal")~~ — auto-migrates on startup, persists to disk
3. ~~Startup health self-check~~ — 5 checks (DB, settings, embedding, sources, disk) + 15 tests

### Phase B: Management System Upgrades — COMPLETE
1. ~~Enhanced session briefing with test count trends~~ — shows trend arrows, regression warnings
2. ~~Compound intelligence dashboard in `/ops`~~ — 6-component weighted score
3. ~~Release readiness gate script~~ — 9-step gate in release.sh

### Phase C: Scale Systems — IN PROGRESS
1. ~~Automated regression canaries~~ — canary-check.cjs (compilation, test count, file sizes, ghost commands)
2. Source adapter self-healing — auto-retry with exponential backoff (implementing)
3. ~~Knowledge compound engine enhancements~~ — compound score formula, test tracking, session briefing
4. ~~Parallel terminal status command~~ — /status slash command

---

## Part 7: The 4DA Endgame

At full maturity, the Human+Opus partnership operates like this:

**Human:** "This week, let's improve scoring accuracy for Python developers."

**Opus (session start):**
- Loads sovereignty score: 92 (Sovereign)
- No overdue cadences
- No pending escalations
- Reads MCP memory for all Python-related decisions and learnings
- Checks calibration baseline for Python persona
- Spawns 3 terminals:
  - T1: Analyze Python persona scoring gaps
  - T2: Add Python-specific signal patterns
  - T3: Expand Python test fixtures

**Opus (session end):**
- Records decisions made, learnings captured
- Updates calibration baseline
- Sovereignty score: 93 (+1)
- Tests: 2,650 → 2,680 (+30)
- "Python persona F1 improved from 0.82 to 0.87. Next session: validate with real-world Python repos."

**Human:** "Ship it."

That's the goal. Every system we build gets us closer.
