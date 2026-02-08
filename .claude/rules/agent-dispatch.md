# 4DA Agent Dispatch Rules

> Slim version - full skill manifest available via MCP Resource: `4da://skills`

---

## Quick Dispatch

| User Intent | Agent |
|-------------|-------|
| "Why is this relevant?" | `relevance-debugger` |
| "What should I pay attention to?" | `insight-synthesizer` |
| "What's trending?" | `trend-analyzer` |
| "Is my config OK?" | `config-validator` |
| "How can I improve relevance?" | `context-optimizer` |
| "Source not working" | `source-debugger` |

---

## How to Invoke

For 4DA-specific tasks, fetch the skill manifest for full instructions:

```
1. Read resource: 4da://skills
2. Find matching skill by triggers
3. Load instructions from .claude/agents/{skill-name}.md
4. Spawn general-purpose agent with those instructions
```

---

## Self-Check

Before responding to 4DA-related questions:

1. Is this about **understanding scores**? → `relevance-debugger`
2. Is this about **source issues**? → `source-debugger`
3. Is this about **configuration**? → `config-validator`
4. Is this about **improving 4DA for the user**? → `context-optimizer`
5. Is this about **understanding what 4DA found**? → `insight-synthesizer` or `trend-analyzer`
6. Is this about **building/coding 4DA**? → dev agents (see skill manifest)

When in doubt, `insight-synthesizer` is a good general-purpose choice for "help me understand" questions.

---

## Full Skill Manifest

Available via MCP Resource: `4da://skills`

Contains:
- All skill names and triggers
- Instruction file paths
- Category (operational vs development)
- Quick reference lookup

*This file is loaded every turn. Fetch the resource for full details.*
