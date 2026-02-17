# 4DA Go-To-Market Strategy

**Document version:** 1.0
**Date:** 2026-02-17
**Author:** Content & Marketing Strategy
**Status:** Ready for execution

---

## Table of Contents

1. [Positioning and Messaging](#1-positioning-and-messaging)
2. [Target Personas](#2-target-personas)
3. [Channel Strategy](#3-channel-strategy)
4. [Launch Sequence](#4-launch-sequence)
5. [Pricing and Conversion Strategy](#5-pricing-and-conversion-strategy)
6. [Metrics and KPIs](#6-metrics-and-kpis)

---

## 1. Positioning and Messaging

### Core Positioning Statement

> For professional developers who waste hours scanning tech news across fragmented sources, 4DA is a privacy-first desktop intelligence app that monitors 11 sources, scores everything against your actual codebase, and rejects 99%+ as noise -- so you only see what matters to your work. Unlike Feedly, Perplexity, or manually checking Hacker News, 4DA understands your tech stack automatically and keeps all data on your machine.

### Category Creation

4DA does not fit neatly into existing categories (RSS reader, news aggregator, AI assistant). The strategic move is to **define a new category**: **Developer Intelligence**.

- **Not** a news reader (you do not browse)
- **Not** a search engine (you do not query)
- **Not** an AI assistant (it does not do tasks)
- **It is** ambient intelligence -- it watches, scores, and surfaces signal while you work

Use the phrase "developer intelligence" consistently across all channels.

### Messaging Hierarchy

**Primary message (lead with this everywhere):**
"All signal. No feed." -- 4DA monitors 11 sources, scores everything against your codebase, and rejects 99%+ as noise.

**Secondary message (use for feature-aware audiences):**
"Privacy-first. Your data never leaves your machine. BYOK. Zero telemetry." -- For developers who care about data sovereignty and do not want another cloud service reading their codebase.

**Tertiary message (use for technical/power-user audiences):**
"5-axis scoring with multi-signal confirmation gate. Auto context discovery from Cargo.toml, package.json, go.mod. Behavior learning with 30-day exponential decay." -- For developers who want to understand the mechanism, not just the promise.

### Elevator Pitches

**10-second pitch (Twitter/X bio, conference intro):**
"4DA is a desktop app that monitors HN, arXiv, Reddit, and 8 other sources, scores everything against your actual codebase, and only shows you what matters. Runs locally, zero telemetry."

**30-second pitch (Product Hunt tagline, podcast intro):**
"You skim 500+ articles a day and still miss the security advisory for a package you use. 4DA fixes that. It scans your project files -- Cargo.toml, package.json, go.mod -- builds a profile of your tech identity, then scores every piece of incoming content from 11 sources across 5 independent axes. An item needs 2+ signals to pass. 99%+ gets rejected. What survives is signal. All local, all private, BYOK."

**60-second pitch (Show HN post, conference talk opener):**
"I was spending 2-3 hours a week scanning Hacker News, Reddit, arXiv, and GitHub trending, trying to stay current. I would miss the arXiv paper directly relevant to my Rust project while reading three 'intro to React' posts. My dependency shipped a breaking change. I found out when CI broke.

So I built 4DA. It is a desktop app that runs in the background. It scans your project files, watches your Git activity, and builds a graduated domain profile -- primary stack, dependencies, detected technologies, learned interests. Then it pulls content from 11 sources and scores each item across 5 independent axes: context, interest, ACE (real-time signals), dependency, and learned behavior. An item needs 2+ independent signals to pass the confirmation gate. Typical rejection rate is 99%+.

What survives gets ranked with quality analysis (kills clickbait), novelty detection (demotes intro content if you are advanced), and intent scoring from your recent work. The result is 5-15 items per day that actually matter to your projects. All data stays local. BYOK for AI features, or fully local with Ollama. Zero telemetry."

### Competitive Positioning Matrix

| Dimension | 4DA | Feedly | Perplexity | Hacker News | RSS Readers |
|-----------|-----|--------|------------|-------------|-------------|
| **Discovery mode** | Ambient (push) | Manual (pull) | Search (pull) | Browse (pull) | Manual (pull) |
| **Personalization** | Automatic (codebase scan) | Manual keywords | Search history | None | Manual feeds |
| **Signal quality** | 5-axis + confirmation gate | Keyword match | LLM ranking | Community votes | Chronological |
| **Privacy** | Local-only, BYOK, zero telemetry | Cloud service | Cloud service | Public data | Varies |
| **Explainability** | Full score breakdown | None | None | Vote count | None |
| **Developer focus** | Codebase-aware scoring | General audience | General audience | Tech-heavy but general | General |
| **Setup time** | 3 minutes (auto-discovery) | 30+ minutes (manual curation) | None (but no personalization) | None (but no personalization) | Hours (feed curation) |
| **Cost** | Free + Pro $12/mo | $6-12/mo | $20/mo | Free | $3-5/mo |

### One-line differentiators vs each competitor

- **vs Feedly:** "Feedly needs you to curate feeds. 4DA reads your Cargo.toml."
- **vs Perplexity:** "Perplexity answers questions. 4DA answers questions you did not know to ask."
- **vs Hacker News:** "HN shows you what 500,000 people find interesting. 4DA shows you what matters to YOUR project."
- **vs RSS readers:** "RSS gives you everything from your feeds. 4DA gives you only what passes 5-axis scoring."
- **vs doing nothing:** "You are spending 2-3 hours a week manually scanning. 4DA reduces that to 5 minutes."

---

## 2. Target Personas

### Persona 1: The Overwhelmed Senior Full-Stack Developer

**Name archetype:** "Alex" -- Senior Full-Stack Developer
**Demographics:** 28-42 years old. 5-12 years experience. Works at mid-size company or late-stage startup. $120-200K salary. Uses 3-5 languages/frameworks regularly.

**Daily routine:**
- Checks HN 3-5x per day (10-15 min each time)
- Scans r/programming, r/rust, r/typescript during lunch
- Has 20+ RSS feeds but rarely opens the reader
- Gets GitHub notification emails but archives most
- Tries to read 1-2 arXiv papers per week but usually does not

**Pain points:**
- Spends 2-4 hours/week scanning sources and still feels behind
- Misses critical dependency updates buried in noise
- Reads content about technologies they already know while missing papers relevant to current work
- Context-switching cost: scanning news kills deep focus
- Every "AI newsletter" and "weekly roundup" adds more noise, not less

**Current solutions:**
- Manual HN/Reddit browsing
- 2-3 email newsletters (Morning Brew for Dev, TLDR, etc.)
- GitHub watch notifications (mostly ignored)
- Asks colleagues "did you see that post about X?"

**Trigger events (when they actively look for a solution):**
- Missed a critical CVE for a dependency they use
- Spent an entire evening reading HN and realized nothing was relevant
- Manager asked "what is the team's take on [emerging tech]?" and they had nothing
- Discovered a relevant tool 3 months after it was released

**Messaging angle:**
Lead with **time savings and relevance**. "Stop spending 2 hours a week scanning sources. 4DA does it in the background and shows you 5-15 items that actually matter to your React/Rust/Go stack."

**Objection handling:**
- "I already use Feedly" -- "Feedly needs manual curation. 4DA reads your package.json."
- "I do not trust AI filtering" -- "Every score is explainable. You can see exactly why each item was surfaced."
- "I do not want another subscription" -- "Free tier includes all 11 sources and the full scoring engine. Pro is only for AI briefings."

### Persona 2: The Open Source Maintainer

**Name archetype:** "Jordan" -- Open Source Maintainer
**Demographics:** 25-45 years old. Maintains 1-5 packages with 500-50K stars. Often works independently or at companies that value OSS. Active on GitHub and Twitter/X.

**Daily routine:**
- Triages GitHub issues/PRs morning and evening
- Monitors mentions of their project across Reddit, HN, Twitter
- Tracks competing/complementary projects for feature parity
- Reads release notes of upstream dependencies
- Writes blog posts and changelogs for their project

**Pain points:**
- Missing discussions about their project on HN/Reddit/Lobsters
- Late to know when a dependency ships a breaking change
- Ecosystem moves fast -- competing projects launch features they should know about
- Reverse mentions are scattered across platforms with no aggregation
- Spending more time monitoring than actually maintaining

**Current solutions:**
- GitHub notifications (overwhelming volume)
- Google Alerts for project name (poor quality)
- Manual Twitter/X searches
- Community members ping them on Discord

**Trigger events:**
- Their project was discussed on HN and they missed it entirely
- A competitor launched a feature that users had requested months ago
- An upstream dependency broke their build and users filed 20 issues before they noticed

**Messaging angle:**
Lead with **ecosystem awareness and reverse mentions**. "4DA tracks when your projects are mentioned across HN, Reddit, and Lobsters. Know about breaking upstream changes before your users file issues."

**Objection handling:**
- "I use GitHub notifications" -- "GitHub only sees GitHub. 4DA sees HN, Reddit, arXiv, and 8 more sources."
- "I can just search Twitter" -- "4DA does it continuously in the background and scores relevance to YOUR project specifically."

### Persona 3: The Tech Lead / Engineering Manager

**Name archetype:** "Morgan" -- Tech Lead / Engineering Manager
**Demographics:** 32-50 years old. Manages 4-15 engineers. Responsible for technology decisions and team roadmap. $150-250K salary. Reads less code, more strategy.

**Daily routine:**
- Morning standup, then context-switches between code review, architecture, and planning
- Tries to stay current on technologies the team uses but lacks time
- Reads 1-2 newsletters over coffee
- Attends vendor demos and conference talks to evaluate tools
- Makes "should we adopt X?" decisions with incomplete information

**Pain points:**
- Needs to make technology decisions but lacks time for deep research
- Team asks "should we use X?" and they cannot give an informed answer
- Responsible for security posture but misses CVE announcements
- The higher they go, the less they code, and the more they fall behind
- Existing newsletters and feeds are too generic -- not tailored to their team's stack

**Current solutions:**
- ThoughtWorks Radar (quarterly, too slow)
- Ask senior devs on the team
- Google searches when a decision is urgent
- Conference talks (annual)

**Trigger events:**
- Approved adoption of a framework that was deprecated 2 months later
- Board/exec asked about a technology trend and they looked uninformed
- A security incident traced to a known CVE they should have caught

**Messaging angle:**
Lead with **decision intelligence and team credibility**. "4DA surfaces what matters to your team's stack. Make informed technology decisions without spending hours researching. The AI briefing gives you a daily executive summary."

**Objection handling:**
- "My team can just tell me" -- "Your team is biased toward what they know. 4DA covers blind spots."
- "I have newsletters for this" -- "Newsletters are one-size-fits-all. 4DA scores against YOUR team's actual dependencies."
- "$12/mo seems expensive" -- "One avoided bad technology decision saves months of migration. One caught CVE saves an incident response."

### Persona 4: The AI/ML Engineer

**Name archetype:** "Riley" -- AI/ML Engineer
**Demographics:** 25-40 years old. Works with PyTorch, transformers, CUDA, or similar. Publishes or reads papers. $130-220K salary. Follows 50+ researchers on Twitter/X.

**Daily routine:**
- Checks arXiv daily for new papers in cs.LG, cs.CL, cs.CV
- Follows 20+ ML researchers on Twitter/X
- Monitors Hugging Face for new model releases
- Reads conference proceedings (NeurIPS, ICML, ICLR)
- Experiments with new architectures in notebooks

**Pain points:**
- arXiv publishes 100+ papers/day in their field -- impossible to read all abstracts
- Critical papers surface on Twitter weeks after publication
- New model releases on Hugging Face are hard to discover unless someone tweets about them
- "Keeping up with AI" is genuinely a full-time job
- Most AI newsletters are too surface-level

**Current solutions:**
- arXiv daily digest (overwhelming volume)
- Twitter/X lists for ML researchers
- Papers With Code
- Semantic Scholar alerts
- Community Discord channels

**Trigger events:**
- A competitor published a paper using a technique they should have found 2 months ago
- Spent a week implementing something that a new library already solved
- Manager asked for "state of the art" summary and they missed 3 key papers

**Messaging angle:**
Lead with **arXiv intelligence and research relevance**. "4DA scores arXiv papers against your actual ML stack -- PyTorch version, model architecture, training framework. Stop reading 100 abstracts to find the 3 that matter to your work."

**Objection handling:**
- "Semantic Scholar already does this" -- "Semantic Scholar is citation-based. 4DA scores against YOUR codebase and dependencies, not academic graphs."
- "I follow the right people on Twitter" -- "People you follow share what interests them, not what is relevant to your specific project."

---

## 3. Channel Strategy

### Channel Priority Matrix

Channels ranked by **expected ROI for a solo bootstrapped developer** with limited time and budget.

#### Tier 1: High ROI, Low Cost (Do These First)

| Channel | Type | Est. Cost | Est. Reach | Why Priority |
|---------|------|-----------|------------|--------------|
| **Show HN** | Earned | $0 | 50-500K views | Target audience IS on HN. One post can make or break launch. |
| **Product Hunt** | Earned | $0 | 20-100K views | Developer tools category is strong. Good for credibility. |
| **GitHub README + Stars** | Owned | $0 | Compounds over time | The README IS the sales page for developers. Stars = social proof. |
| **r/programming, r/rust, r/typescript** | Earned | $0 | 5-50K views per post | Subreddit-specific posts targeting each persona. |
| **Personal Twitter/X** | Owned | $0 | Grows with consistency | Build-in-public thread format. Dev audience is native here. |
| **Dev.to / Hashnode articles** | Earned | $0 | 2-10K views per article | Long-form content that ranks in search. Repurpose for blog. |

**Estimated combined reach from Tier 1 alone:** 100-500K impressions in launch month.
**Time investment:** 15-20 hours/week for 4 weeks.

#### Tier 2: Medium ROI, Some Cost (Add at Week 3-4)

| Channel | Type | Est. Cost | Est. Reach | Why Priority |
|---------|------|-----------|------------|--------------|
| **Newsletter (Substack/Buttondown)** | Owned | $0-10/mo | Compounds | Email list is the most durable owned channel. |
| **YouTube demos** | Owned | $0-100 | 1-10K views | Screen recordings showing 4DA in action. Evergreen search traffic. |
| **Podcast guest appearances** | Earned | $0 | 5-50K per episode | Changelog, Rustacean Station, devtools.fm. Personal story angle. |
| **MCP marketplace listing** | Owned | $0 | Unknown (new channel) | Direct distribution to Claude Code/Cursor users. |
| **Conference lightning talks** | Earned | Travel cost | 200-2000 per talk | Local meetups first, then RustConf, JSConf, etc. |

#### Tier 3: Paid Amplification (Add at $500+/mo Budget)

| Channel | Type | Est. Cost | Est. Reach | Why Priority |
|---------|------|-----------|------------|--------------|
| **Reddit Ads (r/programming)** | Paid | $5-15/day | 10-50K impressions | Highly targeted, dev-specific subreddits. |
| **Twitter/X Ads** | Paid | $10-30/day | 20-100K impressions | Target followers of @rustlang, @typescript, @arxiv. |
| **Google Ads (branded + category)** | Paid | $10-20/day | Varies | Capture "developer news aggregator" searches. |
| **Newsletter sponsorships** | Paid | $200-2000/placement | 10-50K per send | TLDR, Pointer, Bytes, Rust Weekly, etc. |
| **Carbon Ads** | Paid | $100-500/mo | Targets dev sites | Ads on dev documentation sites. |

### Channel-Specific Strategies

#### Show HN (CRITICAL -- plan this carefully)

**Post format:**
```
Show HN: 4DA -- Privacy-first developer intelligence (11 sources, 5-axis scoring)
```

**Post body must include:**
- What it does in 2 sentences
- Why you built it (personal story -- "I was spending 3 hours a week...")
- How it works technically (developers respect technical depth)
- Privacy angle (HN audience cares deeply about this)
- Link to live demo or screenshots
- License (FSL-1.1-Apache-2.0 -- explain the Apache 2.0 conversion)
- Invitation to try it and give feedback

**Timing:** Tuesday or Wednesday, 9-10am ET (peak HN traffic for Show HN).

**Preparation:** Have 3-5 friends/colleagues ready to upvote and leave genuine comments in the first 30 minutes. Respond to EVERY comment within 1 hour. Be in front of your computer for 6+ hours after posting.

#### Product Hunt

**Category:** Developer Tools
**Tagline:** "All signal. No feed. -- Privacy-first developer intelligence."

**Assets needed:**
- 5 screenshots (relevance view, Developer DNA, briefing, score autopsy, settings)
- 1 demo video (60-90 seconds)
- Maker comment explaining the "why"
- 3-5 hunter comments from beta testers

**Timing:** Launch on a Tuesday or Wednesday (highest traffic). Avoid Mondays (competition from well-funded launches) and Fridays (low traffic).

#### Reddit Strategy

**Do NOT cross-post the same content.** Each subreddit needs a tailored post.

| Subreddit | Post angle | Format |
|-----------|-----------|--------|
| r/programming | Technical deep-dive on 5-axis scoring | Long text post with architecture diagram |
| r/rust | "Built with Tauri 2.0 + Rust" -- focus on Rust stack | Technical post, show Rust code snippets |
| r/typescript | MCP integration for Claude Code/Cursor | Focus on TypeScript frontend, MCP tools |
| r/selfhosted | "Local-first, zero telemetry alternative to cloud aggregators" | Privacy angle, comparison table |
| r/machinelearning | arXiv scoring for ML papers | Focus on research paper filtering |
| r/sideproject | "Solo dev, bootstrapped, 8 months of building" | Personal story angle |

**Timing:** Post to each subreddit on different days (not all at once -- looks like spam).

---

## 4. Launch Sequence

### Pre-Launch Phase (Weeks -2 to 0)

**Goal:** Build anticipation and a waitlist of 200-500 people.

**Week -2:**
- [ ] Finalize landing page at 4da.ai (add real screenshots, pricing section, email capture)
- [ ] Set up email capture (Buttondown or Substack) -- "Get notified when 4DA launches"
- [ ] Create Twitter/X thread: "I have been building something for 8 months. Here is why."
- [ ] Write the Show HN post draft (do not post yet -- review and refine)
- [ ] Record 90-second demo video (screen recording with voiceover)
- [ ] Prepare Product Hunt listing (screenshots, video, copy)
- [ ] Reach out to 5-10 dev tool podcast hosts for guest spots

**Week -1:**
- [ ] Post 3-4 build-in-public tweets showing features (one per day)
- [ ] Publish first blog post: "Why I built a developer intelligence tool" (Dev.to + personal blog)
- [ ] Send "launching next week" email to waitlist
- [ ] Share demo video on Twitter/X and LinkedIn
- [ ] DM 10-20 developer influencers with early access offer
- [ ] Prepare GitHub README for launch (ensure it sells, not just documents)
- [ ] Test all download links and installation flows on all 3 platforms

### Launch Week (Week 0)

**Day 1 (Tuesday):**
- [ ] 6:00 AM ET: Submit Show HN post
- [ ] 6:15 AM ET: Submit Product Hunt listing (or have a hunter do it)
- [ ] 7:00 AM ET: Tweet announcement with demo video
- [ ] 7:30 AM ET: Post to r/programming (technical angle)
- [ ] 8:00 AM ET: Send launch email to waitlist
- [ ] All day: Monitor and respond to every HN comment, PH comment, Reddit comment
- [ ] Evening: Post daily metrics on Twitter/X (build in public)

**Day 2 (Wednesday):**
- [ ] Post to r/rust (Tauri/Rust technical angle)
- [ ] Post to r/selfhosted (privacy angle)
- [ ] Respond to all new comments from Day 1
- [ ] Tweet specific feature highlights (Developer DNA, Score Autopsy)
- [ ] Reach out to anyone who tweeted about the launch -- thank them, ask for feedback

**Day 3 (Thursday):**
- [ ] Post to r/machinelearning (arXiv angle)
- [ ] Post to r/sideproject (personal story angle)
- [ ] Publish blog post: "How 4DA's 5-axis scoring works" (technical deep-dive)
- [ ] Share technical post on HN as a comment/follow-up if the Show HN is still active

**Day 4 (Friday):**
- [ ] Post to r/typescript (MCP integration angle)
- [ ] Tweet a "launch week stats" thread (downloads, feedback themes, what surprised you)
- [ ] Send "thank you + first week learnings" email to waitlist

**Day 5-7 (Weekend):**
- [ ] Compile all feedback into categories
- [ ] Fix any critical bugs reported during launch
- [ ] Plan Week 1 content based on questions and feedback received

### Post-Launch Sustain Phase (Weeks 1-4)

**Week 1:**
- [ ] Publish blog post addressing top 3 questions from launch week
- [ ] Record and upload YouTube walkthrough (5-10 min)
- [ ] Submit to developer newsletters for inclusion (TLDR, Pointer, Bytes)
- [ ] First podcast appearance (if booked during pre-launch)
- [ ] Start Reddit Ads on r/programming at $5/day (if budget allows)

**Week 2:**
- [ ] Publish "4DA vs Feedly vs Perplexity" comparison post
- [ ] Tweet daily tips ("Did you know 4DA can..." format)
- [ ] Release first update based on launch feedback (ship fast, build trust)
- [ ] Reach out to dev tool review bloggers/YouTubers

**Week 3:**
- [ ] Publish blog post: "How I bootstrapped a desktop app as a solo dev"
- [ ] Announce Discord/GitHub Discussions community
- [ ] Start collecting testimonials from early users
- [ ] Add testimonials to landing page

**Week 4:**
- [ ] Publish "30-day launch retrospective" (build in public)
- [ ] Share metrics transparently (downloads, DAU, conversion rate)
- [ ] Evaluate paid channel ROI, adjust budget
- [ ] Plan content calendar for months 2-3

### Growth Phase (Ongoing)

**Content flywheel:**
1. Users share Developer DNA on Twitter/X (organic virality)
2. Blog posts rank for "developer news aggregator," "privacy-first RSS alternative"
3. MCP marketplace drives Claude Code/Cursor users
4. GitHub stars compound (social proof)
5. Podcast appearances drive spikes; blog SEO drives steady flow

---

## 5. Pricing and Conversion Strategy

### Tier Structure

| Feature | Free | Pro ($12/mo or $99/yr) |
|---------|------|------------------------|
| 11 source adapters | Yes | Yes |
| 5-axis scoring engine | Yes | Yes |
| Feed UI with filtering | Yes | Yes |
| Basic signal detection | Yes | Yes |
| CLI binary | Yes | Yes |
| MCP server (basic tools) | Yes | Yes |
| **AI Daily Briefings** | Blurred preview | **Full access** |
| **Developer DNA** | Blurred preview | **Full access** |
| **Intelligence Panels** | Blurred preview | **Full access** |
| **Signal Chains** | Limited | **Full access** |
| **Knowledge Gap Analysis** | Limited | **Full access** |
| **Score Autopsy (detailed)** | Basic | **Full 5-axis breakdown** |
| **Priority support** | Community only | **Direct access** |

### Conversion Funnel Design

```
Download (Free)
    |
    v
First scan shows scored results (immediate value -- < 3 minutes)
    |
    v
Blurred AI briefing preview with "Unlock with Pro" CTA
    |
    v
User sees value of scoring, wants deeper intelligence
    |
    v
Developer DNA preview (blurred) -- curiosity gap
    |
    v
Trial trigger: "Try Pro free for 14 days"
    |
    v
Pro trial (no credit card required for first 7 days)
    |
    v
Day 7: "Your trial is halfway done" email with usage stats
    |
    v
Day 12: "2 days left" email showing what they would lose
    |
    v
Conversion or downgrade to Free
```

### Upgrade Triggers (In-App Moments)

These are the specific moments when a user is most likely to upgrade:

1. **First briefing preview**: After first scan, show a blurred daily briefing with a "This is what Pro users see every morning" CTA.
2. **Developer DNA curiosity**: Show 30% of the DNA profile with the rest blurred. Identity-driven FOMO.
3. **Score autopsy on a high-relevance item**: Show basic score, blur the 5-axis breakdown. "Want to know WHY this matters? Upgrade."
4. **Knowledge gap detection**: Show that gaps exist but blur the specific packages. "You have 3 blind spots in your dependencies."
5. **After 7 days of daily use**: They have formed a habit. Surface the upgrade at peak engagement.

### Pricing Page Best Practices

- **Lead with Free**: "4DA is free. 11 sources, full scoring engine, unlimited use."
- **Position Pro as intelligence layer**: "Pro adds the AI intelligence layer that turns raw scores into actionable briefings."
- **Annual discount is visible**: "$12/mo or $99/yr (save 31%)" -- the annual savings should be immediately clear.
- **Show what they miss, not what they get**: Instead of listing Pro features, show a blurred briefing with "Unlock" overlaid. Visual > bullet points.
- **No feature comparison table on pricing page**: It makes Free look sufficient. Instead, show the blurred previews.
- **Money-back guarantee**: "30-day money-back guarantee. No questions asked." Reduces purchase anxiety.

### Annual vs Monthly Optimization

**Goal:** 60%+ of Pro users on annual plans.

**Tactics:**
- Default the pricing toggle to "Annual" (show monthly as the alternative)
- Show annual as "$8.25/mo billed annually" (smaller number, monthly framing)
- At month 3 of monthly billing, offer "Switch to annual and save $45/year"
- Annual users get early access to new Pro features (exclusivity incentive)

### Pricing Anchoring

On the pricing page, show three columns:

| | Free | Pro | Team (Coming Soon) |
|--|------|-----|-------------------|
| Price | $0 | $12/mo | $29/user/mo |
| Sources | 11 | 11 | 11 |
| Scoring | Full | Full | Full |
| AI Briefings | -- | Daily | Daily + Team digest |
| Developer DNA | -- | Personal | Team DNA |
| Status | Available | Available | Waitlist |

The "Team" column is a placeholder that:
1. Makes Pro look like a bargain by contrast
2. Captures enterprise interest early (waitlist emails)
3. Validates demand before building team features

---

## 6. Metrics and KPIs

### North Star Metric

**Daily Active Scored Users (DASU):** Number of unique users who open 4DA and have at least one scored item visible in a given day.

**Why this metric:**
- Combines activation (they set up sources), engagement (they open the app), and value delivery (scoring is working)
- Directly correlates with retention and conversion
- Does not over-index on vanity metrics (downloads, page views)

### Leading Indicators (predict future success)

| Metric | Target (Month 1) | Target (Month 3) | How to Measure |
|--------|-------------------|-------------------|----------------|
| Email waitlist signups | 500 | 2,000 | Buttondown/Substack |
| GitHub stars | 200 | 1,000 | GitHub API |
| Daily downloads | 20-50 | 50-100 | GitHub Releases download count |
| Activation rate (download to first scan) | 60%+ | 70%+ | In-app event (local only) |
| Day 7 retention | 30%+ | 40%+ | Local telemetry-free timestamp check |
| Show HN upvotes | 100+ | N/A | Manual check |
| Product Hunt upvotes | 200+ | N/A | PH dashboard |

### Lagging Indicators (confirm success after the fact)

| Metric | Target (Month 1) | Target (Month 3) | How to Measure |
|--------|-------------------|-------------------|----------------|
| Monthly Active Users (MAU) | 500 | 2,000 | Rough estimate from downloads and retention |
| Free to Pro conversion rate | 2-3% | 4-6% | License key activations |
| Pro Monthly Recurring Revenue (MRR) | $120-360 | $960-2,400 | Payment processor |
| Net Promoter Score (NPS) | 40+ | 50+ | Quarterly survey |
| Churn rate (Pro monthly) | <10% | <7% | Payment processor |
| Organic search traffic to 4da.ai | 100/mo | 1,000/mo | Vercel/Cloudflare analytics |

### Revenue Projections (Conservative)

| Month | Downloads (cumulative) | MAU | Pro Subscribers | MRR |
|-------|----------------------|-----|-----------------|-----|
| 1 | 500-1,000 | 200-400 | 5-15 | $60-180 |
| 3 | 2,000-5,000 | 800-2,000 | 30-80 | $360-960 |
| 6 | 5,000-15,000 | 2,000-6,000 | 80-240 | $960-2,880 |
| 12 | 15,000-50,000 | 5,000-15,000 | 200-600 | $2,400-7,200 |

**Break-even analysis:** At $12/mo Pro pricing, 4DA needs approximately 50 Pro subscribers to cover basic infrastructure costs (domain, signing certificates, CI/CD). At 200 Pro subscribers ($2,400/mo MRR), it sustains a solo developer full-time in most markets.

### Dashboard Recommendations

**Primary dashboard (check daily):**
- Downloads (today, 7-day, 30-day)
- GitHub stars
- Active GitHub issues and their sentiment
- Social mentions (Twitter/X, Reddit, HN)

**Secondary dashboard (check weekly):**
- Activation rate trend
- Retention cohorts
- Pro conversion rate
- MRR and subscriber count
- Top traffic sources to 4da.ai

**Tools (free/cheap for solo dev):**
- GitHub Releases API for download counts
- Vercel Analytics for landing page
- Buttondown for email metrics
- A simple SQLite database tracking key metrics (build this yourself -- you are a developer, and it is on-brand for a privacy-first product)

---

## Appendix: Key Assets Needed Before Launch

| Asset | Status | Priority |
|-------|--------|----------|
| Landing page with real screenshots | Needs screenshots | P0 |
| 90-second demo video | Not started | P0 |
| Show HN post draft | Not started | P0 |
| Product Hunt listing | Not started | P0 |
| Email capture on landing page | Not started | P0 |
| GitHub README (launch-ready) | Exists, needs screenshots | P1 |
| Blog post: "Why I built 4DA" | Not started | P1 |
| Blog post: "How 5-axis scoring works" | Not started | P1 |
| Pricing page on landing page | Not started | P1 |
| 5 product screenshots | Not started | P0 |
| Developer DNA shareable card | Not started | P2 |

---

*This document should be reviewed and updated after launch week based on actual performance data.*
