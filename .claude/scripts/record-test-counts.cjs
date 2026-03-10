#!/usr/bin/env node
/**
 * Record current test counts into ops-state.json.
 *
 * Usage: node record-test-counts.cjs [--rust N] [--frontend N]
 *
 * If no arguments, runs `cargo test --lib` and `pnpm run test -- --run`
 * to count tests automatically. With arguments, uses provided counts
 * (useful when tests were already run and you don't want to re-run).
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const ROOT = path.resolve(__dirname, '..', '..');
const WISDOM_DIR = path.join(__dirname, '..', 'wisdom');
const OPS_STATE = path.join(WISDOM_DIR, 'ops-state.json');
const MAX_HISTORY = 50; // Keep last 50 recordings

function main() {
  const args = process.argv.slice(2);
  let rustCount = null;
  let frontendCount = null;

  // Parse args
  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--rust' && args[i + 1]) rustCount = parseInt(args[i + 1], 10);
    if (args[i] === '--frontend' && args[i + 1]) frontendCount = parseInt(args[i + 1], 10);
  }

  // Auto-detect if not provided
  if (rustCount === null) {
    try {
      const output = execSync('cargo test --lib 2>&1', {
        cwd: path.join(ROOT, 'src-tauri'),
        encoding: 'utf8',
        timeout: 300000,
      });
      const match = output.match(/test result: ok\. (\d+) passed/);
      if (match) rustCount = parseInt(match[1], 10);
    } catch (e) {
      const output = e.stdout || '';
      const match = output.match(/test result: ok\. (\d+) passed/);
      if (match) rustCount = parseInt(match[1], 10);
      else {
        console.error('Failed to get Rust test count. Use --rust N to provide manually.');
        process.exit(1);
      }
    }
  }

  if (frontendCount === null) {
    try {
      const output = execSync('pnpm run test -- --run 2>&1', {
        cwd: ROOT,
        encoding: 'utf8',
        timeout: 120000,
      });
      const match = output.match(/(\d+) passed/);
      if (match) frontendCount = parseInt(match[1], 10);
    } catch (e) {
      const output = e.stdout || '';
      const match = output.match(/(\d+) passed/);
      if (match) frontendCount = parseInt(match[1], 10);
      else {
        console.error('Failed to get frontend test count. Use --frontend N to provide manually.');
        process.exit(1);
      }
    }
  }

  const total = rustCount + frontendCount;
  const entry = {
    date: new Date().toISOString(),
    rust: rustCount,
    frontend: frontendCount,
    total,
  };

  // Load or create state
  if (!fs.existsSync(WISDOM_DIR)) fs.mkdirSync(WISDOM_DIR, { recursive: true });

  let state = {};
  try {
    state = JSON.parse(fs.readFileSync(OPS_STATE, 'utf8'));
  } catch (e) {
    state = {};
  }

  if (!state.testCounts) state.testCounts = { history: [], lastRecorded: null };

  // Append and trim
  state.testCounts.history.push(entry);
  if (state.testCounts.history.length > MAX_HISTORY) {
    state.testCounts.history = state.testCounts.history.slice(-MAX_HISTORY);
  }
  state.testCounts.lastRecorded = entry.date;

  fs.writeFileSync(OPS_STATE, JSON.stringify(state, null, 2));

  // Show delta
  const history = state.testCounts.history;
  let delta = '';
  if (history.length >= 2) {
    const prev = history[history.length - 2];
    const diff = total - prev.total;
    delta = diff > 0 ? ` (+${diff})` : diff < 0 ? ` (${diff})` : ' (=)';
  }

  console.log(`Recorded: ${total} tests${delta} (${rustCount} Rust + ${frontendCount} frontend)`);
}

main();
