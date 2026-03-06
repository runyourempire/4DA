# 4DA Deep Performance Optimization Plan — Phase 2

## Context

Phase 1 (completed) parallelized startup IPC, eliminated duplicate calls, added Rust-side TTL caching for calibration/pulse, created batch personalization command, added visibility-aware polling, and gated conditional mounting. All 852 frontend + 1647 Rust tests pass.

This plan covers **everything remaining** — discovered by 5 parallel deep-exploration agents covering frontend IPC, React rendering, Rust backend, bundle/startup, and SQLite/DB layers.

---

## Tier 1: Critical (P0) — Biggest Bang for Buck

### 1.1 Parallelize 13 Sequential `listen()` Calls in use-analysis.ts
- **File:** `src/hooks/use-analysis.ts` lines 42-187
- **Problem:** 13 `await listen(...)` calls fire sequentially at mount — 13 IPC round-trips before all event listeners register
- **Fix:** `Promise.all([listen(...), listen(...), ...])` with destructured unlistens
- **Saves:** ~12 round-trip latencies on every cold start

### 1.2 N+1 Loop: 17 Sequential IPC Calls on Onboarding Complete
- **File:** `src/components/onboarding/use-quick-setup.ts` lines 314-315
- **Problem:** `for (const interest of interestsToSave) await invoke('add_interest', {...})` + same for `detectedTech`
- **Fix:** `Promise.all([...interests.map(...), ...tech.map(...)])` or add `add_interests_batch`/`add_tech_stacks_batch` Rust commands
- **Saves:** Up to 16 round-trip latencies on first-run onboarding

### 1.3 README Indexing N+1 — One Embedding Call Per Chunk
- **File:** `src-tauri/src/ace/readme_indexing.rs` line 339
- **Problem:** `embed_texts(std::slice::from_ref(&chunk_content))` called inside triple nested loop (projects x sections x chunks). 50 projects x 5 sections x 3 chunks = 750 individual HTTP embedding calls
- **Fix:** Collect all chunk texts first, call `embed_texts(&all_chunks)` in batches of 50-100
- **Saves:** 749 HTTP round-trips during README indexing

### 1.4 Missing `feedback` Table Indexes
- **File:** `src-tauri/src/db/migrations.rs`
- **Problem:** `feedback.created_at` and `feedback.relevant` are unindexed but queried in hot paths (`get_feedback_topic_summary` runs at start of every analysis — 500-row JOIN with ORDER BY on unindexed column)
- **Fix:** Add 3 indexes:
  ```sql
  CREATE INDEX IF NOT EXISTS idx_feedback_created ON feedback(created_at);
  CREATE INDEX IF NOT EXISTS idx_feedback_item_relevant ON feedback(source_item_id, relevant);
  CREATE INDEX IF NOT EXISTS idx_source_items_created ON source_items(created_at);
  ```
- **Saves:** Full table scans on every analysis run

### 1.5 SettingsModal — 47 Individual Store Subscriptions
- **File:** `src/components/SettingsModal.tsx` lines 116-166
- **Problem:** 47 individual `useAppStore(s => s.x)` calls. Any store update to any of these slices triggers a full 476-line component re-render
- **Fix:** Group all data selectors (~12 fields) into a single `useShallow` call. Keep action selectors individual (stable references).

### 1.6 ResultsView — Inline Callback Defeats memo on Every Virtualized Row
- **File:** `src/components/ResultsView.tsx` line 388
- **Problem:** `onToggleExpand={() => setExpandedItem(...)}` inside `virtualizer.getVirtualItems().map()`. `ResultItem` is `memo`-wrapped but this inline arrow defeats memo for every visible row on every scroll event.
- **Fix:** `useCallback` factory or row-level wrapper stabilizing the handler

### 1.7 BriefingView — Inline Callbacks Defeat BriefingCard/SignalActionCard memo
- **File:** `src/components/BriefingView.tsx` lines 264-318
- **Problem:** `onSave`, `onDismiss`, `onRecordInteraction` are inline arrows in `.map()` loops — `BriefingCard` is `memo`-wrapped but always re-renders
- **Fix:** Stable `useCallback` handlers that accept item as parameter

### 1.8 Compress sun-logo.jpg (712 KB -> ~5 KB)
- **File:** `src/assets/sun-logo.jpg`
- **Problem:** 712 KB JPEG displayed at max 40x40px. 89x larger than needed.
- **Fix:** Convert to WebP at 160x160px (4x for HiDPI): target < 8 KB
- **Saves:** ~700 KB from initial bundle load

---

## Tier 2: High Impact (P1)

### 2.1 Cache `ScoringContext` with TTL
- **File:** `src-tauri/src/scoring/context.rs`
- **Problem:** `build_scoring_context` opens 2 connections, runs 8+ DB queries at the start of every analysis. Between rapid runs, inputs don't change.
- **Fix:** `LazyLock<Mutex<Option<(ScoringContext, Instant)>>>` with 5-min TTL, invalidated on feedback/settings writes

