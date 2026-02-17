# 4DA Email Sequences

**Purpose:** Waitlist nurture, launch announcement, and post-download onboarding emails
**Tone:** Developer-native, concise, no jargon. Minimal and confident.
**Constraint:** Each email under 300 words.

---

## Email 1: Welcome to the Waitlist

**Trigger:** User submits email on 4da.ai waitlist form
**Timing:** Immediate (automated)

---

**Subject:** You are on the list.

**Preview text:** 4DA is almost ready. Here is what you are getting into.

---

You signed up for 4DA. Good call.

Here is what we are building: a desktop app that monitors 11 sources -- Hacker News, arXiv, Reddit, GitHub, Product Hunt, YouTube, RSS, and more -- scores every piece of content against your actual codebase, and rejects 99%+ as noise. What survives is signal.

It reads your `Cargo.toml`, `package.json`, `go.mod`, and Git history locally. Builds a graduated profile of your tech identity. Then evaluates incoming content across 5 independent axes. An item needs 2+ signals to pass the confirmation gate. Everything else gets discarded.

All data stays on your machine. Zero telemetry. BYOK for AI features, or fully local with Ollama. We architecturally cannot see your data because we never receive it.

**What to expect:**

- We are in the final stretch before public launch
- You will get an email the day it goes live with direct download links
- No spam between now and then -- just the launch email and maybe one update if something significant changes

