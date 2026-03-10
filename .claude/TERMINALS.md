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

### T1 — System wiring: updater UI, proactive notifications, vuln correlation
- **Status**: working
- **Files**:
  - `src/components/settings/UpdateSection.tsx` (NEW — updater UI)
  - `src-tauri/src/signal_chains.rs` (proactive notification wiring)
  - `src-tauri/src/monitoring_jobs.rs` (chain prediction notifications)
  - `src-tauri/src/monitoring_notifications.rs` (chain notification dispatch)
  - `src-tauri/src/project_health_dimensions.rs` (vuln correlation enhancement)
  - `src-tauri/src/decision_advantage/windows.rs` (dep-aware security windows)
  - `src/components/settings/SettingsContent.tsx` or similar (add UpdateSection)
  - `src/locales/en/ui.json` (i18n keys)

