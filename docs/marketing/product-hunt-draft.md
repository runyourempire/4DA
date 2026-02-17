# Product Hunt Launch Draft -- Ready to Paste

**Launch on:** Tuesday or Wednesday (avoid Monday competition and Friday low traffic)
**Category:** Developer Tools
**Assets needed before launch:** See gallery checklist at the bottom

---

## Tagline (recommended)

```
All signal. No feed. Content scored against your codebase.
```

(58 chars)

---

## Description (under 500 chars)

```
4DA monitors 11 content sources (HN, arXiv, Reddit, GitHub, Product Hunt, YouTube, RSS) and scores every article against your actual codebase. If it doesn't impact your stack, you never see it.

Built with Rust + Tauri. Runs 100% locally. No cloud, no telemetry, no account required.

Free tier: all 11 sources with full 5-axis relevance scoring.
Pro ($12/mo): AI briefings, Developer DNA profiling, priority support.

For developers who want signal, not another feed.
```

(489 chars)

---

## Maker's First Comment

Hey Product Hunt! I'm the maker of 4DA.

I built this because every content tool I tried had the same fundamental problem: they curate by popularity or topic, not by relevance to MY actual work. daily.dev shows me trending React articles when I'm deep in a Rust backend. Feedly collects everything but scores nothing. Newsletters are just someone else's bias about what matters.

4DA flips the model. Your codebase IS the filter. It scans your local projects -- reads your Cargo.toml, package.json, go.mod, whatever defines your stack -- generates vector embeddings, and scores every piece of content from 11 sources across 5 independent axes. An item needs 2+ independent signals to even pass the confirmation gate. Typical rejection rate is 99%+. What survives is signal.

A few things I'm particularly proud of:

- **Privacy-first, for real:** Your data genuinely never leaves your machine. Not "we encrypt it in our cloud" -- it literally does not leave. I cannot see who uses 4DA or how. Zero telemetry.
- **The free tier is genuinely useful:** All 11 sources, the full scoring engine, unlimited use. It's not a 14-day trial and it's not crippled to push you into paying. Most people will never need Pro.
- **MCP integration:** If you use Claude Code, Cursor, or other MCP-compatible AI tools, 4DA feeds your dev context into your AI assistant. Your AI knows about the Tokio update that shipped yesterday.
- **Works offline:** Local embeddings via Ollama. No internet required for scoring after initial content fetch.

The scoring is the core of the product and the hardest thing to get right. It uses 5 independent axes (context match, interest alignment, real-time project signals, dependency relevance, and learned behavior), then applies a confirmation gate that requires agreement across multiple axes. It catches things I would have missed entirely -- dependency breaking changes, relevant arXiv papers, new libraries that solve problems I was actively working on.

I've been using it daily for months. My content consumption went from ~2 hours/day to about 10 minutes.

Would love your feedback. What sources would you want added? How does the scoring match your intuition about what matters? What's missing?

---

## Gallery Assets Checklist

- [ ] **Hero image:** 4DA interface showing scored content items with relevance percentages (dark UI, clean layout)
- [ ] **GIF/Video:** 15-second demo showing codebase scan -> content scoring -> filtered results appearing
- [ ] **Screenshot 1:** Before/after comparison (raw 11-source firehose vs. 4DA filtered output)
- [ ] **Screenshot 2:** Score autopsy view -- the 5-axis breakdown showing WHY an item was surfaced
- [ ] **Screenshot 3:** Developer DNA profile view
- [ ] **Screenshot 4:** AI briefing / intelligence panel (Pro feature preview)
- [ ] **Screenshot 5:** Privacy architecture -- showing local-only data flow (no arrows leaving the machine)
- [ ] **Demo video:** 60-90 second screen recording with voiceover showing install -> first scan -> scored results

---

## Alt Tagline Options

1. `All signal. No feed. Content scored against your codebase.` (58 chars) -- RECOMMENDED
2. `11 sources, 99% noise rejected. Your code sets the filter.` (58 chars)
3. `Developer intelligence that knows your actual codebase.` (55 chars)
4. `Stop reading everything. Start knowing what matters.` (52 chars)
5. `Privacy-first content scoring for developers.` (46 chars)

**Notes on selection:** Option 1 leads with the brand line ("All signal. No feed.") and immediately communicates the mechanism. Option 4 is the strongest emotional hook but says less about the product. Option 2 has the most concrete proof points. Test options 1 and 4 with the PH community in advance if possible.
