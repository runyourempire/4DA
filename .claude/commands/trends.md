# /trends

Analyze patterns and trends in 4DA's content over time.

## Usage

```
/trends                        # 30-day trend analysis
/trends --topic=rust           # Specific topic trend
/trends --compare              # Week-over-week comparison
/trends --anomalies            # Detect unusual patterns
```

## What This Does

This command invokes the **4da-trend-analyzer** agent to apply statistical analysis:

1. **Calculates topic frequency** over time with moving averages
2. **Detects anomalies** using z-score analysis
3. **Identifies rising/falling topics** with percent change
4. **Correlates topics** that appear together
5. **Compares time periods** (this week vs last week vs month ago)

## Example Output

```
## 4DA Trend Analysis

**Period:** Last 30 days | **Data Points:** 1,247 items

### Executive Summary
- **Volume:** ↑ 12% vs previous period
- **Top Rising:** "local-first" (+45%)
- **Top Declining:** "serverless" (-23%)
- **Anomaly:** Jan 18 spike (2.3σ above mean)

### Topic Trends

| Topic | 7d | 30d | Trend | Change |
|-------|-----|-----|-------|--------|
| rust | 45 | 156 | ↑ Rising | +18% |
| ai/llm | 67 | 198 | ↑ Rising | +34% |
| serverless | 8 | 52 | ↓ Falling | -23% |

### Volume Sparkline (14 days)
▃▅▆▄▅▇█▆▅▄▅▆▇▅

### Anomalies Detected

| Date | Type | Value | Expected | Status |
|------|------|-------|----------|--------|
| Jan 18 | Volume | 89 | 58 | ⚠️ +2.3σ |

**Investigation:** Tauri 2.0 release drove legitimate traffic spike.

### Correlations
- rust ↔ async (r=0.72): Often discussed together
- ai ↔ embedding (r=0.68): Technical AI topics

### Predictions
1. "local-first" will continue rising
2. "serverless" will stabilize at lower level
3. AI topics may peak soon (saturation signals)
```

## Agent Reference

Full agent definition: `.claude/agents/4da-trend-analyzer.md`
