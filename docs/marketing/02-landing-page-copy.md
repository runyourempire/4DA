# 4DA Landing Page Copy
## Version 1.0 | February 2026

**Target audience:** Software developers (IC to senior/staff), indie hackers, open-source maintainers
**Conversion goals:** Download app (primary), Install MCP server (secondary), Upgrade to Pro (tertiary)
**Tone:** Minimal, confident, technical but accessible. Linear/Vercel energy -- not enterprise SaaS.

---

## 1. HERO SECTION

### Headline Options

**Option A (Primary -- recommended):**
> # All signal. No feed.

**Option B (Outcome-driven):**
> # Stop browsing. Start knowing.

**Option C (Quantified):**
> # 11 sources. 5 axes. Only what matters.

**Option D (Pain-forward):**
> # Your feed is lying to you.

**Option E (Identity):**
> # Intelligence, not information.

**Option F (Rebellion -- for privacy-intent traffic):**
> # Your feed is watching you. 4DA watches for you.

**Option G (BYOK Power -- for technical audiences):**
> # Your keys. Your data. Your edge.

**Option H (Manifesto -- for high-awareness organic traffic):**
> # Intelligence without surveillance.

---

### Subheadline

> 4DA monitors 11 sources, scores everything against your codebase and tech stack, and rejects 99%+ as noise. What survives is signal.

### CTA Buttons

**Primary:**
> Download for free

**Secondary:**
> `npx` MCP Server

### Trust Signals Line

> Runs locally  --  Zero telemetry  --  BYOK  --  FSL-1.1-Apache-2.0

### Rationale

The hero uses Cialdini's *Liking* principle -- "All signal. No feed." mirrors exactly how developers talk about information overload. The subheadline provides specificity (11 sources, 5 axes, 99%+) to activate *Authority*. The trust signals line hits privacy objections before they form. "Download for free" is stronger than bare "Download" because it eliminates the cost objection at the very first decision point.

---

## 2. SOCIAL PROOF / TRACTION SECTION

### Section Label
`TRACTION`

### Section Headline
> Developers who stopped scrolling

### GitHub Stars / Badges Display
```
[GitHub stars badge]  [Downloads badge]  [MCP installs badge]
```

Frame by scale:
- Under 500 stars: Show the number raw. Developers respect early-stage tools.
- 500--5,000: "500+ developers switched from noise to signal"
- 5,000+: "Trusted by X,000 developers"

### Capability Proof Cards (Replace Testimonials)

Instead of fabricated social proof, use provable product facts that address the same objections:

**Card 1 -- Discovery Proof (addresses "does it actually work?")**
> Automatic Context Discovery: Reads your Cargo.toml, package.json, go.mod, and pyproject.toml. Builds a graduated domain profile from your actual stack -- no manual configuration required.

**Card 2 -- Signal Quality Proof (addresses "is it worth the setup?")**
> Multi-Signal Confirmation Gate: Every item is scored across 5 independent axes. An item needs 2+ confirming signals to surface. Typical rejection rate: 99%+ of incoming content.

**Card 3 -- Privacy Proof (addresses "why not just use X?")**
> Privacy by Architecture: Raw data never leaves your machine. BYOK for AI features. Zero telemetry. Full offline mode with Ollama. Architecturally unable to see your data.

**Card 4 -- Integration Proof (addresses power users)**
> MCP Intelligence Server: 30 tools exposing scored intelligence, signal chains, knowledge gaps, and decision memory to Claude Code, Cursor, and any MCP-compatible host.

### Social Proof Strategy

Use real metrics when available. Until launch:
- Show GitHub stars count (real number, however small)
- Show benchmark test count (84 scoring tests, 606 total)
- Show source count and scoring accuracy data
- Never fabricate quotes or attribute statements to non-existent users

### Rationale

Social proof is placed early because developers research tools skeptically. Testimonial templates target the four core objections (does it work, is it worth it, why not alternatives, power user value). The "used by" framing activates *Social Proof* without requiring enterprise logos -- identity-based framing ("Rust developers, React engineers") lets visitors self-select into the community.

---

## 3. PROBLEM SECTION

