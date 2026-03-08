# 4DA Quality & Performance Plan

## Phase 1: Critical Profile Accuracy Fix (The Drizzle Bug)

**Problem:** Profile title says "Drizzle/Javascript/React developer" because `primary_stack` is sorted alphabetically. `drizzle` (d) beats `rust` (r) in sort order. No frequency/importance weighting exists.

**Root cause:** `sovereign_developer_profile.rs:302` — `primary_stack.sort()` uses alphabetical ordering instead of relevance-based ranking.

**Fix:**

### 1A. Frequency-weighted stack ranking
- **File:** `sovereign_developer_profile.rs:298-332` (`assemble_stack`)
- Replace `primary_stack.sort()` with a scoring function that ranks by:
  1. **Project count** — how many local projects use this tech (from `project_dependencies`)
  2. **Recency** — most recently modified projects weighted higher
  3. **LOC proxy** — projects with more files containing this tech rank higher
  4. **Source** — `tech_stack` (user-declared) > `detected_tech` (ACE-scanned) > `project_dependencies`
- Sort by score descending, not alphabetically

### 1B. Separate declared stack from detected dependencies
- **File:** `domain_profile.rs:46-115` (`build_domain_profile`)
- Currently `tech_stack` entries and `detected_tech` entries both flow into `primary_stack` without distinction
- Add a `source_priority` field: `declared` (from onboarding) > `detected` (from ACE scan) > `inferred` (from deps)
- The identity title should ONLY use `declared` + high-confidence `detected` tech, never raw dependency names like "drizzle"

### 1C. Filter noise from identity title
- **File:** `sovereign_developer_profile.rs:785-797` (`build_identity_summary`)
- Maintain a blocklist of techs that should never appear in the title (ORMs, utility libs, build tools): drizzle, express, lodash, webpack, eslint, prettier, etc.
- Only allow languages, frameworks, and platforms: rust, typescript, react, tauri, wgsl, python, go, etc.
- Add a `is_identity_worthy(tech: &str) -> bool` filter

### 1D. Ensure no other user hits this
- Add integration test: given a user with `tech_stack = [rust, typescript, tauri]` and `project_dependencies` containing `drizzle`, assert the title does NOT contain "Drizzle"
- Add test: stack with [python, rust, go, javascript] should produce "Python/Rust/Go developer" (alphabetical among equals is fine, but languages > libraries)

---

## Phase 2: Profile Loading Performance

**Problem:** Profile tab takes several seconds to load. The backend assembles 5 dimensions sequentially with duplicate work and an N+1 query pattern.

### 2A. Kill the N+1 signal trends query (HIGHEST IMPACT)
- **File:** `tech_radar_compute.rs:310-337` (`overlay_signal_trends`)
- Currently runs one `SELECT COUNT(*) FROM source_items WHERE LOWER(title) LIKE '%tech%'` per tech entry
- Replace with a SINGLE batch query: pre-fetch all source_items from last 30 days, build an in-memory frequency map, then look up each tech
- Or: use a CTE/subquery approach with `GROUP BY` to get all counts in one query

### 2B. Eliminate duplicate domain_profile builds
- **File:** `sovereign_developer_profile.rs:299` and `tech_radar_compute.rs:171`
- `build_domain_profile()` is called twice: once in `assemble_stack()` and once inside `compute_radar()`
- Build it once at the top of `assemble_profile()` and pass it through

### 2C. Cache tech radar in profile assembly
- The tech radar is computed in `assemble_preferences()` just to extract a small summary
- Either: cache the radar result and reuse, or compute a lightweight summary without the full radar pipeline
- Consider: compute radar lazily only when the user navigates to the tech radar view, not on profile load

### 2D. Add database index for signal trends
- **File:** `db/migrations.rs`
- Add: `CREATE INDEX IF NOT EXISTS idx_source_items_created_at ON source_items(created_at)`
- This helps the `created_at >= datetime('now', '-30 days')` filter

### 2E. Cache profile on backend
- After first computation, cache the `SovereignDeveloperProfile` in an `Arc<Mutex<Option<CachedProfile>>>` with a TTL (e.g., 60 seconds)
- Invalidate on: analysis complete, settings change, ACE scan complete
- Frontend already has a `profile-updated` event listener — emit this when cache is invalidated

### 2F. Frontend: show cached data immediately
- Store the last profile in Zustand persist
- On tab switch, show stale data instantly, refresh in background
- Add a subtle loading indicator (not a full spinner) for background refresh

---

## Phase 3: First-Run Experience Fix

