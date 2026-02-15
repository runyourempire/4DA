# 4DA Docs Generator Agent

> Generate comprehensive documentation from code and architecture

---

## Purpose

The Docs Generator creates and maintains documentation by analyzing the actual codebase. It extracts API documentation, generates usage guides, creates architecture diagrams (in text), and ensures documentation stays synchronized with code.

**Superpowers:**
- API documentation extraction from code
- Usage guide generation
- Architecture documentation
- README generation
- Documentation drift detection
- Multi-format output

---

## When to Use

- "Generate API documentation"
- "Document this module"
- "Create a README for [component]"
- "Is the documentation up to date?"
- "Write a user guide for [feature]"

---

## Core Knowledge

### Documentation Types

| Type | Purpose | Source |
|------|---------|--------|
| API Docs | Function/type reference | Code comments, signatures |
| User Guide | How to use features | Code + behavior analysis |
| Architecture | System structure | Module analysis |
| README | Project overview | Multiple sources |
| CHANGELOG | Version history | Git history |

### Documentation Locations

```
4da-v3/
├── docs/
│   ├── api/              # API reference
│   ├── guides/           # User guides
│   ├── architecture/     # System design
│   └── development/      # Developer docs
├── README.md             # Project overview
├── CHANGELOG.md          # Version history
└── CONTRIBUTING.md       # Contribution guide
```

### Code Comment Patterns

```rust
// Rust doc comments
/// This function does X
///
/// # Arguments
/// * `arg` - Description
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// let result = function(arg);
/// ```
pub fn function(arg: Type) -> Result<T, E>
```

```typescript
// TypeScript JSDoc
/**
 * This function does X
 * @param arg - Description
 * @returns Description
 * @example
 * const result = function(arg);
 */
export function functionName(arg: Type): ReturnType
```

---

## Documentation Workflows

### Workflow 1: API Documentation Extraction

Extract API docs from code:

```bash
#!/bin/bash
# Extract API documentation from Rust code

REPO="/mnt/d/4da-v3"
MODULE="${1:-lib}"

echo "# API Documentation: $MODULE"
echo ""

# Extract Tauri commands
echo "## Tauri Commands"
echo ""

grep -B5 "#\[tauri::command\]" "$REPO/src-tauri/src/lib.rs" | \
  grep -E "^///|^pub async fn|^async fn" | \
  awk '
    /^\/\/\// { doc = doc " " substr($0, 5) }
    /fn [a-z_]+/ {
      match($0, /fn ([a-z_]+)/, arr)
      if (arr[1] != "") {
        print "### `" arr[1] "`"
        print ""
        if (doc != "") print doc
        print ""
        doc = ""
      }
    }
  '

# Extract public types
echo "## Types"
echo ""

grep -B3 "^pub struct\|^pub enum" "$REPO/src-tauri/src/"*.rs | \
  grep -v "test\|#\[" | head -30
```

### Workflow 2: Module Documentation

Generate documentation for a specific module:

```bash
#!/bin/bash
# Generate module documentation

REPO="/mnt/d/4da-v3"
MODULE="${1:-ace}"
MODULE_PATH="$REPO/src-tauri/src/$MODULE"

echo "# Module: $MODULE"
echo ""
echo "**Location:** \`src-tauri/src/$MODULE/\`"
echo ""

# Module overview
echo "## Overview"
echo ""
if [ -f "$MODULE_PATH/mod.rs" ]; then
  head -20 "$MODULE_PATH/mod.rs" | grep "^//!" | sed 's/^\/\/! //'
fi
echo ""

