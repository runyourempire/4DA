#!/bin/bash
# Session stop hook - Archive complete session transcript
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SESSIONS_DIR="$SCRIPT_DIR/../sessions"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Read hook input
INPUT=$(cat)
SESSION_ID=$(echo "$INPUT" | grep -o '"session_id":"[^"]*"' | cut -d'"' -f4 || echo "unknown")
TRANSCRIPT_PATH=$(echo "$INPUT" | grep -o '"transcript_path":"[^"]*"' | cut -d'"' -f4 || echo "")

# Ensure transcripts directory exists
mkdir -p "$SESSIONS_DIR/transcripts"

# Archive transcript if it exists
if [ -n "$TRANSCRIPT_PATH" ] && [ -f "$TRANSCRIPT_PATH" ]; then
    ARCHIVE_FILE="$SESSIONS_DIR/transcripts/session_${TIMESTAMP}_${SESSION_ID}.jsonl"
    cp "$TRANSCRIPT_PATH" "$ARCHIVE_FILE"

    # Calculate stats
    LINE_COUNT=$(wc -l < "$ARCHIVE_FILE")
    FILE_SIZE=$(du -h "$ARCHIVE_FILE" | cut -f1)

    # Update metadata
    META_FILE=$(ls -t "$SESSIONS_DIR/transcripts/session_"*"_${SESSION_ID}.meta" 2>/dev/null | head -1)
    if [ -n "$META_FILE" ] && [ -f "$META_FILE" ]; then
        # Create updated metadata
        cat > "$META_FILE" << EOF
{
  "session_id": "$SESSION_ID",
  "started_at": "$(grep -o '"started_at":"[^"]*"' "$META_FILE" | cut -d'"' -f4 || echo "unknown")",
  "ended_at": "$(date -Iseconds)",
  "project": "4DA",
  "status": "completed",
  "transcript_file": "$(basename "$ARCHIVE_FILE")",
  "message_count": $LINE_COUNT,
  "file_size": "$FILE_SIZE"
}
EOF
    fi

    echo "[$(date)] Session archived: $SESSION_ID ($LINE_COUNT messages, $FILE_SIZE)" >> "$SESSIONS_DIR/sessions.log"
fi

echo '{"status": "success"}'
