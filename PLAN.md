# 4DA Hardening Plan

**Objective:** Transform every tab, component, and backend module into production-grade quality — zero hardcoded strings, full accessibility, optimal performance, comprehensive test coverage, and dead code elimination.

**Scope:** 7 phases covering frontend, backend, and infrastructure. Each phase is atomic and independently committable.

---

## Phase 1: File Size & Component Extraction

**Problem:** BriefingView.tsx is 537 lines (exceeds 500-line error threshold). GAME component `containerRef` patterns produce 5 ESLint warnings.

### 1.1 Extract BriefingAtmosphere from BriefingView

**Source:** `src/components/BriefingView.tsx` (537 lines)
**Target:** `src/components/briefing/BriefingAtmosphere.tsx` (~80 lines)

Extract the GAME component lifecycle block (containerRef, useEffect with `registerGameComponent`, cleanup):
- Move the `<div ref={containerRef}>` + useEffect + cleanup into `<BriefingAtmosphere />`
- Props: `{ visible: boolean }` — controls mount/unmount
- Import and render `<BriefingAtmosphere />` in BriefingView where the old block was
- BriefingView drops to ~460 lines (safely under 500)

### 1.2 Extract shared useGameComponent hook

**Target:** `src/hooks/useGameComponent.ts` (~40 lines)

All 6 GAME component mount points use the same pattern:
```tsx
const containerRef = useRef<HTMLDivElement>(null);
useEffect(() => {
  const el = containerRef.current;
  if (!el) return;
  registerGameComponent('component-name', el, { ...params });
  return () => { el.innerHTML = ''; };
}, [deps]);
```

Create a reusable hook:
```tsx
export function useGameComponent(
  name: string,
  params: Record<string, unknown>,
  deps: unknown[] = []
): React.RefObject<HTMLDivElement>
```

**Files to refactor (replace inline pattern with hook):**
- `src/components/BriefingView.tsx` (or new BriefingAtmosphere)
- `src/components/PlaybookView.tsx`
- `src/components/TechRadar.tsx`
- `src/components/IntelligenceProfileCard.tsx`
- `src/components/LearningIndicator.tsx`
- `src/components/channels/ChannelsView.tsx`

This eliminates all 5 ESLint `containerRef` warnings.

### 1.3 Validate file sizes

```bash
pnpm run validate:sizes
```

All files must pass. No new exceptions added to check-file-sizes.cjs.

---

## Phase 2: Accessibility Hardening

**Problem:** 12+ components missing ARIA attributes, keyboard navigation gaps, screen reader blind spots.

### 2.1 DecisionMemory form accessibility

**File:** `src/components/DecisionMemory.tsx`

- Add `role="form"` and `aria-label="Record decision"` to decision input form
- Add `aria-required="true"` to decision text input
- Add `aria-describedby` linking input to error message (when validation fails)
- Add loading state: `aria-busy="true"` on form during submission
- Add `aria-live="polite"` to the decisions list for dynamic updates
- Add visual loading spinner during `handleSubmit` / `handleSupersede` / `handleReconsider`

### 2.2 IntelligencePulse toggle

**File:** `src/components/IntelligenceProfileCard.tsx`

- Add `role="switch"` and `aria-checked` to the Intelligence Pulse visibility toggle
- Add `aria-label="Toggle intelligence pulse display"`

### 2.3 ChannelCard keyboard support

**File:** `src/components/channels/ChannelCard.tsx`

- Add `role="article"` to channel card container
- Ensure channel card actions (edit, delete, toggle) are keyboard-focusable
- Add `aria-label` to icon-only action buttons

### 2.4 ToolkitView navigation

**File:** `src/components/toolkit/ToolkitView.tsx`

- Add `role="tablist"` to tool selector sidebar
- Add `role="tab"` + `aria-selected` to each tool button
- Add `role="tabpanel"` to the active tool content area
- Add `aria-labelledby` linking tabpanel to its tab

### 2.5 ContextPanel

**File:** `src/components/ContextPanel.tsx`

- Add `role="complementary"` and `aria-label="Context panel"` to the aside
- Add `aria-expanded` to collapsible sections

### 2.6 EngagementPulse heatmap

**File:** `src/components/EngagementPulse.tsx` (if exists)

- Add `role="img"` and `aria-label` describing the heatmap data
- Add screen reader text summarizing engagement trend

---

## Phase 3: Remaining i18n

