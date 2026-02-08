# /synthesize

Generate an executive briefing from 4DA's recent discoveries.

## Usage

```
/synthesize                    # Last 7 days, all sources
/synthesize --period=30d       # Last 30 days
/synthesize --topic=rust       # Focus on specific topic
/synthesize --format=brief     # Short summary only
```

## What This Does

This command invokes the **4da-insight-synthesizer** agent to transform raw items into actionable intelligence:

1. **Analyzes recent high-scoring items** from all sources
2. **Identifies key themes** and groups related content
3. **Extracts actionable insights** relevant to your work
4. **Generates recommendations** for what to pay attention to
5. **Highlights trends** and emerging topics

## Example Output

```
## 4DA Executive Briefing

**Period:** Jan 15-22, 2026 | **Items Analyzed:** 247

### TL;DR
Three key themes this week: (1) Rust ecosystem maturing with Tauri 2.0,
(2) AI coding assistants entering mainstream, (3) growing supply chain
security concerns.

**Your top action:** Evaluate Tauri 2.0 migration - 3 relevant articles suggest benefits.

### Key Themes

#### 1. Rust Ecosystem (12 items, avg score: 0.82)
- Tauri 2.0 released with mobile support
- tokio performance improvements
- **Action:** Review changelog before upgrading

#### 2. AI Developer Tools (8 items, avg score: 0.76)
- Claude Code adoption accelerating
- New embedding models available
- **Action:** Test text-embedding-3-large

### Notable Items
| Title | Source | Score | Action |
|-------|--------|-------|--------|
| "Tauri 2.0: What's New" | HN | 0.92 | Read & evaluate |
| "Rust async best practices" | HN | 0.88 | Study patterns |

### Recommended Actions
1. **High:** Review Tauri 2.0 migration guide
2. **Medium:** Test new embedding model
3. **Low:** Explore sqlite-vec alternatives
```

## Agent Reference

Full agent definition: `.claude/agents/4da-insight-synthesizer.md`
