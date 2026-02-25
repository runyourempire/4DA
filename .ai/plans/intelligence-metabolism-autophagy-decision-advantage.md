# Intelligence Metabolism: Autophagy + Decision Advantage

## Context

4DA currently performs a **dumb DELETE** of old content (`monitoring.rs:302` → `DELETE FROM source_items WHERE last_seen < -30 days`). Items are destroyed without extracting any meta-intelligence. This is a missed opportunity — the gap between what the system scored and how the user actually engaged is the highest-quality calibration data in the entire system, and it's thrown away every day.

Simultaneously, 4DA surfaces relevant content but doesn't surface the **temporal competitive edge** — the advantage of seeing something before it matters, before others see it, or before a decision window closes.

This plan introduces two interconnected systems forming an **intelligence metabolism cycle**:

1. **Intelligent Autophagy** — digest content before pruning to extract calibration deltas, topic decay rates, source quality intelligence, and anti-patterns
2. **Decision Advantage** — detect time-bounded decision windows from signal chains + dependency intelligence, track user response, build a compound advantage score
3. **Metabolism Connection** — autophagy calibration improves scoring → better decision window detection → closed windows trigger targeted autophagy

---

## Phase 1: Database Schema (Migration v17)

**File:** `src-tauri/src/db/migrations.rs`
- Change `TARGET_VERSION` from `16` to `17`
- Add Phase 17 migration block after Phase 16

### New Tables

**`digested_intelligence`** — polymorphic intelligence store
- `id`, `digest_type` (calibration/topic_decay/source_autopsy/anti_pattern), `subject`, `data` (JSON), `confidence`, `sample_size`, `created_at`, `expires_at`, `superseded_by` (FK self)
- Indexes: `(digest_type, subject)`, `(created_at)`
- JSON payloads per type:
  - **calibration**: `{ "scored_avg": 0.65, "engaged_avg": 0.72, "delta": +0.07, "sample_size": 42, "period_days": 30 }`
  - **topic_decay**: `{ "half_life_hours": 72, "peak_relevance_age_hours": 6, "engagement_rate_by_age": {"0-24h": 0.32, "24-72h": 0.18, "72h+": 0.04} }`
  - **source_autopsy**: `{ "source_type": "hackernews", "topic": "rust", "items_surfaced": 15, "items_engaged": 4, "engagement_rate": 0.27 }`
  - **anti_pattern**: `{ "pattern_type": "over_scored", "avg_score": 0.68, "engagement_count": 0, "exposure_count": 12, "correction_direction": "reduce", "suggested_penalty": -0.10 }`

**`autophagy_cycles`** — cycle execution metrics
- `id`, `items_analyzed`, `items_pruned`, `calibrations_produced`, `topic_decay_rates_updated`, `source_autopsies_produced`, `anti_patterns_detected`, `db_size_before_bytes`, `db_size_after_bytes`, `duration_ms`, `created_at`

**`decision_windows`** — time-bounded decision opportunities
- `id`, `window_type` (security_patch/migration/adoption/knowledge), `title`, `description`, `urgency` (0-1), `relevance`, `source_item_ids` (JSON), `signal_chain_id`, `dependency`, `status` (open/acted/expired/closed), `opened_at`, `expires_at`, `acted_at`, `closed_at`, `outcome`, `lead_time_hours`, `streets_engine`
- Indexes: `(status)`, `(window_type)`, `(dependency)`

**`advantage_score`** — compound advantage metric history
- `id`, `period` (daily/weekly/monthly), `score`, `items_surfaced`, `avg_lead_time_hours`, `windows_opened`, `windows_acted`, `windows_expired`, `knowledge_gaps_closed`, `calibration_accuracy`, `computed_at`
- Index: `(period, computed_at)`

---

## Phase 2: Intelligent Autophagy (Rust Backend)

### New module: `src-tauri/src/autophagy/`

**`mod.rs`** (~100 lines) — public API, types, re-exports

**`calibration.rs`** (~250 lines) — score-vs-reality extraction
- `analyze_calibration(conn, ace_conn, max_age_days) -> Vec<CalibrationDelta>`
- Selects items in pruning window (last_seen between -23d and -30d)
- Cross-references ACE `interactions` table for actual engagement
- Groups by topic: computes scored_avg vs engaged_avg → delta
- Positive delta = system under-scored things users liked
- Confidence = min(1.0, sample_size / 20.0)
- `store_calibrations()` supersedes previous calibrations for same topic
- `get_latest_calibration(conn, topic)` for scoring pipeline

