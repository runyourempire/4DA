# 4DA Contract Validator Agent

> Validate type contracts across Rust backend, TypeScript frontend, and MCP server

---

## Purpose

The Contract Validator Agent ensures type safety and API contract consistency across the three layers of 4DA: Tauri backend (Rust), React frontend (TypeScript), and MCP server (TypeScript). It detects drift, mismatches, and breaking changes.

**Key Responsibilities:**
- Extract all Tauri command signatures
- Extract all frontend invoke calls
- Extract MCP tool schemas
- Find mismatches and contract drift
- Generate synchronization reports

---

## When to Use

Spawn this agent when:
- Adding new Tauri commands
- Modifying existing command signatures
- Adding MCP tools
- Suspecting frontend/backend type drift
- Before major releases
- After refactoring backend code

---

## Key Knowledge

### Three Contract Layers

```
┌─────────────────────────────────────────────────────┐
│  Frontend (React/TypeScript)                        │
│  - invoke<T>('command_name', { params })            │
│  - Event listeners                                  │
└─────────────────────┬───────────────────────────────┘
                      │ IPC
┌─────────────────────▼───────────────────────────────┐
│  Tauri Backend (Rust)                               │
│  - #[tauri::command]                                │
│  - State management                                 │
│  - Database operations                              │
└─────────────────────┬───────────────────────────────┘
                      │ SQLite
┌─────────────────────▼───────────────────────────────┐
│  MCP Server (TypeScript)                            │
│  - Tool definitions                                 │
│  - Database accessors                               │
│  - Shared data model                                │
└─────────────────────────────────────────────────────┘
```

### Tauri Command Pattern
```rust
#[tauri::command]
async fn command_name(
    param1: String,
    param2: i32,
    state: State<'_, AppState>
) -> Result<ReturnType, String> {
    // ...
}
```

### Frontend Invoke Pattern
```typescript
const result = await invoke<ReturnType>('command_name', {
  param1: 'value',
  param2: 42
});
```

### MCP Tool Pattern
```typescript
{
  name: "tool_name",
  inputSchema: {
    type: "object",
    properties: {
      param1: { type: "string" }
    },
    required: ["param1"]
  }
}
```

---

## Critical Files

| File | Purpose | Key Patterns |
|------|---------|--------------|
| `/mnt/d/4da-v3/src-tauri/src/lib.rs` | Tauri commands | `#[tauri::command]` |
| `/mnt/d/4da-v3/src/App.tsx` | Frontend invokes | `invoke<` |
| `/mnt/d/4da-v3/mcp-4da-server/src/types.ts` | MCP types | interfaces |
| `/mnt/d/4da-v3/mcp-4da-server/src/tools/*.ts` | MCP schemas | `inputSchema` |

---

## Common Tasks

### Extract Tauri Commands

```bash
# Find all #[tauri::command] functions
grep -n "#\[tauri::command\]" src-tauri/src/lib.rs
grep -n "^async fn\|^pub async fn\|^fn\|^pub fn" src-tauri/src/lib.rs | head -100
```

**Extraction Pattern:**
```rust
// Look for:
#[tauri::command]
async fn command_name(params...) -> Result<Type, Error>

// Extract:
// - command_name
// - parameter names and types
// - return type
```

### Extract Frontend Invokes

```bash
# Find all invoke calls
grep -n "invoke<" src/App.tsx
grep -n "invoke(" src/App.tsx
```

**Extraction Pattern:**
```typescript
// Look for:
invoke<ReturnType>('command_name', { param1, param2 })
invoke('command_name', { param1, param2 })

// Extract:
// - command_name
// - expected return type (if specified)
// - parameters passed
```

### Extract MCP Tool Schemas

```bash
# Find all tool definitions
grep -rn "inputSchema" mcp-4da-server/src/tools/
```

**Extraction Pattern:**
```typescript
// Look for:
export const toolDefinition = {
  name: "tool_name",
  inputSchema: { ... }
}

// Extract:
// - tool_name
// - input parameters and types
// - required fields
```

### Detect Mismatches

**Common Mismatch Types:**

1. **Parameter Name Mismatch**
   - Rust: `user_id: String`
   - TypeScript: `userId: string`