If you want to see what we are building before launch day, the source is public: [github.com/runyourempire/4DA](https://github.com/runyourempire/4DA)

Talk soon.

-- The 4DA team

P.S. -- If you use Claude Code or Cursor, we are also shipping an MCP server with 26 tools that gives your AI assistant access to the same scored intelligence feed. MIT licensed. More on that at launch.

---

## Email 2: Launch Day

**Trigger:** Launch day (coordinated with Show HN, Product Hunt, Reddit posts)
**Timing:** 8:00 AM ET on launch Tuesday

---

**Subject:** 4DA is live. Download it now.

**Preview text:** 11 sources. 5-axis scoring. Zero telemetry. Free.

---

4DA is live. You can download it right now.

**Download:**

- [Windows (.msi installer)](https://github.com/runyourempire/4DA/releases/latest)
- [macOS (.dmg -- Apple Silicon + Intel)](https://github.com/runyourempire/4DA/releases/latest)
- [Linux (.AppImage / .deb)](https://github.com/runyourempire/4DA/releases/latest)

**What you get:**

- **11 source adapters** -- HN, arXiv, Reddit, GitHub, Product Hunt, YouTube, RSS, and more
- **5-axis scoring engine** -- Context, Interest, ACE, Dependency, and Learned axes independently evaluate every item
- **Confirmation gate** -- an item needs 2+ independent signals to pass. 99%+ gets rejected.
- **Auto context discovery** -- scans your project files and Git activity to understand your stack. No configuration.
- **Behavior learning** -- save and dismiss actions train the scoring with 30-day exponential decay
- **MCP server** -- 26 tools for Claude Code and Cursor. Your AI assistant gets signal too.
- **Zero telemetry** -- no analytics, no tracking, no account required. Your data stays yours.

The free tier is not a demo. It is the full scoring engine with all 11 sources, the CLI, the MCP server, and behavior learning. Most developers will never need more.

Setup takes under 3 minutes. Point it at a project directory and it builds your profile automatically.

[Download 4DA -- Free](https://github.com/runyourempire/4DA/releases/latest)

-- The 4DA team

P.S. -- We launched on [Show HN](https://news.ycombinator.com) and [Product Hunt](https://producthunt.com) today. If 4DA earns your upvote, we would appreciate it.

---

## Email 3: Onboarding Day 1

**Trigger:** First download detected (or 2 hours after download link click)
**Timing:** Same day as download

---

**Subject:** Your first 3 minutes with 4DA

**Preview text:** Add a project, run a scan, see what your codebase surfaces.

---

You downloaded 4DA. Here is how to get value out of it immediately.

**Step 1: Add a project directory**

Open Settings and add one or more project directories. 4DA's ACE scanner will read your manifest files -- `Cargo.toml`, `package.json`, `go.mod`, `requirements.txt` -- and your recent Git activity to build a technology profile.

It does not send any of this data anywhere. The profile is built and stored locally.

**Step 2: Run your first scan**

4DA will pull content from all 11 sources and score each item against your profile. Your first batch of results should appear within 2-3 minutes. What you see has already passed the confirmation gate -- 2+ independent relevance signals.

**Step 3: Add API keys (optional, BYOK)**

For AI-powered features like daily briefings and score analysis, add your API key in Settings. 4DA supports Anthropic, OpenAI, or fully local inference with Ollama if you prefer to keep everything offline.

Your keys are stored locally in `data/settings.json`. Never transmitted. Never stored remotely.

**What to expect on Day 1:**

The scoring engine starts cold. It does not know your behavior yet. The initial results come from your codebase profile and explicit interests. As you save and dismiss items over the next few days, the Learned axis kicks in and scoring gets sharper.

Give it a few sessions. It gets better.

-- The 4DA team

---

## Email 4: Onboarding Day 3

**Trigger:** 3 days after first download
**Timing:** Morning (9:00 AM local time if available, otherwise 9:00 AM ET)

---

**Subject:** Your scoring is learning.

**Preview text:** Every save and dismiss trains the 5th axis. Here is how to accelerate it.

---

You have been using 4DA for a few days. The scoring engine is already learning from you.

**How behavior learning works:**

Every time you save an item, 4DA reinforces the signals that surfaced it. Every time you dismiss one, it suppresses those signals for future scoring. This is the Learned axis -- the 5th scoring dimension -- and it uses 30-day exponential decay so recent actions always matter more than old ones.

The more you interact, the sharper the scoring gets. You do not need to configure anything. Just use it.

**Tip: Add more project directories**

If you work on multiple projects, add all of them. ACE builds a composite technology profile across your entire codebase surface. A Rust side project and a React production app together give 4DA a more complete picture of what matters to you. More context means better scoring.

Open Settings and add any directory with manifest files or Git history.

**If you use Claude Code or Cursor:**

4DA ships an MCP server with 26 tools. Install it with one command:

```
npx @4da/mcp-server
```

Then ask your AI assistant questions like "What breaking changes affect my dependencies this week?" and it draws from your scored, filtered intelligence feed -- not a generic internet search. Grounded, relevant, real-time.

The MCP server is MIT licensed and always free.

How is scoring working for you so far? Reply to this email -- we read every response.

-- The 4DA team

---

## Email 5: Onboarding Day 7

**Trigger:** 7 days after first download
**Timing:** Morning (9:00 AM local time if available, otherwise 9:00 AM ET)

---

**Subject:** Your first week in signal.

**Preview text:** What 4DA learned about your stack -- and what Pro unlocks next.

---

One week with 4DA. Here is what happened:

Your ACE scanner built a technology profile from your codebase. The scoring engine processed thousands of items from 11 sources and rejected 99%+ of them. The Learned axis has been training on your saves and dismissals. Scoring should be noticeably sharper than Day 1.

That is the free tier working as designed. All 11 sources. Full 5-axis scoring. Behavior learning. The MCP server. The CLI. No expiration. No catch.

**What developers are saying:**

> "I found out about a breaking change in a dependency three days before it hit my production. 4DA surfaced the GitHub issue because it knew my Cargo.toml."

> "I used to spend 45 minutes every morning skimming HN and Reddit. Now I open 4DA, read the 3-5 items that matter, and start coding."

**What Pro adds:**

There is a layer beyond scoring. Pro unlocks AI-powered intelligence features that turn raw signals into synthesized insight:

- **AI Daily Briefings** -- wake up to a written digest of what matters to your stack, generated by AI that understands your profile
- **Developer DNA** -- see your technology identity as a living, weighted graph. Primary stack, secondary tools, exploration zone, dependency surface. Watch it evolve over time.
- **Score Autopsy** -- full 5-axis breakdown explaining exactly why each item scored the way it did
- **Intelligence Panels** -- trend detection, dependency monitoring, and strategic signals across your technology surface

$12/month or $99/year. You bring your own API key. Pro unlocks the intelligence layer built on top.

[Start Pro -- $12/month](https://4da.ai/#pricing)

No pressure. The free tier is a complete product. Pro is for developers who want synthesis, not just signal.

-- The 4DA team

---

## Appendix: Email Sequence Summary

| Email | Trigger | Timing | Goal |
|-------|---------|--------|------|
| 1. Welcome | Waitlist signup | Immediate | Confirm, introduce, set expectations |
| 2. Launch Day | Launch day | 8:00 AM ET | Drive downloads |
| 3. Onboarding Day 1 | First download | Same day | Activate: first project, first scan, API keys |
| 4. Onboarding Day 3 | 3 days post-download | Morning | Engage: behavior learning, more projects, MCP |
| 5. Onboarding Day 7 | 7 days post-download | Morning | Retain and convert: recap, social proof, Pro CTA |

### Metrics to Track

| Metric | Target |
|--------|--------|
| Welcome email open rate | 60%+ (high intent -- they just signed up) |
| Launch email open rate | 50%+ |
| Launch email click-to-download rate | 25%+ |
| Onboarding Day 1 open rate | 45%+ |
| Onboarding Day 3 open rate | 35%+ |
| Onboarding Day 7 open rate | 30%+ |
| Day 7 Pro CTA click rate | 5-10% |

### Voice Guidelines

- Write like you are messaging a colleague, not a customer
- Use "you" and "we" -- never "dear user" or "valued subscriber"
- Technical specificity builds trust: name the files, name the axes, show the command
- No superlatives ("amazing," "incredible," "revolutionary") -- let the mechanism speak
- No manufactured urgency -- developers see through it and it destroys credibility
- Every email should be useful even if they never upgrade

---

*All emails are ready for implementation in Buttondown or any transactional email provider. Subject lines and preview text are optimized for developer email clients (plain text friendly, no HTML-dependent formatting required).*
