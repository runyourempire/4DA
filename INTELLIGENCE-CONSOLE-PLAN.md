# Intelligence Console — Implementation Plan

**Objective:** Transform NL Search from a search box into a compound intelligence system — the single most valuable feature in 4DA. Free users experience genuine intelligence. Pro users get depth that's worth paying for without being asked.

**Philosophy:** Show, don't lock. The free tier proves 4DA is intelligent. The Pro tier proves it's indispensable.

---

## Architecture Overview

```
                    ┌─────────────────────────────┐
                    │   Intelligence Console UI    │
                    │  ┌───────────────────────┐  │
                    │  │  Stack Health Bar      │  │ ← Layer 1 (ambient)
                    │  │  React ▲  Prisma ⚠     │  │
                    │  └───────────────────────┘  │
                    │  ┌───────────────────────┐  │
                    │  │  Search Input          │  │ ← Layer 2 (interactive)
                    │  │  [Ask anything...]     │  │
                    │  └───────────────────────┘  │
                    │  ┌───────────────────────┐  │
                    │  │  Synthesis Panel       │  │ ← Layer 3 (LLM, Pro)
                    │  │  "Based on 14 signals" │  │
                    │  └───────────────────────┘  │
                    │  ┌───────────────────────┐  │
                    │  │  Results + Ghost       │  │ ← Layer 2+5 (tiered)
                    │  │  3 free / all Pro      │  │
                    │  └───────────────────────┘  │
                    │  ┌───────────────────────┐  │
                    │  │  Watches (Pro)         │  │ ← Layer 6 (persistent)
                    │  └───────────────────────┘  │
                    └─────────────────────────────┘
                                  │
              ┌───────────────────┼───────────────────┐
              │                   │                   │
    ┌─────────▼──────┐  ┌────────▼────────┐  ┌──────▼──────────┐
    │ natural_lang   │  │ search_synth    │  │ stack_health    │
    │ _search.rs     │  │ esis.rs         │  │ .rs             │
    │ (overhauled)   │  │ (NEW)           │  │ (NEW)           │
    └────────────────┘  └─────────────────┘  └─────────────────┘
              │                   │                   │
    ┌─────────▼──────────────────▼───────────────────▼──────────┐
    │                    Existing Systems                        │
    │  ACE (detected_tech) | Decisions | Knowledge Gaps         │
    │  Signal Chains | Embeddings | LLM Client | Monitoring     │
    └───────────────────────────────────────────────────────────┘
```

---

## Phase 1: Stack Health Engine (Backend)

**New file:** `src-tauri/src/stack_health.rs` (~250 lines)
**Depends on:** ACE detected_tech, source_items, knowledge_decay

### 1.1 StackHealth struct and computation

```rust
pub struct StackHealth {
    pub technologies: Vec<TechHealthEntry>,
    pub stack_score: u32,              // 0-100 composite
    pub signals_this_week: u32,
    pub suggested_queries: Vec<String>,
    pub missed_signals: MissedIntelligence,
}

pub struct TechHealthEntry {
    pub name: String,
    pub category: String,
    pub status: String,       // "healthy", "attention", "stale", "critical"
    pub signal_count_7d: u32,
    pub days_since_engagement: u32,
    pub has_knowledge_gap: bool,
}

pub struct MissedIntelligence {
    pub total_count: u32,
    pub critical_count: u32,
    pub high_count: u32,
    pub example_titles: Vec<String>,  // Top 3 titles (free teaser)
}
```

**Algorithm:**
1. Query `detected_tech` for all technologies with confidence >= 0.5
2. For each tech, count `source_items` from last 7 days whose title/content mentions it
3. Cross-reference against `knowledge_decay::detect_knowledge_gaps()` for staleness
4. Status logic:
   - `"critical"` = has knowledge gap with severity Critical/High
   - `"attention"` = has knowledge gap with severity Medium, OR 0 signals in 14 days
   - `"stale"` = no engagement in 21+ days
   - `"healthy"` = signals flowing, no gaps
