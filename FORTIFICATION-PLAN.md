# 4DA Fortification Plan — Launch Hardening

> Created: 8 March 2026
> Status: ACTIVE
> Goal: Every system hardened, every edge case handled, zero surprises at launch

---

## Current State (Post-Cleanup)

| Metric | Value |
|--------|-------|
| Rust lib tests | 1,615 passing |
| Frontend tests | 792 passing |
| Integration tests | 114 passing |
| Rust warnings | 21 (all from unregistered command functions) |
| TypeScript errors | 0 |
| Dead code removed this session | ~9,676 lines |
| Files approaching size limit | 7 (warnings only, 0 errors) |

### Already Done (from audit sessions)
- [x] Fix all 9 production unwrap() NaN panics
- [x] Privacy Policy + Terms of Service
- [x] SECURITY.md with responsible disclosure
- [x] NOTICE file (third-party licenses)
- [x] GitHub issue templates (bug, feature, source adapter)
- [x] CONTRIBUTING.md
- [x] LLM auto-fallback (cloud -> Ollama)
- [x] LLM daily token limit enforcement
- [x] File size guards on translation overrides
- [x] Non-atomic paired write fixes (db transactions)
- [x] Error context on bare ? operators
- [x] Centralized rate limiter for source fetching
- [x] Coach hybrid (LLM system deleted, templates kept in Playbook)
- [x] 11 dead modules deleted (~9,676 lines)

---

## PHASE 1: Zero Warnings, Zero Dead Weight
**Scope:** Eliminate all compiler warnings, remove dead command registrations
**Risk:** LOW | **Effort:** 2-3 hours | **Impact:** Clean build, smaller binary

### 1.1 Suppress 21 Rust warnings from unregistered commands
The 7 internal-only modules (suns, competing_tech, content_dna, decision_advantage,
domain_profile, semantic_diff, job_queue) have `#[tauri::command]` functions that are
no longer registered in the invoke_handler. These generate "never used" warnings.

**Action:** Remove `#[tauri::command]` attribute and `pub async` from functions that
are only called internally. Keep only the internal helper functions.

Files:
- `src-tauri/src/suns_commands.rs` — 8 warnings
- `src-tauri/src/decision_advantage_commands.rs` — 4 warnings
- `src-tauri/src/semantic_diff.rs` — 1 warning
- Other scattered warnings

**Verification:** `cargo check 2>&1 | grep warning | wc -l` should be 0.

### 1.2 Remove orphaned frontend types
Types that corresponded to deleted modules may still exist in type files.

**Action:** Grep for type imports that are never used. Clean up `src/types/innovation.ts`,
`src/types/index.ts`, `src/lib/commands.ts`.

**Verification:** `npx tsc --noEmit` clean (already clean, verify no dead exports).

---

## PHASE 2: Keygen License Validation
**Scope:** Replace placeholder, verify Pro tier gate works end-to-end
**Risk:** MEDIUM | **Effort:** 1-2 hours | **Impact:** Pro tier actually enforceable

### 2.1 Replace YOUR_ACCOUNT_ID placeholder
- File: `src-tauri/src/settings/license.rs:13`
- Current: `const KEYGEN_ACCOUNT_ID: &str = "YOUR_ACCOUNT_ID";`
- **Requires:** User creates Keygen.sh account, gets account ID
- **Action:** Replace placeholder with real account ID

### 2.2 Test license validation flow
- Test with invalid key -> should reject gracefully
- Test with no key -> free tier should work fully
- Test with expired cache -> should re-validate
- Test offline behavior -> cached validation should persist 24h

### 2.3 Verify Pro tier gates
- `require_streets_feature()` blocks correctly
- Free tier users see appropriate upgrade prompts
- Templates (now in Playbook) work without Pro

---

## PHASE 3: File Size Compliance
**Scope:** Split 7 files approaching size limits before they become errors
**Risk:** LOW | **Effort:** 3-4 hours | **Impact:** Prevents CI failures

Current warnings (will become errors if they grow):

| File | Lines | Limit | Action |
|------|-------|-------|--------|
| `privacy_tests.rs` | 971 | 600 | Split by test category (scoring, embedding, context) |
| `db/mod.rs` | 756 | 600 | Extract migration logic to `db/migrations.rs` |
| `scoring/simulation/persona_data.rs` | 728 | 600 | Split persona arrays into separate data files |
| `settings/license.rs` | 656 | 600 | Extract Keygen HTTP logic to helper |
| `scoring/simulation/feedback_sim.rs` | 644 | 600 | Extract test fixtures |
| `translation_commands.rs` | 622 | 600 | Extract validation helpers |
| `PlaybookView.tsx` | 433 | 350 | Extract TemplateLibrary section (already separate component, reduce inline JSX) |

