# Claude Code Instructions for 4DA v3

## Project Identity

**4DA = 4 Dimensional Autonomy**
**Tagline:** The internet searches for you.
**Dev Server:** localhost:4444

An ambient intelligence layer that monitors your local context, watches external sources continuously, filters ruthlessly (99.9% rejection), and delivers only what matters - before you know you need it.

---

## Quick Start

```bash
cd /mnt/d/4da-v3
# TODO: Add setup commands after scaffolding
```

---

## Architecture

Read `specs/ARCHITECTURE.md` for the complete stone tablet specification.

### Core Components
1. **Context Membrane** - File indexer + Activity tracker + Semantic memory
2. **World Scanner** - External source monitoring (HN, arXiv, RSS, GitHub)
3. **Relevance Judge** - Multi-stage filtering (embedding → LLM → feedback)
4. **Delivery Engine** - Notifications, digests, ambient feed
5. **Learning Engine** - Implicit/explicit feedback → model updates

---

## Tech Stack

```
Application:  Tauri 2.0 (Rust + WebView)
Frontend:     React 18 + TypeScript + Tailwind CSS
Database:     SQLite 3.45+ with sqlite-vss
Embeddings:   OpenAI text-embedding-3-small (BYOK) or Ollama (local)
LLM:          Anthropic Claude (BYOK) or Ollama (local)
```

---

## Design System: Matte Black Minimalism

### Colors
```css
--bg-primary: #0A0A0A;      /* Matte black background */
--bg-secondary: #141414;     /* Card/panel background */
--bg-tertiary: #1F1F1F;      /* Hover states */
--text-primary: #FFFFFF;     /* Main text */
--text-secondary: #A0A0A0;   /* Secondary text */
--text-muted: #666666;       /* Disabled/hint text */
--accent-primary: #FFFFFF;   /* Primary accent */
--accent-gold: #D4AF37;      /* Highlight (sparingly) */
--border: #2A2A2A;           /* Subtle borders */
--success: #22C55E;          /* Positive feedback */
--error: #EF4444;            /* Errors */
```

### Typography
- **UI Font:** Inter
- **Code Font:** JetBrains Mono
- **Weights:** 400 (body), 500 (emphasis), 600 (headings)

### Principles
1. **Minimal** - Every element earns its place
2. **Spacious** - Generous whitespace
3. **Quiet** - No visual noise
4. **Responsive** - Adapts to window size
5. **Accessible** - WCAG AA contrast ratios

---

## Development Phases

### Phase 0: POC (Current)
- [ ] Tauri app skeleton
- [ ] Single directory file indexing
- [ ] Hacker News source adapter
- [ ] Basic embedding similarity
- [ ] Console output of relevant items

### Phase 1: Core Loop
- [ ] Multiple source adapters
- [ ] LLM relevance scoring
- [ ] System notifications
- [ ] Settings UI
- [ ] BYOK management

### Phase 2: Learning
- [ ] Feedback collection
- [ ] Interest model updates
- [ ] Activity tracking
- [ ] Temporal context

### Phase 3: Polish
- [ ] Email digests
- [ ] Onboarding
- [ ] Auto-updates
- [ ] Documentation

---

## Project Structure

```
4da-v3/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── indexer/     # File indexing
│   │   ├── scanner/     # World scanning
│   │   ├── judge/       # Relevance scoring
│   │   ├── delivery/    # Notifications
│   │   └── db/          # SQLite + vss
│   └── Cargo.toml
├── src/                 # React frontend
│   ├── components/
│   ├── hooks/
│   ├── stores/
│   └── App.tsx
├── specs/               # Design documents
├── docs/                # User documentation
└── .claude/             # Handoff files
```

---

## Code Conventions

### Rust
- Use `thiserror` for error types
- Async with `tokio`
- Log with `tracing`
- Test with `#[cfg(test)]`

### TypeScript
- Strict mode enabled
- React components: `PascalCase.tsx`
- Hooks: `use-kebab-case.ts`
- Prefer `const` and arrow functions

### Naming
- Files: `kebab-case`
- Types: `PascalCase`
- Functions: `camelCase` (TS), `snake_case` (Rust)
- Constants: `SCREAMING_SNAKE_CASE`

---

## Key Principles

1. **Privacy First** - Raw data never leaves the machine
2. **BYOK** - User provides API keys, we never store them remotely
3. **Local First** - Works offline with Ollama fallback
4. **Minimal** - No feature bloat, every element earns its place
5. **Cost Conscious** - Hard daily limits, transparent costs

---

## Commands

```bash
# Development
npm run tauri dev        # Start dev server

# Build
npm run tauri build      # Production build

# Testing
cargo test               # Rust tests
npm test                 # Frontend tests
```

---

## Important Notes

1. **This is v3** - Fresh start, no legacy baggage
2. **Tauri, not Electron** - 10x smaller, 5x faster
3. **SQLite + sqlite-vss** - No external database needed
4. **BYOK is core** - Users own their API keys
5. **91% confidence** - Remaining 9% resolved through iteration

---

## CADE System (CRITICAL - READ FIRST)

This project uses **Cognition-Aware Development Environment (CADE)** to ensure high-quality engineering.

### Truth Source: `.ai/` Directory

Before ANY task, read these files:

| File | Purpose | When to Read |
|------|---------|--------------|
| `.ai/AI_ENGINEERING_CONTRACT.md` | Behavioral rules | Every session |
| `.ai/INVARIANTS.md` | What must ALWAYS/NEVER happen | Before modifications |
| `.ai/ARCHITECTURE.md` | System structure | When touching architecture |
| `.ai/DECISIONS.md` | Why things are this way | Before proposing changes |
| `.ai/FAILURE_MODES.md` | Known fragile areas | Before touching risky code |
| `.ai/TASK_TEMPLATE.md` | How to specify tasks | When receiving tasks |
| `.ai/VALIDATION_CHECKLIST.md` | Completion requirements | Before claiming done |

### Two-Phase Protocol (MANDATORY)

**Phase 1: Orientation (NO CODE)**
1. Read relevant `.ai/` files
2. State the goal explicitly
3. List files to modify
4. Identify relevant invariants
5. Propose approach
6. **Wait for approval**

**Phase 2: Execution (CODE ONLY)**
1. Implement within approved scope
2. Run validation (tests, builds, lints)
3. Verify invariants hold
4. Produce validation report

### Validation Commands

```bash
# Full validation
npm run validate:all

# Individual checks
cargo build && cargo test && cargo fmt --check && cargo clippy
npm run build && npm run test && npm run lint && npm run typecheck
```

---

## Context Management

### Subagent Rules - FOLLOW THESE
**Spawn a subagent when:**
- Modifying 3+ files → Use implementer subagent
- Searching/exploring codebase → Use explorer subagent
- Debugging with logs/traces → Use debugger subagent
- Running tests → Use subagent to isolate output
- Reviewing multiple files → Use reviewer subagent

**Self-check before heavy operations:** "Should this be in a subagent?"

### Memory System
- `.ai/` files are the truth source for engineering behavior
- `.claude/rules/` files are auto-loaded every turn for runtime state
- Use MCP memory tools to persist decisions across sessions
- Session transcripts are archived and searchable

### At Task Boundaries
- Update `.claude/rules/current-state.md`
- Use `/compact [preserve instructions]` proactively

*Full details in `.claude/rules/subagent-rules.md` and `.claude/USAGE.md`*

---

*Read `specs/ARCHITECTURE.md` for the full stone tablet specification.*
*Read `.ai/AI_ENGINEERING_CONTRACT.md` for behavioral rules.*