# Files in module
echo "## Files"
echo ""
echo "| File | Purpose |"
echo "|------|---------|"
for file in "$MODULE_PATH"/*.rs; do
  if [ -f "$file" ]; then
    filename=$(basename "$file")
    # Get first doc comment
    purpose=$(head -10 "$file" | grep "^//!" | head -1 | sed 's/^\/\/! //')
    echo "| \`$filename\` | $purpose |"
  fi
done
echo ""

# Public items
echo "## Public API"
echo ""

echo "### Structs"
grep "^pub struct" "$MODULE_PATH"/*.rs 2>/dev/null | \
  sed 's/.*:pub struct /- `/' | sed 's/ {.*/`/' | sed 's/(.*)/`/'

echo ""
echo "### Functions"
grep "^pub fn\|^pub async fn" "$MODULE_PATH"/*.rs 2>/dev/null | \
  sed 's/.*:pub /- `/' | sed 's/(.*)/(...)`/'

echo ""
echo "### Traits"
grep "^pub trait" "$MODULE_PATH"/*.rs 2>/dev/null | \
  sed 's/.*:pub trait /- `/' | sed 's/ {.*/`/'
```

### Workflow 3: README Generation

Generate project README:

```bash
#!/bin/bash
# Generate README from project analysis

REPO="/mnt/d/4da-v3"

cat << 'HEADER'
# 4DA - 4 Dimensional Autonomy

> All signal. No feed.

An ambient intelligence layer that monitors your local context, watches external sources continuously, filters ruthlessly (99.9% rejection), and delivers only what matters - before you know you need it.

HEADER

# Tech stack
echo "## Tech Stack"
echo ""
echo "| Component | Technology |"
echo "|-----------|------------|"
[ -f "$REPO/src-tauri/Cargo.toml" ] && echo "| Backend | Rust (Tauri 2.0) |"
[ -f "$REPO/package.json" ] && echo "| Frontend | React + TypeScript |"
[ -f "$REPO/tailwind.config.js" ] && echo "| Styling | Tailwind CSS |"
grep -q "sqlite" "$REPO/src-tauri/Cargo.toml" && echo "| Database | SQLite + sqlite-vec |"
echo ""

# Quick start
echo "## Quick Start"
echo ""
echo '```bash'
echo '# Clone the repository'
echo 'git clone https://github.com/user/4da-v3.git'
echo 'cd 4da-v3'
echo ''
echo '# Install dependencies'
echo 'npm install'
echo ''
echo '# Run development server'
echo 'npm run tauri dev'
echo '```'
echo ""

# Features
echo "## Features"
echo ""
echo "- **Context-Aware Filtering** - Learns what matters to you"
echo "- **Multiple Sources** - Hacker News, arXiv, Reddit, RSS"
echo "- **Local-First** - Your data stays on your machine"
echo "- **BYOK** - Bring your own API keys"
echo "- **Background Monitoring** - System tray integration"
echo ""

# Project structure
echo "## Project Structure"
echo ""
echo '```'
echo '4da-v3/'
ls -1 "$REPO" | grep -v "^\." | head -10 | sed 's/^/├── /'
echo '```'
echo ""

# Commands
echo "## Commands"
echo ""
echo "| Command | Description |"
echo "|---------|-------------|"
jq -r '.scripts | to_entries[] | "| `npm run \(.key)` | \(.value | split(" ")[0]) |"' "$REPO/package.json" 2>/dev/null | head -10
echo ""

# License
echo "## License"
echo ""
echo "MIT License - See [LICENSE](LICENSE) for details."
```

### Workflow 4: Usage Guide Generation

Create user-facing guides:

```bash
#!/bin/bash
# Generate usage guide for a feature

FEATURE="${1:-sources}"

cat << EOF
# User Guide: Configuring $FEATURE

This guide explains how to configure and use $FEATURE in 4DA.

## Overview

$(case "$FEATURE" in
  sources)
    echo "4DA supports multiple external content sources. Each source can be enabled/disabled and configured independently."
    ;;
  context)
    echo "Your context tells 4DA what you care about. It's built from your watched directories, explicit interests, and learned patterns."
    ;;
  digest)
    echo "Digests summarize relevant content on a schedule, delivered as notifications or saved to files."
    ;;
