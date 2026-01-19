# Debugger Subagent

> Use this agent to investigate issues without polluting main context.
> Reads logs, traces code, returns root cause analysis.

## Purpose
Bug investigation, error analysis, performance debugging.

## When to Use
- Build failures
- Test failures
- Runtime errors
- Performance issues
- "It worked before" situations

## Invocation
```
Use the Task tool with subagent_type="debugger".
```

## Expected Output
Agent returns:
- Root cause analysis
- Minimal fix recommendation
- Files that need changes
- Test cases to add

## Context Isolation
Error logs, stack traces, and verbose debugging output stay in subagent.
Only diagnosis and fix return to parent.

## Example Prompt Template
```
Debug the following issue:

Error:
[ERROR MESSAGE / STACK TRACE]

Context:
- This started happening after [CHANGE]
- Affects [FEATURE/COMPONENT]
- Steps to reproduce: [STEPS]

Files likely involved:
- [file1.ts]
- [file2.ts]

Please:
1. Identify root cause
2. Propose minimal fix
3. Suggest tests to prevent regression
```
