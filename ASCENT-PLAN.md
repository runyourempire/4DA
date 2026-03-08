# 4DA ASCENT PLAN

> The plan to take 143,000 lines of engine and shape it into a product that speaks for itself.
> Created: 8 March 2026
> Authors: Claude + Antony
> Status: READY TO EXECUTE

---

## Philosophy

Every phase follows one rule: **invisible depth.** The engineering stays. The complexity disappears. Users feel sophistication without being told about it. The best products don't explain their engineering — they just work better than anything else.

Six phases. Each is independently shippable. Each makes the product measurably better. No phase depends on future infrastructure that doesn't exist yet.

---

## PHASE 1: Progressive Disclosure

> Goal: A new user sees 4 tabs, not 9. Views reveal as usage deepens.

### The Problem

Nine navigation tabs on first launch. Every tab is a cognitive commitment. Users with zero context must evaluate: Briefing, Channels, Results, Profile, Insights, Saved, Toolkit, Playbook, Calibrate. The best dev tools (Linear, Raycast, Arc) launched with 3-5 views.

### The Solution

Experience-level view gating. Not monetization gating — maturity gating. The views exist and work. They appear when the user is ready.

### 1.1 View Unlock System

**Files:**
- `src/store/slices/ui-slice.ts` — add `viewUnlockLevel` state
- `src/components/ViewTabBar.tsx` — filter tabs by unlock level
- `src/store/types.ts` — add type for unlock levels

**Unlock Tiers:**

| Tier | Trigger | Views Added |
|------|---------|-------------|
| **Core** | Always visible | Briefing, Results, Playbook |
| **Explorer** | 3+ analysis cycles completed | + Channels, + Insights |
| **Invested** | 5+ saved items OR 2+ decisions recorded | + Saved, + Profile |
| **Power** | 14+ days since first analysis | + Toolkit, + Calibrate |

**Implementation:**

```typescript
// ui-slice.ts
type ViewTier = 'core' | 'explorer' | 'invested' | 'power';

interface ViewUnlockState {
  tier: ViewTier;
  analysisCycleCount: number;
  firstAnalysisDate: string | null;
  manuallyUnlocked: boolean; // user can override via settings
}
```

```typescript
// ViewTabBar.tsx — filter TABS array
const TIER_VIEWS: Record<ViewTier, ViewId[]> = {
  core: ['briefing', 'results', 'playbook'],
  explorer: ['briefing', 'channels', 'results', 'playbook', 'insights'],
  invested: ['briefing', 'channels', 'results', 'profile', 'insights', 'saved', 'playbook'],
  power: ['briefing', 'channels', 'results', 'profile', 'insights', 'saved', 'toolkit', 'playbook', 'calibrate'],
};
```

**Safety valve:** Settings toggle: "Show all views" — immediately unlocks everything. No user should ever feel artificially limited. The progressive disclosure is a UX optimization, not a restriction. Persist to `localStorage`.

**View unlock notification:** When a new tier unlocks, show a toast: "New view unlocked: Channels — organize content by topic." Brief, informative, non-intrusive.

### 1.2 Tab Order Optimization

Reorder tabs for the user's actual workflow:

**Current:** Briefing, Channels, Results, Profile, Insights, Saved, Toolkit, Playbook, Calibrate

**New:** Briefing, Results, Playbook, Channels, Insights, Saved, Profile, Toolkit, Calibrate

Reasoning: Briefing → Results is the core loop. Playbook is free and immediately valuable. Channels and Insights are the first expansion. Profile and Saved are personal management. Toolkit and Calibrate are power-user depth.

### 1.3 Reorder Settings Tabs

**Current tabs:** General, Sources, Profile, Discovery, Health, About
**New tabs:** General, Sources, Projects, Profile, Advanced, About

- "Discovery" → "Projects" (clearer)
- "Health" → "Advanced" (less clinical)

