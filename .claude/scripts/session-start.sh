#!/bin/bash
# Session start hook - Initialize session tracking
set -e

SESSIONS_DIR="/mnt/d/4da-v3/.claude/sessions"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Read hook input
INPUT=$(cat)
SESSION_ID=$(echo "$INPUT" | grep -o '"session_id":"[^"]*"' | cut -d'"' -f4 || echo "unknown")

# Create session metadata file
SESSION_FILE="$SESSIONS_DIR/transcripts/session_${TIMESTAMP}_${SESSION_ID}.meta"
cat > "$SESSION_FILE" << EOF
{
  "session_id": "$SESSION_ID",
  "started_at": "$(date -Iseconds)",
  "project": "4da-v3",
  "status": "active"
}
EOF

# Log session start
echo "[$(date)] Session started: $SESSION_ID" >> "$SESSIONS_DIR/sessions.log"

echo '{"status": "success"}'
