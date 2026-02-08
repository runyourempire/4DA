# 4DA Knowledge Mapper Agent

> Build and explore knowledge graphs from discovered content

---

## Purpose

The Knowledge Mapper transforms isolated content items into connected knowledge structures. It builds graphs of relationships between topics, sources, and concepts - revealing hidden connections and enabling exploration of your information landscape.

**Superpowers:**
- Entity extraction from content
- Relationship discovery between topics
- Graph construction and traversal
- Concept clustering and hierarchy
- Knowledge gap identification

---

## When to Use

- "How are these topics connected?"
- "Show me the knowledge graph for [area]"
- "What concepts link [A] and [B]?"
- "Map my knowledge landscape"
- "Find hidden connections in my feed"

---

## Core Knowledge

### Graph Model

```
Nodes:
├── Topics (extracted keywords, technologies)
├── Sources (HN, arXiv, Reddit)
├── Items (articles, papers, posts)
├── Concepts (higher-level abstractions)
└── Entities (people, companies, projects)

Edges:
├── MENTIONS (Item → Topic)
├── FROM_SOURCE (Item → Source)
├── RELATED_TO (Topic → Topic)
├── INSTANCE_OF (Topic → Concept)
├── AUTHORED_BY (Item → Entity)
└── SIMILAR_TO (Item → Item)
```

### Relationship Types

| Relationship | Description | Weight Factors |
|--------------|-------------|----------------|
| Co-occurrence | Topics in same item | Frequency |
| Sequential | Topics in same source over time | Recency |
| Semantic | Embedding similarity | Cosine distance |
| Explicit | User-defined connection | User weight |
| Inferred | Pattern-based discovery | Confidence |

---

## Mapping Workflows

### Workflow 1: Topic Graph Construction

Build a graph of topic relationships:

```bash
#!/bin/bash
# Build topic co-occurrence graph

DB="/mnt/d/4da-v3/data/4da.db"

echo "=== Topic Graph Construction ==="

# Define core topics
TOPICS=("rust" "typescript" "python" "ai" "llm" "database" "sqlite" "security" "async" "wasm" "tauri" "react")

# Build adjacency list
echo ""
echo "### Topic Adjacency List"
echo "Format: topic -> [connected_topics:weight]"

for topic in "${TOPICS[@]}"; do
  connections=""
  for other in "${TOPICS[@]}"; do
    if [ "$topic" != "$other" ]; then
      weight=$(sqlite3 "$DB" "
        SELECT COUNT(*) FROM items
        WHERE (lower(title) LIKE '%$topic%' OR lower(content) LIKE '%$topic%')
        AND (lower(title) LIKE '%$other%' OR lower(content) LIKE '%$other%')
        AND created_at > datetime('now', '-30 days');
      ")
      if [ "$weight" -gt 0 ]; then
        connections="$connections $other:$weight"
      fi
    fi
  done
  if [ -n "$connections" ]; then
    echo "$topic ->$connections"
  fi
done

# Graph statistics
echo ""
echo "### Graph Statistics"
echo "Nodes: ${#TOPICS[@]}"
total_edges=$(sqlite3 "$DB" "
  WITH pairs AS (
    SELECT DISTINCT
      CASE WHEN t1 < t2 THEN t1 ELSE t2 END as a,
      CASE WHEN t1 < t2 THEN t2 ELSE t1 END as b
    FROM (
      SELECT 'rust' as t1, 'async' as t2
      UNION SELECT 'rust', 'tauri'
      -- etc
    )
  )
  SELECT COUNT(*) FROM pairs;
")
echo "Edges: (calculated from co-occurrence)"
```

### Workflow 2: Entity Extraction

Extract entities (people, companies, projects) from content:

```bash
#!/bin/bash
# Entity extraction from recent items

DB="/mnt/d/4da-v3/data/4da.db"

echo "=== Entity Extraction ==="

# Get recent titles
sqlite3 "$DB" "
SELECT title FROM items
WHERE created_at > datetime('now', '-7 days')
AND relevance_score > 0.6;
" > /tmp/titles.txt

# Extract capitalized words (potential entities)
echo ""
echo "### Potential Entities (Capitalized Terms)"
cat /tmp/titles.txt | \
  grep -oE '\b[A-Z][a-z]+([A-Z][a-z]+)*\b' | \
  sort | uniq -c | sort -rn | head -20

# Known project patterns
echo ""
echo "### Detected Projects"
cat /tmp/titles.txt | \
  grep -oiE '(rust|go|python|typescript|tauri|react|vue|svelte|tokio|axum|sqlite|postgres|redis|docker|kubernetes|aws|gcp|azure)[^a-z]*[0-9.]+' | \
  sort | uniq -c | sort -rn | head -10

# Company patterns
echo ""
echo "### Mentioned Companies/Organizations"
cat /tmp/titles.txt | \
  grep -oiE '(google|microsoft|amazon|meta|apple|anthropic|openai|github|gitlab|mozilla|cloudflare|vercel|netlify)' | \
  sort | uniq -c | sort -rn

# Author patterns (from HN/Reddit)
echo ""
echo "### Frequently Mentioned Authors"
sqlite3 "$DB" "
SELECT json_extract(metadata, '$.by') as author, COUNT(*) as mentions
FROM items
WHERE source_id = 'hackernews'
AND created_at > datetime('now', '-30 days')
AND json_extract(metadata, '$.by') IS NOT NULL
GROUP BY author
ORDER BY mentions DESC
LIMIT 10;
"
```

### Workflow 3: Concept Hierarchy

Build hierarchical concept structure:

```bash
#!/bin/bash
# Concept hierarchy construction

echo "=== Concept Hierarchy ==="

# Define concept taxonomy
cat << 'EOF'
Technology
├── Programming Languages
│   ├── Systems: rust, go, c, cpp
│   ├── Scripting: python, ruby, javascript
│   └── Functional: haskell, elixir, clojure
├── Frameworks
│   ├── Frontend: react, vue, svelte, angular
│   ├── Backend: express, fastify, actix, axum
│   └── Desktop: tauri, electron
├── Data
│   ├── Databases: sqlite, postgres, mysql, redis
│   ├── Processing: pandas, spark, dask
│   └── ML: pytorch, tensorflow, jax
└── Infrastructure
    ├── Cloud: aws, gcp, azure
    ├── Containers: docker, kubernetes
    └── Networking: nginx, caddy, traefik

AI/ML
├── LLMs
│   ├── Models: gpt, claude, llama, mistral
│   ├── Techniques: rag, embeddings, fine-tuning
│   └── Tools: langchain, llamaindex
├── Computer Vision
│   └── Models: diffusion, gan, vae
└── ML Ops
    └── Tools: mlflow, wandb, kubeflow
EOF

# Map items to concepts
echo ""
echo "### Item-to-Concept Mapping (sample)"

DB="/mnt/d/4da-v3/data/4da.db"
sqlite3 "$DB" "
SELECT
  title,
  CASE
    WHEN lower(title) LIKE '%rust%' OR lower(title) LIKE '%go %' THEN 'Systems Language'
    WHEN lower(title) LIKE '%react%' OR lower(title) LIKE '%vue%' THEN 'Frontend Framework'
    WHEN lower(title) LIKE '%llm%' OR lower(title) LIKE '%gpt%' THEN 'LLM'
    WHEN lower(title) LIKE '%database%' OR lower(title) LIKE '%sql%' THEN 'Database'
    ELSE 'Other'
  END as concept
FROM items
WHERE created_at > datetime('now', '-7 days')
AND relevance_score > 0.6
LIMIT 15;
"
```

### Workflow 4: Path Finding

Find connection paths between topics:

```bash
#!/bin/bash
# Find paths between topics

DB="/mnt/d/4da-v3/data/4da.db"
START="${1:-rust}"
END="${2:-ai}"

echo "=== Path Finding: $START → $END ==="

# Direct connection
echo ""
echo "### Direct Connection"
direct=$(sqlite3 "$DB" "
  SELECT COUNT(*) FROM items
  WHERE (lower(title) LIKE '%$START%' AND lower(title) LIKE '%$END%')
  AND created_at > datetime('now', '-30 days');
")
echo "Items containing both '$START' and '$END': $direct"

if [ "$direct" -gt 0 ]; then
  echo ""
  echo "### Direct Connection Items"
  sqlite3 "$DB" "
    SELECT title, relevance_score FROM items
    WHERE (lower(title) LIKE '%$START%' AND lower(title) LIKE '%$END%')
    AND created_at > datetime('now', '-30 days')
    LIMIT 5;
  "
fi

# Two-hop connections (through intermediate topic)
echo ""
echo "### Two-Hop Connections"
echo "Finding topics that connect '$START' to '$END'"

# Topics that co-occur with START
sqlite3 "$DB" "
  SELECT DISTINCT
    CASE
      WHEN lower(title) LIKE '%async%' THEN 'async'
      WHEN lower(title) LIKE '%tokio%' THEN 'tokio'
      WHEN lower(title) LIKE '%embedding%' THEN 'embedding'
      WHEN lower(title) LIKE '%database%' THEN 'database'
      WHEN lower(title) LIKE '%tauri%' THEN 'tauri'
      ELSE NULL
    END as intermediate
  FROM items
  WHERE lower(title) LIKE '%$START%'
  AND created_at > datetime('now', '-30 days')
  HAVING intermediate IS NOT NULL;
" > /tmp/start_topics.txt

# Check which also connect to END
echo "Path: $START → [intermediate] → $END"
while read intermediate; do
  if [ -n "$intermediate" ]; then
    count=$(sqlite3 "$DB" "
      SELECT COUNT(*) FROM items
      WHERE lower(title) LIKE '%$intermediate%' AND lower(title) LIKE '%$END%'
      AND created_at > datetime('now', '-30 days');
    ")
    if [ "$count" -gt 0 ]; then
      echo "  $START → $intermediate ($count items) → $END"
    fi
  fi
done < /tmp/start_topics.txt
```

### Workflow 5: Knowledge Gap Analysis

Identify gaps in the knowledge graph:

```bash
#!/bin/bash
# Knowledge gap analysis

DB="/mnt/d/4da-v3/data/4da.db"

echo "=== Knowledge Gap Analysis ==="

# High-affinity topics with low item coverage
echo ""
echo "### Underserved Interests"
echo "Topics you care about but few items cover"

sqlite3 "$DB" "
WITH affinity_topics AS (
  SELECT topic FROM affinities WHERE score > 0.6
),
topic_coverage AS (
  SELECT
    a.topic,
    (SELECT COUNT(*) FROM items
     WHERE (lower(title) LIKE '%' || a.topic || '%' OR lower(content) LIKE '%' || a.topic || '%')
     AND created_at > datetime('now', '-30 days')) as item_count
  FROM affinity_topics a
)
SELECT topic, item_count
FROM topic_coverage
WHERE item_count < 5
ORDER BY item_count ASC;
"

# Isolated topics (no connections)
echo ""
echo "### Isolated Topics"
echo "Topics that don't connect to your main interests"

# Topics in items but not connected to core topics
CORE_TOPICS="rust typescript ai database"
sqlite3 "$DB" "
SELECT DISTINCT
  CASE
    WHEN lower(title) LIKE '%kubernetes%' THEN 'kubernetes'
    WHEN lower(title) LIKE '%graphql%' THEN 'graphql'
    WHEN lower(title) LIKE '%blockchain%' THEN 'blockchain'
    WHEN lower(title) LIKE '%flutter%' THEN 'flutter'
    ELSE NULL
  END as topic,
  COUNT(*) as count
FROM items
WHERE created_at > datetime('now', '-30 days')
AND relevance_score < 0.5
GROUP BY topic
HAVING topic IS NOT NULL AND count > 2
ORDER BY count DESC;
"

# Missing connections
echo ""
echo "### Missing Connections"
echo "Topic pairs that should connect but don't appear together"

# Example: You use sqlite and ai, but no sqlite+ai items
sqlite3 "$DB" "
SELECT 'sqlite + embeddings' as pair,
  (SELECT COUNT(*) FROM items WHERE lower(title) LIKE '%sqlite%' AND created_at > datetime('now', '-30 days')) as sqlite_count,
  (SELECT COUNT(*) FROM items WHERE lower(title) LIKE '%embedding%' AND created_at > datetime('now', '-30 days')) as embedding_count,
  (SELECT COUNT(*) FROM items WHERE lower(title) LIKE '%sqlite%' AND lower(title) LIKE '%embedding%' AND created_at > datetime('now', '-30 days')) as both_count;
"
```

---

## Output Format

### Knowledge Map Report

