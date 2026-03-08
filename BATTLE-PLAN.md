# 4DA BATTLE PLAN

> Simulation-driven launch strategy. Every decision is explored before planned, planned before implemented.
> Created: 8 March 2026
> Status: ACTIVE
> Phase 0.1: First-launch experience — COMPLETE
> Phase 0.2: Dead command audit — COMPLETE (1 bug fixed: get_latest_digest)
> Phase 0.3: Network manifest — COMPLETE (NETWORK.md, 290 lines)
> Phase 0.4: Offline resilience — COMPLETE (DB corruption recovery added)
> Phase 1.1: Local telemetry — COMPLETE (telemetry.rs, use-telemetry.ts, Phase 25 migration)
> Phase 2: Trusted testers — NOT STARTED (requires external action)
> Method: Simulate all scenarios → identify highest-EV path → build for the probable case → prepare contingencies for edge cases

---

## Part 1: Reality Model

### What We Have (Verified)

| Metric | Value | Assessment |
|--------|-------|------------|
| Rust backend | 104,435 lines | Deep, production-grade |
| TypeScript frontend | 40,761 lines | Mature UI layer |
| Tauri commands (registered) | 152 | Large but tier-gated |
| Tauri commands (exist but unwired) | ~56 | Dead code candidates |
| Frontend views | 9 (4-tier disclosure) | Already solved |
| DB tables | 44 (24-phase migration) | Well-structured |
| Content sources | 10 | Broad coverage |
| Test count | 2,450 (812 FE + 1,638 BE) | Strong for solo dev |
| i18n keys | 1,437 | Full localization ready |
| Commits | 371 in 47 days | High velocity |
| Users | 0 | The problem |
| Revenue | $0 | The bigger problem |
| Production build | Compiles to NSIS installer | Verified working |

### What We Don't Know (The Blind Spots)

1. **Does the value proposition land in 10 seconds?** We've never watched someone encounter 4DA cold.
2. **Which of the 9 views actually matter?** Intuition says briefing + results. Data says nothing.
3. **Does progressive disclosure feel magical or confusing?** Tier unlocking is built but unobserved.
4. **Is the ghost preview persuasive?** The copy is refined but no click-through data exists.
5. **Will people configure Ollama?** The synthesis feature requires it. Adoption rate: unknown.
6. **What's the install-to-first-value latency?** Target is 60 seconds. Actual: unmeasured.
7. **Does the PASIFA score feel meaningful?** Or does it feel like a random number?
8. **Price sensitivity.** $0 to $X/month threshold: unknown.

---

## Part 2: Scenario Simulations

### User Archetype Simulations

We model 5 developer archetypes that represent the probable user distribution.

#### Archetype A: "The Curious Tinkerer" (40% of early adopters)

**Profile:** Mid-level dev, sees 4DA on HN/Reddit, installs because it sounds interesting.
**First 5 minutes simulation:**
```
0:00  Downloads installer from 4da.ai
0:30  Installer finishes, app opens
0:45  Sees splash screen → briefing view
1:00  Briefing says "awaiting first analysis"
1:15  Confusion: "Do I need to do something?"
      → DECISION POINT: Does onboarding guide them to run analysis?
      → If YES: clicks "Run Analysis" → waits 30-60s → sees first results
      → If NO: clicks around tabs → sees empty views → closes app
2:00  If analysis ran: sees 5-15 scored items
2:30  "What are these scores?" → hovers → sees PASIFA breakdown
3:00  Clicks a result → reads it → thinks "I could get this from HN"
      → DECISION POINT: Does the scoring feel different from a feed?
      → If YES: "Oh, it's ranked for ME" → explores more
      → If NO: "This is just another RSS reader" → closes app
5:00  If still engaged: notices stack health → "How does it know I use React?"
      → ACE detection feels magical → user is hooked
```

