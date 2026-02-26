#!/usr/bin/env node
/**
 * AOS Session End — Stop hook
 *
 * At session end, performs git introspection and updates ops-state.json:
 * 1. Detects modified files and areas touched
 * 2. Scans commit messages for bug fix indicators → sets immuneScanPending
 * 3. Updates sovereignty component data (sessionsCompleted, file change frequency)
 * 4. Categorizes session work for strategic drift tracking
 * 5. Writes updated ops-state.json
 *
 * Follows same patterns as wisdom-digest.cjs. Never fails the session stop.
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const CLAUDE_DIR = path.dirname(__dirname);
const PROJECT_ROOT = path.dirname(CLAUDE_DIR);
const WISDOM_DIR = path.join(CLAUDE_DIR, 'wisdom');
const OPS_STATE_FILE = path.join(WISDOM_DIR, 'ops-state.json');

// Bug fix detection patterns
const FIX_PATTERNS = /\b(fix|bug|patch|resolve|regression|hotfix)\b/i;

// Area categorization for drift tracking
const AREA_MAP = [
  { pattern: /^src-tauri\/src\/(scoring|relevance)/, area: 'scoring' },
  { pattern: /^src-tauri\/src\/sources\//, area: 'sources' },
  { pattern: /^src-tauri\/src\/ace\//, area: 'ace' },
  { pattern: /^src-tauri\/src\/extractors\//, area: 'extractors' },
  { pattern: /^src-tauri\//, area: 'backend' },
  { pattern: /^src\/components\//, area: 'frontend' },
  { pattern: /^src\//, area: 'frontend' },
  { pattern: /^\.ai\//, area: 'architecture' },
  { pattern: /^\.claude\//, area: 'tooling' },
  { pattern: /^mcp-/, area: 'mcp' },
  { pattern: /^specs\//, area: 'specs' },
  { pattern: /^site\//, area: 'design' },
  { pattern: /^scripts\//, area: 'tooling' },
];

let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('readable', () => {
  let chunk;
  while ((chunk = process.stdin.read()) !== null) input += chunk;
});

process.stdin.on('end', () => {
  try {
    ensureDir(WISDOM_DIR);

    const state = loadState();
    const modifiedFiles = getModifiedFiles();
    const recentCommits = getRecentCommits();

    // 1. Bug fix detection
    const bugFixCommits = recentCommits.filter(c => FIX_PATTERNS.test(c));
    if (bugFixCommits.length > 0) {
      state.immuneScanPending = true;
      state.immuneContext = {
        commits: bugFixCommits,
        files: modifiedFiles.map(f => f.file),
        timestamp: new Date().toISOString(),
      };
    }

    // 2. Sovereignty component updates
    state.compound.sessionsCompleted = (state.compound.sessionsCompleted || 0) + 1;

    // 3. File change frequency tracking (for metabolism)
    const freq = state.metabolism.fileChangeFrequency || {};
    for (const f of modifiedFiles) {
      freq[f.file] = (freq[f.file] || 0) + 1;
    }
    state.metabolism.fileChangeFrequency = freq;

    // 4. Strategic drift data — categorize this session's work
    const sessionAreas = categorizeSession(modifiedFiles);
    if (sessionAreas.length > 0) {
      const driftEntry = {
        date: new Date().toISOString(),
        areas: sessionAreas,
        filesChanged: modifiedFiles.length,
        commits: recentCommits.length,
      };

      if (!state.drift.recentSessionCategories) {
        state.drift.recentSessionCategories = [];
      }
      state.drift.recentSessionCategories.push(driftEntry);

      // Rolling 30-day window
      const thirtyDaysAgo = Date.now() - 30 * 24 * 60 * 60 * 1000;
      state.drift.recentSessionCategories = state.drift.recentSessionCategories.filter(
        e => new Date(e.date).getTime() > thirtyDaysAgo
      );
    }

    // 5. Write updated state
    saveJSON(OPS_STATE_FILE, state);

    console.log(JSON.stringify({ status: 'success' }));
  } catch (e) {
    // Never fail the session stop
    console.log(JSON.stringify({ status: 'success' }));
  }
});

/**
 * Load ops-state.json with defaults for missing fields.
 */
function loadState() {
  const defaults = {
    sovereignty: { score: 0, components: {}, lastComputed: null },
    cadence: { lastDaily: null, lastWeekly: null, lastMonthly: null },
    escalationQueue: [],
    immuneScanPending: false,
    immuneContext: null,
    metabolism: { fileChangeFrequency: {}, lastMetabolismScan: null },
    drift: { recentSessionCategories: [], lastDriftReport: null },
    compound: {
      sessionsCompleted: 0, decisionsStored: 0, decisionsReferenced: 0,
      antibodiesCreated: 0, antibodiesTriggered: 0, crystallizationsRun: 0, reworkEvents: 0,
    },
  };

  try {
    if (fs.existsSync(OPS_STATE_FILE)) {
      const raw = JSON.parse(fs.readFileSync(OPS_STATE_FILE, 'utf8'));
      return deepMerge(defaults, raw);
    }
  } catch (e) {}
  return defaults;
}

/**
 * Get modified files from git status.
 */
function getModifiedFiles() {
  try {
    const output = execSync('git status --porcelain', {
      cwd: PROJECT_ROOT, encoding: 'utf8', timeout: 5000,
    }).trim();
    if (!output) return [];
    return output.split('\n').filter(Boolean).map(line => {
      const match = line.match(/^\s*(\S+)\s+(.*)/);
      if (!match) return null;
      return { status: match[1], file: match[2] };
    }).filter(Boolean);
  } catch (e) { return []; }
}

/**
 * Get recent commits (last 2 hours).
 */
function getRecentCommits() {
  try {
    const output = execSync('git log --oneline --since="2 hours ago"', {
      cwd: PROJECT_ROOT, encoding: 'utf8', timeout: 5000,
    }).trim();
    return output ? output.split('\n') : [];
  } catch (e) { return []; }
}

/**
 * Categorize session work by area for drift tracking.
 */
function categorizeSession(files) {
  const areas = new Set();
  for (const f of files) {
    for (const mapping of AREA_MAP) {
      if (mapping.pattern.test(f.file)) {
        areas.add(mapping.area);
        break;
      }
    }
  }
  return [...areas];
}

/**
 * Deep merge two objects.
 */
function deepMerge(target, source) {
  const result = { ...target };
  for (const key of Object.keys(source)) {
    if (source[key] && typeof source[key] === 'object' && !Array.isArray(source[key])
        && target[key] && typeof target[key] === 'object' && !Array.isArray(target[key])) {
      result[key] = deepMerge(target[key], source[key]);
    } else {
      result[key] = source[key];
    }
  }
  return result;
}

function ensureDir(dir) {
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
}

function saveJSON(filepath, data) {
  try { fs.writeFileSync(filepath, JSON.stringify(data, null, 2)); } catch (e) {}
}
