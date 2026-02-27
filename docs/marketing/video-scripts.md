# 4DA Video Scripts -- Production Ready

Three launch videos: Product Launch (3-5 min), Demo Walkthrough (8-10 min), Viral Format (2-3 min + Shorts).

---

## Video 1: Product Launch (3-5 min)

### Metadata

| Field | Value |
|-------|-------|
| **Duration** | 3:00 - 5:00 |
| **Tone** | Confident, technical, direct |
| **Format** | Screen recordings with voiceover, brief talking head intro/outro |
| **Goal** | Establish what 4DA is, how it works, why it matters. Primary launch video. |

### Title Options

1. **"4DA: Your Codebase Decides What You Read" -- Product Launch** (recommended)
2. **"I Built a Tool That Rejects 99% of Developer Content"**
3. **"Stop Scrolling HN. Let Your Codebase Filter It."**

### Script

```
[0:00 - 0:15] HOOK
VISUAL: 4DA interface showing 5 high-scoring items in a clean dark UI.

SCRIPT:
"This is every piece of tech content that mattered to my work this
week. Five items. Out of eight hundred. Everything else was noise,
and a tool called 4DA rejected it for me. Here's how it works."

[0:15 - 0:45] THE PROBLEM
VISUAL: Split screen -- left side scrolling HN/Reddit/arXiv endlessly,
right side shows notification badges and unread counts.

SCRIPT:
"If you're a developer, you know the drill. Hacker News, Reddit,
arXiv, GitHub Trending, newsletters, YouTube -- two hours every
morning before you write a line of code. And after all that scrolling,
maybe two or three things were actually relevant to what you're
building. The signal-to-noise ratio is less than 2%. The rest is
content that's interesting but has nothing to do with your work."

[0:45 - 1:30] THE MECHANISM
VISUAL: Show 4DA scanning a project. ACE discovering Cargo.toml,
package.json. The pipeline animation: Codebase -> ACE -> Scoring -> Signal.

SCRIPT:
"4DA takes a fundamentally different approach. Instead of filtering by
topic or trending metrics, it scores everything against your actual
codebase.

When you first launch 4DA, the Autonomous Context Engine -- we call it
ACE -- scans your local projects. It reads your Cargo.toml, your
package.json, your go.mod, your Git history. It builds a graduated
technology profile. Not just 'you use React' -- it's 'you're deep in
React 19 with Server Components, moderate in TypeScript, exploring Rust.'

Then every piece of content from 11 sources gets scored across 5
independent axes: Context match, Interest alignment, ACE project signals,
Dependency relevance, and Learned behavior.

An item needs 2 or more independent signals to pass the confirmation
gate. One axis match is treated as coincidence. Two or more is signal."

[1:30 - 2:30] DEMO
VISUAL: Screen recording showing scored items, clicking into Score Autopsy
view, showing the 5-axis breakdown for a high-scoring item, then a
low-scoring item that got rejected.

SCRIPT:
"Let me show you what this looks like in practice.

[Show high-scoring item] This arXiv paper about vector similarity
optimization scored 87. Context axis flagged it because we use sqlite-vec.
Dependency axis confirmed it because the paper references a library in
our Cargo.toml. Two independent signals. It passed.

[Show rejected item] This Reddit post about React state management?
Interesting, but our React usage is limited to the frontend shell.
Only the Interest axis triggered, and one signal isn't enough. Rejected.

That's the confirmation gate. It's aggressive by design. 99% rejection
means the 1% that survives is genuinely relevant to your work."

[2:30 - 3:15] PRIVACY + PRICING
VISUAL: Architecture diagram showing data flow staying local. Then
pricing comparison (Free vs Pro).

SCRIPT:
"Here's the part that matters to anyone who's going to scan their
codebase: everything runs locally. 4DA is a desktop app built in Rust
with Tauri. Your project files, your reading behavior, your developer
profile -- none of it leaves your machine. Zero telemetry. No account
required. BYOK for AI features, or fully local with Ollama.

The free tier is real. All 11 sources, the full 5-axis scoring engine,
behavior learning, the MCP server with 30 tools for Claude Code and
Cursor, and the CLI. It's not a trial. Most developers will never need
more.

Pro is twelve dollars a month. It adds AI daily briefings, Developer
DNA profiling, Score Autopsy breakdowns, and intelligence panels. You
bring your own API key."

[3:15 - 3:45] CTA
VISUAL: Download page showing Windows/macOS/Linux options. 4DA logo.
"All signal. No feed." tagline.

SCRIPT:
"4DA is available now for Windows, macOS, and Linux. About 15 megabytes.
Setup takes under 3 minutes. You'll see your first scored content within
60 seconds.

Link in the description. All signal. No feed."
```

### YouTube Description

