#!/bin/bash
# Pre-compact backup script
# Runs before context compaction to preserve transcript

set -e

BACKUP_DIR="/mnt/d/4DA/.claude/backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Create backup directory if needed
mkdir -p "$BACKUP_DIR"

# Read hook input from stdin (JSON)
INPUT=$(cat)

# Extract transcript path from hook input
TRANSCRIPT_PATH=$(echo "$INPUT" | grep -o '"transcript_path":"[^"]*"' | cut -d'"' -f4)
TRIGGER=$(echo "$INPUT" | grep -o '"trigger":"[^"]*"' | cut -d'"' -f4)

# Log the compaction event
echo "[$(date)] Compaction triggered: $TRIGGER" >> "$BACKUP_DIR/compaction.log"

# If we have a transcript path, back it up
if [ -n "$TRANSCRIPT_PATH" ] && [ -f "$TRANSCRIPT_PATH" ]; then
    BACKUP_FILE="$BACKUP_DIR/transcript_${TIMESTAMP}.jsonl"
    cp "$TRANSCRIPT_PATH" "$BACKUP_FILE"
    echo "[$(date)] Backed up transcript to: $BACKUP_FILE" >> "$BACKUP_DIR/compaction.log"

    # Keep only last 10 backups to save space
    ls -t "$BACKUP_DIR"/transcript_*.jsonl 2>/dev/null | tail -n +11 | xargs -r rm
fi

# Update current-state.md with compaction notice
STATE_FILE="/mnt/d/4DA/.claude/rules/current-state.md"
if [ -f "$STATE_FILE" ]; then
    # Add compaction marker
    sed -i "s/\*Last updated:.*/\*Last updated: Compaction at $TIMESTAMP ($TRIGGER)\*/" "$STATE_FILE"
fi

echo '{"status": "success", "message": "Pre-compact backup completed"}'
