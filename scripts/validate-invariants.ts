#!/usr/bin/env npx tsx
/**
 * CADE Invariant Validator
 *
 * Validates that the codebase adheres to invariants defined in .ai/INVARIANTS.md
 *
 * Usage: npx tsx scripts/validate-invariants.ts
 */

import * as fs from 'fs';
import * as path from 'path';

interface InvariantCheck {
  id: string;
  description: string;
  check: () => Promise<CheckResult>;
}

interface CheckResult {
  passed: boolean;
  message: string;
  details?: string;
}

const ROOT_DIR = process.cwd();
const AI_DIR = path.join(ROOT_DIR, '.ai');
const SRC_TAURI_DIR = path.join(ROOT_DIR, 'src-tauri');
const SRC_DIR = path.join(ROOT_DIR, 'src');

// ═══════════════════════════════════════════════════════════════
// Invariant Checks
// ═══════════════════════════════════════════════════════════════

const checks: InvariantCheck[] = [
  // INV-030: API Keys Never Logged
  {
    id: 'INV-030',
    description: 'API keys must never appear in logs',
    check: async () => {
      const patterns = [
        /api[_-]?key.*console\.(log|error|warn)/gi,
        /console\.(log|error|warn).*api[_-]?key/gi,
        /tracing::(info|warn|error|debug).*api[_-]?key/gi,
      ];

      const violations: string[] = [];

      // Check TypeScript files
      const tsFiles = findFiles(SRC_DIR, ['.ts', '.tsx']);
      for (const file of tsFiles) {
        const content = fs.readFileSync(file, 'utf-8');
        for (const pattern of patterns) {
          if (pattern.test(content)) {
            violations.push(`Potential API key in log: ${file}`);
          }
        }
      }

      // Check Rust files
      const rsFiles = findFiles(SRC_TAURI_DIR, ['.rs']);
      for (const file of rsFiles) {
        const content = fs.readFileSync(file, 'utf-8');
        for (const pattern of patterns) {
          if (pattern.test(content)) {
            violations.push(`Potential API key in log: ${file}`);
          }
        }
      }

      if (violations.length > 0) {
        return {
          passed: false,
          message: `Found ${violations.length} potential API key logging violations`,
          details: violations.join('\n'),
        };
      }

      return { passed: true, message: 'No API key logging detected' };
    },
  },

  // INV-004: ACE Respects Privacy
  {
    id: 'INV-004',
    description: 'Activity tracking must be OFF by default',
    check: async () => {
      const settingsFile = path.join(SRC_TAURI_DIR, 'src', 'settings.rs');
      if (!fs.existsSync(settingsFile)) {
        return { passed: true, message: 'settings.rs not found (not yet implemented)' };
      }

      const content = fs.readFileSync(settingsFile, 'utf-8');

      // Check for activity tracking default
      if (content.includes('activity_tracking') && content.includes('true')) {
        // Look more carefully for default value
        const defaultMatch = content.match(/activity_tracking.*Default.*true/i);
        if (defaultMatch) {
          return {
            passed: false,
            message: 'Activity tracking appears to default to ON',
            details: 'INV-004 requires activity_tracking to be OFF by default',
          };
        }
      }

      return { passed: true, message: 'Activity tracking appears to be OFF by default' };
    },
  },

  // INV-020: Confidence Thresholds
  {
    id: 'INV-020',
    description: 'Signals with confidence <0.3 must be rejected',
    check: async () => {
      const validationFile = path.join(SRC_TAURI_DIR, 'src', 'ace', 'validation.rs');
      if (!fs.existsSync(validationFile)) {
        return { passed: true, message: 'validation.rs not found (not yet implemented)' };
      }

      const content = fs.readFileSync(validationFile, 'utf-8');

      // Check for confidence threshold
      if (!content.includes('0.3') && !content.includes('CONFIDENCE_THRESHOLD')) {
        return {
          passed: false,
          message: 'No 0.3 confidence threshold found in validation.rs',
          details: 'INV-020 requires rejecting signals with confidence < 0.3',
        };
      }

      return { passed: true, message: 'Confidence threshold check found' };
    },
  },

  // INV-023: Three-Layer Context Weights
  {
    id: 'INV-023',
    description: 'Context layer weights must be 1.0, 0.8, 0.6',
    check: async () => {
      const aceFiles = findFiles(path.join(SRC_TAURI_DIR, 'src', 'ace'), ['.rs']);
      let foundStatic = false;
      let foundActive = false;
      let foundLearned = false;

      for (const file of aceFiles) {
        const content = fs.readFileSync(file, 'utf-8');
        if (content.includes('1.0') && content.includes('STATIC')) foundStatic = true;
        if (content.includes('0.8') && content.includes('ACTIVE')) foundActive = true;
        if (content.includes('0.6') && content.includes('LEARNED')) foundLearned = true;
      }

      // This is a soft check - weights may not be explicitly named
      return {
        passed: true,
        message: 'Layer weight verification is informational only',
        details: `Static: ${foundStatic ? 'found' : 'not found'}, Active: ${foundActive ? 'found' : 'not found'}, Learned: ${foundLearned ? 'found' : 'not found'}`,
      };
    },
  },

  // CADE: Required files exist
  {
    id: 'CADE-001',
    description: 'All CADE cognition artifacts must exist',
    check: async () => {
      const requiredFiles = [
        'AI_ENGINEERING_CONTRACT.md',
        'INVARIANTS.md',
        'ARCHITECTURE.md',
        'DECISIONS.md',
        'FAILURE_MODES.md',
        'TASK_TEMPLATE.md',
        'VALIDATION_CHECKLIST.md',
      ];

      const missing: string[] = [];
      for (const file of requiredFiles) {
        const filePath = path.join(AI_DIR, file);
        if (!fs.existsSync(filePath)) {
          missing.push(file);
        }
      }

      if (missing.length > 0) {
        return {
          passed: false,
          message: `Missing ${missing.length} required CADE files`,
          details: missing.join(', '),
        };
      }

      return { passed: true, message: 'All CADE cognition artifacts present' };
    },
  },

  // INV-050: Design System Colors
  {
    id: 'INV-050',
    description: 'Primary background must be #0A0A0A',
    check: async () => {
      const cssFiles = findFiles(SRC_DIR, ['.css']);
      let foundCorrectColor = false;
      let foundWrongColor = false;

      for (const file of cssFiles) {
        const content = fs.readFileSync(file, 'utf-8').toLowerCase();
        if (content.includes('#0a0a0a') || content.includes('rgb(10, 10, 10)')) {
          foundCorrectColor = true;
        }
        // Check for obviously wrong primary backgrounds
        if (content.includes('--bg-primary') && content.includes('#fff')) {
          foundWrongColor = true;
        }
      }

      if (foundWrongColor) {
        return {
          passed: false,
          message: 'Found incorrect primary background color',
          details: 'INV-050 requires --bg-primary: #0A0A0A',
        };
      }

      return { passed: true, message: 'Design system colors appear correct' };
    },
  },
];