```
4DA scores content from 11 sources against your actual codebase and
rejects 99%+ as noise. Privacy-first. Runs locally. Free.

Download 4DA: https://4da.ai
GitHub: https://github.com/runyourempire/4DA

Timestamps:
0:00 What survived this week
0:15 The information overload problem
0:45 How 4DA works (ACE + 5-axis scoring)
1:30 Live demo
2:30 Privacy model + pricing
3:15 Download

#4DA #DeveloperTools #PrivacyFirst #Rust #Tauri #AllSignalNoFeed
```

---

## Video 2: Demo Walkthrough (8-10 min)

### Metadata

| Field | Value |
|-------|-------|
| **Duration** | 8:00 - 10:00 |
| **Tone** | Tutorial, patient, thorough |
| **Format** | Full screen recording walkthrough with voiceover |
| **Goal** | Show every feature. Onboard viewers who are ready to try it. |

### Title Options

1. **"4DA Complete Walkthrough: Setup to First Scored Feed in 3 Minutes"** (recommended)
2. **"Every Feature in 4DA -- Developer Intelligence Demo"**
3. **"How I Set Up 4DA to Filter 800 Articles a Day"**

### Script

```
[0:00 - 0:30] INTRO
"I'm going to walk through every feature of 4DA in about 9 minutes.
By the end you'll have it installed, scanning your projects, and seeing
your first scored content. Let's go."

[0:30 - 1:30] INSTALLATION
- Show downloading from 4da.ai (platform detection selects installer)
- Run the installer (~15MB, Tauri not Electron)
- First launch shows the Void Engine heartbeat (48px ambient indicator)
- "That pulse means the app is alive and ready."

[1:30 - 3:00] CONFIGURATION
- Open Settings
- Add API key (BYOK -- show blur over actual key)
- "Your key is stored in a local JSON file. It's never transmitted
  to our servers because we don't have servers."
- Configure sources: toggle HN, Reddit, arXiv, GitHub, etc.
- Add RSS feed URLs
- "All 11 sources are available on the free tier."

[3:00 - 4:30] PROJECT SCANNING
- Add project directory
- Watch ACE scan: reads Cargo.toml, package.json, Git history
- Show the context model: language breakdown, dependency list,
  framework detection
- "ACE discovered 47 dependencies, 3 languages, and my recent Git
  activity. This is the profile that defines what's relevant to me."

[4:30 - 6:00] SCORING ENGINE (deep dive)
- Show items arriving and being scored
- Click into a high-scoring item: show the 5-axis breakdown
  - Context: 0.82 (high semantic match)
  - Interest: 0.71 (aligns with declared interests)
  - ACE: 0.89 (directly involves our tech stack)
  - Dependency: 0.95 (mentions a package in our Cargo.toml)
  - Learned: 0.60 (neutral -- not enough history yet)
- "Four out of five axes triggered. This is exactly the kind of
  multi-signal confirmation that means this content matters."
- Show a rejected item: only Interest axis triggered at 0.45
- "One weak signal. Rejected. That's the confirmation gate working."

[6:00 - 7:00] DEVELOPER DNA
- Show the Developer DNA panel
- Language breakdown bars: Rust 62%, TypeScript 31%, SQL 18%
- Dependency surface, framework map
- "This is a living profile. It updates as your codebase evolves."

[7:00 - 8:00] AI BRIEFINGS + MCP
- Show an AI-generated daily briefing
- Open Claude Code, demonstrate MCP query:
  "What breaking changes affect my dependencies?"
- Show 4DA returning scored, filtered results to the AI
- "30 intelligence tools. Your AI assistant gets signal, not noise."

[8:00 - 8:30] PRICING
- Free: all 11 sources, full scoring, MCP, CLI, behavior learning
- Pro ($12/mo): AI briefings, Developer DNA, Score Autopsy, intelligence

[8:30 - 9:00] CLOSING
- "Download at 4da.ai. Setup takes 3 minutes. First results in 60
  seconds. All signal. No feed."
```

### Pinned Comment

```
Download 4DA (free): https://4da.ai

Timestamps:
0:00 What you'll see
0:30 Installation
1:30 API keys (BYOK)
3:00 Project scanning
4:30 Scoring engine deep dive
6:00 Developer DNA
7:00 AI briefings + MCP
8:00 Pricing

What feature are you most interested in? Let me know below.
```

---

## Video 3: Viral Format (2:15 + 0:58 Shorts)

### Metadata

| Field | Value |
|-------|-------|
| **Duration** | 2:15 (full), 0:58 (Shorts) |
| **Tone** | Personal, dramatic, relatable |
| **Format** | Fast cuts, talking head + B-roll, bold text overlays |
| **Goal** | Maximum reach. Optimized for Shorts, TikTok, Twitter/X. |

### Title Options

