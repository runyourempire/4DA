#!/bin/bash
# Prune session transcripts older than 30 days
# Run manually or add to a scheduled task
# Usage: bash .claude/scripts/prune-sessions.sh [days]

DAYS="${1:-30}"
SESSION_DIR="$(git rev-parse --show-toplevel 2>/dev/null)/.claude/sessions/transcripts"
BACKUP_DIR="$(git rev-parse --show-toplevel 2>/dev/null)/.claude/backups"

if [ ! -d "$SESSION_DIR" ]; then
    echo "No session directory found at $SESSION_DIR"
    exit 0
fi

# Count before
before=$(find "$SESSION_DIR" -name "*.jsonl" -type f | wc -l | tr -d ' ')

# Find and remove old files
deleted=$(find "$SESSION_DIR" -name "*.jsonl" -type f -mtime +"$DAYS" | wc -l | tr -d ' ')
find "$SESSION_DIR" -name "*.jsonl" -type f -mtime +"$DAYS" -delete 2>/dev/null

# Also prune old backups
backup_deleted=0
if [ -d "$BACKUP_DIR" ]; then
    backup_deleted=$(find "$BACKUP_DIR" -name "*.jsonl" -type f -mtime +"$DAYS" | wc -l | tr -d ' ')
    find "$BACKUP_DIR" -name "*.jsonl" -type f -mtime +"$DAYS" -delete 2>/dev/null
fi

after=$(find "$SESSION_DIR" -name "*.jsonl" -type f | wc -l | tr -d ' ')

echo "Session pruning complete:"
echo "  Transcripts: ${before} -> ${after} (removed ${deleted})"
echo "  Backups removed: ${backup_deleted}"
echo "  Threshold: ${DAYS} days"
