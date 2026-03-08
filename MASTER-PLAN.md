# 4DA Master Fortification Plan

> Generated: 8 March 2026
> Source: 5-agent parallel deep audit (unwrap, frontend, architecture, security, UX)
> Scope: 146K lines across 528 files, 2,454 tests, 46 DB tables

---

## The Diagnosis

**Tech is 9/10. Feel is 6/10.** The PASIFA scoring engine is world-class but invisible.
The architecture is sound but carrying ~3,000 lines of dead weight. Security is production-grade
with zero critical vulnerabilities. The frontend is clean but 635 design token violations
make it look inconsistent. Users trust the system — they don't love it yet.

### What the Audit Found (Hard Numbers)

| Category | Finding | Severity |
|----------|---------|----------|
| Production unwrap() | Nearly ALL in test blocks — production code is safe | CLEAR |
| Timer/listener leaks | 0 leaks across 93 creates — 100% cleanup rate | CLEAR |
| Design tokens | **635** `text-gray-*` violations across 50+ files | CRITICAL |
| Error boundaries | 12/12 lazy views wrapped — excellent | CLEAR |
| Hardcoded English strings | 0 violations — i18n is comprehensive | CLEAR |
| Accessibility | ~125 icon-only buttons missing aria-labels | MODERATE |
| Dead code annotations | 161 total — 4 types should be deleted now | LOW |
| SQL injection risk | 0 vectors — all queries parameterized | CLEAR |
| API key exposure | 0 leaks — keys never in logs/errors/frontend | CLEAR |
| Critical vulnerabilities | 0 found | CLEAR |
| Unused DB tables | 8-10 tables with zero queries | LOW |
| Console.log in production | 0 — all logging is console.warn in catch handlers | CLEAR |
| State management | 0 anti-patterns — hooks, effects, cleanup all correct | CLEAR |
| Error type migration | 383 functions still return `Result<T, String>` | MEDIUM |
| Loading states | ~50% of views have complete load/empty/error/retry | MODERATE |
| Scoring transparency | PASIFA algorithm completely invisible to users | HIGH |

### What Was Already Better Than Expected
- Unwrap hygiene (my earlier assessment of 473 risky unwraps was wrong — production is clean)
- Timer/listener cleanup (100% — no memory leaks)
- i18n coverage (zero hardcoded strings)
- Security posture (zero critical, zero SQL injection, zero key exposure)
- State management (zero anti-patterns)

---

## The Plan

### Execution Philosophy
- **Phases ordered by impact-per-hour**, not by category
- **Each phase is independently committable** — no phase depends on another
- **Parallel execution where possible** — independent phases can run simultaneously
- **Every phase has a verification step** — measurable, not vibes

---

## PHASE 1: Design Token Migration (CRITICAL)
**Impact:** HIGHEST | **Effort:** 3-4 hours | **Files:** 50+

635 instances of raw `text-gray-*` classes violate the design system and fail WCAG AA
contrast on `#0A0A0A` backgrounds. This is the single biggest visual quality issue.

### Mapping
```
text-gray-300  →  text-white (if heading/emphasis) or text-text-secondary
text-gray-400  →  text-text-secondary (#A0A0A0, ratio 6.3:1 ✅)
text-gray-500  →  text-text-muted (#666666, ratio 4.0:1 — decorative OK)
text-gray-600  →  text-text-muted
text-gray-700  →  text-text-muted (or remove if decorative)
```

### Execution
- 3 parallel agents, each taking ~17 files
- Agent A: A-F components (AboutPanel, ActionBar, App, BriefingCard, etc.)
- Agent B: G-P components (GhostPreview, IntelligenceProfileCard, PlaybookView, etc.)
- Agent C: Q-Z components + settings + search (ResultsView, TechRadar, ViewTabBar, etc.)
- Each agent applies mapping, preserves non-gray Tailwind classes
- Manual review of edge cases (hover states, borders, gradients)

### Verification
```bash
grep -rn "text-gray-" src/components/ --include="*.tsx" | wc -l
# Target: 0
```

---

## PHASE 2: Scoring Transparency (HIGH — Transforms Experience)
**Impact:** HIGH | **Effort:** 6-8 hours | **Files:** 3-5

The PASIFA scoring engine is the competitive edge. Users see a score badge (0.72) with
zero explanation. Making this visible is the single highest-leverage product improvement.

