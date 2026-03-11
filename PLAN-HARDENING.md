# 4DA Hardening Plan — Ship-Ready Quality Gate

**Created:** 2026-03-12
**Goal:** Take 4DA from "impressive prototype" to "flawless v1 launch"
**Approach:** 3 phases, strictly ordered by launch-blocking impact. Each phase is independently shippable and testable. No new features — hardening only.

---

## Phase 1: ZERO-CRASH (Ship-Blocking)

Everything in this phase prevents the app from crashing, losing data, or looking broken on first launch.

### 1.1 Production Unwrap Audit
**Files:** ~15-20 unwrap() calls in production Rust code
**Priority:** CRITICAL — any one of these can crash the app

- [ ] `src-tauri/src/embeddings.rs:521` — unwrap in embedding path
- [ ] `src-tauri/src/game_engine.rs` — unwraps in rendering pipeline
- [ ] `src-tauri/src/state.rs:4` — state initialization unwrap
- [ ] Full grep: `grep -rn '\.unwrap()' src-tauri/src/ --include='*.rs'` excluding `_tests.rs`, `test`, `bench`
- [ ] Replace each with `.unwrap_or_default()`, `.context()`, or explicit match
- [ ] Zero unwrap() in any code path reachable from a Tauri command

**Verification:** `cargo clippy -- -D clippy::unwrap_used` on production modules (exclude test files)

### 1.2 TODO Audit — Blockers vs Wishlist
**Files:** Top 5 TODO-heavy files (989 total across 118 files)
**Priority:** HIGH — some TODOs may be real gaps

- [ ] `src-tauri/src/content_personalization/templates.rs` (86 TODOs) — classify each
- [ ] `src-tauri/src/source_fetching/mod.rs` (53 TODOs) — classify each
- [ ] `src-tauri/src/temporal.rs` (39 TODOs) — classify each
- [ ] `src-tauri/src/content_personalization/template_conditionals.rs` (10 TODOs)
- [ ] `src-tauri/src/content_personalization/commands.rs` (19 TODOs)
- [ ] Tag each as: `BLOCKER` (must fix), `ENHANCE` (post-launch), `DELETE` (stale)
- [ ] Fix all BLOCKERs, convert ENHANCE to GitHub issues, delete stale

**Verification:** Zero `// TODO` comments tagged BLOCKER remain

### 1.3 Cold Start — Zero-Config Test
**Priority:** CRITICAL — the 60-second promise

- [ ] Fresh Windows install: no settings.json, no API keys, no Ollama — app launches, shows content
- [ ] Fresh macOS install: same test
- [ ] Fresh Linux (Ubuntu) install: same test
- [ ] Onboarding flow completes without errors
- [ ] HN + Lobsters + RSS sources work with zero config (no API keys needed)
- [ ] Graceful degradation: no LLM = no briefing, but results still show
- [ ] Graceful degradation: no Ollama = no embeddings, keyword matching still works
- [ ] Error states show helpful messages, not stack traces

**Verification:** Screen recording of each platform cold start, from install to seeing relevant content

### 1.4 Frontend Test Floor to 1,000
**Current:** 894 tests across 63 files
**Target:** 1,000+ tests across 70+ files
**Priority:** HIGH — confidence gate for shipping paid tier

- [ ] Add keyboard navigation tests for: onboarding (4 steps), settings modal (6 tabs), results view (expand/collapse/feedback)
- [ ] Add error boundary tests: API timeout, malformed response, offline state
- [ ] Add accessibility tests: focus trap in modals, aria-live announcements, skip links
- [ ] Add store slice edge cases: concurrent updates, empty states, malformed data

**Verification:** `pnpm run test` shows 1,000+ passing, CI floor updated to 1000

---

## Phase 2: POLISH (Quality Differentiation)

Everything in this phase makes 4DA feel professional and intentional. Not ship-blocking, but the difference between "good enough" and "obviously premium."

### 2.1 IPC Type Safety
**Problem:** 253 Tauri commands return untyped JSON
**Priority:** MEDIUM — prevents subtle frontend bugs

- [ ] Audit `src/lib/commands.ts` — verify every command has a typed return
- [ ] Add runtime validation for top 20 most-called commands (zod schemas)
- [ ] Add `ts-rs` generation verification to CI (Rust struct changes auto-update TS types)
- [ ] Ghost command check runs in CI (already exists in scripts/)

**Verification:** `pnpm run validate:commands` passes with zero ghost commands and zero untyped returns