---

## PHASE 4: Error Boundary Hardening
**Scope:** Every user-facing path handles failure gracefully
**Risk:** MEDIUM | **Effort:** 4-5 hours | **Impact:** No crashes in production

### 4.1 Frontend error boundaries
- Verify every lazy-loaded view has `<ViewErrorBoundary>`
- Add error boundaries around:
  - Settings modal content
  - Template library (new in Playbook)
  - Any component using `invoke()` directly

### 4.2 Backend panic prevention
- Audit remaining `partial_cmp` calls (scan completed, but verify no new ones)
- Verify all database operations have proper error propagation
- Ensure monitoring scheduler can't crash the app (already uses catch_unwind in analysis,
  verify other scheduled tasks)

### 4.3 Graceful degradation paths
- No API key configured -> free briefing works, LLM features show "configure API key" message
- Ollama not running -> embedding returns zero vectors, app still functions
- Database locked/corrupted -> app starts with fresh DB, warns user
- Network offline -> cached results shown, sources marked stale
- Invalid settings.json -> reset to defaults with warning

### 4.4 IPC command error handling
Every Tauri command should return `Result<T, String>` with meaningful error messages.
Audit the ~80 remaining registered commands for bare `?` without context.

---

## PHASE 5: First-Run Experience Audit
**Scope:** New user installs 4DA -> sees value within 60 seconds
**Risk:** HIGH (this is the product) | **Effort:** 3-4 hours | **Impact:** User retention

### 5.1 Cold start validation
- Install fresh (empty data dir)
- Verify splash screen -> welcome step -> quick setup flows without errors
- Verify first scan completes and shows results
- Time the entire flow: target < 60 seconds to first content

### 5.2 Zero-config content
- Verify default sources (HN, Reddit, RSS) fetch without API keys
- Verify free briefing generates without LLM key
- Verify STREETS playbook loads and is navigable
- Verify templates are accessible in Playbook view

### 5.3 Error state UX
- What does the user see if first scan fails?
- What if no internet connection on first run?
- What if the database can't be created (permissions)?

---

## PHASE 6: Security Surface Review
**Scope:** BYOK app handling API keys — security is existential
**Risk:** HIGH | **Effort:** 3-4 hours | **Impact:** Prevents key exfiltration

### 6.1 API key storage audit
- Verify keys are stored in `data/settings.json` (gitignored)
- Verify keys are never logged (grep for key values in tracing output)
- Verify keys are never sent to any endpoint other than the intended API
- Verify keys are not included in error messages or crash reports

### 6.2 IPC surface audit
- All Tauri commands are registered in lib.rs
- Verify no command exposes raw file system access (command_runner deleted)
- Verify no command allows arbitrary code execution (command_runner deleted)
- Verify template content is sanitized before rendering

### 6.3 Update mechanism
- Verify Tauri updater uses HTTPS
- Verify update signatures are validated
- Verify no MITM attack surface on update channel

### 6.4 Dependency audit
- Run `cargo audit` for known vulnerabilities
- Run `pnpm audit` for frontend vulnerabilities
- Address any critical/high findings

---

## PHASE 7: Performance & Binary Size
**Scope:** App launches fast, runs smooth, binary isn't bloated
**Risk:** LOW | **Effort:** 2-3 hours | **Impact:** Professional feel

### 7.1 Startup performance
- Measure cold start time (splash -> ready)
- Target: < 3 seconds on mid-range hardware
- Identify any blocking operations in the startup path

### 7.2 Binary size check
- Build release: `pnpm run tauri build`
- Check installer size (target: < 30MB)
- If bloated: check for debug symbols, unnecessary assets

### 7.3 Memory usage
- Run app for 1 hour with monitoring
- Check for memory leaks (growing RSS)
- Verify SQLite connections are properly closed

### 7.4 Source fetch performance
- Verify rate limiter doesn't cause excessive delays
- Verify parallel fetching works (multiple sources simultaneously)
- Verify timeout handling (no hanging requests)

---

## PHASE 8: Test Coverage Gaps
**Scope:** Critical paths have test coverage, edge cases are handled
**Risk:** MEDIUM | **Effort:** 4-5 hours | **Impact:** Confidence in shipping

