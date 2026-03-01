#!/usr/bin/env node
/**
 * Prompt Analyzer - Automatic Subagent Recommendation System
 *
 * Runs as UserPromptSubmit hook BEFORE Claude processes each message.
 * Analyzes prompt complexity and injects subagent recommendations.
 *
 * This is the "automatic" part - it fires regardless of Claude's judgment.
 */

const fs = require('fs');
const path = require('path');

// Configuration
const CONFIG = {
  threshold: 4,                    // Score threshold for recommendation
  stateFile: path.join(__dirname, '..', 'analyzer-state.json'),
  logFile: path.join(__dirname, '..', 'sessions', 'analyzer.log'),
  enableLogging: true,
};

// Read hook input from stdin
let input = '';
process.stdin.setEncoding('utf8');

process.stdin.on('readable', () => {
  let chunk;
  while ((chunk = process.stdin.read()) !== null) {
    input += chunk;
  }
});

process.stdin.on('end', () => {
  try {
    const hookData = JSON.parse(input);
    const prompt = hookData.prompt || hookData.content || '';
    const sessionId = hookData.session_id || 'unknown';

    // Early exit: skip analysis for very short prompts (< 10 chars)
    // These are things like "y", "ok", "yes", "no", "continue", etc.
    if (prompt.trim().length < 10) {
      console.log(JSON.stringify({ status: 'success' }));
      return;
    }

    // Load state (reads from disk only, no write yet)
    const state = loadState();

    // Reset prompt counter on new session (> 10 min gap between prompts)
    if (state.lastPrompt && (Date.now() - state.lastPrompt) > 10 * 60 * 1000) {
      state.promptCount = 0;
    }

    // Analyze prompt
    const analysis = analyzePrompt(prompt, state);

    // Only write state when the analysis produced a meaningful signal.
    // This prevents disk writes on simple prompts like "commit this" or "looks good".
    const triggered = analysis.score >= CONFIG.threshold;
    if (triggered || analysis.signals.length > 0) {
      state.promptCount = (state.promptCount || 0) + 1;
      state.lastPrompt = Date.now();
      if (triggered) {
        state.complexPromptCount = (state.complexPromptCount || 0) + 1;
      }
      saveState(state);
    }

    // Log only triggered analyses (reduces log noise)
    if (CONFIG.enableLogging && triggered) {
      logAnalysis(prompt, analysis, sessionId);
    }

    // Output result
    console.log(JSON.stringify(analysis.result));

  } catch (e) {
    // On error, pass through without modification
    if (CONFIG.enableLogging) {
      logError(e);
    }
    console.log(JSON.stringify({ status: 'success' }));
  }
});

/**
 * Load persistent state
 */
function loadState() {
  try {
    if (fs.existsSync(CONFIG.stateFile)) {
      return JSON.parse(fs.readFileSync(CONFIG.stateFile, 'utf8'));
    }
  } catch (e) {
    // Ignore errors, return default state
  }
  return { promptCount: 0, complexPromptCount: 0, sessionStart: Date.now() };
}

/**
 * Save persistent state
 */
function saveState(state) {
  try {
    const dir = path.dirname(CONFIG.stateFile);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(CONFIG.stateFile, JSON.stringify(state, null, 2));
  } catch (e) {
    // Ignore write errors
  }
}

/**
 * Log analysis for debugging
 */
function logAnalysis(prompt, analysis, sessionId) {
  try {
    const dir = path.dirname(CONFIG.logFile);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
    const logEntry = {
      timestamp: new Date().toISOString(),
      sessionId,
      promptPreview: prompt.substring(0, 100).replace(/\n/g, ' '),
      score: analysis.score,
      signals: analysis.signals,
      recommended: analysis.recommendedAgent,
      triggered: analysis.score >= CONFIG.threshold,
    };
    fs.appendFileSync(CONFIG.logFile, JSON.stringify(logEntry) + '\n');
  } catch (e) {
    // Ignore log errors
  }
}

