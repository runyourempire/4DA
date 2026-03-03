# 4DA Relevance Improvement Plan

## The Diagnosis: 7 Root Causes of Mediocre Results

### Root Cause 1: "All Results" defaults to showing EVERYTHING
`src/store/index.ts:610` — `showOnlyRelevant: false`

The confirmation gate IS working. ~25-30 of those 368 items actually pass the 2-signal requirement. But the frontend defaults to showing ALL 368, making the gate invisible. The user sees a wall of noise because the UI is hiding the system's best judgment.

### Root Cause 2: Knowledge Gaps has ZERO domain filtering
`src-tauri/src/knowledge_decay.rs:60-90` — `detect_knowledge_gaps()` pulls ALL dependencies from `project_dependencies`, does a `LIKE %name%` match against article titles, and returns anything that matches. No check against user interests. No relevance threshold. No domain awareness. If you have a dep named `space` or `crypto`, every article about space exploration or cryptocurrency shows up as a "knowledge gap."

### Root Cause 3: Off-domain penalty is laughably weak
`scoring_config.rs:67` — `OFF_DOMAIN_PENALTY: f32 = 0.12`

An article about fashion with ZERO overlap to your tech stack (Rust, Tauri, React, SQLite) loses only 12% of its score. If it happens to get a decent embedding similarity (common with broad topics), it can still pass the gate. This should be 50-70%, not 12%.

### Root Cause 4: Active topics threshold is too permissive
`scoring.rs:603` — `topics.iter().filter(|t| t.weight >= 0.3)`

Any topic with weight 0.3 becomes a confirmed ACE signal. But `ace/mod.rs:1516` inserts file_content topics with weight 0.6 automatically. This means almost anything detected from file scanning becomes a confirmed axis, giving weak items a second signal they don't deserve.

### Root Cause 5: Feedback learning requires 5 exposures to start
`ace/mod.rs:765-776` — Affinity requires `total_exposures >= 5` before computing a score. The first 4 dismissals of "fashion" content have ZERO effect. Users lose trust immediately because the system appears to ignore their feedback.

### Root Cause 6: No "domain lock" — the system doesn't know what you ARE
The system knows what you're interested in (declared interests) and what you're working on (ACE topics). But it has no concept of **what you are** — a Rust/TypeScript developer building desktop apps. There's no "negative space" model saying "everything outside Rust/TypeScript/React/Tauri/SQLite is almost certainly noise." The off-domain penalty is the only defense, and at 12%, it's paper.

### Root Cause 7: Quality floor bypass
`scoring.rs:1373-1376` — The quality floor allows items with `combined_score >= 0.55` to pass even with ZERO confirmed signals. This means a single strong embedding match (which can be spurious) bypasses the entire multi-signal gate.

---

## The Plan: Three Phases to Intelligence

### Phase 1: Immediate Impact (Fix the Leaks)

These are surgical changes that dramatically improve perceived quality overnight:

| Change | File | What | Impact |
|--------|------|------|--------|
| **1a** | `store/index.ts:610` | Default `showOnlyRelevant: true` | 368 items -> ~25 curated |
| **1b** | `scoring/pipeline.scoring` | `OFF_DOMAIN_PENALTY: 0.12 -> 0.50` | Crush irrelevant content |
| **1c** | `scoring.rs:603` | Active topic weight threshold `0.3 -> 0.55` | Stop weak signals confirming |
| **1d** | `scoring/pipeline.scoring` | `QUALITY_FLOOR_MIN_SCORE: 0.55 -> 0.70` | Close the 0-signal bypass |
| **1e** | `ace/mod.rs:765` | First "Mark Irrelevant" -> immediate anti-topic (bypass 5-exposure gate) | Instant feedback response |
| **1f** | `knowledge_decay.rs` | Filter gaps: only deps that match user's declared_tech OR detected_tech | Kill "fashion" gaps |
| **1g** | `knowledge_decay.rs` | Score missed_items against user context, drop below 0.3 | Only relevant gap articles |

### Phase 2: Domain Intelligence (Build the Identity)

This is where 4DA becomes genuinely smart — it builds a **User Domain Model** that acts as a pre-filter on everything.

**New concept: `DomainProfile`**

```rust
pub struct DomainProfile {
    // PRIMARY: Technologies the user actively uses (from project scanning)
    primary_stack: Vec<(String, f32)>,    // e.g. [("rust", 0.95), ("react", 0.90), ("tauri", 0.85)]

    // ADJACENT: Technologies related to their stack (auto-inferred)
    adjacent_tech: Vec<(String, f32)>,    // e.g. [("wasm", 0.6), ("typescript", 0.8)]

    // EXCLUDED: Everything else (the negative space)
    // Not stored — anything NOT in primary/adjacent is "excluded"

    // INTENT: What they're working on RIGHT NOW (from last 2h of git)
    current_intent: Vec<String>,          // e.g. ["scoring optimization", "build-time validation"]
}
```

**How it works:**
- Parse `Cargo.toml` dependencies -> extract the actual crates used -> map to technology domains
- Parse `package.json` -> same
- Parse import statements -> which APIs are actually called
- Build adjacency graph: if you use `tauri`, you probably care about `wasm`, `webview`, `desktop`
- **Everything not in primary or adjacent gets a 70-80% penalty** (not 12%)

**Scoring integration:**
- New signal axis: `domain_match` — is this within the user's domain profile?
- Replace the weak `off_domain_penalty` with a proper `domain_relevance_multiplier`
- Domain-locked items need only 1 other signal to pass (they have domain as a free signal)
- Non-domain items need 3+ signals to pass (almost impossible for noise)

### Phase 3: Novelty & Intent (The Innovation)

