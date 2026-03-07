# I Built a Tool That Rejects 99% of Tech Content (And Why That's the Point)

*Reading time: ~10 minutes*
*Platform: Dev.to, Hashnode, personal blog*
*Suggested tags: #productivity #devtools #opensource #rust*

---

Last Tuesday, I opened Hacker News at 8:47 AM. By 9:32 AM, I had 23 browser tabs open. A React Server Components deep-dive. A rant about Kubernetes complexity. An arXiv paper on transformer attention mechanisms. A "Show HN" for a CSS framework I will never use.

I work on a Rust desktop application. None of those tabs were relevant to my work.

This is the story of how that frustration turned into 4DA -- a tool that scores every piece of tech content against your actual codebase and rejects 99% of it as noise. And why that rejection rate is not a bug. It is the entire product.

## The Numbers That Made Me Angry

I tracked my content consumption for a month. Not casually -- I actually logged it. Here is what I found:

- **847 articles** crossed my path from HN, Reddit, arXiv, GitHub Trending, RSS feeds, and newsletters
- **12 of them** were directly relevant to what I was actually building
- That is a **98.6% noise rate**

Twelve articles. Out of 847. I spent roughly 2 hours a day -- 60 hours that month -- scanning content, and the signal-to-noise ratio was 1.4%.

This is not unique to me. For developers, the problem is worse because our content landscape is uniquely fragmented. We are not reading one industry publication. We are monitoring Hacker News, Reddit (multiple subreddits), arXiv, GitHub Trending, Product Hunt, YouTube tech talks, RSS feeds, newsletters, and Bluesky/Mastodon threads. Each source has its own ranking algorithm, its own signal-to-noise characteristics, and its own time sink.

The cognitive cost is not just the time spent reading. It is the context switching. It is the guilt of "I should be staying current." It is opening 15 tabs at 9 AM with the intention of "reading these later" and closing them all unread at 6 PM. It is the nagging feeling that you missed something important somewhere in the flood.

## Why Every Existing Tool Fails at This

I tried everything before building my own thing.

**RSS readers (Feedly, Inoreader):** These solve the aggregation problem, not the relevance problem. Feedly collects feeds beautifully. But with 100+ subscriptions, I just moved the firehose from my browser to a dedicated app. The "unread" counter became a source of anxiety, not productivity.

**daily.dev:** Polished, free, great community features. But it personalizes by engagement patterns -- what you click, what you read, what you bookmark. The problem is that developers click on interesting things all the time that have nothing to do with their current work. I read a fascinating article about ZFS internals last month. I do not work on filesystems. But daily.dev dutifully started showing me more storage-related content. Engagement-based personalization optimizes for curiosity, not relevance.

**Pocket (RIP, July 2025):** A reading list that grew until it became its own form of information overload. And then Mozilla shut it down and users lost their data -- a painful lesson about cloud-dependent tools.

**Curated newsletters (Refind, TLDR, etc.):** Someone else's bias. The curator decides what "the developer community" should care about. But "the developer community" includes React frontend engineers, Kubernetes platform teams, embedded systems programmers, ML researchers, and game developers. A newsletter curated for this group is curated for nobody in particular.

Every one of these tools personalizes using one of three signals: what you clicked (behavioral), what you subscribed to (topical), or what you explicitly saved (manual). None of them use the one signal that actually defines what is relevant to your work.

## The Insight: Your Codebase Already Knows What Matters

Here is the thing that seems obvious in retrospect:

**Your codebase is the most accurate representation of what you care about professionally.**

Not your interests. Not your click history. Not the topics you picked from a dropdown when you signed up for a tool. Your actual `Cargo.toml`. Your `package.json`. Your `go.mod`. Your Git commit history.

If your project depends on `sqlite-vec 0.2.1`, then a Hacker News post about a sqlite-vec security advisory is relevant to you. Not because you "like databases" or "clicked on SQLite articles before" -- but because your project will break if you do not read it.

This is a fundamentally different kind of personalization:

| Approach | Signal Source | Example |
|----------|--------------|---------|
| Behavioral (daily.dev) | "You read 5 Rust articles" | Shows more Rust articles |
| Topical (Feedly) | "You subscribed to Rust feeds" | Shows all Rust feed items |
| Codebase-aware (4DA) | "Your project uses `tokio 1.35` and `sqlite-vec 0.2.1`" | Shows the Tokio 1.36 migration guide and the sqlite-vec security patch |

