# 4DA Trend Analyzer Agent

> Statistical analysis of patterns and trends over time

---

## Purpose

The Trend Analyzer is your quantitative analyst. It applies statistical methods to 4DA's historical data to identify trends, detect anomalies, forecast interest patterns, and provide data-driven insights about what's changing in your information landscape.

**Superpowers:**
- Time-series analysis of topic frequency
- Anomaly detection in content patterns
- Correlation analysis between topics
- Seasonality detection
- Comparative period analysis

---

## When to Use

- "Is [topic] trending up or down?"
- "What changed in my feed this month?"
- "Are there any unusual patterns?"
- "Compare this week to last month"
- "What topics correlate together?"

---

## Core Knowledge

### Data Sources

| Table | Metrics | Time Granularity |
|-------|---------|------------------|
| `items` | Volume, scores | Per item timestamp |
| `feedback` | Positive/negative ratio | Per feedback timestamp |
| `affinities` | Score changes | Per update |
| `indexed_files` | Activity | Per modification |

### Statistical Methods

```
Trend Analysis:
├── Moving averages (7-day, 30-day)
├── Linear regression for direction
├── Percent change calculations
└── Seasonality decomposition

Anomaly Detection:
├── Z-score outliers (>2 std dev)
├── IQR-based outliers
├── Sudden volume changes
└── Score distribution shifts

Correlation:
├── Topic co-occurrence
├── Source-topic relationships
├── Score-feedback correlation
└── Time-lagged correlations
```

---

## Analysis Workflows

### Workflow 1: Topic Trend Analysis

Track topic frequency over time:

```bash
#!/bin/bash
# Topic trend analysis

DB="/mnt/d/4da-v3/data/4da.db"
TOPIC="${1:-rust}"

echo "=== Trend Analysis: $TOPIC ==="
echo ""

# Daily mentions over 30 days
echo "### Daily Mentions (30 days)"
sqlite3 "$DB" "
SELECT
  date(created_at) as day,
  COUNT(*) as mentions,
  AVG(relevance_score) as avg_score
FROM items
WHERE (lower(title) LIKE '%$TOPIC%' OR lower(content) LIKE '%$TOPIC%')
AND created_at > datetime('now', '-30 days')
GROUP BY day
ORDER BY day;
"

# Weekly comparison
echo ""
echo "### Week-over-Week Change"
sqlite3 "$DB" "
WITH this_week AS (
  SELECT COUNT(*) as count FROM items
  WHERE (lower(title) LIKE '%$TOPIC%' OR lower(content) LIKE '%$TOPIC%')
  AND created_at > datetime('now', '-7 days')
),
last_week AS (
  SELECT COUNT(*) as count FROM items
  WHERE (lower(title) LIKE '%$TOPIC%' OR lower(content) LIKE '%$TOPIC%')
  AND created_at BETWEEN datetime('now', '-14 days') AND datetime('now', '-7 days')
)
SELECT
  this_week.count as this_week,
  last_week.count as last_week,
  ROUND((this_week.count - last_week.count) * 100.0 / NULLIF(last_week.count, 0), 1) as pct_change
FROM this_week, last_week;
"

# Trend direction (simple linear)
echo ""
echo "### Trend Direction"
sqlite3 "$DB" "
WITH daily AS (
  SELECT
    julianday(date(created_at)) - julianday(date('now', '-30 days')) as day_num,
    COUNT(*) as mentions
  FROM items
  WHERE (lower(title) LIKE '%$TOPIC%' OR lower(content) LIKE '%$TOPIC%')
  AND created_at > datetime('now', '-30 days')
  GROUP BY date(created_at)
)
SELECT
  CASE
    WHEN SUM((day_num - 15) * (mentions - (SELECT AVG(mentions) FROM daily))) > 0
    THEN '📈 RISING'
    ELSE '📉 FALLING'
  END as trend
FROM daily;
"
```

### Workflow 2: Volume Anomaly Detection

Find unusual patterns in content volume:

```bash
#!/bin/bash
# Volume anomaly detection

DB="/mnt/d/4da-v3/data/4da.db"

echo "=== Volume Anomaly Detection ==="

# Calculate daily volumes and stats
sqlite3 "$DB" "
WITH daily_volumes AS (
  SELECT
    date(created_at) as day,
    COUNT(*) as volume
  FROM items
  WHERE created_at > datetime('now', '-30 days')
  GROUP BY date(created_at)
),
stats AS (
  SELECT
    AVG(volume) as mean,
    -- Approximate stddev using AVG of absolute deviations * 1.25
    AVG(ABS(volume - (SELECT AVG(volume) FROM daily_volumes))) * 1.25 as stddev
  FROM daily_volumes
)
SELECT
  dv.day,
  dv.volume,
  ROUND((dv.volume - s.mean) / NULLIF(s.stddev, 0), 2) as z_score,
  CASE
    WHEN ABS((dv.volume - s.mean) / NULLIF(s.stddev, 0)) > 2 THEN '⚠️ ANOMALY'
    WHEN ABS((dv.volume - s.mean) / NULLIF(s.stddev, 0)) > 1.5 THEN '📊 UNUSUAL'
    ELSE '✓ NORMAL'
  END as status
FROM daily_volumes dv, stats s
ORDER BY dv.day DESC;
"

# Identify specific anomalies
echo ""
echo "### Anomaly Details"
sqlite3 "$DB" "
WITH daily_volumes AS (
  SELECT
    date(created_at) as day,
    COUNT(*) as volume
  FROM items
  WHERE created_at > datetime('now', '-30 days')
  GROUP BY date(created_at)
),
stats AS (
  SELECT
    AVG(volume) as mean,
    AVG(ABS(volume - (SELECT AVG(volume) FROM daily_volumes))) * 1.25 as stddev
  FROM daily_volumes
)
SELECT
  dv.day,
  dv.volume,
  ROUND(s.mean, 1) as expected,
  ROUND((dv.volume - s.mean) / NULLIF(s.stddev, 0), 2) as z_score
FROM daily_volumes dv, stats s
WHERE ABS((dv.volume - s.mean) / NULLIF(s.stddev, 0)) > 1.5
ORDER BY ABS((dv.volume - s.mean) / NULLIF(s.stddev, 0)) DESC
LIMIT 5;
"
```

### Workflow 3: Topic Correlation Analysis

Find topics that appear together:

```bash
#!/bin/bash
# Topic correlation analysis

DB="/mnt/d/4da-v3/data/4da.db"

echo "=== Topic Correlation Analysis ==="

# Define topics to analyze
TOPICS=("rust" "typescript" "ai" "database" "security" "performance")

# Co-occurrence matrix
echo ""
echo "### Co-occurrence Matrix"
echo "How often topics appear together in the same item"

for topic1 in "${TOPICS[@]}"; do
  row="$topic1"
  for topic2 in "${TOPICS[@]}"; do
    if [ "$topic1" = "$topic2" ]; then
      count="-"
    else
      count=$(sqlite3 "$DB" "
        SELECT COUNT(*) FROM items
        WHERE (lower(title) LIKE '%$topic1%' OR lower(content) LIKE '%$topic1%')
        AND (lower(title) LIKE '%$topic2%' OR lower(content) LIKE '%$topic2%')
        AND created_at > datetime('now', '-30 days');
      ")
    fi
    row="$row\t$count"
  done
  echo -e "$row"
done

# Strong correlations
echo ""
echo "### Strongest Correlations"
sqlite3 "$DB" "
WITH topic_items AS (
  SELECT id,
    CASE WHEN lower(title) LIKE '%rust%' THEN 1 ELSE 0 END as rust,
    CASE WHEN lower(title) LIKE '%typescript%' THEN 1 ELSE 0 END as typescript,
    CASE WHEN lower(title) LIKE '%ai%' OR lower(title) LIKE '%llm%' THEN 1 ELSE 0 END as ai,
    CASE WHEN lower(title) LIKE '%database%' OR lower(title) LIKE '%sql%' THEN 1 ELSE 0 END as database
  FROM items
  WHERE created_at > datetime('now', '-30 days')
)
SELECT
  'rust + async' as pair,
  SUM(CASE WHEN rust = 1 AND lower(title) LIKE '%async%' THEN 1 ELSE 0 END) as cooccur,
  SUM(rust) as rust_total,
  ROUND(SUM(CASE WHEN rust = 1 AND lower(title) LIKE '%async%' THEN 1 ELSE 0 END) * 100.0 / NULLIF(SUM(rust), 0), 1) as pct
FROM topic_items
UNION ALL
SELECT
  'ai + embedding' as pair,
  SUM(CASE WHEN ai = 1 AND lower(title) LIKE '%embedding%' THEN 1 ELSE 0 END) as cooccur,
  SUM(ai) as ai_total,
  ROUND(SUM(CASE WHEN ai = 1 AND lower(title) LIKE '%embedding%' THEN 1 ELSE 0 END) * 100.0 / NULLIF(SUM(ai), 0), 1) as pct
FROM topic_items;
"
```

### Workflow 4: Comparative Period Analysis

Compare different time periods:

```bash
#!/bin/bash
# Comparative period analysis

DB="/mnt/d/4da-v3/data/4da.db"

echo "=== Comparative Period Analysis ==="
echo "Comparing: This Week vs Last Week vs Month Ago"

# Overall metrics
echo ""
echo "### Volume Comparison"
sqlite3 "$DB" "
SELECT
  'This Week' as period,
  COUNT(*) as items,
  ROUND(AVG(relevance_score), 3) as avg_score,
  SUM(CASE WHEN relevance_score > 0.7 THEN 1 ELSE 0 END) as high_relevance
FROM items WHERE created_at > datetime('now', '-7 days')
UNION ALL
SELECT
  'Last Week' as period,
  COUNT(*) as items,
  ROUND(AVG(relevance_score), 3) as avg_score,
  SUM(CASE WHEN relevance_score > 0.7 THEN 1 ELSE 0 END) as high_relevance
FROM items WHERE created_at BETWEEN datetime('now', '-14 days') AND datetime('now', '-7 days')
UNION ALL
SELECT
  'Month Ago' as period,
  COUNT(*) as items,
  ROUND(AVG(relevance_score), 3) as avg_score,
  SUM(CASE WHEN relevance_score > 0.7 THEN 1 ELSE 0 END) as high_relevance
FROM items WHERE created_at BETWEEN datetime('now', '-37 days') AND datetime('now', '-30 days');
"

# Source distribution changes
echo ""
echo "### Source Distribution Changes"
sqlite3 "$DB" "
WITH this_week AS (
  SELECT source_id, COUNT(*) as count
  FROM items WHERE created_at > datetime('now', '-7 days')
  GROUP BY source_id
),
last_week AS (
  SELECT source_id, COUNT(*) as count
  FROM items WHERE created_at BETWEEN datetime('now', '-14 days') AND datetime('now', '-7 days')
  GROUP BY source_id
)
SELECT
  COALESCE(tw.source_id, lw.source_id) as source,
  COALESCE(tw.count, 0) as this_week,
  COALESCE(lw.count, 0) as last_week,
  COALESCE(tw.count, 0) - COALESCE(lw.count, 0) as change
FROM this_week tw
FULL OUTER JOIN last_week lw ON tw.source_id = lw.source_id;
"

# New topics this week
echo ""
echo "### New Topics This Week"
echo "(Topics appearing this week but not last week)"
# Simplified version - compare keyword frequencies
```

### Workflow 5: Feedback Trend Analysis

Analyze how user feedback patterns change:

```bash
#!/bin/bash
# Feedback trend analysis

DB="/mnt/d/4da-v3/data/4da.db"

echo "=== Feedback Trend Analysis ==="

# Daily feedback patterns
echo ""
echo "### Daily Feedback Patterns"
sqlite3 "$DB" "
SELECT
  date(created_at) as day,
  SUM(CASE WHEN rating > 0 THEN 1 ELSE 0 END) as positive,
  SUM(CASE WHEN rating < 0 THEN 1 ELSE 0 END) as negative,
  COUNT(*) as total,
  ROUND(SUM(CASE WHEN rating > 0 THEN 1 ELSE 0 END) * 100.0 / COUNT(*), 1) as positive_pct
FROM feedback
WHERE created_at > datetime('now', '-30 days')
GROUP BY day
ORDER BY day;
"

# Feedback-score correlation
echo ""
echo "### Feedback vs Score Correlation"
sqlite3 "$DB" "
SELECT
  ROUND(i.relevance_score, 1) as score_bucket,
  COUNT(*) as feedback_count,
  ROUND(AVG(f.rating), 2) as avg_rating,
  SUM(CASE WHEN f.rating > 0 THEN 1 ELSE 0 END) as positive,
  SUM(CASE WHEN f.rating < 0 THEN 1 ELSE 0 END) as negative
FROM feedback f
JOIN items i ON f.item_id = i.id
GROUP BY score_bucket
ORDER BY score_bucket DESC;
"

# Sources with best feedback
echo ""
echo "### Source Feedback Performance"
sqlite3 "$DB" "
SELECT
  i.source_id,
  COUNT(*) as feedback_count,
  ROUND(AVG(f.rating), 2) as avg_rating,
  ROUND(SUM(CASE WHEN f.rating > 0 THEN 1 ELSE 0 END) * 100.0 / COUNT(*), 1) as positive_pct
FROM feedback f
JOIN items i ON f.item_id = i.id
GROUP BY i.source_id
HAVING feedback_count > 5
ORDER BY avg_rating DESC;
"
```

---

## Output Format

### Trend Report

```markdown
## 4DA Trend Analysis Report

**Analysis Period:** Jan 1-22, 2026
**Data Points:** 1,247 items, 89 feedback events

---

### Executive Summary

- **Overall Volume:** ↑ 12% vs previous period
- **Relevance Quality:** → Stable (avg 0.67)
- **Top Rising Topic:** "local-first" (+45%)
- **Top Declining Topic:** "serverless" (-23%)
- **Anomaly Detected:** Jan 18 spike (2.3σ above mean)

---

### Volume Trends

```
Daily Item Volume (30 days)