### Implementation
1. **Score tooltip on hover** — Show top 3 scoring factors:
   - "Matched your Rust stack (context: 0.4)"
   - "High freshness — posted 2 hours ago (freshness: 1.2x)"
   - "Confirmed by 3 independent signals (confirmation: 1.5x)"

2. **ScoreBreakdownDrawer enhancement** — Already exists, needs:
   - Human-readable factor names (not raw field names)
   - Visual bar chart of factor contributions
   - "Why this scored higher than..." comparison mode

3. **Feedback impact counter** — "Your feedback has shaped 52 future signals"
   Track: total feedback events × items affected by affinity changes

### Files
- `src/components/result-item/ScoreBreakdownDrawer.tsx` — enhance existing drawer
- `src/components/ResultItem.tsx` — add tooltip trigger
- `src/locales/en/ui.json` — add factor name translations
- `src-tauri/src/relevance.rs` — add human-readable factor export (if not already)

### Verification
- Score badges show tooltip on hover
- ScoreBreakdownDrawer shows factor bars with human names
- Feedback counter appears in profile view

---

## PHASE 3: Loading State Consistency (MODERATE)
**Impact:** MODERATE | **Effort:** 4-5 hours | **Files:** 7-10

50% of async views are missing error states with retry. Silent failures erode trust.

### Pattern to implement everywhere
```tsx
{loading ? <Skeleton /> : error ? <ErrorRetry onRetry={reload} message={humanError} /> : data ? <Content /> : <EmptyState />}
```

### Views needing error states + retry
| View | Loading | Empty | Error+Retry | Status |
|------|---------|-------|-------------|--------|
| PlaybookView | MISSING | ✅ | MISSING | Fix |
| DecisionMemory | MISSING | ✅ | MISSING | Fix |
| ChannelsView | MISSING | ✅ | MISSING | Fix |
| TechRadar | ✅ | ✅ | MISSING | Fix |
| BriefingView | ✅ | ✅ | MISSING | Fix |
| SovereignDeveloperProfile | ✅ | ✅ | MISSING | Fix |
| DigestView | ✅ | ✅ | MISSING | Fix |

### Error message humanization
```
"Error"                          → "Something went wrong. Try again?"
"Network error"                  → "Can't reach the internet. Check your connection."
"UNIQUE constraint failed"       → "This item already exists."
"Rate limited"                   → "Too many requests. Waiting 60 seconds..."
"Ollama not running"             → "Local AI is offline. Start Ollama or add an API key."
```

### Verification
- Every view handles: loading → success → error+retry → empty
- Error messages are human-readable (no raw Rust errors shown)

---

## PHASE 4: Accessibility Pass (MODERATE)
**Impact:** MODERATE | **Effort:** 3-4 hours | **Files:** 30+

125 icon-only buttons missing `aria-label`. Good foundation — just needs labeling pass.

### Execution
- Grep for `<button` without `aria-label` that contain only SVG/icon children
- Add context-specific labels: `aria-label={t('action.copyToClipboard')}`
- Add `aria-hidden="true"` to decorative SVGs
- Add focus management: move focus to main content on view switch (ViewRouter.tsx)

### Priority files (most violations)
1. Copy buttons in DeveloperDna, TemplateLibrary, ScoreBreakdownDrawer
2. Menu trigger buttons in ActionBar overflow
3. Icon-only buttons in ResultItem actions
4. SVG interactive elements in TechRadar

### Verification
```bash
# Run axe-core or similar
npx @axe-core/cli src/
```

---

## PHASE 5: Dead Code Purge (LOW — Quick Win)
**Impact:** LOW | **Effort:** 1 hour | **Files:** 5-10

### Immediate deletes
1. `community_intelligence.rs` lines 35-67: 4 unused structs (CommunityContribution,
   ScoringWeightContribution, CommunityWeights, ProfileAdjustment)
2. `calibration_commands.rs` line 196: unused `persona_display_name()` function
3. `agent_memory.rs` line 145: unused `recall_by_tags()` (or wire into frontend)

### Investigate
4. `sovereign_developer_profile.rs` (1,386 lines) — check if any frontend component
   calls its commands. If not: move to archive or delete.
5. `game_engine.rs` (1,163 lines) — verify if only used internally by game_commands

### Verification
```bash
cargo check 2>&1 | grep "warning:" | grep -v "ts-rs"
# Target: 0 warnings
```

---

## PHASE 6: Security Hardening (MEDIUM)
**Impact:** MEDIUM | **Effort:** 2-3 hours | **Files:** 3-5

