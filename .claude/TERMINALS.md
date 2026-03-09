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
- **Status**: done
- **Commit**: `7b11cd1` Production safety: replace silent error swallowing with tracing::warn
- **Delivered**: 43 backend .rs files — 58 filter_map fixes, 47 let\_=fixes, image.rs Result fix
- **Available for**: new tasks — all backend .rs files

### T2 — Git hygiene, embedding fix, file size compliance
- **Status**: done (all pushed to origin)
- **Commits**: `aea18d6` DeveloperDnaSection extract + exceptions, `d8c6de1` hardening plan, `b5b50f4` embedding fix + cargo fmt
- **Files touched**: `src-tauri/src/ace/mod.rs`, `src-tauri/src/channel_commands.rs`, `src-tauri/src/db/sources.rs`, `src-tauri/src/llm.rs`, `src-tauri/src/scoring/context.rs`, `src-tauri/src/semantic_diff.rs`, `src/components/SovereignDeveloperProfile.tsx`, `src/components/DeveloperDnaSection.tsx`, `scripts/check-file-sizes.cjs`, `.ai/HARDENING_PLAN.md`
- **Note**: T1/T5 — rebase needed if you touch any of these files

### T3 — Hardening phases 2-5+7 (i18n, a11y audit, test coverage)
- **Status**: done
- **Commits**: `c04e74e` memo+i18n, next = i18n hardening + 4 new test files (32 new tests, 881 total)
- **Findings**: Phase 2 (a11y) 90% already done. Phase 4 (perf) already well-memoized. ContextPanel.tsx doesn't exist. KeyboardShortcutsModal has full dialog a11y.
- **Remaining for someone**: Phase 6 (Rust dead code), broken Briefing splits from old agents (BriefingContentPanel/BriefingMetrics/BriefingTopPicks have TS errors)
- **Files touched**:
  - `src/components/channels/ChannelCard.tsx` (i18n formatTimeAgo)
  - `src/components/DecisionMemory.tsx` (i18n TYPE_LABELS + date metadata)
  - `src/components/DecisionMemory.test.tsx` (fix i18n key expectations)
  - `src/components/channels/__tests__/ProvenanceTooltip.test.tsx` (new — 7 tests)
  - `src/components/__tests__/IntelligenceProfileCard.test.tsx` (new — 8 tests)
  - `src/components/__tests__/CalibrationView.test.tsx` (new — 9 tests)
  - `src/components/tech-radar/__tests__/RadarSVG.test.tsx` (new — 7 tests)
  - `src/locales/en/ui.json` (11 new keys)

### T5 — Phase 1.1 invoke migration + Phase 2.1 Rust splits + Phase 1.3 frontend decomposition
- **Status**: working
- **Previous commits**: `27e7d91`, `41ae3c5`, `c4cd649`
- **Task**: Migrate 237 raw invoke() → typed commands, split 5 oversized Rust files, decompose BriefingView/SettingsModal/App.tsx
- **SKIPPING T3's files**: DecisionMemory.tsx, ChannelCard.tsx, ToolkitView.tsx, ContextPanel.tsx, SovereignDeveloperProfile.tsx, KeyboardShortcutsModal.tsx, ui.json
- **Files**:
  - `src/lib/commands.ts` (add ~60 missing commands to CommandMap)
  - `src/store/*.ts` (all store slices — invoke → cmd migration)
  - `src/hooks/*.ts` (all hooks EXCEPT T3's — invoke → cmd migration)
  - `src/components/*.tsx` (invoke migration — EXCLUDING T3's claimed files)
  - `src/components/onboarding/*.tsx` (invoke migration)
  - `src/components/settings/*.tsx` (invoke migration)
  - `src/components/playbook/*.tsx` (invoke migration)
  - `src/components/channels/*.tsx` (invoke migration — EXCLUDING ChannelCard.tsx)
  - `src/components/tech-radar/*.tsx` (invoke migration)
  - `src/components/toolkit/tools/*.tsx` (invoke migration)
  - `src/components/search/*.tsx` (invoke migration)
  - `src/components/result-item/*.tsx` (invoke migration)
  - `src/components/BriefingView.tsx` (decomposition)
  - `src/components/SettingsModal.tsx` (decomposition)
  - `src/App.tsx` (decomposition)
  - `src-tauri/src/analysis.rs` (split)
  - `src-tauri/src/settings_commands.rs` (split)
  - `src-tauri/src/sovereign_developer_profile.rs` (split)
  - `src-tauri/src/scoring/pipeline.rs` (split)
  - `src-tauri/src/stacks/profiles.rs` (split)