**Problem:** 3-5 hardcoded strings survived the audit sweep.

### 3.1 BriefingView hardcoded strings

**File:** `src/components/BriefingView.tsx`

Find and replace:
- `"System learned"` → `t('briefing.systemLearned')`
- `"Intelligence Metrics"` → `t('briefing.intelligenceMetrics')`
- Any remaining hardcoded tooltip text or labels

### 3.2 ToolkitView title

**File:** `src/components/toolkit/ToolkitView.tsx`

- Replace hardcoded `"Toolkit"` header → `t('toolkit.title')`

### 3.3 use-analysis.ts toast

**File:** `src/hooks/use-analysis.ts`

- Replace hardcoded toast message strings → `t()` calls
- Import `useTranslation` or use i18n.t() directly

### 3.4 Add keys to locale file

**File:** `src/locales/en/ui.json`

Add all new keys from 3.1-3.3. Verify no orphaned keys remain.

### 3.5 Full i18n sweep

Run grep for remaining hardcoded user-facing strings:
```bash
grep -rn ">[A-Z][a-z]" src/components/ --include="*.tsx" | grep -v "t(" | grep -v test | grep -v mock
```

Fix any survivors.

---

## Phase 4: Performance Optimization

**Problem:** Unmemoized components and computations cause unnecessary re-renders.

### 4.1 SovereignDeveloperProfile sub-components

**File:** `src/components/SovereignDeveloperProfile.tsx`

- Wrap `DimensionRadar` render output in `useMemo`
- Wrap `AffinityList` render output in `useMemo`
- Memoize `formatDNAProfile` result with `useMemo`
- Verify no stale closures introduced

### 4.2 Fix learnedAffinities dependency warnings

**Files:**
- `src/components/IntelligenceProfileCard.tsx`
- `src/components/result-item/BadgeRow.tsx`

- Audit `useEffect` / `useMemo` dependency arrays
- Add missing `learnedAffinities` to dependency arrays OR
- Restructure to use store selector that returns stable references
- Verify with `pnpm run lint` — zero warnings

### 4.3 ResultsView virtual scroll audit

**File:** `src/components/ResultsView.tsx`

- Verify `@tanstack/react-virtual` is using stable `estimateSize` callback
- Ensure row components are wrapped in `memo()` with proper comparison
- Check that sort/filter changes properly reset virtualizer

### 4.4 Store selector optimization

**Files:** Multiple components using `useAppStore`

- Audit components using `useShallow` — ensure selectors return minimal state
- Check for components subscribing to entire slices when they need 1-2 fields
- Fix any selector that causes re-renders on unrelated state changes

---

## Phase 5: Test Coverage for Audit Fixes

**Problem:** Audit fixes (i18n, accessibility, error handling, Pro guards) have zero test coverage.

### 5.1 ProvenanceTooltip Escape key test

**File:** `src/components/channels/__tests__/ProvenanceTooltip.test.tsx` (new)

Test:
- Renders tooltip content
- Pressing Escape calls onClose
- Displays `t('channels.provenance')` label

### 5.2 DecisionMemory error handling tests

**File:** `src/components/__tests__/DecisionMemory.test.tsx` (new or extend)

Tests:
- `handleSubmit` catches invoke errors and shows error toast
- `handleSupersede` catches errors and shows error toast
- `handleReconsider` catches errors and shows error toast
- Loading state shown during async operations
- Form validation prevents empty submissions

### 5.3 IntelligenceProfileCard Pro guard test

**File:** `src/components/__tests__/IntelligenceProfileCard.test.tsx` (new or extend)

Tests:
- KnowledgeGapsCard returns null when `isPro` is false
- KnowledgeGapsCard renders content when `isPro` is true
- Component wrapped in `memo()` prevents unnecessary re-renders

### 5.4 CalibrationView i18n test

**File:** `src/components/__tests__/CalibrationView.test.tsx` (new)

Tests:
- All calibration sections render with i18n keys (not hardcoded strings)
- ARIA roles present on key elements
- Action buttons have accessible labels

### 5.5 RadarSVG a11y test

**File:** `src/components/tech-radar/__tests__/RadarSVG.test.tsx` (new)

Tests:
- SVG has `role="img"` and `aria-label`
- Ring labels use i18n keys

### 5.6 Run full test suite

```bash
pnpm run test
```

All tests must pass. No skipped tests allowed.

---

## Phase 6: Rust Backend Audit

