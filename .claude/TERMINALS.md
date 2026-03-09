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

### T1 — Production safety hardening (Phase 2+3+4)
- **Status**: working
- **Task**: `let _ =` logging, `filter_map ok()` logging, image.rs Result fix
- **Skipping**: `state.rs` (T5), all frontend files (T3/T5)
- **Files**: All backend `.rs` files in src-tauri/src/ EXCEPT state.rs

### T2 — Git hygiene, embedding fix, file size compliance
- **Status**: done (all pushed to origin)
- **Commits**: `aea18d6` DeveloperDnaSection extract + exceptions, `d8c6de1` hardening plan, `b5b50f4` embedding fix + cargo fmt
- **Files touched**: `src-tauri/src/ace/mod.rs`, `src-tauri/src/channel_commands.rs`, `src-tauri/src/db/sources.rs`, `src-tauri/src/llm.rs`, `src-tauri/src/scoring/context.rs`, `src-tauri/src/semantic_diff.rs`, `src/components/SovereignDeveloperProfile.tsx`, `src/components/DeveloperDnaSection.tsx`, `scripts/check-file-sizes.cjs`, `.ai/HARDENING_PLAN.md`
- **Note**: T1/T5 — rebase needed if you touch any of these files

### T3 — Phase 4: Performance optimization + Phase 3: i18n cleanup
- **Status**: done (pushed as c04e74e)
- **Commit**: `c04e74e` memo(BadgeRow) + i18n cleanup (6 hardcoded strings → t(), 9 new keys, test fix)
- **Files touched**:
  - `src/components/IntelligenceProfileCard.tsx`
  - `src/components/result-item/BadgeRow.tsx`
  - `src/components/ResultItem.test.tsx`
  - `src/locales/en/ui.json`

### T5 — Audit hardening: IPC validator, E2E tests, error-path tests, lock ordering
- **Status**: working
- **Task**: Ghost command validator, E2E tests, error-path/a11y tests, state.rs lock ordering
- **WARNING for T1**: Old background agent (a719ff3e96e2f48fb) just completed and dropped Rust file splits into working tree: analysis.rs split (analysis_deep_scan.rs, analysis_status.rs, analysis_tests.rs), settings_commands split (settings_commands_llm.rs, settings_commands_license.rs, settings_commands_tests.rs), sovereign_developer_profile split. These overlap T1's files. T1 should decide: adopt these splits or revert them with `git checkout HEAD -- <files>` + `rm <new files>`.
- **Cleaned up earlier**: Reverted incomplete scoring/pipeline.rs and stacks/profiles.rs splits
- **Files**:
  - `scripts/validate-commands.cjs` (new)
  - `e2e/app-loads.spec.ts` (new)
  - `e2e/keyboard-navigation.spec.ts` (new)
  - `e2e/settings-roundtrip.spec.ts` (new)
  - `e2e/analysis-flow.spec.ts` (new)
  - `e2e/error-recovery.spec.ts` (new)
  - `package.json` (adding validate:commands script)
  - `src-tauri/src/state.rs` (lock ordering comment)
  - `src/components/ActionBar.test.tsx` (a11y + error-path tests)
  - `src/components/BriefingView.test.tsx` (error-path tests)
  - `src/components/ResultsView.test.tsx` (a11y + error-path tests)
  - `src/components/__tests__/smoke.test.tsx` (a11y tests)
  - `src/hooks/__tests__/use-analysis-errors.test.ts` (new — error-path tests)
  - `vitest.config.ts` (coverage thresholds: 40/25/35/40)
