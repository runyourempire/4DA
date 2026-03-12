# PLAN: Optimal 4DA — Application Exploration

> Owner: Opus + Antony
> Created: 12 March 2026
> Status: READY TO EXECUTE
> Philosophy: Not less features — better connected features. Self-sustaining systems.

---

## Overview

Six phases transforming 4DA from a feature-rich application into a self-sustaining
intelligence system that gets smarter with every interaction. Each phase is
independently valuable and verifiable.

**Total estimated scope:** ~2,500 lines production code + ~800 lines tests
**Phases:** 6 (can execute sequentially or parallelize 1+2, 4+5)

---

## Phase 1: Content-First Entry

**Goal:** User sees meaningful content within 60 seconds of first launch.
**Current:** 8-15 minutes (4-step onboarding blocks all content).
**After:** Content visible at t=8-15s, full analysis by t=60-120s.

### The Insight

The scoring pipeline already works in keyword-only mode with zero configuration.
It produces reasonable results (tech content self-selects via keyword + dependency
matching). The onboarding wizard gates content behind configuration that isn't
strictly needed for first value.

### Changes

#### 1.1 — Quick Start bypass on Welcome screen
**File:** `src/components/onboarding/WelcomeStep.tsx`
- Add secondary button: "Skip to Content — configure later"
- On click: call `mark_onboarding_complete`, dismiss onboarding
- Keep primary button "Get Started" for full wizard path
- Visual: secondary/ghost button style, not competing with primary

#### 1.2 — Immediate analysis trigger
**File:** `src/App.tsx` (~line 252)
- Remove the 2-second `setTimeout` before `startAnalysis()`
- When onboarding is skipped, trigger analysis immediately
- Analysis starts fetching sources while user sees loading state

#### 1.3 — FirstRunTransition becomes dismissible overlay
**File:** `src/components/FirstRunTransition.tsx`
- Add "Browse Results Now" button visible from the start (not just celebration)
- Fetching/analyzing phases show real-time item count
- User can dismiss at any time to see partial results
- Celebration phase auto-dismisses after 5 seconds (not blocking)

#### 1.4 — Progressive result rendering
**File:** `src/App.tsx` or analysis event handler
- Listen for `partial-results` events during analysis
- Populate `relevanceResults` incrementally (every 50 items scored)
- Results view shows items as they arrive, sorted by relevance
- Loading indicator shows "Analyzing... 47/312 items scored"

#### 1.5 — Post-skip nudge in Briefing
**File:** `src/components/BriefingView.tsx`
- If `isFirstRun` and interests are empty: show gentle card
- "Want better results? Tell us what you care about" → opens Settings
- Card dismisses permanently after first interaction
- NOT blocking, NOT modal — inline card in briefing flow

#### 1.6 — Auto-detect interests from ACE post-analysis
**File:** `src/App.tsx` or settings initialization
- After first analysis completes, check ACE scan summary
- If detected_tech has entries and user has no interests: auto-populate
- Show toast: "Detected your tech stack: React, TypeScript, Rust"
- User can adjust in Settings anytime

### Verification

- [ ] Fresh install with no API key → content visible in <60 seconds
- [ ] Fresh install with Ollama running → embedding-mode results in <90 seconds
- [ ] Skip button works → onboarding_complete set, main app loads
- [ ] Full onboarding path still works for users who choose it
- [ ] Partial results render progressively during analysis
- [ ] ACE auto-detection populates interests on first run
- [ ] Returning users see no change (onboarding already complete)

---

## Phase 2: Close the Broken Feedback Loops

**Goal:** Upgrade from 5 to 7 closed feedback loops — the most autonomous
content intelligence engine in any desktop application.

### The Insight

Two systems collect valuable data but never analyze it:
1. Decision windows track user actions + lead times but don't learn from outcomes
2. Topic affinities are computed from interactions but never fed into scoring boosts

The infrastructure exists. The plumbing is there. We just need the pump.

### Changes

#### 2.1 — Decision Window Outcome Analyzer
**File:** `src-tauri/src/autophagy/decision_outcomes.rs` (NEW, ~200 lines)

