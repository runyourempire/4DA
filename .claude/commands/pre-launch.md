---
description: "Run the full 4DA pre-launch readiness audit — scoring benchmark, MCP server, first-run experience, and build health"
allowed-tools: ["Read", "Bash", "Glob", "Grep", "Edit", "Write", "Task"]
argument-hint: "[--scoring | --mcp | --fre | --build | --fix]"
---

# /pre-launch

Run the full 4DA pre-launch readiness audit — scoring, MCP server, first-run experience, and build health.

## Arguments

- `$ARGUMENTS` — optional: `--scoring` (scoring only), `--mcp` (MCP server only), `--fre` (first-run experience only), `--build` (build checks only), `--fix` (auto-fix issues found)

## Instructions

You are the 4DA pre-launch readiness system. Your job is to systematically validate that every user-facing system meets launch quality standards. Execute the sections below in order, running parallel subagents where marked.

**Operating principle:** Zero tolerance for regressions. Every check must pass or produce a documented exception with justification.

---

### Step 0: Parse Arguments

If `$ARGUMENTS` contains a section flag (`--scoring`, `--mcp`, `--fre`, `--build`), only run that section. Otherwise run all sections.

If `--fix` is passed, auto-apply fixes for any issues found (with user confirmation for destructive changes).

---

### Step 1: PASIFA Scoring Benchmark (unless skipped)

Run the scoring benchmark baseline tests:

```bash
cd src-tauri && cargo test --lib benchmark -- --nocapture 2>&1
```

**Pass criteria:**
- All benchmark tests pass (currently 22 tests)
- Precision >= 95% for all user profiles
- Rejection rate >= 90% for off-domain content
- Gate invariants hold (0 signals -> cap 0.20, 1 signal -> cap 0.32, 2+ signals -> uncapped)
- Determinism verified (same input -> same output)

**On failure:**
1. Identify which tests failed
2. Read `src-tauri/src/scoring/benchmark.rs` to understand expected behavior
3. Read `src-tauri/scoring/pipeline.scoring` for current thresholds
4. Report: test name, expected vs actual, likely cause
5. If `--fix` passed, attempt to fix the regression

**Output format:**
```
## Scoring Benchmark
Status: PASS | FAIL
Tests: 22/22 passed
Profiles: rust_dev (P:100% R:100%), python_ml (P:100% R:100%)
Gate invariants: HOLD
Determinism: VERIFIED
```

---

### Step 2: MCP Server Publish Readiness (unless skipped)

Run these checks in parallel:

**2a. TypeScript compilation:**
```bash
cd mcp-4da-server && npx tsc --noEmit 2>&1
```

**2b. Doctor command:**
```bash
cd mcp-4da-server && node dist/index.js --doctor 2>&1
```

**2c. Tool count verification:**
Read `mcp-4da-server/src/schema-registry.ts` and count registered tools. Current expected: 27.

**2d. Package.json audit:**
Read `mcp-4da-server/package.json` and verify:
- `name` is scoped (`@4da/mcp-server`)
- `version` follows semver
- `bin` entries point to existing files in `dist/`
- `files` array includes `dist`, `README.md`, `LICENSE`
- `engines` field specifies Node >= 18
- `prepublishOnly` runs build

**2e. Cross-platform paths:**
Search `mcp-4da-server/src/` for hardcoded Unix paths (`/home/`, `/Users/`, `C:\\`) that would break cross-platform.

**2f. Native binding resilience:**
Verify `mcp-4da-server/src/db.ts` has:
- Dynamic import for `better-sqlite3` with try/catch
- Platform-specific build tool instructions in error message
- Type-only import for `BetterSqlite3` namespace

**Pass criteria:**
- Zero TypeScript errors
- Doctor reports all checks pass or only optional warnings
- All 27 tools registered
- Package.json has all required fields
- No hardcoded platform-specific paths
- Native binding failure produces actionable error