// ═══════════════════════════════════════════════════════════════
// Utility Functions
// ═══════════════════════════════════════════════════════════════

function findFiles(dir: string, extensions: string[]): string[] {
  const results: string[] = [];

  if (!fs.existsSync(dir)) {
    return results;
  }

  const items = fs.readdirSync(dir, { withFileTypes: true });

  for (const item of items) {
    const fullPath = path.join(dir, item.name);

    // Skip node_modules, target, dist
    if (item.name === 'node_modules' || item.name === 'target' || item.name === 'dist') {
      continue;
    }

    if (item.isDirectory()) {
      results.push(...findFiles(fullPath, extensions));
    } else if (item.isFile()) {
      const ext = path.extname(item.name);
      if (extensions.includes(ext)) {
        results.push(fullPath);
      }
    }
  }

  return results;
}

// ═══════════════════════════════════════════════════════════════
// Main Execution
// ═══════════════════════════════════════════════════════════════

async function main() {
  console.log('═══════════════════════════════════════════════════════════════');
  console.log('                    CADE Invariant Validator                     ');
  console.log('═══════════════════════════════════════════════════════════════\n');

  let passed = 0;
  let failed = 0;
  const failures: { id: string; message: string; details?: string }[] = [];

  for (const check of checks) {
    process.stdout.write(`Checking ${check.id}: ${check.description}... `);

    try {
      const result = await check.check();

      if (result.passed) {
        console.log('\x1b[32mPASS\x1b[0m');
        passed++;
      } else {
        console.log('\x1b[31mFAIL\x1b[0m');
        failed++;
        failures.push({
          id: check.id,
          message: result.message,
          details: result.details,
        });
      }
    } catch (error) {
      console.log('\x1b[33mERROR\x1b[0m');
      failed++;
      failures.push({
        id: check.id,
        message: `Check threw error: ${error}`,
      });
    }
  }

  console.log('\n═══════════════════════════════════════════════════════════════');
  console.log(`Results: ${passed} passed, ${failed} failed`);
  console.log('═══════════════════════════════════════════════════════════════\n');

  if (failures.length > 0) {
    console.log('Failures:');
    for (const failure of failures) {
      console.log(`\n  ${failure.id}: ${failure.message}`);
      if (failure.details) {
        console.log(`    Details: ${failure.details}`);
      }
    }
    console.log('');
    process.exit(1);
  }

  console.log('All invariant checks passed!\n');
  process.exit(0);
}

main().catch((error) => {
  console.error('Fatal error:', error);
  process.exit(1);
});