No critical vulnerabilities. These are defense-in-depth improvements.

### 6.1 ACE path allowlist
```rust
// ace_commands.rs — restrict paths to home directory
let canonical = fs::canonicalize(&path)?;
let home = dirs::home_dir().ok_or("Cannot determine home directory")?;
if !canonical.starts_with(&home) {
    return Err("Path must be within home directory".into());
}
```

### 6.2 File permissions on settings.json (Unix only)
```rust
// settings/mod.rs — after writing settings
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
}
```

### 6.3 API key input validation (frontend)
```tsx
// AIProviderSection.tsx — trim whitespace, enforce length
const sanitized = value.trim();
if (sanitized.length > 0 && sanitized.length < 20) {
    setError(t('settings.apiKeyTooShort'));
    return;
}
```

### Verification
- ACE scan rejects paths outside home directory
- settings.json has 0600 permissions on Linux/Mac
- API key input trims whitespace and validates length

---

## PHASE 7: Celebratory Micro-Interactions (HIGH — Feel)
**Impact:** HIGH | **Effort:** 3-4 hours | **Files:** 5-8

The difference between "tool" and "companion" is delight moments.

### 7.1 Feedback button animations
- Save: green pulse glow + count badge increment
- Dismiss: subtle fade-out slide
- First save ever: confetti burst (using existing game-celebration-burst component)

### 7.2 Analysis narration personalization
Replace generic: "Fetching from Hacker News"
With personalized: "Found 3 Rust async patterns in your projects. Watching for updates..."

### 7.3 Decision window pop-in cards
When a decision window matches new content: animated card slides in from right edge
with context and "Act" button, not just a toast.

### 7.4 Keyboard shortcut visual feedback
When user presses `r` for analysis: brief flash on the analysis button +
toast "Analysis started (press R again to cancel)"

### Verification
- Manual testing of each interaction
- No janky animations (requestAnimationFrame, not setTimeout)

---

## PHASE 8: First-Run Experience Polish (HIGH — Retention)
**Impact:** HIGH | **Effort:** 4-5 hours | **Files:** 3-5

4-10 minutes before value is too long. Target: value in 90 seconds.

### 8.1 Show estimated time in first-run transition
```tsx
<p className="text-sm text-text-secondary">
  Scanning · ~{estimatedSeconds}s remaining
</p>
```

### 8.2 Personalize narration to detected stack
If ACE detects Rust projects: "Found Rust in 3 projects. Watching for async patterns,
crate updates, and security advisories..."

### 8.3 Reduce setup screens
- Merge "Taste Test" into calibration (one screen, not two)
- Make stack detection show what's being scanned in real-time
- Auto-advance when detection completes (no "Continue" button wait)

### 8.4 Auto-start analysis without waiting for user click
Already implemented (3s auto-start from this session). Verify it works in
fresh install scenario.

### Verification
- Fresh install → useful content in < 2 minutes
- User never sees a screen that appears "frozen"
- Progress indicators at every stage

---

## PHASE 9: Feature-Gate Heavy Dependencies (LOW)
**Impact:** LOW | **Effort:** 1-2 hours | **Files:** 2 (Cargo.toml, lib.rs)

OCR (ocrs, rten) and archive extraction (zip, tar, flate2) are only used in
extractors/ and add binary size. Feature-gate them.

```toml
[features]
default = ["ocr", "archive"]
ocr = ["dep:ocrs", "dep:rten", "dep:image"]
archive = ["dep:zip", "dep:tar", "dep:flate2"]
```

### Verification
```bash
cargo build --no-default-features  # Should compile without OCR/archive
cargo build                         # Should compile with everything (default)
```

---

## PHASE 10: Database Table Audit (LOW)
**Impact:** LOW | **Effort:** 1-2 hours | **Files:** 1-2

8-10 tables appear to have zero production queries. Confirm and document.

### Tables to investigate
- `command_history` — search for SELECT/INSERT/UPDATE on this table name
- `git_commit_history` — same
- `chunk_sentiment` — same
- `item_relationships` — same
- `extraction_jobs` — same
- `query_cache` — same

### Action
For each zero-query table: add comment to migrations.rs explaining status
(experimental, planned, deprecated). Do NOT drop tables in running app —
that would lose user data if they upgrade. Mark as deprecated in next migration.

---

## PHASE 11: Consolidate HTTP Clients (LOW)
**Impact:** LOW | **Effort:** 2 hours | **Files:** 5

