# Implementer Subagent

> Use this agent for focused implementation tasks.
> Gets fresh context, implements, returns summary of changes.

## Purpose
Focused code implementation without context pollution.

## When to Use
- Implementing a specific feature
- Modifying multiple files for one task
- Tasks requiring >3 file modifications
- When main context is getting heavy

## Invocation
```
Use the Task tool with a detailed implementation prompt.
Specify exactly what to implement and which files to modify.
```

## Expected Output
Agent returns:
- Summary of changes made
- List of files modified
- Any issues encountered
- Suggestions for testing

## Context Isolation
- Fresh 200k context window
- Heavy file reads stay in subagent
- Only summary returns to parent

## Best Practices
1. Provide complete context in the prompt (don't assume agent knows history)
2. Specify exact files to modify
3. Include acceptance criteria
4. Mention code conventions to follow

## Example Prompt Template
```
Implement [FEATURE] for the 4DA project.

Context:
- [Relevant architectural decisions]
- [Current state of related code]

Files to modify:
- [file1.ts] - [what to change]
- [file2.ts] - [what to change]

Acceptance criteria:
- [criterion 1]
- [criterion 2]

Follow conventions in .claude/rules/conventions.md
```
