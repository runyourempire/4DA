#!/bin/bash
# Pre-compact backup script
# Runs before context compaction to preserve transcript
# NO set -e — hooks must never fail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKUP_DIR="$SCRIPT_DIR/../backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Create backup directory if needed
mkdir -p "$BACKUP_DIR" 2>/dev/null || true

# Read hook input from stdin (JSON with spaces after colons)
INPUT=$(cat 2>/dev/null || true)

# Parse JSON — handle both "key":"val" and "key": "val" formats
TRANSCRIPT_PATH=$(echo "$INPUT" | grep -oP '"transcript_path"\s*:\s*"\K[^"]+' 2>/dev/null || echo "")
TRIGGER=$(echo "$INPUT" | grep -oP '"trigger"\s*:\s*"\K[^"]+' 2>/dev/null || echo "unknown")

# Log the compaction event
echo "[$(date)] Compaction triggered: $TRIGGER" >> "$BACKUP_DIR/compaction.log" 2>/dev/null || true

# If we have a transcript path, back it up (skip if > 50MB)
if [ -n "$TRANSCRIPT_PATH" ] && [ -f "$TRANSCRIPT_PATH" ]; then
    FILE_BYTES=$(stat -c%s "$TRANSCRIPT_PATH" 2>/dev/null || stat -f%z "$TRANSCRIPT_PATH" 2>/dev/null || echo "0")
    if [ "$FILE_BYTES" -le 52428800 ] 2>/dev/null; then
        BACKUP_FILE="$BACKUP_DIR/transcript_${TIMESTAMP}.jsonl"
        cp "$TRANSCRIPT_PATH" "$BACKUP_FILE" 2>/dev/null || true
        echo "[$(date)] Backed up transcript to: $BACKUP_FILE" >> "$BACKUP_DIR/compaction.log" 2>/dev/null || true
    fi

    # Keep only last 10 backups to save space
    ls -t "$BACKUP_DIR"/transcript_*.jsonl 2>/dev/null | tail -n +11 | xargs rm -f 2>/dev/null || true
fi

echo '{"status": "success", "message": "Pre-compact backup completed"}'
