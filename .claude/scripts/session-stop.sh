#!/bin/bash
# Session stop hook - Archive complete session transcript
# NO set -e — hooks must never fail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SESSIONS_DIR="$SCRIPT_DIR/../sessions"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Read hook input (JSON with spaces after colons)
INPUT=$(cat 2>/dev/null || true)

# Parse JSON fields — handle both "key":"val" and "key": "val" formats
SESSION_ID=$(echo "$INPUT" | grep -oP '"session_id"\s*:\s*"\K[^"]+' 2>/dev/null || echo "unknown")
TRANSCRIPT_PATH=$(echo "$INPUT" | grep -oP '"transcript_path"\s*:\s*"\K[^"]+' 2>/dev/null || echo "")

# Ensure transcripts directory exists
mkdir -p "$SESSIONS_DIR/transcripts" 2>/dev/null || true

# Archive transcript if it exists and hasn't been archived recently
if [ -n "$TRANSCRIPT_PATH" ] && [ -f "$TRANSCRIPT_PATH" ]; then
    # Dedup: skip if this session was archived in the last 5 minutes
    RECENT=$(find "$SESSIONS_DIR/transcripts" -name "session_*_${SESSION_ID}.jsonl" -newermt '5 minutes ago' 2>/dev/null | head -1)
    if [ -n "$RECENT" ]; then
        echo '{"status": "success"}'
        exit 0
    fi

    # Skip transcripts larger than 50MB to avoid slow copies
    FILE_BYTES=$(stat -c%s "$TRANSCRIPT_PATH" 2>/dev/null || stat -f%z "$TRANSCRIPT_PATH" 2>/dev/null || echo "0")
    if [ "$FILE_BYTES" -gt 52428800 ] 2>/dev/null; then
        echo "[$(date)] Skipped archive: $SESSION_ID (${FILE_BYTES} bytes > 50MB limit)" >> "$SESSIONS_DIR/sessions.log" 2>/dev/null || true
        echo '{"status": "success"}'
        exit 0
    fi

    ARCHIVE_FILE="$SESSIONS_DIR/transcripts/session_${TIMESTAMP}_${SESSION_ID}.jsonl"
    cp "$TRANSCRIPT_PATH" "$ARCHIVE_FILE" 2>/dev/null || true

    if [ -f "$ARCHIVE_FILE" ]; then
        LINE_COUNT=$(wc -l < "$ARCHIVE_FILE" 2>/dev/null || echo "0")
        FILE_SIZE=$(du -h "$ARCHIVE_FILE" 2>/dev/null | cut -f1 || echo "?")
        echo "[$(date)] Session archived: $SESSION_ID ($LINE_COUNT messages, $FILE_SIZE)" >> "$SESSIONS_DIR/sessions.log" 2>/dev/null || true
    fi

    # Prune: keep only last 20 archives to cap disk usage
    ls -t "$SESSIONS_DIR/transcripts"/session_*.jsonl 2>/dev/null | tail -n +21 | xargs rm -f 2>/dev/null || true
fi

echo '{"status": "success"}'
