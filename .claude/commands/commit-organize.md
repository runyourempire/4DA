# /commit-organize

Analyze uncommitted changes, intelligently group them into logical commits, and execute with user approval.

## Arguments

- `$ARGUMENTS` — optional flags: `--dry-run` (plan only, don't commit), `--skip-tests` (skip test preflight), `--auto` (commit without showing plan first — use with caution)

## Instructions

You are a commit organization system. Your job is to turn a messy working tree into clean, atomic, bisectable git history. Follow these steps precisely.

---

### Step 1: Preflight

Run these commands in parallel:

```bash
git status --porcelain
git diff --stat
git diff --cached --stat
git log --oneline -5
```

**Abort conditions** (stop and report):
- Working tree is clean → "Nothing to commit."
- Merge conflicts detected (`UU` in status) → "Resolve merge conflicts first."
- Only 1-2 files changed → Skip grouping, propose a single commit directly.

Unless `--skip-tests` was passed, run tests in parallel:

```bash
cd src-tauri && cargo test 2>&1 | grep "test result"
pnpm run test 2>&1 | tail -5
```

If tests fail → STOP. Report failures. Do not commit broken code.

---

### Step 2: Categorize Every Changed File

For each file in `git status`, assign ONE primary category based on what actually changed (read the diff if unclear):

| Category | Commit Priority | Signals |
|----------|----------------|---------|
| `structure` | 1 | New modules from file splits, deleted monoliths, re-export files |
| `infra` | 2 | Cargo.toml, package.json, tsconfig, vite.config, CI/CD, build scripts |
| `refactor` | 3 | Code moves without behavior change, dead code removal, import reorg |
| `feature` | 4 | New functionality, new components, new commands, new API endpoints |
| `fix` | 5 | Bug fixes, SQL corrections, error handling fixes, panic fixes |
| `test` | 6 | Test files — bundle with source unless test-only changes |
| `config` | 7 | Settings files, data files, environment defaults |
| `docs` | 8 | README, ARCHITECTURE.md, DECISIONS.md, comments |
| `legal` | 9 | LICENSE, CLA, NOTICE, copyright headers |
| `style` | 10 | Formatting-only changes (cargo fmt, prettier) |

**How to decide**: Read `git diff [file]` for ambiguous files. A Cargo.toml change that adds a dependency for a new feature goes with `feature`, not `infra`. A test file that was modified because a struct changed goes with `structure`, not `test`.

---

### Step 3: Build Commit Groups

Apply these rules to cluster files into commit groups:

**Cohesion rules** (files that MUST be together):
1. **Module split** — If file A was extracted FROM file B, and both show changes, same commit
2. **Import chain** — If file A's diff shows import changes pointing to file B (also changed), same commit
3. **Test + source** — Test file goes with the source it tests, not in a separate "tests" commit
4. **Config + code** — Cargo.toml/package.json dependency adds go with the code that uses them

**Sizing rules**:
5. **Minimum 2 files** — Merge groups with 1 file into the nearest related group (unless truly independent)
6. **Maximum ~25 files** — Split if a group exceeds this, unless it's one atomic operation (like a file split)

**Ordering rules**:
7. **Foundations first** — Structure/infra commits before features that depend on them
8. **Fixes before features** — Bug fixes in existing code before new code that might mask them
9. **Config/docs last** — These don't affect runtime behavior

**Merge rule**:
10. If two groups have fewer than 3 files each AND share a directory, merge them

---

### Step 4: Security Scan

Check all files being committed. EXCLUDE and WARN about any file matching:
- `.env`, `.env.*` (environment secrets)
- `*credential*`, `*secret*`, `*.key`, `*.pem`, `*.p12` (keys/certs)
- Files containing patterns: `API_KEY=`, `SECRET=`, `PASSWORD=`, `TOKEN=` with actual values

If found, list them and exclude from all commit groups.

---

### Step 5: Present the Plan

Display the full commit plan:

```
## Commit Organization Plan

**Changes analyzed:** N files | +X/-Y lines
**Proposed commits:** N
**Tests:** [passed/skipped]

---

### Commit 1/N: `structure` — [draft message]
| File | Status | Lines |
|------|--------|-------|
| src-tauri/src/lib.rs | Modified | -3200 |
| src-tauri/src/commands.rs | New | +1400 |
| ... | ... | ... |

> **Message:** Decompose lib.rs into focused modules
>
> lib.rs was 4000+ lines. Split into commands, embeddings, events,
> state, types, utils with re-exports preserving crate paths.

---

### Commit 2/N: ...
```

After displaying, ask: **"Proceed with this plan? (You can also say 'merge 1+2', 'split 3', 'reorder', or 'edit message N')"**

If `--dry-run` was passed, stop here.
If `--auto` was passed, skip asking and proceed.

---

### Step 6: Execute

For each commit group in order:

1. Run `cargo fmt` (if any `.rs` files are in the group)
2. `git add [specific files]` — NEVER use `git add -A` or `git add .`
3. Commit with the approved message. Always use HEREDOC format:
   ```bash
   git commit -m "$(cat <<'EOF'
   Message here.

   Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
   EOF
   )"
   ```
4. If pre-commit hook fails:
   - Read the error
   - Fix the issue (usually formatting)
   - Re-stage the fixed files
   - Create a NEW commit (never amend)
5. After each commit, run `git status --short` to verify

After all commits, display:

```
## Done

git log --oneline -[N new commits]
git status  (should be clean)
```

---

### Edge Cases

- **Staged + unstaged changes to same file**: Warn the user. Suggest `git stash` for unstaged or committing everything.
- **Binary files**: Include in commits but note them ("includes binary: icon.png")
- **Submodule changes**: Warn and ask before including
- **Very large diffs (>500 lines in one file)**: Note it, suggest the user review that file's diff before approving

---

### What NOT to do

- Do NOT run `git push` unless explicitly asked
- Do NOT amend existing commits
- Do NOT use `--no-verify` to skip hooks
- Do NOT create empty commits
- Do NOT guess — if you're unsure how to categorize a file, read its diff
