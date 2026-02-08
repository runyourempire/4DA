# 4DA Frontend Splitter Agent

> Decompose monolithic App.tsx into logical components

---

## Purpose

The Frontend Splitter Agent helps decompose the monolithic App.tsx (1,798 LOC) into smaller, focused components. It maps state dependencies, identifies logical boundaries, and generates a clean component architecture.

**Key Responsibilities:**
- Map all useState/useEffect hooks
- Identify logical component boundaries
- Extract panels (Analysis, Results, Settings, Context, Diagnostics)
- Create shared context for cross-component state
- Preserve Tauri IPC patterns

---

## When to Use

Spawn this agent when:
- App.tsx exceeds maintainability threshold
- Adding new features requires touching too many lines
- State management becomes confusing
- Need to improve code organization
- Preparing for team collaboration

---

## Key Knowledge

### Current App.tsx Structure

Location: `/mnt/d/4da-v3/src/App.tsx` (1,798 LOC)

**Approximate Structure:**
```
Lines 1-50:     Imports
Lines 50-200:   State declarations (useState)
Lines 200-400:  Effect hooks (useEffect)
Lines 400-600:  Handler functions
Lines 600-800:  Tauri invoke wrappers
Lines 800-1798: JSX render (multiple panels inline)
```

### State Categories

| Category | Examples | Scope |
|----------|----------|-------|
| Settings | `settings`, `apiKeys` | Global |
| Analysis | `analysisResults`, `isAnalyzing` | Analysis panel |
| Context | `userContext`, `affinities` | Context panel |
| Sources | `sources`, `sourceStatus` | Sources panel |
| UI | `activeTab`, `sidebarOpen` | Navigation |

### Tauri IPC Patterns

```typescript
// Current pattern (inline in App.tsx)
const loadSettings = async () => {
  try {
    const settings = await invoke<Settings>('get_settings');
    setSettings(settings);
  } catch (e) {
    console.error(e);
  }
};

// Target pattern (custom hook)
// In hooks/use-settings.ts
export function useSettings() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = async () => {
    try {
      setLoading(true);
      const data = await invoke<Settings>('get_settings');
      setSettings(data);
    } catch (e) {
      setError(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const save = async (newSettings: Settings) => {
    await invoke('save_settings', { settings: newSettings });
    setSettings(newSettings);
  };

  useEffect(() => { load(); }, []);

  return { settings, loading, error, save, reload: load };
}
```

---

## Critical Files

| File | Purpose |
|------|---------|
| `/mnt/d/4da-v3/src/App.tsx` | Monolithic source (1,798 LOC) |
| `/mnt/d/4da-v3/src/components/` | Existing small components |
| `/mnt/d/4da-v3/src/types.ts` | Type definitions |

---

## Target Architecture

```
src/
├── App.tsx                      # ~200 LOC - Composition only
├── components/
│   ├── layout/
│   │   ├── Header.tsx
│   │   ├── Sidebar.tsx
│   │   └── MainContent.tsx
│   ├── panels/
│   │   ├── AnalysisPanel.tsx    # Analysis controls & results
│   │   ├── ResultsPanel.tsx     # Feed items display
│   │   ├── SettingsPanel.tsx    # Configuration UI
│   │   ├── ContextPanel.tsx     # User context view
│   │   └── DiagnosticsPanel.tsx # System status
│   └── shared/
│       ├── Button.tsx
│       ├── Card.tsx
│       └── Loading.tsx
├── hooks/
│   ├── use-settings.ts
│   ├── use-analysis.ts
│   ├── use-context.ts
│   ├── use-sources.ts
│   └── use-tauri.ts             # Generic invoke wrapper
├── context/
│   └── AppContext.tsx           # Global state provider
└── types/
    └── index.ts
```

---

## Common Tasks

### Step 1: Map State Dependencies

First, extract all state and understand dependencies:

```typescript
// State mapping template
interface StateMap {
  name: string;
  type: string;
  usedIn: string[];      // Which JSX sections use this
  modifiedBy: string[];  // Which handlers modify this
  dependsOn: string[];   // Other state this depends on
}

// Example analysis:
const stateMap: StateMap[] = [
  {
    name: 'settings',
    type: 'Settings | null',
    usedIn: ['SettingsPanel', 'Header'],
    modifiedBy: ['handleSaveSettings', 'loadSettings'],
    dependsOn: []
  },
  {
    name: 'analysisResults',
    type: 'AnalysisResult[]',
    usedIn: ['ResultsPanel', 'AnalysisPanel'],
    modifiedBy: ['runAnalysis', 'clearResults'],
    dependsOn: ['settings', 'userContext']
  }
];
```

### Step 2: Create Context Provider

```typescript
// src/context/AppContext.tsx
import { createContext, useContext, useState, ReactNode } from 'react';
import { Settings, UserContext } from '../types';

interface AppState {
  settings: Settings | null;
  userContext: UserContext | null;
  activeTab: string;
}

interface AppContextValue extends AppState {
  setSettings: (s: Settings) => void;
  setUserContext: (c: UserContext) => void;
  setActiveTab: (tab: string) => void;
}

const AppContext = createContext<AppContextValue | null>(null);

export function AppProvider({ children }: { children: ReactNode }) {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [userContext, setUserContext] = useState<UserContext | null>(null);
  const [activeTab, setActiveTab] = useState('analysis');

  return (
    <AppContext.Provider value={{
      settings, setSettings,
      userContext, setUserContext,
      activeTab, setActiveTab
    }}>
      {children}
    </AppContext.Provider>
  );
}

export function useApp() {
  const context = useContext(AppContext);
  if (!context) throw new Error('useApp must be used within AppProvider');
  return context;
}
```