**Probable outcome:** 50% bounce at minute 1 (empty state), 25% bounce at minute 3 (doesn't feel different), 25% become weekly users.

**Edge cases:**
- User has no projects on their machine → ACE finds nothing → stack health is empty → the "magic" never fires
- User is on macOS or Linux → no installer exists yet → 100% bounce
- User's antivirus flags the unsigned installer → trust destroyed before first launch

**Counter-strategies:**
- [ ] Ensure first analysis auto-triggers on first launch (no manual step)
- [ ] Add "demo data" or sample results for truly cold starts
- [ ] Cross-platform builds before launch (at minimum macOS)
- [ ] Code signing certificate for installer trust

#### Archetype B: "The Tool Collector" (25% of early adopters)

**Profile:** Senior dev, evaluates new tools regularly, has strong opinions.
**First 5 minutes simulation:**
```
0:00  Reads landing page carefully before installing
      → DECISION POINT: Does 4da.ai explain the product in 10 seconds?
      → Currently: 4da.ai exists but landing page quality unknown
0:30  Installs, opens settings FIRST (power user behavior)
1:00  Sees LLM config → "Oh, it uses Ollama" → already has it installed
1:30  Configures GitHub token, RSS feeds, maybe Twitter
2:00  Runs analysis → impressed by source breadth
3:00  Tries Intelligence Console → searches their stack
3:30  Gets synthesis → "This actually knows my stack" → the "holy shit" moment
4:00  Tries calibration → TasteTest → scores improve
5:00  This user becomes a Pro candidate
```

**Probable outcome:** 70% become power users if they reach minute 3. 30% bounce because landing page didn't convince them to install.

**Edge cases:**
- User has Ollama but uses a non-default port/model → synthesis fails silently
- User expects instant results but analysis takes 45s → impatience
- User finds a scoring error → one bad result undermines trust in all results

**Counter-strategies:**
- [ ] Ollama auto-detection (check common ports, show clear status)
- [ ] Progress indicator during analysis with estimated time
- [ ] Score Autopsy for every result (already built — ensure it's discoverable)

#### Archetype C: "The Privacy Advocate" (15% of early adopters)

**Profile:** Security-conscious dev who specifically chose 4DA because it's local-first.
**First 5 minutes simulation:**
```
0:00  Reads FSL license carefully. Checks GitHub for source.
0:30  Inspects settings.json location. Checks what data is stored.
1:00  Runs Wireshark alongside 4DA to verify no phone-home
      → DECISION POINT: Does 4DA make ANY unexpected network calls?
      → Known calls: source fetching, Ollama (local), Keygen (license check)
      → Risk: Keygen check on Pro validation hits external server
1:30  If satisfied: becomes the most loyal user type
2:00  If any unexpected call found: writes a blog post about it → reputation damage
```

**Probable outcome:** 80% become advocates if privacy claims hold. 20% find something that feels wrong (even if benign) and become critics.

**Edge cases:**
- Keygen license validation requires internet → contradicts "works offline" claim
- YouTube source fetches from Google → user expected zero external calls
- Error reporting/telemetry accidentally enabled → catastrophic trust violation

**Counter-strategies:**
- [ ] Audit ALL network calls and document them transparently
- [ ] Keygen validation must gracefully degrade offline (cache last-known-good)
- [ ] Add a "Network Activity" panel in diagnostics showing all outbound calls
- [ ] Privacy page on 4da.ai with complete network call manifest

#### Archetype D: "The Enterprise Evaluator" (10% of early adopters)

**Profile:** Team lead evaluating tools for their team. Needs to justify the purchase.
**First 5 minutes simulation:**
```
0:00  Needs: "Can I show this to my team?"
      → DECISION POINT: Is there a team tier? Shared config? SSO?
      → Currently: Team tier exists in pricing but no team features built
1:00  Installs, evaluates. Wants export functionality.
2:00  Tries exporting developer DNA → gets markdown/SVG
3:00  "Can I share a briefing?" → export exists but sharing UX unclear
4:00  Asks: "What about SOC2? Data residency?" → local-only is actually the answer
5:00  If convinced: potential $50+/month team customer
```

**Probable outcome:** 60% defer ("not ready for teams yet"), 30% adopt personally first, 10% walk away.

**Edge cases:**
- Corporate firewall blocks source fetching → app appears broken
- IT policy prevents unsigned app installation → blocked at step 0
- User needs audit trail → existing logging may not be sufficient

**Counter-strategies:**
- [ ] Proxy support for corporate environments
- [ ] Code signing (repeat — critical for enterprise)
- [ ] "Export briefing" as shareable HTML/PDF
- [ ] Defer team features — focus on individual excellence first

#### Archetype E: "The Accidental User" (10% of early adopters)

**Profile:** Clicked a link, didn't read much, installed on impulse.
**First 60 seconds simulation:**
```
0:00  Opens app. Sees dark UI.
0:10  "What is this?" → no immediate answer on screen
      → DECISION POINT: Does the first screen explain itself?
0:20  If confused → closes app, never returns
0:30  If a single sentence on the briefing view says what 4DA does → stays
```

**Probable outcome:** 80% bounce within 30 seconds. 20% stumble into value.

**Counter-strategies:**
- [ ] First-visit overlay or inline explanation (one sentence, dismissible)
- [ ] Briefing view should have content even before analysis (curated/demo)

---

### Launch Channel Simulations

#### Channel 1: Hacker News "Show HN"

**Simulation:**
```
Post title: "Show HN: 4DA – Local-first intelligence console for developers"
Post body: 2-3 sentences + link to 4da.ai

Hour 0-1: 3-5 upvotes from initial circle
Hour 1-3: Either gains traction or dies
  → Traction scenario (30% probability):
    - Hits front page position 20-30
    - 200-400 clicks to 4da.ai
    - 15-30% click "download" → 30-120 downloads
    - Comments: "Looks cool but Windows only?" (macOS ask)
    - Comments: "What does this do that X doesn't?" (positioning challenge)
    - Comments: "FSL license? So it's not really open source" (license debate)
  → Dies scenario (70% probability):
    - 5-8 upvotes, falls off page 2
    - 20-40 clicks total
    - 5-10 downloads
    - Silence
```

**Counter-strategies for traction scenario:**
- [ ] Have macOS build ready before posting (or acknowledge Windows-first clearly)
- [ ] Prepare 3-sentence "how is this different" answer for comments
- [ ] Prepare FSL license explanation that's honest and non-defensive
- [ ] Have demo video/GIF on landing page (don't make people install to understand)

**Counter-strategies for dies scenario:**
- [ ] Don't post on HN first. Build 5-10 organic users first. Then post with their testimonials.
- [ ] Alternative: post a technical article about PASIFA scoring or local vector search, with 4DA as the example. Technical content performs better than product launches on HN.

#### Channel 2: Reddit (r/programming, r/rust, r/webdev)

**Simulation:**
```
r/programming: High bar. "Show my project" posts get 0-5 upvotes unless genuinely novel.
r/rust: More receptive to Rust projects. Tauri + sqlite-vec angle could work.
r/webdev: Wrong audience (4DA isn't a web dev tool).
r/selfhosted: STRONG fit. Local-first, privacy-first, no cloud. This is their language.
```

**Optimal Reddit strategy:**
- [ ] r/selfhosted first (natural fit, receptive to local-first)
- [ ] r/rust second (technical respect for the engineering)
- [ ] r/programming only after organic traction exists

#### Channel 3: Direct Outreach

**Simulation:**
```
Hand-pick 10 developers you respect. DM them the installer + 2-sentence pitch.
Expected response rate: 40-60% try it. 20-30% give feedback.
That's 2-3 people giving you real data.
```

**This is the highest-ROI channel for pre-launch. No public risk. Maximum learning.**

---

## Part 3: The Battle Plan

### Operating Principles

1. **Simulate before exploring.** Model what will happen. Identify the decision points.
2. **Explore before planning.** Gather real data at decision points. Don't plan from assumptions.
3. **Plan before implementing.** Once data confirms direction, plan the minimum implementation.
4. **Implement with contingencies.** Build the probable path. Prepare pivots for edge cases.
5. **Measure everything locally.** No external analytics. SQLite event table. Privacy-first telemetry.

---

### Phase 0: Internal Hardening (Week 1)
> Goal: Fix the things that would embarrass us if 5 developers used the product tomorrow.

#### 0.1 First-Launch Experience Audit

**Simulation:** Fresh install on a clean Windows machine. No projects. No Ollama. No API keys.

**Expected failures:**
- Briefing view shows "awaiting analysis" with no guidance
- Stack health shows nothing (no ACE data)
- Intelligence Console shows capability preview but no results
- Progressive disclosure starts at "core" tier — only 3 views visible

**Required fixes:**
- [ ] Auto-trigger initial scan on first launch (remove manual step)
- [ ] If no ACE projects found, show a "Add your first project" prompt
- [ ] Briefing should show SOMETHING within 60 seconds (even if it's "Scanning 3 sources...")
- [ ] Add analysis narration to the briefing view (the system already emits narration events)

**Verification gate:** Record a screen capture of fresh install → first useful result. Must be < 90 seconds.

#### 0.2 Dead Command Cleanup

**Fact:** ~56 Tauri commands exist in code but aren't registered in invoke_handler. These are dead code.

**Action:**
- [ ] Identify all `#[tauri::command]` functions not in invoke_handler
- [ ] For each: confirm it's unused → delete or comment with `// TODO: wire up when X feature ships`
- [ ] Reduces attack surface and cognitive load

**Verification gate:** `grep -c "#\[tauri::command\]"` in src/ matches count of commands in invoke_handler ± 5.

#### 0.3 Network Call Manifest

**Why:** Archetype C (Privacy Advocate) will check. If they find an undocumented call, trust is destroyed.

**Action:**
- [ ] Audit every `reqwest` call, every `fetch`, every outbound connection
- [ ] Document in a `NETWORK.md` or in-app diagnostics panel:
  ```
  Source fetching: HN, Reddit, arXiv, GitHub, RSS, YouTube, Dev.to, Lobsters, ProductHunt, Twitter
  LLM calls: Ollama (localhost), or user-configured API (OpenAI/Anthropic/etc)
  License: Keygen.sh (Pro tier validation only, graceful offline degradation)
  Updates: Tauri updater check (configurable)
  Nothing else. Zero telemetry. Zero analytics. Zero phone-home.
  ```

**Verification gate:** Run app with Wireshark for 10 minutes. Every outbound call matches the manifest.

#### 0.4 Offline Resilience Check

**Simulation:** Disconnect internet. Launch 4DA. What happens?

**Expected:**
- Source fetching fails → should show cached results, not crash
- Ollama (local) still works → synthesis should still work
- Keygen validation fails → Pro features should use cached license state
- App should be fully functional with stale data

**Action:**
- [ ] Test offline launch end-to-end
- [ ] Ensure Keygen validation caches last-known-good license for 7 days
- [ ] Source fetching failures should show "Offline — showing cached results" not empty state

**Verification gate:** App launches offline, shows cached briefing, synthesis works via local Ollama.

---

### Phase 1: Local Telemetry (Week 1, parallel with Phase 0)
> Goal: When real humans use this, we learn from every interaction without violating privacy.

#### 1.1 Event Tracking Table

**Design:**
```sql
CREATE TABLE IF NOT EXISTS user_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,      -- 'view_open', 'search', 'ghost_click', 'upgrade_click', 'synthesis_trigger', etc.
    view_id TEXT,                  -- which view they were on
    metadata TEXT,                 -- JSON blob for event-specific data
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    session_id TEXT               -- groups events within a single app session
);
```

**Events to track (minimum viable):**
| Event | Why |
|-------|-----|
| `app_launch` | Session count, daily active |
| `view_open:{view_id}` | Which views people actually use |
| `view_time:{view_id}` | How long they spend per view (tracked on view switch) |
| `analysis_triggered` | Manual vs auto, first vs repeat |
| `analysis_complete` | Duration, result count, relevant count |
| `search_query` | Query text (local only), result count |
| `ghost_preview_shown` | Conversion funnel step 1 |
| `ghost_preview_clicked` | Conversion funnel step 2 |
| `synthesis_triggered` | LLM usage rate |
| `synthesis_completed` | LLM latency |
| `watch_created` | Standing query adoption |
| `tier_unlocked:{tier}` | Progressive disclosure progression |
| `ollama_configured` | LLM adoption rate |
| `first_feedback` | Time from install to first relevance feedback |
| `source_configured` | Which sources people add |

**Privacy constraint:** ALL data stays in local SQLite. No export. No aggregation server. The user can delete this table at any time.

**Tauri command:**
```rust
#[tauri::command]
pub async fn get_usage_analytics(days: Option<u32>) -> Result<UsageReport, String>
```
Shows the user their own usage patterns. Transparency as a feature.

#### 1.2 Analytics Dashboard (In-App)

**Not for us — for the user.** But we learn from it when testers share screenshots.

- View usage heatmap (which views they opened this week)
- Search frequency graph
- "Your 4DA this week: X searches, Y briefings, Z decisions tracked"
- This doubles as a retention feature and data collection method

---

### Phase 2: Trusted Tester Program (Week 2)
> Goal: Get 5-10 developers using 4DA and observing their actual behavior.

#### 2.1 Tester Selection Criteria

| Criterion | Why |
|-----------|-----|
| Uses Windows as primary dev machine | Our only platform currently |
| Has 2+ projects on their machine | ACE needs something to scan |
| Willing to spend 30 min and report back | We need qualitative data |
| Technical enough to install Ollama | Synthesis is the differentiator |
| NOT in our immediate circle | Honest feedback, not politeness |

**Target: 5 minimum, 10 maximum.** More than 10 creates support burden without proportional learning.

#### 2.2 Tester Onboarding Protocol

```
1. Send installer + 3-sentence pitch (no manual, no docs)
2. Ask them to record their screen for first 5 minutes (Loom/OBS)
3. After 1 week: 15-minute call. Questions:
   - What did you use? What did you ignore?
   - What confused you?
   - Would you pay for this? At what price?
   - What would make you recommend this?
4. After 2 weeks: check if they're still using it (retention signal)
```

#### 2.3 Decision Gates from Tester Data

| Signal | Action |
|--------|--------|
| 4/5 testers bounce at "awaiting analysis" | First-launch auto-scan is critical path. Fix before anything else. |
| 3/5 testers never find Intelligence Console | Rename, reposition, or promote it in the UI |
| 0/5 testers configure Ollama | Synthesis can't be the differentiator. Pivot to non-LLM value. |
| 3/5 testers say "I don't understand the score" | PASIFA needs a one-sentence explanation inline |
| 5/5 testers use briefing + results only | Cut or heavily deprioritize other views |
| 2/5 testers say they'd pay $10+/month | Price validation. Proceed with Pro tier. |
| 0/5 testers would pay anything | Value proposition needs rework before monetization. |
| 3/5 testers ask "does this work on Mac?" | macOS build moves to critical path. |
| Any tester finds a privacy concern | Stop everything. Fix. Verify. Then continue. |

**These gates are non-negotiable.** We don't proceed past Phase 2 without tester data informing the direction.

---

### Phase 3: Surface Reduction (Week 3, informed by tester data)
> Goal: Cut everything that doesn't contribute to the first 10 minutes.

#### 3.1 Command Triage

Categorize all 152 registered commands:

**Tier 1 — Core (keep, polish):** ~60 commands
- Analysis, scoring, results, feedback
- Settings, monitoring, source config
- Briefing, digest
- Intelligence Console (search, synthesis, stack health)
- ACE context detection

**Tier 2 — Valuable (keep, lower priority):** ~50 commands
- Decision tracking, tech radar
- Channels, saved items
- Playbook/STREETS
- Sovereign profile, developer DNA

**Tier 3 — Speculative (gate behind Labs or defer):** ~42 commands
- Toolkit (port scanning, HTTP requests, process kill)
- Community intelligence
- Signal chains, semantic shifts
- Agent memory, delegation scoring
- GAME engine achievements
- Template system

**Action:** Tier 3 commands get a `// LABS` comment. They stay compiled but aren't in the default UI. Power users can enable them via a `labs: true` setting.

#### 3.2 View Simplification

The 4-tier progressive disclosure system is already built. Based on tester data, we may:

**Probable adjustment:** Reduce initial views from 3 (briefing, results, playbook) to 2 (briefing, results). Playbook unlocks after first analysis.

**Reasoning:** A new user opening the app and seeing "STREETS Course" alongside their intelligence feed creates confusion. "Is this an educational tool or an intelligence tool?"

#### 3.3 Source Prioritization

10 sources is a lot. Based on signal quality:

**Always active (zero config):** HN, arXiv, Reddit, Lobsters, Dev.to, Product Hunt
**Opt-in (requires user action):** RSS, GitHub, YouTube, Twitter

**Reasoning:** The zero-config sources work immediately. Opt-in sources require API keys or configuration. New users should see value before being asked to configure things.

---

### Phase 4: Conversion Infrastructure (Week 4)
> Goal: When someone wants to pay, the path from desire to purchase has zero friction.

#### 4.1 Conversion Flow

```
Free user searches → sees ghost preview → clicks "Unlock intelligence"
  → In-app modal: "4DA Pro — $X/month"
  → "What you get: [list derived from ghost preview data]"
  → "Buy now" → opens 4da.ai/pro/checkout in browser
  → Stripe Checkout (single product, monthly)
  → Stripe webhook → Keygen creates license
  → User receives license key via email
  → User enters license in Settings → Pro activates
```

**Why not in-app purchase?** Tauri doesn't support native payments. Browser checkout is standard for desktop apps.

#### 4.2 Pricing Simulation

| Price | Expected conversion rate | Monthly revenue per 100 users | Notes |
|-------|------------------------|------------------------------|-------|
| $5/mo | 8-12% | $40-60 | Too cheap. Attracts low-intent users. Hard to sustain. |
| $9/mo | 5-8% | $45-72 | Sweet spot for solo devs. Feels "fair." |
| $15/mo | 3-5% | $45-75 | Requires strong value proof. Better for power users. |
| $29/mo | 1-3% | $29-87 | Enterprise territory. Needs team features to justify. |

**Recommended starting price:** $12/month or $99/year.

**Reasoning:**
- $12/month is above impulse-buy threshold (forces genuine value assessment)
- $99/year ($8.25/month effective) gives 30% discount for commitment
- Annual plan reduces churn and provides runway
- Price can be adjusted down easily, adjusting UP is painful

**Simulation: first 90 days post-launch:**
```
Day 1-7:   100 downloads, 30 active after day 1, 15 active after day 7
Day 8-30:  50 more downloads (organic), 20 total active users
           Ghost preview shown: ~200 times
           Ghost preview clicked: ~30 times (15% CTR)
           Checkout opened: ~10 times
           Purchases: 2-4 ($24-48/month)
Day 31-90: Growth slows without marketing. 200 total downloads.
           Active users: 30-40
           Pro subscribers: 5-10 ($60-120/month)
```

**This is not a business yet. This is validation.** 5-10 paying users proves the model. Then we optimize.

#### 4.3 Stripe Integration (Minimum Viable)

- Single product: "4DA Pro"
- Two prices: monthly ($12) and annual ($99)
- Stripe Checkout (hosted page, not embedded)
- Webhook endpoint: a simple Cloudflare Worker or Vercel Edge Function
  - Receives `checkout.session.completed`
  - Calls Keygen API to create license
  - Sends license key via email (Resend or similar)
- Total infrastructure: 1 serverless function, 1 Stripe account, 1 Keygen account

**Privacy note:** The webhook function is the ONLY server-side code. It processes payment → creates license → done. No user data is stored server-side.

---

### Phase 5: Launch Preparation (Week 5)
> Goal: Everything is ready for the first public mention.

#### 5.1 Landing Page (4da.ai)

**Structure (simulated from archetype analysis):**

```
[Hero]
  Headline: "Your intelligence feed. Local. Private. Yours."
  Subhead: One sentence explaining what 4DA does.
  CTA: "Download for Windows" (macOS coming soon)
  Demo GIF: 10-second loop showing search → synthesis → stack health

[How it works]
  3 steps with screenshots:
  1. Install. Your tech stack is detected automatically.
  2. Search. Natural language queries across 10 sources.
  3. Synthesize. AI briefing grounded in YOUR context.

[Why local?]
  "Your data never leaves your machine. No accounts. No cloud. No tracking."
  Technical credibility: "Built with Rust + SQLite + Ollama"

[Pricing]
  Free: 3 search results, stack health, briefing, 6 sources
  Pro ($12/mo): Full results, AI synthesis, standing queries, decision tracking
  Simple. No tricks. No "contact sales."

[Footer]
  GitHub (source-available) | Privacy | FSL-1.1 License | Trademark notice
```

**Critical: the page must work without JavaScript.** Privacy-conscious users often run NoScript.

#### 5.2 README / GitHub Presence

The repo is at github.com/runyourempire/4DA. It needs:
- [ ] Clear description: what it is, who it's for, how to install
- [ ] Screenshot or demo GIF at the top
- [ ] "Why source-available?" section explaining FSL license
- [ ] Build instructions for contributors (if any)
- [ ] Link to 4da.ai for downloads

#### 5.3 Pre-Launch Checklist

| Item | Status | Blocking? |
|------|--------|-----------|
| Windows installer builds | Done | - |
| macOS build | Not started | Soft blocker (limits audience 50%) |
| Code signing | Not started | Soft blocker (antivirus/trust) |
| Landing page | Not started | Hard blocker |
| Stripe checkout | Not started | Hard blocker for revenue |
| Keygen integration | Done | - |
| Privacy audit | Done (NETWORK.md) | - |
| First-launch experience | Done (warmup + empty states) | - |
| Demo video/GIF | Not started | Hard blocker for landing page |
| Local telemetry | Done (Phase 25 migration) | - |
| Tester feedback | Not started | Hard blocker (per battle plan) |

---

### Phase 6: Controlled Launch (Week 6)
> Goal: First public availability. Small. Measured. No hype.

#### 6.1 Launch Sequence

```
Day 0: Publish to 4da.ai. No announcements.
        Test the full flow: download → install → first value → ghost preview → checkout → Pro activation

Day 1: Post to r/selfhosted (natural fit, receptive audience)
        Title: "I built a local-first intelligence feed for developers"
        Honest, technical, no marketing language

Day 3: If r/selfhosted reception is positive → post to r/rust
        Angle: "Built with Tauri 2 + Rust + sqlite-vec — local vector search for developer intelligence"

Day 7: If both positive → Show HN
        By now we have real users and real feedback to reference

Day 14: Evaluate. Decision gate:
        - If >50 active users and >2 Pro subscribers: continue current path
        - If >50 active users and 0 Pro subscribers: pricing/value problem — adjust
        - If <20 active users: distribution/positioning problem — rethink launch channels
        - If <5 active users: product-market fit problem — go back to testers
```

#### 6.2 Response Protocols

**If someone reports a bug:**
- Respond within 4 hours (shows the product is alive)
- Fix within 24 hours if possible
- Ship a patch and comment with the fix
- This builds trust faster than any marketing

**If someone asks "does it work on Mac?":**
- "macOS build is in progress. Can I email you when it's ready?" → capture leads
- This is also a signal to accelerate macOS build

**If someone criticizes the license:**
- Prepared response: "FSL-1.1 means you can read the source and verify our privacy claims, but can't build a competing product. After 2 years, it converts to Apache 2.0. We chose this to sustain development while being transparent."
- Don't be defensive. Acknowledge the tradeoff.

**If someone finds a security issue:**
- Thank them publicly
- Fix within 24 hours
- If it's serious: pull the download, fix, re-release
- This is the one scenario where speed matters more than polish

---

## Part 4: Contingency Matrix

For every probable scenario, the pre-planned response:

### Product Scenarios

| Scenario | Probability | Response |
|----------|-------------|----------|
| Users love briefing, ignore everything else | 35% | Double down on briefing quality. Make it the hero feature. Other views become "Explore more" secondary. |
| Users love search + synthesis, ignore briefing | 25% | Rename Intelligence Console to primary view. Briefing becomes secondary. |
| Users want mobile/web access | 20% | Acknowledge demand. Don't build it yet. "Local-first means desktop-first. Mobile companion is on the roadmap." |
| A competitor launches something similar | 10% | Differentiate on privacy and scoring quality. They can't copy your local-first architecture easily. |
| Ollama adoption is near-zero | 30% | Build a free cloud synthesis endpoint (rate-limited, privacy-preserving). Or partner with a local LLM provider. |
| macOS demand is overwhelming | 60% | Prioritize macOS build immediately. Tauri supports it — the work is build config, not code. |

### Technical Scenarios

| Scenario | Probability | Response |
|----------|-------------|----------|
| A content source breaks (API change) | 80% within 6 months | Source health monitoring already exists. Auto-disable broken source, notify user, fix within 48h. |
| SQLite database grows too large (>1GB) | 15% | Autophagy system already handles this (digest old content, prune). Add a "Database size" indicator in diagnostics. |
| Ollama model changes break synthesis | 20% | LLM client already handles multiple models. Add model validation on startup. |
| Windows Defender flags the app | 30% | Code signing is the fix. Until then: document how to whitelist. |
| Tauri 3.0 ships with breaking changes | 40% within 12 months | Pin to Tauri 2.x. Upgrade on our schedule, not theirs. Test in a branch first. |
| rusqlite or sqlite-vec major update | 25% | The 24-phase migration system handles schema changes. Dependency updates are a controlled process. |

### Business Scenarios

| Scenario | Probability | Response |
|----------|-------------|----------|
| 0 Pro subscribers after 30 days | 40% | The product is free value only. Either: lower price, improve ghost preview persuasion, or add a time-limited Pro trial. |
| 10+ Pro subscribers in first month | 10% | Validation. Focus on retention: monthly digest email, feature announcements, direct relationship with each subscriber. |
| Someone asks about team pricing | 15% | "Coming soon. What would your team need?" → capture requirements before building. |
| A large company wants an enterprise deal | 5% | Don't panic. "We're focused on individual developers right now. Happy to discuss when our team tier launches." Buy time. |
| Refund request | 20% | Instant refund, no questions. Ask why. Learn from it. |
| Chargeback/fraud | 5% | Stripe handles disputes. Keygen revokes license. Minimal exposure on monthly billing. |

---

## Part 5: Success Metrics

### Week 1-2 (Internal Hardening + Testers)

| Metric | Target | Red Flag |
|--------|--------|----------|
| First-launch to first result | < 90 seconds | > 3 minutes |
| Tester retention (day 7) | 3/5 still using | 1/5 still using |
| Critical bugs found by testers | < 5 | > 10 |
| Privacy audit: undocumented network calls | 0 | Any |

### Week 3-4 (Surface Reduction + Conversion)

| Metric | Target | Red Flag |
|--------|--------|----------|
| Registered commands after cleanup | < 130 | Still at 152 |
| Checkout flow end-to-end test | Works | Any step fails |
| Landing page load time | < 2 seconds | > 5 seconds |

### Week 5-6 (Launch)

| Metric | Target | Red Flag |
|--------|--------|----------|
| Downloads in first week | 50+ | < 10 |
| Day-7 retention | 30%+ | < 10% |
| Ghost preview CTR | 10%+ | < 3% |
| Pro conversions (first 30 days) | 3+ | 0 |
| Negative public feedback | 0 reputation-damaging | Any security/privacy issue |

### Month 2-3 (Growth)

| Metric | Target | Red Flag |
|--------|--------|----------|
| Monthly active users | 50+ | Declining |
| Pro subscriber retention | 80%+ | < 50% |
| Support requests per user | < 0.5/month | > 2/month |
| Source uptime (all 10 sources) | 95%+ | Any source broken > 1 week |

---

## Part 6: What We're NOT Doing

Explicit decisions to defer. These are not on the roadmap until post-launch data says otherwise.

| Feature | Why Not Now |
|---------|------------|
| macOS build | Priority but not blocking Windows launch. Parallel workstream. |
| Linux build | Smallest audience. After macOS. |
| Mobile companion | Fundamentally different product. Not now. |
| Cloud sync | Contradicts local-first. Only if users demand it for multi-device. |
| Team features | No team customers yet. Don't build for imaginary demand. |
| Plugin system | Engineering complexity without user demand. |
| Public API | No external integrations needed yet. |
| i18n beyond English | 1,437 keys ready but no non-English users to test with. |
| Social features | Contradicts privacy positioning. |
| Chrome extension | Different distribution channel. Post-validation. |

---

## Execution Order

```
WEEK 1  ┌─ Phase 0: Internal hardening (first-launch, dead code, privacy audit, offline test)
         └─ Phase 1: Local telemetry (event table, analytics command)

WEEK 2     Phase 2: Trusted tester program (5-10 devs, screen recordings, structured feedback)

WEEK 3  ┌─ Phase 3: Surface reduction (informed by tester data)
         └─ Decision gate: Do testers validate the core value proposition?
              YES → Continue to Phase 4
              NO  → Pause. Rethink positioning. Return to simulation.

WEEK 4     Phase 4: Conversion infrastructure (Stripe, checkout flow, pricing page)

WEEK 5     Phase 5: Launch preparation (landing page, README, demo video, pre-launch checklist)

WEEK 6     Phase 6: Controlled launch (r/selfhosted → r/rust → Show HN)
              ↓
         Decision gate: Launch metrics vs targets
              GREEN  → Continue growth. Optimize conversion. Build macOS.
              YELLOW → Adjust positioning/pricing. More tester cycles.
              RED    → Step back. Fundamental rethink.
```

---

## The Meta-Rule

Every future decision follows this loop:

```
SIMULATE: What are the probable outcomes? Model them with probabilities.
EXPLORE:  What data do we need to pick between outcomes? Gather it.
PLAN:     Given the data, what's the highest-EV path? Write it down.
IMPLEMENT: Build the minimum to execute the plan.
MEASURE:  Did reality match the simulation? Update the model.
REPEAT.
```

We never skip straight to IMPLEMENT. The cost of simulation is hours. The cost of building the wrong thing is weeks.

---

*This plan is a living document. Update it when simulations are invalidated by real data.*