**Problem:** Pro upsell modal appears before any value is delivered. Senior devs will close the app.

### 3A. Never show Pro gate on fresh install
- **Files:** Components that show the Pro modal/gate
- Add a condition: if `analysis_count == 0` (no analysis has been run yet), never show Pro gates
- After first successful analysis + briefing, Pro features can show subtle indicators but NOT modals

### 3B. Search bar — graceful degradation
- When Semantic Search is Pro-gated, the search bar should still work for basic text search (SQLite FTS)
- Only the "semantic" (vector) search requires Pro
- Show: "Upgrade for AI-powered semantic search" as a subtle hint below results, not a blocking modal

### 3C. Defer Pro modal to natural discovery
- Pro features should be discovered organically: user clicks "Insights" tab, sees a small lock icon, clicks it, THEN sees the upgrade prompt
- Never interrupt the core flow (Briefing > Results > Profile) with Pro gates

---

## Phase 4: Source Diversity

**Problem:** Results are JS/React-heavy despite the user being a Rust developer. The source mix doesn't match the detected stack.

### 4A. Stack-aware source recommendations
- After ACE detects the primary stack, suggest or auto-enable relevant sources:
  - Rust stack: This Week in Rust RSS, rust-lang blog, crates.io trending
  - Python stack: Python Weekly, PyPI trending
  - Go stack: Go Weekly, golang blog
- Show this as a one-time "We detected you work with Rust. Want to add Rust-focused sources?" prompt after first analysis

### 4B. Source weight by stack relevance
- In the scoring pipeline, boost items from sources that match the user's primary stack
- A Rust security advisory from a Rust-specific source should score higher than a generic React tutorial from HN

---

## Phase 5: Visual Polish — Game Components

**Problem:** 9 game components compiled with transparent backgrounds but none are visible in the screenshots.

### 5A. Verify game component mounting
- Check if components are actually rendering in the app
- Possible issues: component not imported, container has `display: none`, WebGPU not available in dev mode
- Debug in browser DevTools: check for `<game-*>` custom elements in the DOM

### 5B. Wire components to meaningful states
- Boot ring: show during app startup / analysis
- Status orb: ambient heartbeat in the header or sidebar
- Celebration burst: trigger on first successful briefing, achievement unlock
- Scan ring: show during source scanning
- Score fingerprint: show in the Profile tab as the user's visual identity

### 5C. Ensure transparent compositing works
- Verify WebGPU is available (fall back to WebGL2 if not)
- Check that the canvas has `position: absolute` or similar for overlay compositing
- Test on both the Tauri window and browser mode

---

## Phase 6: UI Text Cleanup

### 6A. Fix "garbage tolerance" label
- **Location:** Briefing view, bottom bar
- "Your garbage tolerance is decreasing by 100%" — replace with clear language
- Suggestion: "Signal quality improving — 100% fewer low-relevance items" or similar

### 6B. Fix untranslated i18n keys
- Screenshot shows `search.title` and `search.subtitle` as raw keys, not translated text
- Check `src/locales/en/` for missing translation entries

### 6C. Window title
- Current: "4DA Home - Ambient Intelligence"
- Recommendation: Just "4DA" — clean, memorable, no subtitle needed
- The subtitle adds nothing and "Ambient Intelligence" is internal jargon that means nothing to a new user

---

## Execution Order

| Priority | Phase | Impact | Effort |
|----------|-------|--------|--------|
| 1 | 1A-1D | Critical — profile accuracy | Small (1 file, ~50 lines) |
| 2 | 2A | High — kills dominant bottleneck | Medium (refactor 1 function) |
| 3 | 3A-3C | High — first impression | Small (conditional checks) |
| 4 | 6A-6C | Medium — polish | Tiny (text changes) |
| 5 | 2B-2F | Medium — profile speed | Medium (caching layer) |
| 6 | 4A-4B | Medium — result quality | Medium (source system) |
| 7 | 5A-5C | Medium — visual differentiation | Medium (component wiring) |

---

## Success Criteria

- [ ] Profile title accurately reflects user's primary language/framework (not dependency names)
- [ ] Profile tab loads in < 500ms (currently several seconds)
- [ ] No Pro modal appears before first analysis completion
- [ ] Search bar provides basic search on free tier without blocking modal
- [ ] Results include Rust-ecosystem content when user's stack is Rust
- [ ] At least 2 game components visually present in the app
- [ ] No raw i18n keys visible in the UI
- [ ] "Garbage tolerance" text replaced with clear language
- [ ] Window title is "4DA" (no subtitle)
