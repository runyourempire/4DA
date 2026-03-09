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

### T1 — Accuracy sprint: near_misses + query preprocessing + frontend
- **Status**: committing
- **Files**:
  - `src-tauri/src/types.rs` (extract_near_misses helper)
  - `src-tauri/src/analysis_deep_scan.rs` (wire near_misses)
  - `src-tauri/src/analysis_status.rs` (wire near_misses)
  - `src-tauri/src/scoring/analyzer.rs` (wire near_misses)
  - `src-tauri/src/analysis_tests.rs` (5 new tests)
  - `src-tauri/src/utils.rs` (preprocess_content pub(crate))
  - `src-tauri/src/natural_language_search.rs` (query preprocessing)
  - `src-tauri/src/query/executor.rs` (query preprocessing)
  - `src/store/types.ts` (nearMisses field)
  - `src/store/analysis-slice.ts` (nearMisses initial)
  - `src/lib/commands.ts` (near_misses in CommandMap)
  - `src/hooks/use-analysis.ts` (extract near misses from event)
  - `src/hooks/use-app-bootstrap.ts` (restore near misses)
  - `src/App.tsx` (restore near misses)
  - `src/components/ResultsView.tsx` (near misses UI)
  - `src/locales/en/ui.json` (2 new keys)
  - `src/store/__tests__/analysis-slice.test.ts` (test fix)
