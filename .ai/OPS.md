# AOS — Autonomous Operations System

## The Operational Bible for 4DA

**Version:** 1.0.0
**Authority:** Below INVARIANTS, WISDOM, DECISIONS in the authority stack. Defines operational delegation, cadences, and scoring.
**Purpose:** Transform 4DA from a reactive toolkit into a self-operating system where Claude handles day-to-day operations and the human makes critical judgments.

---

## Decision Delegation Matrix

Three tiers of operational authority. Claude operates autonomously within Tier 1. Everything else escalates.

### Tier 1 — Autonomous (Claude executes without asking)

| Category | Examples |
|----------|----------|
| Build fixes | Fix compilation errors, resolve import issues, fix linting errors |
| Source recovery | Restart failed source fetchers, retry transient network errors |
| Doc sync | Update generated docs, sync type definitions, update changelogs |
| Lint/format | Run `cargo fmt`, fix clippy warnings, fix ESLint auto-fixable issues |
| Test maintenance | Update snapshots, fix flaky test timing, add missing assertions |
| File size splits | Split files approaching size limits (INV-090) |
| Dependency patches | Apply patch-level updates (x.y.Z) for non-breaking security fixes |
| MCP memory | Record decisions, learnings, code locations, metrics |
| State updates | Update ops-state.json, cadence timestamps, metabolism maps |

### Tier 2 — Recommend (Claude presents options, human decides)

| Category | Examples |
|----------|----------|
| Dependency minor upgrades | Minor version bumps (x.Y.0) that may change behavior |
| Scoring algorithm changes | Threshold adjustments, weight modifications, new signal types |
| Architecture changes | Module restructuring, new module creation, API changes |
| Feature additions | New source adapters, new UI components, new commands |
| Design changes | Color modifications, layout changes, new interaction patterns |
| Authority doc edits | Any modification to .ai/ files (INVARIANTS, WISDOM, DECISIONS) |
| Performance trade-offs | Caching strategies, query optimization with behavior changes |

### Tier 3 — Human Only (Claude surfaces information, human initiates)

| Category | Examples |
|----------|----------|
| Major version upgrades | React, Tauri, Vite major bumps |
| License changes | Any modification to licensing terms |
| Monetization decisions | Pricing, feature gating, tier boundaries |
| Launch decisions | When to release, what to include |
| Strategic pivots | Product direction, target audience changes |
| External communications | README, marketing copy, community announcements |
| Data model changes | Schema migrations that affect existing user data |
| Security incidents | Vulnerability responses, key rotation |

---

## Sovereignty Score Formula

**Total: 0-100** — weighted sum of 10 components.

| # | Component | Weight | Source | Calculation |
|---|-----------|--------|--------|-------------|
| 1 | Build Health | 15% | `cargo check` + `pnpm run build` | 100 if both pass, 50 if one fails, 0 if both fail |
| 2 | Test Health | 15% | `cargo test --lib` + `pnpm run test -- --run` | (passing / total) * 100 |
| 3 | Source Pipeline | 10% | MCP `source_health` | Average health across active sources |
| 4 | Dependency Freshness | 10% | `cargo outdated` + `pnpm outdated` | 100 - (2 * major_behind) - (1 * minor_behind) |
| 5 | Invariant Compliance | 15% | Invariant checks (INV-001 through INV-090) | (passing / total) * 100 |
| 6 | File Size Compliance | 5% | `pnpm run validate:sizes` | 100 if all pass, deduct 10 per violation |
| 7 | Decision Debt | 10% | DECISIONS.md age analysis | 100 - (5 * decisions_older_than_90d_unreviewed) |
| 8 | Strategic Alignment | 5% | Drift detection (dev patterns vs stated priorities) | Alignment score from drift report |
| 9 | Memory Health | 5% | MCP memory utilization ratio | (decisions_referenced / decisions_stored) * 100 |
| 10 | Metabolism | 10% | Hot/warm/cold/dead zone distribution | 100 - (10 * dead_files) - (5 * untested_hot_files) |

**Score = sum(component_score * weight)**

**Thresholds:**
- 90-100: Sovereign — system is self-sustaining
- 70-89: Healthy — minor attention needed
- 50-69: Degraded — targeted remediation required
- Below 50: Critical — War Room activation recommended

---

## Operational Cadences

### Session Cadence (every session)

Triggered automatically by `ops-session-start.cjs`:

