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

### T1 — ASCENT Phase 6 + next backend work
- **Status**: committing
- **Done**: Feature-flagged game_engine + game_achievements behind `#[cfg(feature = "experimental")]`
- **Audit**: Zero production unwrap() calls — all 653 are in test code. No hardening needed.
- **Files**: `src-tauri/src/lib.rs`
