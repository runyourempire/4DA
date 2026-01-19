import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import "./App.css";
import { SplashScreen } from "./components/SplashScreen";

// Wrapper for invoke that handles errors gracefully
const invokeCommand = async <T,>(cmd: string, args?: Record<string, unknown>): Promise<T> => {
  return invoke<T>(cmd, args);
};

interface ContextFile {
  path: string;
  content: string;
  lines: number;
}

interface RelevanceMatch {
  source_file: string;
  matched_text: string;
  similarity: number;
}

interface HNRelevance {
  id: number;
  title: string;
  url: string | null;
  top_score: number;
  matches: RelevanceMatch[];
  relevant: boolean;
}

interface AnalysisProgress {
  stage: string;
  progress: number;
  message: string;
  items_processed: number;
  items_total: number;
}

interface Settings {
  llm: {
    provider: string;
    model: string;
    has_api_key: boolean;
    base_url: string | null;
  };
  rerank: {
    enabled: boolean;
    max_items_per_batch: number;
    min_embedding_score: number;
    daily_token_limit: number;
    daily_cost_limit_cents: number;
  };
  usage: {
    tokens_today: number;
    cost_today_cents: number;
    tokens_total: number;
    items_reranked: number;
  };
  embedding_threshold: number;
}

interface MonitoringStatus {
  enabled: boolean;
  interval_minutes: number;
  is_checking: boolean;
  last_check_ago: string | null;
  total_checks: number;
}

interface UserContext {
  role: string | null;
  tech_stack: string[];
  domains: string[];
  interests: Array<{
    id: number;
    topic: string;
    weight: number;
    source: string;
    has_embedding: boolean;
  }>;
  exclusions: string[];
  stats: {
    interest_count: number;
    exclusion_count: number;
  };
}

// Phase E: System Health Types
interface Anomaly {
  id: number | null;
  anomaly_type: string;
  topic: string | null;
  description: string;
  confidence: number;
  severity: string;
  evidence: string[];
  detected_at: string;
  resolved: boolean;
}

interface SystemHealth {
  anomalies: Anomaly[];
  anomalyCount: number;
  embeddingOperational: boolean;
  rateLimitStatus: {
    global_remaining: number;
    source_remaining: number;
    is_limited: boolean;
  } | null;
  accuracyMetrics: {
    precision: number;
    engagement_rate: number;
    calibration_error: number;
  } | null;
}

interface AppState {
  contextFiles: ContextFile[];
  relevanceResults: HNRelevance[];
  status: string;
  loading: boolean;
  analysisComplete: boolean;
  progress: number;
  progressMessage: string;
  progressStage: string;
}

