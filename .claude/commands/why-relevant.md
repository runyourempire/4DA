# /why-relevant

Analyze why a specific item scored the way it did in 4DA's relevance system.

## Usage

```
/why-relevant [item_id]
/why-relevant hn_12345
/why-relevant          # analyzes most recent high-scoring item
```

## What This Does

This command invokes the **4da-relevance-debugger** agent to perform a complete score autopsy:

1. **Retrieves item details** from the database
2. **Breaks down score components:**
   - Embedding similarity (40%)
   - Keyword match (20%)
   - Source affinity (15%)
   - Recency boost (10%)
   - Feedback adjustment (15%)
3. **Explains in human terms** why each component scored the way it did
4. **Compares to similar items** to provide context
5. **Recommends actions** to improve relevance for similar content

## Example Output

```
## Score Autopsy: "Rust async patterns for beginners"

**Score:** 0.78 | **Source:** Hacker News | **Age:** 2 days

### Component Breakdown
| Component | Value | Contribution |
|-----------|-------|--------------|
| Embedding | 0.82 | 0.328 |
| Keywords | 0.65 | 0.130 |
| Source | 0.70 | 0.105 |
| Recency | 0.85 | 0.085 |
| Feedback | 0.55 | 0.082 |

### Why This Score?
This article scores well because it semantically matches your Rust/async
codebase (0.82 embedding similarity). It would score higher if the title
contained explicit interest terms like "tokio" or "tauri".

### To Improve Similar Scores
- Add "async patterns" as an explicit affinity
- Give positive feedback to boost source trust
```

## Agent Reference

Full agent definition: `.claude/agents/4da-relevance-debugger.md`
