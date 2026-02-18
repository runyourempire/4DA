# 4DA Build-in-Public Tweets -- Week 2

**Schedule:** Monday Feb 23 - Friday Feb 27, 2026
**Voice:** Developer-to-developer, technical, honest. No marketing fluff.
**Platform:** X (Twitter)
**Link:** https://4da.app

---

## Monday: Feature Highlight -- 5-Axis Scoring

**Main Tweet (267 chars):**

> 4DA scores every item across 5 axes: Context, Interest, ACE, Dependency, and Learned. But here's the key -- a single axis can't promote content alone. 2+ signals must independently confirm relevance before an item surfaces. Result: 99%+ noise rejection, zero false confidence.

**Thread 1/2 (278 chars):**

> Why the multi-signal gate? Early prototype had single-axis scoring. A HN post mentioning "React" would score high on Context alone even if it was irrelevant to your actual work. Requiring 2+ confirming axes killed false positives overnight. Simple rule, massive quality jump.

**Thread 2/2 (250 chars):**

> The 5 axes in practice:
> - Context: matches your codebase keywords
> - Interest: topics you've engaged with
> - ACE: auto-discovered project context
> - Dependency: your actual package graph
> - Learned: adapts from your accept/dismiss behavior

---

## Tuesday: Build-in-Public Update -- Tauri vs Electron

**Main Tweet (273 chars):**

> When building 4DA, I chose Tauri 2.0 over Electron because a developer tool that monitors 11 sources shouldn't eat 200MB+ of RAM just to exist. The Tauri binary is ~15MB. Same app. Native webview instead of bundled Chromium. Rust backend instead of Node. #BuildInPublic

**Thread 1/2 (279 chars):**

> The tradeoff is real though. Electron has a massive ecosystem -- any npm package just works. With Tauri + Rust, I'm writing PDF extraction with pdf-extract + lopdf, OCR with the ocrs crate (pure Rust, no C bindings), and vector search with sqlite-vec. More work. Better result.

**Thread 2/2 (254 chars):**

> Memory comparison on my machine running 4DA idle:
> - Tauri 2.0: ~45MB RSS
> - Equivalent Electron app: ~250MB RSS
>
> For a tool that sits in your tray all day scoring content, that 200MB difference matters. Your RAM belongs to your compiler, not your feed reader.

---

## Wednesday: Engagement / Information Overload

**Main Tweet (274 chars):**

> Every developer I talk to has the same problem: too many sources, not enough signal. HN, Reddit, RSS, GitHub releases, dev blogs -- all open in tabs, all half-read, none scored against what you're actually building. The feed isn't broken. The filter is missing.

**Thread 1/2 (271 chars):**

> I built 4DA because I was mass-consuming content with zero retention. 200 HN items/day, mass-opened tabs, skimmed, closed. Maybe 3 were relevant to my stack. The fix wasn't "read less." It was scoring every item against my actual codebase and dependencies. Locally. Privately.

**Thread 2/2 (227 chars):**

> The uncomfortable truth: most developer content is good content -- just not good for you right now. 4DA doesn't judge quality. It judges relevance to your current project, your dependencies, your interests. Everything else disappears.

---

## Thursday: Screenshot/Demo Description

**Main Tweet (275 chars):**

> What you're seeing: 4DA's scored feed. Every item from HN, Reddit, RSS, and GitHub releases ranked against my actual codebase. The numbers on the left are composite relevance scores -- 5 axes, multi-signal confirmed. Gray items below threshold are auto-filtered. No manual curation.

**Thread 1/1 (269 chars):**

> The scoring happens locally. 4DA's ACE engine scanned my project -- detected Rust, Tauri, React, sqlite-vec, every dependency in Cargo.toml and package.json. Items matching 2+ signal axes surface. Everything else drops below threshold. The feed I see is different from yours.

---

## Friday: Week Recap / Metrics

**Main Tweet (271 chars):**

> This week on the 4DA build:
> - 54 Rust modules, 38 React components
> - 11 content sources integrated
> - 27 MCP tools for AI coding assistants
> - sqlite-vec KNN search working locally
>
> What's coming next: Developer DNA profiling -- your coding patterns as a signal axis. #BuildInPublic

**Thread 1/2 (263 chars):**

> Honest build metric: the hardest part this week wasn't scoring algorithms. It was sqlite-vec's KNN query syntax -- it requires `k = ?` in the WHERE clause, not LIMIT. Took longer to debug than I'd like to admit. Documenting quirks like this saves future-me hours.

**Thread 2/2 (235 chars):**

> Pricing decision we landed on: Free tier gets all 11 sources + full 5-axis scoring. No feature gates on the core feed. Pro at $12/mo adds AI briefings and Developer DNA. The scoring engine should be free. The intelligence layer is the upgrade.

---

## Posting Notes

| Day       | Time (EST)     | Hashtags         | Media              |
|-----------|----------------|------------------|--------------------|
| Monday    | 9:00 AM        | None             | Diagram of 5 axes  |
| Tuesday   | 8:30 AM        | #BuildInPublic   | Binary size comparison screenshot |
| Wednesday | 9:00 AM        | None             | None (text-only for engagement) |
| Thursday  | 8:30 AM        | None             | Screenshot of scored feed |
| Friday    | 9:00 AM        | #BuildInPublic   | None or GitHub stats screenshot |

**Engagement strategy:** Reply to comments within 15 minutes of posting. Wednesday's tweet is designed to attract quote-tweets -- engage with every reply. Thread all tweets immediately; do not drip-feed.
