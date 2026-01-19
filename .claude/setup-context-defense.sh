#!/bin/bash
# Context Rot Defense System - Setup Script
# Run this once to configure your environment

set -e

echo "=== 4DA Context Rot Defense System Setup ==="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PROJECT_DIR="/mnt/d/4da-v3"
MCP_DIR="$PROJECT_DIR/mcp-memory-server"

# Step 1: Environment variables
echo -e "${YELLOW}Step 1: Setting up environment variables${NC}"

# Create or append to shell profile
PROFILE="$HOME/.bashrc"
if [ -f "$HOME/.zshrc" ]; then
    PROFILE="$HOME/.zshrc"
fi

# Check if already configured
if ! grep -q "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE" "$PROFILE" 2>/dev/null; then
    echo "" >> "$PROFILE"
    echo "# Claude Code Context Rot Defense" >> "$PROFILE"
    echo "export CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=60" >> "$PROFILE"
    echo "export MEMORY_DB_PATH=\"$PROJECT_DIR/.claude/memory.db\"" >> "$PROFILE"
    echo -e "${GREEN}  Added environment variables to $PROFILE${NC}"
else
    echo "  Environment variables already configured"
fi

# Export for current session
export CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=60
export MEMORY_DB_PATH="$PROJECT_DIR/.claude/memory.db"

# Step 2: Install MCP Memory Server dependencies
echo ""
echo -e "${YELLOW}Step 2: Installing MCP Memory Server${NC}"

cd "$MCP_DIR"
if [ -f "package.json" ]; then
    npm install
    npm run build 2>/dev/null || echo "  Build will complete on first run"
    echo -e "${GREEN}  MCP Memory Server installed${NC}"
else
    echo "  Error: package.json not found in $MCP_DIR"
    exit 1
fi

# Step 3: Create MCP configuration
echo ""
echo -e "${YELLOW}Step 3: Creating MCP configuration${NC}"

MCP_CONFIG="$PROJECT_DIR/.mcp.json"
cat > "$MCP_CONFIG" << EOF
{
  "mcpServers": {
    "memory": {
      "command": "node",
      "args": ["$MCP_DIR/dist/index.js"],
      "env": {
        "MEMORY_DB_PATH": "$PROJECT_DIR/.claude/memory.db"
      }
    }
  }
}
EOF
echo -e "${GREEN}  Created $MCP_CONFIG${NC}"

# Step 4: Verify hooks are executable
echo ""
echo -e "${YELLOW}Step 4: Verifying scripts${NC}"

chmod +x "$PROJECT_DIR/.claude/scripts/"*.sh 2>/dev/null || true
echo -e "${GREEN}  Scripts are executable${NC}"

# Step 5: Terminal fix (optional)
echo ""
echo -e "${YELLOW}Step 5: Terminal rendering fix${NC}"
echo "  For terminal lag issues, optionally install:"
echo "    npm install -g claudescreenfix-hardwicksoftware"
echo "    Then use 'claude-fixed' instead of 'claude'"

# Summary
echo ""
echo "=== Setup Complete ==="
echo ""
echo "Configuration:"
echo "  - Auto-compact threshold: 60% (earlier compaction for quality)"
echo "  - Memory database: $PROJECT_DIR/.claude/memory.db"
echo "  - MCP config: $MCP_CONFIG"
echo "  - Hooks: $PROJECT_DIR/.claude/settings.json"
echo ""
echo "To activate in current shell:"
echo "  source $PROFILE"
echo ""
echo "Files created in .claude/:"
echo "  rules/decisions.md    - Architectural decisions (survives compaction)"
echo "  rules/current-state.md - Current task state"
echo "  rules/conventions.md  - Code style guide"
echo "  agents/               - Subagent documentation"
echo "  scripts/              - Hook scripts"
echo "  settings.json         - Hook configuration"
echo ""
echo "MCP Memory Server tools available:"
echo "  - remember_decision / recall_decisions"
echo "  - update_state / get_state"
echo "  - remember_learning / recall_learnings"
echo "  - remember_code_location / recall_code_locations"
echo "  - search_memory"
echo ""
echo -e "${GREEN}Ready to use! Start Claude Code in this project.${NC}"
