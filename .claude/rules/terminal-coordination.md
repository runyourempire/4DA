# Terminal Coordination Rule

Multiple Claude Code terminals run simultaneously on this repo.

## Before ANY file edits:
1. Read `.claude/TERMINALS.md`
2. Check no other terminal claims the files you need
3. Add your claim with the files you'll modify
4. Only then start editing

## After committing:
1. Remove your claim from `.claude/TERMINALS.md`
2. If working tree has changes from OTHER terminals, do NOT revert them — they belong to another session

## If you see uncommitted changes you didn't make:
- Another terminal is actively working. Do NOT `git checkout`, `git stash`, or revert those files.
- Only touch files you've claimed in TERMINALS.md.

## Forbidden without explicit user request:
- `git checkout HEAD -- <file>` on files you didn't modify
- `git stash` (affects all terminals)
- `git reset` (affects all terminals)
- Deleting or moving files not in your claim
- Creating files not related to your task (especially `llm_judge.rs`)
