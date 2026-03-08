# 4DA Optimal Trajectory Plan

> Created: 8 March 2026
> Authors: Claude + Antony
> Status: READY TO EXECUTE
> Prerequisite: All fortification phases complete, codebase green

---

## Philosophy

Three phases. Each builds on the last. Phase 1 makes users feel something
immediately. Phase 2 makes the product get smarter the longer they use it,
and shows them it's happening. Phase 3 creates network effects without
compromising privacy.

---

## PHASE 1: First-Contact Engineering

> Goal: A developer downloads 4DA and feels "this understands me" within 60 seconds.

### 1.1 Live Analysis Narration

**Problem:** After onboarding, FirstRunTransition shows source-by-source fetching
but the narration is minimal. The gap between "fetching" and "here are your results"
feels like waiting, not discovery.

**Solution:** Real-time narration stream during first analysis.

**Files:**
- `src/components/FirstRunTransition.tsx` (fetching + analyzing phases)
- `src-tauri/src/sources/` (emit richer events during fetch)
- `src/hooks/use-analysis.ts` (new event listener)

**Implementation:**

```
// New Tauri event: analysis-narration
{
  type: "discovery" | "match" | "insight",
  message: "Found 3 articles about Tauri 2.0 security...",
  source: "hackernews",
  relevance: 0.87
}
```

During first analysis, the backend emits narration events as it scores:
- "Scanning Hacker News... 47 items found"
- "3 match your Rust + Tauri stack directly"
- "Found a security advisory for one of your dependencies"
- "Reddit r/rust has 12 items above your relevance threshold"
- "Highest signal: 'Tauri 2.0 Plugin Security Model' (0.92)"

Frontend renders these as a live feed with subtle animations:
- Each narration line fades in, stays 3 seconds, then slides up
- Matched items pulse briefly in accent color
- Source icons appear as each source is scanned
- Progress ring fills around a central "signal count" number

**Acceptance:** User sees at least 5 narration lines during first analysis.
The screen never feels static or empty.

### 1.2 Guaranteed First Impression

**Problem:** If a user's stack is unusual or sources are slow, the first
results might be mediocre. First impressions are permanent.

**Solution:** Curated "seed" content per detected stack that's guaranteed
high quality.

**Files:**
- `src-tauri/src/sources/` (new `seed.rs` module)
- `src-tauri/src/stacks/profiles.rs` (add `seed_urls` to StackProfile)

**Implementation:**

Each StackProfile gets a `seed_content` field: 3-5 hand-curated URLs that
are universally relevant to that stack. These are fetched and scored alongside
regular sources during first analysis only.

```rust
pub struct StackProfile {
    // ... existing fields
    pub seed_content: &'static [SeedItem],
}

pub struct SeedItem {
    pub title: &'static str,
    pub url: &'static str,
    pub source_type: &'static str,
}
```

Examples for Rust profile:
- "This Month in Rust" (latest)
- Rust security advisories RSS
- Trending Rust repos on GitHub (always fresh)

**Constraint:** Seed content is only injected on first analysis. After that,
organic sources take over. Seeds are clearly marked in the UI with a
"Curated for your stack" badge so users understand why they're there.

**Acceptance:** First analysis always returns >= 5 relevant items regardless
of timing or network conditions.

### 1.3 Briefing Cold State Upgrade

**Problem:** Before first analysis, BriefingView shows "Run an analysis to
see intelligence" with a button. This is dead space.

**Solution:** Transform the cold state into an anticipation builder.

**Files:**
- `src/components/BriefingEmptyStates.tsx`
- `src/components/BriefingView.tsx`

**Implementation:**

Replace `BriefingNoDataState` with `BriefingWarmupState`:

```
+---------------------------------------------------+
|  [GAME: briefing-atmosphere shader, subtle]       |
|                                                    |
|  Your Intelligence System                          |
|                                                    |
|  Stack detected: Rust, Tauri, TypeScript           |
|  Sources ready: HN, Reddit, GitHub, arXiv          |
|  Learning: Ready to calibrate to your preferences  |
|                                                    |
|  [  Activate Intelligence  ]                       |
|                                                    |
|  "4DA will scan 5 sources, score every item        |
|   against your profile, and surface what matters." |
+---------------------------------------------------+
```

Show what the system KNOWS about the user (from ACE + onboarding) before
they've run analysis. This proves the system is already contextual.

**Acceptance:** Cold briefing state shows detected stack + enabled sources.
User understands what will happen before clicking Analyze.

### 1.4 Post-Analysis Celebration Upgrade

**Problem:** CelebrationState in FirstRunTransition shows count + top signal
but doesn't create an emotional peak.

**Solution:** Make the celebration feel like a personal intelligence report.