This is what no other tool does. Current dev news tools ask "Does this match your interests?" 4DA should ask **"Does this ADD to what you already know?"**

**3a: Novelty Detection**

```
User has `tauri = "2.0"` in Cargo.toml
  -> "Introduction to Tauri" scores 0 (you already know this)
  -> "Tauri 3.0 Release Notes" scores HIGH (genuinely new)
  -> "Tauri Performance Tips" scores MEDIUM (might know some, probably not all)
```

**Implementation:** Compare article content against user's existing knowledge (indexed code + previously read articles). High semantic similarity to existing knowledge = LOW novelty = score penalty. This is the inverse of the current approach.

**3b: Intent-Aware Scoring**

Parse the last 2 hours of git diffs to extract working intent:
```
Recent commits: "Add build-time compiler toolkit: scoring DSL, proc macros, SQL checker"
  -> Intent: [build systems, proc macros, compile-time validation, DSL design]
  -> Articles about proc macros score 3x
  -> Articles about CSS frameworks score 0.1x
```

This makes the feed feel like it's reading your mind — because it literally is reading your work.

**3c: Content Quality Signal**

Not all articles are equal. Add heuristics:
- **Length**: <500 words = likely shallow clickbait -> -30%
- **Source reputation**: Learned from user saves (already exists but too weak at +/-10%)
- **Title quality**: ALL CAPS, excessive punctuation, "You Won't Believe" -> -50%
- **Originality**: If 3 articles say the same thing, only the first/best survives (topic dedup on steroids)

**3d: Temporal Clustering**

Group related items into narratives instead of showing them individually:
```
INSTEAD OF:
  - "TypeScript 6.0 Announced" (HN)
  - "TypeScript 6.0 Beta" (dev.to)
  - "First Look at TypeScript 6.0" (Reddit)

SHOW:
  TypeScript 6.0 (3 articles)
  "TypeScript 6.0 announced with pattern matching and..."
  [Best article] + [2 more from HN, Reddit]
```

---

## Implementation Priority

| Priority | Phase | What | Lines of Code | Impact | Status |
|----------|-------|------|--------------|--------|--------|
| **NOW** | 1a | Default showOnlyRelevant=true | 1 line | Massive UX improvement | DONE |
| **NOW** | 1b | Off-domain penalty 0.12->0.50 | 1 line | Kills noise | DONE (0.25→0.50) |
| **NOW** | 1c | Topic weight threshold 0.3->0.55 | 1 line | Tightens signals | DONE |
| **NOW** | 1d | Quality floor bypass 0.55->0.70 | 1 line | Closes gate bypass | DONE (0.50→0.70) |
| **NOW** | 1e | Instant anti-topic on Mark Irrelevant | ~20 lines | Trust-building | DONE |
| **NOW** | 1f-1g | Knowledge Gaps domain filter | ~40 lines | Kills absurd categories | DONE |
| **NEXT** | 2 | Domain Profile + domain scoring | ~300 lines | Transformative | DONE |
| **THEN** | 3a | Novelty detection | ~200 lines | Unique differentiator | DONE |
| **THEN** | 3b | Intent-aware scoring from git | ~150 lines | "Reads your mind" | DONE |
| **THEN** | 3c | Content quality signal | ~100 lines | Kills blogspam | DONE |
| **THEN** | 3d | Temporal clustering | ~250 lines | Clean UX | DONE |

---

## The Vision

After these changes, opening 4DA should feel like this:

> **Intelligence Briefing** (first thing you see)
> "You committed a scoring DSL compiler yesterday. Here's an article about compile-time code generation in Rust that uses a similar approach but adds incremental compilation. Also, Tauri 2.1 shipped a hotfix for the webview crash you might hit on Windows."
>
> **Top 8 Results** (curated, domain-locked)
> All Rust/TypeScript/Tauri/React — zero fashion, zero dining, zero space
>
> **Knowledge Gaps** (2-3 items, not 20)
> "rusqlite 0.32 released with breaking API changes (you're on 0.31)"

That's it. No wall of noise. No 368 items. No "fashion" knowledge gaps. Just signal.

---

## Key Code Locations

| Component | File | Lines |
|-----------|------|-------|
| Scoring pipeline | `src-tauri/src/scoring.rs` | ~1900 lines |
| Scoring config (DSL source) | `src-tauri/scoring/pipeline.scoring` | 109 lines |
| Generated config constants | `src-tauri/src/scoring_config.rs` | include!() bridge |
| Knowledge gaps | `src-tauri/src/knowledge_decay.rs` | 290 lines |
| ACE context building | `src-tauri/src/scoring.rs` | 590-674 |
| Active topics loading | `src-tauri/src/ace/mod.rs` | 605-609 |
| Anti-topic learning | `src-tauri/src/ace/mod.rs` | 782-804 |
| Topic affinity computation | `src-tauri/src/ace/mod.rs` | 721-780 |
| Frontend store defaults | `src/store/index.ts` | 610 |
| KnowledgeGaps component | `src/components/KnowledgeGapsPanel.tsx` | 99 lines |
| BriefingView component | `src/components/BriefingView.tsx` | main intelligence view |
| ResultsView component | `src/components/ResultsView.tsx` | all results view |
| Off-domain penalty | `scoring.rs:957` + `pipeline.scoring` | OFF_DOMAIN_PENALTY |
| Quality floor | `scoring.rs:1373-1376` | QUALITY_FLOOR_MIN_SCORE |
| Confirmation gate | `scoring.rs:275-311` | apply_confirmation_gate() |
| Serendipity bypass | `scoring.rs:1501-1546` | compute_serendipity_candidates() |
