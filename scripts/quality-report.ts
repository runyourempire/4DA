#!/usr/bin/env npx tsx
/**
 * CADE Quality Report Generator
 *
 * Aggregates quality metrics and generates weekly reports.
 * Reads from the MCP memory server's SQLite database.
 *
 * Usage: npx tsx scripts/quality-report.ts [--period=7]
 */

import Database from 'better-sqlite3';
import * as fs from 'fs';
import * as path from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Configuration
const DB_PATH = process.env.MEMORY_DB_PATH || path.join(__dirname, '..', 'mcp-memory-server', 'memory.db');
const REPORT_DIR = path.join(__dirname, '..', '.ai', 'reports');

// Quality targets from CADE specification
const TARGETS = {
  rework_rate: 20,          // <20% tasks requiring revision
  regression_frequency: 0,   // 0 per week
  mean_iterations: 5,        // <5 API calls per task
  confidence_accuracy: 80,   // >80% confidence vs actual success
  gate_pass_rate: 70,        // >70% first-attempt CI passes
};

interface MetricRow {
  id: number;
  timestamp: string;
  metric_type: string;
  value: number;
  context: string | null;
}

interface AggregatedMetrics {
  rework_rate: number | null;
  regression_frequency: number | null;
  mean_iterations: number | null;
  confidence_accuracy: number | null;
  gate_pass_rate: number | null;
  total_tasks: number;
  total_reworks: number;
  total_regressions: number;
  total_gate_attempts: number;
  total_gate_passes: number;
}

function main() {
  // Parse arguments
  const args = process.argv.slice(2);
  let periodDays = 7;

  for (const arg of args) {
    if (arg.startsWith('--period=')) {
      periodDays = parseInt(arg.split('=')[1], 10);
    }
  }

  console.log('═══════════════════════════════════════════════════════════════');
  console.log('                    CADE Quality Report                         ');
  console.log('═══════════════════════════════════════════════════════════════\n');

  // Check if database exists
  if (!fs.existsSync(DB_PATH)) {
    console.log('No metrics database found at:', DB_PATH);
    console.log('This is expected if no metrics have been recorded yet.');
    console.log('\nTo start collecting metrics, use the MCP memory server tools:');
    console.log('  - record_metric: Store a quality metric');
    console.log('  - get_metrics: Query stored metrics');
    console.log('  - get_quality_report: Generate a summary\n');
    process.exit(0);
  }

  // Open database
  const db = new Database(DB_PATH, { readonly: true });

  // Check if quality_metrics table exists
  const tableExists = db.prepare(`
    SELECT name FROM sqlite_master
    WHERE type='table' AND name='quality_metrics'
  `).get();

  if (!tableExists) {
    console.log('Quality metrics table not found in database.');
    console.log('The MCP memory server may need to be restarted to create the table.\n');
    db.close();
    process.exit(0);
  }

  // Calculate date range
  const endDate = new Date();
  const startDate = new Date(endDate.getTime() - periodDays * 24 * 60 * 60 * 1000);
  const startDateStr = startDate.toISOString();

  console.log(`Period: ${startDate.toLocaleDateString()} - ${endDate.toLocaleDateString()}`);
  console.log(`Days: ${periodDays}\n`);

  // Query metrics
  const metrics = db.prepare(`
    SELECT * FROM quality_metrics
    WHERE timestamp >= ?
    ORDER BY timestamp DESC
  `).all(startDateStr) as MetricRow[];

  if (metrics.length === 0) {
    console.log('No metrics recorded in this period.\n');
    console.log('Metrics are recorded automatically during development:');
    console.log('  - task_complete: When a task is finished');
    console.log('  - task_rework: When a task requires revision');
    console.log('  - regression: When an invariant violation occurs');
    console.log('  - gate_pass/gate_fail: CI pipeline results');
    console.log('  - iteration_count: API calls per task\n');
    db.close();
    process.exit(0);
  }

  // Aggregate metrics
  const aggregated = aggregateMetrics(metrics);

  // Generate report
  const report = generateReport(aggregated, periodDays, metrics.length);

  // Print to console
  console.log(report);

  // Save to file
  ensureReportDir();
  const reportFileName = `quality-report-${endDate.toISOString().split('T')[0]}.md`;
  const reportPath = path.join(REPORT_DIR, reportFileName);
  fs.writeFileSync(reportPath, report);
  console.log(`\nReport saved to: ${reportPath}\n`);

  db.close();
}

