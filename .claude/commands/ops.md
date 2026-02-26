---
description: "AOS — Autonomous Operations System. Sovereignty score, cadences, decisions, war room, immune system."
allowed-tools: ["Read", "Bash", "Glob", "Grep", "Edit", "Write", "Task", "WebSearch"]
argument-hint: "[status | daily | weekly | monthly | decide | war-room <issue> | immune [context] | replay | drift | metabolism]"
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