5 locations create reqwest::Client independently. Consolidate to single Lazy global
with shared configuration (User-Agent, timeout, connection pool).

### Files
- `embeddings.rs` — has Lazy client (keep as base)
- `llm.rs` — creates per-request (consolidate)
- `health.rs` — references global (verify)
- `sources/github.rs` — ad-hoc (consolidate)
- `ace/watcher.rs` — ad-hoc (consolidate)

### Implementation
```rust
// src/http.rs
pub(crate) static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent("4DA/1.0")
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()
        .expect("HTTP client initialization failed")
});
```

---

## PHASE 12: Error Type Migration (MEDIUM — Long-term)
**Impact:** MEDIUM | **Effort:** 20+ hours | **Files:** 50+

383 functions still return `Result<T, String>` instead of typed `FourDaError`.
This is the largest tech debt item but lowest priority because the bridge
(`From<String> for FourDaError`) means everything compiles.

### Strategy
- Do NOT batch-migrate all 383 at once
- Migrate during feature work: when touching a function, convert it
- Priority: scoring pipeline, analysis, ACE (highest-traffic paths)
- Keep the `From<String>` bridge indefinitely (it's a 5-line safety net)

---

## Execution Matrix

| Phase | Impact | Effort | Parallel? | Prerequisite |
|-------|--------|--------|-----------|--------------|
| 1. Design Tokens | CRITICAL | 3-4h | Yes (3 agents) | None |
| 2. Scoring Transparency | HIGH | 6-8h | Yes | None |
| 3. Loading States | MODERATE | 4-5h | Yes | None |
| 4. Accessibility | MODERATE | 3-4h | Yes (after Phase 1) | Phase 1 |
| 5. Dead Code | LOW | 1h | Yes | None |
| 6. Security | MEDIUM | 2-3h | Yes | None |
| 7. Micro-Interactions | HIGH | 3-4h | Yes | None |
| 8. First-Run Polish | HIGH | 4-5h | Yes | None |
| 9. Feature Gates | LOW | 1-2h | Yes | None |
| 10. DB Audit | LOW | 1-2h | Yes | None |
| 11. HTTP Clients | LOW | 2h | Yes | None |
| 12. Error Types | MEDIUM | 20h+ | Gradual | None |

### Optimal Parallel Execution

**Wave 1** (can all run simultaneously):
- Phase 1: Design Tokens (3 agents)
- Phase 5: Dead Code (1 agent)
- Phase 6: Security (1 agent)
- Phase 9: Feature Gates (1 agent)

**Wave 2** (after Wave 1):
- Phase 2: Scoring Transparency
- Phase 3: Loading States
- Phase 4: Accessibility (needs Phase 1 tokens in place)
- Phase 10: DB Audit

**Wave 3** (after core UX is solid):
- Phase 7: Micro-Interactions
- Phase 8: First-Run Polish
- Phase 11: HTTP Clients

**Ongoing:**
- Phase 12: Error Type Migration (during regular feature work)

---

## Success Metrics

| Metric | Current | Target | Phase |
|--------|---------|--------|-------|
| `text-gray-*` violations | 635 | 0 | Phase 1 |
| Icon buttons with aria-label | 142/267 | 267/267 | Phase 4 |
| Views with error+retry | ~6/13 | 13/13 | Phase 3 |
| Dead code annotations | 161 | <140 | Phase 5 |
| Security findings (MEDIUM+) | 4 | 0 | Phase 6 |
| First-run time to value | 4-10 min | <2 min | Phase 8 |
| Score explanation visible | No | Yes | Phase 2 |
| Feedback delight animations | No | Yes | Phase 7 |

---

## What This Plan Does NOT Cover

- Company registration / legal (human-only)
- Keygen account setup (human-only)
- Clean machine install test (manual)
- Payment processing integration (post-launch)
- Marketing / landing page (separate workstream)
- Community beta program (requires human decisions)

---

## The Bottom Line

**40-50 hours of focused work transforms 4DA from "impressive tool" to "essential companion."**

The foundation is genuinely strong — zero critical security issues, clean state management,
comprehensive i18n, disciplined error handling. The work ahead is about communication (make
the scoring visible), consistency (design tokens, loading states), and delight (micro-interactions,
personalized narration).

Phase 1 (design tokens) is the highest-ROI single action: 3-4 hours to fix the most visible
quality issue across the entire app. Phase 2 (scoring transparency) is the highest-ROI product
action: 6-8 hours to make the competitive edge visible to every user, every session.