The codebase approach is not incrementally better. It is categorically different.

## How 4DA Actually Works

4DA is a desktop application built with Rust and Tauri 2.0. It runs entirely on your machine. Here is the pipeline:

### Stage 1: Understanding Your Context

When you first run 4DA, the Autonomous Context Engine (ACE) scans your local project directories. It reads manifest files -- `Cargo.toml`, `package.json`, `go.mod`, `requirements.txt`, and more. It parses your Git history to understand what you have been working on recently. It builds what we call a "graduated technology profile."

This is not just "you use React." It is "you are deep in React 19 with Server Components, moderate in TypeScript, and exploring Rust through a side project." Weighted. Nuanced. Automatic. Zero configuration required.

### Stage 2: Ingesting Content

4DA monitors 11 sources continuously: Hacker News, arXiv, Reddit, GitHub, Product Hunt, YouTube, Dev.to, Lobsters, RSS feeds, and more. Every article, paper, discussion, and repository enters the scoring pipeline.

### Stage 3: The 5-Axis Scoring Engine

Every piece of incoming content is evaluated across five independent axes:

- **Context** -- Does this match code you are actually writing? Uses KNN embedding similarity against your codebase context via sqlite-vec.
- **Interest** -- Does the content align with your declared interests and observed patterns?
- **ACE** -- Does this involve your tech stack or active technologies?
- **Dependency** -- Does this mention a specific package from your installed dependencies? A version bump, a CVE, a deprecation notice?
- **Learned** -- Have your past actions (saves, dismissals) indicated this kind of content is or is not valuable? Uses 30-day exponential decay.

### Stage 4: The Confirmation Gate

Here is the part that makes the 99% rejection rate possible: an item needs **2 or more independent signal axes** to pass the confirmation gate. A single signal match is not enough -- it gets treated as coincidence and filtered out.

This is aggressive by design. A keyword match alone does not surface an article. An interest match alone does not surface an article. The content needs to be independently confirmed by at least two different relevance dimensions.

### Stage 5: What Survives is Signal

What makes it through -- typically less than 1% of all scanned content -- is your intelligence feed. No infinite scroll. No engagement optimization. No algorithmic amplification. Just the content that multiple independent signals agree is relevant to your actual work.

## The Results

After using 4DA daily for several months:

**Before:** 2 hours per day across HN, Reddit, arXiv, and various RSS feeds. High anxiety about missing important developments. 15+ tabs of "read later" articles.

**After:** 10 minutes per day. I open 4DA, scan the 3-5 items that survived the gauntlet, read the ones that look relevant, and get back to coding. The anxiety is gone because I trust the scoring. If something impacts my work, it surfaces. If it does not, I never see it.

The time savings are real, but the cognitive savings are bigger. I no longer feel guilty about "not keeping up" because keeping up is no longer my job. It is the tool's job.

## Why Privacy Is Non-Negotiable

There is a reason 4DA is a desktop app and not a web service. To score content against your codebase, the tool needs to read your project files. Your `Cargo.toml`, your `package.json`, your Git history -- these are sensitive.

4DA's privacy architecture is not a feature. It is the foundation:

- **Zero telemetry.** Not "anonymized telemetry." Zero. Nothing leaves your machine.
- **No account required.** Download, install, run.
- **BYOK.** Your API keys go directly to the AI provider. 4DA never sees them.
- **Ollama for offline operation.** Fully local inference. No network dependency.
- **SQLite on your machine.** Regular SQLite file in your `data/` directory. Delete it and it is gone.
- **FSL-1.1-Apache-2.0 license.** Source is public. Converts to Apache 2.0 after 2 years.

## Try It Yourself

1. **Download the app** (~15MB -- Tauri, not Electron)
2. **Point it at your project directories** (or let ACE auto-discover them)
3. **Wait about 60 seconds** for the initial scan
4. **See your first scored content** from 11 sources, ranked by relevance to your codebase

The free tier includes everything: all 11 sources, the full scoring engine, feedback-driven scoring, MCP server (30 tools for Claude Code and Cursor), and the CLI. It is not a trial.

Pro ($12/month or $99/year) adds AI briefings, Developer DNA profiling, and Score Autopsy. But most developers will find the free tier more than sufficient.

The tagline is "All signal. No feed." That is not marketing. That is the design philosophy.

---

**Download 4DA:** [https://4da.ai](https://4da.ai)

**View the source:** [https://github.com/runyourempire/4DA](https://github.com/runyourempire/4DA)
