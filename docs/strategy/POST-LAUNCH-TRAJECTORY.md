# 4DA Post-Launch Trajectory Plan

> **Status:** DRAFT — Strategic planning document
> **Author:** Claude (Lead Senior Dev) + Antony (Product Owner)
> **Created:** 10 March 2026
> **Premise:** 4DA has launched successfully. Customers are paying. Everything is moving.
> **Question:** What makes 4DA uninstallable? Where do we grow? What's optimal?

---

## Part 1: The Honest Audit — Where We Actually Are

### What's Shipping Today

| Category | Count | Quality |
|----------|-------|---------|
| Content Sources | 12 + custom RSS/YT/Twitter/GH | Production-ready |
| Scoring Pipeline | 8-phase PASIFA v2 | Battle-tested, 84 benchmarks |
| UI Views | 10 (progressive disclosure) | Polished |
| Tauri Commands | 160+ | Comprehensive |
| MCP Tools | 30 across 8 categories | Complete but underutilized |
| Tests | 2,541 (1,660 Rust + 881 frontend) | Strong coverage |
| Languages | 11 (i18n) | Production |
| STREETS Modules | 7 (all free) | Written, personalized |
| Pro Features | 14 gated commands | Working |
| Licensing | Keygen + ed25519 self-signed | Working |

### What's Built But Not Fully Wired

- Email digest sending (config exists, SMTP not wired)
- Toolkit HTTP execution (feature-gated, framework exists)
- STREETS CoachGPT (framework exists, needs live integration)
- Video curriculum (structure exists, no content)
- Search synthesis UI (command exists, UI partial)
- Delegation scoring (experimental flag)

### What Doesn't Exist Yet