### 2.2 Parallelize `recordInteraction` — Two Independent Writes
- **File:** `src/store/feedback-slice.ts` lines 87-106
- **Problem:** `ace_record_interaction` then `ace_record_accuracy_feedback` — sequential on every save/dismiss/click
- **Fix:** `Promise.all([invoke('ace_record_interaction', ...), invoke('ace_record_accuracy_feedback', ...)])`
- **Saves:** 1 round-trip on every user feedback action

### 2.3 Parallelize `saveSettings` — Two Independent Writes
- **File:** `src/store/settings-slice.ts` lines 79-92
- **Problem:** `set_llm_provider` then `set_rerank_config` — sequential independent writes
- **Fix:** `Promise.all([...])`

### 2.4 `useVisibilityPolling` for CommandDeck + PortScanner
- **Files:** `src/components/command-deck/CommandDeck.tsx` line 45, `src/components/toolkit/tools/PortScanner.tsx` line 68
- **Problem:** Both use raw `setInterval` (5s) without visibility awareness — keeps polling when app is backgrounded
- **Fix:** Replace with `useVisibilityPolling`

### 2.5 ViewTabBar — Array Selectors Without useShallow, No memo
- **File:** `src/components/ViewTabBar.tsx` lines 29-34
- **Problem:** `channels ?? []` creates new reference on null, `decisionWindows` is array without `useShallow`. Re-renders on every background fetch.
- **Fix:** Wrap in `React.memo`, consolidate data selectors with `useShallow`

### 2.6 FirstRunTransition — 5 Heavy Computations in Render Body
- **File:** `src/components/FirstRunTransition.tsx` lines 32-56
- **Problem:** `.filter()`, `.reduce() + .sort()`, `.find() x 3`, `buildStackInsights()` all run on every render during analysis progress updates
- **Fix:** Wrap all 5 in `useMemo`

### 2.7 Lazy-load Onboarding Component
- **File:** `src/App.tsx` line 13
- **Problem:** `import { Onboarding }` is eager — only shown on first launch, every returning user loads it for nothing
- **Fix:** `const Onboarding = lazy(() => import('./components/Onboarding').then(m => ({ default: m.Onboarding })))`

### 2.8 Lazy-load ProValueBadge
- **File:** `src/App.tsx` line 28
- **Problem:** Declared non-critical but imported eagerly, inconsistent with other non-critical components
- **Fix:** Move to lazy group

### 2.9 ACE Database Missing PRAGMA Tuning
- **File:** `src-tauri/src/ace/db.rs` lines 17-19
- **Problem:** ACE DB only has WAL + busy_timeout. Missing `cache_size`, `mmap_size`, `temp_store` that the main DB has.
- **Fix:** Add same PRAGMA block as main DB

### 2.10 Deep Fetch N+1 — Individual `upsert_source_item` Per Item
- **File:** `src-tauri/src/source_fetching/fetcher.rs` lines 596-610
- **Problem:** Deep fetch path uses individual `upsert_source_item` per item (no transaction). Shallow path already uses `batch_upsert_source_items`.
- **Fix:** Use `batch_upsert_source_items` in deep path too

---

## Tier 3: Medium Impact (P2)

### 3.1 Defer Settings-Only Hooks from Global Mount
- **Files:** `src/hooks/use-context-discovery.ts`, `use-system-health.ts`, `use-user-context.ts`
- **Problem:** These fire IPC on every startup but their data is only consumed by SettingsModal (lazy loaded)
- **Fix:** Move load triggers to fire when SettingsModal mounts instead of App mount

### 3.2 Cache `get_feedback_topic_summary` Until Next Feedback Write
- **File:** `src-tauri/src/db/sources.rs` lines 624-688
- **Problem:** 500-row JOIN + in-memory tokenization runs at the start of every analysis. Changes only on feedback writes.
- **Fix:** In-memory cache invalidated on `record_feedback`

### 3.3 Cache `compute_radar` with 5-min TTL
- **File:** `src-tauri/src/tech_radar.rs`
- **Problem:** `get_radar_entry` calls full `compute_radar` (multi-table read) just to find one entry
- **Fix:** TTL cache for radar, or have `get_radar_entry` call `get_tech_radar` internally

### 3.4 Redundant DB Connections in monitoring.rs
- **File:** `src-tauri/src/monitoring.rs` lines 267, 297, 359
- **Problem:** Three separate `open_db_connection()` calls in the same scheduler tick
- **Fix:** Open once, pass to all three operations

### 3.5 Redundant DB Connections in scoring/context.rs
- **File:** `src-tauri/src/scoring/context.rs` lines 69, 106
- **Problem:** Two `open_db_connection()` calls in `build_scoring_context`
- **Fix:** Share one connection

