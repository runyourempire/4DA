# Show HN Draft -- Ready to Paste

**Post on:** Tuesday or Wednesday, 9-10am ET
**Have ready:** Be at your computer for 6+ hours after posting. Respond to EVERY comment within 1 hour. Let the product speak for itself.

---

## Title

```
Show HN: 4DA – Scores HN/arXiv/RSS against your actual codebase locally (Rust/Tauri)
```

## Body

I built a desktop app that reads your Cargo.toml, package.json, go.mod, etc., builds a vector profile of your projects, then scores every piece of content it fetches against that profile. Everything runs locally -- your codebase context never leaves your machine.

The scoring pipeline has 15 dimensions: vector similarity against your project files, dependency version matching (it knows when a library you use ships a breaking change), keyword specificity weighting, semantic ACE boost from your actual source code, domain relevance penalties, content quality filtering, taste learning from your feedback, topic-aware freshness decay, and more. An item needs multiple independent signals to pass. The result is ~10 items per day instead of 500.

It monitors HN, Reddit, arXiv, GitHub Trending, RSS, YouTube, Product Hunt, and others. No manual keyword setup -- it auto-discovers your dependencies, languages, and architecture patterns by scanning your local projects.

**Tech:** Rust backend (Tauri 2.0), React frontend, SQLite + sqlite-vec for KNN search, local embeddings via Ollama. ~15MB download.

**Privacy:** No cloud, no telemetry, no account. I cannot see who uses it or how. BYOK for external LLM APIs, or run fully offline with Ollama.

**What's free:** All sources, the full scoring engine, and a 7-module developer optimization curriculum (STREETS). Pro adds AI briefings and deeper analytics.

**License:** FSL-1.1-Apache-2.0 (source available now, converts to Apache 2.0 after 2 years).

Also ships an MCP server (MIT licensed, 30 tools) that plugs into Claude Code, Cursor, etc. -- so your AI coding tools can query your intelligence feed.

I have been using it daily for months. The biggest win is not time saved -- it is catching things I would have missed entirely: a dep shipping a breaking change, an arXiv paper directly relevant to a problem I was stuck on.

Would love feedback on the scoring approach. That is the hardest part and I am still tuning it.

Try it: [download link]