### Section Label
`THE PROBLEM`

### Section Headline
> Staying current is a full-time job

### Pain Points

**Pain Point 1 -- The Security Blind Spot**
> You skim 500+ articles a day and still miss the security advisory for a package you actually use.

**Pain Point 2 -- The Relevance Inversion**
> You read three "intro to X" posts about tech you already know while the arXiv paper relevant to your project scrolls past unseen.

**Pain Point 3 -- The Production Surprise**
> Your dependency ships a breaking change. You find out when production breaks.

### Amplification Paragraph

> The problem is not information scarcity. It is the opposite. You are drowning in content that was never meant for you. General-purpose feeds optimize for engagement -- not relevance to your actual work. So you spend cognitive cycles filtering noise that a machine should be filtering for you.

### Rationale

PAS framework (Problem-Agitate-Solution) is in motion here. Each pain point is designed to produce a nod -- these are universal developer experiences. The amplification paragraph agitates by naming the root cause (engagement optimization vs. relevance) and positions machine filtering as the logical resolution. This activates *Liking* through shared frustration -- "they understand my actual problem."

---

## 4. HOW IT WORKS SECTION

### Section Label
`HOW IT WORKS`

### Section Headline
> From noise to signal in four stages

### Pipeline Visualization

```
Your Codebase  -->  ACE Scanner  -->  5-Axis Scoring  -->  Signal
Cargo.toml,        Builds domain     Context, Interest,    Only what
package.json,      profile           ACE, Dep, Learned     matters
go.mod
```

### Confirmation Gate Callout

> An item needs **2+ independent signals** to pass the confirmation gate. Everything else is rejected.

### Expanded Stage Descriptions

**Stage 1: Your Codebase**

4DA reads your manifest files -- `Cargo.toml`, `package.json`, `go.mod`, `requirements.txt` -- and your recent Git activity. It never sends this data anywhere. It builds understanding locally.

**Stage 2: ACE Scanner**

The Autonomous Context Engine constructs a graduated technology profile. Not just "you use React" but "you are deep in React 19 with server components, moderate in TypeScript, and exploring Rust through a side project." Weighted. Nuanced. Automatic.

**Stage 3: 5-Axis Scoring**

Every piece of incoming content is evaluated across five independent axes:

- **Context** -- Does this match your explicit interests?
- **Interest** -- Does the content quality and topic align?
- **ACE** -- Does your codebase context confirm relevance?
- **Dependency** -- Does this affect a package you actually depend on?
- **Learned** -- Have your past actions (saves, dismissals) trained the model to surface or suppress this?

An item must score on 2+ axes independently. Single-signal matches are rejected as coincidence.

**Stage 4: Signal**

What survives is what matters. Typically less than 1% of all scanned content. No engagement tricks. No algorithmic amplification. Just relevance to your actual work.

### Rationale

The four-stage pipeline uses *Authority* -- it demonstrates technical sophistication without intimidation. The confirmation gate concept (2+ independent signals) is the core differentiator and should be emphasized visually. Each stage explanation uses precise language that developers trust. The "not just X but Y" pattern in the ACE description ("not just 'you use React' but...") sells the depth of the system without requiring the reader to study documentation.

---

## 5. FEATURES SECTION

### Section Label
`FEATURES`

### Section Headline
> Everything noise reduction needs

### Section Subtitle
> Every feature exists to reduce noise and surface what actually matters to your work.

### Feature Cards (ordered by persuasion impact)

**Card 1 -- The Differentiator**
```
Tag:    SCORING
Title:  5-Axis Scoring
Body:   Context, Interest, ACE, Dependency, and Learned axes
        independently evaluate every item. Multi-signal confirmation
        gate rejects 99%+ of incoming content. No single signal can
        pass something through alone.
```

**Card 2 -- The Trust Builder**
```
Tag:    PRIVACY
Title:  Privacy First
Body:   All data stays on your machine. Raw content never leaves.
        BYOK for Anthropic, OpenAI, or fully local with Ollama.
        Zero telemetry. No analytics. No tracking pixels.
        We literally cannot see your data.
```