1. Load ops-state.json and compute sovereignty delta
2. Check for overdue cadences (daily/weekly/monthly)
3. Check escalation queue for pending items
4. Check immune scan pending flag
5. Output briefing message with instructions

### Daily Cadence

Triggered when `lastDaily` > 24 hours ago. Run via `/ops daily`.

| # | Check | Tool/Command | Pass Criteria |
|---|-------|--------------|---------------|
| 1 | Source health | MCP `source_health` | All active sources responsive |
| 2 | Build validation | `cargo check` + `pnpm run build` | Zero errors |
| 3 | Test run | `cargo test --lib` + `pnpm run test -- --run` | All pass |
| 4 | File size validation | `pnpm run validate:sizes` | All within limits |
| 5 | Sovereignty score | Compute all 10 components | Score recorded |
| 6 | Store metrics | MCP `record_metric` type: `ops-daily` | Recorded |

### Weekly Cadence

Triggered when `lastWeekly` > 7 days ago. Run via `/ops weekly`.

| # | Check | Method | Output |
|---|-------|--------|--------|
| 1 | Everything in daily | (above) | (above) |
| 2 | Strategic drift | Spawn drift-detector agent | Drift report with alignment score |
| 3 | Codebase metabolism | Git log analysis (30 days) | Hot/warm/cold/dead zone map |
| 4 | Decision propagation | Search for constraint violations | Violation report with file:line |
| 5 | Pre-launch subset | Build health + MCP contracts | Pass/fail per system |
| 6 | Attention report | MCP `attention_report` | Distribution analysis |

### Monthly Cadence

Triggered when `lastMonthly` > 30 days ago. Run via `/ops monthly`.

| # | Check | Method | Output |
|---|-------|--------|--------|
| 1 | Everything in weekly | (above) | (above) |
| 2 | Decision replay | Spawn decision-replay agent | Per-decision verdict |
| 3 | Full invariant audit | Run all INV-NNN checks | Compliance report |
| 4 | Wisdom crystallization | Trigger `/crystallize` | Promoted patterns |
| 5 | Dependency assessment | Major version analysis + web search | Upgrade recommendations |
| 6 | Compound intelligence | Compute compound score | Monthly trend report |
| 7 | Strategic report | Summary for human review | Action items |

---

## Escalation Protocol

When presenting a Tier 2 or Tier 3 decision to the human, use this format:

```
ESCALATION — [Tier N] [ID]
━━━━━━━━━━━━━━━━━━━━━━━━━━

SITUATION: [One sentence describing what happened or what was discovered]

TRIED: [What autonomous investigation was done before escalating]

OPTIONS:
  1. [Option A] — [trade-offs, confidence: high/medium/low]
  2. [Option B] — [trade-offs, confidence: high/medium/low]
  3. [Do nothing] — [what happens if deferred]

RECOMMENDATION: Option [N] — [specific rationale with evidence]

ACTION NEEDED: [Exact action the human should take, or "approve option N"]
```

---

## Immune Memory Rules

### When to Create Antibodies

An antibody MUST be created when:
- A bug fix commit is detected (keywords: fix, bug, patch, resolve, regression, hotfix)
- A regression is found during any cadence check
- A failure mode is documented in FAILURE_MODES.md

### Antibody Pattern Format

Stored in MCP memory with topic `antibody`:

```
topic: "antibody"
content: "PATTERN: [vulnerability class]. FOUND IN: [file:line]. FIX: [what was done]. SCAN FOR: [regex or grep pattern to detect similar issues]"
context: "Created from fix in commit [hash]. Severity: [critical/high/medium/low]"
```

### Scan Scope

When an antibody is created, scan:
1. All files in the same module as the fix
2. All files that import from the fixed file
3. All files matching the same language (Rust or TypeScript)
4. Report matches with severity assessment (likely bug / review needed / false positive)

---

## Codebase Metabolism Zones

Classification based on git change frequency over 30 days:

| Zone | Threshold | Actions |
|------|-----------|---------|
| Hot | >5 changes/30d | Needs extra test coverage, frequent review, candidate for abstraction |
| Warm | 1-4 changes/30d | Normal maintenance, adequate coverage required |
| Cold | 0 changes for 30d+ | Check for bit rot, outdated patterns, still-valid dependencies |
| Dead | Not imported/referenced | Flag for removal, verify no dynamic references |

