# 4DA Outreach Templates

Last updated: 2026-02-18

---

## Section 1: Podcast Outreach Email Templates

---

### Template A: The Technical Story Angle

**Target shows:** Changelog, Rustacean Station, devtools.fm, Ship It!, CoRecursive

#### Subject Line Options

1. `A 5-axis scoring architecture for developer content (built in Rust + Tauri)`
2. `We built a relevance engine that rejects 99% of developer content -- here's the architecture`
3. `sqlite-vec, Tauri 2.0, and MCP: building a local-first intelligence app in Rust`

#### Email Body

```
Hi [HOST_NAME],

I'm [YOUR_NAME], and I built 4DA -- a desktop app that scores developer content
from 11 sources (HN, arXiv, Reddit, GitHub, etc.) against your actual codebase
using a 5-axis relevance engine written in Rust.

I built it because I was spending 45+ minutes every morning reading things that
had nothing to do with the code I was actually shipping. The insight was simple:
my Cargo.toml and package.json already know what's relevant to me. So I built
a system that uses them as the filter.

I've been listening to [SHOW_NAME] for [TIME_PERIOD] and thought this could make
a good episode because of the technical decisions involved:

- **5-axis scoring architecture** -- Context, Interest, ACE, Dependency, and
  Learned axes each independently evaluate every item. A multi-signal
  confirmation gate requires 2+ axes to agree before anything enters the feed.
  This is what makes 99%+ noise rejection possible.

- **Privacy-first design** -- all data stays local. No cloud. No sync server.
  BYOK for LLM features. Ollama fallback for fully offline operation. We
  architecturally cannot see user data because we never receive it.

- **Tauri 2.0 vs Electron** -- ~15MB binary vs 150MB+. Rust backend with
  React frontend. The tradeoffs, the sharp edges, and what Tauri 2.0
  actually delivers for a real production app.

- **MCP integration** -- 27 tools that let Claude Code and Cursor query your
  scored intelligence feed. Building on the Model Context Protocol and what
  it means for developer tooling.

- **sqlite-vec for local vector search** -- embedding-based KNN search without
  any external vector database. The gotchas (k in WHERE, not LIMIT) and why
  local-first vector search matters.

I'd love to do a [15-minute pre-call / async Q&A over email] to see if there's
a good episode fit. No pressure either way.

[YOUR_NAME]
[YOUR_TITLE_OR_CONTEXT]
https://4da.app
https://github.com/runyourempire/4DA
```

---

### Template B: The Founder Journey Angle

**Target shows:** IndieHackers Podcast, Syntax.fm, Software Engineering Daily, The Changelog: Founders Talk, devtools.fm

#### Subject Line Options

1. `Solo dev, Rust + Tauri, $0 free tier, $12/mo Pro -- building 4DA in public`
2. `I quit reading the internet and built a system to read it for me`
3. `From information overload to 5 items a day: the 4DA story`

#### Email Body

```
Hi [HOST_NAME],

I'm [YOUR_NAME], a solo developer building 4DA -- a privacy-first desktop app
that surfaces developer-relevant content from 11 sources and rejects 99%+ of
the noise.

The problem is one every developer knows: information overload. The average
developer spends 2+ hours daily consuming content with a signal-to-noise ratio
under 2%. That's 500+ hours a year reading things that don't matter to your work.

The insight came from staring at my own Cargo.toml. My codebase already knows
what's relevant to me -- my dependencies, my tech stack, my architecture
patterns. What if the codebase itself was the filter?

So I built 4DA (4 Dimensional Autonomy). It scans your project manifests,
builds a technology profile, then scores incoming content from Hacker News,
arXiv, Reddit, GitHub, Product Hunt, YouTube, RSS, and more against that
profile using 5 independent scoring axes.

**The architecture bet:** Rust + Tauri 2.0 for a ~15MB binary instead of
Electron's 150MB+. SQLite with sqlite-vec for local vector search. Everything
runs on the user's machine. Zero telemetry. No account required.

**The business model:** Free tier is a complete product -- all 11 sources,
full scoring engine, 27 MCP tools, CLI. Pro at $12/month ($99/year) adds AI
briefings, Developer DNA profiling, and Score Autopsy. BYOK -- users bring
their own API key, so my LLM costs are zero.

**The licensing bet:** FSL-1.1-Apache-2.0. Source-available now, converts to
full Apache 2.0 after 2 years. Balances openness with sustainable monetization.

I think there's a good story here about [RELEVANT_SHOW_THEME -- e.g., "building
sustainable dev tools as a solo founder" / "the indie hacker path for developer
tools" / "choosing Rust for a desktop app in 2026"]. Happy to do a quick
[15-minute call / async exchange] to explore the fit.

[YOUR_NAME]
https://4da.app
https://github.com/runyourempire/4DA
```

