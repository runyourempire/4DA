# Reviewer Subagent

> Use this agent to review code changes without context pollution.
> Reads files, analyzes, returns focused feedback.

## Purpose
Code review, architecture review, security review.

## When to Use
- After implementing a feature
- Before committing changes
- When refactoring
- Security audits

## Invocation
```
Use the Task tool with subagent_type="code-reviewer" or "architect-review".
```

## Expected Output
Agent returns:
- Issues found (categorized by severity)
- Suggestions for improvement
- Security concerns
- Architecture alignment notes

## Context Isolation
File contents read by reviewer stay in subagent context.
Only review findings return to parent.

## Example Prompt Template
```
Review the following changes for [PURPOSE]:

Files changed:
- [file1.ts]
- [file2.ts]

Focus areas:
- [ ] Code quality
- [ ] Security
- [ ] Architecture alignment
- [ ] Performance
- [ ] Error handling

Context:
- This implements [FEATURE]
- Should follow patterns in [REFERENCE_FILE]
```