**Files:**
- `src/components/SettingsModal.tsx` — update `SettingsTab` type and `TAB_IDS`

### Acceptance Criteria
- [ ] New user sees exactly 3 tabs: Briefing, Results, Playbook
- [ ] After 3 analyses, Channels and Insights appear with toast notification
- [ ] After saving 5 items or recording 2 decisions, Saved and Profile appear
- [ ] After 14 days, Toolkit and Calibrate appear
- [ ] "Show all views" toggle in Settings immediately reveals everything
- [ ] Tab order matches new prioritization
- [ ] Settings tabs renamed
- [ ] All existing tests still pass
- [ ] No feature removed — only visibility changed

---

## PHASE 2: Wire the Unwired

> Goal: Five features that are 90% built become 100% functional.

### The Problem

Five backend functions exist but are never called from the pipeline. The frontend already renders their output. The gap is 5-30 lines of Rust per feature.

### 2.1 Analysis Narration — Wire Backend Emissions

**The gap:** `analysis_narration::emit_narration()` exists. `NarrationEvent` struct exists. Frontend listens on `analysis-narration` events. But the analysis pipeline never calls `emit_narration()`.

**Files:**
- `src-tauri/src/analysis.rs` — add narration calls in `run_deep_initial_scan_impl()` and `run_multi_source_analysis_impl()`
- `src-tauri/src/source_fetching/fetcher.rs` — emit narration during source fetching

**Narration points (7 emissions):**

```rust
// 1. Analysis start (analysis.rs, beginning of run_deep_initial_scan_impl)
emit_narration(&app, NarrationEvent {
    narration_type: "discovery".into(),
    message: format!("Scanning {} sources for intelligence...", source_count),
    source: None,
    relevance: None,
});

// 2. Per-source fetch complete (source_fetching/fetcher.rs, after each source returns)
emit_narration(&app, NarrationEvent {
    narration_type: "discovery".into(),
    message: format!("{}: {} items found", source_name, item_count),
    source: Some(source_name.into()),
    relevance: None,
});

// 3. Scoring start (analysis.rs, before scoring loop)
emit_narration(&app, NarrationEvent {
    narration_type: "insight".into(),
    message: format!("Scoring {} items against your profile...", total_items),
    source: None,
    relevance: None,
});

// 4. High-relevance match found (analysis.rs, inside scoring loop when score > 0.7)
emit_narration(&app, NarrationEvent {
    narration_type: "match".into(),
    message: format!("High match: \"{}\"", item.title.chars().take(60).collect::<String>()),
    source: Some(item.source_type.clone()),
    relevance: Some(score as f32),
});

// 5. Stack-specific match (analysis.rs, when stack profile boost applied)
emit_narration(&app, NarrationEvent {
    narration_type: "match".into(),
    message: format!("Matches your {} stack directly", profile_name),
    source: Some(item.source_type.clone()),
    relevance: Some(score as f32),
});

// 6. Scoring complete summary (analysis.rs, after scoring loop)
emit_narration(&app, NarrationEvent {
    narration_type: "insight".into(),
    message: format!("{} items scored, {} above your threshold", total, relevant_count),
    source: None,
    relevance: None,
});

// 7. Top signal callout (analysis.rs, after sorting by score)
emit_narration(&app, NarrationEvent {
    narration_type: "insight".into(),
    message: format!("Top signal: \"{}\" ({:.0}%)", top_item.title.chars().take(50).collect::<String>(), top_score * 100.0),
    source: Some(top_item.source_type.clone()),
    relevance: Some(top_score as f32),
});
```

**Constraint:** Limit narration to first analysis AND analyses triggered from the UI (not background monitoring). Add a `narration_enabled: bool` parameter to the analysis functions, default `false` for monitoring, `true` for user-triggered.

### 2.2 Intelligence History — Wire Snapshot Recording