---

### Template C: The AI/Privacy Angle

**Target shows:** Latent Space, Practical AI, AI in Action, Privacy-focused podcasts, FLOSS Weekly, Self-Hosted

#### Subject Line Options

1. `Local-first AI architecture: LLMs, vector search, and MCP without a cloud`
2. `Privacy-first developer intelligence -- no telemetry, no cloud, no account`
3. `27 MCP tools, Ollama fallback, sqlite-vec: building AI features that respect privacy`

#### Email Body

```
Hi [HOST_NAME],

I'm [YOUR_NAME], building 4DA -- a developer intelligence tool with an
AI architecture that runs 100% locally with zero telemetry.

Most AI-powered developer tools require sending your data to a cloud service.
4DA takes the opposite approach: everything stays on your machine, and the AI
features are designed to work even if you never connect to an external API.

Here's the architecture:

**Local LLMs via Ollama.** AI features (daily briefings, score explanations)
work with local models. Ollama at localhost:11434 is the default. Users who
want faster responses can BYOK for Anthropic or OpenAI -- but the key never
leaves their machine and we never see it.

**sqlite-vec for vector search.** Embedding-based similarity scoring without
Pinecone, Weaviate, or any external vector database. 384-dimensional embeddings
stored in SQLite. KNN queries run in milliseconds on commodity hardware.

**BYOK (Bring Your Own Key).** No account system. No API proxy. The user's
key goes directly to the provider. We architecturally cannot intercept it
because there's no server in between.

**27 MCP tools.** 4DA ships an MCP server that gives Claude Code, Cursor,
and any MCP-compatible host access to scored intelligence. Your AI assistant
queries your filtered feed instead of generic search results. The MCP server
is MIT licensed and always free.

**Zero telemetry.** Not "anonymized" telemetry. Not "opt-out" telemetry.
Zero. There is no telemetry code in the application. The privacy guarantee
is architectural, not policy-based.

I'd love to discuss [the technical architecture of local-first AI / how MCP
changes the developer tools landscape / building privacy-respecting AI
features] on [SHOW_NAME]. Happy to do a [15-minute pre-call / answer
questions async].

[YOUR_NAME]
https://4da.app
https://github.com/runyourempire/4DA
```

---

## Section 2: Influencer Outreach Templates (Post-Launch)

---

### Template 1: Twitter/X DM (Short) -- Tier 1 Influencers (50K+ followers)

**Character count: under 280**

```
Hi [NAME] -- big fan of your [SPECIFIC_WORK]. Built 4DA: scores dev content
from 11 sources against your codebase. Rust + Tauri, runs locally, 99% noise
rejected. Would love to hear your take if you try it. No strings.
4da.app
```

**Variations:**

```
[NAME] -- built something I think you'd find interesting. 4DA scores HN/Reddit/
arXiv/GitHub content against your actual Cargo.toml/package.json. Privacy-first,
15MB binary, free. Would appreciate your take. 4da.app
```

```
Hi [NAME], your [SPECIFIC_POST/TWEET] resonated. Built 4DA to solve that exact
problem -- 5-axis relevance scoring against your codebase. Rust + Tauri, fully
local, free tier. Check it out if interested. 4da.app
```

---

### Template 2: Twitter/X DM (Medium) -- Tier 2 Influencers (5-50K followers)

**Character count: ~500**