```rust
pub struct DecisionWindowOutcome {
    pub window_type: String,        // security_patch, migration, adoption, knowledge
    pub dependency: Option<String>,
    pub windows_acted: i64,
    pub windows_expired: i64,
    pub response_rate: f32,         // acted / (acted + expired)
    pub avg_lead_time_hours: f32,
    pub confidence: f32,            // sqrt(sample_size / 20).min(1.0)
    pub sample_size: i64,
}
```

- Query `decision_windows` for last 30 days of outcomes
- Group by (window_type, dependency)
- Compute response_rate, avg_lead_time, confidence
- Store results to `digested_intelligence` with digest_type='decision_outcome'
- Supersede previous entries for same subject

#### 2.2 — Wire into Autophagy Cycle
**File:** `src-tauri/src/autophagy/digest.rs` (~line 79)
- Add `analyze_decision_window_outcomes()` as analyzer #6
- Add outcome count to `AutophagyCycleResult`
- Log: "Analyzed N decision window outcomes"

#### 2.3 — Bridge Topic Affinities into Scoring Boosts
**File:** `src-tauri/src/scoring/context.rs` (~line 145, after persona boost merging)

```rust
// Merge ACE topic affinities into feedback boosts
// Only high-confidence affinities (>0.6), significant signal (>0.2)
// Scaled to 50% weight vs explicit feedback (save/dismiss)
if let Ok(affinities) = ace.get_topic_affinities() {
    for aff in affinities {
        if aff.confidence > 0.6 && aff.affinity_score.abs() > 0.2 {
            let scaled = aff.affinity_score as f64 * 0.5;
            feedback_boosts
                .entry(aff.topic.to_lowercase())
                .and_modify(|v| *v = (*v + scaled).clamp(-0.5, 0.5))
                .or_insert(scaled.clamp(-0.5, 0.5));
        }
    }
}
```

- Affinity signals now influence scoring without explicit save/dismiss
- Browsing behavior (click, scroll, dwell) silently improves results
- Capped at ±0.5 to prevent runaway boosts

#### 2.4 — Decision Outcome Intelligence in Scoring
**File:** `src-tauri/src/scoring/context.rs`
- Load `decision_outcome` records from `digested_intelligence`
- Store as `ScoringContext.window_calibrations: HashMap<String, f32>`
- In pipeline Phase 6: if item would create a decision window, check
  historical response_rate for that window_type
- High response_rate → boost urgency. Low response_rate → dampen urgency.

#### 2.5 — Tests
**File:** `src-tauri/src/autophagy/decision_outcomes_tests.rs` (NEW, ~200 lines)
- Test empty window table → empty outcomes
- Test mixed acted/expired windows → correct response_rate
- Test confidence calculation → sqrt scaling
- Test storage to digested_intelligence → correct supersession
- Test affinity bridging → boosts appear in feedback_boosts

**File:** `src-tauri/src/scoring/context_tests.rs` (extend, ~150 lines)
- Test affinity merging with existing feedback boosts (additive, clamped)
- Test empty affinities → no change
- Test low-confidence affinities → filtered out

### Verification

- [ ] Autophagy cycle includes decision window analysis (check logs)
- [ ] `digested_intelligence` contains `decision_outcome` records after cycle
- [ ] Topic affinities appear in `feedback_boosts` after context build
- [ ] Scoring changes when affinities exist vs don't (measurable delta)
- [ ] All existing autophagy tests still pass
- [ ] All existing scoring tests still pass
- [ ] New tests: ≥10 tests covering both loops

### Feedback Loop Summary After Phase 2

| # | Loop | Status |
|---|------|--------|
| 1 | Autophagy Calibration (score vs engagement) | Working |
| 2 | Topic Decay (per-topic half-lives) | Working |
| 3 | Source Quality (engagement rate per source) | Working |
| 4 | ACE Context (project/dependency detection) | Working |
| 5 | Feedback Boosts (save/dismiss → topic scoring) | Working |
| 6 | **Decision Window Outcomes** | **NEW** |
| 7 | **Behavioral Topic Affinities** | **NEW** |

---

## Phase 3: STREETS Contextual Discovery

