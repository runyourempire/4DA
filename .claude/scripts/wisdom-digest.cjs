#!/usr/bin/env node
/**
 * Wisdom Digest — Stop hook
 *
 * At session end, captures what happened and writes a pending digest
 * for the next session to process. This is the "consequence detection"
 * half of the autonomous wisdom layer.
 *
 * Detects: files modified, areas touched, commits made, architecture changes.
 * Writes: .claude/wisdom/pending.json (consumed by wisdom-auto.cjs on next session start).
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const CLAUDE_DIR = path.dirname(__dirname);
const PROJECT_ROOT = path.dirname(CLAUDE_DIR);
const WISDOM_DIR = path.join(CLAUDE_DIR, 'wisdom');
const STATE_FILE = path.join(WISDOM_DIR, 'state.json');
const PENDING_FILE = path.join(WISDOM_DIR, 'pending.json');

let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('readable', () => {
  let chunk;
  while ((chunk = process.stdin.read()) !== null) input += chunk;
});

process.stdin.on('end', () => {
  try {
    ensureDir(WISDOM_DIR);

    // Detect what happened this session via git
    const modifiedFiles = getModifiedFiles();
    const recentCommits = getRecentCommits();
    const areas = detectAreas(modifiedFiles);
    const aiFilesTouched = modifiedFiles.some(f => f.file.startsWith('.ai/'));
    const wisdomFilesTouched = modifiedFiles.some(f => f.file.includes('WISDOM'));

    // Update session counter
    const state = loadJSON(STATE_FILE, { sessionCount: 0 });
    state.sessionCount = (state.sessionCount || 0) + 1;
    state.lastSessionEnd = new Date().toISOString();
    saveJSON(STATE_FILE, state);

    // Only write digest if something meaningful happened
    if (modifiedFiles.length > 0 || recentCommits.length > 0) {
      const digest = {
        timestamp: new Date().toISOString(),
        sessionNumber: state.sessionCount,
        filesModified: modifiedFiles.length,
        fileList: modifiedFiles.slice(0, 15).map(f => f.file),
        areas: [...areas],
        aiFilesTouched,
        wisdomFilesTouched,
        recentCommits: recentCommits.slice(0, 5),
        processed: false,
      };
      saveJSON(PENDING_FILE, digest);
    }

    console.log(JSON.stringify({ status: 'success' }));
  } catch (e) {
    // Never fail the session stop
    console.log(JSON.stringify({ status: 'success' }));
  }
});

function getModifiedFiles() {
  try {
    const output = execSync('git status --porcelain', {
      cwd: PROJECT_ROOT, encoding: 'utf8', timeout: 5000,
    }).trim();
    if (!output) return [];
    return output.split('\n').filter(Boolean).map(line => ({
      status: line.substring(0, 2).trim(),
      file: line.substring(3),
    }));
  } catch (e) { return []; }
}

function getRecentCommits() {
  try {
    const output = execSync('git log --oneline --since="2 hours ago"', {
      cwd: PROJECT_ROOT, encoding: 'utf8', timeout: 5000,
    }).trim();
    return output ? output.split('\n') : [];
  } catch (e) { return []; }
}

function detectAreas(files) {
  const areas = new Set();
  for (const f of files) {
    if (f.file.startsWith('src-tauri/')) areas.add('backend');
    else if (f.file.startsWith('src/')) areas.add('frontend');
    if (f.file.startsWith('.ai/')) areas.add('architecture');
    if (f.file.startsWith('.claude/')) areas.add('tooling');
    if (f.file.startsWith('specs/')) areas.add('specifications');
    if (f.file.startsWith('mcp-')) areas.add('mcp');
  }
  return areas;
}

function ensureDir(dir) {
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
}

function loadJSON(filepath, fallback) {
  try {
    if (fs.existsSync(filepath)) return JSON.parse(fs.readFileSync(filepath, 'utf8'));
  } catch (e) {}
  return fallback;
}

function saveJSON(filepath, data) {
  try { fs.writeFileSync(filepath, JSON.stringify(data, null, 2)); } catch (e) {}
}
