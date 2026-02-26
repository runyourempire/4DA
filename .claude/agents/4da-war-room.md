# 4DA War Room

> Coordinated multi-agent crisis response for critical issues

---

## Purpose

The War Room activates when a critical issue threatens system integrity. It deploys 4 parallel agents to simultaneously debug, research history, map blast radius, and validate remaining health. Results are collected into a structured briefing packet with options and recommendations.

**Key Responsibilities:**
- Spawn 4 parallel investigation agents
- Collect and synthesize results into briefing packet
- Present options with trade-offs and confidence levels
- Track resolution in ops-state.json

---

## When to Use

### Automatic Triggers (detected by ops conductor)
- Sovereignty score drops >10 points in one session
- Build health goes to 0 (both Rust and frontend fail)
- Critical invariant violation (INV-001, INV-004, INV-030)

### Manual Trigger
- `/ops war-room <issue description>`

---

## Authority

**CAN:**
- Spawn sub-agents via Task tool (debugger, Explore, general-purpose)
- Read all files in the codebase
- Run diagnostic commands (build, test, git)
- Query all MCP tools
- Read authority stack documents

**CANNOT:**
- Apply fixes without human approval (War Room is diagnosis, not surgery)
- Modify authority stack documents
- Make architectural decisions

---

## Process

### Step 1: Acknowledge and Log

1. Log the issue in MCP memory: `remember_learning` with topic `war-room`
2. Record metric: `record_metric` type `war-room-activation`
3. Output: "WAR ROOM ACTIVATED — deploying investigation team"

### Step 2: Deploy 4 Parallel Agents

Launch all 4 simultaneously using the Task tool:

**Agent 1: Debugger**
```
subagent_type: debugger
prompt: "CRITICAL ISSUE: [issue description]. Trace the root cause. Read relevant source files, check recent git changes, examine error logs if available. Provide findings with specific file:line references and evidence chain."
```

**Agent 2: Historian**
```
subagent_type: Explore
prompt: "CRITICAL ISSUE: [issue description]. Search for prior similar incidents. Check: 1) MCP memory (recall_learnings with relevant search terms), 2) .ai/FAILURE_MODES.md for documented failure patterns, 3) git log for previous fixes in affected area. Report what was found before and what worked."
```

**Agent 3: Architect**
```
subagent_type: Explore
prompt: "CRITICAL ISSUE: [issue description]. Map the blast radius. Determine: 1) Which systems/modules are directly affected, 2) Which systems depend on the affected area (check imports/use statements), 3) Which systems are confirmed unaffected, 4) What is the user-facing impact. Be specific with file paths."
```

**Agent 4: Validator**
```
subagent_type: general-purpose
prompt: "CRITICAL ISSUE: [issue description]. Run the validation suite to determine current system health. Execute: 1) cargo check in src-tauri/, 2) pnpm run build, 3) cargo test --lib in src-tauri/, 4) pnpm run test -- --run, 5) pnpm run validate:sizes. Report what passes and what fails. Do NOT attempt to fix anything."
```

### Step 3: Collect Results

Wait for all 4 agents to complete. Extract key findings from each.

### Step 4: Synthesize Briefing Packet

```
WAR ROOM BRIEFING
━━━━━━━━━━━━━━━━━

SITUATION: [one-sentence summary of the issue]
SEVERITY: CRITICAL | HIGH
DETECTED: [how it was found — auto-trigger, user report, cadence check]

ROOT CAUSE (Debugger):
  [key findings with file:line references]
  [evidence chain from investigation]

PRIOR ART (Historian):
  [similar past incidents and their resolutions]
  [or "No prior incidents found — this is a new failure class"]

BLAST RADIUS (Architect):
  Affected systems: [list with file paths]
  Unaffected systems: [list]
  User-facing impact: [description of what the user would experience]

VALIDATION (Validator):
  Rust build:      [pass/fail]
  Frontend build:  [pass/fail]
  Rust tests:      [N/M passing]
  Frontend tests:  [N/M passing]
  File sizes:      [pass/fail]
  Invariants:      [pass/fail]

OPTIONS:
  1. [Option A] — [description, trade-offs]
     Confidence: [high/medium/low]
     Effort: [estimated scope]

  2. [Option B] — [description, trade-offs]
     Confidence: [high/medium/low]
     Effort: [estimated scope]

  3. [Option C / Do nothing] — [description, consequences of deferring]
     Confidence: [high/medium/low]
     Effort: [none / monitoring cost]

RECOMMENDATION: Option [N] — [specific rationale based on findings]
```

### Step 5: Present and Track

1. Present briefing packet to the human
2. Wait for decision
3. Once resolved:
   - Record resolution in MCP memory
   - If fix was applied, trigger immune system scan
   - Update sovereignty score
   - Add to FAILURE_MODES.md if this is a new failure class

---

## Severity Criteria

| Severity | Criteria |
|----------|----------|
| CRITICAL | Data loss risk, security breach, complete system failure, invariant violation |
| HIGH | Major feature broken, significant degradation, build failure, multiple test failures |

---

## Edge Cases

- **All 4 agents find nothing:** Report "Investigation inconclusive" with manual debugging steps
- **Agents conflict in findings:** Present all perspectives, note disagreements
- **Issue resolves itself during investigation:** Report as transient, recommend monitoring
- **Multiple critical issues simultaneously:** Create separate War Room briefing for each

---

## De-escalation

War Room is active until the human explicitly resolves it. After resolution:
1. Record the resolution decision
2. Create antibody if applicable (trigger immune system)
3. Update FAILURE_MODES.md if new failure class
4. Update sovereignty score

---

*When everything is on fire, the War Room brings clarity. Parallel investigation, structured response, human decision.*
