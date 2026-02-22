# /crystallize

Review accumulated learnings in MCP memory and propose promotions to formal constraints.

## Arguments

- `$ARGUMENTS` — optional: `--topic=<topic>` to focus on a specific area, `--dry-run` to report without proposing

## Instructions

You are the crystallization engine for 4DA's wisdom layer. Your job is to find patterns in accumulated experience and propose their promotion to formal project constraints.

Read `.ai/WISDOM.md` section "Crystallization" for the full process definition.

---

### Step 1: Gather Memory

Query all accumulated learnings and decisions from MCP memory:

```
recall_learnings({ search: "" })          — all learnings
recall_decisions({})                      — all decisions
search_memory({ query: "failed" })        — failure patterns
search_memory({ query: "regression" })    — regressions
search_memory({ query: "gotcha" })        — gotchas
search_memory({ query: "mistake" })       — mistakes
```

If `--topic` was provided, filter results to that topic.

---

### Step 2: Identify Clusters

Group learnings by topic and semantic similarity. A cluster is 3+ learnings that teach the same lesson. Look for:

- **Repeated gotchas** — the same technical trap encountered multiple times
- **Reversed decisions** — patterns of choosing X, discovering it's wrong, choosing Y
- **Recurring regressions** — the same area breaking repeatedly
- **Proven patterns** — approaches that consistently succeed

For each cluster found, record:
- Topic name
- Number of entries
- The consistent lesson across entries
- Specific MCP memory entries that form the cluster

---

### Step 3: Check Against Existing Constraints

For each cluster, verify it isn't already captured:

1. Read `.ai/INVARIANTS.md` — is this already an invariant?
2. Read `.ai/DECISIONS.md` — is this already a decision?
3. Read `.ai/FAILURE_MODES.md` — is this already a failure mode?

If already captured, skip. If partially captured (existing entry is incomplete), propose an update rather than a new entry.

---

### Step 4: Adversarial Test

For each remaining candidate, answer:

> "Under what conditions would this constraint be wrong?"

If the answer is "never" or "only in conditions that don't apply to 4DA," the candidate passes.
If the answer reveals legitimate exceptions, note them — the constraint may need scoping.

---

### Step 5: Present Proposals

For each candidate that passes adversarial testing, present a proposal:

```
## Crystallization Proposal N

**Source:** N learnings over N sessions
**Pattern:** [one-sentence summary of what was learned repeatedly]
**Adversarial test:** [when would this be wrong? + answer]

**Proposed entry:**

### [INV-NNN / AD-NNN / FM-SEVERITY-NNN]: [Title]
- **What/Decision:** [formal statement]
- **Rationale:** [why, citing the source learnings]
- **Date:** [today]
- **Status:** Final

**Destination:** [INVARIANTS.md / DECISIONS.md / FAILURE_MODES.md]
```

---

### Step 6: Execute (with approval)

If `--dry-run` was passed, stop after presenting proposals.

Otherwise, ask the human to approve, modify, or reject each proposal. For approved proposals:

1. Add the entry to the destination file using the established format
2. Record the crystallization event in MCP memory:
   ```
   remember_learning({
     topic: "crystallization",
     content: "Promoted [pattern] to [destination] as [ID]",
     context: "Source: N learnings from [topics]"
   })
   ```
3. Record a quality metric:
   ```
   record_metric({
     metric_type: "crystallization",
     value: 1,
     context: "[ID]: [short description]"
   })
   ```

---

### Step 7: Report

Display a summary:

```
## Crystallization Report

**Memory scanned:** N learnings, N decisions
**Clusters found:** N
**Already captured:** N (skipped)
**Proposals presented:** N
**Approved:** N
**Rejected:** N

**Next review:** Run /crystallize again when 10+ new learnings accumulate.
```

---

### When No Clusters Are Found

If no clusters of 3+ exist, report:

```
## Crystallization Report

No patterns ready for crystallization.

**Memory contains:** N learnings, N decisions
**Largest cluster:** N entries on [topic] (needs 3+ to crystallize)

The wisdom layer is accumulating experience. Continue recording
consequences and run /crystallize again later.
```

This is not a failure. Crystallization requires patience. Premature promotion of single learnings into formal constraints would violate W-7 (simplicity) by creating constraints that haven't earned their place.
