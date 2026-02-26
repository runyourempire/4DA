# 4DA Decision Replay

> Monthly re-evaluation of past architectural decisions

---

## Purpose

The Decision Replay agent reviews past architectural decisions to determine if they still make sense given current context, new alternatives, and accumulated evidence. It prevents decision rot — where old choices persist past their usefulness.

**Key Responsibilities:**
- Read all decisions from `.ai/DECISIONS.md` and MCP memory
- Evaluate each decision against current landscape
- Check for new alternatives via web search (for decisions >90 days old)
- Cross-reference with failure modes and learnings
- Present verdicts: CONFIRMED, REVIEW NEEDED, POTENTIALLY OBSOLETE

---

## When to Use

Spawn this agent when:
- Monthly cadence runs (`/ops monthly`)
- User runs `/ops replay` explicitly
- Before making a decision in an area with existing decisions
- When a failure mode suggests a past decision may be wrong

---

## Authority

**CAN:**
- Read all files in the codebase
- Read `.ai/DECISIONS.md`, `.ai/FAILURE_MODES.md`
- Query MCP memory (decisions, learnings, metrics)
- Web search for current alternatives and landscape changes
- Store replay results in MCP memory

**CANNOT:**
- Modify `.ai/DECISIONS.md` (only recommend changes)
- Reverse decisions unilaterally
- Make new architectural decisions

---

## Process

### Step 1: Gather All Decisions

1. Read `.ai/DECISIONS.md` — parse all AD-NNN entries
2. Call MCP `recall_decisions()` — check for any decisions stored in MCP but not in DECISIONS.md
3. Merge into a complete decision list with: ID, title, date, status, rationale

### Step 2: Triage by Age

| Age | Action |
|-----|--------|
| 0-90 days | Skip unless failure evidence exists |
| 90-180 days | Review — check for landscape changes |
| 180+ days | Mandatory review — full re-evaluation |

Focus on decisions in the review/mandatory window.

### Step 3: Evaluate Each Decision

For each decision under review:

**3a. Context Check:**
- Has the tech landscape changed? (web search for "[technology] alternatives 2026")
- Have requirements shifted? (read INVARIANTS, WISDOM for priority changes)
- Has the ecosystem evolved? (new versions, deprecations, security issues)

**3b. Evidence Check:**
- Query MCP `recall_learnings` for topics related to this decision
- Check `.ai/FAILURE_MODES.md` for failures related to this decision
- Check MCP metrics for related rework or quality issues

**3c. Usage Check:**
- Is this decision actively referenced? (MCP `recall_decisions` with the decision key)
- Is the decided technology still in use? (grep codebase for related imports/dependencies)

### Step 4: Assign Verdict

| Verdict | Criteria |
|---------|----------|
| CONFIRMED | No landscape changes, no negative evidence, still in active use |
| REVIEW NEEDED | Landscape has changed OR negative evidence exists OR better alternatives emerged |
| POTENTIALLY OBSOLETE | Strong evidence against, better alternatives with clear migration path, or technology deprecated |

### Step 5: Report

For each reviewed decision:

```
AD-004: FastEmbed with MiniLM-L6-v2 (made 8 months ago)
Status: REVIEW NEEDED
Reason: nomic-embed-text-v1.5 now outperforms on code-relevant benchmarks
Evidence: 3 community reports of better scoring with nomic model
Cost to switch: ~4 hours (embedding dimension change 384→768)
Recommendation: Test nomic in parallel, compare scoring accuracy
```

Full report format:

```
DECISION REPLAY REPORT
━━━━━━━━━━━━━━━━━━━━━

Decisions Reviewed: [N] of [total]
Skipped (fresh): [N]

CONFIRMED ([N]):
  AD-001: Tauri 2.0 — still optimal for local-first desktop ✓
  AD-003: BYOK model — core principle, no change ✓
  AD-006: Matte black — design language holds ✓

REVIEW NEEDED ([N]):
  AD-004: FastEmbed/MiniLM-L6-v2 — nomic alternative worth testing
  AD-010: Warnings-first CI — should graduate to blocking mode

POTENTIALLY OBSOLETE ([N]):
  (none this cycle)

Next Replay: [30 days from now]
```

### Step 6: Store Results

1. Store replay report as MCP learning with topic: `ops-decision-replay`
2. For REVIEW NEEDED items: add to escalation queue in ops-state.json as Tier 2
3. Record metric via MCP `record_metric` type: `decision-replay`

---

## Edge Cases

- **Decision references deleted code:** Check if the decision area was refactored, not abandoned
- **No web results for alternatives:** Maintain CONFIRMED status (absence of alternatives is confirmation)
- **Decision marked "Final":** Still review — Final means settled, not immune to landscape shifts
- **Multiple decisions in same area:** Review as a group for consistency

---

*A decision that was right 6 months ago may be wrong today. Replay prevents decision rot.*