### Dead Zone Detection

For each cold file:
1. Search for import/use statements referencing the file
2. Search for dynamic imports or lazy loading patterns
3. Check if referenced in configuration files
4. If zero references found → classify as Dead

### Metabolism Score

```
score = 100 - (10 * dead_files) - (5 * untested_hot_files) - (2 * cold_files_with_outdated_patterns)
```

---

## Decision Replay Schedule

### Frequency

Monthly, as part of the monthly cadence.

### Re-evaluation Criteria

For each decision in DECISIONS.md:

| Factor | Trigger |
|--------|---------|
| Age | >90 days since decision → mandatory review |
| Landscape | New alternatives emerged (web search for category) |
| Evidence | Related learnings or failures in MCP memory |
| Usage | Decision referenced 0 times in MCP → may be irrelevant |
| Failures | Related entries in FAILURE_MODES.md |

### Staleness Thresholds

- **0-90 days:** Fresh — skip unless failure evidence exists
- **90-180 days:** Review — check for landscape changes
- **180+ days:** Mandatory review — full re-evaluation with web search

### Verdicts

- **CONFIRMED:** Decision still valid, no changes needed
- **REVIEW NEEDED:** Landscape has changed, human should evaluate
- **POTENTIALLY OBSOLETE:** Strong evidence suggests revisiting

---

## War Room Activation Criteria

### Automatic Triggers

1. Sovereignty score drops >10 points in a single session
2. Build health goes to 0 (both Rust and frontend fail)
3. Critical invariant violation detected (INV-001, INV-004, INV-030)

### Manual Trigger

`/ops war-room <issue description>`

### Agent Deployment

Four parallel agents are spawned:

| Role | Agent Type | Mission |
|------|-----------|---------|
| Debugger | `debugger` | Trace root cause with evidence |
| Historian | `Explore` | Search MCP memory + FAILURE_MODES.md for prior incidents |
| Architect | `Explore` | Map blast radius — affected and unaffected systems |
| Validator | `general-purpose` | Run full validation suite, confirm what still works |

### Briefing Packet Format

```
WAR ROOM BRIEFING
━━━━━━━━━━━━━━━━━

SITUATION: [one sentence]
SEVERITY: CRITICAL | HIGH
DETECTED: [how it was found]

ROOT CAUSE (Debugger): [findings]
PRIOR ART (Historian): [similar past incidents]
BLAST RADIUS (Architect): [affected/unaffected systems]
VALIDATION (Validator): [test results, build status]

OPTIONS: [numbered list with trade-offs and confidence]
RECOMMENDATION: [option + rationale]
```

---

## Compound Intelligence Metrics

### What's Tracked

| Metric | Source | Meaning |
|--------|--------|---------|
| decisions_stored | MCP `recall_decisions` count | Total knowledge base |
| decisions_referenced | MCP `recall_decisions` with search hits | Active knowledge usage |
| antibodies_created | ops-state.json compound.antibodiesCreated | Immune system growth |
| antibodies_triggered | ops-state.json compound.antibodiesTriggered | Immune system effectiveness |
| crystallizations_run | ops-state.json compound.crystallizationsRun | Wisdom accumulation |
| rework_events | MCP `get_metrics` type: rework | Quality regression indicator |
| gate_passes | MCP `get_metrics` type: gate_pass | Quality improvement indicator |
| sessions_completed | ops-state.json compound.sessionsCompleted | Productivity denominator |

### Compound Intelligence Score (0-100)

| Component | Weight | Calculation |
|-----------|--------|-------------|
| Memory Utilization | 20% | (decisions_referenced / decisions_stored) * 100 |
| Immune Effectiveness | 20% | (1 - bugs_repeated / antibodies_created) * 100 |
| Wisdom Accumulation | 15% | (crystallizations / sessions * 100) normalized |
| Quality Trend | 20% | gate_pass_rate slope (positive = improving) |
| Rework Trend | 15% | inverse rework_rate slope (declining = good) |
| Session Productivity | 10% | avg sovereignty delta per session |

### How Improvement is Measured

Month-over-month comparison of compound score. The system is getting smarter if:
- Score increases month-over-month
- Memory utilization trends upward
- Rework events trend downward
- Antibodies prevent repeat bugs (triggered count > 0)

---

*AOS is the conductor. Everything else is an instrument. The sovereignty score is the tuning fork.*
