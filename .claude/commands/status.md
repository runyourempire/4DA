---
description: "Show active terminal status — what's running, file claims, and potential conflicts."
allowed-tools: ["Read", "Bash", "Grep"]
---

# /status — Terminal & Session Status

Show the current state of all active Claude Code terminals and session health.

## Steps

1. **Read `.claude/TERMINALS.md`** — parse all `### T[N]` entries
2. For each terminal entry, extract:
   - Terminal number and description
   - Status (working / committing / done)
   - Claimed files list
3. **Conflict detection:** Check if any two terminals claim the same file
4. **Git status:** Run `git status --short` to see uncommitted changes
5. **Cross-reference:** For each uncommitted file, check which terminal (if any) claims it. Flag unclaimed modified files as potential orphans.

## Display Format

```
TERMINAL STATUS
━━━━━━━━━━━━━━━━━━━━━━━━

T1 — [description]        [STATUS]
  Files: file1.rs, file2.ts

T2 — [description]        [STATUS]
  Files: file3.rs

CONFLICTS: none | list overlapping files
UNCLAIMED CHANGES: none | list files modified but not claimed

GIT: [branch] | [N files changed] | [clean/dirty]
```

## Edge Cases

- **No terminals active:** Show "No active terminals. All clear."
- **TERMINALS.md missing:** Show "Terminal coordination file not found. Create .claude/TERMINALS.md to enable multi-terminal tracking."
- **Done entries:** Show but mark as ready to clean up
