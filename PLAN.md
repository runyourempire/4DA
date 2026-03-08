# Pro Tier — Make It Irresistible

**Objective:** Transform 4DA Pro from "nice analytics panels" into an irresistible upgrade with visible daily value, intelligent free-tier limits, and a flagship search feature.

**Outcome:** 13+ working Pro features, 3 usage gates, zero vapor promises.

---

## Phase 1: Wire & Clean (30 min)

Quick wins that immediately improve the Pro package.

### 1.1 Wire `get_semantic_shifts` into Tauri

**Files:** `src-tauri/src/semantic_diff.rs`, `src-tauri/src/lib.rs`

- Add `#[tauri::command]` attribute above the function (line ~244 in semantic_diff.rs)
- Make function `pub async` with correct Tauri signature
- Register in lib.rs invoke_handler after `resolve_signal_chain` (~line 355)
- Already has `require_pro_feature` call — no gate work needed
- Already in PRO_FEATURES list — no license.rs changes needed

**Verify:** `cargo build` passes, command callable from frontend

### 1.2 Clean vapor features from PRO_FEATURES

**File:** `src-tauri/src/settings/license.rs`

- Remove `"generate_audio_briefing"` from PRO_FEATURES (no backend, no UI)
- Remove `"get_predicted_context"` from PRO_FEATURES (no backend, no UI)
- Remove `"generate_context_packet"` from PRO_FEATURES (no backend, no UI)
- Keep `"natural_language_query"` — implementing in Phase 2

**Result:** PRO_FEATURES list is honest — every entry has a working backend

### 1.3 Clean game-components cache bust

**File:** `src/lib/game-components.ts`, `src/lib/game-components/types.d.ts`

- Remove `?v=2` query param from signal-waveform import
- Remove duplicate `declare module` in types.d.ts
- Components reload correctly on dev server restart (no hack needed)

---

## Phase 2: Natural Language Search (2-3 hours)

The flagship Pro feature. Frontend already exists and works — needs backend only.

### 2.1 Create `natural_language_query` command

**New file:** `src-tauri/src/natural_language_search.rs`

**Architecture:**
```
User query -> Local intent parser -> SQL/FTS query -> Optional LLM enhancement -> Results
```

**Local intent parser (no LLM required):**
- Extract keywords (strip stop words, stem)
- Detect time ranges ("last week", "from last month", "yesterday")
- Detect file types ("pdfs", "images", "xlsx files")
- Detect intent (find/summarize/compare/timeline/count)
- Compute confidence score

**Query execution:**
- FTS5 full-text search on `source_items.title` and `source_items.content`
- If embeddings available: sqlite-vec KNN similarity search
- Apply time range filter (`created_at > ?`)
- Apply source_type filter if file types detected
- Combine FTS + vector scores, rank by relevance
- Cap at 20 results

**Optional LLM enhancement (BYOK/Ollama):**
- If LLM available: send query to LLM for better intent extraction
- System prompt: "Extract search intent from this natural language query. Return JSON with: keywords, time_range, file_types, intent, entities"
- Graceful fallback to local parser if LLM unavailable

**Return type (matches existing frontend interface):**
```rust
struct QueryResult {
    query: String,
    intent: String,
    items: Vec<QueryResultItem>,
    total_count: usize,
    execution_ms: u64,
    summary: Option<String>,
    parsed: ParsedQuery,
}
```

**Pro gate:** `require_pro_feature("natural_language_query")?` at top

### 2.2 Register command

**File:** `src-tauri/src/lib.rs`

- Add `mod natural_language_search;` declaration
- Add `natural_language_search::natural_language_query` to invoke_handler

### 2.3 Add NL Search ProGate in frontend

**File:** `src/components/NaturalLanguageSearch.tsx`

- Wrap the search input area with `<ProGate feature="natural_language_query">`
- Free users see the blurred search UI with upgrade prompt
- The "try these" example queries remain visible (teaser)

---

## Phase 3: Intelligent Free Tier Gates (1-2 hours)

Three limits that make free users feel the ceiling without crippling the experience.

### 3.1 Channel Limit Gate

**Limit:** Free = 3 custom channels, Pro = unlimited

**Backend — `src-tauri/src/channel_commands.rs`:**
- In `create_custom_channel`: count existing custom channels
- If count >= 3 AND not Pro: return error with upgrade message
- Built-in seed channels don't count toward the limit

**Frontend — `src/components/channels/ChannelsView.tsx`:**
- Show channel count badge: "2/3 channels" for free, no limit for Pro
- When at limit: replace "Create Channel" button with Pro upgrade prompt

### 3.2 Monitoring Frequency Gate

**Limit:** Free = minimum 30 min interval, Pro = minimum 5 min