5. Stack score = weighted average of tech health (higher confidence techs weighted more)
6. Suggested queries = generated from techs with status != "healthy" (e.g. "security updates for Prisma")
7. Missed signals = items matching stack tech that user never engaged with (no ace_topic_affinity interaction)

### 1.2 Retrospective intelligence ("What You Missed")

In same module. Query:
```sql
SELECT s.title, s.source_type, s.created_at
FROM source_items s
WHERE s.created_at >= datetime('now', '-30 days')
AND (LOWER(s.title) LIKE '%{tech}%' OR LOWER(s.content) LIKE '%{tech}%')
AND s.id NOT IN (SELECT item_id FROM interaction_log)
ORDER BY s.created_at DESC
```

For each detected tech, find unengaged items. Categorize by priority based on challenge keywords (CVE, deprecated, breaking, vulnerability → critical/high).

### 1.3 Tauri commands

```rust
#[tauri::command]
pub async fn get_stack_health() -> Result<StackHealth, String>
// No Pro gate — free users see health scores (the hook)

#[tauri::command]
pub async fn get_missed_intelligence(days: Option<u32>) -> Result<MissedIntelligence, String>
// No Pro gate on counts. Full details (titles) Pro-gated.
```

### 1.4 Register in lib.rs

- Add `mod stack_health;`
- Add both commands to `invoke_handler`

**Verify:** `cargo check`

---

## Phase 2: NL Search Backend Overhaul

**File:** `src-tauri/src/natural_language_search.rs` (modify existing)
**Depends on:** Phase 1, ACE, decisions, knowledge_decay

### 2.1 Expand QueryResult with intelligence fields

Add to `QueryResult`:
```rust
pub struct QueryResult {
    // ... existing fields ...
    pub stack_context: Vec<StackContextEntry>,     // NEW
    pub related_decisions: Vec<RelatedDecision>,   // NEW (Pro only, empty for free)
    pub knowledge_gaps: Vec<QueryGap>,             // NEW (Pro only, empty for free)
    pub ghost_preview: Option<GhostPreview>,       // NEW (free only, None for Pro)
    pub is_pro: bool,                              // NEW
}

pub struct StackContextEntry {
    pub name: String,
    pub version: Option<String>,   // from tech_stack or detected_tech
    pub project_count: u32,        // how many projects use this
    pub relevant: bool,            // does this tech relate to the query?
}

pub struct RelatedDecision {
    pub id: i64,
    pub subject: String,
    pub decision: String,
    pub relation: String,  // "supports", "challenges", "related"
}

pub struct QueryGap {
    pub technology: String,
    pub days_stale: u32,
    pub severity: String,
}

pub struct GhostPreview {
    pub total_results: usize,
    pub hidden_results: usize,      // total - 3 shown
    pub decision_conflicts: usize,
    pub knowledge_gaps: usize,
    pub synthesis_available: bool,   // true if LLM is configured
}
```

### 2.2 Stack-aware query boosting

In `execute_text_search` and `merge_results`:
1. Before search, load user's detected technologies from ACE
2. If a result mentions a user's tech, boost relevance by 0.15
3. Build `stack_context` by matching query keywords against detected_tech names
4. Include version info from tech_stack table if available

### 2.3 Decision cross-referencing

After search results are collected:
1. Load active decisions via `decisions::list_decisions(&conn, None, None, 50)`
2. For each decision, check if `subject` or `context_tags` overlap with query keywords
3. Build `related_decisions` vec (Pro only — empty vec for free)

### 2.4 Knowledge gap detection for query

After search:
1. Load knowledge gaps via `knowledge_decay::detect_knowledge_gaps(&conn)`
2. Filter to gaps whose `dependency` matches any query keyword
3. Build `knowledge_gaps` vec (Pro only — empty vec for free)

### 2.5 Tiered response

Replace the hard Pro gate (`require_pro_feature`) with tiered logic:

```rust
let is_pro = crate::settings::is_pro();

// All users get: parsed query, intent, stack_context, execution_ms
// Free users: 3 results + ghost_preview
// Pro users: all results + decisions + gaps + no ghost_preview

if !is_pro {
    items.truncate(3);
    ghost_preview = Some(GhostPreview { ... });
    related_decisions = Vec::new();
    knowledge_gaps = Vec::new();
}
```

### 2.6 Update frontend interface

The frontend `QueryResult` TypeScript interface must match the new Rust struct. Update `NaturalLanguageSearch.tsx` interfaces.

**Verify:** `cargo check`

---

## Phase 3: LLM Synthesis Engine (Backend)

**New file:** `src-tauri/src/search_synthesis.rs` (~200 lines)
**Depends on:** Phase 2, LLM client

### 3.1 Synthesis function

```rust
pub async fn synthesize_search_results(
    query: &str,
    intent: &str,
    results: &[QueryResultItem],
    stack_context: &[StackContextEntry],
    decisions: &[RelatedDecision],
    app_handle: &tauri::AppHandle,
) -> Result<String, String>
```

**Logic:**
1. Check if LLM is configured (any provider). If not, return early with hint.
2. Build system prompt:
   ```
   You are the user's technical intelligence advisor. You know their stack:
   {stack_context formatted}. They have {N} active decisions. Synthesize
   the search results into a 2-3 sentence briefing. Be specific about
   versions, dates, and actionable items. Reference their stack directly.
   Do not hallucinate — only reference information from the provided results.
   ```
3. Build user message: query + top 5 result titles/previews as context
4. Call `LLMClient::new(provider).complete(system, messages).await`
5. Emit progressive events via `app_handle.emit("search-synthesis-chunk", chunk)`
6. Return full synthesis text

### 3.2 Streaming via Tauri events

Since `LLMClient` doesn't support streaming yet, use a simpler approach:
1. Emit `search-synthesis-start` when synthesis begins
2. Call LLM (non-streaming, ~2-3 seconds)
3. Emit `search-synthesis-complete` with the full synthesis text

Frontend listens for these events and shows a typing animation during wait.

### 3.3 Tauri command (separate from main search)

```rust
#[tauri::command]
pub async fn synthesize_search(
    app: tauri::AppHandle,
    query_text: String,
) -> Result<String, String>
```

This is a SEPARATE command from `natural_language_query`. The frontend:
1. Calls `natural_language_query` → gets instant results (200ms)
2. If Pro + LLM configured, calls `synthesize_search` → gets synthesis (2-3s)

This keeps the main search fast and adds synthesis as progressive enhancement.

Pro-gated: `require_pro_feature("synthesize_search")?;`

### 3.4 Register in lib.rs

- Add `mod search_synthesis;`
- Add `search_synthesis::synthesize_search` to `invoke_handler`

**Verify:** `cargo check`

---

## Phase 4: Standing Queries (Backend)

**New file:** `src-tauri/src/standing_queries.rs` (~300 lines)
**Depends on:** Phase 2, monitoring cycle, DB migration

### 4.1 DB table

Add to ACE migration (or db migration):
```sql
CREATE TABLE IF NOT EXISTS standing_queries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    query_text TEXT NOT NULL,
    keywords TEXT NOT NULL,           -- JSON array of extracted keywords
    created_at TEXT DEFAULT (datetime('now')),
    last_run TEXT,
    total_matches INTEGER DEFAULT 0,
    new_matches INTEGER DEFAULT 0,    -- since last_run
    active INTEGER DEFAULT 1
);
```

### 4.2 CRUD commands

```rust
#[tauri::command]
pub async fn create_standing_query(query_text: String) -> Result<i64, String>
// Pro-gated. Extracts keywords, inserts row. Max 10 active queries.

#[tauri::command]
pub async fn list_standing_queries() -> Result<Vec<StandingQuery>, String>
// Pro-gated.

#[tauri::command]
pub async fn delete_standing_query(id: i64) -> Result<(), String>
// Pro-gated.

#[tauri::command]
pub async fn get_standing_query_matches(id: i64) -> Result<Vec<QueryResultItem>, String>
// Pro-gated. Returns recent items matching this query's keywords.
```