**Goal:** STREETS surfaces when it's relevant to what the user is reading,
not just as a tab they might never click. Make the personalization visible.

### The Insight

STREETS lesson completion feeds directly into scoring (topics → affinity signals
at 1.0 weight). Feed Echo blocks show relevant intelligence inside lessons.
But users don't know STREETS exists unless they click the tab. And when they
do use it, they don't realize content is personalized for them.

The connection between intelligence feed and STREETS is bidirectional — but invisible.

### Changes

#### 3.1 — Contextual STREETS suggestions in Briefing
**File:** `src/components/BriefingView.tsx` or new `StreetsContextCard.tsx`

When user has saved 3+ items in a topic cluster, show a card:

```
┌─────────────────────────────────────────────┐
│  📘 STREETS: Revenue Engines                │
│                                             │
│  You've saved 4 items about pricing and     │
│  monetization this week.                    │
│                                             │
│  Module R covers 8 ways to turn this        │
│  knowledge into income.                     │
│                                             │
│  [Open Module R]          [Not now]         │
└─────────────────────────────────────────────┘
```

- Query: match saved item topics against STREETS module descriptions
- Frequency cap: max 1 suggestion per day, max 3 per week
- "Not now" dismisses for 7 days. Never shows for completed modules.
- Only suggest modules the user hasn't started yet

**Backend support:**
**File:** `src-tauri/src/streets_engine.rs` or new command
- `get_streets_suggestion(topics: Vec<String>) -> Option<StreetsSuggestion>`
- Maps topic clusters to most relevant module
- Returns module_id, title, reason, match_strength
- Respects frequency caps (stored in settings or kv_store)

#### 3.2 — Personalization depth indicator
**File:** `src/components/playbook/PlaybookView.tsx` or lesson renderer

Make personalization visible:

```
┌─────────────────────────────────────────────┐
│  ✨ Personalized for your setup             │
│  Using: Ryzen 7 5800X · 32GB · RTX 3070    │
│  Feed: 3 matching articles from today       │
│  Depth: L3 (profile + intelligence feed)    │
└─────────────────────────────────────────────┘
```

- Show at top of each personalized lesson
- List the data sources used for personalization
- Depth badge: L1 (static), L2 (profile), L3 (insights), L4 (connections), L5 (temporal)
- Collapsible — users can hide it

#### 3.3 — Lesson completion celebration
**File:** `src/components/playbook/PlaybookView.tsx`

When a user completes a lesson:
- Animated checkmark (not just a toast)
- Show what the scoring engine learned: "Now tracking: infrastructure, local LLMs, GPU computing"
- Show next lesson preview
- If module complete: show module completion badge with stats

When a user completes a module:
- Module badge animation
- Progress summary: "Module S complete — 6/6 lessons, 12 topics learned"
- Suggest next module based on their feed topics

#### 3.4 — Feed Echo enhancement
**File:** `src/components/playbook/FeedEchoBlock.tsx` (existing)

Currently shows relevant feed items inside lessons. Enhance:
- Show score and match reason for each echoed item
- "Save" button directly in the echo block (saves to main feed)
- Count badge: "3 new items since last visit"
- Make it visually distinct from static content (card with border)

### Verification

- [ ] Save 3+ items about a topic → STREETS suggestion appears in Briefing
- [ ] Suggestion links to correct module, opens correctly
- [ ] "Not now" dismisses for 7 days
- [ ] Completed modules never suggested
- [ ] Frequency cap: max 1/day, 3/week
- [ ] Personalization indicator shows at top of personalized lessons
- [ ] Lesson completion shows topics learned
- [ ] Module completion shows badge and next module suggestion
- [ ] Feed Echo blocks have save buttons and count badges

---

## Phase 4: Scoring Validation

**Goal:** Prove the scoring engine works before any external user touches it.
Systematic internal validation using persona simulation.

### The Insight

18,047 lines of scoring infrastructure (29% of Rust code) have never been
validated by anyone outside this team. The engine has 5 (soon 7) feedback loops,
a simulation corpus, a benchmark framework. We can use all of this to validate
precision before launch.

### Method