### 2.2 Slim the Frontend Hooks
**Problem:** `use-analysis.ts` (~10K) and `use-app-bootstrap.ts` (~10K) are too heavy
**Priority:** MEDIUM — reduces bundle size and improves maintainability

- [ ] Extract computation-heavy logic from `use-analysis` into Rust commands
- [ ] Extract initialization sequence from `use-app-bootstrap` into a Rust-side init command
- [ ] Hooks should be <500 lines each — thin wrappers over store + Tauri events
- [ ] Verify no functionality regression after extraction

**Verification:** Both hooks under 500 lines, all 1,000+ frontend tests pass

### 2.3 Binary Size Optimization
**Problem:** OCR + Archive features add ~100MB+ to binary, most users won't use them
**Priority:** MEDIUM — affects download size and first impression

- [ ] Make `ocr` feature opt-in (not default) in Cargo.toml
- [ ] Make `archive` feature opt-in (not default) in Cargo.toml
- [ ] Measure binary size: before vs after (target: <25MB download)
- [ ] Add feature flag UI in settings: "Enable document extraction (PDF, Office, archives)"
- [ ] Lazy-load OCR models on first use, not at startup

**Verification:** Default binary under 25MB, `cargo build --release` size reported in CI

### 2.4 Accessibility Hardening
**Priority:** MEDIUM — legal requirement in many jurisdictions, quality signal

- [ ] Focus trap in all modals (Settings, Onboarding, Article Reader)
- [ ] Arrow key navigation in results list
- [ ] Escape key closes all modals/drawers
- [ ] Tab order follows visual layout
- [ ] Screen reader announcement on view changes (aria-live regions)
- [ ] Color contrast audit (WCAG AA minimum)
- [ ] Add 20-30 keyboard navigation tests

**Verification:** axe-core audit shows zero critical/serious violations

---

## Phase 3: LAUNCH GATE (Final Checkpoint)

This phase is the pre-flight checklist. Nothing here is implementation — it's all verification.

### 3.1 Release Gate Script
- [ ] Run `scripts/release.sh` — all 9 steps must pass
- [ ] Sovereignty score >80
- [ ] All cadences current (no overdue)
- [ ] Version numbers consistent (Cargo.toml, package.json, tauri.conf.json)
- [ ] Build succeeds on all 3 platforms

### 3.2 Security Final Audit
- [ ] `cargo audit` — zero known vulnerabilities
- [ ] No API keys in any committed file (grep for sk-, pk_, api_key)
- [ ] CSP headers verified in production build
- [ ] settings.json not in git history
- [ ] PLAN-SECURITY-KEYMANAGEMENT.md Phase 1 (encrypted key storage) complete

### 3.3 Legal Ready
- [ ] Bank account open (CBA business account)
- [ ] Trademark deeds signed, witnessed, filed with IP Australia
- [ ] Privacy policy reflects actual data practices
- [ ] Terms of service reviewed
- [ ] FSL-1.1-Apache-2.0 license header in all source files

### 3.4 Distribution Ready
- [ ] GitHub release pipeline tested (draft release, verify artifacts)
- [ ] Auto-updater tested (install v0.9, update to v1.0)
- [ ] npm placeholder published (`@4da/mcp-server`)
- [ ] crates.io placeholder published
- [ ] 4da.ai landing page reflects current features
- [ ] shop.4da.ai merch store tested (end-to-end purchase)

### 3.5 First User Experience Test
- [ ] Cold start on each platform — time to first relevant content <60s
- [ ] Onboarding → calibration → first briefing — smooth flow
- [ ] Settings: add API key → briefing quality improves noticeably
- [ ] Save/dismiss items → scoring adapts within 2-3 cycles
- [ ] No error toasts, no console errors, no layout shifts

---

## Execution Order

```
Phase 1.1 (unwrap audit)     ─┐
Phase 1.2 (TODO audit)       ─┼─ Can run in parallel
Phase 1.3 (cold start test)  ─┤
Phase 1.4 (test floor)       ─┘
          │
          ▼
Phase 2.1 (IPC types)        ─┐
Phase 2.2 (slim hooks)       ─┼─ Can run in parallel
Phase 2.3 (binary size)      ─┤
Phase 2.4 (accessibility)    ─┘
          │
          ▼
Phase 3 (launch gate)        ─── Sequential, all must pass
```

## Rules

1. **No new features.** If it's not in this plan, it waits until post-launch.
2. **Every change gets tests.** No exceptions.
3. **Each phase independently verified.** Don't start Phase 2 until Phase 1 is green.
4. **Commit atomically.** One concern per commit. No mixing.
5. **If something breaks, fix it before moving on.** No debt accumulation.
