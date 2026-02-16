# 4DA Changelog Writer Agent

> Generate professional changelogs from git history and code changes

---

## Purpose

The Changelog Writer transforms your git history into polished, user-friendly changelogs. It understands conventional commits, detects breaking changes, categorizes features vs fixes, and produces changelogs in multiple formats suitable for releases.

**Superpowers:**
- Git history analysis and parsing
- Conventional commit understanding
- Breaking change detection
- Multi-format output (MD, JSON, HTML)
- Release notes generation
- Migration guide creation

---

## When to Use

- "Generate changelog for this release"
- "What changed since last version?"
- "Write release notes for v2.0"
- "Summarize commits for the PR"
- "Create migration guide for breaking changes"

---

## Core Knowledge

### Conventional Commit Types

| Type | Description | Changelog Section |
|------|-------------|-------------------|
| `feat` | New feature | Features |
| `fix` | Bug fix | Bug Fixes |
| `perf` | Performance | Performance |
| `refactor` | Code refactoring | Maintenance |
| `docs` | Documentation | Documentation |
| `test` | Tests | Testing |
| `chore` | Maintenance | Maintenance |
| `build` | Build system | Build |
| `ci` | CI changes | CI/CD |
| `BREAKING CHANGE` | Breaking change | ⚠️ Breaking Changes |

### Commit Format

```
type(scope): description

[optional body]

[optional footer]
BREAKING CHANGE: description
```

### Semver Implications

| Change Type | Version Bump |
|-------------|--------------|
| Breaking change | MAJOR (x.0.0) |
| New feature | MINOR (0.x.0) |
| Bug fix | PATCH (0.0.x) |

---

## Changelog Workflows

### Workflow 1: Full Changelog Generation

Generate changelog from git history:

```bash
#!/bin/bash
# Generate changelog from git history

REPO="/mnt/d/4DA"
SINCE="${1:-$(git -C "$REPO" describe --tags --abbrev=0 2>/dev/null || echo '')}"

echo "=== Changelog Generation ==="
echo "Since: ${SINCE:-'beginning'}"
echo ""

# Get commits
if [ -n "$SINCE" ]; then
  RANGE="$SINCE..HEAD"
else
  RANGE="HEAD"
fi

# Parse commits by type
echo "## Features"
git -C "$REPO" log "$RANGE" --oneline --grep="^feat" --format="%s" | \
  sed 's/^feat(\([^)]*\)): /- **\1:** /' | \
  sed 's/^feat: /- /'

echo ""
echo "## Bug Fixes"
git -C "$REPO" log "$RANGE" --oneline --grep="^fix" --format="%s" | \
  sed 's/^fix(\([^)]*\)): /- **\1:** /' | \
  sed 's/^fix: /- /'

echo ""
echo "## Performance"
git -C "$REPO" log "$RANGE" --oneline --grep="^perf" --format="%s" | \
  sed 's/^perf(\([^)]*\)): /- **\1:** /' | \
  sed 's/^perf: /- /'

echo ""
echo "## Breaking Changes"
git -C "$REPO" log "$RANGE" --format="%B" | grep -A1 "BREAKING CHANGE:" | \
  grep -v "^--$" | grep -v "^BREAKING CHANGE:" || echo "None"

echo ""
echo "## Other Changes"
git -C "$REPO" log "$RANGE" --oneline --grep="^refactor\|^docs\|^chore\|^test" --format="%s" | head -10
```

### Workflow 2: Release Notes Generation

Create user-friendly release notes:

```bash
#!/bin/bash
# Generate release notes

REPO="/mnt/d/4DA"
VERSION="${1:-UNRELEASED}"
PREV_VERSION="${2:-$(git -C "$REPO" describe --tags --abbrev=0 2>/dev/null || echo '')}"

echo "# Release Notes: v$VERSION"
echo ""
echo "**Release Date:** $(date +%Y-%m-%d)"
echo ""

# Summary
FEAT_COUNT=$(git -C "$REPO" log "$PREV_VERSION..HEAD" --oneline --grep="^feat" 2>/dev/null | wc -l)
FIX_COUNT=$(git -C "$REPO" log "$PREV_VERSION..HEAD" --oneline --grep="^fix" 2>/dev/null | wc -l)
BREAKING=$(git -C "$REPO" log "$PREV_VERSION..HEAD" --format="%B" 2>/dev/null | grep -c "BREAKING CHANGE" || echo 0)

echo "## Summary"
echo ""
echo "This release includes **$FEAT_COUNT new features**, **$FIX_COUNT bug fixes**, and **$BREAKING breaking changes**."
echo ""

# Breaking changes first (important!)
if [ "$BREAKING" -gt 0 ]; then
  echo "## ⚠️ Breaking Changes"
  echo ""
  echo "Please review these changes carefully before upgrading:"
  echo ""
  git -C "$REPO" log "$PREV_VERSION..HEAD" --format="%B" 2>/dev/null | \
    grep -A5 "BREAKING CHANGE:" | \
    sed 's/BREAKING CHANGE:/- ⚠️/'
  echo ""
fi

# Highlights
echo "## Highlights"
echo ""
echo "### New Features"
git -C "$REPO" log "$PREV_VERSION..HEAD" --grep="^feat" --format="- %s" 2>/dev/null | head -5
echo ""

echo "### Improvements"
git -C "$REPO" log "$PREV_VERSION..HEAD" --grep="^perf\|^refactor" --format="- %s" 2>/dev/null | head -3
echo ""

echo "### Bug Fixes"
git -C "$REPO" log "$PREV_VERSION..HEAD" --grep="^fix" --format="- %s" 2>/dev/null | head -5
echo ""

# Contributors
echo "## Contributors"
echo ""
git -C "$REPO" log "$PREV_VERSION..HEAD" --format="%an" 2>/dev/null | sort | uniq -c | sort -rn | \
  awk '{print "- " $2 " (" $1 " commits)"}'
echo ""

# Full changelog link
echo "## Full Changelog"
echo ""
echo "See [$PREV_VERSION...v$VERSION](../../compare/$PREV_VERSION...v$VERSION) for all changes."
```

### Workflow 3: PR Summary Generation

Summarize changes for a pull request:

```bash
#!/bin/bash
# Generate PR summary from branch changes

REPO="/mnt/d/4DA"
BASE="${1:-main}"
HEAD="${2:-HEAD}"

echo "## Pull Request Summary"
echo ""

# Stats
FILES=$(git -C "$REPO" diff --stat "$BASE...$HEAD" | tail -1)
COMMITS=$(git -C "$REPO" rev-list --count "$BASE...$HEAD")

echo "**$COMMITS commits** | $FILES"
echo ""

# Changes by type
echo "### Changes"
echo ""

FEATS=$(git -C "$REPO" log "$BASE...$HEAD" --oneline --grep="^feat" | wc -l)
FIXES=$(git -C "$REPO" log "$BASE...$HEAD" --oneline --grep="^fix" | wc -l)
REFACTORS=$(git -C "$REPO" log "$BASE...$HEAD" --oneline --grep="^refactor" | wc -l)

[ "$FEATS" -gt 0 ] && echo "- ✨ $FEATS new feature(s)"
[ "$FIXES" -gt 0 ] && echo "- 🐛 $FIXES bug fix(es)"
[ "$REFACTORS" -gt 0 ] && echo "- ♻️ $REFACTORS refactor(s)"

echo ""

# Key changes
echo "### Key Changes"
echo ""
git -C "$REPO" log "$BASE...$HEAD" --oneline --format="- %s"
echo ""

# Files changed
echo "### Files Modified"
echo ""
echo "<details>"
echo "<summary>Click to expand ($(git -C "$REPO" diff --name-only "$BASE...$HEAD" | wc -l) files)</summary>"
echo ""
echo '```'
git -C "$REPO" diff --name-only "$BASE...$HEAD"
echo '```'
echo "</details>"
```

### Workflow 4: Migration Guide Generation

Create migration guides for breaking changes:

```bash
#!/bin/bash
# Generate migration guide

REPO="/mnt/d/4DA"
FROM_VERSION="${1:-v1.0.0}"
TO_VERSION="${2:-v2.0.0}"