### 3.6 Redundant `assemble_profile` in Sovereign Profile Exports
- **File:** `src-tauri/src/sovereign_developer_profile.rs` lines 993-1006
- **Problem:** Three commands each open a new connection and run full `assemble_profile`
- **Fix:** Export commands call the getter and format result

### 3.7 Merge Two `registry.lock()` Calls in lib.rs
- **File:** `src-tauri/src/lib.rs` lines 248-256
- **Problem:** Two separate lock acquisitions where one would suffice
- **Fix:** Single scoped lock

### 3.8 `save_render_provenance` Missing Transaction
- **File:** `src-tauri/src/db/channels.rs` lines 356-379
- **Problem:** N individual auto-commit writes per channel render
- **Fix:** Wrap in `conn.unchecked_transaction()`

### 3.9 `store_source_autopsies` Not Batched
- **File:** `src-tauri/src/autophagy/source_autopsy.rs` line 101
- **Problem:** UPDATE + INSERT per source type with no transaction wrapper
- **Fix:** Wrap in single transaction

### 3.10 Inefficient Pruning Subquery Pattern
- **Files:** `src-tauri/src/db/history.rs` lines 169-173, `db/cache.rs`
- **Problem:** `DELETE WHERE id NOT IN (SELECT ... LIMIT 200)` materializes full subquery
- **Fix:** `DELETE WHERE id < (SELECT id ... LIMIT 1 OFFSET 199)`

### 3.11 React.memo Gaps — FeedbackButtons, ResultItemCollapsed, ChannelCard, SignalRow
- **Files:** Multiple components in `src/components/`
- **Problem:** Leaf components in hot paths without `React.memo`
- **Fix:** Add `memo()` wrapper to each

### 3.12 `useShallow` Consolidation — Multiple Components
- **Files:** `PersonalizationSection.tsx`, `AutophagyInsights.tsx`, `DecisionWindowsPanel.tsx`, `ChannelsView.tsx`, `BriefingEmptyStates.tsx`, `AchievementsPanel.tsx`, `IntelligenceProfileCard.tsx`
- **Problem:** Object/array selectors without `useShallow`, causing unnecessary re-renders
- **Fix:** Batch data selectors with `useShallow`

### 3.13 Eliminate `format!` in Hash Hot Paths
- **Files:** `src-tauri/src/db/sources.rs` line 56, `source_fetching/fetcher.rs` line 185
- **Problem:** `format!("{}{}", title, content)` then hash — allocates string just to hash it
- **Fix:** Feed components directly to hasher

---

## Tier 4: Low Impact / Structural (P3)

### 4.1 Separate `@tauri-apps/plugin-updater` from Eager Vendor Chunk
- **File:** `vite.config.ts` line 23
- Check is deferred 5s but bundled eagerly

### 4.2 Move `listen()` Side Effect Out of Store Creation
- **File:** `src/store/unified-profile-slice.ts` lines 25-29
- `listen('profile-updated')` fires at module evaluation time (store import)

### 4.3 Delete Unused `react.svg`
- **File:** `src/assets/react.svg` (4.1 KB)
- Default Vite template file, appears unused

### 4.4 Lazy localStorage Reads in toolkit-slice
- **File:** `src/store/toolkit-slice.ts` lines 7-8
- `JSON.parse(localStorage.getItem(...))` at store creation

### 4.5 `tokio::fs::read_to_string` for README Indexing
- **File:** `src-tauri/src/ace/readme_indexing.rs` line 299
- `std::fs::read_to_string` blocks Tokio thread

### 4.6 Deduplicate `checkAgentDataExists` IPC
- **File:** `src/store/agent-slice.ts` lines 62-69
- Separate `recall_agent_memories` with `limit: 1` — derive from `loadAgentMemories` result instead

### 4.7 Add missing `PRAGMA` settings for digested_intelligence index
- **File:** `src-tauri/src/db/migrations.rs`
- `digested_intelligence.superseded_by` unindexed, `channel_renders(channel_id, version)` composite missing

### 4.8 Parallelize SplashScreen Backend Probes
- **File:** `src/components/SplashScreen.tsx` lines 60-98
- 3 sequential probes could be `Promise.allSettled` with animated stage advancement

---

## Execution Strategy

**Phase 2A (Frontend — LOW RISK):** Items 1.1, 1.2, 1.5, 1.6, 1.7, 1.8, 2.1-2.8, 3.1, 3.11, 3.12
**Phase 2B (Rust Backend — MEDIUM RISK):** Items 1.3, 1.4, 2.9, 2.10, 3.2-3.10, 3.13
**Phase 2C (Asset/Bundle — LOW RISK):** Items 1.8, 4.1, 4.3

Each item is independently verifiable. Run `pnpm run build`, `pnpm run test -- --run`, and `cargo test --lib` after each batch.