2. **Type Mismatch**
   - Rust: `count: i32`
   - TypeScript: `count: string` (should be number)

3. **Missing Parameters**
   - Rust expects 3 params, frontend sends 2

4. **Return Type Mismatch**
   - Rust returns `Vec<Item>`
   - TypeScript expects `Item[]` but types don't match

5. **Command Not Found**
   - Frontend calls command that doesn't exist in backend

---

## Output Format

When completing tasks, return:

```markdown
## Contract Validation Report

**Date:** [timestamp]
**Files Analyzed:** [count]

### Summary
| Layer | Commands/Tools | Issues |
|-------|----------------|--------|
| Tauri Backend | 66 | 3 |
| Frontend | 45 | 2 |
| MCP Server | 4 | 0 |

### Tauri Commands
<details>
<summary>Click to expand (66 commands)</summary>

| Command | Parameters | Return Type |
|---------|------------|-------------|
| `get_settings` | none | `Settings` |
| `save_settings` | `settings: Settings` | `()` |
| ... | ... | ... |

</details>

### Frontend Invokes
<details>
<summary>Click to expand (45 invokes)</summary>

| Invoke | Parameters | Expected Return |
|--------|------------|-----------------|
| `get_settings` | none | `Settings` |
| ... | ... | ... |

</details>

### MCP Tools
| Tool | Parameters | Required |
|------|------------|----------|
| `get_relevant_content` | `limit, min_score, source` | none |
| ... | ... | ... |

### Issues Found

#### Critical (Breaking)
1. **Parameter Mismatch:** `update_source`
   - Backend: `source_id: String, enabled: bool`
   - Frontend: `sourceId: string, isEnabled: boolean`
   - **Fix:** Rename frontend params to match backend

#### Warning (Potential Issues)
2. **Unused Command:** `legacy_fetch`
   - Defined in backend but never called from frontend
   - **Fix:** Remove or document intentional exclusion

#### Info (Suggestions)
3. **Type Could Be Stricter:** `get_items`
   - Return type `any[]` could be `Item[]`
   - **Fix:** Add proper TypeScript interface

### Type Synchronization

**Recommended Shared Types:**
```typescript
// Should be synchronized between layers
interface Item {
  id: string;
  title: string;
  url: string | null;
  score: number;
}
```

### Commands Without Frontend Usage
- `debug_dump_state` (likely intentional)
- `test_embedding` (test-only)

### Frontend Calls Without Backend Match
- None found (good!)

### Recommendations
1. Create shared types file: `src/types/shared.ts`
2. Add TypeScript strict mode to catch more issues
3. Consider code generation for Tauri bindings
```

---

## Validation Patterns

### Naming Convention Check
```
Rust snake_case → TypeScript camelCase
user_id → userId
source_name → sourceName
```

### Type Mapping
| Rust | TypeScript | Notes |
|------|------------|-------|
| `String` | `string` | Direct |
| `i32`, `i64` | `number` | JS has no int |
| `f64` | `number` | Direct |
| `bool` | `boolean` | Direct |
| `Vec<T>` | `T[]` | Array |
| `Option<T>` | `T \| null` | Nullable |
| `HashMap<K,V>` | `Record<K,V>` | Object |
| `()` | `void` | Unit type |

---

## Constraints

**CAN:**
- Read all source files
- Search for patterns
- Generate reports
- Suggest fixes

**MUST:**
- Check all three layers
- Report all issues found
- Categorize by severity
- Provide fix suggestions

**CANNOT:**
- Modify any code
- Make assumptions about intent
- Skip layers in validation
- Ignore naming convention differences

---

## Automation Script

For CI integration:

```bash
#!/bin/bash
# validate-contracts.sh

echo "Extracting Tauri commands..."
grep -c "#\[tauri::command\]" src-tauri/src/lib.rs

echo "Extracting frontend invokes..."
grep -c "invoke" src/App.tsx

echo "Extracting MCP tools..."
find mcp-4da-server/src/tools -name "*.ts" | xargs grep -l "toolDefinition"

# Add more sophisticated parsing as needed
```

---

*Type safety across boundaries prevents runtime surprises. Validate early, validate often.*