**Backend — `src-tauri/src/monitoring_commands.rs`:**
- In `set_monitoring_interval`: check license tier
- If not Pro AND minutes < 30: clamp to 30 and return warning
- Pro users: minimum 5 min

**Frontend — wherever monitoring settings are configured:**
- Show interval options with Pro badges on < 30 min options
- Selecting a Pro interval triggers upgrade prompt if free tier

### 3.3 History Depth Gate

**Limit:** Free = 30 days of results, Pro = unlimited

**Backend — `src-tauri/src/db/mod.rs` or relevant query functions:**
- Add `history_cutoff` parameter to source item queries
- For free tier: `AND s.created_at > datetime('now', '-30 days')`
- For Pro: no date filter
- Determine tier at query time via settings manager

**Frontend — results area:**
- When free tier: show subtle footer "Showing last 30 days — unlock full history with Pro"
- Don't hide old items completely — show them grayed with lock icon
- Clicking a locked historical item triggers ProGate

---

## Phase 4: Pro Polish Features (2-3 hours)

### 4.1 Weekly Intelligence Digest

**New file:** `src-tauri/src/weekly_digest.rs`

**What it does:** Aggregates the week's intelligence into a single structured report.

**Data sources (all already computed):**
- Attention report: "This week you focused on: React, Rust, authentication"
- Knowledge gaps: "Gaps detected: React 19 migration, Bun runtime"
- Signal chains: "Emerging pattern: 3 signals about WebGPU adoption"
- Project health: "tauri-app health: 85/100, 2 stale dependencies"
- Scoring stats: "Processed 612 items, 39 relevant, 1 top pick"

**Format:** Structured JSON that frontend renders as a card/panel

**Trigger:** Manual `generate_weekly_digest` command (Pro-gated)

**Frontend:** New panel in Insights view

### 4.2 Decision Impact Tracking

**Files:** `src-tauri/src/decision_advantage/`, `src-tauri/src/db/`

**What it does:** After recording a decision ("adopted React 19"), 4DA monitors incoming signals that relate to that decision and surfaces them.

**Implementation:**
- When new decision recorded: extract `subject` + `context_tags` as tracking terms
- During analysis: cross-reference relevant items against active decision subjects
- New query: `get_decision_signals(decision_id)` returns matching items
- Store matches in `decision_signals` junction table

**Match types:** supporting, challenging, related

**Frontend:** In Decision Journal, each decision shows a "Signals" badge. Expand to see matching items.

**Pro gate:** `require_pro_feature("get_decision_signals")?`

---

## Phase 5: Commit & Validate

### 5.1 Validation checklist
- [ ] `cargo build` — Rust compiles cleanly
- [ ] `cargo test` — all tests pass (from src-tauri/)
- [ ] `npx tsc --noEmit` — TypeScript clean
- [ ] `pnpm run validate:sizes` — no file size violations
- [ ] `pnpm run validate:all` — full validation suite

### 5.2 Commit strategy
- Phase 1: "Wire semantic_shifts + clean vapor features"
- Phase 2: "Implement natural_language_query backend"
- Phase 3: "Add free tier gates (channels, monitoring, history)"
- Phase 4: "Add weekly digest + decision impact tracking"

---

## Final Pro Package

### Working Pro Features (13):
1. AI Briefing (generate + retrieve)
2. Attention Report
3. Knowledge Gaps
4. Signal Chains
5. Project Health
6. Developer DNA (view + markdown + SVG)
7. Semantic Shifts (newly wired)
8. Natural Language Search (newly implemented)
9. Weekly Intelligence Digest (new)
10. Decision Impact Tracking (new)

### Usage Gates (3):
11. Unlimited custom channels (free: 3)
12. Fast monitoring (free: 30 min, Pro: 5 min)
13. Full history (free: 30 days, Pro: unlimited)

### Removed vapor:
- ~~Audio briefing~~ (removed from gate)
- ~~Predicted context~~ (removed from gate)
- ~~Context packet~~ (removed from gate)

**Every listed feature works. Zero promises that can't be delivered.**

---

## Execution Order

```
Phase 1.1  ->  Wire semantic_shifts        [5 min]
Phase 1.2  ->  Clean vapor features         [5 min]
Phase 1.3  ->  Clean cache bust hack        [2 min]
Phase 2.1  ->  NL search backend            [90 min]
Phase 2.2  ->  Register + frontend gate     [15 min]
Phase 3.1  ->  Channel limit gate           [30 min]
Phase 3.2  ->  Monitoring frequency gate    [20 min]
Phase 3.3  ->  History depth gate           [30 min]
Phase 4.1  ->  Weekly intelligence digest   [60 min]
Phase 4.2  ->  Decision impact tracking     [60 min]
Phase 5    ->  Validate + commit            [20 min]
```
