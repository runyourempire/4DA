#!/bin/bash
# Manual transcript save - call anytime to save current session state
# Usage: ./.claude/scripts/save-transcript-now.sh [optional-note]
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SESSIONS_DIR="$SCRIPT_DIR/../sessions"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
NOTE="${1:-manual-save}"

# Find most recent transcript in Claude's session directory
# Claude Code stores transcripts in ~/.claude/projects/{hash}/{session-id}.jsonl
CLAUDE_SESSIONS_DIR="$HOME/.claude/projects"

if [ -d "$CLAUDE_SESSIONS_DIR" ]; then
    # Find most recently modified jsonl file
    LATEST_TRANSCRIPT=$(find "$CLAUDE_SESSIONS_DIR" -name "*.jsonl" -type f -mmin -60 2>/dev/null | head -1)

    if [ -n "$LATEST_TRANSCRIPT" ] && [ -f "$LATEST_TRANSCRIPT" ]; then
        ARCHIVE_FILE="$SESSIONS_DIR/transcripts/snapshot_${TIMESTAMP}_${NOTE}.jsonl"
        cp "$LATEST_TRANSCRIPT" "$ARCHIVE_FILE"

        LINE_COUNT=$(wc -l < "$ARCHIVE_FILE")
        echo "Saved snapshot: $ARCHIVE_FILE ($LINE_COUNT messages)"
        echo "[$(date)] Snapshot saved: $NOTE ($LINE_COUNT messages)" >> "$SESSIONS_DIR/sessions.log"
    else
        echo "No active transcript found"
    fi
else
    echo "Claude sessions directory not found"
fi