**Problem:** 100 .rs files not audited for dead code after streamlining deleted 42 frontend files and removed features.

### 6.1 Dead code detection

```bash
cd src-tauri && cargo build 2>&1 | grep "warning.*dead_code\|warning.*unused"
```

For each warning:
- If the function is genuinely unused → delete it
- If it's a Tauri command not yet wired → wire it or delete it
- If it's used but compiler can't see the usage → add `#[allow(dead_code)]` with comment explaining why

### 6.2 Orphaned Tauri commands

**File:** `src-tauri/src/lib.rs`

Cross-reference every function in `invoke_handler!` against frontend `invoke()` calls:
```bash
grep -rn "invoke(" src/ --include="*.ts" --include="*.tsx" | grep -oP "'[a-z_]+'" | sort -u
```

Compare against registered commands. Remove any command that:
- Has no frontend caller
- Has no MCP server caller
- Is not part of the public API

### 6.3 Orphaned modules

Check for modules declared in `lib.rs` or `mod.rs` that export nothing used:
- Review each `mod declaration` in lib.rs
- If the module only contained code for deleted features → remove the module file

### 6.4 Validate Rust

```bash
cd src-tauri && cargo test && cargo clippy -- -D warnings
```

Zero warnings, zero test failures.

---

## Phase 7: Final Polish & Validation

### 7.1 DecisionMemory loading state

**File:** `src/components/DecisionMemory.tsx`

- Add `isSubmitting` state boolean
- Set true before invoke, false in finally block
- Disable submit button and show spinner during submission
- Apply to all three handlers (submit, supersede, reconsider)

### 7.2 Keyboard shortcuts modal audit

**File:** `src/components/KeyboardShortcutsModal.tsx`

- Verify all listed shortcuts still work after streamlining
- Remove any shortcuts for deleted features
- Add shortcuts for any new features without shortcuts

### 7.3 Full validation suite

```bash
pnpm run validate:all
pnpm run validate:sizes
cd src-tauri && cargo test
pnpm run test
pnpm run lint
npx tsc --noEmit
```

All must pass clean.

### 7.4 Commit strategy

One commit per phase:
1. `Extract BriefingAtmosphere + useGameComponent hook, fix file sizes`
2. `Add ARIA roles, keyboard nav, screen reader support across 6 components`
3. `Replace final hardcoded strings with i18n`
4. `Memoize SovereignProfile + fix dependency warnings`
5. `Add test coverage for audit fixes (5 new test files)`
6. `Remove dead Rust code and orphaned commands`
7. `DecisionMemory loading state + final polish`

---

## Success Criteria

- [ ] Zero files over 500 lines (TypeScript) or 1000 lines (Rust)
- [ ] Zero ESLint warnings
- [ ] Zero hardcoded user-facing strings in components
- [ ] Every interactive element has ARIA attributes
- [ ] Every async form has loading/error states
- [ ] Every Pro-gated feature has null guard for free tier
- [ ] All audit fixes have test coverage
- [ ] Zero dead code warnings in Rust
- [ ] `pnpm run validate:all` passes clean
- [ ] `cargo clippy -- -D warnings` passes clean

---

## Appendix: Pro Tier Strategy

> Preserved from previous planning session. Execute after hardening is complete.

### Pro Tier — Make It Irresistible

**Objective:** Transform 4DA Pro from "nice analytics panels" into an irresistible upgrade with visible daily value, intelligent free-tier limits, and a flagship search feature.

**Outcome:** 13+ working Pro features, 3 usage gates, zero vapor promises.

#### Quick Wins
- Wire `get_semantic_shifts` into Tauri (already has Pro gate)
- Clean vapor features from PRO_FEATURES list (remove audio_briefing, predicted_context, context_packet)
- Clean game-components cache bust hack

#### Flagship: Natural Language Search
- Local intent parser (keywords, time ranges, file types, intent detection)
- FTS5 + sqlite-vec KNN hybrid search
- Optional LLM enhancement (BYOK/Ollama)
- ProGate wrapper in frontend

#### Usage Gates (3)
- Channel limit: Free = 3 custom, Pro = unlimited
- Monitoring frequency: Free = 30 min, Pro = 5 min
- History depth: Free = 30 days, Pro = unlimited

#### Polish Features
- Weekly Intelligence Digest (aggregates attention, gaps, signals, health, stats)
- Decision Impact Tracking (monitors signals related to recorded decisions)