#### 4.1 — Define 10 Developer Personas
**File:** `src-tauri/src/scoring/simulation/personas.rs` (NEW, ~200 lines)

```rust
pub struct SimulatedPersona {
    pub name: &'static str,
    pub interests: Vec<&'static str>,
    pub tech_stack: Vec<&'static str>,
    pub dependencies: Vec<&'static str>,
    pub role: &'static str,
    pub expected_topics: Vec<&'static str>,     // Should score high
    pub anti_topics: Vec<&'static str>,         // Should score low
}
```

10 personas:
1. **Rust Systems Dev** — Rust, async, networking, OS, performance
2. **React Frontend** — React, TypeScript, CSS, UI/UX, web performance
3. **ML Engineer** — Python, PyTorch, transformers, MLOps, GPU computing
4. **DevOps/Platform** — Kubernetes, Terraform, CI/CD, monitoring, cloud
5. **Mobile Dev** — Swift, Kotlin, React Native, Flutter, mobile UX
6. **Security Engineer** — CVEs, pentesting, cryptography, supply chain
7. **Data Engineer** — SQL, Spark, Kafka, dbt, data pipelines
8. **Indie Hacker** — SaaS, pricing, growth, marketing, revenue
9. **Game Dev** — Unity, Unreal, Godot, graphics, physics
10. **Backend Java/Go** — microservices, gRPC, databases, enterprise

#### 4.2 — Run Scoring Pipeline Per Persona
**File:** `src-tauri/src/scoring/simulation/validation.rs` (NEW, ~300 lines)

For each persona:
1. Build a `ScoringContext` with the persona's interests/stack/deps
2. Score the last 500 fetched items through pipeline_v2
3. Take top 20 results (precision@20)
4. Auto-judge: does each result's topics overlap with `expected_topics`?
5. Check: do any `anti_topics` appear in top 20?

```rust
pub struct ValidationResult {
    pub persona: String,
    pub precision_at_20: f32,       // % of top 20 matching expected topics
    pub anti_topic_leaks: usize,    // count of anti-topics in top 20
    pub avg_score_relevant: f32,    // mean score of matching items
    pub avg_score_irrelevant: f32,  // mean score of non-matching items
    pub separation: f32,            // relevant_avg - irrelevant_avg (higher = better)
}
```

#### 4.3 — Aggregate and Report
**File:** `src-tauri/src/scoring/simulation/validation.rs`

```rust
pub struct ValidationReport {
    pub timestamp: String,
    pub personas: Vec<ValidationResult>,
    pub overall_precision: f32,     // mean precision across all personas
    pub worst_persona: String,      // lowest precision
    pub best_persona: String,       // highest precision
    pub separation_score: f32,      // mean separation (relevant vs irrelevant)
    pub recommendations: Vec<String>,
}
```

Target thresholds:
- precision@20 ≥ 0.70 per persona (14/20 relevant)
- separation ≥ 0.15 (relevant items score meaningfully higher)
- anti_topic_leaks ≤ 2 per persona

#### 4.4 — Tauri Command for Validation
**File:** `src-tauri/src/scoring/simulation/mod.rs`

```rust
#[tauri::command]
pub async fn run_scoring_validation() -> Result<ValidationReport> { ... }
```

- Can be triggered from Settings → Advanced or via MCP tool
- Returns full report with per-persona breakdown
- Stores results in `digested_intelligence` for trend tracking

### Verification

- [ ] 10 personas defined with distinct interest profiles
- [ ] Validation runs against real fetched content (not synthetic)
- [ ] precision@20 ≥ 0.70 for ≥8 of 10 personas
- [ ] separation ≥ 0.15 overall
- [ ] anti_topic_leaks ≤ 2 per persona
- [ ] Report identifies weakest persona for targeted tuning
- [ ] Validation can re-run after tuning to measure improvement

---

## Phase 5: Ghost Command Cleanup

**Goal:** Reduce attack surface from 252 to ~170 commands without deleting code.
Feature-flag unintegrated systems behind compile-time flags.

### The Insight

76 commands (30%) have zero frontend callers. They compile, register, expand
attack surface, and deliver zero value. Don't delete them — gate them.