**`topic_decay.rs`** (~200 lines) — topic-aware decay rate intelligence
- `analyze_topic_decay(conn, ace_conn) -> Vec<TopicDecayProfile>`
- Joins source_items with interactions, buckets engagement by content age
- Computes per-topic half-life (where engagement drops to 50% of peak)
- Default profile: 72h half-life, 6h peak
- Security content → longer half-life; hype content → shorter

**`source_autopsy.rs`** (~200 lines) — per-source per-topic engagement quality
- `analyze_sources(conn, ace_conn, max_age_days) -> Vec<SourceAutopsy>`
- For each source_type x topic: items_surfaced, items_engaged, engagement_rate, avg_score
- More nuanced than the existing EMA-based source_preferences

**`anti_patterns.rs`** (~200 lines) — systematic over/under-scoring detection
- `detect_anti_patterns(conn, ace_conn, threshold) -> Vec<AntiPattern>`
- OverScored: topic scored > threshold for 5+ items, engagement_rate < 5%
- UnderScored: saved/clicked items scored below threshold
- `apply_corrections()` — strengthens ACE anti_topics or weakens penalties

**`digest.rs`** (~300 lines) — orchestrator
- `run_autophagy_cycle(max_age_days) -> Result<AutophagyCycleResult>`
- Runs all 4 analyzers in sequence
- Records cycle metrics to `autophagy_cycles`
- Applies anti-pattern corrections to ACE
- Delegates to existing `db.cleanup_old_items()` for actual pruning
- Computes compound advantage score after cycle
- Falls back to dumb cleanup if any analyzer fails

### New file: `src-tauri/src/autophagy_commands.rs` (~80 lines)
- `get_autophagy_status()` — latest cycle result + calibration summary
- `get_autophagy_history(limit)` — cycle history

### Scheduler Integration

**File:** `src-tauri/src/monitoring.rs`

Replace the cleanup block at lines 296-315 with autophagy call:
```rust
// Intelligent Autophagy - extract intelligence then prune (replaces dumb DELETE)
match crate::autophagy::run_autophagy_cycle(max_age_days).await {
    Ok(result) => { /* log autophagy metrics */ }
    Err(e) => { /* warn, fallback to db.cleanup_old_items() */ }
}
```

The autophagy cycle runs inside the existing daily `BEHAVIOR_DECAY_INTERVAL` block, replacing the cleanup section. No new scheduler timestamp needed — piggybacks on the existing daily job.

---

## Phase 3: Decision Advantage (Rust Backend)

### New module: `src-tauri/src/decision_advantage/`

**`mod.rs`** (~120 lines) — types, re-exports, ts-rs derives

**`windows.rs`** (~350 lines) — window detection + lifecycle
- `detect_decision_windows(conn) -> Vec<DecisionWindow>` — scans recent scored items + signal chains + deps
  - **SecurityPatch**: signal_type=security_alert + dependency match → urgency 0.80-0.95, expires 7d
  - **Migration**: signal_type=breaking_change + dependency match → urgency by version delta, expires 30d
  - **Adoption**: signal_type=tool_discovery + stack-adjacent → urgency by chain length, expires 14d
  - **Knowledge**: knowledge gap severity escalating → urgency from severity, no expiry
- `get_open_windows(conn)` — cached per analysis run via ScoringContext
- `transition_window(conn, id, status, outcome)` — user acted/closed
- `expire_stale_windows(conn)` — auto-expire past expires_at
- Deduplication: no duplicate windows for same dependency+type

**`compound_score.rs`** (~200 lines) — running advantage metric
- `compute_compound_score(conn, period) -> CompoundAdvantageScore`
- Components: window response rate (0.30 weight), avg lead time (0.20), calibration accuracy from autophagy (0.25), knowledge gap closure (0.15), items surfaced log-scaled (0.10)
- Score grows as system learns — compounding proof of value