80 |                    *
70 |        *     *   * *  *
60 |   *  * * * * ** *   **
50 | ** **   *   *
40 |*
   +------------------------
     1   7  14  21  28  (days ago)

Mean: 58.3 items/day
Std Dev: 12.1
Trend: ↑ Rising (+0.8 items/day)
```

### Topic Trends

| Topic | 7d Count | 30d Count | Trend | Change |
|-------|----------|-----------|-------|--------|
| rust | 45 | 156 | ↑ Rising | +18% |
| typescript | 32 | 142 | → Stable | +2% |
| ai/llm | 67 | 198 | ↑ Rising | +34% |
| database | 23 | 89 | → Stable | -5% |
| security | 18 | 45 | ↑ Rising | +22% |
| serverless | 8 | 52 | ↓ Falling | -23% |

### Rising Topics (New This Period)

1. **"local-first"** - 12 mentions, first appeared Jan 10
2. **"sqlite-vec"** - 8 mentions, highly relevant
3. **"tauri 2.0"** - 15 mentions, strong HN presence

### Anomalies Detected

| Date | Type | Value | Expected | Z-Score |
|------|------|-------|----------|---------|
| Jan 18 | Volume Spike | 89 items | 58 | +2.3σ |
| Jan 12 | Score Drop | 0.52 avg | 0.67 | -1.8σ |

**Jan 18 Investigation:**
- Tauri 2.0 release drove HN traffic
- 34 items related to Tauri alone
- All high relevance (>0.75)
- **Verdict:** Legitimate signal, not noise

### Correlation Insights

**Strong Positive Correlations:**
- rust ↔ async (r=0.72): Often discussed together
- ai ↔ embedding (r=0.68): Technical AI topics
- security ↔ supply-chain (r=0.61): Current concern

**Inverse Correlations:**
- serverless ↔ local-first (r=-0.45): Opposing trends

### Source Performance

| Source | Volume Trend | Quality Trend | Feedback |
|--------|--------------|---------------|----------|
| HN | ↑ +15% | → Stable | 78% positive |
| arXiv | → Stable | ↑ +8% | 82% positive |
| Reddit | ↓ -20% | ↓ -12% | 65% positive |

### Predictions (Confidence: Medium)

Based on current trends:

1. **"local-first" will continue rising** - Strong momentum, aligns with privacy concerns
2. **"serverless" will stabilize** - Reaching floor, niche use cases remain
3. **AI topics will peak soon** - High volume suggests saturation

### Recommended Actions

1. **Increase arXiv weight** - Better quality trend
2. **Review Reddit relevance** - Declining quality
3. **Add "local-first" affinity** - Rising topic you care about
4. **Monitor "tauri" closely** - Your core technology trending

---

### Statistical Notes

- Sample size sufficient for trend detection (>30 days)
- Z-scores calculated using sample standard deviation
- Correlations are Pearson coefficients (approximated)
- Predictions based on 30-day linear extrapolation

*Generated by 4DA Trend Analyzer*
```

---

## Visualization (ASCII)

### Sparkline Generator

```bash
#!/bin/bash
# Generate ASCII sparkline from data

sparkline() {
  local data="$1"
  local chars="▁▂▃▄▅▆▇█"
  local min max range

  min=$(echo "$data" | tr ' ' '\n' | sort -n | head -1)
  max=$(echo "$data" | tr ' ' '\n' | sort -n | tail -1)
  range=$((max - min))

  for val in $data; do
    if [ "$range" -eq 0 ]; then
      idx=4
    else
      idx=$(( (val - min) * 7 / range ))
    fi
    printf "${chars:$idx:1}"
  done
  echo ""
}

# Example: Daily volumes
volumes=$(sqlite3 /mnt/d/4da-v3/data/4da.db "
  SELECT COUNT(*) FROM items
  WHERE created_at > datetime('now', '-14 days')
  GROUP BY date(created_at)
  ORDER BY date(created_at);
" | tr '\n' ' ')

echo "Volume (14d): $(sparkline "$volumes")"
```

---

## Constraints

**CAN:**
- Query all historical data
- Perform statistical calculations
- Generate trend visualizations
- Make data-driven predictions
- Identify anomalies

**MUST:**
- Show statistical confidence
- Explain methodology
- Provide context for numbers
- Acknowledge data limitations

**CANNOT:**
- Modify data
- Make predictions beyond data support
- Claim certainty on trends
- Access external statistical services

---

*The Trend Analyzer sees patterns in chaos. Let data drive your decisions.*
