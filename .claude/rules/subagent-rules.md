# Subagent Spawning Rules

> These rules are auto-loaded every turn. Follow them without needing to remember.
> Purpose: Isolate heavy work in fresh context before quality degrades.

---

## MANDATORY Subagent Triggers

Spawn a subagent when ANY of these conditions are true:

### 1. Multi-File Operations
- Task requires modifying **3+ files** → Spawn implementer subagent
- Reason: Heavy file reads pollute context

### 2. Search/Exploration
- Need to search codebase for unknown location → Spawn explorer subagent
- Need to understand unfamiliar code area → Spawn explorer subagent
- Reason: Grep/glob output is high-volume, low-signal

### 3. Debugging with Logs
- Error investigation requires reading logs/traces → Spawn debugger subagent
- Build failure with long output → Spawn debugger subagent
- Reason: Log content is extremely context-polluting

### 4. Test Runs
- Running test suite with potential failures → Spawn subagent
- Test output can be massive, keep it isolated

### 5. Code Review
- Reviewing changes across multiple files → Spawn reviewer subagent
- Reason: Keeps review analysis separate from implementation context

---

## HOW to Spawn

```
I'll use a [explorer/implementer/debugger/reviewer] subagent for this task.

[Use Task tool with detailed prompt including:]
- Exact task description
- Relevant file paths
- What output format is needed
- Any decisions/conventions to follow
```

---

## Subagent Prompt Template

When spawning, include:

```
Task: [SPECIFIC TASK]

Context:
- Project: 4DA v3 (Tauri + React + SQLite)
- Follow conventions in .claude/rules/conventions.md

Files likely involved:
- [file1]
- [file2]

Return:
- [What specific output is needed]
- Keep response concise (summary, not full exploration)
```

---

## EXCEPTIONS (Don't Use Subagent)

- Single file edit with known location
- Quick lookup of specific function/class
- Simple question that doesn't require exploration
- Task already well-understood from current context

---

## Self-Check Before Heavy Operations

Before ANY of these operations, ask: "Should this be in a subagent?"

- `Grep` with broad pattern
- `Glob` across large directory
- `Read` of file >500 lines
- `Bash` running tests or builds
- Any task description containing "find", "search", "explore", "investigate"

If yes → Spawn subagent first.

---

*This file is loaded every turn. These are standing orders, not suggestions.*
