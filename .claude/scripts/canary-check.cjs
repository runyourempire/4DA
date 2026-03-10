#!/usr/bin/env node
/**
 * Canary Check — Quick regression detection for session start.
 *
 * Runs in < 30s. Checks:
 * 1. cargo check --lib (does it compile?)
 * 2. Test count vs. last recorded (are we losing tests?)
 * 3. File size limits (anything approaching limits?)
 * 4. Ghost command check (IPC drift?)
 *
 * Outputs JSON: { status: "success"|"error", message?: string }
 * If any canary fails, the message instructs Claude to fix before proceeding.
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const ROOT = path.resolve(__dirname, '..', '..');
const OPS_STATE = path.join(__dirname, '..', 'wisdom', 'ops-state.json');

function main() {
  const failures = [];
  const warnings = [];

  // 1. Cargo check (compilation)
  try {
    execSync('cargo check --lib', {
      cwd: path.join(ROOT, 'src-tauri'),
      stdio: 'pipe',
      timeout: 60000,
    });
  } catch (e) {
    failures.push('COMPILATION FAILURE: `cargo check --lib` failed. Fix compilation errors before any other work.');
  }

  // 2. Test count regression check
  try {
    const state = JSON.parse(fs.readFileSync(OPS_STATE, 'utf8'));
    const history = state.testCounts?.history || [];
    if (history.length > 0) {
      const last = history[history.length - 1];
      // Quick test count — run cargo test with --lib and count
      try {
        const rustOutput = execSync('cargo test --lib 2>&1 || true', {
          cwd: path.join(ROOT, 'src-tauri'),
          stdio: 'pipe',
          timeout: 120000,
          encoding: 'utf8',
        });
        const rustMatch = rustOutput.match(/test result: ok\. (\d+) passed/);
        if (rustMatch) {
          const currentRust = parseInt(rustMatch[1], 10);
          if (currentRust < last.rust) {
            failures.push(
              `TEST REGRESSION: Rust tests dropped from ${last.rust} to ${currentRust} (lost ${last.rust - currentRust} tests). Investigate before proceeding.`
            );
          }
        }
      } catch (e) {
        // cargo test failure is caught by compilation check
      }
    }
  } catch (e) {
    // No ops-state.json — skip test count check
  }

  // 3. File size check
  try {
    execSync('node scripts/check-file-sizes.cjs', {
      cwd: ROOT,
      stdio: 'pipe',
      timeout: 10000,
    });
  } catch (e) {
    const output = e.stdout?.toString() || e.message;
    warnings.push(`FILE SIZE WARNING: ${output.split('\n').filter(l => l.includes('ERROR')).join('; ') || 'Check file sizes'}`);
  }

  // 4. Ghost command check (IPC drift)
  try {
    const cmdFile = path.join(ROOT, 'src', 'lib', 'commands.ts');
    if (fs.existsSync(cmdFile)) {
      const cmdContent = fs.readFileSync(cmdFile, 'utf8');
      // Extract command names from CommandMap interface
      const cmdMapMatch = cmdContent.match(/interface CommandMap\s*\{([^}]+)\}/s);
      if (cmdMapMatch) {
        const frontendCmds = [...cmdMapMatch[1].matchAll(/['"]?(\w+)['"]?\s*:/g)].map(m => m[1]);

        // Check lib.rs for registered commands
        const libRs = fs.readFileSync(path.join(ROOT, 'src-tauri', 'src', 'lib.rs'), 'utf8');
        const missing = frontendCmds.filter(cmd => {
          // Convert camelCase to snake_case for Rust matching
          const snake = cmd.replace(/([A-Z])/g, '_$1').toLowerCase();
          return !libRs.includes(snake) && !libRs.includes(cmd);
        });

        if (missing.length > 0) {
          warnings.push(`GHOST COMMANDS: Frontend declares ${missing.length} command(s) not found in lib.rs invoke_handler: ${missing.slice(0, 5).join(', ')}`);
        }
      }
    }
  } catch (e) {
    // Non-critical
  }

  // Report
  if (failures.length > 0) {
    const msg = [
      'CANARY ALERT — Regressions detected. Fix these BEFORE any feature work:',
      '',
      ...failures.map((f, i) => `${i + 1}. ${f}`),
      ...(warnings.length > 0 ? ['', 'Warnings:', ...warnings.map(w => `  - ${w}`)] : []),
    ].join('\n');
    console.log(JSON.stringify({ status: 'error', message: msg }));
  } else if (warnings.length > 0) {
    const msg = [
      'Canary check passed with warnings:',
      ...warnings.map(w => `  - ${w}`),
    ].join('\n');
    console.log(JSON.stringify({ status: 'success', message: msg }));
  } else {
    console.log(JSON.stringify({ status: 'success' }));
  }
}

main();