1. **"I Built an AI That Reads the Internet For Me"** (47 chars, recommended)
2. **"I Stopped Reading Hacker News. Here's What I Built Instead."** (57 chars)
3. **"How I Filter 99% of Tech News With a Desktop App I Made"** (55 chars)

### Thumbnail

- Creator at desk, head in hands, floating screenshots of HN/Reddit/newsletters
- Text overlay: "800 articles. 3 matter."
- High contrast, dark theme, cinematic lighting

### Full Script (2:15)

```
[0:00 - 0:03] HOOK
Close-up, direct to camera. Intense.

"I built an AI that reads the entire internet for me every single
day. Let me explain."

[0:03 - 0:25] THE STORY
B-roll: endless tabs, notification badges, newsletters.
Text overlays stack as each source is named.

"Six months ago, I was spending two hours every morning before I
even wrote a line of code. Hacker News. Reddit. arXiv. GitHub
Trending. Three newsletters. Two Discord servers. A YouTube feed
I never finished.

I wasn't staying informed. I was drowning. And the worst part?
I'd close all those tabs and couldn't tell you a single thing
that was actually relevant to what I was building."

[0:25 - 0:55] THE BREAKING POINT
Hard cut to empty text editor. Cursor blinking.

"So one Saturday I snapped. I opened a new Rust project and I
said: what if I built something that reads all of this FOR me?

Not a recommendation engine trained on everyone else's behavior.
Not a feed that rewards engagement bait. Something that knows
MY code, MY stack, MY projects -- and filters everything through
that lens.

Something that runs on my machine. Doesn't phone home. Doesn't
track me. Doesn't sell my reading habits."

[0:55 - 1:30] THE BUILD
Fast montage: Rust code, terminal, UI taking shape.

"I built it in Tauri -- Rust plus React. Added eleven content
sources. Built a five-axis scoring algorithm. Added a project
scanner that reads your actual codebase.

I called it 4DA. Four Dimensional Autonomy. Because the whole
point is: you decide what matters.

The first time I ran it against a full day of content -- 800+
articles, papers, posts -- it rejected 99% of them. And the
eight items it kept? Every single one was directly relevant to
a project I was actively shipping."

[1:30 - 1:55] THE RESULT
4DA interface, clean and focused. 8 items with high scores.

"That two-hour morning scroll? Gone. Replaced by a sixty-second
scan of content already scored against my codebase.

The scoring engine is free. The data stays on my machine. There's
no account to create.

I built the tool I wish existed. And now it does."

[1:55 - 2:15] CTA
4DA logo, download link.

"It's called 4DA. Link in description. Sixty seconds to first
results. All signal. No feed."
```

### Shorts Edit (0:58)

```
[0:00 - 0:03] "I built an AI that reads the internet for me."
[0:03 - 0:12] "I was spending two hours every morning scrolling
              HN, Reddit, arXiv before writing a single line."
[0:12 - 0:25] "So I built a desktop app that scans eleven sources,
              scores everything against my actual codebase, and
              rejects 99% as noise."
[0:25 - 0:40] "It runs locally. Zero telemetry. Your data never
              leaves your machine. The scoring engine is free."
[0:40 - 0:55] "Eight hundred articles a day. It keeps three. And
              every one is relevant to code I'm actually shipping."
[0:55 - 0:58] "It's called 4DA. Link in bio."
```

### Cross-Platform Distribution

- **YouTube Shorts:** Upload as separate video, 9:16 vertical, 24-48h after full version
- **TikTok:** Same 0:58 edit, caption with #devlife #developer #productivity #buildinpublic
- **Twitter/X:** Upload full 2:15 as native video (not YouTube link), pin during launch week
- **Instagram Reels:** Same 0:58 edit

---

## Production Notes (All Videos)

### Visual Style

- Dark UI recordings on #0A0A0A background
- Text overlays: Inter Bold, white on dark
- Accent highlights: #F97316 (orange) for emphasis
- Gold #D4AF37 for axis names and special callouts
- 4K preferred, minimum 1080p

### Audio

- Background music: subtle ambient electronic, lower than voice
- No music during technical deep dives (let explanations breathe)
- Hardcoded captions for Shorts (mandatory)

### B-Roll Shot List

1. Browser with 20+ tabs (HN, Reddit, arXiv, newsletters)
2. Email inbox with unread newsletter count
3. Close-up of hands closing browser tabs rapidly
4. Empty text editor with blinking cursor
5. Rust code being typed (use actual 4DA code)
6. Terminal showing `cargo build` output
7. 4DA UI first launch with Void Engine heartbeat
8. 4DA interface with scored items (clean, composed)
9. Score Autopsy 5-axis breakdown view
10. Developer DNA panel
11. Claude Code terminal with MCP query results
12. 4DA logo on #0A0A0A background

---

*Extracted from 4DA Video Content Strategy (docs/marketing/04-video-content-strategy.md). Scripts are production-ready.*
