# Architectural Decisions Log

> This file persists critical decisions across context compaction.
> Updated by Claude at decision points. Re-injected fresh each turn.
> Format: Decision → Rationale → Date

---

## Core Architecture

### AD-001: Tauri 2.0 over Electron
- **Decision**: Use Tauri 2.0 (Rust + WebView) instead of Electron
- **Rationale**: 10x smaller binary, 5x faster startup, native Rust performance for indexing
- **Date**: Project inception
- **Status**: Final

### AD-002: SQLite + sqlite-vss for Vector Storage
- **Decision**: Use SQLite with sqlite-vss extension for embeddings
- **Rationale**: No external database needed, single file, portable, sufficient for local-first app
- **Date**: Project inception
- **Status**: Final

### AD-003: BYOK (Bring Your Own Key) Model
- **Decision**: Users provide their own API keys, never stored remotely
- **Rationale**: Privacy-first principle, no server costs, user controls their data
- **Date**: Project inception
- **Status**: Final

---

## Embedding Strategy

### AD-004: OpenAI text-embedding-3-small as Primary
- **Decision**: Use text-embedding-3-small (1536 dimensions) with Ollama fallback
- **Rationale**: Best cost/quality ratio, widely available, Ollama for offline
- **Date**: Project inception
- **Status**: Final

---

## Frontend Architecture

### AD-005: React 18 + TypeScript + Tailwind
- **Decision**: Standard modern web stack
- **Rationale**: Developer familiarity, excellent tooling, Tailwind for rapid UI
- **Date**: Project inception
- **Status**: Final

---

## Design System

### AD-006: Matte Black Minimalism
- **Decision**: Dark theme (#0A0A0A base), minimal chrome, gold accent sparingly
- **Rationale**: Ambient tool should be visually quiet, not attention-seeking
- **Date**: Project inception
- **Status**: Final

---

## Decisions Pending

<!-- Add decisions under consideration here -->

---

## Rejected Alternatives

### REJ-001: Electron
- **Reason**: Too heavy for an ambient background tool

### REJ-002: External Vector DB (Pinecone, Weaviate)
- **Reason**: Violates local-first principle, adds complexity

### REJ-003: Server-side API key storage
- **Reason**: Privacy violation, liability

---

*Last updated: Session start*
