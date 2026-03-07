# Product Hunt Launch Draft -- Ready to Paste

**Launch on:** Tuesday or Wednesday, 12:01 AM PT (PH resets daily at midnight PT)
**Category:** Developer Tools
**Topics:** Developer Tools, Privacy, Open Source, Artificial Intelligence, Productivity
**Assets needed before launch:** See gallery checklist at the bottom
**Be ready:** Respond to every comment within 30 minutes for the first 6 hours

---

## Tagline (60 chars max)

```
Your codebase decides what you read. Not an algorithm.
```

(54 chars)

**Why this works:** Product Hunt's audience includes non-developers. "Your codebase decides" is concrete and immediately communicates the mechanism. "Not an algorithm" creates contrast with every other content tool they have seen. It invites curiosity: "wait, how does a codebase decide?"

---

## Description (260 chars max)

```
4DA scans your local projects and scores tech content from 11 sources against your actual code. 99% gets rejected as noise. What survives is relevant to YOUR stack. Runs locally, zero telemetry, free forever. Windows, macOS, Linux.
```

(232 chars)

**Breakdown of what each sentence does:**
1. Mechanism -- what the product does (scan + score)
2. Proof point -- quantified result (99% rejection)
3. Benefit -- personalized to YOU (not generic curation)
4. Trust + availability -- local, private, free, cross-platform

---

## Maker Comment (post immediately after launch goes live)

```
Hey Product Hunt -- I built 4DA because I tracked my content habits for a month and the numbers made me angry.

847 articles crossed my path from HN, Reddit, arXiv, GitHub Trending, and RSS feeds. I logged every one. 12 were directly relevant to what I was building. That is a 98.6% noise rate. I was spending 2 hours a day staying current and getting almost nothing useful out of it.

I tried everything. Feedly collects feeds beautifully but scores nothing. daily.dev personalizes by what you click -- so when I read one fascinating article about ZFS internals, it started showing me storage content for weeks. I do not work on storage. Newsletters are just someone else's opinion about what "developers" should care about.

The fix was obvious in retrospect: my Cargo.toml already knows what matters. My package.json. My go.mod. Your codebase is the most accurate signal of what is professionally relevant to you -- not your clicks, not your subscriptions, not a topic checklist you filled out once.

So I built 4DA. It reads your manifest files locally, builds a technology profile, and scores every piece of content across 5 independent axes. An article needs 2+ confirming signals to even surface. Everything else gets rejected. Typical result: 5-15 items per day that actually impact your work, instead of 500+ that are interesting-but-irrelevant.

Things I want you to know:

- The free tier is real. All 11 sources, the full scoring engine, feedback-driven scoring, the MCP server (30 tools for Claude Code / Cursor). It is not a trial. Most people will never need Pro.

- Privacy is not a feature -- it is the architecture. Your data literally cannot reach me. There is no server. No telemetry. No account. I genuinely do not know how many people use 4DA. That is by design.

- It works offline. Local embeddings via Ollama. No internet required for scoring after initial content fetch.

- ~15 MB installed. Rust backend (Tauri 2.0), React frontend, SQLite with vector search. It is fast.

My content consumption went from ~2 hours/day to about 10 minutes. The biggest win is not time saved -- it is catching things I would have missed entirely. A dependency shipping a breaking change. An arXiv paper solving a problem I was stuck on.

Would love to hear what you think. Especially: does the scoring match your intuition about what matters? That is the hardest part to get right.
```

**Why this version works:**
- Opens with a specific, relatable number (847 articles, 12 relevant) -- not a vague problem statement
- Names competitors respectfully but explains exactly why they fail
- "My Cargo.toml already knows what matters" is the memorable insight
- Feature list is short and leads with what matters to PH audience (free, private, small)
- Ends with a genuine question that invites engagement
- No exclamation marks, no hype language, no "we're so excited" -- just a developer explaining a tool

---

## First Comment (post 2-5 minutes after maker comment)

```
For the developers in the thread -- some technical depth:

**How the scoring works:**
4DA uses a 5-axis scoring system called PASIFA. Each axis evaluates content independently:

1. Context match -- does this align with your explicit interests?
2. Interest alignment -- does the content quality/topic fit?
3. ACE (Autonomous Context Engine) -- does your codebase context confirm relevance?
4. Dependency relevance -- does this affect a package you actually use?
5. Feedback signals -- have your saves/dismissals adjusted the scoring?

An item needs 2+ independent signals to pass the confirmation gate. Single-signal matches get rejected as coincidence. This is what produces the 99%+ rejection rate -- and why what survives is actually useful.

**Stack:**
- Rust backend with Tauri 2.0
- React + TypeScript frontend
- SQLite + sqlite-vec for vector similarity search
- Local embeddings via Ollama (fully offline capable)
- ~15 MB installed size

**MCP server (free, MIT licensed):**
If you use Claude Code or Cursor, 4DA exposes 30 MCP tools -- query your scored feed, check dependency advisories, surface signal chains, all from your AI assistant. Install: `npx @4da/mcp-server`

**Pricing:**
- Free forever: all 11 sources, full scoring engine, MCP server, feedback-driven scoring, CLI
- Pro ($12/mo or $99/yr): AI daily briefings, Developer DNA profiling, intelligence panels, score autopsy

**License:** FSL-1.1-Apache-2.0 (source available, converts to Apache 2.0 after 2 years). The MCP server is MIT today.

Happy to answer questions about the scoring algorithm, privacy architecture, or anything else under the hood.
```

**Why a separate technical comment:**
- PH audience is mixed. The maker comment tells the story for everyone. This comment provides depth for developers who want to evaluate seriously.
- It demonstrates technical authority without cluttering the main narrative.
- The MCP server mention gives power users a reason to engage.
- Offering to answer questions invites a comment thread, which boosts PH ranking.

