#!/bin/bash
# Git hygiene monitor — warns when uncommitted changes accumulate
# Runs on UserPromptSubmit to catch buildup early

cd "$(git rev-parse --show-toplevel 2>/dev/null)" || exit 0

# Count changed files (modified + untracked, excluding .claude/)
file_count=$(git status --porcelain 2>/dev/null | grep -v '^\?\? \.claude/' | wc -l | tr -d ' ')

# Count net lines changed (modified tracked files only)
line_stat=$(git diff --shortstat 2>/dev/null)
insertions=$(echo "$line_stat" | grep -oP '\d+(?= insertion)' || echo "0")
deletions=$(echo "$line_stat" | grep -oP '\d+(?= deletion)' || echo "0")
total_lines=$(( ${insertions:-0} + ${deletions:-0} ))

# Tiered warnings
if [ "$file_count" -ge 20 ]; then
    echo "GIT HYGIENE WARNING: ${file_count} uncommitted files and ~${total_lines} lines changed. This is getting risky — a single bad operation could lose all of this. Strongly recommend running /commit-organize now."
elif [ "$file_count" -ge 12 ]; then
    echo "GIT HYGIENE NOTICE: ${file_count} uncommitted files building up (~${total_lines} lines). Consider running /commit-organize to checkpoint your work."
elif [ "$file_count" -ge 8 ] && [ "$total_lines" -ge 300 ]; then
    echo "GIT HYGIENE NOTE: ${file_count} files with ~${total_lines} lines uncommitted. /commit-organize is available when you're ready to checkpoint."
fi
