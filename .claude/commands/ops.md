---
description: "AOS — Autonomous Operations System. Sovereignty score, cadences, decisions, war room, immune system."
allowed-tools: ["Read", "Bash", "Glob", "Grep", "Edit", "Write", "Task", "WebSearch"]
argument-hint: "[status | daily | weekly | monthly | decide | war-room <issue> | immune [context] | replay | drift | metabolism | compound]"
---

# /ops — Autonomous Operations System

Run the 4DA Autonomous Operations System. Computes sovereignty score, executes operational cadences, manages decisions, and coordinates specialized agents.

## Arguments

- `$ARGUMENTS` — subcommand (default: `status`)

## Authority Document

Read `.ai/OPS.md` before executing any subcommand. It defines the delegation matrix, score formula, cadence checks, and escalation protocol.

## Subcommands

### `/ops` or `/ops status`

Compute sovereignty score and show brief status.

1. Read `.claude/wisdom/ops-state.json`
2. Read `.ai/OPS.md` for score formula
3. Compute each of the 10 sovereignty components:
   - **Build Health (15%):** Run `cargo check` in `src-tauri/` and `pnpm run build` — both pass = 100, one fails = 50, both = 0
   - **Test Health (15%):** Run `cargo test --lib` in `src-tauri/` and `pnpm run test -- --run` — (passing/total)*100
   - **Source Pipeline (10%):** Call MCP `source_health` — average health
   - **Dependency Freshness (10%):** Estimate from state or run checks
   - **Invariant Compliance (15%):** Check INV-030 (key exposure), INV-090 (file sizes), INV-050 (theme)
   - **File Size Compliance (5%):** Run `pnpm run validate:sizes` — 100 if pass, -10 per violation
   - **Decision Debt (10%):** Read `.ai/DECISIONS.md`, count decisions >90d unreviewed
   - **Strategic Alignment (5%):** Use last drift score from ops-state.json
   - **Memory Health (5%):** MCP `recall_decisions` utilization
   - **Metabolism (10%):** Hot/cold/dead zone distribution from ops-state.json
4. Compute weighted total
5. Update ops-state.json with new scores
6. Record metric via MCP `record_metric`
7. Display sovereignty status with bar chart and pending items
8. Include compound score (from last `/ops compound` run in ops-state.json) alongside sovereignty score in the summary line
9. Include test count (from ops-state.json `testCounts.history` latest entry) in the summary line after sovereignty and compound scores
   - Format: `Sovereignty: XX% | Compound: XX% | Tests: NNNN (rust) + NNNN (frontend) = NNNN total`

### `/ops daily`

Execute daily operations cadence. Spawn `4da-ops-conductor` agent:

```
Spawn the 4da-ops-conductor agent with Task tool.
Prompt: "Execute the daily cadence as defined in .ai/OPS.md. Run all 9 daily checks, compute sovereignty score, auto-remediate Tier 1 issues, and report results. Update cadence.lastDaily in ops-state.json when complete."
```

### `/ops weekly`

Execute weekly operations cadence. Spawn `4da-ops-conductor` agent:

```
Spawn the 4da-ops-conductor agent with Task tool.
Prompt: "Execute the weekly cadence as defined in .ai/OPS.md. This includes all daily checks PLUS strategic drift detection (spawn 4da-drift-detector), codebase metabolism analysis, decision propagation check, and attention report. Update cadence.lastWeekly in ops-state.json when complete."
```

### `/ops monthly`

Execute monthly deep audit. Spawn `4da-ops-conductor` agent:

```
Spawn the 4da-ops-conductor agent with Task tool.
Prompt: "Execute the monthly cadence as defined in .ai/OPS.md. This includes all weekly checks PLUS decision replay (spawn 4da-decision-replay), full invariant audit, wisdom crystallization recommendation, dependency major version assessment, and compound intelligence report. Update cadence.lastMonthly in ops-state.json when complete."
```

### `/ops decide`

Present the next pending Tier 2/3 decision from the escalation queue.

1. Read ops-state.json escalation queue
2. Filter to unresolved items
3. If none: report "No pending decisions — system is autonomous"
4. If items exist: present the oldest highest-tier item using the Escalation Protocol format from OPS.md
5. After human responds, update the item as resolved

### `/ops war-room <issue>`

Activate War Room protocol for a critical issue. Spawn `4da-war-room` agent:

The issue description is everything after "war-room" in `$ARGUMENTS`.

```
Spawn the 4da-war-room agent with Task tool.
Prompt: "CRITICAL ISSUE: [issue from arguments]. Execute War Room protocol as defined in .ai/OPS.md. Deploy 4 parallel agents (debugger, historian, architect, validator), collect results into briefing packet, present options with recommendations."
```

### `/ops immune [context]`

Run immune memory scan. Spawn `4da-immune-system` agent:

If context provided in arguments, use it. Otherwise check ops-state.json `immuneContext`.

```
Spawn the 4da-immune-system agent with Task tool.
Prompt: "Run immune system scan. [Context from arguments or ops-state.json immuneContext]. Analyze the bug fix, extract the vulnerability pattern, create an antibody in MCP memory, scan the codebase for similar issues, and report findings. Update ops-state.json to clear immuneScanPending."
```

After completion, update ops-state.json: set `immuneScanPending: false`, increment `compound.antibodiesCreated`.

### `/ops replay`

Run decision replay — re-evaluate past architectural decisions. Spawn `4da-decision-replay` agent:

```
Spawn the 4da-decision-replay agent with Task tool.
Prompt: "Run decision replay as defined in .ai/OPS.md. Read all decisions from .ai/DECISIONS.md and MCP memory. For each decision older than 90 days, evaluate context changes, new evidence, and landscape shifts. Present verdicts: CONFIRMED, REVIEW NEEDED, or POTENTIALLY OBSOLETE."
```

### `/ops drift`

Run strategic drift detection. Spawn `4da-drift-detector` agent:

```
Spawn the 4da-drift-detector agent with Task tool.
Prompt: "Run strategic drift detection as defined in .ai/OPS.md. Analyze git log for 30 days, categorize commits by area, compare development distribution against stated priorities in INVARIANTS.md and WISDOM.md. Compute alignment score and identify neglected/over-invested areas."
```

### `/ops metabolism`

Run codebase metabolism analysis:

1. Analyze git log for the last 30 days
2. For each source file, count changes: `git log --oneline --since="30 days ago" -- <file>`
3. Classify files into Hot (>5 changes), Warm (1-4), Cold (0 for 30d+), Dead (no references)
4. For dead candidates, grep for import/use statements
5. Update metabolism data in ops-state.json
6. Display metabolism report with hot zones, cold zones, dead tissue

### `/ops compound`

Show compound intelligence report — how the Human+Opus partnership is accumulating knowledge over time.

1. Read `.claude/wisdom/ops-state.json`
2. Read MCP memory metrics via `recall_decisions` (count), `recall_learnings` (count), `recall_code_locations` (count)
3. Read test counts from ops-state.json `testCounts.history` — show trend over last 5 entries
4. Gather compound data:
   - **Sessions completed:** from ops-state.json `compound.sessionsCompleted`
   - **Decisions stored / referenced:** ratio = leverage (higher means decisions are being reused, not just stored)
   - **Antibodies created / triggered:** ratio = protection (higher means the immune system is catching real issues)
   - **Crystallizations run:** from ops-state.json `compound.crystallizations`
   - **Rework events:** from ops-state.json `compound.reworkEvents` (lower is better — rework means a prior decision was wrong)
   - **Test trend:** chart last 5 data points from `testCounts.history` using ASCII sparkline (e.g., `▁▃▅▇█`)
   - **Knowledge density:** (decisions + learnings + code_locations) per 1000 LOC — compute LOC via `find src-tauri/src -name '*.rs' | xargs wc -l` and `find src -name '*.ts' -o -name '*.tsx' | xargs wc -l`
5. Compute **Compound Score** (0-100) using weighted formula:
   - **Test growth rate (20%):** % increase in tests from oldest to newest of last 5 history entries. 0% growth = 50, >20% growth = 100, negative = 0
   - **Decision reference ratio (20%):** decisions_referenced / decisions_stored. Ratio ≥1.0 = 100, 0.5 = 75, 0.0 = 0
   - **Antibody effectiveness (15%):** antibodies_triggered / antibodies_created. Ratio ≥0.5 = 100, 0.25 = 75, 0.0 = 25 (having antibodies untriggered still has value)
   - **Low rework rate (15%):** 100 - (rework_events / sessions_completed * 200). Clamped to 0-100. Zero rework = 100
   - **Knowledge density (15%):** (total_knowledge_items / total_KLOC). ≥50 per KLOC = 100, 25 = 75, 10 = 50, <5 = 25
   - **Session consistency (15%):** sessions in last 30 days / 30. Daily sessions = 100, every other day = 50. From ops-state.json `compound.recentSessionDates`
6. Display formatted report:

```
╔══════════════════════════════════════════════╗
║        COMPOUND INTELLIGENCE REPORT          ║
╠══════════════════════════════════════════════╣
║                                              ║
║  Sessions completed:     NNN                 ║
║  Decisions:              NN stored / NN ref   → leverage: X.Xx
║  Antibodies:             NN created / NN hit  → protection: X.Xx
║  Crystallizations:       NN                  ║
║  Rework events:          NN (target: 0)      ║
║                                              ║
║  Test trend (last 5):    ▁▃▅▇█  NNNN→NNNN   ║
║  Knowledge density:      XX.X per KLOC       ║
║                                              ║
╠══════════════════════════════════════════════╣
║  COMPOUND SCORE                              ║
║                                              ║
║  Test growth ........... XX% (weight: 20%)   ║
║  Decision leverage ..... XX% (weight: 20%)   ║
║  Antibody effectiveness  XX% (weight: 15%)   ║
║  Low rework rate ....... XX% (weight: 15%)   ║
║  Knowledge density ..... XX% (weight: 15%)   ║
║  Session consistency ... XX% (weight: 15%)   ║
║                                              ║
║  ══════════════════════════════════          ║
║  COMPOUND SCORE: XX%                         ║
╚══════════════════════════════════════════════╝
```

7. Update ops-state.json: set `compound.lastScore`, `compound.lastRun`, and `compound.scoreHistory` (append latest)
8. Record metric via MCP `record_metric` with name `compound_score`

## Edge Cases

- **Dev server running (exe locked):** Use `--lib` flag for cargo commands
- **ops-state.json missing:** Create with defaults, compute fresh
- **MCP tools unavailable:** Skip MCP-dependent components, note in report
- **Git history < 30 days:** Use available history, note limited data

## What NOT to Do

- Do NOT modify `.ai/` authority docs without presenting as Tier 2 escalation
- Do NOT auto-fix Tier 2/3 issues
- Do NOT skip recording metrics after cadence runs
- Do NOT update cadence timestamps if the cadence was interrupted or incomplete
- Do NOT run `pnpm run tauri build` (too slow for operational checks)
