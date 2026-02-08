# /optimize-context

Analyze and optimize your local context for better relevance.

## Usage

```
/optimize-context              # Full analysis
/optimize-context --quick      # Quick recommendations only
/optimize-context --directory=/path  # Analyze specific directory
```

## What This Does

This command invokes the **4da-context-optimizer** agent to improve your context:

1. **Analyzes watched directories** - file types, sizes, structure
2. **Detects technology stack** - languages, frameworks, tools
3. **Identifies noise** - build artifacts, generated files
4. **Finds coverage gaps** - projects not being watched
5. **Infers interests** - from code patterns and git activity
6. **Generates recommendations** - with specific actions

## Example Output

```
## Context Optimization Report

**Directories Analyzed:** 3 | **Context Score:** 72/100

### Current Context Summary
| Directory | Tech Stack | Files |
|-----------|------------|-------|
| ~/projects/4da-v3 | Rust, TypeScript, Tauri | 847 |
| ~/projects/api | Go, REST | 234 |
| ~/projects/ml | Python, ML | 156 |

### Detected Technology Stack
| Technology | Confidence | Evidence |
|------------|------------|----------|
| Rust | High | Cargo.toml, .rs files |
| TypeScript | High | package.json, .ts files |
| Tauri | High | tauri.conf.json |
| React | High | package.json deps |

### Optimization Recommendations

#### High Priority

1. **Add exclusion: `target/`**
   - Impact: Remove 450MB of noise
   - Command: `jq '.ace.excluded_patterns += ["target"]' settings.json`

2. **Add directory: `~/projects/rust-utils`**
   - Contains 12 Rust utilities you frequently use
   - Would improve Rust content relevance

3. **Declare interest: "vector databases"**
   - Detected in code but not in affinities
   - Command: `sqlite3 4da.db "INSERT INTO affinities..."`

#### Medium Priority

4. **Enable arXiv categories: cs.DB, cs.IR**
   - You work with databases and retrieval
   - Currently only cs.AI enabled

### Noise Detection
| Pattern | Size | Action |
|---------|------|--------|
| node_modules | 120MB | Already excluded ✓ |
| target/ | 450MB | **Add to exclusions** |
| .next/ | 89MB | **Add to exclusions** |

### Context Quality Score
| Dimension | Score | Notes |
|-----------|-------|-------|
| Coverage | 8/10 | Most projects watched |
| Noise | 6/10 | Some artifacts indexed |
| Freshness | 9/10 | Good scan interval |
| **Overall** | **72/100** | Room for improvement |

### Quick Wins
```bash
# Add exclusions
jq '.ace.excluded_patterns += ["target", ".next"]' settings.json > tmp && mv tmp settings.json

# Add interest
sqlite3 data/4da.db "INSERT INTO affinities (topic, score, source) VALUES ('vector databases', 0.8, 'explicit');"
```
```

## Agent Reference

Full agent definition: `.claude/agents/4da-context-optimizer.md`
