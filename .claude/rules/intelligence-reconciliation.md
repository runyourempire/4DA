# Intelligence Reconciliation

Before touching ANY intelligence surface (Briefing, Preemption, Blind Spots, Evidence, Knowledge Decay, Signal Chains), read:

- `docs/strategy/INTELLIGENCE-RECONCILIATION.md` — THE plan: 12→5 tab collapse, one canonical type.
- `docs/strategy/EVIDENCE-ITEM-SCHEMA.md` — the canonical `EvidenceItem` contract every intelligence surface must emit.
- `.claude/rules/intelligence-doctrine.md` — the ten enforced rules (no new types, no vanity metrics, etc).

The Momentum tab is **being deleted**. Five parallel intelligence systems are collapsing into one pipeline. Do NOT add new Alert/Gap/Recommendation/Insight struct variants — extend `EvidenceItem` via ADR instead.
