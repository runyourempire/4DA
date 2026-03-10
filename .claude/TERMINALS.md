# Terminal Coordination

## Protocol
1. **Before editing**: Read this file. If your files are claimed by another terminal, STOP.
2. **Claim files**: Add your entry below with the files you'll touch.
3. **After committing**: Remove your entry.
4. **Conflicts**: If two terminals touch the same file, the one that committed first wins. The other must rebase.

## Active Terminals

<!-- Add entries below. Format:
### T[N] — [short description]
- **Status**: working | committing | done
- **Files**: list of files being modified
-->

### T1 — Phase 3.2 frontend error-path tests + console audit
- **Status**: working
- **Files**:
  - Frontend test files (new error-path tests)
  - `src/App.css` (potential text-muted color fix)
  - `.ai/HARDENING_PLAN.md` (status updates)


### T3 — Rust error-path tests, DB concurrency tests, plan status updates
- **Status**: committing
- **Files**:
  - `src-tauri/src/hardening_error_path_tests.rs` (new — 35 tests)
  - `src-tauri/src/db/stress_tests.rs` (new — 5 tests)
  - `src-tauri/src/db/concurrency_tests.rs` (new — 6 tests)
  - `src-tauri/src/lib.rs` (module declaration ordering)
  - `src-tauri/src/db/mod.rs` (module declarations)
  - `scripts/check-file-sizes.cjs` (added 6 exceptions)
  - `ASCENT-PLAN.md` (status updates)