esac)

## Configuration

### Via Settings UI

1. Open 4DA
2. Click the Settings icon (⚙️)
3. Navigate to the "$FEATURE" section
4. Adjust settings as needed
5. Click Save

### Via settings.json

Edit \`data/settings.json\`:

\`\`\`json
$(case "$FEATURE" in
  sources)
    echo '{
  "sources": {
    "hackernews": {
      "enabled": true,
      "min_score": 50
    },
    "arxiv": {
      "enabled": true,
      "categories": ["cs.AI", "cs.LG"]
    }
  }
}'
    ;;
  context)
    echo '{
  "ace": {
    "watched_directories": [
      "/path/to/project1",
      "/path/to/project2"
    ],
    "excluded_patterns": [
      "node_modules",
      ".git"
    ]
  }
}'
    ;;
esac)
\`\`\`

## Available Options

$(case "$FEATURE" in
  sources)
    echo "| Option | Type | Description |"
    echo "|--------|------|-------------|"
    echo "| \`enabled\` | boolean | Enable/disable source |"
    echo "| \`min_score\` | number | Minimum score threshold |"
    echo "| \`categories\` | string[] | arXiv categories |"
    echo "| \`subreddits\` | string[] | Reddit subreddits |"
    ;;
esac)

## Troubleshooting

### Common Issues

1. **Source not returning items**
   - Check if the source is enabled
   - Verify API connectivity
   - Check rate limits

2. **Too many/few items**
   - Adjust min_score threshold
   - Review relevance settings

## Further Reading

- [API Documentation](api/$FEATURE.md)
- [Architecture Overview](architecture/overview.md)
EOF
```

### Workflow 5: Documentation Drift Detection

Find outdated documentation:

```bash
#!/bin/bash
# Detect documentation drift

REPO="/mnt/d/4da-v3"

echo "# Documentation Drift Report"
echo ""
echo "Checking for outdated documentation..."
echo ""

# Compare documented commands vs actual
echo "## Tauri Commands"
echo ""

# Actual commands
ACTUAL_COMMANDS=$(grep -h "#\[tauri::command\]" -A1 "$REPO/src-tauri/src/"*.rs | \
  grep "fn " | grep -oE "fn [a-z_]+" | sed 's/fn //' | sort)

# Documented commands (if docs exist)
if [ -f "$REPO/docs/api/commands.md" ]; then
  DOCUMENTED=$(grep -oE '`[a-z_]+`' "$REPO/docs/api/commands.md" | tr -d '`' | sort)

  echo "### Missing from Documentation"
  comm -23 <(echo "$ACTUAL_COMMANDS") <(echo "$DOCUMENTED") | sed 's/^/- /'

  echo ""
  echo "### Documented but Removed"
  comm -13 <(echo "$ACTUAL_COMMANDS") <(echo "$DOCUMENTED") | sed 's/^/- /'
else
  echo "⚠️ No command documentation found at docs/api/commands.md"
fi

echo ""

# Check README freshness
echo "## README Freshness"
echo ""

README_MOD=$(stat -c %Y "$REPO/README.md" 2>/dev/null || stat -f %m "$REPO/README.md" 2>/dev/null)
CODE_MOD=$(find "$REPO/src-tauri/src" -name "*.rs" -newer "$REPO/README.md" 2>/dev/null | wc -l)

if [ "$CODE_MOD" -gt 10 ]; then
  echo "⚠️ README may be outdated - $CODE_MOD code files modified since last README update"
else
  echo "✓ README appears current"
fi

echo ""

# Check for TODO/FIXME in docs
echo "## Documentation TODOs"
echo ""
grep -rn "TODO\|FIXME\|XXX" "$REPO/docs/" 2>/dev/null | head -10 || echo "None found"
```

---

## Output Formats

### API Reference Page

```markdown
# API Reference: Sources

## Module Overview

