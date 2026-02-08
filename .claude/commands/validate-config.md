# /validate-config

Validate all 4DA configuration files and settings.

## Usage

```
/validate-config               # Full validation
/validate-config --security    # Security audit only
/validate-config --fix         # Show fix commands
```

## What This Does

This command invokes the **4da-config-validator** agent to check configuration health:

1. **Validates JSON syntax** in all config files
2. **Checks required fields** against schema
3. **Verifies paths exist** (watched directories)
4. **Security audit** (API keys, permissions, dangerous paths)
5. **Cross-file consistency** (DB paths match, dimensions align)
6. **Generates fix commands** for issues found

## Files Checked

- `/mnt/d/4da-v3/data/settings.json` - Main settings
- `/mnt/d/4da-v3/.mcp.json` - MCP server config
- `/mnt/d/4da-v3/src-tauri/tauri.conf.json` - Tauri config
- `/mnt/d/4da-v3/.claude/settings.json` - Claude Code settings

## Example Output

```
## Configuration Validation Report

**Status:** ⚠️ 2 Warnings, 1 Error

### Files Checked
| File | Status | Issues |
|------|--------|--------|
| settings.json | ⚠️ Warning | API key in file |
| .mcp.json | ✓ Valid | - |
| tauri.conf.json | ✓ Valid | - |

### Critical Issues

1. **ERROR: Watched directory does not exist**
   - Path: `/home/user/projects/old-project`
   - Fix: `jq 'del(.ace.watched_directories[] | select(. == "/home/user/projects/old-project"))' settings.json`

### Warnings

1. **API key stored in settings.json**
   - Risk: Credential exposure if file shared
   - Fix: Use environment variable `OPENAI_API_KEY` instead

2. **High daily API limit ($50)**
   - Risk: Unexpected costs
   - Recommendation: Start with $5-10

### Security Audit
| Check | Status |
|-------|--------|
| No hardcoded credentials | ⚠️ Found |
| Safe watched paths | ✓ Pass |
| File permissions | ✓ Pass |

### Recommendations
1. Move API keys to environment variables
2. Remove non-existent watched directory
3. Reduce daily limit to $10
```

## Agent Reference

Full agent definition: `.claude/agents/4da-config-validator.md`