**`scoring_boost.rs`** (~150 lines) — pipeline integration
- `compute_decision_window_boost(open_windows, title, content, topics, matched_deps) -> (f32, Option<i64>)`
- Returns boost [0.0, 0.20] and matched window_id
- Security windows = 0.20, migration = 0.15, adoption/knowledge = 0.10
- Matches by: dependency name overlap, topic overlap, title keywords

### New file: `src-tauri/src/decision_advantage_commands.rs` (~120 lines)
- `get_decision_windows()` — open windows
- `act_on_decision_window(window_id, outcome)` — mark acted
- `close_decision_window(window_id)` — manual close
- `get_compound_advantage()` — current score

### Scheduler Integration

**File:** `src-tauri/src/monitoring.rs`

Add `last_window_check: AtomicU64` to `MonitoringState` (after line 55).
Add `const WINDOW_CHECK_INTERVAL: u64 = 3600` (1 hour).
Add hourly check in scheduler loop (after anomaly check):
```rust
// Decision window detection - every hour
let new_windows = decision_advantage::detect_decision_windows(&conn);
decision_advantage::expire_stale_windows(&conn);
app.emit("decision-windows-updated", new_windows.len());
```

---

## Phase 4: Scoring Pipeline Integration

**File:** `src-tauri/src/scoring/mod.rs`

### ScoringContext additions (line 62-84)
```rust
pub open_windows: Vec<decision_advantage::DecisionWindow>,
pub calibration_deltas: HashMap<String, f32>,     // topic -> delta
pub topic_half_lives: HashMap<String, f32>,        // topic -> half_life_hours
```

### build_scoring_context() additions (after line 148)
```rust
let open_windows = decision_advantage::get_open_windows(&conn);
let calibration_deltas = autophagy::load_calibration_deltas(&conn);
let topic_half_lives = autophagy::load_topic_decay_profiles(&conn);
```
All graceful no-ops when no digested data exists (return empty maps).

### score_item() — Decision window boost (after line 466, before line 468)
```rust
let (window_boost, matched_window_id) = if !ctx.open_windows.is_empty() {
    decision_advantage::compute_decision_window_boost(
        &ctx.open_windows, input.title, input.content, &topics, &matched_dep_names,
    )
} else { (0.0, None) };
let base_score = (base_score + window_boost).clamp(0.0, 1.0);
```

### Topic-aware freshness (explanation.rs, enhance `compute_temporal_freshness`)
```rust
pub(crate) fn compute_temporal_freshness_with_profile(
    created_at: &DateTime<Utc>,
    half_life_hours: Option<f32>,
) -> f32 {
    match half_life_hours {
        Some(hl) => {
            let age_hours = ((Utc::now() - *created_at).num_minutes() as f32 / 60.0).max(0.0);
            0.80 + 0.30 * 0.5_f32.powf(age_hours / hl) // maps to [0.80, 1.10]
        }
        None => compute_temporal_freshness(created_at),
    }
}
```
Called from score_item() at the freshness stage — looks up dominant topic in `ctx.topic_half_lives`, falls back to existing fixed tiers.

### ScoreBreakdown additions
**File:** `src-tauri/src/types.rs` — add `window_boost: f32`, `matched_window_id: Option<i64>`

---

## Phase 5: VoidSignal + Events

**File:** `src-tauri/src/void_engine.rs`

Add 3 fields to `VoidSignal` struct (after line 36):
```rust
pub metabolism: f32,        // 0=no autophagy data, 1=fully calibrated
pub open_windows: u32,      // count of open decision windows
pub advantage_trend: f32,   // -1 declining, 0 stable, +1 growing
```

Update `Default`, `differs_from()`, and the signal computation functions.

**Frontend mirror:** `src/types/common.ts` — add matching fields to VoidSignal interface.

**Heartbeat integration:** `src/components/void-engine/VoidHeartbeat.tsx`
- `metabolism` influences glow radius (calibrated system = subtle healthy glow)
- `open_windows > 0` increases pulse speed (something needs attention)

### Tauri Events
- `autophagy-cycle-complete` — emitted after each digest cycle with `AutophagyCycleResult`
- `decision-windows-updated` — emitted hourly when windows change (count)

---

## Phase 6: Frontend

### New types: `src/types/autophagy.ts` (~60 lines)
- `AutophagyCycle`, `CalibrationDelta`, `DecisionWindow`, `CompoundAdvantageScore`

