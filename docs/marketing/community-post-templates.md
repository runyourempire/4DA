# Community Post Templates

**Product:** 4DA (4 Dimensional Autonomy)
**Date:** 2026-02-18
**Purpose:** Ready-to-post templates tailored to each community's culture and norms.

---

## Table of Contents

1. [r/rust](#1-rrust)
2. [r/commandline](#2-rcommandline)
3. [r/selfhosted](#3-rselfhosted)
4. [Dev.to Article Intro](#4-devto-article-intro)

---

## 1. r/rust

### Community Rules and Norms

- Posts must be directly related to the Rust programming language or ecosystem.
- Self-promotion is tolerated only when the project is genuinely interesting from a Rust perspective and the poster participates in discussion.
- The community values technical substance over marketing polish. Show code, discuss tradeoffs, ask real questions.
- Flair the post correctly (likely "Show r/rust" or similar project showcase flair if available, otherwise use the default).
- Do not link directly to a product page. Link to a GitHub repo or a technical writeup.
- r/rust readers will read your Cargo.toml, your error handling strategy, and your dependency choices before they look at your landing page.

### Post Title

```
Built a desktop app with Tauri 2.0: sqlite-vec for KNN search, ocrs for pure-Rust OCR, and a scoring engine that reads your Cargo.toml
```

### Post Body

```
I've been working on a Tauri 2.0 desktop app for the past 8 months and wanted to share some of
the architecture decisions and Rust-specific gotchas I ran into. The app (4DA) monitors content
sources (HN, arXiv, Reddit, GitHub, RSS, etc.) and scores everything against your local codebase
for relevance. The interesting parts are all in the Rust backend.

**Stack overview:**

- Tauri 2.0 (with a React/TS frontend, but the heavy lifting is all Rust)
- rusqlite + sqlite-vec for vector similarity search
- ocrs for OCR (pure Rust -- no C bindings, no tesseract dependency)
- pdf-extract + lopdf for PDF text extraction
- docx-rs + calamine for Office document parsing
- thiserror for error types, anyhow for application errors
- ts-rs v10 with serde-compat for generating TypeScript types from Rust structs

**Things I learned the hard way:**

1. **sqlite-vec KNN queries** require `k = ?` in the WHERE clause. If you put `LIMIT ?` at the
   end of the query like you would with normal SQL, it silently returns wrong results. Took me
   two days to figure out why my similarity search was garbage.

2. **MutexGuard<T> is not Send.** I had a `MutexGuard<SourceRegistry>` that I was accidentally
   holding across an await point. The compiler error was clear once I understood it, but tracking
   down which await was the problem in a long async function was not fun. Solution was to scope
   the guard tightly and clone what I needed before the await.

3. **ocrs vs tesseract:** I went with ocrs because I did not want to require users to install
   tesseract system-wide. Pure Rust, compiles with the rest of the project, no runtime
   dependencies. The accuracy is good enough for extracting text from screenshots and diagrams.
   If you need production OCR at scale, tesseract is still better, but for a desktop app where
   you control the input, ocrs works.

4. **No unwrap() in production code.** I enforced this as a project rule early on. Every fallible
   operation returns a Result and gets propagated or handled with a graceful fallback. It made
   the error handling verbose in some places but the app basically never panics in user hands.

**The scoring algorithm (PASIFA):**

The core scoring uses 5 independent axes -- context match, interest match, ACE (real-time project
signals), dependency relevance, and learned behavior. Each axis produces a confidence-weighted
score. An item needs 2+ independent signals above threshold to pass the confirmation gate. The
threshold auto-tunes using a 30-day exponential decay on user behavior. Typical rejection rate
is 99%+ of incoming content.

The vector similarity search uses sqlite-vec's KNN implementation against embeddings generated
locally via Ollama. If Ollama is not available, it falls back to zero vectors and relies on the
non-embedding axes for scoring.

**Privacy model:**

Everything runs locally. No network calls except to fetch content from the configured sources.
BYOK for any AI features. Zero telemetry. The database is a plain SQLite file you can inspect
with any SQLite client.

**License:** FSL-1.1-Apache-2.0 (converts to Apache 2.0 after 2 years). The MCP server component
is MIT licensed separately.

I would genuinely appreciate feedback on the architecture. A few open questions:

- For the sqlite-vec integration, I'm doing batch inserts of embeddings with a transaction. Is
  there a better pattern for bulk vector upserts with rusqlite?
- The ocrs crate works well but the model loading is slow on first run (~3 seconds). Anyone found
  a good way to warm this up in the background with Tauri?
- Is there interest in a standalone Rust crate for the PASIFA scoring algorithm? I've been
  thinking about extracting it.

Repo link: [link to GitHub]
```

### Suggested Timing

- **Day:** Tuesday, Wednesday, or Thursday
- **Time:** 9:00-11:00 AM ET (European Rustaceans are still online, US coasts are active)
- **Avoid:** Weekends (lower traffic), Mondays (competition with release announcements)

### What NOT to Do

- Do not lead with what the product does for the user. Lead with what you built in Rust and how.
- Do not use marketing language ("all signal, no feed," "developer intelligence"). The community will downvote anything that reads like ad copy.
- Do not link to a landing page or product website as the primary link. Link to the GitHub repo.
- Do not post and disappear. r/rust expects you to answer technical questions in the comments for hours. If someone asks about your error handling strategy, you answer with code.
- Do not exaggerate performance claims. If someone benchmarks your sqlite-vec queries and gets different numbers, your credibility is gone.
- Do not cross-post this to r/programming at the same time. Space them out by at least 2 days.
- Do not mention pricing or Pro tier in the post body. If someone asks, answer honestly in comments.

---

## 2. r/commandline

### Community Rules and Norms

- Posts should be about command-line tools, terminal workflows, and shell scripting.
- The community respects tools that do one thing well, are composable with other Unix tools, and produce parseable output.
- Show the actual commands. Screenshots of terminal output are good. Marketing screenshots of a GUI are bad.
- If your tool has a GUI, that is fine, but the post should focus on the CLI interface.
- Brevity is valued. Long marketing copy gets scrolled past. Show the commands, show the output.

### Post Title

```
4da: CLI tool that scores tech content against your codebase. Pipes to jq, works in cron jobs.
```

### Post Body

```
I built a CLI binary that monitors 11 content sources (HN, arXiv, Reddit, GitHub, Product Hunt,
YouTube, RSS) and scores everything against your local project files. Sharing because the CLI
interface was designed for terminal-native workflows.

**Quick examples:**

    # Morning briefing -- what's relevant to your stack today
    $ 4da briefing
    ╭─────────────────────────────────────────────────────────╮
    │ Daily Briefing - 2026-02-18                             │
    │ 7 items scored above threshold (of 342 fetched)         │
    ╰─────────────────────────────────────────────────────────╯
    [0.94] CVE-2026-1847 affects tokio 1.36 (you use 1.35)
    [0.91] sqlite-vec 0.2.0 released -- breaking API change
    [0.87] arXiv: Efficient KNN for embedded vector stores
    [0.83] Tauri 2.1 beta -- new IPC model
    ...

    # Filter for critical signals only
    $ 4da signals --critical
    ⚠ CVE-2026-1847: tokio <= 1.36 remote DoS (your Cargo.lock: tokio 1.35.1)
    ⚠ Breaking change: sqlite-vec 0.2.0 renames vec_search -> vec_knn

    # Find knowledge gaps in your dependencies
    $ 4da gaps
    3 gaps detected:
      - serde_json 1.0.114 (6 articles scored, 0 read)
      - tower-http 0.5.1 (new major features, no engagement)
      - axum 0.7 (migration guide published 3 weeks ago)

    # JSON output for scripting
    $ 4da briefing --json | jq '.items[] | select(.score > 0.9) | .title'
    "CVE-2026-1847 affects tokio 1.36"
    "sqlite-vec 0.2.0 released"

    # Use in a cron job -- send high-priority items to ntfy
    $ 4da signals --critical --json | jq -r '.[].title' | while read -r line; do
        curl -d "$line" ntfy.sh/my-dev-alerts
      done

    # Pipe to fzf for interactive selection
    $ 4da briefing --json | jq -r '.items[] | "\(.score | tostring) \(.title) \(.url)"' | \
        fzf --preview 'echo {}' | awk '{print $NF}' | xargs open

**How it works:**

It reads your project files (Cargo.toml, package.json, go.mod, pyproject.toml, etc.) to build
a context profile, then scores incoming content across 5 axes. Requires 2+ independent signals
above threshold to surface an item. Everything runs locally.

**Install:**

    # From GitHub releases (pre-built binaries)
    $ curl -sSL https://4da.app/install.sh | sh

    # Or from source
    $ cargo install 4da-cli

**Output formats:** human-readable (default), --json, --csv
**Config:** ~/.config/4da/config.toml or environment variables
**Data:** SQLite database at ~/.local/share/4da/4da.db (inspect it yourself)

The CLI binary is the same codebase as the desktop app but without the GUI. All scoring
happens locally, no cloud calls. BYOK if you want AI-powered briefings.

Free and open-source (FSL-1.1-Apache-2.0, converts to Apache 2.0 after 2 years).
```

### Suggested Timing

- **Day:** Tuesday or Wednesday
- **Time:** 10:00 AM - 1:00 PM ET
- **Avoid:** Posting within 24 hours of the r/rust post

### What NOT to Do

- Do not lead with a description of the desktop app or GUI. This is r/commandline. The CLI is the product here.
- Do not post a wall of text without actual command examples. Show real terminal output.
- Do not use words like "intelligence," "AI-powered," or "smart" in the title. This community values concrete capabilities over buzzwords.
- Do not hide the install method. The first question will be "how do I install it?" -- answer it in the post.
- Do not claim it replaces tools people love (like newsboat). Position it as complementary.
- Do not fake the terminal output. If someone runs the commands and gets different formatting, they will call it out.
- Do not post without testing that the --json output actually pipes cleanly to jq. Someone will try it within minutes.

---

## 3. r/selfhosted

### Community Rules and Norms

- Posts should relate to self-hosting software or services.
- The community is deeply skeptical of cloud dependencies, telemetry, and "freemium" products that degrade the free tier over time.
- Leading with privacy and data sovereignty is expected, not a differentiator -- it is the baseline.
- Be explicit about what network calls the app makes. If it phones home for anything, disclose it upfront.
- The community will ask about Docker, reverse proxies, and multi-user setups. Have answers ready even if the answer is "not yet."
- r/selfhosted users will inspect your network traffic. Do not claim "zero telemetry" unless it is literally true.

### Post Title

```
4DA: local-only developer content scoring. SQLite database, zero telemetry, zero cloud dependency. Your data never leaves your machine.
```

### Post Body

```
I've been lurking here for years and finally have something worth sharing. 4DA is a desktop app
that monitors developer content sources and scores them against your local projects for relevance.
Posting here because the privacy architecture was designed with this community's values in mind.

**What it does:**

Pulls content from 11 sources (Hacker News, arXiv, Reddit, GitHub, Product Hunt, YouTube, RSS
feeds, and others), scores each item against your local codebase using vector embeddings, and
surfaces only what is relevant to your actual tech stack. Rejects 99%+ as noise.

**Privacy and data architecture:**

- **Zero telemetry.** No analytics, no usage tracking, no crash reporting. Nothing phones home.
  Set up Wireshark and verify -- I mean it.
- **Zero cloud dependency.** The app works fully offline if you use Ollama for embeddings. The
  only network calls are to fetch content from the sources you configure.
- **BYOK.** If you want AI features (briefings, etc.), you provide your own API key. It calls
  the provider directly from your machine. We never see the key.
- **No account required.** No email, no signup, no login. Download, run, done.
- **SQLite database.** All data stored in a single `4da.db` file you can inspect with any SQLite
  client. `sqlite3 ~/.local/share/4da/4da.db ".tables"` and you can see exactly what is stored.
- **Config is a JSON file.** No proprietary config format. Edit it with your text editor.
- **Ollama for local embeddings.** Vector embeddings for content scoring are generated locally
  via Ollama. No data sent to OpenAI/Anthropic/Google unless you explicitly configure an
  external provider with your own key.

**Network calls the app makes (full transparency):**

1. HTTP GET to each content source you enable (HN API, Reddit API, arXiv API, etc.)
2. If you configure an external AI provider: HTTPS to that provider's API endpoint with your key
3. That is it. No other network calls. No update checks, no license validation, no telemetry
   endpoints.

**What this is NOT:**

- Not a web service. There is nothing to self-host in the traditional sense -- it is a desktop
  app (Tauri 2.0, Rust backend) that runs on your machine.
- Not a SaaS product with a "local mode." There is no cloud version. Local is the only version.
- Not open-core with a proprietary server. The scoring engine, all 11 sources, and the full
  pipeline run locally in the free tier.

**Tech stack for the curious:**

- Rust backend with Tauri 2.0
- SQLite + sqlite-vec for vector similarity search
- React + TypeScript frontend
- Ollama for local AI (optional)
- FSL-1.1-Apache-2.0 license (converts to full Apache 2.0 after 2 years)

**Pricing model (being transparent because this community hates bait-and-switch):**

- Free tier: all 11 sources, full scoring engine, CLI binary, basic MCP server. No time limit,
  no feature degradation, no "after 30 days" lockout.
- Pro ($12/mo or $99/yr): adds AI-generated briefings, Developer DNA profiling, detailed score
  breakdowns. These features require LLM calls, which is why they are paid -- the compute cost
  is real.
- The free tier will never be degraded to push people to Pro. That is a commitment, not a
  marketing line.

I know the immediate question: "Can I run this in Docker?" Not yet -- it is a desktop app with
a GUI. But the CLI binary could theoretically run in a container. If there is interest, I could
look into a headless mode for server deployment.

Happy to answer any questions about the data architecture or network behavior.

Download: [link to GitHub releases]
```

### Suggested Timing

- **Day:** Wednesday or Thursday (stagger 1-2 days after the r/rust post)
- **Time:** 10:00 AM - 12:00 PM ET
- **Avoid:** Same day as r/rust or r/commandline posts

### What NOT to Do

- Do not use the phrase "all signal, no feed" or any tagline. r/selfhosted does not want a pitch. They want a technical description.
- Do not bury the privacy details below the fold. Lead with them. In this community, "what network calls does it make?" is question #1.
- Do not claim "open source" unless the license meets the OSI definition. FSL-1.1-Apache-2.0 is source-available, not open source by the strict definition. The community will correct you aggressively if you get this wrong.
- Do not dodge the Docker question. Address it proactively in the post, even if the answer is "not supported yet."
- Do not link to a marketing landing page. Link directly to the GitHub repo or the releases page.
- Do not post if you have any telemetry, analytics, or tracking -- even "anonymous" crash reporting. This community considers all of it a violation. The claim must be literally true.
- Do not compare yourself favorably to other self-hosted projects. This community is protective of its ecosystem.
- Do not edit the post later to add marketing content. People will notice and call it out.

---

## 4. Dev.to Article Intro

### Platform Rules and Norms

- Dev.to uses YAML front matter for metadata (title, tags, series, cover image, canonical URL).
- Maximum 4 tags per article. Tags should be existing popular tags, not custom ones.
- The community values educational content over product announcements. Frame it as "here is what I learned" not "here is what I built."
- Articles that teach a concept while referencing a project perform 5-10x better than pure product posts.
- Use headers, code blocks, and short paragraphs. Dev.to readers skim aggressively.
- A cover image significantly increases engagement (Dev.to uses 1000x420px).
- Canonical URL should point to your own blog if you cross-post.

### Front Matter

```yaml
---
title: "I Built a Tool That Rejects 99% of Tech Content (And Why That's the Point)"
published: true
description: "How I stopped drowning in tech news by building a scoring engine that reads my Cargo.toml instead of my browsing history."
tags: productivity, devtools, rust, opensource
cover_image: https://4da.app/blog/images/noise-rejection-cover.png
canonical_url: https://4da.app/blog/noise-rejection
series: "Building Developer Intelligence"
---
```

### Article Body

```markdown
## The morning routine nobody talks about

Here is what my morning used to look like:

1. Open Hacker News. Scan 30 titles. Open 8 tabs. Read 3 articles. Close the rest.
2. Check r/programming. Scroll past 20 posts. Open 2. Read 1.
3. Glance at arXiv cs.LG. See 47 new papers. Read 0 abstracts because who has time.
4. Check GitHub trending. See 10 repos in languages I do not use. Close the tab.
5. Open the 4 newsletters that arrived overnight. Skim headlines. Archive all of them.

**Total time: 90 minutes. Articles relevant to my actual work: maybe 2.**

I tracked this for a month. Out of 847 pieces of content that crossed my screen, 12 were
directly relevant to the projects I was working on. That is a **98.6% noise rate.**

The problem is not that there is too little good content. The problem is that every content
platform optimizes for the wrong signal. Hacker News optimizes for community interest. RSS
feeds optimize for recency. Newsletters optimize for broad appeal. None of them know what
I am actually building.

## The insight that changed everything

One morning, staring at yet another "Introduction to React Hooks" article (I write Rust),
it hit me: **my codebase already knows what matters to me.**

My `Cargo.toml` lists my dependencies. My source files reveal the patterns I use. My Git
history shows what I am actively working on. My `package.json`, `go.mod`, `pyproject.toml`
-- all of these are a machine-readable description of my technical identity.

What if, instead of curating feeds by topic, I scored content against my actual code?

## What 99% rejection actually looks like

I built this idea into a tool called [4DA](https://github.com/...). It monitors 11 content
sources and scores every piece of incoming content across 5 independent axes:

| Axis | What it measures | Example |
|------|-----------------|---------|
| **Context** | Match against your project files | "tokio 1.36 CVE" scores high if your Cargo.lock contains tokio |
| **Interest** | Alignment with your stated interests | "Rust async patterns" scores high if you write async Rust |
| **ACE** | Real-time signals from your active work | Scores higher for technologies you touched in the last 48 hours |
| **Dependency** | Direct dependency relevance | A breaking change in a crate you use scores maximum |
| **Learned** | Behavioral pattern from your history | Items similar to ones you clicked before score higher |

The key mechanism is the **confirmation gate**: an item needs 2 or more independent axes to
score above threshold before it surfaces. This kills false positives. A single keyword match
is not enough. The system needs corroboration.

The result: out of ~300-500 items fetched per day across all 11 sources, typically 5-15
survive. Everything else gets rejected. My morning content time went from 90 minutes to
about 10.

## The privacy question

Every content tool I evaluated before building this had the same problem: they needed my
data in their cloud. My reading patterns, my interests, my browsing behavior -- all shipped
to someone else's server to be processed, profiled, and often sold.

4DA takes the opposite approach:

- **All processing happens locally.** The scoring engine runs on your machine.
- **Your codebase context never leaves your machine.** The project scanner reads your local
  files and stores embeddings in a local SQLite database.
- **Zero telemetry.** No analytics, no crash reporting, no usage tracking.
- **BYOK.** If you want AI features, you bring your own API key. Direct calls to the
  provider. We never see the key.

The database is a plain SQLite file. You can inspect it with `sqlite3`. There are no
proprietary formats, no encrypted blobs, no hidden data.

## The technical bits (for the curious)

The backend is Rust (Tauri 2.0). Vector similarity search uses sqlite-vec. Local embeddings
are generated via Ollama, so the entire pipeline can run offline. OCR is handled by the
`ocrs` crate (pure Rust, no C dependencies). PDF extraction uses `pdf-extract` + `lopdf`.

The scoring algorithm (PASIFA -- Privacy Aware Semantic Intelligence 4 File Analysis)
uses confidence-weighted thresholds that auto-tune over time based on your behavior, with
a 30-day exponential decay so it adapts as your projects change.

If you want the full architecture deep-dive, I am writing a separate post on the Rust
implementation details.

## Try it yourself

4DA is free to download and use. The free tier includes all 11 sources (HN, arXiv, Reddit,
GitHub, Product Hunt, YouTube, RSS, and more) with the full scoring engine. There is no
trial period and no feature degradation.

A paid Pro tier ($12/mo) adds AI-generated daily briefings and a Developer DNA profile that
maps your technical identity from your codebase. But the core scoring -- the part that
rejects 99% of noise -- is free.

**What you should see in your first 60 seconds:**

1. Install and open the app
2. It auto-discovers your local projects
3. Content starts flowing and getting scored
4. Irrelevant items drop away. What remains is signal.

[Download 4DA](https://4da.app) | [GitHub repo](https://github.com/...)

---

*This is Part 1 of a series on building developer intelligence tools. Next up: the Rust
architecture behind 5-axis content scoring with sqlite-vec.*

---

*What does your "staying current" routine look like? I would love to hear how others handle
the information overload problem. Drop a comment below.*
```

### Suggested Timing

- **Day:** Tuesday
- **Time:** 8:00-10:00 AM ET (Dev.to's highest engagement window)
- **Sequence:** Publish this 1-2 days before or after the Reddit posts, not on the same day. Stagger to avoid the appearance of a coordinated marketing blitz.

### What NOT to Do

- Do not make the article purely about 4DA. It must teach something. The article is about the noise rejection problem and the scoring approach. 4DA is the implementation, not the subject.
- Do not use the `#showdev` tag as the primary tag. Use `#productivity` or `#devtools` first. `#showdev` articles get less reach than educational ones.
- Do not skip the YAML front matter. Articles without proper tags get significantly less distribution.
- Do not write a short post. Dev.to rewards articles in the 1,500-2,500 word range. Anything under 800 words gets minimal engagement.
- Do not use aggressive CTAs ("Download now!", "Sign up today!"). One link at the end is sufficient. Dev.to readers are allergic to hard sells.
- Do not cross-post the exact same content to Hashnode without setting a canonical URL. Duplicate content hurts SEO for both.
- Do not publish and walk away. Dev.to rewards authors who reply to every comment in the first 24 hours. Comment engagement boosts article visibility in the feed.
- Do not use a cover image with the product logo prominently displayed. Use something abstract or a screenshot that tells the story visually. Logo-forward covers read as ads.

---

## Appendix: Posting Sequence

To avoid the appearance of a coordinated spam campaign, stagger posts across communities:

| Day | Platform | Post |
|-----|----------|------|
| Tuesday | r/rust | Technical architecture post |
| Wednesday | Dev.to | Noise rejection article |
| Thursday | r/selfhosted | Privacy and local-first post |
| Following Tuesday | r/commandline | CLI tool post |

Space each post by at least 24 hours. If any post gains significant traction (front page, 100+ upvotes), delay the next post by an extra day to ride the momentum without splitting attention.

Never post to multiple subreddits on the same day. Reddit's anti-spam systems flag accounts that post the same product to multiple communities simultaneously, and moderators share notes across developer subreddits.