### Changes

#### 5.1 — Create feature flags
**File:** `src-tauri/Cargo.toml`

```toml
[features]
default = []
ocr = ["dep:ocrs", "dep:rten", "dep:image"]
archive = ["dep:zip", "dep:tar", "dep:flate2"]
team-sync = []
enterprise = []
game-engine = []
streets-execution = []
void-universe = []
```

#### 5.2 — Gate Team Sync commands
**Files:** `src-tauri/src/team_sync*.rs`, `src-tauri/src/lib.rs`

- Wrap all team_sync modules with `#[cfg(feature = "team-sync")]`
- Remove from invoke_handler unless feature enabled
- Keep module files unchanged (just add cfg gate)

Commands gated: ~20 (create_team, join_team, share_signal, etc.)

#### 5.3 — Gate Enterprise commands
**Files:** `src-tauri/src/enterprise*.rs`, `src-tauri/src/audit*.rs`, `src-tauri/src/webhooks.rs`

- Wrap enterprise, audit, webhook modules with `#[cfg(feature = "enterprise")]`
- Remove from invoke_handler unless feature enabled

Commands gated: ~15 (register_webhook, export_audit, get_organization, etc.)

#### 5.4 — Gate GAME Engine commands
**Files:** `src-tauri/src/game_engine.rs`, `src-tauri/src/game_commands.rs`

- Wrap with `#[cfg(feature = "game-engine")]`

Commands gated: 3 (get_game_state, get_achievements, check_daily_streak)

#### 5.5 — Gate STREETS Execution commands
**Files:** `src-tauri/src/streets_commands.rs` (execution subset)

- Gate only the execution commands: execute_lesson_commands, execute_streets_command, parse_lesson_commands
- Keep playbook content commands (get_playbook_modules, etc.) always available

Commands gated: 3

#### 5.6 — Prune unused Toolkit commands
**Files:** `src-tauri/src/toolkit*.rs`

- Gate 7 ghost toolkit commands behind a `toolkit-advanced` feature
- Keep the 2 working commands (toolkit_scan, toolkit_status or whichever are active)

Commands gated: 7

### Verification

- [ ] `cargo build` with default features → compiles with ~170 commands
- [ ] `cargo build --all-features` → compiles with all 252 commands
- [ ] `cargo test --lib` passes with default features
- [ ] `cargo test --lib --all-features` passes with all features
- [ ] Frontend works normally (no IPC failures — all active commands still registered)
- [ ] Binary size decreases measurably with fewer features

---

## Phase 6: STREETS Content Polish

**Goal:** Address the gaps identified in the content audit. Ensure every module
is accurate, current, and professionally complete.

### Audit Summary (from deep content review)

Overall: 8.5/10. Content is genuinely strong — accurate pricing, real benchmarks,
practical advice. Not hype. The following are the identified gaps:

### Changes

#### 6.1 — Ollama version references
**File:** `docs/streets/module-s-sovereign-setup.md`
- Verify Ollama version references are current (currently pinned to 0.3.x)
- Update any outdated CLI flags or model names
- Add note about checking ollama.ai for latest version

#### 6.2 — Platform risk discussion
**File:** `docs/streets/module-r-revenue-engines.md`
- Add section to Lesson 6 or 7: "Platform Risk & Diversification"
- Cover: What happens when Vercel changes affiliate rates, YouTube demonetizes,
  Gumroad is acquired
- Practical: "Never have >40% of income from one platform"
- Template addition: add "Platform Dependency Audit" to Stream Stack template

#### 6.3 — Post-launch scaling depth
**File:** `docs/streets/module-e2-evolving-edge.md`
- Strengthen "when to kill a stream" decision framework
- Add "scaling from $500/mo to $10K/mo" practical playbook
- Cover: hiring first contractor, automating fulfillment, raising prices

#### 6.4 — Tax/legal regional depth
**File:** `docs/streets/regions/*.json`
- Expand regional data with:
  - VAT/GST thresholds per region
  - Digital product tax treatment
  - Common business entity comparison (LLC vs Ltd vs Pty Ltd vs GmbH)