### 4.3 Monitoring integration

Add function `pub fn evaluate_standing_queries(conn: &Connection) -> Vec<StandingQueryAlert>`:
1. Load active standing queries
2. For each, find source_items created since `last_run` matching keywords
3. Update `last_run` and `new_matches` count
4. Return alerts for queries with new matches

**Hook into monitoring cycle:** In `lib.rs` scheduled-analysis listener, after analysis completes:
```rust
// After analysis results emitted
if crate::settings::is_pro() {
    if let Ok(conn) = crate::open_db_connection() {
        let alerts = standing_queries::evaluate_standing_queries(&conn);
        if !alerts.is_empty() {
            let _ = handle.emit("standing-query-matches", &alerts);
        }
    }
}
```

### 4.4 Register in lib.rs

- Add `mod standing_queries;`
- Add all 4 commands to `invoke_handler`
- Add evaluation hook in monitoring cycle

**Verify:** `cargo check`

---

## Phase 5: Frontend — Intelligence Console UI

**Major overhaul:** `src/components/NaturalLanguageSearch.tsx`
**New files:** `src/components/search/StackHealthBar.tsx`, `src/components/search/GhostPreview.tsx`, `src/components/search/SynthesisPanel.tsx`, `src/components/search/StandingQueries.tsx`
**Depends on:** Phases 1-4

### 5.1 Remove ProGate wrapper from App.tsx

In `src/App.tsx`, replace:
```tsx
<ProGate feature="Natural Language Search">
  <NaturalLanguageSearch defaultExpanded={true} />
</ProGate>
```
With:
```tsx
<NaturalLanguageSearch defaultExpanded={true} />
```

The tiered experience is now handled INSIDE the component via the backend response.

### 5.2 StackHealthBar component (~120 lines)

**File:** `src/components/search/StackHealthBar.tsx`

```tsx
interface StackHealthBarProps {
  health: StackHealth | null;
  onSuggestedQuery: (query: string) => void;
}
```

Renders:
- Horizontal bar of tech pills: `React ▲` `Prisma ⚠` `TypeScript ●` `Node ▼`
- Color coding: healthy=green, attention=amber, stale=gray, critical=red
- Stack score badge: `74/100`
- "What you missed" banner (if missed_signals.total_count > 0): `"23 signals matched your stack last month. 3 critical." [See details →]`
- Suggested queries as clickable chips below

On mount, calls `invoke('get_stack_health')`.

### 5.3 GhostPreview component (~80 lines)

**File:** `src/components/search/GhostPreview.tsx`

```tsx
interface GhostPreviewProps {
  preview: GhostPreview;
  isPro: boolean;
}
```

Renders (only for free users, after results):
```
── Pro Intelligence ──────────────────────
  📊 LLM Synthesis available
  🔍 {hidden_results} more results
  ⚖️ {decision_conflicts} decisions affected
  🧠 {knowledge_gaps} knowledge gaps

  [Upgrade to Pro →]
```

Not blurred. Not locked. Just showing the SHAPE of what Pro provides. Clean, informative, respectful.

### 5.4 SynthesisPanel component (~100 lines)

**File:** `src/components/search/SynthesisPanel.tsx`

```tsx
interface SynthesisPanelProps {
  query: string;
  isPro: boolean;
  hasLLM: boolean;
  results: QueryResultItem[];
}
```

For Pro users with LLM configured:
1. After main results load, automatically calls `invoke('synthesize_search', { queryText })`
2. Shows loading state: `"Analyzing 14 signals..."` with subtle animation
3. Synthesis text appears in a highlighted panel above results
4. Grounded — references specific results by name

For Pro users without LLM: `"Configure Ollama for AI synthesis"` hint link.
For free users: Hidden (ghost preview covers this).