**Output format:**
```
## MCP Server
Status: PASS | FAIL
TypeScript: clean (0 errors)
Tools: 27/27 registered
Doctor: all pass (N warnings)
Package: valid for npm publish
Cross-platform: no hardcoded paths
Native bindings: graceful degradation verified
```

---

### Step 3: First-Run Experience Audit (unless skipped)

Spawn an explorer subagent to audit the onboarding flow:

**3a. Read these files:**
- `src/components/Onboarding.tsx`
- `src/components/onboarding/*.tsx` (all step components)
- `src/components/FirstRunTransition.tsx`
- `src/components/SplashScreen.tsx`
- `src/components/BriefingEmptyStates.tsx`

**3b. Check for these issues:**

| Issue | Severity | How to Check |
|-------|----------|-------------|
| Dead buttons (onClick does nothing) | P0 | Search for `onClick` handlers that are empty or undefined |
| Collapsed sections hiding content | P0 | Check if sections default to `collapsed: true` without user trigger |
| Missing loading states | P1 | Check async operations have loading indicators |
| Zero-results dead ends | P1 | Verify empty state components exist for all data views |
| Misleading time estimates | P1 | Search for hardcoded time strings ("takes 2 minutes") |
| Missing error boundaries | P2 | Check ErrorBoundary wraps all route-level components |
| Accessibility gaps | P2 | Check for missing aria-labels on interactive elements |

**3c. If `--fix` and issues found:**
- P0 issues: Fix immediately
- P1 issues: Fix if straightforward (<10 lines each)
- P2 issues: Report only (user decides)

**Output format:**
```
## First-Run Experience
Status: PASS | FAIL
P0 issues: 0 (was 3, fixed in c14c52e)
P1 issues: 0
P2 issues: 2 (documented below)
Files checked: 8
```

---

### Step 4: Build Health (unless skipped)

Run these in parallel:

**4a. Rust build:**
```bash
cd src-tauri && cargo check 2>&1
```

**4b. Frontend build:**
```bash
pnpm run build 2>&1
```

**4c. File size validation:**
```bash
pnpm run validate:sizes 2>&1
```

**4d. Rust tests:**
```bash
cd src-tauri && cargo test --lib 2>&1
```

**4e. Frontend tests:**
```bash
pnpm run test -- --run 2>&1
```

**Pass criteria:**
- Zero compilation errors (Rust + TypeScript)
- All file sizes within limits (TS: 500 lines, Rust: 1000 lines)
- All tests pass
- No clippy warnings with `--deny warnings`

**Output format:**
```
## Build Health
Status: PASS | FAIL
Rust: compiles clean
Frontend: builds clean
File sizes: all within limits
Rust tests: N/N passed
Frontend tests: N/N passed
```

---

### Step 5: Summary Report

Combine all section results into a final report:

```
# 4DA Pre-Launch Readiness Report
Date: [current date]
Commit: [git rev-parse --short HEAD]

| Section | Status | Details |
|---------|--------|---------|
| Scoring Benchmark | PASS/FAIL | 22/22 tests, 100% precision |
| MCP Server | PASS/FAIL | 27 tools, clean build |
| First-Run Experience | PASS/FAIL | 0 P0, 0 P1 issues |
| Build Health | PASS/FAIL | All tests pass |

## Overall: READY / NOT READY

[If NOT READY, list blocking issues with fix instructions]

## Next Steps
[3-5 genuinely high-impact actions based on current state — not generic advice]
```

---

### Edge Cases

- **Dev server running (exe locked):** Use `cargo test --lib` and `cargo check --lib` to avoid rebuilding the binary
- **Database missing:** Skip scoring benchmark DB-dependent tests, note in report
- **MCP server not built:** Run `cd mcp-4da-server && pnpm run build` first
- **Ollama not running:** Note as optional warning, not a blocker

### What NOT to Do

- Do NOT modify scoring thresholds to make tests pass
- Do NOT skip failing tests — report them
- Do NOT run `pnpm run tauri build` (takes too long for a readiness check)
- Do NOT push to npm — this is validation only
- Do NOT commit changes unless `--fix` was explicitly passed