**The gap:** `intelligence_history::record_intelligence_snapshot()` exists. `get_intelligence_growth` Tauri command works. Nothing calls `record_intelligence_snapshot()`.

**Files:**
- `src-tauri/src/analysis.rs` — add snapshot recording after analysis completes

**Implementation:**

```rust
// At the end of run_multi_source_analysis_impl(), after results are stored:
if let Ok(conn) = crate::open_db_connection() {
    // Compute accuracy from autophagy if available
    let accuracy = crate::autophagy::get_current_accuracy(&conn).unwrap_or(0.0);
    let topics = items_above_threshold as i64;
    let _ = crate::intelligence_history::record_intelligence_snapshot(
        &conn,
        accuracy,
        topics,
        total_scored as i64,
        items_above_threshold as i64,
    );
}
```

**Acceptance:** After each analysis, `intelligence_history` table has a new row. IntelligenceProfileCard shows growth after 3+ analyses.

### 2.3 Weekly Digest Scheduling

**The gap:** `weekly_digest::should_generate_digest()` checks if 6+ days since last briefing. `monitoring_jobs::maybe_generate_digest()` exists but may not call this check properly.

**Files:**
- `src-tauri/src/monitoring_jobs.rs` — verify `maybe_generate_digest()` calls `should_generate_digest()` and emits tray notification

**Implementation:**

```rust
// In maybe_generate_digest(), ensure it:
// 1. Checks should_generate_digest()
// 2. If true, generates the digest
// 3. Emits a tray notification via tauri-plugin-notification
// 4. Emits a "digest-ready" event to the frontend

if should_generate_digest(&conn) {
    match crate::weekly_digest::generate_weekly_digest_impl(&conn) {
        Ok(digest) => {
            let _ = app.emit("digest-ready", &digest);
            // System tray notification
            if let Ok(()) = app.notification()
                .builder()
                .title("4DA Weekly Digest")
                .body(format!("{} items analyzed, {} relevant this week",
                    digest.stats.total_items_analyzed,
                    digest.stats.relevant_items))
                .show() {
                info!(target: "4da::digest", "Weekly digest notification sent");
            }
        }
        Err(e) => warn!(target: "4da::digest", "Digest generation failed: {e}"),
    }
}
```

### 2.4 Standing Query Results Display

**The gap:** Standing query evaluation already runs in the monitoring tick (wired in lib.rs setup). It emits `standing-query-matches` events. No UI handles this event.

**Files:**
- `src/components/BriefingView.tsx` — listen for standing query matches, show notification badge
- `src/components/StandingQueryResults.tsx` — new small component (alert-style)

**Implementation:**

```typescript
// In BriefingView, add event listener:
useEffect(() => {
  const unlisten = listen<Array<{ query_id: number; query_name: string; new_matches: number }>>(
    'standing-query-matches', (event) => {
      const alerts = event.payload.filter(a => a.new_matches > 0);
      if (alerts.length > 0) {
        addToast('info', `${alerts.length} standing query match${alerts.length > 1 ? 'es' : ''} found`);
      }
    }
  );
  return () => { unlisten.then(fn => fn()); };
}, [addToast]);
```

### 2.5 Feedback Loop Visibility

**The gap:** When a user clicks relevant/not-relevant, the feedback is recorded silently. The user gets no confirmation that the system learned.

**Files:**
- `src/hooks/use-feedback.ts` or relevant feedback handler
- Toast system (already exists)

**Implementation:**

After recording feedback, show a contextual toast:
- Relevant click: "Got it — boosting similar content"
- Not relevant click: "Noted — reducing this pattern"
- Save click: "Saved — your preferences are shaping future results"

These are one-line additions to existing feedback handlers. No new infrastructure.

### Acceptance Criteria
- [ ] First analysis emits 5+ narration events visible in LoadingState feed
- [ ] Narration events NOT emitted during background monitoring cycles
- [ ] intelligence_history table populated after each analysis
- [ ] IntelligenceProfileCard shows growth after 3+ analyses
- [ ] Weekly digest generates after 6+ days, tray notification appears
- [ ] Standing query matches show toast notification
- [ ] Feedback clicks show contextual learning confirmation
- [ ] All changes < 50 lines each