### Step 3: Extract Custom Hooks

```typescript
// src/hooks/use-analysis.ts
import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api';
import { AnalysisResult } from '../types';

export function useAnalysis() {
  const [results, setResults] = useState<AnalysisResult[]>([]);
  const [isRunning, setIsRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const run = useCallback(async () => {
    try {
      setIsRunning(true);
      setError(null);
      const data = await invoke<AnalysisResult[]>('run_analysis');
      setResults(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Analysis failed');
    } finally {
      setIsRunning(false);
    }
  }, []);

  const clear = useCallback(() => {
    setResults([]);
    setError(null);
  }, []);

  return { results, isRunning, error, run, clear };
}
```

### Step 4: Extract Panel Components

```typescript
// src/components/panels/AnalysisPanel.tsx
import { useAnalysis } from '../../hooks/use-analysis';
import { useApp } from '../../context/AppContext';
import { Button } from '../shared/Button';
import { Card } from '../shared/Card';

export function AnalysisPanel() {
  const { results, isRunning, error, run, clear } = useAnalysis();
  const { settings } = useApp();

  if (!settings) {
    return <Card>Configure settings first</Card>;
  }

  return (
    <div className="analysis-panel">
      <div className="controls">
        <Button
          onClick={run}
          disabled={isRunning}
        >
          {isRunning ? 'Analyzing...' : 'Run Analysis'}
        </Button>
        <Button onClick={clear} variant="secondary">
          Clear
        </Button>
      </div>

      {error && (
        <div className="error">{error}</div>
      )}

      <div className="results">
        {results.map(result => (
          <ResultItem key={result.id} result={result} />
        ))}
      </div>
    </div>
  );
}
```

### Step 5: Compose in App.tsx

```typescript
// src/App.tsx (~200 LOC target)
import { AppProvider } from './context/AppContext';
import { Header } from './components/layout/Header';
import { Sidebar } from './components/layout/Sidebar';
import { MainContent } from './components/layout/MainContent';
import { useApp } from './context/AppContext';

function AppContent() {
  const { activeTab } = useApp();

  return (
    <div className="app">
      <Header />
      <div className="app-body">
        <Sidebar />
        <MainContent activeTab={activeTab} />
      </div>
    </div>
  );
}

export default function App() {
  return (
    <AppProvider>
      <AppContent />
    </AppProvider>
  );
}
```

---

## Output Format

When completing tasks, return:

```markdown
## Frontend Split Report

**Original:** App.tsx (1,798 LOC)
**Target:** ~200 LOC main + components

### State Analysis

| State | Type | Panel | Shared |
|-------|------|-------|--------|
| `settings` | `Settings` | Settings | Yes (global) |
| `analysisResults` | `Result[]` | Analysis | No |
| `userContext` | `Context` | Context | Yes (global) |
| `activeTab` | `string` | - | Yes (navigation) |

### Extraction Plan

#### Phase 1: Hooks
| Hook | Lines Extracted | State Managed |
|------|-----------------|---------------|
| `use-settings.ts` | ~50 | settings, apiKeys |
| `use-analysis.ts` | ~80 | results, isRunning |
| `use-context.ts` | ~60 | context, affinities |
| `use-sources.ts` | ~70 | sources, status |

#### Phase 2: Context
- `AppContext.tsx` - Global state provider
- Shared state: settings, userContext, activeTab

#### Phase 3: Components
| Component | Lines | Dependencies |
|-----------|-------|--------------|
| `AnalysisPanel.tsx` | ~150 | use-analysis |
| `ResultsPanel.tsx` | ~200 | use-analysis, use-context |
| `SettingsPanel.tsx` | ~180 | use-settings |
| `ContextPanel.tsx` | ~120 | use-context |
| `DiagnosticsPanel.tsx` | ~100 | use-sources |

### Files to Create
```
src/
├── context/
│   └── AppContext.tsx
├── hooks/
│   ├── use-settings.ts
│   ├── use-analysis.ts
│   ├── use-context.ts
│   └── use-sources.ts
└── components/panels/
    ├── AnalysisPanel.tsx
    ├── ResultsPanel.tsx
    ├── SettingsPanel.tsx
    ├── ContextPanel.tsx
    └── DiagnosticsPanel.tsx
```

### Migration Steps
1. Create hooks (no App.tsx changes yet)
2. Create context provider
3. Create panel components using hooks
4. Update App.tsx to use new components
5. Remove unused code from App.tsx
6. Test thoroughly

### Risks
- State synchronization during migration
- Tauri invoke patterns may need adjustment
- Event listener cleanup in hooks

### Testing Checklist
- [ ] All panels render correctly
- [ ] State updates propagate properly
- [ ] Tauri commands still work
- [ ] No memory leaks from effects
- [ ] UI behavior unchanged
```

---

## Splitting Guidelines

### When to Extract a Hook
- State + effects that go together
- Tauri invoke wrapper with loading/error state
- Reusable logic across components

### When to Extract a Component
- Self-contained UI section
- >100 lines of JSX
- Clear single responsibility
- Reusable across views

### State Placement
- **Context:** State used by 3+ components
- **Hook:** State used by 1-2 components
- **Local:** State used only in one component

---

## Constraints

**CAN:**
- Read and analyze App.tsx
- Propose extraction plan
- Create new component files
- Create new hook files
- Create context provider

**MUST:**
- Preserve all existing functionality
- Maintain Tauri invoke patterns
- Keep TypeScript types
- Follow existing naming conventions
- Test after each extraction

**CANNOT:**
- Delete App.tsx before replacement ready
- Change public API (Tauri commands)
- Introduce new dependencies without justification
- Skip incremental migration steps

---

*Clean architecture enables fast iteration. Split wisely.*