echo "# Migration Guide: $FROM_VERSION → $TO_VERSION"
echo ""
echo "This guide helps you migrate your project from $FROM_VERSION to $TO_VERSION."
echo ""

# Find breaking changes
echo "## Breaking Changes"
echo ""

git -C "$REPO" log "$FROM_VERSION..$TO_VERSION" --format="%B" 2>/dev/null | \
  grep -B5 -A10 "BREAKING CHANGE:" | while read -r line; do
  echo "$line"
done

# API changes
echo ""
echo "## API Changes"
echo ""

# Find removed functions/types
echo "### Removed"
git -C "$REPO" diff "$FROM_VERSION..$TO_VERSION" -- "*.rs" "*.ts" | \
  grep "^-.*pub fn\|^-.*export function\|^-.*export const" | \
  head -10 | sed 's/^-/- ~~/'

echo ""
echo "### Added"
git -C "$REPO" diff "$FROM_VERSION..$TO_VERSION" -- "*.rs" "*.ts" | \
  grep "^+.*pub fn\|^+.*export function\|^+.*export const" | \
  head -10 | sed 's/^+/- /'

echo ""
echo "### Changed"
echo ""
echo "(Review diff for signature changes)"

# Configuration changes
echo ""
echo "## Configuration Changes"
echo ""

echo "### settings.json"
git -C "$REPO" diff "$FROM_VERSION..$TO_VERSION" -- "data/settings.json" "src-tauri/tauri.conf.json" 2>/dev/null | head -30

# Step-by-step migration
echo ""
echo "## Migration Steps"
echo ""
echo "1. **Backup your data**"
echo '   ```bash'
echo '   cp -r data/ data.backup/'
echo '   ```'
echo ""
echo "2. **Update configuration**"
echo "   Review the configuration changes above and update your settings.json"
echo ""
echo "3. **Update code**"
echo "   If you have custom integrations, review the API changes"
echo ""
echo "4. **Test**"
echo '   ```bash'
echo '   npm run tauri dev'
echo '   ```'
echo ""
echo "5. **Report issues**"
echo "   If you encounter problems, please open an issue"
```

### Workflow 5: Changelog Formatting

Convert to different output formats:

```bash
#!/bin/bash
# Multi-format changelog output

REPO="/mnt/d/4DA"
FORMAT="${1:-markdown}"
VERSION="${2:-$(date +%Y.%m.%d)}"

generate_data() {
  # Generate JSON data structure
  cat << EOF
{
  "version": "$VERSION",
  "date": "$(date +%Y-%m-%d)",
  "features": [
$(git -C "$REPO" log HEAD~20..HEAD --grep="^feat" --format='    {"message": "%s", "hash": "%h"},' | sed '$ s/,$//')
  ],
  "fixes": [
$(git -C "$REPO" log HEAD~20..HEAD --grep="^fix" --format='    {"message": "%s", "hash": "%h"},' | sed '$ s/,$//')
  ],
  "breaking": [
$(git -C "$REPO" log HEAD~20..HEAD --format="%B" | grep -A1 "BREAKING CHANGE:" | grep -v "BREAKING CHANGE:" | grep -v "^--$" | sed 's/.*/    {"description": "&"},/' | sed '$ s/,$//')
  ]
}
EOF
}

case "$FORMAT" in
  json)
    generate_data
    ;;

  markdown)
    echo "# Changelog"
    echo ""
    echo "## [$VERSION] - $(date +%Y-%m-%d)"
    echo ""
    echo "### Features"
    git -C "$REPO" log HEAD~20..HEAD --grep="^feat" --format="- %s"
    echo ""
    echo "### Bug Fixes"
    git -C "$REPO" log HEAD~20..HEAD --grep="^fix" --format="- %s"
    ;;

  html)
    echo "<!DOCTYPE html>"
    echo "<html><head><title>Changelog $VERSION</title></head><body>"
    echo "<h1>Changelog</h1>"
    echo "<h2>Version $VERSION - $(date +%Y-%m-%d)</h2>"
    echo "<h3>Features</h3><ul>"
    git -C "$REPO" log HEAD~20..HEAD --grep="^feat" --format="<li>%s</li>"
    echo "</ul>"
    echo "<h3>Bug Fixes</h3><ul>"
    git -C "$REPO" log HEAD~20..HEAD --grep="^fix" --format="<li>%s</li>"
    echo "</ul>"
    echo "</body></html>"
    ;;

  *)
    echo "Unknown format: $FORMAT"
    echo "Supported: json, markdown, html"
    ;;