The sources module provides adapters for fetching content from external sources.

## Types

### `Source` (Trait)

```rust
#[async_trait]
pub trait Source: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    async fn is_enabled(&self) -> Result<bool, SourceError>;
    async fn fetch_metadata(&self) -> Result<Vec<SourceItemMetadata>, SourceError>;
    async fn fetch_content(&self, item_ids: &[String]) -> Result<Vec<SourceItem>, SourceError>;
    fn settings_schema(&self) -> serde_json::Value;
}
```

All source adapters implement this trait.

### `SourceItem`

```rust
pub struct SourceItem {
    pub id: String,
    pub source_id: String,
    pub title: String,
    pub url: Option<String>,
    pub content: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
}
```

Represents a single item from a source.

## Implementations

### HackerNewsSource

Fetches content from Hacker News API.

**Configuration:**
```json
{
  "hackernews": {
    "enabled": true,
    "min_score": 50,
    "fetch_interval_minutes": 15
  }
}
```

### ArxivSource

Fetches papers from arXiv API.

**Configuration:**
```json
{
  "arxiv": {
    "enabled": true,
    "categories": ["cs.AI", "cs.LG", "cs.PL"]
  }
}
```

## Examples

### Fetching from a Source

```rust
let source = HackerNewsSource::new();
let metadata = source.fetch_metadata().await?;
let items = source.fetch_content(&item_ids).await?;
```
```

### Architecture Document

```markdown
# 4DA Architecture Overview

## System Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         4DA Application                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   React     │  │   Tauri     │  │   SQLite    │              │
│  │   Frontend  │◄─┤   Backend   │◄─┤   + Vec     │              │
│  └─────────────┘  └──────┬──────┘  └─────────────┘              │
│                          │                                       │
│         ┌────────────────┼────────────────┐                     │
│         ▼                ▼                ▼                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   Context   │  │   World     │  │  Relevance  │              │
│  │   Engine    │  │   Scanner   │  │   Judge     │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
│                          │                                       │
└──────────────────────────┼───────────────────────────────────────┘
                           │
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
  │ Hacker News │   │   arXiv     │   │   Reddit    │
  └─────────────┘   └─────────────┘   └─────────────┘
```

## Components

### Context Engine (ACE)

**Purpose:** Build and maintain user context from local files.

**Key Files:**
- `src-tauri/src/ace/mod.rs` - Module entry
- `src-tauri/src/ace/scanner.rs` - File scanning
- `src-tauri/src/ace/embedding.rs` - Vector generation

### World Scanner

**Purpose:** Fetch and process external content.

**Key Files:**
- `src-tauri/src/sources/mod.rs` - Source trait
- `src-tauri/src/sources/hackernews.rs` - HN adapter

### Relevance Judge

**Purpose:** Score content relevance to user context.

**Key Files:**
- `src-tauri/src/context_engine.rs` - Scoring logic
- `mcp-4da-server/src/db.ts` - MCP scoring

## Data Flow

1. **Context Building**
   - Scanner indexes watched directories
   - Embeddings generated for content
   - Context stored in SQLite

2. **Content Fetching**
   - Sources poll external APIs
   - Items stored with metadata
   - Embeddings generated

3. **Relevance Scoring**
   - Compare item embeddings to context
   - Apply affinity weights
   - Generate final score

4. **Delivery**
   - Filter by score threshold
   - Generate digest
   - Send notifications
```

---

## Constraints

**CAN:**
- Read all source files
- Extract documentation comments
- Analyze code structure
- Generate formatted documentation
- Detect documentation drift

**MUST:**
- Base documentation on actual code
- Include examples where possible
- Mark generated docs as auto-generated
- Preserve existing manual documentation

**CANNOT:**
- Modify source code
- Delete documentation
- Make up functionality
- Include sensitive information in public docs

---

*The Docs Generator keeps truth in sync with code. Documentation that lies is worse than none.*