**Card 3 -- The "How?" Answer**
```
Tag:    CONTEXT
Title:  Auto Context Discovery
Body:   Scans your Cargo.toml, package.json, go.mod, and Git activity
        to build a graduated domain profile of your technology identity.
        No configuration required. It reads your code, not your mind.
```

**Card 4 -- The Power User Hook**
```
Tag:    INTEGRATION
Title:  MCP Integration
Body:   30 tools for Claude Code and Cursor. Query your intelligence
        feed, check dependencies, and surface relevant content directly
        in your editor. Your AI assistant gets the same signal you do.
```

**Card 5 -- The Gets-Better Story**
```
Tag:    LEARNING
Title:  Behavior Learning
Body:   Learns from your save and dismiss actions with 30-day
        exponential decay. The scoring improves continuously as you
        use it. Recent actions matter more than old ones. It adapts
        to you.
```

**Card 6 -- The Terminal Native**
```
Tag:    CLI
Title:  CLI Binary
Body:   Terminal-native access to your intelligence feed. Scriptable,
        composable, pipe-friendly. Designed for developers who live
        in the terminal and want signal without opening another window.
```

### Rationale

Card ordering follows persuasion priority: lead with the unique mechanism (scoring), follow with the trust objection (privacy), explain the magic (context discovery), hook power users (MCP), show the improvement trajectory (learning), and close with accessibility (CLI). Each description ends with a punchy sentence that reinforces the value proposition. *Authority* is established through technical specificity. *Commitment/Consistency* is activated by the learning card -- the tool gets better the more you use it, which makes switching costs feel like investment rather than lock-in.

---

## 5B. PRIVACY AS REBELLION SECTION

### Section Label
`PRIVACY`

### Section Headline
> Your tools should work for you, not profile you

### Section Subtitle
> 4DA is built on a simple principle: developer intelligence does not require developer surveillance. Every architectural decision enforces this.

### Feature Cards

**Card 1 -- Zero Telemetry**
```
Tag:    ZERO TELEMETRY
Title:  We genuinely cannot see your data
Body:   No analytics SDK. No error reporting service. No "phone home"
        on startup. The application does not contain code that sends
        data to any server we operate. We do not know how many people
        use 4DA. That is by design.
```

**Card 2 -- BYOK Architecture**
```
Tag:    BYOK
Title:  Your keys go directly to your provider
Body:   When you use an AI feature, your request goes from your machine
        directly to the AI provider you chose. 4DA does not proxy it.
        We do not see the request. We do not see the response. Your API
        key is stored locally and never transmitted.
```

**Card 3 -- Source Verifiable**
```
Tag:    VERIFIABLE
Title:  Do not trust us. Verify.
Body:   The source code is public under FSL-1.1-Apache-2.0. Read every
        line. Build from source. Monitor the network traffic. You will
        find content fetches and your BYOK API calls. Nothing else.
        After 2 years, it converts to Apache 2.0. Full open source.
```

**Card 4 -- Desktop, Not SaaS**
```
Tag:    LOCAL
Title:  There is no 4DA server
Body:   4DA is a Tauri desktop app with a SQLite database on your disk.
        No cloud sync. No account required for the free tier. No server
        that can be breached, acquired, or shut down. When Pocket died
        in 2025, users lost their data. Your 4DA data is a file on
        your machine.
```

### Rebellion Taglines Callout