**Files:**
- `src/components/FirstRunTransition.tsx` (CelebrationState)

**Implementation:**

After analysis completes, show:

```
+---------------------------------------------------+
|  Intelligence Activated                            |
|                                                    |
|     [87]  items analyzed                           |
|     [23]  relevant to you                          |
|     [ 3]  match your active dependencies           |
|                                                    |
|  Your #1 Signal:                                   |
|  "Tauri 2.0 Plugin Security Model"                 |
|  Score: 0.92 | Source: Hacker News                  |
|  Matched: tauri, security, plugin-api              |
|                                                    |
|  Stack Insight:                                    |
|  "Your Rust + Tauri combination has 3 active       |
|   ecosystem shifts tracked by 4DA"                 |
|                                                    |
|  [See Your Briefing]  [Browse All Results]         |
+---------------------------------------------------+
```

The key: show WHY the #1 signal matched. Show the matched keywords/deps.
This proves the scoring is personal, not generic.

**Acceptance:** User sees their top signal with match explanation.

---

## PHASE 2: Compound Advantage Loops

> Goal: The longer you use 4DA, the smarter it gets. And you can SEE it getting smarter.

### 2.1 Intelligence Improvement Metric

**Problem:** IntelligenceProfileCard shows accuracy % but not improvement
over time. Users can't feel the compounding.

**Solution:** Track and display a "learning curve" — how much better scoring
has gotten since day 1.

**Files:**
- `src-tauri/src/autophagy.rs` (store historical accuracy)
- `src-tauri/src/db.rs` (new table: `intelligence_history`)
- `src/components/IntelligenceProfileCard.tsx`

**Implementation:**

New table:
```sql
CREATE TABLE IF NOT EXISTS intelligence_history (
  id INTEGER PRIMARY KEY,
  recorded_at TEXT NOT NULL DEFAULT (datetime('now')),
  accuracy REAL NOT NULL,
  topics_learned INTEGER NOT NULL,
  items_analyzed INTEGER NOT NULL,
  relevant_found INTEGER NOT NULL
);
```

After each analysis cycle, record a snapshot. On the profile card, show:

```
Intelligence Growth
Day 1: 34% accuracy  →  Today: 76% accuracy
[============================>          ] +124% improvement

12 topics learned  |  3 anti-topics identified
```

The improvement percentage is `((current - initial) / initial) * 100`.
This is the single most compelling metric: your tool is getting smarter.

**Acceptance:** After 3+ analysis cycles, users see a growth percentage.

### 2.2 Decision-Aware Scoring Feedback

**Problem:** Decision windows already boost scoring, but users don't know
their decisions are affecting results.

**Solution:** Show decision influence in result items.

**Files:**
- `src/components/result-item/BadgeRow.tsx`
- `src-tauri/src/decision_advantage/scoring_boost.rs` (return match info)
- `src/types/index.ts` (extend SourceRelevance)

**Implementation:**

When a decision window boosts an item, the result card shows a badge:

```
[Decision Match: "Evaluating Tauri → Leptos migration"]
```

This creates a visible feedback loop: record a decision → see it influence
your results → trust the system more → record more decisions.

The boost function already returns `matched_window_id`. Extend the pipeline
to include this in the SourceRelevance struct so the frontend can display it.

```rust
// In SourceRelevance (returned to frontend)
pub decision_window_match: Option<String>, // subject of matched window
pub decision_boost_applied: f32,           // how much boost was added
```

**Acceptance:** Items boosted by decision windows show which decision
matched them.

### 2.3 Weekly Digest as Re-engagement Hook

**Problem:** Weekly digest exists but has no delivery mechanism.

**Solution:** System tray notification + digest view.

**Files:**
- `src-tauri/src/weekly_digest.rs` (schedule generation)
- `src-tauri/src/monitoring_jobs.rs` (add digest job)
- `src/components/DigestView.tsx` (new component)
- `src/components/BriefingView.tsx` (embed digest section)

**Implementation:**

Every Monday at the user's configured time (default 9am local), generate
the digest and:
1. Show a system tray notification: "Your weekly intelligence digest is ready"
2. Store the digest in SQLite for retrieval
3. Show a "Weekly Digest" section at the top of BriefingView

Digest content (already structured in `weekly_digest.rs`):
- Top 10 items from the week
- Trending topics in your stack
- Active signal chains
- Knowledge gaps (stale dependencies)
- Stats: items analyzed, relevance accuracy, topics learned

**Acceptance:** Users receive a weekly notification. Clicking it opens
the digest in the briefing view.

### 2.4 Scoring Delta Visualization

**Problem:** Users can't see how their scores are shifting over time.