function aggregateMetrics(metrics: MetricRow[]): AggregatedMetrics {
  const result: AggregatedMetrics = {
    rework_rate: null,
    regression_frequency: null,
    mean_iterations: null,
    confidence_accuracy: null,
    gate_pass_rate: null,
    total_tasks: 0,
    total_reworks: 0,
    total_regressions: 0,
    total_gate_attempts: 0,
    total_gate_passes: 0,
  };

  const iterationCounts: number[] = [];
  const confidenceAccuracies: number[] = [];

  for (const metric of metrics) {
    switch (metric.metric_type) {
      case 'task_complete':
        result.total_tasks++;
        break;
      case 'task_rework':
        result.total_reworks++;
        break;
      case 'regression':
        result.total_regressions++;
        break;
      case 'gate_pass':
        result.total_gate_attempts++;
        result.total_gate_passes++;
        break;
      case 'gate_fail':
        result.total_gate_attempts++;
        break;
      case 'iteration_count':
        iterationCounts.push(metric.value);
        break;
      case 'confidence_accuracy':
        confidenceAccuracies.push(metric.value);
        break;
    }
  }

  // Calculate rates
  if (result.total_tasks > 0) {
    result.rework_rate = (result.total_reworks / result.total_tasks) * 100;
  }

  result.regression_frequency = result.total_regressions;

  if (iterationCounts.length > 0) {
    result.mean_iterations = iterationCounts.reduce((a, b) => a + b, 0) / iterationCounts.length;
  }

  if (confidenceAccuracies.length > 0) {
    result.confidence_accuracy = confidenceAccuracies.reduce((a, b) => a + b, 0) / confidenceAccuracies.length;
  }

  if (result.total_gate_attempts > 0) {
    result.gate_pass_rate = (result.total_gate_passes / result.total_gate_attempts) * 100;
  }

  return result;
}

