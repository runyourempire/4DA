//! Settings module for 4DA
//!
//! Manages user configuration including API keys (BYOK), preferences,
//! and usage limits. Settings are stored in the app data directory.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// ============================================================================
// Settings Types
// ============================================================================

/// LLM Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMProvider {
    /// Provider type: "anthropic", "openai", "ollama"
    pub provider: String,
    /// API key (empty for Ollama)
    pub api_key: String,
    /// Model to use (e.g., "claude-3-haiku-20240307", "gpt-4o-mini")
    pub model: String,
    /// Base URL (for Ollama or custom endpoints)
    pub base_url: Option<String>,
}

impl Default for LLMProvider {
    fn default() -> Self {
        Self {
            provider: "none".to_string(),
            api_key: String::new(),
            model: String::new(),
            base_url: None,
        }
    }
}

/// Cost tracking for LLM usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    /// Total tokens used today
    pub tokens_today: u64,
    /// Estimated cost today (USD cents)
    pub cost_today_cents: u64,
    /// Date of current stats (YYYY-MM-DD)
    pub stats_date: String,
    /// Total tokens used all time
    pub tokens_total: u64,
    /// Total items re-ranked
    pub items_reranked: u64,
}

impl Default for UsageStats {
    fn default() -> Self {
        Self {
            tokens_today: 0,
            cost_today_cents: 0,
            stats_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            tokens_total: 0,
            items_reranked: 0,
        }
    }
}

/// Re-ranking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankConfig {
    /// Whether LLM re-ranking is enabled
    pub enabled: bool,
    /// Maximum items to send to LLM per analysis (cost control)
    pub max_items_per_batch: usize,
    /// Minimum embedding score to consider for re-ranking
    pub min_embedding_score: f32,
    /// Daily token limit (0 = unlimited)
    pub daily_token_limit: u64,
    /// Daily cost limit in cents (0 = unlimited)
    pub daily_cost_limit_cents: u64,
}

impl Default for RerankConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_items_per_batch: 15,
            min_embedding_score: 0.25,
            daily_token_limit: 100_000, // ~$0.10 for Haiku
            daily_cost_limit_cents: 50, // $0.50/day default limit
        }
    }
}

/// Monitoring configuration (persisted)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Whether continuous monitoring is enabled
    pub enabled: bool,
    /// Interval between checks in minutes
    pub interval_minutes: u64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_minutes: 30,
        }
    }
}

/// Main settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// LLM provider configuration
    pub llm: LLMProvider,
    /// Re-ranking configuration
    pub rerank: RerankConfig,
    /// Usage statistics
    pub usage: UsageStats,
    /// Context directories to watch
    pub context_dirs: Vec<String>,
    /// Embedding relevance threshold (Stage 1)
    pub embedding_threshold: f32,
    /// Monitoring configuration
    #[serde(default)]
    pub monitoring: MonitoringConfig,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            llm: LLMProvider::default(),
            rerank: RerankConfig::default(),
            usage: UsageStats::default(),
            context_dirs: vec![],
            embedding_threshold: 0.30,
            monitoring: MonitoringConfig::default(),
        }
    }
}

// ============================================================================
// Settings Manager
// ============================================================================

/// Manages loading, saving, and accessing settings
pub struct SettingsManager {
    settings: Settings,
    settings_path: PathBuf,
}

impl SettingsManager {
    /// Create a new settings manager, loading from disk if available
    pub fn new(data_dir: &PathBuf) -> Self {
        let settings_path = data_dir.join("settings.json");

        let settings = if settings_path.exists() {
            match fs::read_to_string(&settings_path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
                    println!("[4DA/Settings] Failed to parse settings: {}", e);
                    Settings::default()
                }),
                Err(e) => {
                    println!("[4DA/Settings] Failed to read settings: {}", e);
                    Settings::default()
                }
            }
        } else {
            println!("[4DA/Settings] No settings file found, using defaults");
            Settings::default()
        };

        Self {
            settings,
            settings_path,
        }
    }

    /// Save settings to disk
    pub fn save(&self) -> Result<(), String> {
        // Ensure parent directory exists
        if let Some(parent) = self.settings_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let json = serde_json::to_string_pretty(&self.settings).map_err(|e| e.to_string())?;

        fs::write(&self.settings_path, json).map_err(|e| e.to_string())?;
        println!("[4DA/Settings] Settings saved to {:?}", self.settings_path);
        Ok(())
    }

    /// Get current settings
    pub fn get(&self) -> &Settings {
        &self.settings
    }

    /// Get mutable settings
    pub fn get_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Update LLM provider settings
    pub fn set_llm_provider(&mut self, provider: LLMProvider) -> Result<(), String> {
        self.settings.llm = provider;
        self.save()
    }

    /// Update re-rank configuration
    pub fn set_rerank_config(&mut self, config: RerankConfig) -> Result<(), String> {
        self.settings.rerank = config;
        self.save()
    }

    /// Update monitoring configuration
    pub fn set_monitoring_config(&mut self, config: MonitoringConfig) -> Result<(), String> {
        self.settings.monitoring = config;
        self.save()
    }

    /// Get monitoring configuration
    pub fn get_monitoring_config(&self) -> &MonitoringConfig {
        &self.settings.monitoring
    }

    /// Check if LLM re-ranking is configured and enabled
    pub fn is_rerank_enabled(&self) -> bool {
        self.settings.rerank.enabled
            && !self.settings.llm.api_key.is_empty()
            && self.settings.llm.provider != "none"
    }

    /// Check if within daily limits
    pub fn within_daily_limits(&mut self) -> bool {
        // Reset stats if new day
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        if self.settings.usage.stats_date != today {
            self.settings.usage.tokens_today = 0;
            self.settings.usage.cost_today_cents = 0;
            self.settings.usage.stats_date = today;
            let _ = self.save();
        }

        let token_ok = self.settings.rerank.daily_token_limit == 0
            || self.settings.usage.tokens_today < self.settings.rerank.daily_token_limit;

        let cost_ok = self.settings.rerank.daily_cost_limit_cents == 0
            || self.settings.usage.cost_today_cents < self.settings.rerank.daily_cost_limit_cents;

        token_ok && cost_ok
    }

    /// Record token usage
    pub fn record_usage(&mut self, tokens: u64, cost_cents: u64) {
        self.settings.usage.tokens_today += tokens;
        self.settings.usage.cost_today_cents += cost_cents;
        self.settings.usage.tokens_total += tokens;
        self.settings.usage.items_reranked += 1;
        let _ = self.save();
    }

    /// Get usage summary
    pub fn usage_summary(&self) -> String {
        format!(
            "Today: {} tokens (~${:.3}) | Total: {} tokens | {} items re-ranked",
            self.settings.usage.tokens_today,
            self.settings.usage.cost_today_cents as f64 / 100.0,
            self.settings.usage.tokens_total,
            self.settings.usage.items_reranked
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert!(!settings.rerank.enabled);
        assert_eq!(settings.embedding_threshold, 0.30);
        assert_eq!(settings.rerank.max_items_per_batch, 15);
    }

    #[test]
    fn test_llm_provider_default() {
        let provider = LLMProvider::default();
        assert_eq!(provider.provider, "none");
        assert!(provider.api_key.is_empty());
    }
}
