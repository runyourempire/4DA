# Twitter/X Thread: "I built a developer intelligence tool. Here is the architecture."

**Type:** Build-in-public thread
**Voice:** Developer-to-developer, technical, zero marketing fluff
**Hashtags (use sparingly):** #BuildInPublic #DevTools #Rust
**Goal:** Explain the architecture, earn respect, drive downloads

---

## The Thread

---

**[1/13]**

I built a desktop app that monitors 11 content sources, scores everything against your actual codebase, and rejects 99%+ as noise.

It replaced 2 hours of daily HN/Reddit/arXiv scanning with 10 minutes.

Here is the full architecture. Thread.

*(278 chars)*

---

**[2/13]**

The problem: I tracked my content consumption for a month.

847 articles seen across HN, Reddit, arXiv, GitHub Trending, and newsletters.

12 were relevant to my actual work.

That is a 98.6% noise rate. The filter was broken. The filter was me.

*(253 chars)*

---

**[3/13]**

The insight that changed everything:

Your codebase already knows what matters to you. Your dependencies, your language, your architecture patterns -- they define relevance better than any interest checkbox or trending algorithm ever could.

*(247 chars)*

---

**[4/13]**

The stack:

- Tauri 2.0 (Rust backend, not Electron)
- React + TypeScript frontend
- SQLite + sqlite-vec for vector search
- Ollama for local embeddings
- ~15MB binary vs 200MB+ Electron apps

Everything runs on your machine. No cloud. No servers.

*(266 chars)*

---

**[5/13]**

First, ACE (Autonomous Context Engine) scans your local projects.

It reads Cargo.toml, package.json, go.mod, pyproject.toml, requirements.txt, Gemfile, .csproj -- whatever you have.

It also watches your Git activity in real time. Zero configuration.

*(261 chars)*

---

**[6/13]**

Then it scores every piece of content across 5 independent axes:

1. Context -- does this match code you are writing? (embedding similarity)
2. Interest -- does this match your declared interests?
3. ACE -- does this involve your tech stack?

*(253 chars)*

---

**[7/13]**

4. Dependency -- does this mention packages from your installed deps?
5. Learned -- has your click/save/dismiss behavior confirmed this kind of content?

Each axis answers a fundamentally different question. That independence is the whole point.

*(260 chars)*

---

**[8/13]**

The key architectural decision: the confirmation gate.

A single signal -- no matter how strong -- can NEVER make an item relevant alone. The score gets hard-capped below the relevance threshold.

2+ independent axes must confirm. This kills false positives.

*(268 chars)*

---

**[9/13]**

Why 2+ signals? Because keyword matching alone gives you "frustrating" matching "Rust." Embedding similarity alone gives you tangentially related content. Dependency matching alone surfaces every changelog.

The gate forces corroboration. Like peer review for relevance.

*(277 chars)*

---

**[10/13]**

Privacy architecture. Non-negotiable:

- All scoring happens locally
- Zero telemetry. Zero tracking.
- BYOK -- your API keys stay on your machine
- Works fully offline with Ollama
- No account required
- No cloud dependency

Your codebase context never leaves your device.

*(278 chars)*

---

**[11/13]**

MCP integration: 30 tools that plug into Claude Code, Cursor, or any MCP-compatible AI assistant.

Your AI coding tools get real-time intelligence -- dependency updates, security advisories, relevant papers -- without you leaving the editor.

*(249 chars)*

---

**[12/13]**

Results after months of daily use:

- 11 sources (HN, arXiv, Reddit, GitHub, Product Hunt, YouTube, RSS, and more)
- 99%+ noise rejection
- 5-15 relevant items per day
- ~10 min/day instead of 2+ hours

The scoring auto-tunes from your behavior over time.

*(264 chars)*

---

**[13/13]**

4DA is free. All 11 sources, full 5-axis scoring engine, no time limit.

Pro ($12/mo) adds AI briefings, Developer DNA profiling, and Score Autopsy.

FSL-1.1-Apache-2.0 license (converts to Apache 2.0 after 2 years).

Download: https://4da.app

#BuildInPublic

*(267 chars)*

---

## Posting Notes

**Timing:** Tuesday-Thursday, 8-10am ET for maximum dev audience reach.

**Pacing:** Post tweet 1, then reply-chain tweets 2-13 in rapid succession (1-2 min apart). Do not spread across hours -- threads perform best when posted as a complete unit.

**Media suggestions:**
- Tweet 4: Screenshot of the Tauri binary size comparison
- Tweet 5: Screenshot of ACE scanning a project directory
- Tweet 6-7: Diagram of the 5 axes
- Tweet 8: Code snippet from `gate.rs` showing the confirmation gate logic
- Tweet 10: Architecture diagram showing local-only data flow
- Tweet 12: Screenshot of 4DA interface with scored items

**Engagement strategy:**
- Pin tweet 1 to profile
- Quote-tweet tweet 8 (confirmation gate) separately -- it is the most novel technical insight and can stand alone
- Reply to your own thread with "AMA about the architecture" to drive replies
- If the thread gets traction, follow up 24h later with a "things I would do differently" thread

**Cross-posting:**
- Repurpose tweets 1-3 as a LinkedIn post (combine into paragraph form)
- Tweet 8-9 (confirmation gate) works as a standalone dev.to article hook
- The full thread maps to the "How 4DA's 5-axis scoring works" blog post outline in the GTM strategy
