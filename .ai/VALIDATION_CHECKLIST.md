# Validation Checklist
## Pre-Completion Requirements for All Tasks

---

## Purpose

A task is NOT complete until this checklist passes. This prevents:
- Claiming "done" when tests fail
- Introducing invariant violations
- Scope creep going unnoticed
- Missing validation artifacts

---

## The Checklist

### Build Validation
- [ ] **Rust builds:** `cargo build` succeeds without errors
- [ ] **Frontend builds:** `npm run build` succeeds without errors
- [ ] **No new warnings:** (or warnings explicitly acknowledged)

### Test Validation
- [ ] **Rust tests pass:** `cargo test --all`
- [ ] **Frontend tests pass:** `npm run test`
- [ ] **New logic has tests:** (if applicable)

### Lint Validation
- [ ] **Rust formatting:** `cargo fmt --check` passes
- [ ] **Rust lints:** `cargo clippy -- -D warnings` passes
- [ ] **Frontend lints:** `npm run lint` passes
- [ ] **TypeScript types:** `npm run typecheck` passes

### Invariant Validation
- [ ] **No invariant violations:** Check `.ai/INVARIANTS.md` for relevant invariants
- [ ] **Privacy preserved:** No data leaks, keys protected
- [ ] **Performance bounds:** Within specified limits

### Scope Validation
- [ ] **Only approved files modified:** Check against task spec
- [ ] **No unplanned changes:** No "while I'm here" improvements
- [ ] **No new abstractions:** Unless explicitly approved

### Documentation Validation
- [ ] **Comments where needed:** Only for non-obvious logic
- [ ] **No excessive comments:** Self-documenting code preferred

---

## Validation Report Template

When completing a task, produce this report:

```markdown
## Validation Report

**Task:** [Task name]
**Date:** [Completion date]

### Build Status
- Rust: [PASS/FAIL]
- Frontend: [PASS/FAIL]

### Test Status
- Rust Tests: [PASS/FAIL] (X passed, Y failed)
- Frontend Tests: [PASS/FAIL] (X passed, Y failed)

### Lint Status
- cargo fmt: [PASS/FAIL]
- cargo clippy: [PASS/FAIL]
- ESLint: [PASS/FAIL]
- TypeScript: [PASS/FAIL]

### Invariant Check
- Relevant Invariants: [List which were checked]
- Violations: [NONE or describe]

### Files Modified
- `path/to/file1.rs` - [brief description of change]
- `path/to/file2.ts` - [brief description of change]

### Unexpected Changes
[NONE or list any deviations from task spec]

### Notes
[Any observations, concerns, or follow-up items]
```

---

## Quick Validation Commands

```bash
# Full validation (run all checks)
npm run validate:all

# Individual checks
cargo build                           # Rust build
cargo test --all                      # Rust tests
cargo fmt --check                     # Rust formatting
cargo clippy -- -D warnings           # Rust lints
npm run build                         # Frontend build
npm run test                          # Frontend tests
npm run lint                          # ESLint
npm run typecheck                     # TypeScript
```

---

## When Validation Fails

### If Build Fails
1. Read the full error message
2. Identify the root cause (not just the symptom)
3. Fix the specific issue
4. Re-run full validation

### If Tests Fail
1. Run failed test in isolation
2. Understand what's being tested
3. Determine if:
   - Test is correct and code is wrong
   - Code is correct and test needs update
   - Both need changes
4. Fix and re-run

### If Lints Fail
1. Review each lint warning/error
2. Apply automatic fixes where available
3. Manually fix remaining issues
4. Never disable lints without team approval

### If Invariants Violated
1. **STOP** - This is serious
2. Identify which invariant is violated
3. Determine if change is necessary or accidental
4. If accidental: revert to restore invariant
5. If necessary: discuss with team before proceeding

---

## Validation Artifacts Checklist

For audit trail, ensure these are captured:

- [ ] Terminal output from test run
- [ ] Terminal output from build
- [ ] Git diff of changes
- [ ] Validation report (above template)

---

## Common Validation Pitfalls

### "Tests Pass Locally"
- CI environment may differ
- Always verify in clean state

### "Just a Small Change"
- Small changes can violate invariants
- Always run full validation

### "I'll Fix That Later"
- No. Fix it now or revert.
- Technical debt is rejected at checklist

### "The Lint Is Wrong"
- Lints encode team decisions
- Discuss before disabling

---

## Integration with CI

This checklist maps to CI gates:

| Checklist Item | CI Job |
|----------------|--------|
| Rust builds | `rust-gate` |
| Frontend builds | `frontend-gate` |
| Tests pass | `rust-gate`, `frontend-gate` |
| Lints pass | `rust-gate`, `frontend-gate` |
| Invariants | `invariant-check` |

**Green CI = Checklist passed**

---

*Validation is not optional. Skip it and you're not done.*