### 8.1 Critical path tests (missing)
- First-run onboarding flow (component test)
- Settings save/load roundtrip
- Source fetch -> score -> display pipeline (integration)
- Taste test complete flow -> profile saved
- Playbook lesson navigation
- Template library load and display

### 8.2 Edge case tests
- Empty database (first run)
- Corrupted settings.json
- Very long article titles (truncation)
- Unicode in all text fields
- Concurrent database writes
- Rate limiter under load

### 8.3 Regression tests for fixed bugs
- NaN panic in scoring (fixed: unwrap_or)
- UTF-8 panic in streets_engine keyword matching (fixed)
- Non-atomic paired writes (fixed: transactions)

---

## PHASE 9: Documentation Completeness
**Scope:** Every required document exists and is accurate
**Risk:** LOW | **Effort:** 2-3 hours | **Impact:** Legal compliance, community trust

### 9.1 Already complete
- [x] LICENSE (FSL-1.1-Apache-2.0)
- [x] NOTICE (third-party dependencies)
- [x] SECURITY.md (responsible disclosure)
- [x] CONTRIBUTING.md
- [x] Privacy Policy
- [x] Terms of Service
- [x] GitHub issue templates

### 9.2 Needed
- [ ] CODE_OF_CONDUCT.md — Standard Contributor Covenant
- [ ] Update LICENSE copyright to "4DA Systems Pty Ltd" (after company registered)
- [ ] README.md trademark notice in footer
- [ ] Verify all docs reference correct contact info (support@4da.ai)

---

## PHASE 10: Build & Release Pipeline
**Scope:** Repeatable, verified release process
**Risk:** MEDIUM | **Effort:** 3-4 hours | **Impact:** Can ship confidently

### 10.1 Release build verification
- `pnpm run tauri build` completes without errors
- Installer runs on clean Windows machine
- App launches, shows splash, reaches main UI
- All default sources fetch successfully
- Settings persist across app restart

### 10.2 Update mechanism test
- Verify Tauri updater endpoint is configured
- Verify update notification appears (UpdateBanner component)
- Test install flow (manual trigger)

### 10.3 Release checklist (for launch day)
1. All tests passing (cargo test + pnpm test)
2. Release build successful
3. Installer tested on clean machine
4. Version number correct in tauri.conf.json
5. Changelog written
6. Git tag created
7. GitHub release published with installer
8. Website updated (4da.ai)

---

## Execution Order (Recommended)

| Priority | Phase | Hours | Why this order |
|----------|-------|-------|----------------|
| 1 | Phase 1: Zero Warnings | 2-3h | Quick win, clean foundation |
| 2 | Phase 6: Security Review | 3-4h | BYOK security is existential |
| 3 | Phase 4: Error Boundaries | 4-5h | No crashes in production |
| 4 | Phase 5: First-Run Audit | 3-4h | This IS the product |
| 5 | Phase 3: File Size Compliance | 3-4h | Prevents future CI failures |
| 6 | Phase 2: Keygen License | 1-2h | Blocked on user creating account |
| 7 | Phase 8: Test Coverage | 4-5h | Confidence before shipping |
| 8 | Phase 7: Performance | 2-3h | Polish, not blocking |
| 9 | Phase 9: Documentation | 2-3h | Low effort, high trust |
| 10 | Phase 10: Build Pipeline | 3-4h | Final gate before launch |

**Total estimated: ~30-37 hours of focused work**

---

## What This Plan Does NOT Cover (User-Only Tasks)

These are items from PRE-LAUNCH-PLAN.md that require human action:
- Company registration (Director ID, ASIC, ABN)
- Trademark assignments to company
- US/EU trademark filings
- Bank account setup
- Payment processing (Stripe/LemonSqueezy/Paddle)
- Insurance
- Accounting setup
- Domain portfolio
- Keygen account creation (needed for Phase 2)
- npm/crates.io name reservation

---

## Success Criteria

The app is launch-ready when ALL of these are true:
- [ ] 0 Rust warnings, 0 TypeScript errors
- [ ] All tests passing (target: 2,500+)
- [ ] Release build completes and installs on clean machine
- [ ] First-run flow works in < 60 seconds
- [ ] No API keys in logs, errors, or telemetry
- [ ] Every user-facing error has a graceful fallback
- [ ] cargo audit + pnpm audit show 0 critical vulnerabilities
- [ ] All legal docs in place and accurate
- [ ] Keygen license validation working (placeholder replaced)
- [ ] File size limits: 0 warnings, 0 errors