esac
```

---

## Output Format

### Standard Changelog (CHANGELOG.md)

```markdown
# Changelog

All notable changes to 4DA will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- MCP server for agent integration
- Topic clustering in digests
- Relevance explanations

### Changed
- Improved embedding similarity scoring
- Faster file scanning with debouncing

### Deprecated
- Legacy ACE modules (moved to _future/)

### Removed
- Unused Tauri commands (17 removed)

### Fixed
- Race condition in file watcher
- Memory leak in scanner

### Security
- Updated dependencies for CVE-2026-XXXX

---

## [1.0.0] - 2026-01-15

### Added
- Initial release
- Hacker News source adapter
- arXiv source adapter
- Basic relevance scoring
- System tray integration

### Breaking Changes
- Requires Tauri 2.0 (not compatible with 1.x)

---

## [0.9.0] - 2026-01-01

### Added
- Beta release for testing
```

### Release Announcement

```markdown
# 🎉 4DA v2.0.0 Released!

We're excited to announce the release of 4DA v2.0.0, bringing significant improvements to your signal-to-noise ratio.

## What's New

### 🤖 MCP Server Integration

4DA now exposes an MCP server that allows AI agents (like Claude Code) to:
- Query your relevant content
- Understand your context
- Provide feedback for learning

### 📊 Smarter Digests

Digests now include:
- Topic clustering (related items grouped together)
- Relevance explanations ("why this matters")
- Optional LLM-powered summaries

### ⚡ Performance Improvements

- 40% faster file scanning
- Reduced memory usage
- Better embedding caching

## Breaking Changes

⚠️ **Database Migration Required**

The database schema has changed. Run the migration:

```bash
./migrate.sh v1-to-v2
```

## Upgrade Guide

1. Backup your data directory
2. Download v2.0.0
3. Run the migration script
4. Review your settings (new options available)

## Thank You

Thanks to everyone who provided feedback during the beta!

---

[Download v2.0.0](releases/v2.0.0) | [Full Changelog](CHANGELOG.md) | [Migration Guide](docs/migration-v2.md)
```

---

## Commit Analysis

### Parsing Commits

```bash
#!/bin/bash
# Parse and analyze commits

REPO="/mnt/d/4DA"

echo "=== Commit Analysis ==="

# Commit type distribution
echo ""
echo "### Commit Types (last 100)"
git -C "$REPO" log -100 --oneline | \
  grep -oE "^[a-f0-9]+ (feat|fix|perf|refactor|docs|test|chore|build|ci)" | \
  cut -d' ' -f2 | sort | uniq -c | sort -rn

# Scope distribution
echo ""
echo "### Scopes (last 100)"
git -C "$REPO" log -100 --oneline | \
  grep -oE "\([a-z-]+\):" | tr -d '():' | sort | uniq -c | sort -rn

# Commit frequency
echo ""
echo "### Commit Frequency (by day of week)"
git -C "$REPO" log --format="%ad" --date=format:"%A" | sort | uniq -c | sort -rn

# Most active files
echo ""
echo "### Most Changed Files (last 30 days)"
git -C "$REPO" log --since="30 days ago" --name-only --format="" | \
  sort | uniq -c | sort -rn | head -10
```

---

## Constraints

**CAN:**
- Read git history
- Parse commit messages
- Generate formatted output
- Detect breaking changes
- Create migration guides

**MUST:**
- Follow conventional commit format
- Highlight breaking changes prominently
- Include version and date
- Link to full diffs

**CANNOT:**
- Modify git history
- Create commits
- Tag releases (without approval)
- Fabricate changes

---

*The Changelog Writer turns git noise into release music. Every commit tells part of the story.*
