# Explorer Subagent

> Use this agent to explore the codebase without polluting main context.
> Returns condensed summaries, keeping heavy search output isolated.

## Purpose
Codebase exploration, file discovery, pattern finding, architecture understanding.

## When to Use
- Finding files matching patterns
- Understanding code structure
- Searching for implementations
- Answering "where is X?" questions

## Invocation
```
Use the Task tool with subagent_type="Explore" for codebase exploration.
```

## Expected Output
Agent returns:
- Condensed summary (1000-2000 tokens max)
- Key file paths discovered
- Relevant code snippets (abbreviated)
- Answers to exploration questions

## Context Isolation
This agent runs in its own 200k context window. Heavy grep/glob output stays isolated from main conversation.

## Example Prompts
- "Find all files related to authentication"
- "What's the project structure?"
- "Where is error handling implemented?"
- "Find all React components that use useState"