- Team/org features (single-user only)
- REST/GraphQL API (Tauri IPC + MCP only)
- Mobile app
- Web dashboard
- Source plugin API (community can't add sources)
- IDE extension (VS Code, JetBrains)
- Auto-update distribution pipeline (update banner exists)
- SSO/SAML/SCIM
- Audit logging
- Admin console

---

## Part 2: Pricing Architecture Redesign

### Current State Problems

The current naming is fragmented:
- **4DA Free** → genuinely good product
- **4DA Pro** ($12/mo, $99/yr) → AI intelligence layer
- **4DA Team** → referenced in code but no features differentiate it
- **STREETS Playbook** ($0) → free modules inside 4DA
- **STREETS Community** ($29/mo) → Discord + coaching
- **STREETS Cohort** ($499/8wk) → live program

**Problems with this:**
1. "Pro" is the most overused word in SaaS. Every tool has a "Pro" tier. It communicates nothing specific.
2. The STREETS pricing is separate from 4DA pricing, creating cognitive overhead: "Wait, I need two subscriptions?"
3. "Team" exists as a tier string but has zero differentiated features. It's a ghost tier.
4. There's no enterprise story at all — no SSO, no admin, no invoicing, no custom contracts.
5. The free→pro jump is binary. Users go from "$0 forever" to "$12/mo" with no middle ground.

### Tier Naming Analysis

Evaluated options against developer tool peers:

| Tool | Free | Individual Paid | Team | Enterprise |
|------|------|----------------|------|------------|
| Raycast | Free | Pro ($8/mo) | Teams ($15/mo/user) | — |
| Linear | Free | — | Standard ($8/user/mo) | Plus ($14/user/mo) |
| Obsidian | Free | — | — | (was Commercial, now free) |
| Tailscale | Personal | Starter ($6/user/mo) | Premium ($18/user/mo) | Enterprise (custom) |
| GitKraken | Free | Pro ($4.95/mo) | Advanced ($12/seat/mo) | Business ($16/seat/mo) |
| PostHog | Free (usage-based) | — | — | Enterprise (custom) |
| 4DA (current) | Free | Pro ($12/mo) | Team (empty) | — |

**Key insight from research:** The best-converting tier names are identity-based, not feature-based. Users should see the tier name and think "that's me" — not "what do I get?"

### The Redesigned Pricing Structure

#### Rename Rationale

**"Pro" → "Signal"**

Why:
- "Signal" is 4DA's core value proposition ("All signal. No feed.")
- It's identity-based: a Signal user is someone who values synthesis over noise
- It's not generic — you can't confuse it with every other "Pro" tier on the internet
- It reinforces the brand vocabulary every time someone refers to their tier
- The tagline writes itself: "Free to discover. Signal to command."

Alternative considered: "Power User" — too generic, too long, sounds like it's for nerds (in the wrong way). "Sovereign" — too pretentious for a $12/mo plan. "Intelligence" — too generic. "Autonomous" — brand-adjacent but doesn't flow as a tier name. "Edge" — too close to Microsoft.

**New 4-tier structure:**

```
┌─────────────────────────────────────────────────────────────────┐
│                        4DA Pricing                              │
├─────────┬──────────┬──────────────┬─────────────────────────────┤
│  FREE   │  SIGNAL  │   TEAM       │  ENTERPRISE                 │
│  $0     │  $12/mo  │  $29/user/mo │  Custom                     │
│  forever│  $99/yr  │  $249/user/yr│  (annual contract)          │
├─────────┼──────────┼──────────────┼─────────────────────────────┤
│ 12 src  │ + AI     │ + Team       │ + Managed config            │
│ PASIFA  │  Brief-  │   Briefings  │   deployment                │
│ Scoring │  ings    │ + Shared     │ + SSO / SAML                │
│ ACE     │ + Dev    │   Decision   │ + SCIM directory            │
│ Signals │  DNA     │   Journal    │   sync                      │
│ MCP 30  │ + Score  │ + Team       │ + Audit logs                │
│ STREETS │  Autopsy │   Tech Radar │ + Admin console             │
│  free   │ + Intel  │ + Aggregate  │ + Custom sources            │
│ Feed    │  Panels  │   stack      │ + Priority support          │
│ CLI     │ + NL     │   signals    │ + Volume licensing           │
│         │  Query   │ + Seat mgmt  │ + SLA                      │
│         │ + Seman- │ + Central    │ + Procurement               │
│         │  tic     │   billing    │   docs (SOC2, DPA)          │
│         │  Shifts  │              │ + Invoice billing            │
└─────────┴──────────┴──────────────┴─────────────────────────────┘
```

#### STREETS Simplification

**STREETS tiers (Community $29/mo and Cohort $499/8wk) are deprecated.** Those plans were early-stage thinking and are no longer part of the strategy.

**What stays:**
- STREETS Playbook (all 7 modules) → FREE forever, included in all tiers
- STREETS personalization (tailored to user's stack) → FREE forever
- The free playbook is a trust builder and top-of-funnel asset

**What's removed:**
- STREETS Community tier ($29/mo) — coaching features not shipping
- STREETS Cohort tier ($499/8wk) — live program not viable as solo founder
- CoachGPT, video curriculum, strategy deep dives — all deprecated
- `STREETS_COMMUNITY_FEATURES` and `STREETS_COHORT_FEATURES` constants in license.rs → remove
- `StreetsMembershipSection.tsx` → simplify or remove
- coaching DB tables (coach_sessions, coach_messages, etc.) → leave in place but don't build on them

**The coaching/cohort concept may return if/when there's a team to run it. For now, 4DA's pricing is 4DA's pricing. One product, one pricing page, zero confusion.**

#### Detailed Tier Breakdown

**FREE — $0 forever**
- 12 content sources + custom RSS/Twitter/YouTube/GitHub
- Full PASIFA scoring engine (8-phase pipeline)
- ACE auto-context discovery
- Signal classification
- Feed UI with save, dismiss, feedback
- CLI access
- MCP server (30 tools)
- STREETS Playbook (all 7 modules)
- Behavior learning
- Auto-updates
- Zero telemetry
- Community support (GitHub issues)

**SIGNAL — $12/mo or $99/yr (save $45)**
Everything in Free, plus:
- AI Daily Briefings (morning synthesis digest)
- Developer DNA profiling (technology identity)
- Score Autopsy (full 5-axis visual breakdown)
- Intelligence Panels (trend detection, semantic shifts)
- Signal Chains (causal connections between events)
- Knowledge Gap Detection (blind spot analysis)
- Natural Language Query ("show me articles about Rust async")
- Project Health Radar (dependency freshness + security)
- Decision Signals (time-bounded decision windows)
- Weekly Digest (exportable summary)
- Semantic Shift Detection (narrative changes in your topics)
- Search Synthesis (LLM-enhanced search results)
- Priority email support

**TEAM — $29/user/mo or $249/user/yr (save $99/user)**
Everything in Signal, plus:
- Team config file (shared sources, blocked domains, default context dirs)
- Team briefings (aggregate signals across the team)
- Shared decision journal (team architecture decisions)
- Team Tech Radar (collective adoption signals)
- Seat management via Keygen (invite/remove members)
- Centralized billing (one invoice per team)
- STREETS Playbook included (same as all tiers)

**ENTERPRISE — $22/user/mo (25+ seats, annual) or custom (100+ seats)**
Everything in Team, plus:
- SSO license activation (Okta, Azure AD, Google Workspace via SAML/OIDC)
- SCIM directory sync (auto-provision/deprovision seats)
- Local audit logging (exportable JSON/CSV)
- Managed configuration deployment (MDM profiles, URL-based config)
- Custom source adapters (internal Git forges, Confluence, Jira)
- Volume licensing (25-100+ seats with tiered discounts)
- Support SLA (1-business-day response, dedicated Slack channel)
- Procurement documentation package (security whitepaper, DPA, vendor risk assessment)
- Invoice billing (NET-30, PO support)
- Quarterly business review (100+ seats)

---

## Part 3: Making 4DA Uninstallable

### The Uninstallability Framework

A developer tool becomes uninstallable when it satisfies 3 conditions simultaneously:
1. **Daily habit** — user opens it every day without thinking
2. **Accumulated value** — the longer you use it, the more valuable it becomes
3. **Switching cost** — leaving means losing something irreplaceable

#### Condition 1: Daily Habit — The Morning Briefing

**Current state:** Briefing view exists but requires the user to open the app and navigate to it.

**Target state:** 4DA is the first thing a developer sees every morning.

**Implementation:**

1. **Desktop notification at 8am local time (configurable):**
   ```
   ┌──────────────────────────────────────────┐
   │ 4DA Morning Briefing                      │
   │                                            │
   │ 🔒 CVE-2026-4412 affects lodash 4.17.x    │
   │    (you use this in 3 projects)            │
   │                                            │
   │ React 20 RC1 released — 2 breaking        │
   │    changes affect your codebase            │
   │                                            │
   │ [Open Briefing]        [Dismiss]           │
   └──────────────────────────────────────────┘
   ```

2. **Briefing as system tray popup (not full app launch):**
   - Lightweight 400px window that appears from tray
   - Shows top 3-5 items with one-line summaries
   - Click to expand into full app
   - Dismiss to mark as seen

3. **Digest email (Signal tier):**
   - Wire the existing SMTP config
   - Daily or weekly, user's choice
   - HTML email with actionable links back to 4DA

4. **CLI briefing command:**
   - `4da brief` in terminal shows today's top signals
   - Developers who live in the terminal get value without opening the GUI
   - Already partially exists via MCP `daily_briefing`

#### Condition 2: Accumulated Value — The Knowledge Flywheel

**Current state:** Behavior learning exists but users don't SEE it compounding.

**Target state:** Users feel their 4DA getting smarter every week. Visible compounding.

**Implementation:**

1. **"Your Intelligence This Month" card (monthly summary):**
   ```
   ┌──────────────────────────────────────────┐
   │ Your Intelligence — February 2026         │
   │                                            │
   │ Relevance accuracy:  72% → 84% (+12%)     │
   │ Topics tracked:      14 → 19 (+5 new)     │
   │ Noise rejected:      2,847 items (99.2%)  │
   │ Time saved:          ~4.2 hours            │
   │ Security alerts:     3 (2 acted on)        │
   │ Decisions recorded:  7                     │
   │                                            │
   │ Your model has learned from 142 feedback   │
   │ signals this month.                        │
   └──────────────────────────────────────────┘
   ```

2. **Relevance accuracy tracking:**
   - Track prediction accuracy over time
   - Show users the curve: "Your 4DA was 60% accurate in week 1. After 8 weeks of feedback, it's 87% accurate."
   - This creates a "training" narrative — users feel invested in their model

3. **Decision Memory as institutional memory:**
   - Every architectural decision logged becomes searchable context
   - "Why did we choose PostgreSQL over MongoDB?" — the answer is in 4DA, not lost in Slack
   - Teams compound this faster than individuals

4. **Knowledge graph visualization:**
   - Surface the topic connections panel more prominently
   - Show how your knowledge areas connect and where gaps exist
   - "You know React deeply, TypeScript deeply, but your testing knowledge is shallow"
   - Update this monthly to show growth

#### Condition 3: Switching Cost — Irreplaceable Data

**Current state:** Data is local but not deeply entangled with the user's workflow.

**Target state:** 4DA becomes the system of record for developer intelligence.

**Implementation:**

1. **Dependency intelligence (the killer feature):**
   - Parse `package.json`, `Cargo.toml`, `requirements.txt`, `go.mod`, `Gemfile`, `pyproject.toml`, `pom.xml`
   - Build a dependency graph per project, updated on file watch
   - Cross-reference with incoming content:
     - CVE feeds → "CRITICAL: This vulnerability affects YOUR lodash 4.17.20"
     - Release notes → "React 20 dropped. You use createContext() (deprecated in 20). 12 files affected."
     - Changelogs → "Vite 6.1 has a breaking config change. Your vite.config.ts needs updating."
   - Score boost for content that mentions dependencies you actually use
   - Nobody else does this. This is the moat.

2. **Decision journal as team knowledge base:**
   - Decisions logged in 4DA persist forever
   - Searchable, cross-referenced with the content that informed them
   - "We chose Prisma over Drizzle because of X" — linked to the 3 articles that informed the decision
   - Leaving 4DA means losing your architectural decision history

3. **Developer DNA portability (but rooted in 4DA):**
   - The DNA profile is exportable (markdown, SVG) but only 4DA keeps it updated
   - Used in STREETS personalization, briefing context, team profiles
   - It's your professional identity document, maintained automatically

4. **Standing queries as persistent intelligence:**
   - "Alert me whenever someone mentions [my-oss-project]"
   - "Watch for Rust async runtime comparisons"
   - "Track any security advisory for my dependencies"
   - These accumulate over time and become your personal intelligence network

---

## Part 4: Enterprise — Honest Assessment & Redesign

### The Hard Truth About Enterprise for 4DA

After researching how Sentry, GitLab, Grafana, PostHog, and Tailscale structure their
enterprise tiers, here's what actually matters — and what doesn't apply to 4DA.

**4DA is fundamentally different from these companies.** They're all server-side tools
(error tracking, CI/CD, observability, analytics). 4DA is a **desktop application**.
This changes everything about enterprise:

| Traditional OSS Enterprise | 4DA Enterprise |
|---------------------------|----------------|
| Self-hosted = run our server in your infra | Self-hosted = already done (it's a desktop app) |
| Multi-tenant cloud with data isolation | No cloud. No multi-tenancy. No data to isolate. |
| SSO for web login | SSO for license activation (much simpler) |
| Audit logs of server-side actions | Audit logs of local actions (exported on demand) |
| Data residency across regions | Data never leaves. Residency is automatic. |
| SOC 2 Type II for cloud infrastructure | SOC 2 mostly N/A — no cloud to audit |
| $25K+ annual minimum (Grafana) | Must be appropriate for dev tool budget |

**The insight:** 4DA's privacy architecture — the thing that makes it hard to monetize
at enterprise scale — is ALSO the thing that makes enterprise compliance trivially easy.
There's no data to breach. There's no server to audit. There's no cloud to secure.

### What Enterprise Buyers Actually Want (from 4DA specifically)

Based on research into bottom-up PLG (Tailscale's model: individual → team → enterprise),
enterprise buyers of a desktop dev tool care about:

**1. License Management (non-negotiable)**
- "Give me one invoice for 50 developers"
- "Let me add/remove seats without emailing support"
- "Auto-provision licenses when someone joins via SSO"
- Current gap: Keygen supports this but 4DA doesn't expose it

**2. Configuration Governance (high priority)**
- "I want every developer to have the same approved source list"
- "Block these domains across the org" (compliance: no social media sources)
- "Set default context directories to our monorepo"
- This is NOT "admin console" — it's a shared config file or MDM profile

**3. Procurement Documentation (non-negotiable for >$10K deals)**
- Security questionnaire responses
- Data flow documentation (trivial for 4DA — "data stays local")
- Privacy impact assessment
- Vendor risk assessment
- SOC 2 is NOT required for a desktop app that processes no customer data
  in the cloud. What IS required: clear documentation that proves this.

**4. SSO / Identity (important, not day-one)**
- Enterprise dev teams use Okta/Azure AD/Google Workspace
- SSO for 4DA = "authenticate once, activate license automatically"
- NOT SSO for "logging into 4DA" (there's no login — it's a desktop app)
- SCIM for auto-provisioning: new employee joins → 4DA license auto-assigned
- This is about license lifecycle, not authentication

**5. Support SLA (important for justifying budget)**
- Slack/email channel with response time guarantee
- Not 24/7 (we're not infrastructure). 1-business-day response is fine.
- Bug escalation path
- Dedicated point of contact for >50 seat accounts

**6. Team Intelligence (the value differentiator)**
- The only thing that justifies enterprise pricing over individual Signal seats
- "What's our collective exposure?" "What tech decisions are drifting?"
- This is WHERE 4DA adds value beyond seat licensing

### What Enterprise Buyers DON'T Care About (for 4DA)

- **SOC 2 Type II audit** — 4DA processes no data in the cloud. SOC 2 is for
  cloud services. A desktop app with zero telemetry doesn't need SOC 2.
  What enterprises need is a clear letter/doc stating "no data leaves the device."
- **Multi-tenant data isolation** — There is no tenant. Each developer's 4DA is
  isolated by definition (separate machine, separate database).
- **Data residency** — Data never leaves the machine. Residency = wherever the
  laptop is. This is a selling point, not a compliance burden.
- **Complex deployment infrastructure** — No Docker, no Kubernetes, no Helm charts
  for the app itself. It's a single binary. MDM (Jamf, Intune, SCCM) handles
  distribution to managed devices.

### The Bottom-Up Enterprise Motion (Tailscale Model)

Tailscale went from 0 to 20,000 paying customers using this exact model:

```
Stage 1: PERSONAL                    Stage 2: TEAM                     Stage 3: ENTERPRISE
Individual dev discovers 4DA  →  Brings to team ("you all need this") → IT/Eng Manager buys org license
Free tier, self-serve          →  Self-serve Team at $29/user/mo       → Enterprise with procurement
No sales involvement           →  No sales involvement                 → Light-touch sales (not enterprise sales)
```

**Key Tailscale insight:** "What is needed in the homelab, also works in the enterprise."
4DA follows this exactly — same binary, same features, different license key.

**The conversion trigger:** When 3+ devs on the same team independently install 4DA,
the engineering manager asks "can we get a team license?" That's the sales moment.
We don't need to create it. We need to make it easy to act on.

### Enterprise Feature Roadmap (Revised — Realistic for Solo Founder)

#### Phase E1: Team License & Config (Month 3-4, minimal engineering)

**Team license via Keygen:**
- Create Keygen "group" policy for teams
- Team admin purchases N seats → receives team activation code
- Each developer enters code → activates Signal tier + team_id
- Admin can add/remove seats via Keygen dashboard (no custom UI needed yet)
- Billing: Stripe subscription linked to Keygen policy

**Shared configuration:**
- Team config file: `team-config.json` (JSON, distributed via Git, MDM, or URL)
- Fields: approved_sources, blocked_domains, default_context_dirs, embedding_provider
- 4DA loads team config on startup if present
- Individual settings merge with team config (team = base, personal = overlay)
- This is NOT a cloud-synced admin console. It's a config file. Simple, auditable, versionable.

```json
// Example: team-config.json
{
  "team_id": "acme-engineering",
  "approved_sources": ["hackernews", "github", "arxiv", "rss"],
  "blocked_domains": ["facebook.com", "tiktok.com"],
  "default_context_dirs": ["/Users/*/projects/acme-*"],
  "require_embedding_provider": "openai",
  "require_ollama_fallback": true
}
```

**Why this works:** No server needed. No admin console to build. The config file can
be checked into the team's repo, deployed via MDM, or downloaded from an internal URL.
IT teams already manage config files for dozens of tools this way.

#### Phase E2: Procurement Documentation (Month 4-5, zero engineering)

**Security documentation package:**
- Architecture whitepaper: "4DA processes all data locally. No cloud. No telemetry."
- Data flow diagram showing: API keys → LLM provider (user's own), all other data local
- Vendor risk assessment template (pre-filled — most answers are "N/A, local processing")
- Privacy impact assessment (trivially simple for local-first app)
- GDPR Data Processing Agreement (minimal — 4DA is a data processor only on the user's machine)
- Response template for standard security questionnaires

**The key move:** Proactively create a "Security & Privacy" page at 4da.ai/security
with downloadable docs. Enterprise buyers finding this page before they even contact
us removes the single biggest friction point in enterprise procurement.

**The FSL-1.1 letter:**
Sentry uses the identical license and sells to enterprises. Prepare a one-page
explanation: "FSL-1.1-Apache-2.0 means you can use 4DA freely internally.
You cannot fork it to build a competing product. After 2 years, it becomes Apache 2.0.
This is the same license used by Sentry ($100M+ ARR)."

#### Phase E3: SSO & Directory Sync (Month 6+, moderate engineering)

**SSO for license activation (not for app login):**
- Integration via WorkOS ($125/connection/month — they handle SAML/OIDC complexity)
- Flow: Employee logs in via Okta/Azure AD → WorkOS validates → returns org_id
  → 4DA activates license under that org → no manual key entry
- Alternative for cost-sensitive: direct SAML integration (more engineering, no per-connection cost)

**SCIM directory sync:**
- New employee provisioned in Azure AD → SCIM webhook → Keygen auto-assigns seat
- Employee deprovisioned → SCIM webhook → Keygen revokes seat → 4DA downgrades to Free
- This eliminates "zombie seats" (paying for people who left)

**Audit log (local, exportable):**
- Separate SQLite file: `data/audit.db`
- Events: license activation, source config change, API key change, data export
- Exportable as JSON/CSV on demand
- NOT phoned home. Stays on device. Admin requests export from developers.
- For larger orgs: optional endpoint to POST audit events to a central SIEM

#### Phase E4: Team Intelligence (Month 6+, significant engineering)

**This is the only feature that justifies enterprise pricing above "N × Signal seats".**

**Anonymous team context aggregation:**
- Each 4DA instance periodically generates an anonymous summary:
  - Tech stack fingerprint (not files — just "uses: React 19, TypeScript 5.x, PostgreSQL")
  - Saved signal types (not content — just "saved 3 security alerts, 2 tool discoveries")
  - Decision count (not decisions — just "recorded 5 decisions this week")
- Summary is posted to a lightweight relay (self-hosted or 4da.ai/relay)
- Relay aggregates: "Team uses: React (5/5 devs), TypeScript (5/5), PostgreSQL (3/5)"
- Team members see: "Your team collectively encountered 8 security signals this week"

**Privacy invariant preserved:** The relay sees anonymous hashed summaries.
Even if compromised, an attacker learns "a team of ~5 uses React." That's public info.

**Why this matters for enterprise:** Engineering managers can't currently answer
"what's our team's collective technology exposure?" without asking everyone.
4DA answers this automatically, privately, continuously.

**Realistic assessment:** This is the hardest feature to build. It requires a relay
service (even if lightweight). It introduces a network component to a local-first app.
It should NOT be in the initial enterprise offering. Teams of 5-20 can get value
from individual Signal licenses + shared config file long before this exists.

### Enterprise Pricing Reality Check

**The question:** What can we charge enterprises?

**Benchmark analysis:**

| Tool | Enterprise Price | What They Get | Delivery |
|------|-----------------|---------------|----------|
| Raycast | $15/user/mo | Shared commands, team sync | Cloud |
| Tailscale | $18/user/mo (Premium) | SSO, SCIM, compliance | Cloud |
| GitKraken | $16/user/mo (Business) | SSO, admin, insights | Cloud |
| Linear | $14/user/mo (Plus) | More features | Cloud |
| Grafana | $25K/yr minimum | Enterprise plugins, SLA | Cloud/Self-hosted |

**4DA's enterprise pricing should be:**
- **Team:** $29/user/mo — includes team config, shared context, central billing
- **Enterprise (>25 seats):** $22/user/mo (volume discount) — includes SSO, SCIM,
  audit logs, procurement docs, SLA, dedicated support channel
- **Enterprise (>100 seats):** Custom — includes everything above + custom sources,
  managed deployment support, quarterly business review

**Why NOT "Contact Sales" custom pricing for all enterprise:**
- Solo founder cannot run enterprise sales cycles
- Self-service with docs is more scalable than sales calls
- The Tailscale lesson: self-serve gets you to 20,000 customers without a sales team
- "Contact Sales" on a pricing page loses 50%+ of enterprise evaluators who don't
  want to sit through a demo to learn the price

**Recommended approach:** Show the price. Let them buy. Offer "talk to us" for >100 seats only.

---

## Part 5: Distribution & Growth Engines

### Distribution Channel Priority Matrix

| Channel | Effort | Reach | Conversion | Priority |
|---------|--------|-------|------------|----------|
| MCP npm package | Low | High | High | **P0** |
| Show HN + Product Hunt | Low | High | Medium | **P0** |
| VS Code extension | Medium | Very High | Medium | **P1** |
| Source plugin API | Medium | High (community) | High | **P1** |
| STREETS web presence | Low | Medium | High | **P1** |
| Blog SEO | Low | Medium | Medium | **P2** |
| YouTube | Medium | Medium | Medium | **P2** |
| Conference talks | High | Low-Medium | High | **P3** |
| Partnerships | High | Variable | Variable | **P3** |

### P0: MCP as Distribution Engine

**Action:** Publish `mcp-4da-server` to npm as standalone package.

**Why this is the #1 distribution play:**
- Every Claude Code, Cursor, Windsurf, and Cline user can install it
- It already has 30 tools — it's not a demo, it's a real product
- It works without 4DA installed (degraded mode with own lightweight DB)
- With 4DA installed, it becomes 10x more powerful (full scoring, ACE context)
- Zero marketing cost — it appears in MCP tool directories
- Users who install the MCP server and find it valuable will install the desktop app

**Implementation:**
1. Package `mcp-4da-server` for npm (`npx @4da/mcp`)
2. Add a "standalone mode" that works without 4DA's database
3. When 4DA is installed, auto-detect and connect to the full database
4. Add to Claude Code MCP registry, awesome-mcp-servers, Cursor marketplace
5. README with "Get the full experience: install 4DA desktop app"

### P1: VS Code Extension

**What it does:**
- Status bar item: "4DA: 3 new signals"
- Click to see a mini-panel with top signals relevant to the current file/project
- Links back to 4DA desktop for full briefing
- Uses MCP server under the hood (already built)
- Dependency alerts: "This package has a known vulnerability" (inline)
- Hovering over an import shows version info + any relevant signals

**Why:**
- Developers spend 8+ hours/day in VS Code
- Extension marketplace is the largest developer tool distribution channel
- Low effort because MCP server does all the heavy lifting
- Creates "ambient awareness" without requiring app switching

### P1: Source Plugin API

**The play:** Let the community build sources. 4DA goes from 12 sources to 100+.

**Architecture:**
- Define a `SourcePlugin` trait/interface
- Plugin is a WASM module or standalone binary that 4DA shells out to
- Plugin returns `Vec<SourceItem>` — same struct as internal sources
- Plugin registry (initially curated, later community-submitted)
- Installation: `4da plugin install stack-overflow` (downloads WASM to plugins/ dir)

**High-value community sources:**
- Stack Overflow (questions in your stack)
- npm/crates.io/PyPI changelogs (dependency updates)
- CVE/NVD feeds (security advisories)
- Conference talk feeds (upcoming talks on your topics)
- Bluesky/Mastodon (decentralized social)
- Discord channel monitors (community discussion)
- Substack newsletters (developer blogs)
- IndieHackers/micro.blog (builder community)
- AWS/GCP/Azure status pages (cloud provider changes)

### P1: STREETS Web Presence

**Action:** Publish STREETS playbook content at streets.4da.ai as free web content.

**Why:**
- "Free playbook for developer entrepreneurs" drives organic traffic
- SEO: target keywords like "developer side income", "developer passive income", "developer business ideas"
- Every web visitor sees: "Get personalized STREETS inside 4DA — download free"
- The web version is good; the 4DA version is better (personalized to your stack)
- Content marketing flywheel: STREETS content → Google traffic → 4DA downloads → Pro upgrades

---

## Part 6: The Uninstallability Sequence

### Week 1-2: Retention Foundation
- [ ] Daily briefing desktop notification system
- [ ] "Your Intelligence This Month" summary card
- [ ] Relevance accuracy tracking (show users their model improving)
- [ ] Wire email digest sending (SMTP already in config)

### Week 3-4: Distribution Launch
- [ ] Publish `mcp-4da-server` to npm as standalone
- [ ] Add to MCP registries (Claude Code, awesome-mcp-servers)
- [ ] Publish STREETS modules as web content at streets.4da.ai
- [ ] Create VS Code extension skeleton (status bar + mini panel)

### Month 2: The Moat
- [ ] Dependency intelligence v1 (parse dep files, cross-reference with content)
- [ ] CVE matching against user's actual dependencies
- [ ] Changelog intelligence (summarize what changed in deps you use)
- [ ] Standing queries ("watch for X" persistent intelligence)

### Month 3: Team License & Config
- [ ] Keygen team/group policy setup
- [ ] Team activation code flow (admin purchases → devs activate)
- [ ] Shared team-config.json support (loaded on startup)
- [ ] Central billing via Stripe + Keygen

### Month 4: Enterprise Procurement Readiness
- [ ] Security & privacy documentation package (zero engineering)
- [ ] 4da.ai/security page with downloadable docs
- [ ] FSL-1.1 enterprise FAQ
- [ ] Vendor risk assessment pre-filled template
- [ ] Source plugin API v1 (community sources)

### Month 5: Enterprise Features
- [ ] SSO for license activation (WorkOS integration)
- [ ] SCIM directory sync (auto-provision seats)
- [ ] Local audit logging (exportable)
- [ ] VS Code extension v1 (full release)
- [ ] Enterprise pricing page (self-serve, not "Contact Sales")

### Month 6: Platform Expansion
- [ ] Team intelligence v1 (anonymous aggregate signals)
- [ ] IDE integrations beyond VS Code (JetBrains, Neovim)
- [ ] Source plugin community registry
- [ ] Enterprise volume discount automation

---

## Part 7: Revenue Projections (Revised)

### Conservative Model (12 months post-launch)

| Tier | Users | Price | Monthly Revenue |
|------|-------|-------|-----------------|
| Free | 10,000-30,000 | $0 | $0 |
| Signal (individual) | 200-500 | $12/mo avg | $2,400-6,000 |
| Team (5-person avg) | 10-30 teams (50-150 seats) | $29/user/mo | $1,450-4,350 |
| Enterprise (50-seat avg) | 1-3 orgs | $22/user/mo × 50 = $1,100/mo | $1,100-3,300 |
| **Total MRR** | | | **$4,950-13,650** |
| **Total ARR** | | | **$59,400-163,800** |

**Reality check:** These numbers are MORE conservative than the previous projections.
Enterprise revenue from a solo-founder desktop tool takes 12-18 months to materialize.
The real early revenue comes from Signal (individual) conversions.

**Milestone targets:**
- Month 3: $1K MRR (80-100 Signal subscribers)
- Month 6: $3K MRR (200+ Signal, first Team accounts)
- Month 12: $8K MRR (400+ Signal, 10+ Teams, maybe 1 Enterprise)
- Month 18: $15K+ MRR (if enterprise motion works)

### Key Metrics to Track

| Metric | Target | Why |
|--------|--------|-----|
| Daily Active Users (DAU) | 30% of installed base | Measures habit formation |
| DAU/MAU (stickiness) | >40% | Desktop tools should be >20%; 40% = embedded in workflow |
| Free → Signal conversion | 5-8% | Industry average for dev tools is 3-5%; 4DA's free tier is generous enough to drive higher |
| Signal → Team expansion | 15-20% of Signal users | When one person on a team uses 4DA, they bring the team |
| Briefing open rate | >60% | If <60%, the briefing isn't valuable enough |
| Feedback actions/user/week | >5 | Users who give 5+ feedback signals/week have 3x retention |
| Time to first relevant signal | <60 seconds | The zero-config promise |
| NPS | >50 | Developer tools with NPS >50 grow via word of mouth |

---

## Part 8: Competitive Moat Analysis (Post-Expansion)

### Moat Depth After Full Execution

| Moat Layer | Current Depth | After 6 Months | Competitor Time to Copy |
|------------|--------------|-----------------|------------------------|
| Auto-context from codebase (ACE) | Deep | Very Deep | 12-18 months |
| 8-phase scoring pipeline | Deep | Very Deep (+ dep intelligence) | 12-18 months |
| Dependency intelligence | None | Deep | 18-24 months |
| MCP tool ecosystem | Built | Distribution channel | 6-12 months (concept), 18+ (quality) |
| STREETS integration | Built | Revenue engine | Not replicable (brand + content) |
| Privacy-by-architecture | Deep | Enterprise-validated | 24+ months (requires full rewrite for cloud tools) |
| Source plugin ecosystem | None | Community-driven | 12-18 months |
| Team knowledge compound | None | Growing | 12+ months |
| User behavior model | Growing | Deep | Cannot copy (per-user data) |

### The Defensibility Stack

```
Layer 5: Community (source plugins, STREETS, MCP ecosystem)
Layer 4: Institutional Memory (decisions, knowledge graph, team context)
Layer 3: Behavioral Model (6+ months of feedback = irreplaceable)
Layer 2: Dependency Intelligence (cross-references YOUR deps with world)
Layer 1: ACE + PASIFA (auto-context + scoring = core engine)
Layer 0: Privacy Architecture (local-first = can't be retrofitted into cloud tools)
```

Each layer compounds on the one below it. A competitor would need to replicate all 6 layers simultaneously to be equivalent — and the behavioral model (Layer 3) is literally impossible to copy because it's unique to each user.

---

## Part 9: Technical Implementation Notes

### Pricing Tier Migration (Code Changes — Full Audit)

**Current code touchpoints (from deep codebase audit):**

The tier system touches 15+ files. Here's every change needed:

#### Backend (Rust)

1. **`src-tauri/src/settings/license.rs`** (612 lines) — Core gating logic
   - `is_pro()`: Currently `matches!(tier, "pro" | "team")` → Change to `matches!(tier, "signal" | "team" | "enterprise")`
   - `is_pro_feature_available()`: Same pattern update
   - `is_trial_active()`: Update tier exclusion check
   - `get_trial_status()`: Update tier check
   - `PRO_FEATURES` constant: No change (features stay the same, only tier name changes)
   - `get_streets_tier()`: Update `"pro" | "team"` → `"signal" | "team" | "enterprise"`
   - `is_streets_feature_available()`: Same update

2. **`src-tauri/src/settings/mod.rs`** — Settings struct
   - `LicenseConfig`: Add fields: `team_id: Option<String>`, `org_id: Option<String>`, `seat_count: Option<u32>`
   - Default tier remains `"free"`

3. **`src-tauri/src/settings_commands_license.rs`** — License activation commands
   - Line 563: `settings.license.tier = "pro"` → handle "signal" tier from Keygen metadata
   - Activation flow: map Keygen tier metadata to local tier string
   - Add team activation endpoint (validates team license, sets team_id)

4. **`src-tauri/src/settings/license_tests.rs`** — All tier-related tests
   - Update test assertions for "signal" tier
   - Add tests for "enterprise" tier behavior

5. **Every `require_pro_feature()` call** (14 files):
   - `attention.rs`, `decision_signals.rs`, `developer_dna.rs`, `digest_commands.rs`
   - `knowledge_decay.rs`, `project_health.rs`, `search_synthesis.rs`, `semantic_diff.rs`
   - `signal_chains.rs`, `standing_queries.rs`, `weekly_digest.rs`
   - No code changes needed — they call `require_pro_feature()` which checks `is_pro()` internally
   - Error messages still say "requires 4DA Pro" → Change to "requires 4DA Signal"

#### Frontend (TypeScript/React)

6. **`src/components/ProGate.tsx`** — Paywall component
   - Rename all "Pro" copy → "Signal"
   - Update upgrade link from `https://4da.ai/streets` → `https://4da.ai/pricing`
   - Update CTA: "Start Signal — $12/month"

7. **`src/components/ProValuePanel.tsx`** — Marketing panel
   - All "Pro" references → "Signal"
   - Update feature descriptions

8. **`src/components/ProValueBadge.tsx`** — Header badge
   - Rename badge text
   - Consider renaming component file: `ProValueBadge.tsx` → `SignalValueBadge.tsx`

9. **`src/store/license-slice.ts`** — Zustand store
   - `tier` type: `'free' | 'pro' | 'team'` → `'free' | 'signal' | 'team' | 'enterprise'`
   - `isPro()` method: update tier checks
   - Tier display mapping

10. **`src/types/settings.ts`** — TypeScript types
    - `LicenseConfig.tier`: Update union type
    - Add `team_id`, `org_id`, `seat_count` fields

11. **`src/types/analysis.ts`** — ProValueReport interface (no change needed, type name is internal)

12. **`src/components/settings/LicenseSection.tsx`** — License settings tab
    - Display tier names: "Free", "Signal", "Team", "Enterprise"
    - Update upgrade messaging
    - Add team activation UI for Team tier

13. **`src/components/settings/StreetsMembershipSection.tsx`** — STREETS settings
    - Simplify or remove entirely — STREETS is free for everyone, no separate tier
    - Remove separate STREETS license activation UI (no longer needed)

14. **`src/components/ViewTabBar.tsx`** — Tab navigation
    - Progressive disclosure (`ViewTier: core → explorer → invested → power`) is engagement-based, NOT license-based
    - **No changes needed** — this is correct behavior (features unlock by usage, not payment)

15. **`src/App.tsx`** — Header tier badge
    - Update tier badge display text

#### Configuration

16. **`data/settings.example.json`** — Example config
    - `"license": { "tier": "free" }` — default stays the same

17. **Keygen dashboard** — External
    - Create new product/policies: "signal", "team", "enterprise"
    - Team tier: enable group licensing with seat count
    - Enterprise tier: custom license generation flow

18. **Landing page copy** (`docs/marketing/02-landing-page-copy.md`)
    - Update entire pricing section
    - "Pro" → "Signal" everywhere
    - Add Team and Enterprise cards

19. **i18n translation files** — All 11 languages
    - Any hardcoded "Pro" strings in translation keys

#### Renaming Strategy

**Option A (conservative):** Keep filenames as `ProGate.tsx`, `ProValuePanel.tsx` but change displayed text. Internal names don't matter.

**Option B (clean break):** Rename files to `SignalGate.tsx`, `SignalValuePanel.tsx`. More work, cleaner codebase.

**Recommendation:** Option A for now. File renames are low-value churn. Display text is what users see.

### Dependency Intelligence Architecture

```
                    ┌─────────────────────────┐
                    │   Dependency Parser      │
                    │                          │
                    │  package.json            │
                    │  Cargo.toml              │
                    │  requirements.txt        │
                    │  go.mod                  │
                    │  Gemfile                 │
                    │  pyproject.toml          │
                    │  pom.xml                 │
                    │  build.gradle            │
                    └────────────┬─────────────┘
                                 │
                    ┌────────────▼─────────────┐
                    │   Dependency Graph       │
                    │                          │
                    │  project → [dep@version] │
                    │  dep → [transitive deps] │
                    │  SQLite table: deps      │
                    └────────────┬─────────────┘
                                 │
                    ┌────────────▼─────────────┐
                    │   Cross-Reference Engine  │
                    │                          │
                    │  Incoming content         │
                    │    × Dependency graph     │
                    │    = Relevance signals    │
                    │                          │
                    │  CVE feed                │
                    │    × Dependency versions  │
                    │    = Security alerts      │
                    │                          │
                    │  Changelog               │
                    │    × Your dep versions   │
                    │    = Migration signals    │
                    └──────────────────────────┘
```

**DB schema addition:**
```sql
CREATE TABLE user_dependencies (
    id INTEGER PRIMARY KEY,
    project_path TEXT NOT NULL,
    package_name TEXT NOT NULL,
    version TEXT NOT NULL,
    ecosystem TEXT NOT NULL,  -- npm, cargo, pip, go, ruby, etc.
    is_direct INTEGER DEFAULT 1,
    detected_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL,
    UNIQUE(project_path, package_name, ecosystem)
);

CREATE INDEX idx_deps_package ON user_dependencies(package_name);
CREATE INDEX idx_deps_ecosystem ON user_dependencies(ecosystem);
```

### Team Context Protocol

```
                    Developer A                Developer B
                    ┌──────────┐              ┌──────────┐
                    │ 4DA      │              │ 4DA      │
                    │ (local)  │              │ (local)  │
                    │          │              │          │
                    │ ACE scan │              │ ACE scan │
                    │ Scoring  │              │ Scoring  │
                    │ Feedback │              │ Feedback │
                    └────┬─────┘              └────┬─────┘
                         │                         │
                    ┌────▼─────────────────────────▼────┐
                    │        Anonymous Relay             │
                    │   (self-hosted or 4da.ai)          │
                    │                                    │
                    │   Receives:                        │
                    │   - Stack summary (not files)      │
                    │   - Saved item IDs (not content)   │
                    │   - Decision hashes (not details)  │
                    │   - Aggregate topic embeddings     │
                    │                                    │
                    │   Returns:                         │
                    │   - "3 team members use lodash"    │
                    │   - "2 people saved this article"  │
                    │   - "Team stack: React, TS, PG"    │
                    └──────────────────────────────────┘
```

**Privacy invariant preserved:** The relay never sees raw data. It sees anonymous hashes and aggregated embeddings. Even if the relay is compromised, an attacker learns "someone on this team uses React" — nothing actionable.

---

## Part 10: The One-Line Summary for Each Stakeholder

**For the solo developer:** "4DA learns what you work on, tells you what matters, and gets smarter every day. After 3 months, it knows your stack better than you do."

**For the team lead:** "Every developer on your team sees the 3 things that happened overnight that affect your codebase. No Slack channels, no newsletters, no manual scanning."

**For the enterprise buyer:** "4DA runs on each developer's machine. Codebases never leave the device. Same binary, enterprise license key, config file via MDM. No server to provision. No data to secure. Your compliance team will love it."

**For the competitor analyst:** "They built a scoring engine in Rust that cross-references your actual dependency tree with the entire developer content ecosystem. And they did it local-first. To compete, you'd need to rebuild your entire architecture."

---

## Decision Required

Before implementation begins, one decision is needed:

**Do we rename "Pro" to "Signal" now, or keep "Pro" and add "Team" and "Enterprise" as new tiers?**

Arguments for renaming now:
- Clean break, consistent brand vocabulary from day one of post-launch
- "Signal" reinforces the core value prop every time it's mentioned
- Avoids the "Pro vs Team" confusion that every SaaS eventually hits

Arguments for keeping "Pro" temporarily:
- Existing customers already think of themselves as "Pro" users
- Renaming mid-flight creates communication overhead
- "Pro" is universally understood (low cognitive load)

**Recommendation:** Rename to "Signal" now. It's early enough that the customer base is small. The brand differentiation is worth the one-time communication cost. Ship the rename alongside the Team tier launch so it feels like a cohesive pricing evolution, not a random rename.

---

---

## Part 11: Seamless & Optimal UX — The Full Journey Redesign

### Current Journey Audit (from deep codebase exploration)

The current UX journey is already well-choreographed:

```
Splash (800ms min) → Onboarding (4 steps) → First-Run Transition → Main App
    ↓                    ↓                        ↓                    ↓
 6-stage init    Welcome → Taste Test →     Preparing →           9 views
 Progress bar    Quick Setup → Calibration  Fetching →            Progressive
 Error retry     Skip/Resume capable        Analyzing →           disclosure
                                            Celebrating
```

**What's working well:**
- Taste Test gamifies calibration (15-30 card interactions)
- Progressive disclosure (core → explorer → invested → power) based on engagement, not payment
- Keyboard shortcuts (j/k navigation, space expand, o open, b toggle briefing)
- First-run celebration shows actual intelligence value in <15 minutes
- 24 Zustand slices give granular state management

**What needs optimization for post-launch:**

### Journey Optimization 1: The 30-Second Hook

**Problem:** Current time-to-value is ~15 minutes (onboarding + first analysis). Post-launch users have even less patience — they heard about 4DA, downloaded it, and need to feel the value *immediately*.

**Solution: Parallel initialization**
- Start ACE project scanning DURING onboarding step 2 (Taste Test), not after
- Begin source fetching during step 3 (Quick Setup) — user configures while data arrives
- By the time they hit step 4 (Calibration), first results are already scored
- First-Run Transition becomes a 5-second celebration, not a 5-minute wait

**Implementation:**
```
Step 1 (Welcome)     → Start background: detect project dirs
Step 2 (Taste Test)  → Start background: ACE scan detected dirs
Step 3 (Quick Setup) → Start background: fetch sources + embed context
Step 4 (Calibration) → Analysis already 80% done, show readiness
Celebration          → Results ready immediately
```

### Journey Optimization 2: The Morning Ritual

**Problem:** Users open 4DA on day 1, see great results, then forget about it. DAU drops after day 3.

**Solution: The Briefing as Morning Anchor**

```
┌─────────────────────────────────────────────────────────────────┐
│                    THE MORNING RITUAL                            │
│                                                                  │
│  8:00 AM  System tray notification (configurable)                │
│           "3 signals for you this morning"                       │
│                                                                  │
│  Click → Lightweight 400px tray popup                            │
│           Top 3 items, one line each                             │
│           [Open Full Briefing] [Dismiss]                         │
│                                                                  │
│  OR      CLI: `4da brief` in terminal                            │
│           Formatted briefing, no GUI needed                      │
│                                                                  │
│  OR      Email digest (Signal tier)                              │
│           HTML email, mobile-readable                             │
│           One-click to full briefing in app                       │
│                                                                  │
│  Result: User sees 4DA intelligence before they open their IDE   │
└─────────────────────────────────────────────────────────────────┘
```

**Notification rules:**
- Only notify if there's genuinely relevant content (don't cry wolf)
- Respect OS quiet hours / focus mode
- Configurable time, frequency, and content type
- "No news is good news" — if nothing relevant, don't notify

### Journey Optimization 3: The View Architecture Cleanup

**Current:** 9 views, progressively disclosed. This is mostly right, but the naming and order could be tighter.

**Current view order:** Briefing → Results → Playbook → Channels → Insights → Saved → Profile → Toolkit → Calibrate

**Proposed refinement:**

| Current | Proposed | Rationale |
|---------|----------|-----------|
| Briefing | **Briefing** | Stays — this is the daily anchor |
| Results | **Feed** | "Results" sounds like search results. "Feed" communicates "your personalized stream" |
| Playbook | **STREETS** | Brand name, more memorable than generic "Playbook" |
| Channels | **Watches** | "Channels" is ambiguous (YouTube? Slack?). "Watches" = standing queries you're watching |
| Insights | **Radar** | "Insights" is vague. "Radar" = Tech Radar + Decision Memory combined |
| Saved | **Library** | "Saved" is utilitarian. "Library" implies curated personal collection |
| Profile | **DNA** | "Profile" is generic. "DNA" = Developer DNA, reinforces the brand feature |
| Toolkit | **Toolkit** | Stays — clear and specific |
| Calibrate | **System** | "Calibrate" sounds technical. "System" = system health, tuning, diagnostics |

**Progressive disclosure (unchanged logic — this is already excellent):**
```
core:      Briefing, Feed, STREETS         (engagement: first 3 cycles)
explorer:  + Watches, Radar                (engagement: 3+ analysis cycles)
invested:  + Library, DNA                  (engagement: 5+ saves or 2+ decisions)
power:     + Toolkit, System               (engagement: 14+ days active)
```

### Journey Optimization 4: The Feedback Flywheel

**Problem:** Behavior learning exists but the feedback surface is thin. Only save/dismiss/not-relevant buttons. Users don't see the system learning.

**Solution: Visible learning + effortless feedback**

1. **Inline feedback on every result item:**
   ```
   ┌──────────────────────────────────────┐
   │ React 20 RC1 Released                │
   │ Score: 0.87 — matches your stack     │
   │                                       │
   │ [👍 Relevant] [👎 Not for me] [📌 Save] │
   │                                       │
   │ 💡 Scored high because: React in 3    │
   │    projects, dependency match          │
   └──────────────────────────────────────┘
   ```

2. **"Your 4DA is Learning" toast (monthly):**
   ```
   ┌──────────────────────────────────────┐
   │ 📈 Your relevance improved this month │
   │                                       │
   │ Accuracy: 72% → 84%                  │
   │ Based on 142 feedback signals         │
   │                                       │
   │ Topics learned: +3 (Rust async,       │
   │   WebAssembly, edge computing)        │
   │ Topics excluded: +1 (blockchain)      │
   └──────────────────────────────────────┘
   ```

3. **Keyboard shortcuts for feedback (already partially exist):**
   - `y` = relevant (upvote)
   - `n` = not relevant (downvote)
   - `s` = save to library
   - Users who give 5+ feedback signals/week have 3x retention

### Journey Optimization 5: Upgrade Experience

**Current:** ProGate shows a modal with license key input. Functional but not inspiring.

**Proposed: The Signal Upgrade Flow**

1. **Blurred preview (already exists)** — show Pro features with blur + "Unlock with Signal"
2. **Value proof before gate:** Show what the user is missing
   ```
   ┌──────────────────────────────────────┐
   │ 🔒 AI Briefing — Signal Feature       │
   │                                       │
   │ Your briefing would have contained:   │
   │ • 2 security alerts for your deps     │
   │ • 1 breaking change in Next.js        │
   │ • 3 trending tools in your stack      │
   │                                       │
   │ [Start 30-day free trial]             │
   │ [Upgrade to Signal — $12/mo]          │
   │                                       │
   │ "What would you have missed today?"   │
   └──────────────────────────────────────┘
   ```
3. **No account required for trial** — one click to start, zero friction
4. **Stripe checkout** — opens in browser, returns license key via deep link
5. **Activation celebration** — confetti + unlock animation when key is activated

### Journey Optimization 6: Team Onboarding

**For teams, the admin sets up once; developers get value immediately.**

```
Admin Flow:
1. Purchase Team license on 4da.ai/pricing ($29/user/mo × N seats)
2. Receive team activation code + team-config.json template
3. Customize team-config.json (approved sources, default context dirs, etc.)
4. Distribute config file (check into team repo, or push via MDM)
5. Share activation code with team (email, Slack, wiki)

Developer Flow:
1. Download 4DA (same binary — no special "team" build)
2. Open → Onboarding → Enter team activation code
3. 4DA loads team-config.json → sources pre-configured, context dirs set
4. Personal preferences merge on top of team defaults
5. That's it. Same app, better defaults.
```

**The key insight:** No admin console needed. A JSON config file distributed via
the team's existing infrastructure (Git, MDM, email) is simpler, more auditable,
and more enterprise-friendly than a custom web admin panel.

### Journey Optimization 7: Enterprise Self-Service

**Enterprise buyers don't want to "Contact Sales" and wait 2 weeks.**

```
Enterprise Flow:
1. Visit 4da.ai/pricing → see transparent enterprise pricing
2. Calculator: $22/user/mo × 50 seats × 12 months = $13,200/yr
3. Click "Buy Enterprise" → Stripe checkout
4. Receive: enterprise license key + team-config.json template + docs package
5. IT distributes 4DA binary via MDM (Jamf, Intune, SCCM)
6. Each developer enters enterprise code → activated instantly
7. SSO (if needed): follow setup guide → WorkOS handles identity

Timeline: <1 hour from "I want this" to "my team is using it"
No sales calls required under 100 seats.
```

**Why this works for 4DA:** The product is a desktop app. There IS no complex cloud
infrastructure to provision. Enterprise "deployment" is literally "push a binary to
managed devices via MDM + distribute a config file." This is how enterprises already
deploy tools like VS Code, Slack, and Docker Desktop. 4DA fits into existing workflows.

---

## Part 12: The Seamless Experience Principles

### P1: No Dead Ends
Every screen should have a clear next action. If there are no results, show why and what to do about it. If a feature is locked, show what you'd see if it wasn't. Never show an empty state without guidance.

### P2: Intelligence, Not Information
4DA doesn't show you data. It tells you what matters and why. Every screen should have a "so what?" answer embedded in the UI. "React 20 released" is information. "React 20 released — you use React 19.0 in 3 projects, here's what changes" is intelligence.

### P3: Ambient, Not Attention-Seeking
4DA should never interrupt flow state. Notifications are gentle, dismissible, and respect OS focus modes. The heartbeat (Void Engine) communicates system state without demanding attention. The app should feel like a quiet assistant, not a needy notification machine.

### P4: Your Model, Your Investment
Users should feel that their 4DA is *theirs* — trained on their behavior, tuned to their stack, learning from their decisions. This creates switching cost through attachment, not lock-in through data hostage. Every interaction makes the model better. Every day without 4DA is a day your model doesn't learn.

### P5: Team Intelligence = Personal Intelligence × N
Team features don't replace personal intelligence — they amplify it. A team member's personal briefing gets enriched by anonymous team signals. A team's Tech Radar shows collective adoption patterns. The individual experience gets better with a team, not different.

### P6: Enterprise = Same App, More Trust
Enterprise features are trust features: SSO (trust authentication), audit logs (trust compliance), self-hosted (trust data residency), admin console (trust governance). The core product experience is identical. Enterprise customers don't get a different app — they get the same app with institutional trust guarantees.

---

## Part 13: Naming & Brand Vocabulary

### The 4DA Vocabulary System

Every product should have a consistent vocabulary. Here's 4DA's:

| Concept | Word | NOT |
|---------|------|-----|
| Content stream | **Feed** | Results, items, list |
| AI summary | **Briefing** | Digest, newsletter, report |
| Relevance engine | **PASIFA** | Algorithm, AI, model |
| Context discovery | **ACE** | Scanner, indexer, crawler |
| User identity | **Developer DNA** | Profile, account, identity |
| Content quality | **Signal** | Score, rating, relevance |
| Noise rejection | **Gate** | Filter, block, hide |
| Dependency tracking | **Watch** | Monitor, track, follow |
| Learning system | **Memory** | Learning, training, AI |
| Revenue playbook | **STREETS** | Course, tutorial, guide |
| Ambient indicator | **Heartbeat** | Status, indicator, icon |
| Tier 1 (free) | **Free** | Basic, starter, lite |
| Tier 2 (paid individual) | **Signal** | Pro, premium, plus |
| Tier 3 (paid team) | **Team** | Business, organization |
| Tier 4 (enterprise) | **Enterprise** | Corporate, custom |

### Brand Taglines by Tier

- **Free:** "All signal. No feed."
- **Signal:** "Your intelligence, synthesized."
- **Team:** "Shared intelligence. Private data."
- **Enterprise:** "Institutional intelligence. Zero compromise."

---

*This document is the strategic north star. Implementation begins with Part 6's sequence. Each phase gets its own spec before coding starts.*
