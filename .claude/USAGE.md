# Context Rot Defense System - Usage Guide

## Automatic Subagent System (NEW)

Every prompt is automatically analyzed before Claude sees it. Complex tasks trigger automatic recommendations.

**What happens:**
1. You type a prompt
2. `analyze-prompt.cjs` scores complexity
3. If score >= 4, recommendation is injected
4. Claude sees the recommendation before responding

**Triggers:**
| Signal | Example | Score |
|--------|---------|-------|
| Exploration | "find all files", "search codebase" | +3 |
| Multi-file | "refactor", "implement feature" | +4 |
| Debugging | "fix bug", "error", "investigate" | +3 |
| Testing | "run tests", "write tests" | +3 |
| Review | "code review", "audit" | +3 |
| Multiple files mentioned | 3+ .ts/.rs files | +3-5 |
| Session fatigue | 20+ prompts in session | +1 |

**View analysis logs:**
```bash
cat .claude/sessions/analyzer.log
```

**Adjust threshold:**
Edit `.claude/scripts/analyze-prompt.cjs` line 16: `threshold: 4`

---

## Quick Reference

### Manual Compaction (Best Practice)
```
/compact Focus on preserving: current task state, architectural decisions, file modifications
```
Do this at task boundaries, not mid-task.

### Check Context Usage
```
/context
```

### When Context Gets Heavy
1. First try: `/compact [instructions]`
2. If quality degraded: Consider fresh session with handoff

---

## MCP Memory Tools

### Session History Tools

**List past sessions:**
```
Use list_sessions to see all archived session transcripts
```

**Search through past conversations:**
```
Use search_sessions with query: "context rot"
- Returns sessions where that topic was discussed
- Shows snippets of matching content
```

**Read exact conversation from a past session:**
```
Use get_session_messages with:
- session_file: "session_20260119_143052_abc123.jsonl"
- offset: 0 (start from beginning)
- limit: 50 (number of messages)
```

**Index new sessions:**
```
Use index_sessions to index any new unindexed session transcripts
```

### Store a Decision
```
Use remember_decision with:
- key: "unique-identifier"
- decision: "What was decided"
- rationale: "Why"
- alternatives: "What was rejected"
```

### Recall Decisions
```
Use recall_decisions with:
- key: "specific-key" (optional)
- search: "search term" (optional)
```

### Track State
```
Use update_state with:
- key: "current_task" | "blocked_on" | "last_modified"
- value: "description"
```

### Store Learnings
```
Use remember_learning with:
- topic: "tauri" | "react" | "sqlite"
- content: "What was learned"
- context: "When/how discovered"
```

### Find Code Locations
```
Use remember_code_location with:
- name: "indexer-main"
- file_path: "src-tauri/src/indexer/mod.rs"
- line_number: 45
- purpose: "Main indexing entry point"
```

---

## Subagent Usage

### For Exploration (keeps search output isolated)
```
I'll use a subagent to explore the codebase for [QUERY]
```

### For Implementation (fresh context for heavy work)
```
I'll spawn an implementer subagent to handle this multi-file change
```

### For Review (isolated analysis)
```
I'll use a reviewer subagent to check these changes
```

### For Debugging (keeps logs isolated)
```
I'll use a debugger subagent to investigate this error
```

---

## File-Based Memory (Auto-Loaded)

### decisions.md
- Updated when architectural choices are made
- Survives compaction (re-injected each turn)
- Format: Decision → Rationale → Date

### current-state.md
- Updated at task boundaries
- Tracks: active task, progress, files modified
- Prevents "what were we doing?" confusion

### conventions.md
- Code style reference
- Re-injected each turn
- Follow without needing to remember

---

## Workflow Best Practices

### Starting a Session
1. Check `current-state.md` for context
2. Run `get_state` to see MCP state
3. Run `recall_decisions` for key decisions

### During Development
1. Update `current-state.md` at task boundaries
2. Use `remember_decision` for significant choices
3. Use subagents for multi-file operations
4. `/compact` at ~60-70% context (before auto-trigger)

### Ending a Session
1. Update `current-state.md` with final state
2. Use `update_state` with key context
3. Ensure decisions.md is current

### If Quality Degrades
1. Check `/context` - how full?
2. Try `/compact [preserve instructions]`
3. If still degraded: fresh session with handoff

---

## Environment Variables

```bash
# In your shell profile
export CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=60  # Compact at 60% instead of 95%
export MEMORY_DB_PATH="/mnt/d/4DA/.claude/memory.db"
```

---

## Hooks (Automatic)

### PreCompact Hook
- Runs before any compaction (auto or manual)
- Backs up transcript to `.claude/backups/`
- Updates `current-state.md` timestamp

---

## Session Archiving

Sessions are automatically archived when you exit Claude Code (via the Stop hook).

**Archive location:** `.claude/sessions/transcripts/`

**What's saved:**
- Complete conversation history (user + assistant messages)
- Indexed for full-text search
- Metadata (date, message count)

**To manually save current session:**
```bash
./.claude/scripts/save-transcript-now.sh "descriptive-note"
```

**Session search examples:**
- "What did we discuss about authentication?"
  → `search_sessions` with query "authentication"
- "Show me that conversation from last week"
  → `list_sessions` then `get_session_messages`
- "Find when we made that database decision"
  → `search_sessions` with query "database decision"

---

## Troubleshooting

### "Going in circles" / Repeated suggestions
- Context rot symptom
- Solution: `/compact` with clear preservation instructions

### Terminal lag / unresponsive
- Likely rendering bug, not context rot
- Solution: `npm install -g claudescreenfix-hardwicksoftware`

### Lost decisions after compaction
- Decisions not in `decisions.md` or MCP
- Solution: Always `remember_decision` for important choices

### MCP tools not available
- Server not running
- Solution: Check `.mcp.json`, rebuild server
