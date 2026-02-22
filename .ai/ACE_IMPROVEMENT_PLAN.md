# ACE Improvement Plan

> **STATUS: COMPLETE** — All items implemented as of 2026-02-08.
> This file is retained for historical context only.

## What Was Done

PASIFA (Privacy Aware Semantic Intelligence 4 File Analysis) is fully operational:

1. **README indexing** from discovered projects — feeds into KNN semantic search
2. **Semantic ACE boost** via `compute_semantic_ace_boost()` in `scoring.rs` — embeds ACE topics and computes cosine similarity (falls back to keyword matching if embeddings unavailable)
3. **Unified scoring pipeline** — `score_item()` combines context similarity, interest matching, ACE boost, affinity multipliers, anti-topic penalties, temporal freshness, and feedback signals in a single pass
4. **Incremental updates** — file watcher triggers re-indexing, embeddings updated in sqlite-vec
5. **Behavior learning** — affinities and anti-topics from user interactions feed back into scoring via `compute_affinity_multiplier()` and `compute_unified_relevance()`

## Current Scoring Formula (Actual)

```
context_score    = KNN similarity against user's context chunks
interest_score   = max cosine similarity against interest embeddings
semantic_boost   = cosine similarity of item topics vs ACE topic embeddings
base_score       = (context * 0.5 + interest * 0.5 + semantic_boost) * freshness
combined         = base_score * affinity_mult * (1.0 - anti_penalty) + feedback_boost
```

All semantic. No arbitrary keyword matching. ACE feeds into the embedding space, not alongside it.

## No Further Action Required

The "Two-System Problem" described in the original plan is solved. Discovery and scoring share one embedding space.