**Solution:** Add a sparkline to the Intelligence Metrics showing
score distribution shifts.

**Files:**
- `src/components/IntelligenceProfileCard.tsx`
- `src-tauri/src/autophagy.rs` (expose score distribution history)

**Implementation:**

Track the median and 90th percentile scores per analysis cycle.
Show a 30-day sparkline:

```
Score Distribution (30 days)
Median:  0.23 → 0.41  ↑ +78%
Top 10%: 0.67 → 0.82  ↑ +22%
[sparkline visualization]
```

Rising medians mean the system is getting better at finding relevant
content. Rising 90th percentile means the best matches are getting stronger.

**Acceptance:** After 7+ days, users see score trend sparklines.

---

## PHASE 3: Community Intelligence Network

> Goal: Network effects without compromising privacy. Anonymous pattern
> sharing that makes every user's 4DA smarter.

### 3.1 Architecture: Privacy-Preserving Pattern Sharing

**Core principle:** Share PATTERNS, never DATA.

What gets shared (opt-in only):
- Anonymized scoring model weights (which signal types matter most)
- Stack profile effectiveness metrics (accuracy per profile)
- Aggregated topic trend signals (not individual items or URLs)
- Pain point validation (which pain points are actually painful)

What NEVER gets shared:
- Raw content, URLs, or article titles
- User identity, API keys, or profile data
- Individual scoring decisions or preferences
- Tech stack composition or interests
- Any data that could identify a user

**Protocol:**
```
User's 4DA  →  anonymized weights  →  4DA Community API
                                           ↓
                aggregate model    ←  improved weights
```

### 3.2 Settings Toggle: Community Intelligence

**Files:**
- `src/components/settings/CommunityIntelligenceSection.tsx` (new)
- `src/components/SettingsModal.tsx` (add tab/section)
- `src/store/settings-slice.ts` (add setting)
- `src-tauri/src/settings/mod.rs` (add field to Settings struct)
- `src/locales/en/ui.json` (i18n keys)

**Implementation:**

New settings section following MonitoringSection pattern:

```
+---------------------------------------------------+
|  [Shield Icon]                                     |
|  Community Intelligence                            |
|  Help improve scoring for all 4DA users            |
|                                                    |
|  [Toggle: OFF by default]                          |
|                                                    |
|  When enabled, 4DA shares anonymized scoring       |
|  patterns with the 4DA community. This helps       |
|  improve scoring accuracy for developers with      |
|  similar stacks.                                   |
|                                                    |
|  What's shared:                                    |
|  - Anonymized scoring weights                      |
|  - Stack profile accuracy metrics                  |
|  - Aggregated topic trend signals                  |
|                                                    |
|  What's NEVER shared:                              |
|  - Your content, URLs, or bookmarks                |
|  - Your identity or API keys                       |
|  - Your tech stack or interests                    |
|  - Any personally identifiable information         |
|                                                    |
|  [Learn more about our privacy approach]           |
|                                                    |
|  Contribution frequency:                           |
|  [Weekly ▾]                                        |
|                                                    |
|  Last contribution: Never                          |
|  Community size: --                                |
+---------------------------------------------------+
```

**Rust Settings struct:**
```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommunityIntelligenceConfig {
    pub enabled: bool,           // default: false
    pub frequency: String,       // "weekly" | "monthly"
    pub last_contributed: Option<String>,
    pub anonymous_id: Option<String>, // random UUID, no user link
}
```

**Store slice:**
```typescript
// In SettingsForm or dedicated community-intelligence-slice.ts
communityIntelligence: {
  enabled: boolean;
  frequency: 'weekly' | 'monthly';
  lastContributed: string | null;
};
setCommunityIntelligenceEnabled: (enabled: boolean) => void;
setCommunityIntelligenceFrequency: (freq: 'weekly' | 'monthly') => void;
```

**Toggle behavior:**
- OFF by default (privacy-first principle)
- First toggle ON shows a confirmation dialog explaining exactly what's shared
- Toggle saves to settings.json immediately
- When enabled, a small "Community" badge appears in the briefing header

### 3.3 Anonymous Contribution Pipeline

**Files:**
- `src-tauri/src/community_intelligence.rs` (new module)
- `src-tauri/src/lib.rs` (register module + commands)

**Implementation:**

```rust
pub struct CommunityContribution {
    pub anonymous_id: String,        // random UUID, regenerated monthly
    pub app_version: String,
    pub contribution_type: String,   // "scoring_weights" | "profile_accuracy"
    pub payload: serde_json::Value,  // anonymized data
}
```