### 5.5 StandingQueries panel (~100 lines)

**File:** `src/components/search/StandingQueries.tsx`

```tsx
interface StandingQueriesProps {
  isPro: boolean;
}
```

For Pro users:
- "My Watches" section below search results
- List of active standing queries with match counts
- "Watch this" button appears on each search result
- Delete button per watch
- Badge showing new matches since last check

Listens for `standing-query-matches` Tauri event to show notifications.

### 5.6 Overhaul NaturalLanguageSearch.tsx

The parent component orchestrates all sub-components:

```tsx
export function NaturalLanguageSearch({ defaultExpanded }: Props) {
  // State
  const [health, setHealth] = useState<StackHealth | null>(null);
  const [result, setResult] = useState<QueryResult | null>(null);
  const [synthesis, setSynthesis] = useState<string | null>(null);
  const { isPro } = useLicense();

  // Load stack health on mount
  useEffect(() => {
    invoke('get_stack_health').then(setHealth).catch(() => {});
  }, []);

  // Search handler (no Pro gate — tiered response)
  const handleSearch = async () => {
    const searchResult = await invoke('natural_language_query', { queryText: query });
    setResult(searchResult);

    // Pro: auto-trigger synthesis if LLM configured
    if (searchResult.is_pro && hasLLM) {
      invoke('synthesize_search', { app, queryText: query })
        .then(setSynthesis)
        .catch(() => {});
    }
  };

  return (
    <div>
      <StackHealthBar health={health} onSuggestedQuery={setQuery} />
      <SearchInput ... />
      {result && <>
        <StackContext entries={result.stack_context} />
        <SynthesisPanel ... />
        <ResultsList items={result.items} />
        {result.ghost_preview && <GhostPreview preview={result.ghost_preview} />}
        {result.related_decisions.length > 0 && <DecisionConflicts ... />}
        {result.knowledge_gaps.length > 0 && <GapAlerts ... />}
      </>}
      {isPro && <StandingQueries />}
    </div>
  );
}
```

### 5.7 File size management

The parent `NaturalLanguageSearch.tsx` must stay under 350 lines. Sub-components in `src/components/search/` directory keep each file focused.

Expected sizes:
- `NaturalLanguageSearch.tsx`: ~200 lines (orchestrator)
- `StackHealthBar.tsx`: ~120 lines
- `GhostPreview.tsx`: ~80 lines
- `SynthesisPanel.tsx`: ~100 lines
- `StandingQueries.tsx`: ~100 lines

### 5.8 i18n keys

Add to `src/locales/en/ui.json`:
```json
{
  "search.stackContext": "Your stack",
  "search.stackScore": "Stack Score",
  "search.missedSignals": "{{count}} signals matched your stack. {{critical}} critical.",
  "search.missedAction": "See details",
  "search.synthesizing": "Analyzing {{count}} signals...",
  "search.synthesisHint": "Configure Ollama for AI synthesis",
  "search.ghostTitle": "Pro Intelligence",
  "search.ghostSynthesis": "LLM Synthesis available",
  "search.ghostResults": "{{count}} more results",
  "search.ghostDecisions": "{{count}} decisions affected",
  "search.ghostGaps": "{{count}} knowledge gaps",
  "search.watchThis": "Watch this",
  "search.myWatches": "My Watches",
  "search.newMatches": "{{count}} new",
  "search.noWatches": "Watch a search to get notified of new matches",
  "search.suggestedQueries": "Suggested for your stack",
  "search.healthy": "healthy",
  "search.attention": "needs attention",
  "search.stale": "stale",
  "search.critical": "critical"
}
```

**Verify:** `npx tsc --noEmit`, `pnpm run validate:sizes`

---

## Phase 6: Integration & Polish

### 6.1 Monitoring cycle hook for standing queries

In `src-tauri/src/lib.rs`, inside the `scheduled-analysis` listener, after `analysis-complete` emission:

```rust
// Evaluate standing queries for Pro users
if crate::settings::is_pro() {
    let standing_handle = handle.clone();
    tauri::async_runtime::spawn(async move {
        if let Ok(conn) = crate::open_db_connection() {
            let alerts = standing_queries::evaluate_standing_queries(&conn);
            if !alerts.is_empty() {
                let _ = standing_handle.emit("standing-query-matches", &alerts);
            }
        }
    });
}
```

### 6.2 Void Engine integration

When standing queries match, pulse the void engine:
```rust
if alerts.iter().any(|a| a.new_matches > 0) {
    crate::events::void_signal_notification(&standing_handle, false, total_new);
}
```

### 6.3 Update Pro features list

In `src-tauri/src/settings/license.rs`, update `PRO_FEATURES`:
- Keep `natural_language_query` but clarify it's now "full results" (free gets 3)
- Add `synthesize_search`
- Add `create_standing_query`, `list_standing_queries`, `delete_standing_query`, `get_standing_query_matches`
- Keep `generate_weekly_digest`, `get_decision_signals`

Note: `get_stack_health` and `get_missed_intelligence` are NOT Pro-gated — they're the hook.

### 6.4 Full validation

```bash
cd src-tauri && cargo check          # Rust compilation
cd src-tauri && cargo test           # Rust tests
npx tsc --noEmit                     # TypeScript types
pnpm run test                        # Frontend tests
pnpm run validate:sizes              # File sizes
pnpm run validate:all                # Full suite
```

### 6.5 Commit strategy

One commit per phase:
1. `Add stack health engine — ambient intelligence for detected tech stack`
2. `Overhaul NL search — tiered response, stack boosting, decision/gap cross-referencing`
3. `Add LLM synthesis engine — grounded intelligence briefing from search results`
4. `Add standing queries — persistent intelligence monitoring with watch system`
5. `Build Intelligence Console UI — stack health, ghost preview, synthesis, watches`
6. `Wire standing queries into monitoring cycle, update Pro features list`

---

## Execution Order & Dependencies

```
Phase 1 (stack_health.rs)          ← independent, start here
    │
Phase 2 (NL search overhaul)      ← uses Phase 1 types
    │
Phase 3 (search_synthesis.rs)     ← uses Phase 2 results
    │
Phase 4 (standing_queries.rs)     ← independent of 2/3, can parallel
    │
Phase 5 (Frontend)                ← needs all backend phases
    │
Phase 6 (Integration)             ← wiring + validation
```

**Parallelizable:** Phase 1 + Phase 4 can run simultaneously.
**Critical path:** Phase 2 → Phase 3 → Phase 5.

---

## Risk Assessment

| Risk | Mitigation |
|------|------------|
| LLM synthesis is slow (>5s) | Separate command, progressive UI, timeout at 15s |
| Stack health has no data (new user) | Graceful empty state: "Run analysis to discover your stack" |
| Ghost preview not persuasive enough | Show specific numbers, not vague promises |
| Standing queries overwhelm monitoring | Max 10 queries, evaluate only after analysis completes |
| File size limits exceeded | Sub-component extraction pattern, 5 files instead of 1 |
| No LLM configured | Everything works without LLM, synthesis is progressive enhancement |
| Free search returns empty | Stack health + suggested queries guide users to productive searches |

---

## Success Criteria

- [ ] Free user searches and gets 3 results + ghost preview in <300ms
- [ ] Pro user searches and gets all results + decisions + gaps in <500ms
- [ ] LLM synthesis appears within 5 seconds when configured
- [ ] Stack health loads on mount showing real detected technologies
- [ ] Ghost preview shows specific numbers (not blur/lock)
- [ ] Standing queries evaluate on each monitoring cycle
- [ ] "What you missed" shows real unengaged signals
- [ ] All files under size limits (TSX <350 warn, <500 error)
- [ ] `pnpm run validate:all` passes clean
- [ ] `cargo check` with zero warnings