```
Hi [NAME] -- I've been following your work on [SPECIFIC_TOPIC] and thought
you might find 4DA interesting.

It's a desktop app (Rust + Tauri 2.0, ~15MB) that monitors 11 sources -- HN,
arXiv, Reddit, GitHub, Product Hunt, YouTube, RSS, and more -- and scores
every item against your actual codebase using 5 independent relevance axes.

The idea: your Cargo.toml / package.json already knows what's relevant to you.
4DA uses that as the filter. 99%+ of content gets rejected. What survives
actually matters to your work.

Runs 100% locally. Zero telemetry. Free tier is a complete product.

Would love to offer you a complimentary Pro license if you want to try it.
No obligation to post about it -- genuinely just want feedback from someone
who thinks about [THEIR_AREA].

4da.app
```

---

### Template 3: Email for YouTube Reviewers

**Target:** Fireship, Theo, ThePrimeagen, Traversy Media, James Q Quick, Developer tool review channels

#### Subject Line Options

1. `4DA: Rust-based dev intelligence app -- complimentary Pro for review`
2. `15MB alternative to daily.dev that scores content against your codebase`
3. `Built a 5-axis content scoring engine in Rust -- want to review it?`

#### Email Body

```
Hi [REVIEWER_NAME],

I'm [YOUR_NAME], building 4DA -- a developer intelligence desktop app built
with Rust and Tauri 2.0 that scores content from 11 sources against your
actual codebase.

I'd love to offer you a complimentary Pro license to review on your channel.
No editorial requirements -- honest take, good or bad.

**Why this makes good video content:**

- **Live demo is compelling.** Install takes under 3 minutes. Point it at a
  real project, watch it scan the codebase and start scoring. The before/after
  of a raw HN feed vs a scored, filtered feed is visually clear.

- **Architecture is interesting.** 5 scoring axes, multi-signal confirmation
  gate, sqlite-vec for local vector search, MCP integration with 27 tools.
  Good material for a technical breakdown.

- **Tauri 2.0 vs Electron comparison.** ~15MB binary. Rust backend. Real
  production app (not a demo) that shows what Tauri 2.0 actually delivers.
  Good angle if your audience cares about desktop app architecture.

- **Privacy angle.** Zero telemetry, BYOK, no account, runs offline with
  Ollama. The privacy guarantee is architectural, not just a policy page.

- **Free tier is a complete product.** All 11 sources, full scoring engine,
  MCP tools, CLI. Pro ($12/mo) adds AI briefings and diagnostics. Your
  audience can try everything immediately after watching.

**What I can provide:**

- Download link for [PLATFORM]
- Complimentary Pro license (permanent, not time-limited)
- Architecture walkthrough call if useful for your script
- Screenshots, diagrams, or B-roll assets
- Answers to any technical questions async

**Key links:**
- Website: https://4da.app
- GitHub: https://github.com/runyourempire/4DA
- License: FSL-1.1-Apache-2.0 (converts to Apache 2.0 after 2 years)

Happy to jump on a quick call or answer questions over email -- whatever
works best for your workflow.

[YOUR_NAME]
[EMAIL]
```

---

### Template 4: Email for Newsletter Editors

**Target:** TLDR, Pointer, Bytes, Rust Weekly, This Week in Rust, Console.dev, Changelog News, Hacker Newsletter

#### Subject Line Options

1. `4DA: open-source dev intelligence -- 11 sources scored against your codebase`
2. `New Rust + Tauri dev tool: 5-axis content scoring, privacy-first, free`
3. `Pitch: 4DA -- the codebase-aware alternative to daily.dev`

#### Email Body