- Add disclaimer: "This is general information, not tax advice"

#### 6.5 — Personalization depth increase
**Files:** All `docs/streets/module-*.md` files
- Add branching conditionals for experience level:
  ```
  {? if computed.experience_years < 3 ?}
  Start with Digital Products — lowest risk, fastest feedback loop.
  {? elif computed.experience_years < 8 ?}
  Your experience unlocks Consulting and API Products — higher margins.
  {? else ?}
  At your level, focus on Open Source+ and Data Products — compound over time.
  {? endif ?}
  ```
- Add stack-specific recommendations:
  ```
  {? if profile.stack contains "react" ?}
  React developers have strong demand for: UI component libraries,
  Next.js templates, design system tooling.
  {? endif ?}
  ```

#### 6.6 — Video manifest cleanup
**File:** `docs/streets/video-manifest.json`
- Videos are referenced but don't exist — this is intentional (future content)
- Add `"status": "planned"` field to each entry
- Add `"placeholder_text"` for what the video will cover
- Ensure no broken references in the UI (FeedEchoBlock or lesson renderer)

### Verification

- [ ] All Ollama references verified against current CLI
- [ ] Platform risk section added to Module R
- [ ] Post-launch scaling content added to Module E2
- [ ] Regional data expanded with VAT/GST info
- [ ] Experience-level branching added to ≥3 modules
- [ ] Stack-specific recommendations added to Module R
- [ ] Video manifest has status field, no broken references
- [ ] All personalization blocks render correctly ({? if ?} syntax valid)
- [ ] Markdown renders correctly in PlaybookView

---

## Execution Order

```
Phase 1: Content-First Entry          ← Do first (highest user impact)
Phase 2: Close the Broken Loops       ← Can parallel with Phase 1
Phase 3: STREETS Contextual Discovery ← Depends on Phase 2 (uses topic data)
Phase 4: Scoring Validation           ← Independent, do anytime
Phase 5: Ghost Command Cleanup        ← Independent, do anytime
Phase 6: STREETS Content Polish       ← Independent, do anytime
```

### Recommended Parallel Execution

```
Session A: Phase 1 + Phase 5     (frontend + Cargo.toml, no overlap)
Session B: Phase 2               (Rust backend, scoring pipeline)
Session C: Phase 3 + Phase 6     (frontend + content, STREETS focus)
Session D: Phase 4               (scoring validation, independent)
```

---

## Success Criteria

### Content-First
- [ ] Time to first content: <60 seconds (fresh install, no API key)
- [ ] Time to personalized content: <5 minutes (with ACE auto-detection)

### Self-Sustaining
- [ ] 7 closed feedback loops operational
- [ ] Scoring improves measurably after 50 interactions (A/B test)
- [ ] Decision windows learn from outcomes (response_rate tracked)

### STREETS Professional
- [ ] Content audit score: ≥9/10 (from current 8.5)
- [ ] Personalization branches on experience level and tech stack
- [ ] Contextual suggestions surface in Briefing (not just tab)
- [ ] Lesson completion shows visible learning confirmation

### Scoring Validated
- [ ] precision@20 ≥ 0.70 for 8+ personas
- [ ] separation ≥ 0.15 (relevant vs irrelevant gap)
- [ ] Anti-topic leaks ≤ 2 per persona

### Lean Binary
- [ ] Default features: ~170 commands (down from 252)
- [ ] All features: 252 commands (nothing deleted)
- [ ] All tests pass on both configurations

---

## What This Achieves

After all 6 phases, 4DA is:

1. **Instantly valuable** — content in 60 seconds, not 15 minutes
2. **Self-improving** — 7 feedback loops, every interaction makes it smarter
3. **Contextually deep** — STREETS surfaces when relevant, not when clicked
4. **Validated** — scoring accuracy proven across 10 developer profiles
5. **Lean** — 30% fewer commands in the binary, zero deleted code
6. **Professional** — STREETS content at 9/10, personalized by experience and stack

The system works independently for a single developer. When Team/Enterprise
enables later, the individual intelligence compounds into collective intelligence.
The foundation is self-sustaining. The trajectory is compounding.
