# 4DA Strategic Drift Detector

> Compare development patterns against stated priorities

---

## Purpose

The Drift Detector compares what we're actually building (git history, session data) against what we say matters (authority stack priorities). It surfaces misalignment before it becomes strategic debt.

**Key Responsibilities:**
- Analyze 30-day git commit history by area
- Read stated priorities from authority stack
- Compute development distribution
- Identify neglected and over-invested areas
- Calculate alignment score
- Store drift report in MCP memory and ops-state.json

---

## When to Use

Spawn this agent when:
- Weekly cadence runs (`/ops weekly`)
- User runs `/ops drift` explicitly
- Strategic planning sessions
- Before major direction decisions

---

## Authority

**CAN:**
- Read all files in the codebase
- Run git log commands for analysis
- Read `.ai/INVARIANTS.md`, `.ai/WISDOM.md`, `.ai/DECISIONS.md`
- Read ops-state.json drift data
- Store drift reports in MCP memory
- Update ops-state.json drift section

**CANNOT:**
- Modify source code
- Change priorities or authority documents
- Make strategic decisions (only surface data)

---

## Process

### Step 1: Read Stated Priorities

Read these authority documents and extract priorities:

**From `.ai/INVARIANTS.md`:**
- INV-001: Precision >85% → scoring must receive attention
- INV-002: Zero configuration → onboarding must receive attention
- INV-003: Never fail silently → error handling must be maintained
- INV-004: Privacy absolute → no regressions allowed
- INV-005: Learns but doesn't creep → transparency must be maintained

**From `.ai/WISDOM.md`:**
- Product wisdom principles (PW-1 through PW-5)
- Any stated focus areas

**From `.ai/DECISIONS.md`:**
- Recent decisions that imply focus areas (e.g., if AD-017 mentions Pro tier, monetization needs attention)

### Step 2: Analyze Git History

```bash
git log --oneline --since="30 days ago"
```

Categorize each commit by area:

| Area | Patterns in commit path/message |
|------|------|
| backend | `src-tauri/` (excluding scoring, sources, ace, extractors) |
| scoring | `src-tauri/src/scoring/`, `src-tauri/src/relevance` |
| sources | `src-tauri/src/sources/` |
| ace | `src-tauri/src/ace/` |
| frontend | `src/components/`, `src/` |
| design | `site/`, CSS, theme, UI polish |
| architecture | `.ai/`, system design |
| tooling | `.claude/`, `scripts/`, build config |
| mcp | `mcp-*/` |
| docs | `*.md` (non-authority), README |
| onboarding | commits mentioning onboarding, first-run, setup |

Use `git log --format="%H %s" --since="30 days ago"` to get commit hash + message, then `git show --stat <hash>` for affected files.

### Step 3: Supplement with Session Data

Read `ops-state.json` `drift.recentSessionCategories` for session-level categorization that may include uncommitted work.

### Step 4: Compute Distribution

Calculate percentage of commits per area. Total = 100%.

### Step 5: Compare Against Priorities

For each stated priority, check if the corresponding area is receiving proportional attention:

| Priority | Expected Area | Minimum Attention |
|----------|--------------|-------------------|
| INV-001 (precision) | scoring | >10% |
| INV-002 (zero config) | onboarding | >5% |
| INV-004 (privacy) | backend (security) | monitored |
| PW-2 (attention sovereignty) | scoring, sources | >15% combined |

Flag misalignments:
- Area has high priority but <5% attention → DRIFT WARNING
- Area has low priority but >30% attention → OVER-INVESTMENT WARNING
- Priority area untouched for >14 days → NEGLECT WARNING

### Step 6: Compute Alignment Score

```
alignment = 100 - (10 * drift_warnings) - (5 * over_investment_warnings) - (15 * neglect_warnings)
```

Clamped to 0-100.

### Step 7: Report

```
STRATEGIC DRIFT REPORT
━━━━━━━━━━━━━━━━━━━━━

Development Distribution (30d):
  Frontend polish:  42% ████████░░
  Backend/scoring:  18% ███░░░░░░░
  Sources:           8% █░░░░░░░░░
  MCP server:       12% ██░░░░░░░░
  Tooling/infra:    15% ███░░░░░░░
  Architecture:      5% █░░░░░░░░░

Stated Priorities vs Reality:
  "Zero-config value" (INV-002): onboarding untouched 21 days ⚠ DRIFT
  "Precision >85%" (INV-001): scoring active, aligned ✓
  "Privacy architecture" (INV-004): no regressions detected ✓

Alignment Score: 72/100

Recommendation: [Specific, actionable recommendation based on findings]
```

### Step 8: Store Results

1. Store drift report as MCP learning with topic: `ops-drift-report`
2. Update ops-state.json:
   - `drift.lastDriftReport`: current timestamp
   - Update sovereignty `strategicAlignment` component with alignment score

---

## Edge Cases

- **New project (<30 days history):** Use available history, note limited data
- **All areas equally distributed:** Report as balanced, alignment score 100
- **Single area dominates (>60%):** Flag potential tunnel vision regardless of priority
- **No commits in 30 days:** Report as stalled, sovereignty impact

---

*Drift is invisible until it's expensive. This agent makes it visible while it's cheap.*
