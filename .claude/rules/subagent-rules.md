# Subagent Rules

Spawn a subagent when ANY of these are true:

1. **3+ files** to modify → implementer subagent
2. **Searching** for unknown code location → explorer subagent
3. **Debugging** with logs/traces or build failures → debugger subagent
4. **Running tests** with potential failures → test subagent
5. **Reviewing** changes across multiple files → reviewer subagent

**Don't use subagent for:** single file edits, quick lookups, tasks already understood from current context.