**Scoring weight contribution:**
```rust
pub struct ScoringWeightContribution {
    // No user identity
    // No content or URLs
    // Only aggregate statistics
    pub profile_ids: Vec<String>,          // e.g. ["rust_systems", "nextjs_fullstack"]
    pub accuracy_by_profile: Vec<f32>,     // e.g. [0.78, 0.65]
    pub signal_effectiveness: HashMap<String, f32>, // e.g. {"dep_match": 0.9, "keyword": 0.4}
    pub pain_point_hit_rates: Vec<(String, f32)>,   // e.g. [("app_router_migration", 0.7)]
    pub total_items_scored: u64,           // aggregate only
    pub avg_relevant_ratio: f32,          // e.g. 0.23
}
```

**API endpoint:** `https://community.4da.ai/api/v1/contribute`
- POST only, no authentication required
- Rate limited per anonymous_id
- Returns: aggregate improvement metrics

### 3.4 Community-Enhanced Scoring (Receive Side)

**Files:**
- `src-tauri/src/community_intelligence.rs`
- `src-tauri/src/scoring/pipeline.rs` (integrate community weights)

**Implementation:**

When community intelligence is enabled, 4DA periodically fetches
aggregate community weights:

```rust
pub struct CommunityWeights {
    pub version: u32,
    pub generated_at: String,
    pub contributors: u64,           // how many users contributed
    pub profile_adjustments: HashMap<String, ProfileAdjustment>,
}

pub struct ProfileAdjustment {
    pub keyword_boost_multipliers: HashMap<String, f32>,
    pub pain_point_severity_adjustments: HashMap<String, f32>,
    pub signal_type_weights: HashMap<String, f32>,
}
```

These weights are applied as a SECONDARY signal (multiplied at 0.1x weight)
so community intelligence nudges scores but never overrides personal preferences.

**Safeguards:**
- Community weights cached locally (24h TTL)
- If API is unreachable, local-only scoring continues
- User can inspect community weight influence in Insights tab
- Kill switch: toggle OFF immediately stops all community features

### 3.5 Community Insights Display

**Files:**
- `src/components/CommunityInsights.tsx` (new)
- `src/components/BriefingView.tsx` (add section)

**Implementation:**

When enabled, show in BriefingView:

```
+---------------------------------------------------+
|  Community Intelligence                            |
|  247 developers contributing anonymously            |
|                                                    |
|  Trending in your stack:                           |
|  - Rust developers seeing +15% more security       |
|    content this week                                |
|  - Tauri 2.0 pain points validated by 34 devs      |
|                                                    |
|  Your contribution: scoring accuracy improved       |
|  by an estimated 8% from community patterns         |
+---------------------------------------------------+
```

---

## Execution Order

### Sprint 1: Phase 1 (First-Contact)
1. 1.3 Briefing cold state upgrade (simplest, immediate visual impact)
2. 1.1 Live analysis narration (backend events + frontend feed)
3. 1.4 Post-analysis celebration upgrade (extend existing component)
4. 1.2 Guaranteed first impression (seed content per profile)

### Sprint 2: Phase 2 (Compound Loops)
5. 2.1 Intelligence improvement metric (new table + card update)
6. 2.2 Decision-aware scoring feedback (extend pipeline + badges)
7. 2.4 Scoring delta visualization (sparklines)
8. 2.3 Weekly digest delivery (scheduling + notification)

### Sprint 3: Phase 3 (Community Intelligence)
9.  3.2 Settings toggle (frontend only, no backend needed yet)
10. 3.1 Architecture + anonymous ID generation
11. 3.3 Contribution pipeline (Rust module)
12. 3.4 Community-enhanced scoring (receive + apply)
13. 3.5 Community insights display

### Validation Gate (after each sprint)
- [ ] All 753+ frontend tests pass
- [ ] All 1,620+ Rust tests pass
- [ ] Zero ESLint warnings
- [ ] TypeScript clean
- [ ] Production build clean
- [ ] No privacy regression (no user data in community payloads)

---

## Success Metrics

| Metric | Phase 1 Target | Phase 2 Target | Phase 3 Target |
|--------|---------------|---------------|---------------|
| Time to first relevant result | < 90 seconds | -- | -- |
| Narration events during first analysis | >= 5 | -- | -- |
| Users who see accuracy improvement | -- | > 80% after 7 days | -- |
| Decision badges shown per session | -- | >= 1 (if decisions exist) | -- |
| Community opt-in rate | -- | -- | > 30% of Pro users |
| Scoring improvement from community | -- | -- | +5-15% accuracy |

---

## Non-Goals

- No user accounts or authentication for community features
- No social features, profiles, or identifiable user data
- No mandatory community participation
- No degradation of offline/local-only functionality
- No changes to the privacy architecture
- No telemetry, analytics, or tracking of any kind
