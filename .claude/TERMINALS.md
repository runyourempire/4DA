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

### T1 — Team Relay (all 6 phases)
- **Status**: working
- **Files**:
  - `src-tauri/Cargo.toml` (new crates)
  - `src-tauri/Cargo.lock` (lockfile)
  - `src-tauri/src/team_sync.rs` (NEW)
  - `src-tauri/src/team_sync_types.rs` (NEW)
  - `src-tauri/src/team_sync_crypto.rs` (NEW)
  - `src-tauri/src/team_sync_scheduler.rs` (NEW)
  - `src-tauri/src/team_sync_commands.rs` (NEW)
  - `src-tauri/src/team_sync_tests.rs` (NEW)
  - `src-tauri/src/db/migrations.rs` (new phases)
  - `src-tauri/src/lib.rs` (module declarations + command registration)
  - `relay/` (NEW — entire directory)