function generateReport(metrics: AggregatedMetrics, periodDays: number, totalRecords: number): string {
  const lines: string[] = [];

  lines.push('# CADE Quality Report');
  lines.push('');
  lines.push(`**Generated:** ${new Date().toISOString()}`);
  lines.push(`**Period:** Last ${periodDays} days`);
  lines.push(`**Total Records:** ${totalRecords}`);
  lines.push('');

  lines.push('## Summary');
  lines.push('');
  lines.push('| Metric | Value | Target | Status |');
  lines.push('|--------|-------|--------|--------|');

  // Rework Rate
  const reworkStatus = metrics.rework_rate !== null
    ? (metrics.rework_rate <= TARGETS.rework_rate ? 'PASS' : 'FAIL')
    : 'N/A';
  lines.push(`| Rework Rate | ${formatPercent(metrics.rework_rate)} | <${TARGETS.rework_rate}% | ${statusEmoji(reworkStatus)} ${reworkStatus} |`);

  // Regression Frequency
  const regressionStatus = metrics.regression_frequency !== null
    ? (metrics.regression_frequency <= TARGETS.regression_frequency ? 'PASS' : 'FAIL')
    : 'N/A';
  lines.push(`| Regressions | ${metrics.regression_frequency ?? 'N/A'} | ${TARGETS.regression_frequency}/week | ${statusEmoji(regressionStatus)} ${regressionStatus} |`);

  // Mean Iterations
  const iterationStatus = metrics.mean_iterations !== null
    ? (metrics.mean_iterations <= TARGETS.mean_iterations ? 'PASS' : 'FAIL')
    : 'N/A';
  lines.push(`| Mean Iterations | ${metrics.mean_iterations?.toFixed(1) ?? 'N/A'} | <${TARGETS.mean_iterations} | ${statusEmoji(iterationStatus)} ${iterationStatus} |`);

  // Confidence Accuracy
  const confidenceStatus = metrics.confidence_accuracy !== null
    ? (metrics.confidence_accuracy >= TARGETS.confidence_accuracy ? 'PASS' : 'FAIL')
    : 'N/A';
  lines.push(`| Confidence Accuracy | ${formatPercent(metrics.confidence_accuracy)} | >${TARGETS.confidence_accuracy}% | ${statusEmoji(confidenceStatus)} ${confidenceStatus} |`);

  // Gate Pass Rate
  const gateStatus = metrics.gate_pass_rate !== null
    ? (metrics.gate_pass_rate >= TARGETS.gate_pass_rate ? 'PASS' : 'FAIL')
    : 'N/A';
  lines.push(`| Gate Pass Rate | ${formatPercent(metrics.gate_pass_rate)} | >${TARGETS.gate_pass_rate}% | ${statusEmoji(gateStatus)} ${gateStatus} |`);

  lines.push('');

  // Raw counts
  lines.push('## Raw Counts');
  lines.push('');
  lines.push(`- **Tasks Completed:** ${metrics.total_tasks}`);
  lines.push(`- **Tasks Requiring Rework:** ${metrics.total_reworks}`);
  lines.push(`- **Invariant Regressions:** ${metrics.total_regressions}`);
  lines.push(`- **CI Gate Attempts:** ${metrics.total_gate_attempts}`);
  lines.push(`- **CI Gate Passes:** ${metrics.total_gate_passes}`);
  lines.push('');

  // Recommendations
  lines.push('## Recommendations');
  lines.push('');

  const recommendations: string[] = [];

  if (metrics.rework_rate !== null && metrics.rework_rate > TARGETS.rework_rate) {
    recommendations.push('- **High rework rate:** Consider more thorough orientation phase before implementation');
  }

  if (metrics.regression_frequency !== null && metrics.regression_frequency > TARGETS.regression_frequency) {
    recommendations.push('- **Regressions detected:** Review .ai/INVARIANTS.md and add missing checks');
  }

  if (metrics.mean_iterations !== null && metrics.mean_iterations > TARGETS.mean_iterations) {
    recommendations.push('- **High iteration count:** Tasks may be under-specified; improve .ai/TASK_TEMPLATE.md usage');
  }

  if (metrics.confidence_accuracy !== null && metrics.confidence_accuracy < TARGETS.confidence_accuracy) {
    recommendations.push('- **Low confidence accuracy:** Agent overconfidence detected; review validation requirements');
  }

  if (metrics.gate_pass_rate !== null && metrics.gate_pass_rate < TARGETS.gate_pass_rate) {
    recommendations.push('- **Low gate pass rate:** Consider adding pre-commit checks or improving local validation');
  }

  if (recommendations.length === 0) {
    lines.push('All metrics within targets. Continue current practices.');
  } else {
    lines.push(...recommendations);
  }

  lines.push('');
  lines.push('---');
  lines.push('');
  lines.push('*Generated by CADE Quality Report Generator*');

  return lines.join('\n');
}

function formatPercent(value: number | null): string {
  if (value === null) return 'N/A';
  return `${value.toFixed(1)}%`;
}

function statusEmoji(status: string): string {
  switch (status) {
    case 'PASS': return '\u2705';  // Green checkmark
    case 'FAIL': return '\u274C';  // Red X
    default: return '\u2796';      // Neutral dash
  }
}

function ensureReportDir() {
  if (!fs.existsSync(REPORT_DIR)) {
    fs.mkdirSync(REPORT_DIR, { recursive: true });
  }
}

main();
