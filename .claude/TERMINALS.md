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

### T1 — Move 3 (Professional Card) + Move 4 (Chain Prediction)
- **Status**: working
- **Files**:
  - `src-tauri/Cargo.toml` (add resvg/tiny-skia for PNG)
  - `src-tauri/src/developer_dna.rs` (professional SVG + PNG export)
  - `src-tauri/src/signal_chains.rs` (chain lifecycle prediction)
  - `src-tauri/src/lib.rs` (command registration)
  - `src/components/DeveloperDna.tsx` (PNG download button)
  - `src/components/SignalChains.tsx` (prediction UI)
  - `src/lib/commands.ts` (new commands)


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

### T4 — Operational hardening + management systems
- **Status**: working
- **Files**:
  - `docs/strategy/OPUS-OPERATIONS-PLAYBOOK.md` (NEW)
  - `src-tauri/src/sources/adapter_tests.rs` (NEW)
  - `.ai/DECISIONS.md` (AD-023)

### T5 — Source adapter resilience tests
- **Status**: done
- **Files**:
  - `src-tauri/src/sources/adapter_resilience_tests.rs` (NEW)
  - `src-tauri/src/sources/mod.rs` (module declaration)

### T7 — Startup health self-check
- **Status**: working
- **Files**:
  - `src-tauri/src/startup_health.rs` (NEW)
  - `src-tauri/src/lib.rs` (mod declaration + call + command registration only)