---

## Engagement Strategy -- First 24 Hours

### Comments to Seed (ask a friend or alt to post)

**Comment 1 (privacy angle):**
"How do you handle the privacy aspect when fetching content from external sources? Does anything identifying get sent?"

**Your reply:** Walk through the content fetching flow. Only public API calls to HN/Reddit/etc. No user context attached. Explain the BYOK model for AI features. Point to the source code for verification.

**Comment 2 (comparison angle):**
"How does this compare to daily.dev or Feedly for staying current?"

**Your reply:** Respectful but clear differentiation. daily.dev personalizes by engagement (clicks), Feedly aggregates but doesn't score, 4DA scores against your actual codebase. Different category entirely.

**Comment 3 (skepticism angle):**
"99% rejection rate sounds aggressive. Don't you miss things?"

**Your reply:** Explain the confirmation gate (2+ independent signals). Share a concrete example of something it caught that you would have missed. Acknowledge the tuning challenge honestly.

### Response Guidelines

- Reply to every comment within 30 minutes for the first 6 hours
- Be specific and technical, never defensive
- If someone reports a bug or problem, acknowledge it publicly and fix it fast
- Upvote genuine questions from other commenters (builds goodwill)
- Never ask for upvotes directly

---

## Alt Tagline Options (ranked)

1. `Your codebase decides what you read. Not an algorithm.` (54 chars) -- RECOMMENDED
2. `All signal. No feed. Content scored against your code.` (54 chars) -- brand-forward
3. `Stop reading everything. Start knowing what matters.` (52 chars) -- emotional hook
4. `11 sources. 99% noise rejected. Your code is the filter.` (57 chars) -- proof-forward
5. `Developer intelligence that reads your codebase, not you.` (57 chars) -- privacy angle

**Selection notes:** Option 1 is recommended because it communicates the unique mechanism immediately and creates curiosity. "Your codebase decides" is a statement nobody has heard before -- it stops the scroll. Option 2 is the strongest brand line but may be too abstract for PH's mixed audience. Option 3 is the strongest emotional hook but says less about what the product actually does.

---

## Gallery Assets Checklist

**Priority order (PH shows first image as hero):**

1. [ ] **Hero image (1270x760):** Clean dark UI showing 4DA's main feed with scored content items. Relevance percentages visible. A few items showing 85%+, most greyed/rejected. Tagline overlaid: "Your codebase decides what you read."

2. [ ] **GIF or video thumbnail (1270x760):** 15-second loop: codebase scan animating -> content items streaming in -> scores appearing -> low-score items fading out, high-score items remaining. No audio needed for GIF.

3. [ ] **Screenshot -- The Mechanism (1270x760):** Score autopsy view showing the 5-axis breakdown for a single item. Each axis labeled. Shows WHY an item scored high. Caption: "Every score is explainable across 5 independent axes."

4. [ ] **Screenshot -- Before/After (1270x760):** Split screen. Left: "11 sources, 847 items/week" (wall of content). Right: "After 4DA: 12 items that matter" (clean, scored feed). Caption: "98.6% noise, removed."

5. [ ] **Screenshot -- Privacy Architecture (1270x760):** Simple diagram showing data flow. All arrows stay within "Your Machine" boundary. No arrows going to any cloud. Caption: "There is no 4DA server. Your data stays on your machine."

6. [ ] **Screenshot -- MCP Integration (1270x760):** Claude Code or Cursor terminal showing 4DA MCP tools in action. A developer querying "what breaking changes affect my deps?" Caption: "30 MCP tools for AI-assisted development."

7. [ ] **Demo video (60-90 seconds):** Install -> first scan -> scored results appearing -> score autopsy -> dismiss/save actions. Voiceover explaining the flow. End with "Free. Local. Private."

### Image Specifications
- All images: 1270x760px (PH recommended), PNG or JPG
- Dark background (#0A0A0A) matching app UI
- Text overlays in Inter font, white (#FFFFFF)
- Accent highlights in gold (#D4AF37) for key metrics
- No stock photos. Real product screenshots only.

---

## Topics / Tags (select up to 5 on Product Hunt)

1. Developer Tools (primary)
2. Privacy
3. Open Source
4. Artificial Intelligence
5. Productivity

---

## Pre-Launch Checklist

- [ ] Product page created on Product Hunt (draft mode)
- [ ] All gallery assets uploaded and ordered
- [ ] Tagline and description filled in
- [ ] Maker comment drafted and ready to paste
- [ ] First technical comment drafted and ready to paste
- [ ] Download links working for all 3 platforms
- [ ] Landing page updated with "Featured on Product Hunt" badge placeholder
- [ ] Social accounts ready to amplify (Twitter/X, LinkedIn, relevant Discord/Slack communities)
- [ ] 2-3 friends briefed to leave genuine comments in the first hour
- [ ] Email to existing users/followers scheduled for launch morning
- [ ] GitHub README updated with Product Hunt badge
- [ ] Response plan: who monitors comments, target response time (30 min)

---

## Post-Launch Actions (within 24 hours)

- [ ] Pin the technical first comment if it gets engagement
- [ ] Write a "thank you" update comment at the 12-hour mark with any interesting stats (downloads, feedback themes)
- [ ] Share the PH link on Twitter/X, LinkedIn, relevant subreddits (r/rust, r/programming, r/selfhosted)
- [ ] Respond to every single comment -- no exceptions
- [ ] If featured: update landing page with "Product of the Day" badge
- [ ] Log all feature requests and bug reports from comments into GitHub issues

---

*Last updated: March 2026. All copy is ready to paste directly into Product Hunt fields.*