```markdown
## 4DA Knowledge Map

**Generated:** 2026-01-22
**Items Mapped:** 847
**Topics Identified:** 45
**Relationships:** 234

---

### Graph Overview

```
                    ┌─────────┐
                    │   AI    │
                    └────┬────┘
                         │
         ┌───────────────┼───────────────┐
         │               │               │
    ┌────┴────┐    ┌────┴────┐    ┌────┴────┐
    │embedding │    │   LLM   │    │   RAG   │
    └────┬────┘    └────┬────┘    └────┬────┘
         │               │               │
         └───────┬───────┴───────┬───────┘
                 │               │
           ┌─────┴─────┐   ┌─────┴─────┐
           │  sqlite   │   │   rust    │
           │   -vec    │   │  (tauri)  │
           └───────────┘   └───────────┘
```

### Central Topics (High Connectivity)

| Topic | Connections | Centrality |
|-------|-------------|------------|
| rust | 12 | 0.85 |
| ai | 10 | 0.78 |
| database | 8 | 0.72 |
| typescript | 7 | 0.68 |
| async | 6 | 0.61 |

### Strongest Relationships

| Topic A | Topic B | Co-occurrences | Strength |
|---------|---------|----------------|----------|
| rust | async | 45 | Strong |
| ai | embedding | 38 | Strong |
| tauri | rust | 34 | Strong |
| sqlite | database | 28 | Medium |
| llm | rag | 23 | Medium |

### Topic Clusters

**Cluster 1: Systems Programming**
- rust, async, tokio, tauri, wasm
- 156 items, avg score 0.78

**Cluster 2: AI/ML**
- ai, llm, embedding, rag, claude
- 98 items, avg score 0.72

**Cluster 3: Data**
- database, sqlite, postgres, sql
- 67 items, avg score 0.68

**Cluster 4: Frontend**
- typescript, react, frontend, ui
- 54 items, avg score 0.65

### Entity Map

**Projects:**
- Tauri (45 mentions, central to your work)
- tokio (23 mentions, Rust async)
- sqlite-vec (12 mentions, growing)

**Companies:**
- Anthropic (18 mentions, Claude/AI)
- Mozilla (8 mentions, Rust ecosystem)

**People:**
- (extracted from HN authors)

### Knowledge Paths

**Your Core Path:**
```
You → rust → tauri → desktop apps → local-first → AI
```

**Emerging Path:**
```
sqlite-vec → embeddings → RAG → LLM → productivity
```

### Gaps Identified

1. **No items on:** Rust + WASM + AI
   - You use all three, but no intersection
   - Potential opportunity area

2. **Weak connection:** TypeScript ↔ AI
   - Strong in both, but rarely together
   - Consider: AI coding in TS projects

3. **Missing:** Tauri + mobile
   - Tauri 2.0 has mobile support
   - No items covering this yet

### Recommendations

1. **Explore:** "sqlite-vec machine learning"
2. **Add affinity:** "local-first AI"
3. **Watch for:** Tauri mobile content
4. **Deepen:** Rust + WASM intersection

---

### Interactive Exploration

To explore the graph:

```bash
# Find all connections for a topic
./map-topic.sh "rust"

# Find path between topics
./find-path.sh "sqlite" "ai"

# Get cluster members
./get-cluster.sh "Systems Programming"
```

*Generated by 4DA Knowledge Mapper*
```

---

## Visualization (ASCII Graph)

```bash
#!/bin/bash
# ASCII graph visualization

print_graph() {
  echo "
       rust ─────┬───── async
        │        │        │
        │        │        │
      tauri ─────┼───── tokio
        │        │        │
        │        │        │
       wasm ─────┴───── performance
  "
}

# For larger graphs, use adjacency list format
print_adjacency() {
  echo "Graph Adjacency (edges > 5 occurrences):"
  echo ""
  echo "rust: async(45), tauri(34), tokio(23), wasm(12)"
  echo "ai: embedding(38), llm(28), rag(23), claude(15)"
  echo "database: sqlite(28), postgres(18), sql(15)"
}
```

---

## Constraints

**CAN:**
- Extract entities from text
- Build relationship graphs
- Calculate graph metrics
- Find paths and clusters
- Identify gaps

**MUST:**
- Base graphs on actual data
- Explain relationship strength
- Provide exploration tools
- Acknowledge limitations

**CANNOT:**
- Invent relationships
- Access external graph databases
- Modify item content
- Make causal claims from correlation

---

*The Knowledge Mapper reveals the hidden structure of information. See connections others miss.*