### New Zustand slices
- `src/store/autophagy-slice.ts` (~80 lines) — cycles, calibrations, load actions
- `src/store/decision-advantage-slice.ts` (~100 lines) — windows, compound score, act/close actions

### New components
- **`DecisionWindowsPanel.tsx`** (~250 lines) — cards for open windows with type badge (color-coded), urgency bar, time remaining, Act/Dismiss buttons, STREETS engine badge. **FREE tier.**
- **`CompoundAdvantageScore.tsx`** (~100 lines) — compact score (0-100), trend arrow, 7-day sparkline. **FREE tier.**
- **`AutophagyInsights.tsx`** (~200 lines) — calibration heatmap, decay rates, source quality, anti-patterns. **PRO-gated** via `<ProGate>`.

### View integration
- `BriefingView` — add `DecisionWindowsPanel` below signal action cards, add `CompoundAdvantageScore` near `EngagementPulse`
- `Insights view` (activeView==='insights') — add `AutophagyInsights` panel (Pro-gated)

### ResultItem enhancement
- When `score_breakdown.matched_window_id` exists, show a decision window badge on the item

---

## Phase 7: MCP Server Tools (3 new tools)

**File:** `mcp-4da-server/src/index.ts` + new tool files

1. **`autophagy_status`** — digest cycle history, calibration health, topic decay profiles
2. **`decision_windows`** — list open/closed windows, window lifecycle management
3. **`compound_advantage`** — compound advantage score, decision velocity metrics

---

## Phase 8: Module Registration + Commands

**File:** `src-tauri/src/lib.rs`
- Add `pub mod autophagy;` and `pub mod decision_advantage;`
- Add `mod autophagy_commands;` and `mod decision_advantage_commands;`
- Register 6 new Tauri commands in `generate_handler!` macro

---

## Pro Gating

| Feature | Tier | Rationale |
|---------|------|-----------|
| Autophagy engine (better scoring for everyone) | FREE | Core product improvement |
| Decision windows panel | FREE | Drives engagement, makes product stickier |
| Compound advantage score | FREE | Motivational metric, visible compounding value |
| Autophagy insights dashboard | PRO | Deep analytics, power user feature |
| Topic decay rate details | PRO | Deep analytical view |
| Calibration heatmap | PRO | Deep analytical view |

## STREETS Alignment

| Window Type | Engine | Rationale |
|-------------|--------|-----------|
| security_patch | Automation | Automated security monitoring |
| migration | Consulting | Migration advice/planning |
| adoption | Digital Products | Tool evaluation/adoption |
| knowledge | Education | Knowledge gap closure |

---

## Implementation Order

```
Phase 1: Schema v17          (foundation — enables everything)
    |
    ├── Phase 2: Autophagy   (can test independently)
    ├── Phase 3: Decision Advantage (can test independently)
    |
    └── Phase 4: Scoring Integration (wires both into pipeline)
            |
            ├── Phase 5: VoidSignal (small, depends on Phase 4)
            ├── Phase 6: Frontend (visual layer)
            ├── Phase 7: MCP tools (external API)
            └── Phase 8: Registration + wiring
```

Phases 2 and 3 are independent and can be built in parallel. Phase 4 connects them. Phases 5-8 are the integration layer.

---

## File Size Compliance

All new Rust files < 350 lines (well under 600 warn). All new TSX files < 250 lines (well under 350 warn). No existing file grows beyond its current size class.

## New Files Summary