function App() {
  const [state, setState] = useState<AppState>({
    contextFiles: [],
    relevanceResults: [],
    status: "Ready to analyze",
    loading: false,
    analysisComplete: false,
    progress: 0,
    progressMessage: "",
    progressStage: "",
  });

  const [expandedItem, setExpandedItem] = useState<number | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [settingsForm, setSettingsForm] = useState({
    provider: "anthropic",
    apiKey: "",
    model: "claude-3-haiku-20240307",
    baseUrl: "",
    rerankEnabled: false,
    maxItems: 15,
    minScore: 0.25,
    dailyTokenLimit: 100000,
    dailyCostLimit: 50,
  });
  const [settingsStatus, setSettingsStatus] = useState("");
  const [monitoring, setMonitoring] = useState<MonitoringStatus | null>(null);
  const [monitoringInterval, setMonitoringInterval] = useState(30);
  const [showSplash, setShowSplash] = useState(true);

  // Context Engine state
  const [userContext, setUserContext] = useState<UserContext | null>(null);
  const [newInterest, setNewInterest] = useState("");
  const [newExclusion, setNewExclusion] = useState("");
  const [newTechStack, setNewTechStack] = useState("");
  const [newRole, setNewRole] = useState("");

  // System Health state (Phase E)
  const [systemHealth, setSystemHealth] = useState<SystemHealth | null>(null);
  const [similarTopicQuery, setSimilarTopicQuery] = useState("");
  const [similarTopicResults, setSimilarTopicResults] = useState<Array<{ topic: string; similarity: number }>>([]);

  // Browser mode detection
  const [isBrowserMode, setIsBrowserMode] = useState(false);

  // Autonomous Context Discovery state
  const [scanDirectories, setScanDirectories] = useState<string[]>([]);
  const [newScanDir, setNewScanDir] = useState("");
  const [isScanning, setIsScanning] = useState(false);
  const [discoveredContext, setDiscoveredContext] = useState<{
    tech: Array<{ name: string; category: string; confidence: number }>;
    topics: string[];
    lastScan: string | null;
  }>({ tech: [], topics: [], lastScan: null });

  // Set up event listeners for background analysis
  useEffect(() => {
    let unlistenProgress: UnlistenFn | null = null;
    let unlistenComplete: UnlistenFn | null = null;
    let unlistenError: UnlistenFn | null = null;

    const setupListeners = async () => {
      // Listen for progress updates
      unlistenProgress = await listen<AnalysisProgress>("analysis-progress", (event) => {
        const { stage, progress, message, items_processed, items_total } = event.payload;
        setState((s) => ({
          ...s,
          progress,
          progressMessage: message,
          progressStage: stage,
          status: items_total > 0
            ? `${message} (${items_processed}/${items_total})`
            : message,
        }));
      });

      // Listen for completion
      unlistenComplete = await listen<HNRelevance[]>("analysis-complete", (event) => {
        const results = event.payload;
        const relevantCount = results.filter((r) => r.relevant).length;
        setState((s) => ({
          ...s,
          relevanceResults: results,
          status: `Analysis complete: ${relevantCount}/${results.length} items relevant`,
          loading: false,
          analysisComplete: true,
          progress: 1,
          progressStage: "complete",
        }));
      });

      // Listen for errors
      unlistenError = await listen<string>("analysis-error", (event) => {
        setState((s) => ({
          ...s,
          status: `Error: ${event.payload}`,
          loading: false,
          progress: 0,
          progressStage: "error",
        }));
      });
    };

    setupListeners();

    return () => {
      // Cleanup listeners
      if (unlistenProgress) unlistenProgress();
      if (unlistenComplete) unlistenComplete();
      if (unlistenError) unlistenError();
    };
  }, []);

  useEffect(() => {
    loadContextFiles();
    loadSettings();
    loadMonitoringStatus();
    loadUserContext();
    loadSystemHealth();
    loadDiscoveredContext();

    // Auto-trigger initial analysis after a short delay for better autonomy
    const autoAnalyzeTimer = setTimeout(async () => {
      try {
        const status = await invokeCommand<MonitoringStatus>("get_monitoring_status");
        // Only auto-analyze if monitoring is enabled and no checks have been done yet
        if (status.enabled && status.total_checks === 0) {
          console.log("[4DA] Auto-triggering initial analysis...");
          await invokeCommand("start_background_analysis");
        }
      } catch (error) {
        console.log("Auto-analyze check failed:", error);
      }
    }, 3000); // 3 second delay to let app fully initialize

    return () => clearTimeout(autoAnalyzeTimer);
  }, []);

  async function loadSettings() {
    try {
      const s = await invokeCommand<Settings>("get_settings");
      setSettings(s);
      // Initialize form with current settings
      setSettingsForm((f) => ({
        ...f,
        provider: s.llm.provider !== "none" ? s.llm.provider : "anthropic",
        model: s.llm.model || "claude-3-haiku-20240307",
        baseUrl: s.llm.base_url || "",
        rerankEnabled: s.rerank.enabled,
        maxItems: s.rerank.max_items_per_batch,
        minScore: s.rerank.min_embedding_score,
        dailyTokenLimit: s.rerank.daily_token_limit,
        dailyCostLimit: s.rerank.daily_cost_limit_cents,
      }));
    } catch (error) {
      console.log("Settings not available:", error);
    }
  }

  async function loadContextFiles() {
    setState((s) => ({ ...s, loading: true, status: "Loading context files..." }));
    try {
      const files = await invokeCommand<ContextFile[]>("get_context_files");
      setState((s) => ({
        ...s,
        contextFiles: files,
        status: `Loaded ${files.length} context files. Click "Analyze" to compute relevance.`,
        loading: false,
      }));
      setIsBrowserMode(false);
    } catch (error) {
      const errorMsg = String(error);
      if (errorMsg.includes("invoke") || errorMsg.includes("__TAURI__")) {
        setIsBrowserMode(true);
        setState((s) => ({
          ...s,
          status: "Browser mode detected",
          loading: false,
        }));
      } else {
        setState((s) => ({ ...s, status: `Error: ${error}`, loading: false }));
      }
    }
  }

  async function clearContext() {
    try {
      const result = await invokeCommand<string>("clear_context");
      setState((s) => ({
        ...s,
        contextFiles: [],
        relevanceResults: [],
        status: result,
      }));
    } catch (error) {
      console.error("Failed to clear context:", error);
    }
  }

  async function indexContext() {
    setState((s) => ({ ...s, loading: true, status: "Indexing context files..." }));
    try {
      const result = await invokeCommand<string>("index_context");
      // Reload context files to show updated list
      const files = await invokeCommand<ContextFile[]>("get_context_files");
      setState((s) => ({
        ...s,
        contextFiles: files,
        status: result,
        loading: false,
      }));
    } catch (error) {
      setState((s) => ({
        ...s,
        status: `Index failed: ${error}`,
        loading: false,
      }));
    }
  }

  async function startAnalysis() {
    setState((s) => ({
      ...s,
      loading: true,
      status: "Starting background analysis...",
      analysisComplete: false,
      progress: 0,
      progressMessage: "Initializing...",
      progressStage: "init",
    }));

    try {
      await invokeCommand("start_background_analysis");
    } catch (error) {
      const errorMsg = String(error);
      if (errorMsg.includes("invoke") || errorMsg.includes("__TAURI__")) {
        setState((s) => ({
          ...s,
          status: "Cannot analyze in browser mode. Open through Tauri window.",
          loading: false,
        }));
      } else if (errorMsg.includes("already running")) {
        setState((s) => ({
          ...s,
          status: "Analysis already in progress...",
        }));
      } else {
        setState((s) => ({ ...s, status: `Error: ${error}`, loading: false }));
      }
    }
  }

  async function saveSettings() {
    setSettingsStatus("Saving...");
    try {
      // Save LLM provider
      await invokeCommand("set_llm_provider", {
        provider: settingsForm.provider,
        apiKey: settingsForm.apiKey || "",
        model: settingsForm.model,
        baseUrl: settingsForm.baseUrl || null,
      });

      // Save rerank config
      await invokeCommand("set_rerank_config", {
        enabled: settingsForm.rerankEnabled,
        maxItems: settingsForm.maxItems,
        minScore: settingsForm.minScore,
        dailyTokenLimit: settingsForm.dailyTokenLimit,
        dailyCostLimit: settingsForm.dailyCostLimit,
      });

      setSettingsStatus("Settings saved!");
      await loadSettings();
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function testConnection() {
    setSettingsStatus("Testing connection...");
    try {
      // First save current settings
      await saveSettings();

      const result = await invokeCommand<{ success: boolean; message: string }>("test_llm_connection");
      setSettingsStatus(result.message);
    } catch (error) {
      setSettingsStatus(`Connection failed: ${error}`);
    }
  }

  async function loadMonitoringStatus() {
    try {
      const status = await invokeCommand<MonitoringStatus>("get_monitoring_status");
      setMonitoring(status);
      setMonitoringInterval(status.interval_minutes);
    } catch (error) {
      console.log("Monitoring status not available:", error);
    }
  }

  async function toggleMonitoring() {
    if (!monitoring) return;
    try {
      const newEnabled = !monitoring.enabled;
      await invokeCommand("set_monitoring_enabled", { enabled: newEnabled });
      await loadMonitoringStatus();
      setSettingsStatus(newEnabled ? "Monitoring enabled" : "Monitoring disabled");
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function updateMonitoringInterval() {
    try {
      await invokeCommand("set_monitoring_interval", { minutes: monitoringInterval });
      await loadMonitoringStatus();
      setSettingsStatus(`Interval set to ${monitoringInterval} minutes`);
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function testNotification() {
    try {
      await invokeCommand("trigger_notification_test");
      setSettingsStatus("Test notification sent!");
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Notification error: ${error}`);
    }
  }

  // Context Engine functions
  async function loadUserContext() {
    try {
      const ctx = await invokeCommand<UserContext>("get_user_context");
      setUserContext(ctx);
      if (ctx.role) setNewRole(ctx.role);
    } catch (error) {
      console.log("Context not available:", error);
    }
  }

  // Autonomous Context Discovery functions
  async function runFullScan() {
    if (scanDirectories.length === 0) {
      setSettingsStatus("Add directories to scan first");
      setTimeout(() => setSettingsStatus(""), 2000);
      return;
    }

    setIsScanning(true);
    setSettingsStatus("Scanning directories for context...");

    try {
      const result = await invokeCommand<{
        success: boolean;
        manifest_scan: { detected_tech: number; confidence: number };
        git_scan: { repos_analyzed: number; total_commits: number };
        combined: { total_topics: number; topics: string[] };
      }>("ace_full_scan", { paths: scanDirectories });

      // Also get the detected tech details
      const techResult = await invokeCommand<{
        detected_tech: Array<{ name: string; category: string; confidence: number }>;
      }>("ace_get_detected_tech");

      setDiscoveredContext({
        tech: techResult.detected_tech || [],
        topics: result.combined?.topics || [],
        lastScan: new Date().toISOString(),
      });

      setSettingsStatus(`Scan complete: ${techResult.detected_tech?.length || 0} technologies, ${result.combined?.total_topics || 0} topics discovered`);
      setTimeout(() => setSettingsStatus(""), 3000);
    } catch (error) {
      console.error("Full scan failed:", error);
      setSettingsStatus(`Scan failed: ${error}`);
    } finally {
      setIsScanning(false);
    }
  }

  async function addScanDirectory() {
    if (newScanDir.trim() && !scanDirectories.includes(newScanDir.trim())) {
      const newDirs = [...scanDirectories, newScanDir.trim()];
      setScanDirectories(newDirs);
      setNewScanDir("");
      // Persist to settings
      try {
        await invokeCommand("set_context_dirs", { dirs: newDirs });
      } catch (error) {
        console.error("Failed to save directories:", error);
      }
    }
  }

  async function removeScanDirectory(dir: string) {
    const newDirs = scanDirectories.filter(d => d !== dir);
    setScanDirectories(newDirs);
    // Persist to settings
    try {
      await invokeCommand("set_context_dirs", { dirs: newDirs });
    } catch (error) {
      console.error("Failed to save directories:", error);
    }
  }

  async function loadDiscoveredContext() {
    try {
      // Load saved directories from settings
      const dirs = await invokeCommand<string[]>("get_context_dirs");
      if (dirs && dirs.length > 0) {
        setScanDirectories(dirs);
      }

      // Load detected tech
      const techResult = await invokeCommand<{
        detected_tech: Array<{ name: string; category: string; confidence: number }>;
      }>("ace_get_detected_tech");

      if (techResult.detected_tech && techResult.detected_tech.length > 0) {
        setDiscoveredContext(prev => ({
          ...prev,
          tech: techResult.detected_tech,
        }));
      }

      // Load active topics
      const topicsResult = await invokeCommand<{
        topics: Array<{ topic: string; weight: number }>;
      }>("ace_get_active_topics");

      if (topicsResult.topics && topicsResult.topics.length > 0) {
        setDiscoveredContext(prev => ({
          ...prev,
          topics: topicsResult.topics.map(t => t.topic),
        }));
      }
    } catch (error) {
      console.log("No discovered context yet:", error);
    }
  }

  // System Health functions (Phase E)
  async function loadSystemHealth() {
    // Load each component independently so partial failures don't block everything
    let anomalies: Anomaly[] = [];
    let anomalyCount = 0;
    let embeddingOperational = false;
    let rateLimitStatus = null;
    let accuracyMetrics = null;

    // Fetch anomalies
    try {
      const result = await invokeCommand<{ anomalies: Anomaly[]; count: number }>("ace_get_unresolved_anomalies");
      anomalies = result.anomalies || [];
      anomalyCount = result.count || 0;
    } catch (error) {
      console.log("Anomalies not available:", error);
    }

    // Fetch embedding status
    try {
      const result = await invokeCommand<{ operational: boolean }>("ace_embedding_status");
      console.log("Embedding status result:", result);
      embeddingOperational = result.operational ?? false;
    } catch (error) {
      console.error("Embedding status error:", error);
    }

    // Fetch rate limit status
    try {
      const result = await invokeCommand<{ global_remaining: number; source_remaining: number; is_limited: boolean }>("ace_get_rate_limit_status", { source: "global" });
      console.log("Rate limit status result:", result);
      rateLimitStatus = result;
    } catch (error) {
      console.error("Rate limit status error:", error);
    }

    // Fetch accuracy metrics
    try {
      const result = await invokeCommand<{ precision: number; engagement_rate: number; calibration_error: number }>("ace_get_accuracy_metrics");
      accuracyMetrics = result;
    } catch (error) {
      console.log("Accuracy metrics not available:", error);
    }

    setSystemHealth({
      anomalies,
      anomalyCount,
      embeddingOperational,
      rateLimitStatus,
      accuracyMetrics,
    });
  }

  async function runAnomalyDetection() {
    try {
      setSettingsStatus("Running anomaly detection...");
      const result = await invokeCommand<{ anomalies: Anomaly[]; count: number }>("ace_detect_anomalies");
      await loadSystemHealth();
      setSettingsStatus(`Found ${result.count} anomalies`);
      setTimeout(() => setSettingsStatus(""), 3000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function resolveAnomaly(anomalyId: number) {
    try {
      await invokeCommand("ace_resolve_anomaly", { anomalyId });
      setSettingsStatus("Anomaly resolved");
      await loadSystemHealth();
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function findSimilarTopics() {
    if (!similarTopicQuery.trim()) return;
    try {
      const result = await invokeCommand<{ query: string; results: Array<{ topic: string; similarity: number }> }>(
        "ace_find_similar_topics",
        { query: similarTopicQuery.trim(), topK: 5 }
      );
      setSimilarTopicResults(result.results || []);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function saveWatcherState() {
    try {
      await invokeCommand("ace_save_watcher_state");
      setSettingsStatus("Watcher state saved");
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function addInterest() {
    if (!newInterest.trim()) return;
    try {
      await invokeCommand("add_interest", { topic: newInterest.trim() });
      setNewInterest("");
      setSettingsStatus(`Added interest: ${newInterest}`);
      await loadUserContext();
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function removeInterest(topic: string) {
    try {
      await invokeCommand("remove_interest", { topic });
      setSettingsStatus(`Removed interest: ${topic}`);
      await loadUserContext();
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function addExclusion() {
    if (!newExclusion.trim()) return;
    try {
      await invokeCommand("add_exclusion", { topic: newExclusion.trim() });
      setNewExclusion("");
      setSettingsStatus(`Added exclusion: ${newExclusion}`);
      await loadUserContext();
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function removeExclusion(topic: string) {
    try {
      await invokeCommand("remove_exclusion", { topic });
      setSettingsStatus(`Removed exclusion: ${topic}`);
      await loadUserContext();
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function addTechStack() {
    if (!newTechStack.trim()) return;
    try {
      await invokeCommand("add_tech_stack", { technology: newTechStack.trim() });
      setNewTechStack("");
      setSettingsStatus(`Added technology: ${newTechStack}`);
      await loadUserContext();
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function removeTechStack(technology: string) {
    try {
      await invokeCommand("remove_tech_stack", { technology });
      setSettingsStatus(`Removed technology: ${technology}`);
      await loadUserContext();
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  async function updateRole() {
    try {
      await invokeCommand("set_user_role", { role: newRole.trim() || null });
      setSettingsStatus(`Role updated to: ${newRole || "(none)"}`);
      await loadUserContext();
      setTimeout(() => setSettingsStatus(""), 2000);
    } catch (error) {
      setSettingsStatus(`Error: ${error}`);
    }
  }

  function formatScore(score: number): string {
    return (score * 100).toFixed(1) + "%";
  }

  function getScoreColor(score: number): string {
    if (score >= 0.5) return "text-success";
    if (score >= 0.35) return "text-accent-gold";
    return "text-text-muted";
  }

  function getStageLabel(stage: string): string {
    switch (stage) {
      case "init": return "Initializing";
      case "context": return "Loading Context";
      case "fetch": return "Fetching Stories";
      case "scrape": return "Scraping Content";
      case "embed": return "Embedding";
      case "relevance": return "Computing Relevance";
      case "rerank": return "LLM Re-ranking";
      case "complete": return "Complete";
      default: return stage;
    }
  }

  const providerModels: Record<string, string[]> = {
    anthropic: ["claude-3-haiku-20240307", "claude-3-sonnet-20240229", "claude-3-opus-20240229"],
    openai: ["gpt-4o-mini", "gpt-4o", "gpt-4-turbo", "gpt-3.5-turbo"],
    ollama: ["llama3", "mistral", "mixtral", "phi3"],
  };

  return (
    <>
      {/* Splash Screen */}
      {showSplash && (
        <SplashScreen onComplete={() => setShowSplash(false)} minimumDisplayTime={2500} />
      )}

      <div className={`min-h-screen bg-bg-primary text-text-primary p-6 ${showSplash ? 'opacity-0' : 'opacity-100 transition-opacity duration-300'}`}>
      {/* Header */}
      <header className="mb-6 flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">4DA Home</h1>
          <p className="text-text-secondary text-sm mt-1">Phase 3 - Continuous Monitoring</p>
        </div>
        <div className="flex items-center gap-3">
          {monitoring?.enabled && (
            <div className="flex items-center gap-2 px-3 py-1.5 bg-success/10 border border-success/30 rounded">
              <div className="w-2 h-2 bg-success rounded-full animate-pulse" />
              <span className="text-xs text-success">Monitoring</span>
            </div>
          )}
          <button
            onClick={() => setShowSettings(true)}
            className="px-3 py-1.5 text-sm bg-bg-secondary text-text-secondary border border-border rounded hover:bg-bg-tertiary transition-colors"
          >
            Settings
          </button>
        </div>
      </header>

      {/* Browser Mode Warning */}
      {isBrowserMode && (
        <div className="mb-6 px-4 py-3 bg-error/10 border border-error/50 rounded flex items-center gap-3">
          <svg className="w-5 h-5 text-error flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
          <div className="flex-1">
            <p className="text-sm font-medium text-error">Running in Browser Mode</p>
            <p className="text-xs text-text-muted">Open 4DA through the desktop app for full functionality. Tauri API unavailable.</p>
          </div>
        </div>
      )}

      {/* Status Bar with Progress */}
      <div className="mb-6 px-4 py-3 bg-bg-secondary rounded border border-border">
        <div className="flex items-center gap-3">
          {state.loading && (
            <div className="w-2 h-2 bg-accent-gold rounded-full animate-pulse" />
          )}
          <span className="text-sm text-text-secondary font-mono flex-1">{state.status}</span>
          {settings?.rerank.enabled && settings?.llm.has_api_key && (
            <span className="text-xs text-accent-gold px-2 py-0.5 border border-accent-gold/30 rounded">
              LLM
            </span>
          )}
          <button
            onClick={startAnalysis}
            disabled={state.loading}
            className="px-4 py-1.5 text-sm bg-accent-primary text-bg-primary font-medium rounded hover:opacity-90 transition-opacity disabled:opacity-30"
          >
            {state.loading ? "Analyzing..." : "Analyze"}
          </button>
        </div>

        {/* Progress Bar */}
        {state.loading && state.progress > 0 && (
          <div className="mt-3">
            <div className="flex justify-between text-xs text-text-muted mb-1">
              <span>{getStageLabel(state.progressStage)}</span>
              <span>{Math.round(state.progress * 100)}%</span>
            </div>
            <div className="w-full h-1.5 bg-bg-tertiary rounded-full overflow-hidden">
              <div
                className="h-full bg-accent-gold transition-all duration-300 ease-out"
                style={{ width: `${state.progress * 100}%` }}
              />
            </div>
          </div>
        )}
      </div>

      {/* Main Content */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Context Files Panel - Narrow */}
        <section className="bg-bg-secondary rounded border border-border">
          <div className="px-4 py-3 border-b border-border flex items-center justify-between">
            <div>
              <h2 className="font-medium">Context</h2>
              <p className="text-xs text-text-muted mt-1">{state.contextFiles.length} files in directory</p>
            </div>
            <div className="flex gap-2">
              <button
                onClick={loadContextFiles}
                className="px-2 py-1 text-xs bg-bg-tertiary text-text-secondary border border-border rounded hover:bg-bg-primary transition-colors"
                title="Reload files from directory"
              >
                ↻
              </button>
              {state.contextFiles.length > 0 && (
                <>
                  <button
                    onClick={indexContext}
                    disabled={state.loading}
                    className="px-2 py-1 text-xs bg-success/10 text-success border border-success/30 rounded hover:bg-success/20 transition-colors disabled:opacity-50"
                    title="Index files for analysis (embed and store)"
                  >
                    Index
                  </button>
                  <button
                    onClick={clearContext}
                    className="px-2 py-1 text-xs bg-error/10 text-error border border-error/30 rounded hover:bg-error/20 transition-colors"
                    title="Clear indexed context from database"
                  >
                    Clear
                  </button>
                </>
              )}
            </div>
          </div>
          <div className="p-3 max-h-[calc(100vh-320px)] overflow-y-auto">
            {state.contextFiles.length === 0 ? (
              <div className="text-text-muted text-sm p-2 space-y-2">
                <p>No files in context directory</p>
                <p className="text-xs">Add files to your context directory, then click Index to enable context-based analysis.</p>
              </div>
            ) : (
              <ul className="space-y-1">
                {state.contextFiles.map((file) => (
                  <li
                    key={file.path}
                    className="px-2 py-1.5 bg-bg-tertiary rounded text-xs"
                  >
                    <div className="font-mono text-text-primary truncate">
                      {file.path.split("/").pop()?.split("\\").pop()}
                    </div>
                    <div className="text-text-muted">{file.lines} lines</div>
                  </li>
                ))}
              </ul>
            )}

            {/* ACE Discovered Context */}
            {(discoveredContext.tech.length > 0 || discoveredContext.topics.length > 0) && (
              <div className="mt-3 pt-3 border-t border-border">
                <div className="text-xs text-text-muted mb-2 flex items-center gap-1">
                  <span>ACE Discovered</span>
                  <span className="px-1 py-0.5 text-[9px] bg-accent-gold/20 text-accent-gold rounded">AUTO</span>
                </div>
                {discoveredContext.tech.length > 0 && (
                  <div className="mb-2">
                    <div className="flex flex-wrap gap-1">
                      {discoveredContext.tech.slice(0, 6).map((tech) => (
                        <span
                          key={tech.name}
                          className="px-1.5 py-0.5 text-[10px] bg-success/10 text-success rounded"
                          title={`${tech.category} - ${Math.round(tech.confidence * 100)}%`}
                        >
                          {tech.name}
                        </span>
                      ))}
                      {discoveredContext.tech.length > 6 && (
                        <span className="text-[10px] text-text-muted">+{discoveredContext.tech.length - 6}</span>
                      )}
                    </div>
                  </div>
                )}
                {discoveredContext.topics.length > 0 && (
                  <div className="flex flex-wrap gap-1">
                    {discoveredContext.topics.slice(0, 4).map((topic) => (
                      <span
                        key={topic}
                        className="px-1.5 py-0.5 text-[10px] bg-accent-gold/10 text-accent-gold rounded"
                      >
                        {topic}
                      </span>
                    ))}
                    {discoveredContext.topics.length > 4 && (
                      <span className="text-[10px] text-text-muted">+{discoveredContext.topics.length - 4}</span>
                    )}
                  </div>
                )}
              </div>
            )}
          </div>
        </section>

        {/* Relevance Results Panel - Wide */}
        <section className="lg:col-span-2 bg-bg-secondary rounded border border-border">
          <div className="px-4 py-3 border-b border-border flex items-center justify-between">
            <div>
              <h2 className="font-medium">Relevance Analysis</h2>
              <p className="text-xs text-text-muted mt-1">
                {state.analysisComplete
                  ? `Sorted by relevance score (threshold: 30%)`
                  : "Click Analyze to compute relevance scores"}
              </p>
            </div>
            {state.analysisComplete && (
              <div className="text-xs text-text-muted">
                <span className="text-success">{state.relevanceResults.filter((r) => r.relevant).length} relevant</span>
                {" / "}
                <span>{state.relevanceResults.length} total</span>
              </div>
            )}
          </div>
          <div className="p-4 max-h-[calc(100vh-320px)] overflow-y-auto">
            {!state.analysisComplete ? (
              <div className="text-center py-12 text-text-muted">
                {state.loading ? (
                  <>
                    <p className="text-lg mb-2">Analyzing...</p>
                    <p className="text-sm">{state.progressMessage}</p>
                  </>
                ) : (
                  <>
                    <p className="text-lg mb-2">No analysis yet</p>
                    <p className="text-sm">
                      Click "Analyze" to fetch HN stories and compute relevance scores
                    </p>
                  </>
                )}
              </div>
            ) : (
              <ul className="space-y-3">
                {state.relevanceResults.map((item) => (
                  <li
                    key={item.id}
                    className={`rounded border transition-colors ${
                      item.relevant
                        ? "bg-bg-tertiary border-border"
                        : "bg-bg-primary border-border/50"
                    }`}
                  >
                    {/* Main Row */}
                    <button
                      onClick={() => setExpandedItem(expandedItem === item.id ? null : item.id)}
                      className="w-full px-4 py-3 text-left"
                    >
                      <div className="flex items-start gap-3">
                        {/* Score Badge */}
                        <div
                          className={`flex-shrink-0 w-14 text-center py-1 rounded font-mono text-sm font-medium ${getScoreColor(
                            item.top_score
                          )}`}
                        >
                          {formatScore(item.top_score)}
                        </div>

                        {/* Title and URL */}
                        <div className="flex-1 min-w-0">
                          <div
                            className={`text-sm ${
                              item.relevant ? "text-text-primary" : "text-text-secondary"
                            }`}
                          >
                            {item.title}
                          </div>
                          {item.url && (
                            <div className="text-xs text-text-muted truncate font-mono mt-1">
                              {item.url}
                            </div>
                          )}
                        </div>

                        {/* Expand Indicator */}
                        <div className="text-text-muted text-xs">
                          {expandedItem === item.id ? "-" : "+"}
                        </div>
                      </div>
                    </button>

                    {/* Expanded Matches */}
                    {expandedItem === item.id && (
                      <div className="px-4 pb-3 border-t border-border/50 mt-2 pt-3">
                        <div className="text-xs text-text-muted mb-2 font-medium">
                          Top Matches:
                        </div>
                        <ul className="space-y-2">
                          {item.matches.map((match, i) => (
                            <li
                              key={i}
                              className="text-xs bg-bg-primary rounded p-2 border border-border/30"
                            >
                              <div className="flex items-center gap-2 mb-1">
                                <span className={`font-mono ${getScoreColor(match.similarity)}`}>
                                  {formatScore(match.similarity)}
                                </span>
                                <span className="text-text-muted">-&gt;</span>
                                <span className="text-accent-gold font-medium">
                                  {match.source_file}
                                </span>
                              </div>
                              <div className="text-text-secondary pl-12 leading-relaxed">
                                "{match.matched_text}"
                              </div>
                            </li>
                          ))}
                        </ul>
                      </div>
                    )}
                  </li>
                ))}
              </ul>
            )}
          </div>
        </section>
      </div>

      {/* Footer */}
      <footer className="mt-6 text-center text-xs text-text-muted">
        The internet searches for you.
      </footer>

      {/* Settings Modal */}
      {showSettings && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50 p-4">
          <div className="bg-bg-secondary border border-border rounded-lg w-full max-w-lg max-h-[90vh] overflow-y-auto">
            <div className="px-6 py-4 border-b border-border flex items-center justify-between">
              <h2 className="text-lg font-medium">Settings</h2>
              <button
                onClick={() => setShowSettings(false)}
                className="text-text-muted hover:text-text-primary"
              >
                x
              </button>
            </div>

            <div className="p-6 space-y-6">
              {/* LLM Provider Section */}
              <div>
                <h3 className="text-sm font-medium text-text-primary mb-3">LLM Provider (BYOK)</h3>

                <div className="space-y-3">
                  <div>
                    <label className="text-xs text-text-muted block mb-1">Provider</label>
                    <select
                      value={settingsForm.provider}
                      onChange={(e) => {
                        const newProvider = e.target.value;
                        setSettingsForm((f) => ({
                          ...f,
                          provider: newProvider,
                          model: providerModels[newProvider]?.[0] || "",
                          baseUrl: newProvider === "ollama" ? "http://localhost:11434" : "",
                        }));
                      }}
                      className="w-full px-3 py-2 bg-bg-tertiary border border-border rounded text-sm text-text-primary"
                    >
                      <option value="anthropic">Anthropic Claude</option>
                      <option value="openai">OpenAI</option>
                      <option value="ollama">Ollama (Local)</option>
                    </select>
                  </div>

                  {settingsForm.provider !== "ollama" && (
                    <div>
                      <label className="text-xs text-text-muted block mb-1">API Key</label>
                      <input
                        type="password"
                        value={settingsForm.apiKey}
                        onChange={(e) => setSettingsForm((f) => ({ ...f, apiKey: e.target.value }))}
                        placeholder={settings?.llm.has_api_key ? "(key saved)" : "Enter your API key"}
                        className="w-full px-3 py-2 bg-bg-tertiary border border-border rounded text-sm text-text-primary placeholder:text-text-muted"
                      />
                    </div>
                  )}

                  <div>
                    <label className="text-xs text-text-muted block mb-1">Model</label>
                    <select
                      value={settingsForm.model}
                      onChange={(e) => setSettingsForm((f) => ({ ...f, model: e.target.value }))}
                      className="w-full px-3 py-2 bg-bg-tertiary border border-border rounded text-sm text-text-primary"
                    >
                      {(providerModels[settingsForm.provider] || []).map((m) => (
                        <option key={m} value={m}>{m}</option>
                      ))}
                    </select>
                  </div>

                  {settingsForm.provider === "ollama" && (
                    <div>
                      <label className="text-xs text-text-muted block mb-1">Base URL</label>
                      <input
                        type="text"
                        value={settingsForm.baseUrl}
                        onChange={(e) => setSettingsForm((f) => ({ ...f, baseUrl: e.target.value }))}
                        placeholder="http://localhost:11434"
                        className="w-full px-3 py-2 bg-bg-tertiary border border-border rounded text-sm text-text-primary placeholder:text-text-muted"
                      />
                    </div>
                  )}
                </div>
              </div>

              {/* Re-ranking Section */}
              <div>
                <h3 className="text-sm font-medium text-text-primary mb-3">LLM Re-ranking</h3>

                <div className="space-y-3">
                  <label className="flex items-center gap-3 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={settingsForm.rerankEnabled}
                      onChange={(e) => setSettingsForm((f) => ({ ...f, rerankEnabled: e.target.checked }))}
                      className="w-4 h-4 accent-accent-gold"
                    />
                    <span className="text-sm text-text-secondary">Enable LLM re-ranking</span>
                  </label>

                  <p className="text-xs text-text-muted">
                    When enabled, items passing the embedding threshold will be sent to the LLM
                    for deeper relevance analysis. This improves precision but costs money.
                  </p>

                  <div className="grid grid-cols-2 gap-3">
                    <div>
                      <label className="text-xs text-text-muted block mb-1">Max items/batch</label>
                      <input
                        type="number"
                        value={settingsForm.maxItems}
                        onChange={(e) => setSettingsForm((f) => ({ ...f, maxItems: parseInt(e.target.value) || 15 }))}
                        className="w-full px-3 py-2 bg-bg-tertiary border border-border rounded text-sm text-text-primary"
                      />
                    </div>
                    <div>
                      <label className="text-xs text-text-muted block mb-1">Min embedding score</label>
                      <input
                        type="number"
                        step="0.05"
                        value={settingsForm.minScore}
                        onChange={(e) => setSettingsForm((f) => ({ ...f, minScore: parseFloat(e.target.value) || 0.25 }))}
                        className="w-full px-3 py-2 bg-bg-tertiary border border-border rounded text-sm text-text-primary"
                      />
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-3">
                    <div>
                      <label className="text-xs text-text-muted block mb-1">Daily token limit</label>
                      <input
                        type="number"
                        value={settingsForm.dailyTokenLimit}
                        onChange={(e) => setSettingsForm((f) => ({ ...f, dailyTokenLimit: parseInt(e.target.value) || 100000 }))}
                        className="w-full px-3 py-2 bg-bg-tertiary border border-border rounded text-sm text-text-primary"
                      />
                    </div>
                    <div>
                      <label className="text-xs text-text-muted block mb-1">Daily cost limit (cents)</label>
                      <input
                        type="number"
                        value={settingsForm.dailyCostLimit}
                        onChange={(e) => setSettingsForm((f) => ({ ...f, dailyCostLimit: parseInt(e.target.value) || 50 }))}
                        className="w-full px-3 py-2 bg-bg-tertiary border border-border rounded text-sm text-text-primary"
                      />
                    </div>
                  </div>
                </div>
              </div>

              {/* Usage Stats */}
              {settings && (
                <div>
                  <h3 className="text-sm font-medium text-text-primary mb-3">Usage</h3>
                  <div className="bg-bg-tertiary rounded p-3 text-xs font-mono text-text-secondary space-y-1">
                    <div>Today: {settings.usage.tokens_today.toLocaleString()} tokens (~${(settings.usage.cost_today_cents / 100).toFixed(3)})</div>
                    <div>Total: {settings.usage.tokens_total.toLocaleString()} tokens</div>
                    <div>Items re-ranked: {settings.usage.items_reranked}</div>
                  </div>
                </div>
              )}

              {/* Continuous Monitoring */}
              <div>
                <h3 className="text-sm font-medium text-text-primary mb-3">Continuous Monitoring</h3>
                <p className="text-xs text-text-muted mb-3">
                  When enabled, 4DA will automatically analyze sources at the specified interval
                  and send notifications when new relevant items are found.
                </p>

                {monitoring ? (
                  <div className="space-y-3">
                    <div className="flex items-center justify-between">
                      <span className="text-sm text-text-secondary">
                        Status: {monitoring.enabled ? (
                          <span className="text-success">Active</span>
                        ) : (
                          <span className="text-text-muted">Inactive</span>
                        )}
                        {monitoring.is_checking && (
                          <span className="text-accent-gold ml-2">(checking...)</span>
                        )}
                      </span>
                      <button
                        onClick={toggleMonitoring}
                        className={`px-3 py-1 text-sm rounded transition-colors ${
                          monitoring.enabled
                            ? "bg-error/20 text-error hover:bg-error/30"
                            : "bg-success/20 text-success hover:bg-success/30"
                        }`}
                      >
                        {monitoring.enabled ? "Stop" : "Start"}
                      </button>
                    </div>

                    <div className="flex items-center gap-2">
                      <label className="text-xs text-text-muted">Check every:</label>
                      <input
                        type="number"
                        min="5"
                        max="1440"
                        value={monitoringInterval}
                        onChange={(e) => setMonitoringInterval(parseInt(e.target.value) || 30)}
                        className="w-20 px-2 py-1 bg-bg-tertiary border border-border rounded text-sm text-text-primary"
                      />
                      <span className="text-xs text-text-muted">minutes</span>
                      <button
                        onClick={updateMonitoringInterval}
                        className="px-2 py-1 text-xs bg-bg-tertiary border border-border rounded text-text-secondary hover:bg-bg-primary transition-colors"
                      >
                        Set
                      </button>
                    </div>

                    <div className="bg-bg-tertiary rounded p-3 text-xs font-mono text-text-secondary space-y-1">
                      <div>Total checks: {monitoring.total_checks}</div>
                      {monitoring.last_check_ago && (
                        <div>Last check: {monitoring.last_check_ago}</div>
                      )}
                    </div>

                    <button
                      onClick={testNotification}
                      className="w-full px-3 py-1.5 text-xs bg-bg-tertiary border border-border rounded text-text-secondary hover:bg-bg-primary transition-colors"
                    >
                      Test Notification
                    </button>
                  </div>
                ) : (
                  <div className="text-xs text-text-muted">Loading monitoring status...</div>
                )}
              </div>

              {/* Automatic Context Discovery */}
              <div>
                <h3 className="text-sm font-medium text-text-primary mb-3">
                  Automatic Context Discovery
                  <span className="ml-2 px-1.5 py-0.5 text-[10px] bg-accent-gold/20 text-accent-gold rounded">ACE</span>
                </h3>
                <p className="text-xs text-text-muted mb-3">
                  Add directories to scan. 4DA will automatically detect your tech stack, projects, and interests
                  from project manifests and git history.
                </p>

                <div className="space-y-3">
                  {/* Add directory input */}
                  <div className="flex gap-2">
                    <input
                      type="text"
                      value={newScanDir}
                      onChange={(e) => setNewScanDir(e.target.value)}
                      onKeyDown={(e) => e.key === "Enter" && addScanDirectory()}
                      placeholder="e.g. ~/projects, D:\code, /home/user/work"
                      className="flex-1 px-3 py-1.5 bg-bg-tertiary border border-border rounded text-sm text-text-primary placeholder:text-text-muted"
                    />
                    <button
                      onClick={addScanDirectory}
                      className="px-3 py-1.5 text-xs bg-bg-tertiary border border-border rounded text-text-secondary hover:bg-bg-primary transition-colors"
                    >
                      Add
                    </button>
                  </div>

                  {/* Directory list */}
                  <div className="space-y-1">
                    {scanDirectories.length === 0 ? (
                      <p className="text-xs text-text-muted italic">No directories added. Add directories containing your projects to enable automatic context detection.</p>
                    ) : (
                      scanDirectories.map((dir) => (
                        <div key={dir} className="flex items-center justify-between px-2 py-1 bg-bg-tertiary rounded text-xs">
                          <span className="font-mono text-text-primary truncate">{dir}</span>
                          <button
                            onClick={() => removeScanDirectory(dir)}
                            className="text-text-muted hover:text-error ml-2"
                          >
                            ×
                          </button>
                        </div>
                      ))
                    )}
                  </div>

                  {/* Scan button */}
                  <button
                    onClick={runFullScan}
                    disabled={isScanning || scanDirectories.length === 0}
                    className="w-full px-3 py-2 text-sm bg-accent-gold/20 text-accent-gold border border-accent-gold/30 rounded hover:bg-accent-gold/30 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {isScanning ? "Scanning..." : "Scan for Context"}
                  </button>

                  {/* Discovered context display */}
                  {(discoveredContext.tech.length > 0 || discoveredContext.topics.length > 0) && (
                    <div className="bg-bg-tertiary rounded p-3 space-y-2">
                      <div className="text-xs text-text-muted">
                        Discovered Context {discoveredContext.lastScan && `(${new Date(discoveredContext.lastScan).toLocaleDateString()})`}
                      </div>
                      {discoveredContext.tech.length > 0 && (
                        <div>
                          <div className="text-xs text-text-muted mb-1">Tech Stack:</div>
                          <div className="flex flex-wrap gap-1">
                            {discoveredContext.tech.slice(0, 10).map((tech) => (
                              <span
                                key={tech.name}
                                className="px-1.5 py-0.5 text-[10px] bg-success/10 text-success rounded"
                                title={`${tech.category} - ${Math.round(tech.confidence * 100)}% confidence`}
                              >
                                {tech.name}
                              </span>
                            ))}
                            {discoveredContext.tech.length > 10 && (
                              <span className="text-[10px] text-text-muted">+{discoveredContext.tech.length - 10} more</span>
                            )}
                          </div>
                        </div>
                      )}
                      {discoveredContext.topics.length > 0 && (
                        <div>
                          <div className="text-xs text-text-muted mb-1">Topics:</div>
                          <div className="flex flex-wrap gap-1">
                            {discoveredContext.topics.slice(0, 8).map((topic) => (
                              <span
                                key={topic}
                                className="px-1.5 py-0.5 text-[10px] bg-accent-gold/10 text-accent-gold rounded"
                              >
                                {topic}
                              </span>
                            ))}
                            {discoveredContext.topics.length > 8 && (
                              <span className="text-[10px] text-text-muted">+{discoveredContext.topics.length - 8} more</span>
                            )}
                          </div>
                        </div>
                      )}
                    </div>
                  )}
                </div>
              </div>

              {/* Manual Context / Personalization */}
              <div>
                <h3 className="text-sm font-medium text-text-primary mb-3">Manual Adjustments</h3>
                <p className="text-xs text-text-muted mb-3">
                  Fine-tune your context manually. Add interests to boost specific topics, or exclude topics you don't want to see.
                </p>

                {userContext ? (
                  <div className="space-y-4">
                    {/* Role */}
                    <div>
                      <label className="text-xs text-text-muted block mb-1">Your Role</label>
                      <div className="flex gap-2">
                        <input
                          type="text"
                          value={newRole}
                          onChange={(e) => setNewRole(e.target.value)}
                          placeholder="e.g. Backend Developer"
                          className="flex-1 px-3 py-1.5 bg-bg-tertiary border border-border rounded text-sm text-text-primary placeholder:text-text-muted"
                        />
                        <button
                          onClick={updateRole}
                          className="px-3 py-1.5 text-xs bg-bg-tertiary border border-border rounded text-text-secondary hover:bg-bg-primary transition-colors"
                        >
                          Set
                        </button>
                      </div>
                    </div>

                    {/* Tech Stack */}
                    <div>
                      <label className="text-xs text-text-muted block mb-1">Tech Stack</label>
                      <div className="flex gap-2 mb-2">
                        <input
                          type="text"
                          value={newTechStack}
                          onChange={(e) => setNewTechStack(e.target.value)}
                          onKeyDown={(e) => e.key === "Enter" && addTechStack()}
                          placeholder="e.g. Rust, TypeScript"
                          className="flex-1 px-3 py-1.5 bg-bg-tertiary border border-border rounded text-sm text-text-primary placeholder:text-text-muted"
                        />
                        <button
                          onClick={addTechStack}
                          className="px-3 py-1.5 text-xs bg-bg-tertiary border border-border rounded text-text-secondary hover:bg-bg-primary transition-colors"
                        >
                          Add
                        </button>
                      </div>
                      <div className="flex flex-wrap gap-1">
                        {userContext.tech_stack.map((tech) => (
                          <span
                            key={tech}
                            className="inline-flex items-center gap-1 px-2 py-0.5 bg-accent-gold/10 text-accent-gold text-xs rounded"
                          >
                            {tech}
                            <button
                              onClick={() => removeTechStack(tech)}
                              className="hover:text-error"
                            >
                              ×
                            </button>
                          </span>
                        ))}
                        {userContext.tech_stack.length === 0 && (
                          <span className="text-xs text-text-muted">No technologies added</span>
                        )}
                      </div>
                    </div>

                    {/* Interests */}
                    <div>
                      <label className="text-xs text-text-muted block mb-1">
                        Interests <span className="text-success">({userContext.interests.length})</span>
                      </label>
                      <div className="flex gap-2 mb-2">
                        <input
                          type="text"
                          value={newInterest}
                          onChange={(e) => setNewInterest(e.target.value)}
                          onKeyDown={(e) => e.key === "Enter" && addInterest()}
                          placeholder="e.g. machine learning, distributed systems"
                          className="flex-1 px-3 py-1.5 bg-bg-tertiary border border-border rounded text-sm text-text-primary placeholder:text-text-muted"
                        />
                        <button
                          onClick={addInterest}
                          className="px-3 py-1.5 text-xs bg-success/20 text-success border border-success/30 rounded hover:bg-success/30 transition-colors"
                        >
                          Add
                        </button>
                      </div>
                      <div className="flex flex-wrap gap-1 max-h-24 overflow-y-auto">
                        {userContext.interests.map((interest) => (
                          <span
                            key={interest.topic}
                            className="inline-flex items-center gap-1 px-2 py-0.5 bg-success/10 text-success text-xs rounded"
                            title={interest.has_embedding ? "Has embedding" : "No embedding"}
                          >
                            {interest.has_embedding && <span className="w-1.5 h-1.5 bg-success rounded-full" />}
                            {interest.topic}
                            <button
                              onClick={() => removeInterest(interest.topic)}
                              className="hover:text-error"
                            >
                              ×
                            </button>
                          </span>
                        ))}
                        {userContext.interests.length === 0 && (
                          <span className="text-xs text-text-muted">No interests added</span>
                        )}
                      </div>
                    </div>

                    {/* Exclusions */}
                    <div>
                      <label className="text-xs text-text-muted block mb-1">
                        Exclusions <span className="text-error">({userContext.exclusions.length})</span>
                      </label>
                      <div className="flex gap-2 mb-2">
                        <input
                          type="text"
                          value={newExclusion}
                          onChange={(e) => setNewExclusion(e.target.value)}
                          onKeyDown={(e) => e.key === "Enter" && addExclusion()}
                          placeholder="e.g. crypto, sports"
                          className="flex-1 px-3 py-1.5 bg-bg-tertiary border border-border rounded text-sm text-text-primary placeholder:text-text-muted"
                        />
                        <button
                          onClick={addExclusion}
                          className="px-3 py-1.5 text-xs bg-error/20 text-error border border-error/30 rounded hover:bg-error/30 transition-colors"
                        >
                          Block
                        </button>
                      </div>
                      <div className="flex flex-wrap gap-1">
                        {userContext.exclusions.map((exclusion) => (
                          <span
                            key={exclusion}
                            className="inline-flex items-center gap-1 px-2 py-0.5 bg-error/10 text-error text-xs rounded"
                          >
                            {exclusion}
                            <button
                              onClick={() => removeExclusion(exclusion)}
                              className="hover:text-text-primary"
                            >
                              ×
                            </button>
                          </span>
                        ))}
                        {userContext.exclusions.length === 0 && (
                          <span className="text-xs text-text-muted">No exclusions set</span>
                        )}
                      </div>
                    </div>
                  </div>
                ) : (
                  <div className="text-xs text-text-muted">Loading context...</div>
                )}
              </div>

              {/* System Health (Phase E) */}
              <div>
                <h3 className="text-sm font-medium text-text-primary mb-3">System Health</h3>
                <p className="text-xs text-text-muted mb-3">
                  Monitor system health, detect anomalies, and explore topic similarities.
                </p>

                {systemHealth ? (
                  <div className="space-y-4">
                    {/* Service Status */}
                    <div className="grid grid-cols-2 gap-3">
                      <div className="bg-bg-tertiary rounded p-3">
                        <div className="text-xs text-text-muted mb-1">Embedding Service</div>
                        <div className={`text-sm font-medium ${systemHealth.embeddingOperational ? 'text-success' : 'text-error'}`}>
                          {systemHealth.embeddingOperational ? 'Operational' : 'Offline'}
                        </div>
                      </div>
                      <div className="bg-bg-tertiary rounded p-3">
                        <div className="text-xs text-text-muted mb-1">Rate Limit</div>
                        <div className={`text-sm font-medium ${
                          systemHealth.rateLimitStatus
                            ? (systemHealth.rateLimitStatus.is_limited ? 'text-error' : 'text-success')
                            : 'text-text-muted'
                        }`}>
                          {systemHealth.rateLimitStatus
                            ? (systemHealth.rateLimitStatus.is_limited
                                ? 'Limited'
                                : `${systemHealth.rateLimitStatus.global_remaining} remaining`)
                            : 'N/A'}
                        </div>
                      </div>
                    </div>

                    {/* Accuracy Metrics */}
                    {systemHealth.accuracyMetrics && (
                      <div className="bg-bg-tertiary rounded p-3">
                        <div className="text-xs text-text-muted mb-2">Accuracy Metrics</div>
                        <div className="grid grid-cols-3 gap-2 text-xs">
                          <div>
                            <span className="text-text-muted">Precision:</span>
                            <span className="ml-1 text-text-primary">{(systemHealth.accuracyMetrics.precision * 100).toFixed(1)}%</span>
                          </div>
                          <div>
                            <span className="text-text-muted">Engagement:</span>
                            <span className="ml-1 text-text-primary">{(systemHealth.accuracyMetrics.engagement_rate * 100).toFixed(1)}%</span>
                          </div>
                          <div>
                            <span className="text-text-muted">Calibration:</span>
                            <span className="ml-1 text-text-primary">{(systemHealth.accuracyMetrics.calibration_error * 100).toFixed(1)}%</span>
                          </div>
                        </div>
                        {systemHealth.accuracyMetrics.precision === 0 &&
                         systemHealth.accuracyMetrics.engagement_rate === 0 && (
                          <div className="text-xs text-text-muted mt-2 italic">
                            Metrics update as you interact with results (click, save, dismiss)
                          </div>
                        )}
                      </div>
                    )}

                    {/* Anomaly Detection */}
                    <div>
                      <div className="flex items-center justify-between mb-2">
                        <label className="text-xs text-text-muted">
                          Anomalies <span className={systemHealth.anomalyCount > 0 ? 'text-error' : 'text-success'}>({systemHealth.anomalyCount})</span>
                        </label>
                        <button
                          onClick={runAnomalyDetection}
                          className="px-2 py-1 text-xs bg-bg-tertiary border border-border rounded text-text-secondary hover:bg-bg-primary transition-colors"
                        >
                          Scan Now
                        </button>
                      </div>
                      {systemHealth.anomalies.length > 0 ? (
                        <div className="space-y-2 max-h-32 overflow-y-auto">
                          {systemHealth.anomalies.map((anomaly, i) => (
                            <div
                              key={anomaly.id || i}
                              className={`text-xs p-2 rounded border ${
                                anomaly.severity === 'high' || anomaly.severity === 'critical'
                                  ? 'bg-error/10 border-error/30'
                                  : anomaly.severity === 'medium'
                                  ? 'bg-accent-gold/10 border-accent-gold/30'
                                  : 'bg-bg-tertiary border-border'
                              }`}
                            >
                              <div className="flex items-start justify-between gap-2">
                                <div>
                                  <span className={`font-medium ${
                                    anomaly.severity === 'high' || anomaly.severity === 'critical' ? 'text-error' :
                                    anomaly.severity === 'medium' ? 'text-accent-gold' : 'text-text-secondary'
                                  }`}>
                                    {anomaly.anomaly_type.replace(/_/g, ' ')}
                                  </span>
                                  {anomaly.topic && <span className="text-text-muted ml-1">({anomaly.topic})</span>}
                                </div>
                                {anomaly.id && (
                                  <button
                                    onClick={() => resolveAnomaly(anomaly.id!)}
                                    className="text-success hover:text-success/80 text-xs"
                                  >
                                    Resolve
                                  </button>
                                )}
                              </div>
                              <div className="text-text-muted mt-1">{anomaly.description}</div>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <div className="text-xs text-success bg-success/10 rounded p-2">No anomalies detected</div>
                      )}
                    </div>

                    {/* Topic Similarity Search */}
                    <div>
                      <label className="text-xs text-text-muted block mb-1">Find Similar Topics</label>
                      <div className="flex gap-2 mb-2">
                        <input
                          type="text"
                          value={similarTopicQuery}
                          onChange={(e) => setSimilarTopicQuery(e.target.value)}
                          onKeyDown={(e) => e.key === "Enter" && findSimilarTopics()}
                          placeholder="e.g. machine learning"
                          className="flex-1 px-3 py-1.5 bg-bg-tertiary border border-border rounded text-sm text-text-primary placeholder:text-text-muted"
                        />
                        <button
                          onClick={findSimilarTopics}
                          className="px-3 py-1.5 text-xs bg-bg-tertiary border border-border rounded text-text-secondary hover:bg-bg-primary transition-colors"
                        >
                          Search
                        </button>
                      </div>
                      {similarTopicResults.length > 0 && (
                        <div className="space-y-1">
                          {similarTopicResults.map((result, i) => (
                            <div key={i} className="flex items-center justify-between text-xs bg-bg-tertiary rounded px-2 py-1">
                              <span className="text-text-primary">{result.topic}</span>
                              <span className="text-accent-gold">{(result.similarity * 100).toFixed(1)}%</span>
                            </div>
                          ))}
                        </div>
                      )}
                    </div>

                    {/* Watcher State */}
                    <div className="flex gap-2">
                      <button
                        onClick={saveWatcherState}
                        className="flex-1 px-3 py-1.5 text-xs bg-bg-tertiary border border-border rounded text-text-secondary hover:bg-bg-primary transition-colors"
                      >
                        Save Watcher State
                      </button>
                      <button
                        onClick={loadSystemHealth}
                        className="px-3 py-1.5 text-xs bg-bg-tertiary border border-border rounded text-text-secondary hover:bg-bg-primary transition-colors"
                      >
                        Refresh
                      </button>
                    </div>
                  </div>
                ) : (
                  <div className="text-xs text-text-muted">Loading system health...</div>
                )}
              </div>

              {/* Status */}
              {settingsStatus && (
                <div className={`text-sm p-3 rounded ${settingsStatus.includes("Error") || settingsStatus.includes("failed") ? "bg-error/20 text-error" : "bg-success/20 text-success"}`}>
                  {settingsStatus}
                </div>
              )}

              {/* Actions */}
              <div className="flex gap-3">
                <button
                  onClick={saveSettings}
                  className="flex-1 px-4 py-2 text-sm bg-accent-primary text-bg-primary font-medium rounded hover:opacity-90 transition-opacity"
                >
                  Save Settings
                </button>
                <button
                  onClick={testConnection}
                  className="px-4 py-2 text-sm bg-bg-tertiary text-text-secondary border border-border rounded hover:bg-bg-primary transition-colors"
                >
                  Test Connection
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
      </div>
    </>
  );
}

export default App;