---

## PHASE 3: Naming & Copy Pass

> Goal: Every user-facing string becomes instantly clear. Internal names stay rich.

### The Rule

Code stays philosophical (these names are accurate and meaningful for us). User-facing text becomes functional. We're translating, not simplifying.

### 3.1 Navigation Labels

**File:** `src/locales/en/ui.json` — update nav keys

| Current | New Label | New Subtitle |
|---------|-----------|-------------|
| Briefing | Briefing | Your daily intelligence |
| Channels | Channels | Topics you follow |
| Results | Results | Latest scored content |
| Profile | Profile | Your developer identity |
| Insights | Decisions | Strategy & tech radar |
| Saved | Saved | Your bookmarks |
| Toolkit | Toolkit | Debug & test scoring |
| Playbook | Playbook | STREETS business course |
| Calibrate | System | Health & calibration |

Key change: "Insights" → "Decisions" (the view contains TechRadar and DecisionMemory — "Decisions" is what it actually is).

### 3.2 Component Header Strings

**File:** `src/locales/en/ui.json`

| Where | Current | New |
|-------|---------|-----|
| SovereignDeveloperProfile header | "Sovereign Developer Profile" | "Your Profile" |
| IntelligenceProfileCard | "Intelligence Growth" | "Learning Progress" |
| AutophagyInsights panel | "Intelligence Metabolism" | "How 4DA Learns" |
| CompoundAdvantageScore | "Compound Advantage" | "Your Edge" |
| IntelligencePulse | "Intelligence Pulse" | "Live Signals" |
| DelegationAdvisor | "Delegation Advisor" | "Task Coaching" |
| DecisionWindows panel | "Decision Windows" | "Decisions to Make" |
| CalibrationView | "Rig Calibration" | "System Health" |
| KnowledgeGapsPanel | "Knowledge Gaps" | "Learning Opportunities" |
| SignalsPanel | "Signals" | "Key Signals" |

### 3.3 Settings Section Labels

**File:** `src/locales/en/ui.json`

| Current | New |
|---------|-----|
| "Discovery" tab | "Projects" |
| "Health" tab | "Advanced" |
| "Monitoring Section" | "Background Updates" |
| "Notification threshold" | "Alert level" |
| "Close to tray" | "Keep running in background" |
| "Digest Section" | "Weekly Summary" |

### 3.4 Score & Feedback Language

**File:** `src/locales/en/ui.json`

| Context | Current | New |
|---------|---------|-----|
| Score label | "Relevance: 0.87" | "87% relevant" |
| Empty briefing | "Run an analysis to see intelligence" | "Run your first analysis to see what 4DA finds for you" |
| No results | "No items found" | "Nothing above your relevance threshold yet" |
| First feedback | (none) | "Your first feedback — 4DA is now learning your preferences" |

### 3.5 Tooltip & Description Enrichment

Every tab subtitle should tell the user exactly what they'll find:

```json
{
  "nav.briefing.subtitle": "AI-generated summary of what matters",
  "nav.results.subtitle": "Every item scored against your profile",
  "nav.playbook.subtitle": "7-module developer business course",
  "nav.channels.subtitle": "Custom topic feeds you define",
  "nav.insights.subtitle": "Technology decisions & radar",
  "nav.saved.subtitle": "Items you've bookmarked",
  "nav.profile.subtitle": "How 4DA sees your dev identity",
  "nav.toolkit.subtitle": "Score testing & source debugging",
  "nav.calibrate.subtitle": "System health & tuning"
}
```

