# Show HN Draft -- Ready to Paste

**Post on:** Tuesday or Wednesday, 9-10am ET
**Have ready:** Be at your computer for 6+ hours after posting. Respond to EVERY comment within 1 hour. Let the product speak for itself.

---

## Title

```
Show HN: 4DA -- Desktop app that scores tech content against your codebase (Rust + Tauri)
```

## Body

Hi HN,

I built 4DA because I was spending too much time trying to stay current. Every morning was the same routine: open HN, open Reddit, check arXiv, browse GitHub Trending, scan a few newsletters. Two hours later, maybe 3 articles were actually relevant to what I was working on.

4DA takes a different approach. Instead of curating by topic or trending metrics, it scans your local codebase and scores every piece of content against your actual project context. It reads your Cargo.toml, package.json, go.mod -- whatever defines your stack -- and builds a profile automatically. No manual keyword setup, no interest checklists.

**What it does:**

- Monitors 11 sources: HN, arXiv, Reddit, GitHub, Product Hunt, YouTube, RSS, and more
- Scans your local projects to build a context profile (auto-discovers dependencies, languages, architecture patterns)
- Scores every article across 5 independent axes using vector embeddings (PASIFA algorithm) -- an item needs 2+ independent signals to pass
- Rejects 99%+ of content as irrelevant noise
- Surfaces only what impacts your specific stack

The result is 5-15 items per day that actually matter to what you are working on, instead of 500+ items that are interesting-but-irrelevant.

**Tech stack:**

- Rust backend with Tauri 2.0
- React + TypeScript frontend
- SQLite + sqlite-vec for vector similarity search
- Local embeddings via Ollama (works fully offline)
- MCP integration for AI coding tools (Claude Code, Cursor, etc.)

**Privacy model:**

All processing happens locally. No cloud. No telemetry. No account required. Your codebase context and reading patterns never leave your machine. BYOK if you want to use external LLM APIs, or run fully local with Ollama. Zero data collection -- I genuinely cannot see who is using it or how.

**License:** FSL-1.1-Apache-2.0 (converts to Apache 2.0 after 2 years -- source is available for inspection).

**Pricing:**

Free tier includes all 11 sources with the full 5-axis scoring engine. This is not a crippled trial -- it is a genuinely useful tool at the free tier. Pro ($12/mo or $99/yr) adds AI-generated daily briefings, Developer DNA profiling, and detailed intelligence panels.

I have been using it daily for months and my "staying current" time went from about 2 hours a day to around 10 minutes. The biggest win is not just time saved -- it is catching things I would have missed entirely. A dependency shipping a breaking change. An arXiv paper directly relevant to a problem I was stuck on. A new library that does exactly what I was about to build from scratch.

I would love feedback on the approach. Particularly interested in whether the scoring accuracy matches your sense of what is relevant -- that is the hardest part to get right and I am still tuning it.

Try it: [download link]
