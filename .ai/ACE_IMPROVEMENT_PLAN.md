# ACE Improvement Plan: Achieving PASIFA

> **STATUS: IMPLEMENTED** (2026-02-03)
> Deep README indexing, section weighting, and recursive discovery are complete.
> See `MISSION_ACCOMPLISHED.md` for implementation details.

**P**rivate **A**utonomous **S**ystem for **I**ntelligent **F**ile **A**nalysis

## Current State Assessment

### What's Working
- [x] Auto-discovery finds directories without user input
- [x] Manifest scanning detects 77 technologies
- [x] KNN vector search is implemented AND used (lib.rs:1542)
- [x] Behavior learning tracks affinities and anti-topics
- [x] Context files are embedded semantically

### Critical Gap: The Two-System Problem

```
SYSTEM A (Context Files)          SYSTEM B (ACE Discovery)
------------------------          -----------------------
User adds files manually          Auto-discovers directories
Files embedded → KNN search       Manifests → keyword list
Semantic similarity ✅            Keyword matching only ⚠️
Limited scope                     System-wide scope
```

**The Problem**: ACE discovers rich context but only contributes keyword boost (max 0.3).
If no context files exist, relevance scoring is nearly blind.

---

## The Fix: Unify Discovery with Embedding

### Principle
Discovered directories should BE the context source, not just a keyword boost.

### Implementation

#### Step 1: Auto-index discovered directories

Currently:
- Discovery finds directories
- Manifests parsed for keywords
- NO file content indexed

After:
- Discovery finds directories
- Key files indexed with embeddings:
  - README.md → what the project does
  - Main source files → what you're building
  - Documentation → your domain knowledge

```rust
// In ace_full_scan or initialize_ace_on_startup
async fn index_discovered_context(paths: &[PathBuf]) {
    let db = get_database()?;

    for path in paths {
        // Index README if exists
        if let Some(readme) = find_readme(path) {
            let content = fs::read_to_string(&readme)?;
            index_context_file(&db, &readme, &content)?;
        }

        // Index key source files (limited to avoid overwhelming)
        let key_files = find_key_files(path, 5); // max 5 per project
        for file in key_files {
            let content = fs::read_to_string(&file)?;
            index_context_file(&db, &file, &content)?;
        }
    }
}
```

#### Step 2: Smart file selection

Not every file matters. Priority:
1. README.md, README.txt (project intent)
2. Main entry files (main.rs, index.ts, app.py)
3. Config files that reveal stack (tsconfig, Cargo.toml)
4. Recent files (modified in last 30 days)

#### Step 3: Incremental updates

File watcher already exists. When files change:
- Re-index that file
- Update embedding in sqlite-vec
- No full re-scan needed

---

## Scoring Formula (Current vs Proposed)

### Current
```
base_score = context_score * 0.5 + interest_score * 0.5
combined = base_score + ace_boost  // ace_boost is keyword-based, max 0.3
```

### Proposed (PASIFA)
```
// All semantic, no keyword matching
context_similarity = KNN_search(item_embedding, unified_context_embeddings)
combined = context_similarity  // Single source of truth
```

**Why simpler is better**:
- One embedding space
- One similarity computation
- No arbitrary weight tuning
- ACE discovery feeds INTO context, not alongside it

---

## Quick Win: Index README files from discovered projects

The simplest high-impact fix:

```rust
// After discovery finds directories, index READMEs
for discovered_dir in &discovered_dirs {
    let readme_path = discovered_dir.join("README.md");
    if readme_path.exists() {
        let content = fs::read_to_string(&readme_path)?;
        let chunks = chunk_text(&content, readme_path.to_str().unwrap());

        for (chunk_text, source_info) in chunks {
            let embedding = embed_texts(&[chunk_text.clone()])?;
            db.upsert_context_chunk(&source_info, &chunk_text, &embedding[0])?;
        }
    }
}
```

This immediately makes discovered projects contribute to semantic matching.

---

## PASIFA Checklist

| Principle | Current | Target | How |
|-----------|---------|--------|-----|
| **Private** | ✅ Local | ✅ Keep | No change needed |
| **Accurate** | ⚠️ Keywords | Semantic | Embed ACE context |
| **System** | ⚠️ Fragmented | Unified | One context source |
| **Inference** | ⚠️ Basic | Smart | Weight by recency, activity |
| **File** | ⚠️ Metadata only | Content | Read actual files |
| **Autonomy** | ✅ Auto-discover | ✅ Keep | No change needed |

---

## Priority Order

1. **Index README files** from discovered projects (HIGH IMPACT, LOW EFFORT)
2. **Embed ACE topics** for semantic matching (MEDIUM IMPACT, MEDIUM EFFORT)
3. **Index key source files** from projects (HIGH IMPACT, MEDIUM EFFORT)
4. **Wire Context Engine L2/L3** properly (MEDIUM IMPACT, MEDIUM EFFORT)
5. **Cross-file relationships** (HIGH IMPACT, HIGH EFFORT - future)

---

## Anti-Goals (Avoid Over-Engineering)

- DON'T build full AST parsing
- DON'T index every file (overwhelming)
- DON'T add more weight parameters
- DON'T create separate embedding spaces
- DON'T require user configuration

**The goal is SIMPLER, not more complex.**