### Acceptance Criteria
- [ ] All i18n keys updated in `src/locales/en/ui.json`
- [ ] No code-level renames (internal names preserved)
- [ ] Every tab has a clear subtitle
- [ ] Score displays as percentage, not decimal
- [ ] Empty states have actionable copy
- [ ] All existing tests pass (tests use i18n mocks, not string matching)

---

## PHASE 4: Core Loop Polish

> Goal: The fetch → score → display → learn cycle becomes flawless.

### 4.1 Inline Match Reason on Results

**The problem:** `generateFallbackReason()` in `ResultItem.tsx` already builds reasons, but they're only visible when expanding the item. The most compelling information is hidden behind a click.

**Files:**
- `src/components/result-item/ResultItemCollapsed.tsx` — show match reason inline

**Implementation:**

Below the title in the collapsed result item, show a single-line match reason:

```
Tauri 2.0 Plugin Security Model         0.92
Matches your dependency: tauri · Security alert
```

The reason line uses `text-text-muted text-xs` — subtle, not competing with the title. Only show for items with score > 0.5 (below that, the reason isn't meaningful enough).

This is the single most important UX change in the entire plan. It answers "why is this here?" without any user action.

### 4.2 Decision Match Badge Visibility

**The current state:** `BadgeRow.tsx` already renders decision match badges when `decision_window_match` is present on an item. But the scoring pipeline doesn't pass this data through to the frontend.

**Files:**
- `src-tauri/src/decision_advantage/scoring_boost.rs` — return matched window subject
- `src-tauri/src/analysis.rs` — pass decision match info into SourceRelevance struct
- `src-tauri/src/types.rs` — add `decision_window_match: Option<String>` and `decision_boost_applied: f32` to SourceRelevance

**Implementation:**

When the decision advantage scoring boost is applied, store the matched window's subject in the SourceRelevance. The frontend BadgeRow already renders this — it just needs the data.

```rust
// In SourceRelevance (types.rs)
#[serde(skip_serializing_if = "Option::is_none")]
pub decision_window_match: Option<String>,
#[serde(default)]
pub decision_boost_applied: f32,
```

### 4.3 Zero-Result Handling

**The problem:** If an analysis returns zero items above threshold, the user sees an empty results view. This is a crisis moment — the user thinks the tool is broken.

**Files:**
- `src/components/ResultsView.tsx` — add zero-result state
- `src/components/ZeroResultsGuide.tsx` — new component

**Implementation:**

When `relevanceResults.length === 0` and `analysisComplete === true`:

```
+---------------------------------------------------+
|  No items matched your current threshold (0.35)    |
|                                                     |
|  Here's what was closest:                           |
|  1. "Tauri 2.0 Security Model" — 0.32 (3% below)  |
|  2. "Rust Async Patterns" — 0.28 (20% below)      |
|  3. "TypeScript 5.8 Features" — 0.25 (29% below)  |
|                                                     |
|  Suggestions:                                       |
|  - Lower your threshold to 0.25 in Settings        |
|  - Add more interests in your Profile               |
|  - Enable more sources in Settings > Sources        |
|                                                     |
|  [Adjust Threshold]  [Add Interests]                |
+---------------------------------------------------+
```

This requires the backend to return the top N items regardless of threshold when the filtered count is zero. Add a `near_misses: Vec<SourceRelevance>` field to the analysis result, populated when relevant count is 0.

### 4.4 Source Quality Suggestions

**The problem:** If a source consistently produces low-scoring items, the user doesn't know. They keep fetching from it, wasting time.

**Files:**
- `src-tauri/src/source_fetching/fetcher.rs` or `src-tauri/src/health.rs` — compute per-source relevance ratio
- `src/components/BriefingView.tsx` — show source quality alert

**Implementation:**

After each analysis, compute `relevant_items / total_items` per source. If a source drops below 5% relevance ratio over the last 3 analyses:

```
Reddit r/programming: 3% of items are relevant to you.
Consider replacing with r/rust (similar stacks see 45% relevance).
[Swap Source]  [Keep It]
```

Store per-source stats in `source_health` table (already exists). The suggestion system uses stack profile data to recommend better subreddit/RSS alternatives.

### 4.5 Score Display Format

**The problem:** Scores displayed as "0.87" are developer-readable but user-hostile.

**Files:**
- `src/components/result-item/ResultItemCollapsed.tsx` — format score display
- `src/components/result-item/ScoreBreakdownDrawer.tsx` — format scores

**Implementation:**

All user-facing scores rendered as percentages: "87%" instead of "0.87". The internal representation stays as floats. Only the display changes.

Add a utility:
```typescript
export function formatScore(score: number): string {
  return `${Math.round(score * 100)}%`;
}
```

Use it everywhere a score is rendered to the user.

### Acceptance Criteria
- [ ] Every result item with score > 0.5 shows inline match reason without expanding
- [ ] Decision window matches appear as badges on boosted items
- [ ] Zero-result analysis shows closest items with threshold adjustment guidance
- [ ] Sources with < 5% relevance ratio show replacement suggestions
- [ ] All scores displayed as percentages
- [ ] Core loop feels responsive and informative at every step

---

## PHASE 5: Production Hardening

> Goal: Every edge case is handled. The app is bulletproof.

### 5.1 File Size Compliance

**Files over limits (must split):**

| File | Lines | Limit | Action |
|------|-------|-------|--------|
| stacks/profiles.rs | 1,482 | 1,000 | Split into profiles_core.rs + profiles_data.rs |
| scoring/simulation/corpus.rs | 1,467 | 1,000 | Split corpus data into corpus_data.rs |
| scoring/pipeline.rs | 1,389 | 1,000 | Extract helpers into pipeline_helpers.rs |
| sovereign_developer_profile.rs | 1,386 | 1,000 | Split into profile_builder.rs + profile_export.rs |
| settings_commands.rs | 1,383 | 1,000 | Split by domain: settings_general.rs + settings_sources.rs |
| settings/mod.rs | 1,367 | 1,000 | Extract types into settings_types.rs |
| scoring/benchmark.rs | 1,328 | 1,000 | Move benchmark data to benchmark_data.rs |
| db/migrations.rs | 1,304 | 1,000 | Split older migrations to migrations_v1.rs |
| game_engine.rs | 1,158 | 1,000 | Split into game_state.rs + game_logic.rs |
| llm.rs | 1,098 | 1,000 | Extract providers to llm_providers.rs |
| analysis.rs | 1,070 | 1,000 | Extract scan/analysis to analysis_scan.rs |
| ace_commands.rs | 1,055 | 1,000 | Split by feature area |

**Frontend files over limits:**

| File | Lines | Limit | Action |
|------|-------|-------|--------|
| lib/commands.ts | 575 | 500 | Split by domain: commands-analysis.ts, commands-settings.ts, etc. |
| SettingsModal.tsx | 526 | 500 | Extract tab content to SettingsGeneral.tsx, SettingsHealth.tsx |
| BriefingView.tsx | 495 | 500 | Close to limit — monitor, extract if it grows |
| App.tsx | 476 | 500 | Close to limit — monitor |
| SovereignDeveloperProfile.tsx | 460 | 500 | Close to limit — monitor |

### 5.2 Error UX Audit

Every Tauri command that can fail should produce a user-actionable toast.

**Files:**
- All store slices that call `invoke()` — audit `.catch()` handlers

**Pattern:**
```typescript
// BAD (current in some places):
.catch(() => {})

// GOOD:
.catch((e) => {
  addToast('error', t('errors.briefingFailed', 'Could not generate briefing. Check your AI provider settings.'));
  console.error('Briefing generation failed:', e);
})
```

Audit all `invoke()` calls in store slices. Every `.catch(() => {})` gets replaced with meaningful error handling. Count them, fix them all.

### 5.3 Offline Resilience Audit

Test every view with no network connectivity. Document behavior.

**Expected behavior:**
- Results: Show cached results from last analysis, with "Offline — showing cached data" banner
- Briefing: Show last briefing with "Generated on [date]" indicator
- Channels: Show last rendered channels
- Playbook: Fully functional (all local content)
- Settings: Fully functional (local storage)
- Profile: Fully functional (local data)

**Files to check:**
- Every component that calls `invoke()` with network-dependent commands
- `source_fetching/fetcher.rs` — graceful timeout handling per source

### 5.4 Memory & Resource Audit

**WebGL contexts:** Each GAME component creates a WebGL2 context. Verify contexts are properly destroyed when components unmount. The `use-game-component.ts` hook handles this — verify cleanup path.

**Event listeners:** Every `listen()` call must have a corresponding cleanup in the effect return. Audit all `useEffect` blocks with `listen()`.

**SQLite connections:** Verify `open_db_connection()` connections are properly scoped and dropped. No connection leaks during long-running sessions.

### 5.5 Startup Performance

Measure cold start time from app launch to first interactive frame.

**Target:** Under 2 seconds on reasonable hardware.

**Optimizations if needed:**
- Lazy-load all non-critical views (already done for most)
- Defer ACE full scan to after first render
- Defer monitoring scheduler start by 5 seconds
- Pre-warm database connections during splash screen

### Acceptance Criteria
- [ ] Zero files exceed error thresholds (1,000 Rust, 500 TS)
- [ ] Warning-level files reduced from 7 to 0
- [ ] Every invoke().catch() has meaningful error messaging
- [ ] App works offline with cached data
- [ ] No WebGL context leaks (verify with Chrome DevTools)
- [ ] No event listener leaks (verify with React DevTools)
- [ ] Cold start under 2 seconds
- [ ] All 2,350+ tests pass after splits

---

## PHASE 6: Strategic Pruning

> Goal: Remove dead weight. Keep everything that earns its place.

### The Pruning Principle

We don't prune features that are connected to the core value loop. We prune features that:
1. Have no connection to content intelligence
2. Add maintenance burden without user value
3. Could be re-added later without architectural cost

### 6.1 Game Achievement System — Demote to Experimental

**Files:** `game_engine.rs` (1,158), `game_achievements.rs` (448), `game_commands.rs` (80)
**Total:** 1,686 lines

**Why:** Gamification badges in a professional intelligence tool create tonal dissonance. The GAME *shader components* are core visual identity. The achievement *system* is a different concept.

**Action:** Don't delete. Wrap in `#[cfg(feature = "experimental")]`. Remove from invoke_handler. The code stays, the feature disappears from production. Can be re-enabled for future experiments.

```rust
// lib.rs — conditional compilation
#[cfg(feature = "experimental")]
mod game_achievements;
#[cfg(feature = "experimental")]
mod game_commands;
#[cfg(feature = "experimental")]
mod game_engine;
```

```toml
# Cargo.toml
[features]
experimental = []
```

### 6.2 Toolkit HTTP — Demote to Experimental

**Files:** `toolkit_http.rs` (354)
**Total:** 354 lines

**Why:** A built-in HTTP client inside a content intelligence tool. Users have Postman, curl, httpie. This adds attack surface (arbitrary HTTP requests from the app) for minimal relevance to the core product.

**Action:** Same `#[cfg(feature = "experimental")]` treatment. Remove from invoke_handler. Keep code intact.

### 6.3 Delegation System — Demote to Experimental

**Files:** `delegation.rs` (609), `DelegationAdvisor.tsx`, `DelegationDashboard.tsx`
**Total:** ~800 lines

**Why:** AI coaching on task delegation is a compelling feature but disconnected from content intelligence. It belongs in a future "AI coaching" expansion, not in the core product launch.

**Action:** `#[cfg(feature = "experimental")]` for Rust. Frontend components stay (they're already lazy-loaded) but are hidden from navigation. Re-enable when we build the coaching expansion.

### 6.4 Assess but Preserve

These stay but get scrutinized:

- **Templates** (`template_data.rs`, `templates.rs`) — Stay. Directly supports Playbook value.
- **Sovereign Profile** — Stay. Renamed to "Your Profile" but functionality is core identity.
- **Community Intelligence** — Stay. Toggle and UI are ready for when the API exists.
- **Channels** — Stay. Core content organization feature.

### Acceptance Criteria
- [ ] Game achievements: hidden in production, code preserved behind feature flag
- [ ] Toolkit HTTP: hidden in production, code preserved behind feature flag
- [ ] Delegation: hidden in production, code preserved behind feature flag
- [ ] ~2,840 lines moved behind feature flags (not deleted)
- [ ] Production binary size reduced
- [ ] No functionality loss for anything connected to content intelligence
- [ ] `cargo build --features experimental` still compiles all pruned code

---

## Execution Order

### Sprint 1: Phase 3 (Naming) + Phase 1 (Progressive Disclosure)
Start with naming because it requires zero architectural changes — pure i18n updates. Then implement progressive disclosure.

**Estimated scope:** ~200 lines changed/added
**Risk:** Zero (no structural changes)

### Sprint 2: Phase 2 (Wire the Unwired)
Connect the 5 unwired features. Each is 5-50 lines of Rust.

**Estimated scope:** ~150 lines added
**Risk:** Low (adding calls to existing functions)

### Sprint 3: Phase 4 (Core Loop Polish)
Inline match reasons, zero-result handling, score formatting, source quality suggestions.

**Estimated scope:** ~400 lines added
**Risk:** Low-medium (UI changes to core views)

### Sprint 4: Phase 6 (Strategic Pruning)
Feature-flag experimental systems. Clean and tight.

**Estimated scope:** ~50 lines changed (feature flags)
**Risk:** Low (no code deleted)

### Sprint 5: Phase 5 (Production Hardening)
File splits, error audit, offline testing, memory profiling.

**Estimated scope:** ~500 lines moved (not added)
**Risk:** Medium (file reorganization, must not break imports)

### Validation Gate (after each sprint)
- [ ] cargo check (0 errors)
- [ ] cargo test (1,597+ tests pass)
- [ ] pnpm run test (753+ tests pass)
- [ ] pnpm run lint (0 warnings)
- [ ] pnpm run build (clean production build)
- [ ] tsc --noEmit (0 errors)

---

## Success Metrics

| Metric | Before | Target |
|--------|--------|--------|
| Navigation tabs visible to new user | 9 | 3 |
| Narration events during first analysis | 0 | 7+ |
| Intelligence history snapshots | Not recorded | 1 per analysis |
| Weekly digest delivery | No notification | Tray notification |
| Standing query result visibility | No UI | Toast + briefing badge |
| Score format | Decimal (0.87) | Percentage (87%) |
| User-facing jargon terms | ~10 | 0 |
| Files exceeding size limits | 12+ | 0 |
| Silent .catch(() => {}) handlers | Unknown | 0 |
| Offline functionality | Untested | All views graceful |
| Code behind experimental flags | 0 lines | ~2,840 lines |
| Cold start time | Unmeasured | < 2 seconds |

---

## What This Plan Does NOT Do

- No new features. Every change improves what exists.
- No new database tables. Every table already exists.
- No new dependencies. Everything uses existing crates/packages.
- No new Tauri commands. Everything uses existing command surface.
- No removal of any working feature. Pruning uses feature flags, not deletion.
- No changes to the privacy architecture. The privacy promise is absolute.
- No changes to PASIFA scoring logic. The engine is not the problem.
- No changes to the GAME compiler or shader components. Visual identity is settled.
- No changes to the STREETS playbook content. Educational content is complete.

This plan takes 143,000 lines of engine and shapes the body to match.