| File | Lines | Purpose |
|------|-------|---------|
| `src-tauri/src/autophagy/mod.rs` | ~100 | Module root, public API |
| `src-tauri/src/autophagy/calibration.rs` | ~250 | Score-vs-reality extraction |
| `src-tauri/src/autophagy/topic_decay.rs` | ~200 | Topic half-life computation |
| `src-tauri/src/autophagy/source_autopsy.rs` | ~200 | Per-source quality analysis |
| `src-tauri/src/autophagy/anti_patterns.rs` | ~200 | Over/under-scoring detection |
| `src-tauri/src/autophagy/digest.rs` | ~300 | Digest cycle orchestrator |
| `src-tauri/src/autophagy_commands.rs` | ~80 | Tauri commands |
| `src-tauri/src/decision_advantage/mod.rs` | ~120 | Module root, types |
| `src-tauri/src/decision_advantage/windows.rs` | ~350 | Window detection + lifecycle |
| `src-tauri/src/decision_advantage/compound_score.rs` | ~200 | Advantage metric |
| `src-tauri/src/decision_advantage/scoring_boost.rs` | ~150 | Scoring pipeline boost |
| `src-tauri/src/decision_advantage_commands.rs` | ~120 | Tauri commands |
| `src/types/autophagy.ts` | ~60 | Frontend types |
| `src/store/autophagy-slice.ts` | ~80 | Zustand slice |
| `src/store/decision-advantage-slice.ts` | ~100 | Zustand slice |
| `src/components/DecisionWindowsPanel.tsx` | ~250 | Decision windows UI |
| `src/components/CompoundAdvantageScore.tsx` | ~100 | Advantage score widget |
| `src/components/AutophagyInsights.tsx` | ~200 | Autophagy dashboard (Pro) |

## Modified Files Summary

| File | Change |
|------|--------|
| `src-tauri/src/db/migrations.rs` | v17 migration (4 new tables) |
| `src-tauri/src/monitoring.rs` | Replace cleanup with autophagy, add window check job |
| `src-tauri/src/scoring/mod.rs` | 3 new ScoringContext fields, window boost injection |
| `src-tauri/src/scoring/explanation.rs` | Topic-aware freshness function |
| `src-tauri/src/types.rs` | ScoreBreakdown additions |
| `src-tauri/src/void_engine.rs` | 3 new VoidSignal dimensions |
| `src-tauri/src/lib.rs` | Module registration + commands |
| `src/types/common.ts` | VoidSignal frontend mirror |
| `src/types/analysis.ts` | ScoreBreakdown frontend mirror |
| `src/store/index.ts` | Add 2 new slices |
| `src/App.tsx` | Wire new event listeners |
| `src/components/BriefingView.tsx` | Add DecisionWindowsPanel + CompoundAdvantageScore |
| `src/components/void-engine/VoidHeartbeat.tsx` | Metabolism + window pulse |
| `mcp-4da-server/src/index.ts` | 3 new tools |

## Testing Strategy

Each Rust module includes `#[cfg(test)]` with in-memory SQLite + v17 schema. Key test scenarios:
- Calibration: positive/negative deltas, confidence scaling, supersession
- Topic decay: default fallback, security slow-decay, hype fast-decay
- Anti-patterns: over-scored/under-scored detection, minimum sample size
- Digest cycle: empty DB graceful no-op, full cycle with data, fallback on error
- Decision windows: detection per type, deduplication, expiry, transition lifecycle
- Compound score: zero-data baseline, improvement with actions, penalty for expiry
- Scoring boost: no-op without windows, correct boost per window type, cap at 0.20

## Verification

1. `cargo test` — all new unit tests pass
2. `pnpm run test` — frontend tests pass
3. `pnpm run tauri dev` — app boots, autophagy cycle runs on first daily tick
4. Manual: trigger analysis, check that decision windows appear for deps with security/breaking signals
5. Manual: wait for daily cycle, verify `autophagy_cycles` table has a row with non-zero intelligence counts
6. Manual: check BriefingView shows DecisionWindowsPanel and CompoundAdvantageScore
7. `pnpm run validate:sizes` — all files within limits

---

## Conceptual Foundation

**Intelligent Autophagy**: In biology, autophagy is the cell eating its own damaged components to fuel renewal. For 4DA, this means the act of pruning old content IS the learning opportunity. Before any item is deleted, its death generates signal: the delta between predicted score and actual engagement is pure calibration gold. The system literally metabolizes its own history into sharper judgment.

**Decision Advantage**: Boyd's OODA loop — the entity that observes, orients, decides, and acts fastest wins. 4DA's unique ACE context means it can surface signals filtered through YOUR specific stack that no public aggregator can match. Decision windows crystallize this into time-bounded opportunities where early action creates disproportionate value.

**Intelligence Metabolism**: The combined system forms a self-reinforcing cycle. Autophagy produces calibration data → scoring improves → decision window detection improves → closed windows trigger targeted autophagy → the system gets leaner AND smarter with each cycle. This is the self-sustaining sun: self-generating energy through an internal fusion reaction of data consumption and intelligence production.
