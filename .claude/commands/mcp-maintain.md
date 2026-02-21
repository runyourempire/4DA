---
description: "Audit and maintain the 4DA MCP server — detect schema/doc drift, validate type safety, check hardcoded paths, and verify npm publish readiness"
allowed-tools: ["Read", "Bash", "Glob", "Grep", "Edit", "Write", "Task"]
argument-hint: "[--audit | --fix | --publish-check | --sync-agents]"
---

# /mcp-maintain

Maintain the 4DA MCP server — audit consistency, detect drift, validate publish readiness, and fix issues.

## Arguments

- `$ARGUMENTS` — optional: `--audit` (full audit only), `--fix` (auto-fix issues), `--publish-check` (npm publish readiness), `--sync-agents` (update agent docs to match reality)

## Instructions

You are the 4DA MCP server maintenance system. Your job is to detect and fix drift between the actual codebase and its documentation, schemas, agents, and published artifacts. Run all sections unless a specific flag limits scope.

**Fix policy when `--fix` is passed:**
- **Additive** (missing schemas, missing exports, missing case handlers): apply silently
- **Destructive** (orphan file deletion, agent doc rewrites): show the proposed change and ask "Apply? (y/n)" before each
- **Never** (even with --fix): remove tools, modify tool behavior, change package version, npm publish

---

### Step 0: Establish Ground Truth

This step has two phases. Do NOT skip — every subsequent check depends on accurate ground truth.

**0a. Read source-of-truth files:**

Read all of these in parallel:

- `mcp-4da-server/src/schema-registry.ts` — canonical tool list (tool names + categories)
- `mcp-4da-server/src/index.ts` — tool dispatcher (case statements)
- `mcp-4da-server/src/tools/index.ts` — tool exports
- `mcp-4da-server/package.json` — version, deps, bin entries
- `mcp-4da-server/README.md` — public documentation

**0b. Run baseline commands in parallel:**

```bash
# Schema count
ls mcp-4da-server/src/schemas/*.json | wc -l
```

```bash
# Test results
cd mcp-4da-server && npx vitest run 2>&1 | tail -5
```

```bash
# Git state
git rev-parse --short HEAD
```

**Extract these values for use in all subsequent steps:**
- **TOOL_COUNT**: number of tools in TOOL_REGISTRY in schema-registry.ts
- **TOOL_NAMES**: sorted list of tool names from TOOL_REGISTRY keys
- **VERSION**: version field from package.json
- **SCHEMA_COUNT**: number of .json files counted above
- **TEST_PASS/TOTAL**: from vitest output
- **COMMIT**: short hash from git

---

### Step 1: Schema-Code Consistency

**For each tool in the registry:**
1. Verify a matching schema file exists in `src/schemas/{tool-name}.json`
2. Verify a matching tool implementation file exists in `src/tools/`
3. Verify the tool is handled in the `index.ts` dispatcher (case statement)
4. Verify the tool is exported from `src/tools/index.ts`

**Report any mismatches:**
```
SCHEMA DRIFT:
  Missing schema: tool_name (in registry but no .json file)
  Orphan schema: old_tool.json (file exists but not in registry)
  Missing handler: tool_name (in registry but no case in index.ts)
  Missing export: tool_name (not exported from tools/index.ts)
```

If `--fix` passed: create missing schemas from tool inputSchema, add missing case handlers and exports. For orphan file deletion, show the filename and ask before removing.

---

### Step 2: Type Safety Scan

Search for type quality issues:

```
Grep for: `as any`, `as unknown as`, `: any`, `// @ts-ignore`, `// @ts-expect-error`
```

**Acceptable:** `as unknown as ToolParams` in index.ts dispatcher (MCP protocol boundary — args arrive as `Record<string, unknown>` from MCP SDK)

**Unacceptable:** `any` in tool implementations, `@ts-ignore` anywhere

Count and report with file:line references.

---

### Step 3: Hardcoded Path Detection

Search all `.ts` files for paths that would break on other machines:

```
Patterns to flag:
  /mnt/d/          (WSL dev path)
  /home/           (specific Linux user)
  C:\\Users\\      (specific Windows user)
  D:\\4DA          (dev machine)
  /Users/          (specific macOS user)