```
Hi [EDITOR_NAME],

Pitching 4DA for [NEWSLETTER_NAME] -- a developer intelligence desktop app
that scores content from 11 sources against your actual codebase.

**One-liner:** 4DA monitors HN, arXiv, Reddit, GitHub, Product Hunt, YouTube,
and RSS, then scores every item using 5 independent relevance axes built from
your project manifests. 99%+ of content is rejected. What survives is actually
relevant to your work.

**Why your readers care:**

- **Built with Rust + Tauri 2.0.** ~15MB binary, not Electron. Interesting
  to readers who follow the Rust ecosystem or care about desktop app architecture.

- **Privacy-first architecture.** Runs 100% locally. Zero telemetry. BYOK.
  No account required. The privacy guarantee is architectural -- there is no
  server to send data to.

- **Free tier is a complete product.** All 11 sources, full 5-axis scoring
  engine, 27 MCP tools for Claude Code/Cursor, CLI binary. No trial, no
  expiration, no credit card.

- **Pro tier at $12/month ($99/year).** AI briefings, Developer DNA profiling,
  Score Autopsy. BYOK model -- users bring their own LLM key.

- **Source-available.** FSL-1.1-Apache-2.0. Converts to Apache 2.0 after
  2 years. Readers can audit every line.

**Key stats:**
- 11 content sources monitored
- 5-axis relevance scoring (Context, Interest, ACE, Dependency, Learned)
- 99%+ noise rejection via multi-signal confirmation gate
- 27 MCP tools for AI assistant integration
- ~15MB binary (Tauri 2.0)
- $0 free / $12 Pro per month

**Angles that fit different newsletters:**

- **Rust-focused (This Week in Rust, Rust Weekly):** Tauri 2.0 production app,
  sqlite-vec integration, ocrs for pure-Rust OCR, real-world Rust desktop
  architecture decisions.

- **Dev tools (Console.dev, Changelog):** New category of developer tool --
  codebase-aware content scoring vs behavioral recommendations.

- **General dev (TLDR, Bytes, Pointer):** Information overload solution for
  developers. Free, privacy-first, interesting architecture.

**Links:**
- Website: https://4da.app
- GitHub: https://github.com/runyourempire/4DA
- Download: https://github.com/runyourempire/4DA/releases/latest

Happy to provide a review copy, answer questions, or supply any additional
information you need.

[YOUR_NAME]
[EMAIL]
```

---

## Usage Notes

### Personalization Checklist

Before sending any template, replace all bracketed placeholders:

- `[HOST_NAME]` / `[REVIEWER_NAME]` / `[EDITOR_NAME]` -- use their first name
- `[YOUR_NAME]` -- your name
- `[YOUR_TITLE_OR_CONTEXT]` -- e.g., "Solo developer" or "Founder, 4DA"
- `[SHOW_NAME]` / `[NEWSLETTER_NAME]` -- the exact name of their show or publication
- `[SPECIFIC_WORK]` -- reference a specific episode, tweet, article, or video
- `[SPECIFIC_TOPIC]` -- their area of focus
- `[SPECIFIC_POST/TWEET]` -- a particular piece of content that connects to 4DA
- `[TIME_PERIOD]` -- how long you have been following their work
- `[PLATFORM]` -- Windows, macOS, or Linux
- `[EMAIL]` -- your contact email
- `[RELEVANT_SHOW_THEME]` -- match the angle to what their show covers

### Sending Best Practices

1. **Research first.** Listen to at least 2 recent episodes or read 4 recent issues before reaching out. Reference something specific. Generic pitches get deleted.

2. **One follow-up maximum.** If no reply after 7-10 days, send one brief follow-up referencing the original email. After that, move on.

3. **Subject lines matter most.** For podcast hosts, specificity beats cleverness. "A 5-axis scoring architecture for developer content" tells them exactly what the episode would be about.

4. **Keep it short.** These templates are already at the upper limit of acceptable length. Do not add more content. If anything, cut.

5. **Respect the no.** A non-response is a no. A polite decline is a no. Thank them and move on.

6. **Track everything.** Log every outreach in a spreadsheet: date sent, template used, personalization notes, response, follow-up date, outcome.

### Template Selection Guide

| Audience | Template | Lead Angle |
|----------|----------|------------|
| Rust/systems podcasts | A (Technical) | Architecture decisions, Tauri 2.0, sqlite-vec |
| General dev podcasts | B (Founder) | Solo dev story, business model, problem/solution |
| AI/ML podcasts | C (AI/Privacy) | Local LLMs, MCP, zero telemetry |
| Privacy podcasts | C (AI/Privacy) | Architectural privacy, no cloud, BYOK |
| Indie hacker podcasts | B (Founder) | Bootstrapping, pricing, licensing |
| Twitter Tier 1 (50K+) | 1 (Short DM) | Respect their time, offer Pro license |
| Twitter Tier 2 (5-50K) | 2 (Medium DM) | More context, offer Pro license + feedback ask |
| YouTube reviewers | 3 (YouTube Email) | Visual demo potential, provide everything |
| Newsletter editors | 4 (Newsletter Email) | Stats-forward, angle matching, easy copy-paste |