> **Intelligence without surveillance.**
>
> Your feed is watching you. 4DA watches for you.
> Your keys. Your data. Your edge.
>
> [Download -- it's free and private]

### Blog Post Link

> Read the full privacy architecture: [Why 4DA Is a Desktop App](/blog/privacy-architecture)

### Rationale

This section converts privacy from a feature bullet ("Privacy First" in the Features section) into a standalone value proposition. For visitors arriving via privacy-intent channels, this is the decision point. The four cards answer the four questions a privacy-conscious developer asks: "Do you track me?" (Zero Telemetry), "What happens to my API keys?" (BYOK), "How can I verify?" (Source Verifiable), "What if you shut down?" (Desktop, Not SaaS). The Pocket shutdown reference (July 2025) is fresh in developer memory. The rebellion taglines callout displays all three non-confrontational taglines as a visual statement. The blog post link converts the curious into advocates.

---

## 6. PRICING SECTION

### Section Label
`PRICING`

### Section Headline
> Free to use. Pro to command.

### Section Subtitle
> The free tier is not a demo. It is a complete noise-reduction engine. Pro adds intelligence layers for developers who want more than signal -- they want synthesis.

### Free Tier Card

```
FREE
$0 forever

Includes:
-- 11 source adapters (HN, arXiv, Reddit, GitHub, Product Hunt, YouTube, RSS, and more)
-- 5-axis scoring engine with confirmation gate
-- Full feed UI with save, dismiss, and search
-- Signal detection and classification
-- ACE auto-context discovery
-- CLI binary access
-- Behavior learning from your actions
-- MCP server (30 tools)
-- Auto-updates
-- Zero telemetry, always

CTA: [Download Free]
```

### Pro Tier Card

```
PRO
$12/month  or  $99/year (save $45)

Everything in Free, plus:

-- AI Daily Briefings: Wake up to a synthesized digest of what matters,
   written by AI that understands your stack
-- Developer DNA: See your technology identity profile -- what you use,
   how deep, how it evolves over time
-- Intelligence Panels: Trend detection, dependency monitoring, and
   strategic signals across your entire technology surface
-- Score Autopsy: Full 5-axis breakdown explaining exactly why each
   item was scored the way it was
-- Priority support and feature requests

CTA: [Start Pro -- $12/month]
```

### Annual Savings Callout

> Annual plan: $99/year saves you $45. That is less than one mass-market SaaS subscription for a tool that actually respects your attention.

### FAQ: Objection Handling

**Q: Is the free tier actually useful or just a teaser?**

A: The free tier includes all 11 source adapters, the full 5-axis scoring engine, the feed UI, behavior learning, the CLI, and the MCP server. It is a complete product. Pro adds AI synthesis and intelligence layers on top. Most individual developers will find the free tier more than sufficient.

**Q: Why should I pay when I bring my own API keys?**

A: Your API keys power the AI models. The Pro subscription unlocks the intelligence features built on top of those models -- briefing generation, DNA profiling, trend analysis, and score autopsy. Think of it as: you bring the engine, Pro unlocks the dashboard.

**Q: How does 4DA compare to Feedly / Inoreader / daily.dev?**

A: Feed readers show you everything and let you filter manually. daily.dev personalizes by engagement patterns. 4DA scores against your actual codebase. It reads your `package.json`, not your click history. The result: 99%+ rejection rate instead of "here is everything, good luck." Different category entirely.

**Q: What about privacy? Do you see my codebase data?**

A: No. 4DA is a desktop application. Your manifest files, git history, scoring data, and intelligence feed all stay on your machine. There is no server. There is no account required for the free tier. There is no telemetry. We architecturally cannot see your data because we never receive it.

**Q: What if I stop paying for Pro?**

A: You keep the free tier permanently. Your data, your scoring history, your behavior learning -- all of it stays. You lose access to AI briefings, DNA profiling, and intelligence panels. No lock-in. No data hostage.

**Q: Is this open source?**

A: 4DA is licensed under FSL-1.1-Apache-2.0. The source code is publicly available and readable. After 2 years, it converts to Apache 2.0 -- a fully permissive open-source license. The MCP server is MIT-licensed today.

### Rationale

The pricing section uses several Cialdini principles simultaneously. *Reciprocity*: the free tier is genuinely generous (all 11 sources, full scoring, MCP server), creating goodwill that primes upgrade consideration. *Commitment/Consistency*: once a developer uses the free tier and builds behavior learning, upgrading to Pro feels like a natural progression -- not a separate purchase. *Scarcity*: the annual savings frame creates mild urgency without feeling manipulative. No artificial countdown timers. The FAQ section preemptively handles the "yeah, but..." objections. The "you bring the engine, Pro unlocks the dashboard" metaphor reframes BYOK as a feature rather than a cost-shifting mechanism.

---

## 7. DEVELOPER DNA SECTION

### Section Label
`DEVELOPER DNA`

### Section Headline
> See what your code says about you

### Section Subtitle
> Developer DNA builds a living profile of your technology identity -- not from surveys or self-reporting, but from what you actually build.

### Body Copy

Your tech stack is not a list on your resume. It is a weighted, evolving graph of technologies you actively use, the depth at which you use them, and how that profile changes over time.

Developer DNA scans your local projects and constructs this graph automatically:

- **Primary technologies** -- your daily drivers, weighted by recency and depth
- **Secondary technologies** -- tools you use regularly but are not your core
- **Exploration zone** -- things you are experimenting with or learning
- **Dependency surface** -- the full graph of packages your projects depend on

This profile powers 4DA's scoring engine. When a new arXiv paper mentions a technique relevant to your primary stack, it surfaces. When someone posts an "Intro to React" tutorial and you have been shipping React for 4 years, it does not.

But Developer DNA is more than a scoring input. It is a mirror.

> See how your technology identity evolves over time. Watch your exploration zone graduate to secondary, and secondary to primary. Track your dependency surface growing or shrinking. Understand your own trajectory.

### CTA

> Unlock Developer DNA with Pro -- $12/month

### Rationale

Developer DNA activates *Liking* and *Commitment/Consistency* simultaneously. Developers are deeply identified with their tech stack -- showing them an accurate, evolving profile creates emotional resonance. The "mirror" metaphor is intentional: people are drawn to accurate reflections of themselves. The progression from "it powers scoring" to "it is a mirror" moves the value proposition from utility to identity -- a stronger purchase motivator. The section ends with the Pro CTA because DNA is a Pro feature, and by this point the reader is curious enough about their own profile to consider upgrading.

---

## 8. MCP INTEGRATION SECTION

### Section Label
`MCP`

### Section Headline
> Your AI assistant gets signal too

### Section Subtitle
> 21 MCP tools that give Claude Code and Cursor real-time access to your intelligence feed.

### Body Copy

If you use Claude Code or Cursor, 4DA's MCP server turns your AI assistant into a context-aware collaborator. Instead of asking your AI to search the internet, give it access to your already-scored, already-filtered intelligence feed.

### Installation Code Block

```bash
# Install in one command
npx @4da/mcp-server

# Or add to your Claude Code config
{
  "mcpServers": {
    "4da": {
      "command": "npx",
      "args": ["@4da/mcp-server"]
    }
  }
}
```

### Example Queries (what you can ask your AI assistant)

```
"What breaking changes affect my dependencies this week?"
"Summarize the top-scored items from the last 24 hours."
"Are there any security advisories for packages in my Cargo.toml?"
"What is trending in the Rust ecosystem right now?"
```

### Closing Line

> 30 tools. Zero configuration. Your AI stops hallucinating about what is new -- because it has a grounded, scored, real-time feed to draw from.

### CTA

> Install MCP Server -- MIT Licensed, always free

### Rationale

The MCP section targets the highest-value developer segment: those already using AI-assisted development. This is the *Authority* principle -- demonstrating deep integration with the developer's existing workflow. The code block is critical: developers trust tools they can install in one line. Showing `npx` installation and the JSON config makes it tangible and copy-pasteable. The natural-language query examples demonstrate value without requiring the reader to understand MCP protocol internals. The "MIT Licensed, always free" CTA removes both cost and licensing objections for this entry point.

---

## 9. DOWNLOAD / FINAL CTA SECTION

### Section Label
`DOWNLOAD`

### Section Headline
> Get started in under 3 minutes

### Section Subtitle
> Pre-built binaries with auto-updates. No Rust toolchain required.

### Platform Table

| Platform | Format | Auto-updates |
|----------|--------|-------------|
| Windows | .msi installer | Yes |
| macOS | .dmg (Apple Silicon + Intel) | Yes |
| Linux | .AppImage / .deb | Yes |

### MCP Alternative Box

> Or install the MCP server for Claude Code / Cursor:
>
> `npx @4da/mcp-server`

### Final Persuasive Push

> 4DA is free. It runs locally. It respects your privacy. It gets smarter every day you use it. And it takes 3 minutes to set up.
>
> The only question is how long you want to keep manually filtering noise.

### Trust Reinforcement Line

> No account required -- No telemetry -- No tracking -- Your data stays yours -- FSL-1.1-Apache-2.0

### CTA Button

> Download 4DA

### P.S. (for long-form / email versions)

> P.S. -- 4DA's scoring engine rejects 99%+ of content before you ever see it. The items that survive earned their place through 2+ independent relevance signals scored against your actual codebase. Not your browsing history. Not your click patterns. Your code. That is the difference.

### Rationale

The download section uses *Commitment/Consistency* -- you have read this far, the logical next step is to download. Every remaining friction point is removed: free, local, no account, no telemetry, 3 minutes. The "only question" close uses mild loss aversion -- every day without 4DA is a day spent manually filtering. The trust reinforcement line repeats the privacy message because this is the decision point where objections resurface hardest. Multiple CTA paths (platform table links + MCP box + download button) serve different user preferences and entry points. The P.S. reinforces the core mechanism for readers who skip to the bottom.

---

## 10. SEO META TAGS

### Title Tag
```html
<title>4DA -- Privacy-First Developer Intelligence | All Signal. No Feed.</title>
```

### Meta Description
```html
<meta name="description" content="4DA monitors 11 sources, scores content against your codebase using 5-axis analysis, and rejects 99%+ as noise. Desktop app for Windows, macOS, Linux. Free tier. Zero telemetry. BYOK.">
```

### Open Graph Tags
```html
<meta property="og:title" content="4DA -- All signal. No feed.">
<meta property="og:description" content="Privacy-first developer intelligence. 11 sources scored against your codebase. 99%+ noise rejected. Free desktop app for Windows, macOS, Linux.">
<meta property="og:type" content="website">
<meta property="og:url" content="https://4da.ai">
<meta property="og:image" content="https://4da.ai/og-image.png">
<meta property="og:image:width" content="1200">
<meta property="og:image:height" content="630">
<meta property="og:site_name" content="4DA">
```

### Twitter Card
```html
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="4DA -- All signal. No feed.">
<meta name="twitter:description" content="Privacy-first developer intelligence. 11 sources. 5-axis scoring. 99%+ noise rejected. Free desktop app.">
<meta name="twitter:image" content="https://4da.ai/og-image.png">
```

### Additional SEO Tags
```html
<meta name="keywords" content="developer intelligence, privacy-first, developer tools, content curation, noise reduction, codebase scoring, MCP server, Claude Code, Cursor, Hacker News filter, arXiv filter, developer feed, BYOK, local-first, desktop app, Tauri">
<meta name="robots" content="index, follow">
<link rel="canonical" href="https://4da.ai">
<meta name="theme-color" content="#0A0A0A">
```

### Structured Data (JSON-LD)
```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  "name": "4DA",
  "alternateName": "4 Dimensional Autonomy",
  "description": "Privacy-first developer intelligence desktop app. Monitors 11 sources, scores content against your codebase, rejects 99%+ as noise.",
  "applicationCategory": "DeveloperApplication",
  "operatingSystem": "Windows, macOS, Linux",
  "offers": [
    {
      "@type": "Offer",
      "price": "0",
      "priceCurrency": "USD",
      "description": "Free tier with 11 sources, 5-axis scoring, and MCP server"
    },
    {
      "@type": "Offer",
      "price": "12",
      "priceCurrency": "USD",
      "description": "Pro tier with AI briefings, Developer DNA, and intelligence panels"
    }
  ],
  "downloadUrl": "https://github.com/runyourempire/4DA/releases/latest",
  "softwareVersion": "latest",
  "author": {
    "@type": "Organization",
    "name": "4DA"
  }
}
</script>
```

### Rationale

The title tag front-loads "Privacy-First Developer Intelligence" because that is the primary search differentiator. "All Signal. No Feed." is appended for brand recognition. The meta description hits the key numbers (11 sources, 5-axis, 99%+) and platform coverage within the 155-character sweet spot for SERP display. OG image dimensions (1200x630) are optimized for social sharing across Twitter/LinkedIn/Slack. The JSON-LD structured data helps Google surface this as a software application with free and paid tiers, which can trigger rich snippets in developer-intent search results.

---

## PSYCHOLOGICAL TRIGGERS APPLIED

### Cialdini's 6 Principles -- Deployment Map

| Principle | Where Applied | How |
|-----------|--------------|-----|
| **Reciprocity** | Free tier generosity (Section 6) | Free tier includes all 11 sources, full scoring engine, MCP server, CLI, behavior learning -- genuinely useful, not a crippled demo. Creates goodwill that primes the upgrade decision. |
| **Commitment/Consistency** | Behavior learning (Section 5), Developer DNA (Section 7), Download CTA (Section 9) | The tool improves with use, making continued use feel like an investment. DNA profiling ties the developer's identity to the product. Reading 9 sections and then seeing "Download" feels like the natural next step, not a hard sell. |
| **Social Proof** | Traction section (Section 2), Testimonial templates | GitHub stars, download counts, and testimonials from recognizable developer roles validate the choice. Identity-based community framing ("Rust developers, React engineers") lets visitors self-select. |
| **Authority** | 5-Axis Scoring (Section 4), Technical specificity throughout, Architecture detail | Precise numbers (99%+, 5 axes, 2+ signals, 30-day exponential decay, 21 MCP tools) signal engineering rigor. The architecture itself is the authority -- no appeals to celebrity endorsement needed. |
| **Liking** | Problem section (Section 3), Developer-native tone, Developer DNA (Section 7) | Pain points mirror lived experience. Voice of Customer language ("when production breaks," "phones home") creates "they get me" resonance. DNA mirrors identity back to the reader. |
| **Scarcity** | Annual pricing saves framing (Section 6) | Mild scarcity through savings ($45 saved on annual). No artificial countdown timers or "limited spots" -- developers see through manufactured urgency and it destroys trust. |

### Emotional Triggers

| Trigger | Location | Mechanism |
|---------|----------|-----------|
| Fear of missing out (professional) | Problem section, pain point 3 | "You find out when production breaks" -- professional consequences, not FOMO vanity |
| Control / Autonomy | Privacy messaging, BYOK, trust signals | "Your data stays on your machine" -- developer agency, ownership |
| Identity | Developer DNA section | "See what your code says about you" -- self-knowledge, self-reflection |
| Efficiency / Time reclaimed | Testimonial templates, subheadline | "45 minutes every morning" -- concrete time savings |
| Technical respect | Architecture explanation, scoring axis detail, code blocks | Implied: "We treat you like the engineer you are" |
| Curiosity | Developer DNA, Score Autopsy | "What would my profile look like?" -- the desire to see your own data |

### Voice of Customer Language

The following phrases mirror how developers actually talk, not how marketers describe developer problems:

- "when production breaks" (not "service disruption event")
- "scrolls past unseen" (not "goes unnoticed in your feed")
- "your daily drivers" (not "primary technology stack")
- "pipe-friendly" (not "integrates with shell workflows")
- "live in the terminal" (not "prefer command-line interfaces")
- "phones home" (not "transmits data to external servers")
- "good luck" (not "requires manual review")
- "copy-pasteable" (not "easy to configure")

---

## A/B TEST SUGGESTIONS

### Test 1: Hero Headline Emotional Angle

| Variant | Copy | Angle |
|---------|------|-------|
| Control | "All signal. No feed." | Brand-first (identity) |
| A | "Stop browsing. Start knowing." | Outcome-first (benefit) |
| B | "Your feed is lying to you." | Pain-first (agitation) |

**Hypothesis:** "All signal. No feed." is the strongest brand statement but may not convert cold traffic. "Stop browsing. Start knowing." speaks to the desired outcome. "Your feed is lying to you." provokes an emotional reaction. Test which emotional entry point drives more downloads from cold traffic (paid ads, HN/Reddit posts).

### Test 2: Primary CTA Button Text

| Variant | Copy |
|---------|------|
| Control | "Download for free" |
| A | "Download 4DA" |
| B | "Get 4DA -- Free" |
| C | "Start filtering noise" |

**Hypothesis:** "Download for free" removes the cost objection. "Start filtering noise" is action-oriented and benefit-linked. Test whether objection-removal or benefit-framing drives higher click-through.

### Test 3: Second Fold Content

| Variant | What appears below the hero |
|---------|----------------------------|
| Control | Problem section (empathy-first) |
| A | Features section (value-first) |
| B | Screenshot carousel (proof-first) |

**Hypothesis:** Developers who already feel the pain want validation (problem section). Developers evaluating multiple tools want capability assessment (features). Developers who are skeptical want visual proof (screenshots). Split test reveals which audience segment is dominant in your traffic.

### Test 4: Social Proof Placement

| Variant | Placement |
|---------|-----------|
| Control | After hero, before problem section |
| A | After features, before pricing |

**Hypothesis:** Early social proof builds trust before the reader invests time scrolling. Late social proof validates right before the purchase decision. Position near the highest-friction decision point likely converts better for the Pro upgrade, while early placement likely converts better for free downloads.

### Test 5: Pricing Anchor Order

| Variant | Display |
|---------|---------|
| Control | "$12/month or $99/year (save $45)" |
| A | "$99/year or $12/month" |

**Hypothesis:** Leading with annual anchors the effective price lower ($8.25/mo) and makes monthly feel expensive by comparison. Test whether anchor order affects Pro conversion rate.

### Test 6: Developer DNA as Standalone Conversion Page

| Variant | Structure |
|---------|-----------|
| Control | DNA section within the landing page |
| A | Separate "/dna" page with deeper content and interactive preview |

**Hypothesis:** Developer DNA is the most emotionally resonant Pro feature. A dedicated page with an interactive preview (e.g., "paste your package.json and see a preview of your DNA") could be a higher-converting Pro upgrade path than a section embedded in the main page. The standalone page also creates a shareable URL for organic distribution.

---

## IMPLEMENTATION NOTES

### Copy Length Guidance

The full landing page rendered with all sections will be long. This is intentional for SEO and for developers who research thoroughly before installing. The visual hierarchy must ensure:

1. **Hero + trust signals** are visible without scrolling on desktop (above the fold)
2. **Problem section** is scannable -- three bullet points, no wall of text
3. **How It Works pipeline** is visual/diagrammatic, not paragraph-heavy
4. **Features** use the card grid layout -- no one reads six paragraphs in sequence
5. **Pricing** is a clear two-column comparison, not prose
6. **Download CTA** appears at minimum 3 times: hero buttons, pricing section, and final section

### Typography Hierarchy (matches existing site CSS)

| Element | Font | Size | Weight | Color |
|---------|------|------|--------|-------|
| Section labels | JetBrains Mono | 12px | 400 | #F97316 (accent) |
| Section headlines | Inter | 32px | 600 | #FFFFFF |
| Body copy | Inter | 16px | 400 | #A0A0A0 |
| Feature card titles | Inter | 16px | 600 | #FFFFFF |
| Feature card body | Inter | 14px | 400 | #A0A0A0 |
| Code / technical terms | JetBrains Mono | 14px | 400 | #F97316 |
| Trust signals / muted | Inter | 13px | 400 | #666666 |
| CTA buttons (primary) | Inter | 15px | 500 | #0A0A0A on #FFFFFF |
| CTA buttons (secondary) | Inter | 15px | 500 | #FFFFFF on transparent |

### Assets to Create Separately

- **OG image** (1200x630): Dark #0A0A0A background, "All signal. No feed." headline, 4DA logo, orange accent line
- **Screenshot set** for carousel: Relevance Analysis, Developer DNA, Daily Briefing, Score Autopsy
- **Proof cards**: Use provable product metrics (benchmark test count, scoring accuracy, noise rejection data) instead of testimonials
- **Badges**: GitHub stars via shields.io, npm download count, Discord/community member count
- **Favicon and app icon**: Ensure the hero-sun.jpg asset works at 16x16 and 32x32

### Section IDs for Navigation (anchor links)

```
#hero
#traction
#problem
#how-it-works
#features
#pricing
#developer-dna
#mcp
#download
```

---

*Document prepared for 4DA landing page implementation. All copy is immediately usable. Rationale sections explain the psychological reasoning behind key choices and can be stripped when implementing.*