```

**Acceptable:** Platform-detection paths in db.ts and config-validator.ts (e.g., checking `C:\\` as a volume root for validation is correct behavior, not a hardcoded dev path).

If `--fix` passed: replace with portable alternatives (env vars, `__dirname`-relative, `process.cwd()`-relative).

---

### Step 4: Agent Documentation Drift

Read these agent files and compare against ground truth:

```
.claude/agents/4da-mcp-server-dev.md
.claude/agents/4da-mcp-tester.md
.claude/agents/4da-contract-validator.md
```

**For each agent file:** extract every factual claim (tool count, file lists, code patterns, version numbers, path references) and compare against TOOL_COUNT, TOOL_NAMES, and actual file structure from Step 0. Report any mismatch with the specific line and the correct value.

**Report format:**
```
AGENT DRIFT:
  {filename}:
    - Line {N}: Claims {wrong thing} → actually {correct thing}
    - Line {N}: References {old path} → should be {correct path}
```

If `--fix` or `--sync-agents` passed: show the proposed changes for each agent file and ask "Apply rewrites to {filename}? (y/n)" before modifying. Preserve the agent's purpose, constraints, and output format sections — only update factual content (file trees, tool lists, code patterns, paths).

---

### Step 5: README Accuracy

Compare README.md against ground truth:

- Tool count matches TOOL_COUNT
- Tool names in table match TOOL_NAMES
- Feature claims match implementation
- Setup instructions are current (--doctor, --setup, --http flags)
- Version number matches VERSION
- Transport options documented (stdio + HTTP)

---

### Step 6: Build & Package Health

Run 6a and 6b for all modes. Run 6c and 6d only if `--publish-check` or no flags.

**6a. TypeScript compilation (always):**
```bash
cd mcp-4da-server && npx tsc --noEmit
```

**6b. Tests (always):**
```bash
cd mcp-4da-server && npx vitest run
```

**6c. Doctor (publish-check or default):**
```bash
cd mcp-4da-server && node dist/index.js --doctor
```

**6d. Package contents (publish-check or default):**
```bash
cd mcp-4da-server && npm pack --dry-run 2>&1 | grep -E "total files|package size|__tests__"
```

**Verify:**
- Zero TypeScript errors
- All tests pass
- Doctor reports no failures (warnings acceptable)
- No `__tests__` directory in package
- Package size is reasonable (< 200 kB compressed)

---

### Step 7: Dependency Health

Check for:
- Outdated dependencies: `cd mcp-4da-server && npm outdated 2>&1`
- Security vulnerabilities: `cd mcp-4da-server && npm audit 2>&1`
- Unnecessary dependencies (imported but unused)

---

### Step 8: Maintenance Report

Generate a structured report using values from Step 0:

```
# MCP Server Maintenance Report
Date: [use current date from system]
Version: {VERSION}
Commit: {COMMIT}

## Health Summary
| Check | Status | Details |
|-------|--------|---------|
| Schema-Code Sync | PASS/FAIL | {TOOL_COUNT} tools, {SCHEMA_COUNT} schemas |
| Type Safety | PASS/WARN | {N} unsafe casts ({M} acceptable) |
| Hardcoded Paths | PASS/FAIL | {N} found |
| Agent Docs | PASS/DRIFT | {N} agents need update |
| README | PASS/DRIFT | {N} inaccuracies |
| TypeScript | PASS/FAIL | {N} errors |
| Tests | PASS/FAIL | {TEST_PASS}/{TEST_TOTAL} pass |
| Package | PASS/FAIL | {size} kB, {files} files |
| Dependencies | PASS/WARN | {N} outdated, {M} vulnerable |

## Issues Found
{numbered list of all issues, sorted by severity}

## Auto-Fixed (if --fix)
{numbered list of fixes applied}

## Requires Manual Action
{numbered list of issues that need human decision}

## Next Steps
{3-5 genuinely high-impact actions based on findings — not generic advice}
```

---

### Edge Cases

- **MCP server not built yet:** Run `cd mcp-4da-server && pnpm run build` before Step 6c/6d
- **No test database:** Doctor will report warning, not failure — acceptable
- **Agent files missing:** Note absence in report, don't create from scratch (use mcp-server-dev agent for that)
- **npm not logged in:** Skip publish-specific checks, note in report

### What NOT to Do

- Do NOT publish to npm — this is validation only
- Do NOT delete tools without explicit approval
- Do NOT modify tool behavior (only fix documentation/schemas/types)
- Do NOT add new tools — that's the mcp-server-dev agent's job
- Do NOT downgrade dependencies without approval