function logError(error) {
  try {
    const dir = path.dirname(CONFIG.logFile);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
    fs.appendFileSync(CONFIG.logFile, JSON.stringify({
      timestamp: new Date().toISOString(),
      error: error.message,
    }) + '\n');
  } catch (e) {
    // Ignore
  }
}

/**
 * Main analysis function
 */
function analyzePrompt(prompt, state) {
  const lowerPrompt = prompt.toLowerCase();
  const signals = [];
  let score = 0;
  let recommendedAgent = null;
  let agentReason = '';

  // === EXPLORATION SIGNALS (High confidence) ===
  const explorationPatterns = [
    { pattern: /where (is|are|can i find)/i, weight: 3 },
    { pattern: /find (all|the|every|where)/i, weight: 3 },
    { pattern: /search (for|the|through|across)/i, weight: 3 },
    { pattern: /look (for|through|at all)/i, weight: 2 },
    { pattern: /what files/i, weight: 3 },
    { pattern: /which (files|components|functions|modules)/i, weight: 3 },
    { pattern: /how (is|are|does) .{1,30} (implemented|work|structured|organized)/i, weight: 3 },
    { pattern: /understand (the|how|this)/i, weight: 2 },
    { pattern: /explore (the|this)/i, weight: 3 },
    { pattern: /codebase/i, weight: 2 },
    { pattern: /project structure/i, weight: 3 },
    { pattern: /walk me through/i, weight: 2 },
    { pattern: /show me (all|the|where)/i, weight: 2 },
  ];

  for (const { pattern, weight } of explorationPatterns) {
    if (pattern.test(prompt)) {
      signals.push('exploration');
      score += weight;
      if (!recommendedAgent) {
        recommendedAgent = 'Explore';
        agentReason = 'Codebase exploration should be isolated to prevent search output pollution';
      }
      break;
    }
  }

  // === MULTI-FILE / IMPLEMENTATION SIGNALS ===
  const multiFilePatterns = [
    { pattern: /refactor/i, weight: 4 },
    { pattern: /rename .{1,20} across/i, weight: 4 },
    { pattern: /update (all|every|multiple)/i, weight: 3 },
    { pattern: /change .{1,20} everywhere/i, weight: 4 },
    { pattern: /implement .{1,30} feature/i, weight: 4 },
    { pattern: /add (a |the |new )?(component|feature|module|system)/i, weight: 3 },
    { pattern: /create (a |the |new )?(component|feature|module|system)/i, weight: 3 },
    { pattern: /build (a|the) .{1,30} (feature|module|system)/i, weight: 4 },
    { pattern: /migrate/i, weight: 4 },
    { pattern: /restructure/i, weight: 4 },
    { pattern: /rewrite/i, weight: 4 },
    { pattern: /overhaul/i, weight: 5 },
  ];

  for (const { pattern, weight } of multiFilePatterns) {
    if (pattern.test(prompt)) {
      signals.push('multi-file');
      score += weight;
      if (!recommendedAgent || weight > 3) {
        recommendedAgent = 'general-purpose';
        agentReason = 'Multi-file implementation should use fresh context for each major task';
      }
      break;
    }
  }

  // === DEBUGGING SIGNALS ===
  // Tightened patterns: require surrounding context to avoid false positives
  // on casual mentions like "fix the commit" or "error in my understanding".
  // Compound patterns (multiple debug signals in one prompt) listed first with higher weight.
  const debugPatterns = [
    // Compound: debug + error context in same prompt (strongest signal)
    { pattern: /debug.{0,50}(crash|error|fail|bug|broken)|crash.{0,50}debug/i, weight: 5 },
    { pattern: /fix (the|this|a) (bug|error|issue|problem|crash|failure)/i, weight: 4 },
    { pattern: /(compile|build|test|runtime) (error|failure)/i, weight: 4 },
    { pattern: /stack ?trace/i, weight: 4 },
    { pattern: /TypeError|ReferenceError|SyntaxError|panic!?\b/i, weight: 4 },
    { pattern: /(is |it's |it is )(broken|not working|failing)/i, weight: 3 },
    { pattern: /(it|app|server|build|test|process|page) crash(es|ed|ing)/i, weight: 3 },
    { pattern: /debug (the|this|why)/i, weight: 3 },
    { pattern: /investigate (the|this|why)/i, weight: 3 },
    { pattern: /why (is|does|isn't|doesn't|won't|can't) (it|this|the)/i, weight: 3 },
    { pattern: /what's wrong with/i, weight: 3 },
    { pattern: /troubleshoot/i, weight: 3 },
  ];

  for (const { pattern, weight } of debugPatterns) {
    if (pattern.test(prompt)) {
      signals.push('debugging');
      score += weight;
      if (!recommendedAgent || recommendedAgent !== 'Explore') {
        recommendedAgent = 'debugger';
        agentReason = 'Debugging with logs/traces should be isolated to prevent context pollution';
      }
      break;
    }
  }

  // === TEST SIGNALS ===
  const testPatterns = [
    { pattern: /run (the |all |)tests/i, weight: 3 },
    { pattern: /test (the|this|all|everything)/i, weight: 2 },
    { pattern: /write tests? for/i, weight: 3 },
    { pattern: /add tests? (for|to)/i, weight: 3 },
    { pattern: /npm test/i, weight: 3 },
    { pattern: /cargo test/i, weight: 3 },
    { pattern: /pytest/i, weight: 3 },
    { pattern: /jest/i, weight: 2 },
    { pattern: /test coverage/i, weight: 3 },
  ];

  for (const { pattern, weight } of testPatterns) {
    if (pattern.test(prompt)) {
      signals.push('testing');
      score += weight;
      break;
    }
  }

  // === REVIEW SIGNALS ===
  const reviewPatterns = [
    { pattern: /review (the|this|my|our)/i, weight: 3 },
    { pattern: /code review/i, weight: 4 },
    { pattern: /check (the|this|my) (code|changes|implementation|work)/i, weight: 2 },
    { pattern: /look over/i, weight: 2 },
    { pattern: /audit/i, weight: 3 },
    { pattern: /security review/i, weight: 4 },
  ];

  for (const { pattern, weight } of reviewPatterns) {
    if (pattern.test(prompt)) {
      signals.push('review');
      score += weight;
      if (!recommendedAgent) {
        recommendedAgent = 'code-reviewer';
        agentReason = 'Code review analysis should be isolated for focused attention';
      }
      break;
    }
  }

  // === FILE COUNT SIGNALS ===
  const fileExtensions = prompt.match(/\w+\.(ts|tsx|js|jsx|rs|py|go|java|md|json|yaml|yml|toml|css|scss|html)\b/gi);
  if (fileExtensions) {
    const uniqueFiles = new Set(fileExtensions.map(f => f.toLowerCase()));
    if (uniqueFiles.size >= 3) {
      signals.push(`${uniqueFiles.size}-files-mentioned`);
      score += Math.min(uniqueFiles.size, 5);
    }
  }

  // === PATH SIGNALS ===
  const pathMentions = prompt.match(/(?:src|lib|components|hooks|utils|services|api|pages|routes|modules|features)\/[\w\-\/]+/gi);
  if (pathMentions && pathMentions.length >= 2) {
    signals.push('multiple-paths');
    score += 2;
  }

  // === COMPLEXITY KEYWORDS ===
  const complexityKeywords = [
    { word: 'architecture', weight: 2 },
    { word: 'restructure', weight: 3 },
    { word: 'overhaul', weight: 3 },
    { word: 'redesign', weight: 3 },
    { word: 'integrate', weight: 2 },
    { word: 'integration', weight: 2 },
    { word: 'infrastructure', weight: 2 },
    { word: 'comprehensive', weight: 2 },
    { word: 'complete rewrite', weight: 4 },
    { word: 'entire', weight: 1 },
    { word: 'everything', weight: 1 },
    { word: 'all the', weight: 1 },
    { word: 'every single', weight: 2 },
    { word: 'throughout', weight: 1 },
    { word: 'across the', weight: 1 },
  ];

  for (const { word, weight } of complexityKeywords) {
    if (lowerPrompt.includes(word)) {
      signals.push('complexity');
      score += weight;
    }
  }

  // === SESSION FATIGUE SIGNAL ===
  // Only trigger after very long sessions where context window is likely saturated
  if (state.promptCount > 50) {
    score += 1;
    signals.push('session-fatigue');
  }

  // === BUILD RESULT ===
  const triggered = score >= CONFIG.threshold;

  let result = { status: 'success' };

  if (triggered && recommendedAgent) {
    const implTask = isImplementationTask(prompt, signals);
    result.message = buildRecommendation(signals, recommendedAgent, agentReason, score, implTask);
  }

  return {
    score,
    signals: [...new Set(signals)],
    recommendedAgent,
    agentReason,
    result,
  };
}

/**
 * Build the recommendation message
 */
function buildRecommendation(signals, agent, reason, score, isImplementationTask) {
  const signalList = [...new Set(signals)].join(', ');

  let msg = `\n━━━ AUTOMATIC SUBAGENT RECOMMENDATION ━━━\n`;
  msg += `Detected: ${signalList}\n`;
  msg += `Complexity score: ${score}/${CONFIG.threshold} (threshold)\n`;
  msg += `\n`;
  msg += `⚡ RECOMMENDED: Spawn "${agent}" subagent\n`;
  msg += `Reason: ${reason}\n`;
  msg += `\n`;
  msg += `How: Use Task tool with subagent_type="${agent}"\n`;
  msg += `Include: Detailed task description, relevant file paths, expected output format\n`;

  // Two-phase protocol reminder for implementation tasks
  if (isImplementationTask) {
    msg += `\n`;
    msg += `━━━ CADE TWO-PHASE PROTOCOL REQUIRED ━━━\n`;
    msg += `This appears to be an implementation task. Follow the protocol:\n`;
    msg += `\n`;
    msg += `PHASE 1 (Orientation - NO CODE):\n`;
    msg += `  1. Consult .ai/WISDOM.md wisdom gates for this action type\n`;
    msg += `  2. Check .ai/INVARIANTS.md for relevant constraints\n`;
    msg += `  3. Review .ai/FAILURE_MODES.md for risky areas\n`;
    msg += `  4. Check MCP memory for prior art (recall_decisions, recall_learnings)\n`;
    msg += `  5. State goal, list files to modify, identify invariants\n`;
    msg += `  6. Propose approach and WAIT for approval\n`;
    msg += `\n`;
    msg += `PHASE 2 (Execution - CODE ONLY):\n`;
    msg += `  After approval: Implement, validate, record consequences\n`;
  }

  msg += `━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n`;

  return msg;
}

/**
 * Detect if this is an implementation task requiring two-phase protocol
 */
function isImplementationTask(prompt, signals) {
  const implementationSignals = ['multi-file', 'debugging', 'testing', 'complexity'];
  const hasImplementationSignal = signals.some(s => implementationSignals.includes(s));

  // Additional implementation keywords
  const implementationPatterns = [
    /implement/i,
    /create/i,
    /add (a|the|new)/i,
    /build/i,
    /write/i,
    /fix/i,
    /change/i,
    /update/i,
    /modify/i,
    /refactor/i,
    /integrate/i,
  ];

  const hasImplementationKeyword = implementationPatterns.some(p => p.test(prompt));

  // Exclude pure exploration/questions
  const questionPatterns = [
    /^(what|where|how|why|which|can you explain)/i,
    /\?$/,
    /show me/i,
    /tell me/i,
  ];
  const isPureQuestion = questionPatterns.some(p => p.test(prompt.trim()));

  return (hasImplementationSignal || hasImplementationKeyword) && !isPureQuestion;
}
