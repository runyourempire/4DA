//! ACE - Autonomous Context Engine
//!
//! The brain of 4DA. Implements autonomous context detection with:
//! - Project manifest scanning (Cargo.toml, package.json, etc.)
//! - Real-time file watching for context updates
//! - Git history analysis
//! - Behavior learning from user interactions
//! - Confidence scoring on all signals
//! - Cross-validation between sources
//! - Graceful degradation
//!
//! ACE always hits its mark.

pub mod anomaly;
pub mod behavior;
pub mod confidence;
pub mod db;
pub mod embedding;
pub mod git;
pub mod health;
#[cfg(test)]
mod integration_tests;
pub mod scanner;
pub mod validation;
pub mod watcher;

use parking_lot::Mutex;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

pub use anomaly::{Anomaly, AnomalyConfig, AnomalyDetector, AnomalySeverity, AnomalyType};
pub use behavior::{
    ActivityPatterns, AntiTopic, BehaviorAction, BehaviorConfig, BehaviorLearner, BehaviorSignal,
    LearnedBehavior, SourcePreference, TopicAffinity,
};
pub use confidence::{ConfidenceScore, SignalConfidence};
pub use embedding::{EmbeddingConfig, EmbeddingProvider, EmbeddingService};
pub use git::{CommitInfo, GitAnalyzer, GitSignal};
pub use health::{
    AccuracyMetrics, AccuracyTracker, AlertSeverity, AlertType, AuditEntry, AuditEntryType,
    AuditLogger, ComponentStatus, ContextQuality, DailyMetrics, FallbackChain, FallbackLevel,
    FeedbackResult, FeedbackType, HealthAlert, HealthMonitor, HealthSnapshot, HealthStatus,
};
pub use scanner::{ManifestType, ProjectScanner, ProjectSignal};
pub use validation::{SignalValidator, ValidatedSignal, ValidationResult};
pub use watcher::{
    FileChange, FileChangeType, FileWatcher, InteractionRateLimiter, RateLimitStatus,
    WatcherConfig, WatcherStatePersistence,
};

// ============================================================================
// Core ACE Types
// ============================================================================

/// The Autonomous Context Engine
pub struct ACE {
    conn: Arc<Mutex<Connection>>,
    scanner: ProjectScanner,
    validator: SignalValidator,
    git_analyzer: GitAnalyzer,
    watcher: Option<Mutex<FileWatcher>>,
    watcher_persistence: Option<WatcherStatePersistence>,
    behavior_learner: BehaviorLearner,
    health_monitor: Mutex<HealthMonitor>,
    fallback_chain: Mutex<FallbackChain>,
    audit_logger: Mutex<AuditLogger>,
    accuracy_tracker: Mutex<AccuracyTracker>,
    anomaly_detector: Mutex<AnomalyDetector>,
    embedding_service: Option<Mutex<EmbeddingService>>,
    rate_limiter: InteractionRateLimiter,
}

/// A detected technology with confidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedTech {
    pub name: String,
    pub category: TechCategory,
    pub confidence: f32,
    pub source: DetectionSource,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TechCategory {
    Language,
    Framework,
    Library,
    Tool,
    Database,
    Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DetectionSource {
    Manifest,      // From package.json, Cargo.toml, etc.
    FileExtension, // From file type counts
    FileContent,   // From imports, code analysis
    GitHistory,    // From commit patterns
    UserExplicit,  // User declared
}

/// Active topic detected from current work
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTopic {
    pub topic: String,
    pub weight: f32,
    pub confidence: f32,
    pub source: TopicSource,
    pub last_seen: String,
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TopicSource {
    FileContent,
    GitCommit,
    ImportStatement,
    ProjectManifest,
    ActivityTracker,
}

/// Exclusion with strength levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exclusion {
    pub topic: String,
    pub strength: ExclusionStrength,
    pub source: ExclusionSource,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ExclusionStrength {
    Soft,     // Reduce score by 50%
    Hard,     // Reduce score by 90%
    Absolute, // Score = 0, never show
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExclusionSource {
    UserExplicit,
    LearnedHard,
    LearnedSoft,
}

impl ExclusionStrength {
    pub fn apply_to_score(&self, base_score: f32) -> f32 {
        match self {
            ExclusionStrength::Soft => base_score * 0.5,
            ExclusionStrength::Hard => base_score * 0.1,
            ExclusionStrength::Absolute => 0.0,
        }
    }
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub project_scanner: ComponentHealth,
    pub file_watcher: ComponentHealth,
    pub git_analyzer: ComponentHealth,
    pub behavior_learner: ComponentHealth,
    pub overall_status: HealthStatus,
    pub context_quality: ContextQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: HealthStatus,
    pub last_success: Option<String>,
    pub error_count: u32,
    pub last_error: Option<String>,
}

// HealthStatus and ContextQuality are imported from health.rs

// ============================================================================
// ACE Implementation
// ============================================================================

impl ACE {
    /// Create a new ACE instance
    pub fn new(conn: Arc<Mutex<Connection>>) -> Result<Self, String> {
        // Run migrations
        db::migrate(&conn)?;

        let scanner = ProjectScanner::new();
        let validator = SignalValidator::new();
        let git_analyzer = GitAnalyzer::default();
        let behavior_learner = BehaviorLearner::default();
        let health_monitor = HealthMonitor::new(conn.clone());
        let fallback_chain = FallbackChain::new();
        let audit_logger = AuditLogger::new(conn.clone());
        let accuracy_tracker = AccuracyTracker::new(conn.clone());
        let anomaly_detector = AnomalyDetector::new(conn.clone(), AnomalyConfig::default());
        let watcher_persistence = WatcherStatePersistence::new(conn.clone()).ok();

        // Initialize embedding service with mock provider by default
        // Can be configured later with real provider
        let embedding_config = EmbeddingConfig::default();
        let embedding_service = EmbeddingService::new(embedding_config, conn.clone());

        // Rate limiter: 1000 global, 100 per source, per minute
        let rate_limiter = InteractionRateLimiter::new(1000, 100, 60);

        // Initialize file watcher (empty, ready to add paths)
        let watcher_config = WatcherConfig::default();
        let watcher = FileWatcher::new(watcher_config);

        Ok(Self {
            conn,
            scanner,
            validator,
            git_analyzer,
            watcher: Some(Mutex::new(watcher)),
            watcher_persistence,
            behavior_learner,
            health_monitor: Mutex::new(health_monitor),
            fallback_chain: Mutex::new(fallback_chain),
            audit_logger: Mutex::new(audit_logger),
            accuracy_tracker: Mutex::new(accuracy_tracker),
            anomaly_detector: Mutex::new(anomaly_detector),
            embedding_service: Some(Mutex::new(embedding_service)),
            rate_limiter,
        })
    }

    /// Start file watching for real-time context updates
    pub fn start_watching(&mut self, paths: &[PathBuf]) -> Result<(), String> {
        let config = WatcherConfig::default();
        let mut watcher = FileWatcher::new(config);

        // Set up callback to process file changes
        let conn = self.conn.clone();
        watcher.set_callback(move |changes| {
            if let Err(e) = process_file_changes(&conn, &changes) {
                println!("[ACE/Watcher] Error processing changes: {}", e);
            }
        });

        // Watch each path
        for path in paths {
            if path.exists() {
                watcher.watch(path)?;
            }
        }

        self.watcher = Some(Mutex::new(watcher));
        println!("[ACE] File watching started for {} paths", paths.len());
        Ok(())
    }

    /// Stop file watching
    pub fn stop_watching(&mut self) {
        if let Some(ref watcher) = self.watcher {
            watcher.lock().stop();
        }
        self.watcher = None;
        println!("[ACE] File watching stopped");
    }

    /// Check if file watching is active
    pub fn is_watching(&self) -> bool {
        self.watcher
            .as_ref()
            .map_or(false, |w| w.lock().is_watching())
    }

    /// Analyze git repositories in the given paths
    pub fn analyze_git_repos(&self, paths: &[PathBuf]) -> Result<Vec<GitSignal>, String> {
        let mut signals = Vec::new();

        for path in paths {
            if !path.exists() {
                continue;
            }

            // Find git repos in this path
            let repos = self.git_analyzer.find_repos(path, 3);

            for repo_path in repos {
                match self.git_analyzer.analyze_repo(&repo_path) {
                    Ok(signal) => {
                        println!(
                            "[ACE/Git] Analyzed: {} ({} commits, confidence: {:.0}%)",
                            signal.repo_name,
                            signal.recent_commits.len(),
                            signal.confidence * 100.0
                        );
                        signals.push(signal);
                    }
                    Err(e) => {
                        println!("[ACE/Git] Failed to analyze {}: {}", repo_path.display(), e);
                    }
                }
            }
        }

        // Store git signals in database
        self.store_git_signals(&signals)?;

        Ok(signals)
    }

    /// Store git signals in database
    fn store_git_signals(&self, signals: &[GitSignal]) -> Result<(), String> {
        let conn = self.conn.lock();

        for signal in signals {
            // Store extracted topics as active topics
            for topic in &signal.extracted_topics {
                conn.execute(
                    "INSERT INTO active_topics (topic, weight, confidence, source, last_seen)
                     VALUES (?1, ?2, ?3, 'git_commit', datetime('now'))
                     ON CONFLICT(topic) DO UPDATE SET
                        weight = MAX(excluded.weight, active_topics.weight),
                        confidence = MAX(excluded.confidence, active_topics.confidence),
                        last_seen = datetime('now')",
                    rusqlite::params![topic, 0.7, signal.confidence],
                )
                .map_err(|e| format!("Failed to store git topic: {}", e))?;
            }

            // Store git signal record
            let topics_json = serde_json::to_string(&signal.extracted_topics).unwrap_or_default();
            conn.execute(
                "INSERT INTO git_signals (repo_path, commit_hash, commit_message, extracted_topics, timestamp)
                 VALUES (?1, ?2, ?3, ?4, datetime('now'))",
                rusqlite::params![
                    signal.repo_path.to_string_lossy(),
                    signal.last_commit.clone().unwrap_or_default(),
                    signal.recent_commits.first().map(|c| c.message.clone()).unwrap_or_default(),
                    topics_json
                ],
            ).map_err(|e| format!("Failed to store git signal: {}", e))?;
        }

        Ok(())
    }

    /// Perform autonomous context detection
    /// This is the main entry point for ACE - scans without requiring user input
    pub fn detect_context(&self, scan_paths: &[PathBuf]) -> Result<AutonomousContext, String> {
        println!("[ACE] Starting autonomous context detection...");

        let mut detected_tech: Vec<DetectedTech> = Vec::new();
        let mut active_topics: Vec<ActiveTopic> = Vec::new();
        let mut projects_found = 0;

        // Scan each path for project manifests
        for path in scan_paths {
            if !path.exists() {
                continue;
            }

            match self.scanner.scan_directory(path) {
                Ok(signals) => {
                    for signal in signals {
                        projects_found += 1;

                        // Validate signal
                        let validated = self.validator.validate_project_signal(&signal);

                        if validated.confidence >= 0.3 {
                            // Extract technologies
                            for lang in &signal.languages {
                                detected_tech.push(DetectedTech {
                                    name: lang.clone(),
                                    category: TechCategory::Language,
                                    confidence: validated.confidence,
                                    source: DetectionSource::Manifest,
                                    evidence: vec![format!(
                                        "Found in {}",
                                        signal.manifest_path.display()
                                    )],
                                });
                            }

                            for framework in &signal.frameworks {
                                detected_tech.push(DetectedTech {
                                    name: framework.clone(),
                                    category: TechCategory::Framework,
                                    confidence: validated.confidence * 0.9,
                                    source: DetectionSource::Manifest,
                                    evidence: vec![format!(
                                        "Dependency in {}",
                                        signal.manifest_path.display()
                                    )],
                                });
                            }

                            for dep in &signal.dependencies {
                                // Only include well-known dependencies
                                if is_notable_dependency(dep) {
                                    detected_tech.push(DetectedTech {
                                        name: dep.clone(),
                                        category: TechCategory::Library,
                                        confidence: validated.confidence * 0.7,
                                        source: DetectionSource::Manifest,
                                        evidence: vec![format!(
                                            "Dependency in {}",
                                            signal.manifest_path.display()
                                        )],
                                    });
                                }
                            }

                            // Create active topics from project
                            for lang in &signal.languages {
                                active_topics.push(ActiveTopic {
                                    topic: lang.clone(),
                                    weight: 0.8,
                                    confidence: validated.confidence,
                                    source: TopicSource::ProjectManifest,
                                    last_seen: chrono::Utc::now().to_rfc3339(),
                                    embedding: None,
                                });
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("[ACE] Warning: Failed to scan {}: {}", path.display(), e);
                }
            }
        }

        // Deduplicate and merge technologies
        let merged_tech = merge_detected_tech(detected_tech);

        // Compute overall confidence
        let context_confidence = if merged_tech.is_empty() {
            0.3 // Minimal confidence with no detections
        } else {
            let avg_confidence: f32 =
                merged_tech.iter().map(|t| t.confidence).sum::<f32>() / merged_tech.len() as f32;
            avg_confidence.min(0.95) // Cap at 0.95 for inferred
        };

        println!(
            "[ACE] Detected {} technologies from {} projects (confidence: {:.0}%)",
            merged_tech.len(),
            projects_found,
            context_confidence * 100.0
        );

        // Store results in database
        self.store_detected_context(&merged_tech, &active_topics)?;

        Ok(AutonomousContext {
            detected_tech: merged_tech,
            active_topics,
            projects_scanned: projects_found,
            context_confidence,
            detection_time: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Store detected context in database
    fn store_detected_context(
        &self,
        tech: &[DetectedTech],
        topics: &[ActiveTopic],
    ) -> Result<(), String> {
        let conn = self.conn.lock();

        // Store detected technologies
        for t in tech {
            let source_str = match t.source {
                DetectionSource::Manifest => "manifest",
                DetectionSource::FileExtension => "file_extension",
                DetectionSource::FileContent => "file_content",
                DetectionSource::GitHistory => "git_history",
                DetectionSource::UserExplicit => "explicit",
            };

            let category_str = match t.category {
                TechCategory::Language => "language",
                TechCategory::Framework => "framework",
                TechCategory::Library => "library",
                TechCategory::Tool => "tool",
                TechCategory::Database => "database",
                TechCategory::Platform => "platform",
            };

            conn.execute(
                "INSERT INTO detected_tech (name, category, confidence, source, evidence)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(name) DO UPDATE SET
                    confidence = MAX(excluded.confidence, detected_tech.confidence),
                    evidence = excluded.evidence,
                    updated_at = datetime('now')",
                rusqlite::params![
                    t.name,
                    category_str,
                    t.confidence,
                    source_str,
                    t.evidence.join("; ")
                ],
            )
            .map_err(|e| format!("Failed to store detected tech: {}", e))?;
        }

        // Store active topics
        for topic in topics {
            let source_str = match topic.source {
                TopicSource::FileContent => "file_content",
                TopicSource::GitCommit => "git_commit",
                TopicSource::ImportStatement => "import",
                TopicSource::ProjectManifest => "manifest",
                TopicSource::ActivityTracker => "activity",
            };

            conn.execute(
                "INSERT INTO active_topics (topic, weight, confidence, source, last_seen)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(topic) DO UPDATE SET
                    weight = MAX(excluded.weight, active_topics.weight),
                    confidence = MAX(excluded.confidence, active_topics.confidence),
                    last_seen = excluded.last_seen",
                rusqlite::params![
                    topic.topic,
                    topic.weight,
                    topic.confidence,
                    source_str,
                    topic.last_seen
                ],
            )
            .map_err(|e| format!("Failed to store active topic: {}", e))?;
        }

        Ok(())
    }

    /// Get all detected technologies
    pub fn get_detected_tech(&self) -> Result<Vec<DetectedTech>, String> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT name, category, confidence, source, evidence FROM detected_tech ORDER BY confidence DESC"
        ).map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                let category_str: String = row.get(1)?;
                let source_str: String = row.get(3)?;
                let evidence_str: String = row.get(4)?;

                Ok(DetectedTech {
                    name: row.get(0)?,
                    category: match category_str.as_str() {
                        "language" => TechCategory::Language,
                        "framework" => TechCategory::Framework,
                        "library" => TechCategory::Library,
                        "tool" => TechCategory::Tool,
                        "database" => TechCategory::Database,
                        _ => TechCategory::Platform,
                    },
                    confidence: row.get(2)?,
                    source: match source_str.as_str() {
                        "manifest" => DetectionSource::Manifest,
                        "file_extension" => DetectionSource::FileExtension,
                        "file_content" => DetectionSource::FileContent,
                        "git_history" => DetectionSource::GitHistory,
                        _ => DetectionSource::UserExplicit,
                    },
                    evidence: evidence_str.split("; ").map(String::from).collect(),
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    }

    /// Get active topics
    pub fn get_active_topics(&self) -> Result<Vec<ActiveTopic>, String> {
        let conn = self.conn.lock();
        let mut stmt = conn
            .prepare(
                "SELECT topic, weight, confidence, source, last_seen FROM active_topics
             WHERE last_seen > datetime('now', '-7 days')
             ORDER BY weight DESC",
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                let source_str: String = row.get(3)?;

                Ok(ActiveTopic {
                    topic: row.get(0)?,
                    weight: row.get(1)?,
                    confidence: row.get(2)?,
                    source: match source_str.as_str() {
                        "file_content" => TopicSource::FileContent,
                        "git_commit" => TopicSource::GitCommit,
                        "import" => TopicSource::ImportStatement,
                        "manifest" => TopicSource::ProjectManifest,
                        _ => TopicSource::ActivityTracker,
                    },
                    last_seen: row.get(4)?,
                    embedding: None,
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    }

    /// Log an audit entry
    pub fn log_audit(&self, entry: &AuditEntry) -> Result<(), String> {
        let conn = self.conn.lock();
        let entry_type_str = match entry.entry_type {
            AuditEntryType::ContextUpdate => "context_update",
            AuditEntryType::RelevanceDecision => "relevance_decision",
            AuditEntryType::ExclusionApplied => "exclusion_applied",
            AuditEntryType::FeedbackReceived => "feedback_received",
            AuditEntryType::AnomalyDetected => "anomaly_detected",
            AuditEntryType::FallbackActivated => "fallback_activated",
            AuditEntryType::HealthCheck => "health_check",
            AuditEntryType::ConfigChange => "config_change",
        };

        conn.execute(
            "INSERT INTO audit_log (entry_type, action, reason, confidence) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![entry_type_str, entry.action, entry.reason, entry.confidence],
        ).map_err(|e| format!("Failed to log audit: {}", e))?;

        Ok(())
    }

    /// Get system health
    pub fn get_health(&self) -> SystemHealth {
        // For now, return healthy status - will be enhanced with real checks
        SystemHealth {
            project_scanner: ComponentHealth {
                status: HealthStatus::Healthy,
                last_success: Some(chrono::Utc::now().to_rfc3339()),
                error_count: 0,
                last_error: None,
            },
            file_watcher: ComponentHealth {
                status: HealthStatus::Disabled, // Not implemented yet
                last_success: None,
                error_count: 0,
                last_error: None,
            },
            git_analyzer: ComponentHealth {
                status: HealthStatus::Disabled, // Not implemented yet
                last_success: None,
                error_count: 0,
                last_error: None,
            },
            behavior_learner: ComponentHealth {
                status: HealthStatus::Healthy,
                last_success: Some(chrono::Utc::now().to_rfc3339()),
                error_count: 0,
                last_error: None,
            },
            overall_status: HealthStatus::Healthy,
            context_quality: ContextQuality::Good,
        }
    }

    // ========================================================================
    // Behavior Learning Methods (Phase C)
    // ========================================================================

    /// Record a user interaction (click, save, dismiss, etc.)
    /// Returns Err if rate limited
    pub fn record_interaction(
        &self,
        item_id: i64,
        action: BehaviorAction,
        item_topics: Vec<String>,
        item_source: String,
    ) -> Result<(), String> {
        // Check rate limit
        if !self.rate_limiter.check(&item_source) {
            return Err("Rate limited: too many interactions".to_string());
        }

        let signal = BehaviorLearner::create_signal(
            item_id,
            action.clone(),
            item_topics.clone(),
            item_source.clone(),
        );

        // Store in database
        self.store_interaction(&signal)?;

        // Update topic affinities
        self.update_topic_affinities(&signal)?;

        // Check for anti-topic updates
        if signal.signal_strength < -0.5 {
            self.update_anti_topics(&item_topics, signal.signal_strength)?;
        }

        // Update source preferences
        self.update_source_preference(&item_source, signal.signal_strength)?;

        // Update activity patterns
        self.update_activity_patterns(signal.signal_strength)?;

        println!(
            "[ACE/Behavior] Recorded: {:?} on item {} (strength: {:.2})",
            action, item_id, signal.signal_strength
        );

        Ok(())
    }

    /// Get rate limit status for a source
    pub fn get_rate_limit_status(&self, source: &str) -> RateLimitStatus {
        self.rate_limiter.status(source)
    }

    /// Store an interaction in the database
    fn store_interaction(&self, signal: &BehaviorSignal) -> Result<(), String> {
        let conn = self.conn.lock();

        let action_type = match &signal.action {
            BehaviorAction::Click { .. } => "click",
            BehaviorAction::Save => "save",
            BehaviorAction::Share => "share",
            BehaviorAction::Dismiss => "dismiss",
            BehaviorAction::MarkIrrelevant => "mark_irrelevant",
            BehaviorAction::Scroll { .. } => "scroll",
            BehaviorAction::Ignore => "ignore",
        };

        let action_data = serde_json::to_string(&signal.action).unwrap_or_default();
        let topics_json = serde_json::to_string(&signal.item_topics).unwrap_or_default();

        conn.execute(
            "INSERT INTO interactions (item_id, action_type, action_data, item_topics, item_source, signal_strength)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                signal.item_id,
                action_type,
                action_data,
                topics_json,
                signal.item_source,
                signal.signal_strength
            ],
        ).map_err(|e| format!("Failed to store interaction: {}", e))?;

        Ok(())
    }

    /// Update topic affinities based on interaction
    fn update_topic_affinities(&self, signal: &BehaviorSignal) -> Result<(), String> {
        let conn = self.conn.lock();

        for topic in &signal.item_topics {
            // Upsert topic affinity
            if signal.signal_strength > 0.0 {
                conn.execute(
                    "INSERT INTO topic_affinities (topic, positive_signals, total_exposures, last_interaction)
                     VALUES (?1, 1, 1, datetime('now'))
                     ON CONFLICT(topic) DO UPDATE SET
                        positive_signals = topic_affinities.positive_signals + 1,
                        total_exposures = topic_affinities.total_exposures + 1,
                        last_interaction = datetime('now'),
                        decay_applied = 0,
                        updated_at = datetime('now')",
                    rusqlite::params![topic],
                )
            } else if signal.signal_strength < 0.0 {
                conn.execute(
                    "INSERT INTO topic_affinities (topic, negative_signals, total_exposures, last_interaction)
                     VALUES (?1, 1, 1, datetime('now'))
                     ON CONFLICT(topic) DO UPDATE SET
                        negative_signals = topic_affinities.negative_signals + 1,
                        total_exposures = topic_affinities.total_exposures + 1,
                        last_interaction = datetime('now'),
                        decay_applied = 0,
                        updated_at = datetime('now')",
                    rusqlite::params![topic],
                )
            } else {
                conn.execute(
                    "INSERT INTO topic_affinities (topic, total_exposures, last_interaction)
                     VALUES (?1, 1, datetime('now'))
                     ON CONFLICT(topic) DO UPDATE SET
                        total_exposures = topic_affinities.total_exposures + 1,
                        last_interaction = datetime('now'),
                        updated_at = datetime('now')",
                    rusqlite::params![topic],
                )
            }.map_err(|e| format!("Failed to update topic affinity: {}", e))?;

            // Recompute affinity score
            conn.execute(
                "UPDATE topic_affinities SET
                    affinity_score = CASE
                        WHEN total_exposures >= 5 THEN
                            (CAST(positive_signals AS REAL) - CAST(negative_signals AS REAL)) / CAST(total_exposures AS REAL)
                            * MIN(CAST(total_exposures AS REAL) / 20.0, 1.0)
                        ELSE 0.0
                    END,
                    confidence = MIN(CAST(total_exposures AS REAL) / 20.0, 1.0)
                 WHERE topic = ?1",
                rusqlite::params![topic],
            ).map_err(|e| format!("Failed to recompute affinity: {}", e))?;
        }

        Ok(())
    }

    /// Update anti-topics based on rejections
    fn update_anti_topics(&self, topics: &[String], signal_strength: f32) -> Result<(), String> {
        if signal_strength >= -0.5 {
            return Ok(()); // Not a strong rejection
        }

        let conn = self.conn.lock();

        for topic in topics {
            conn.execute(
                "INSERT INTO anti_topics (topic, rejection_count, confidence, last_rejection)
                 VALUES (?1, 1, 0.2, datetime('now'))
                 ON CONFLICT(topic) DO UPDATE SET
                    rejection_count = anti_topics.rejection_count + 1,
                    confidence = MIN(CAST(anti_topics.rejection_count + 1 AS REAL) / 10.0, 0.9),
                    last_rejection = datetime('now'),
                    updated_at = datetime('now')",
                rusqlite::params![topic],
            )
            .map_err(|e| format!("Failed to update anti-topic: {}", e))?;
        }

        Ok(())
    }

    /// Update source preferences
    fn update_source_preference(&self, source: &str, signal_strength: f32) -> Result<(), String> {
        let conn = self.conn.lock();
        let alpha = 0.1; // Learning rate

        conn.execute(
            "INSERT INTO source_preferences (source, score, interactions, last_interaction)
             VALUES (?1, ?2, 1, datetime('now'))
             ON CONFLICT(source) DO UPDATE SET
                score = source_preferences.score * (1.0 - ?3) + ?2 * ?3,
                interactions = source_preferences.interactions + 1,
                last_interaction = datetime('now'),
                updated_at = datetime('now')",
            rusqlite::params![source, signal_strength, alpha],
        )
        .map_err(|e| format!("Failed to update source preference: {}", e))?;

        Ok(())
    }

    /// Update activity patterns
    fn update_activity_patterns(&self, signal_strength: f32) -> Result<(), String> {
        let conn = self.conn.lock();
        let now = chrono::Utc::now();
        let hour = now.format("%H").to_string().parse::<usize>().unwrap_or(0);
        let day = now.format("%w").to_string().parse::<usize>().unwrap_or(0);

        // Get current patterns
        let (hourly, daily, total): (String, String, u32) = conn.query_row(
            "SELECT hourly_engagement, daily_engagement, total_tracked FROM activity_patterns WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ).unwrap_or_else(|_| (
            "[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]".to_string(),
            "[0,0,0,0,0,0,0]".to_string(),
            0
        ));

        // Parse and update
        let mut hourly_arr: Vec<f32> =
            serde_json::from_str(&hourly).unwrap_or_else(|_| vec![0.0; 24]);
        let mut daily_arr: Vec<f32> = serde_json::from_str(&daily).unwrap_or_else(|_| vec![0.0; 7]);

        let alpha = 0.05;
        if hour < 24 {
            hourly_arr[hour] = hourly_arr[hour] * (1.0 - alpha) + signal_strength * alpha;
        }
        if day < 7 {
            daily_arr[day] = daily_arr[day] * (1.0 - alpha) + signal_strength * alpha;
        }

        let hourly_json = serde_json::to_string(&hourly_arr).unwrap_or_default();
        let daily_json = serde_json::to_string(&daily_arr).unwrap_or_default();

        conn.execute(
            "UPDATE activity_patterns SET
                hourly_engagement = ?1,
                daily_engagement = ?2,
                total_tracked = ?3,
                updated_at = datetime('now')
             WHERE id = 1",
            rusqlite::params![hourly_json, daily_json, total + 1],
        )
        .map_err(|e| format!("Failed to update activity patterns: {}", e))?;

        Ok(())
    }

    /// Get topic affinities from the database
    pub fn get_topic_affinities(&self) -> Result<Vec<TopicAffinity>, String> {
        let conn = self.conn.lock();
        let mut stmt = conn
            .prepare(
                "SELECT topic, positive_signals, negative_signals, total_exposures,
                    affinity_score, confidence, last_interaction
             FROM topic_affinities
             WHERE total_exposures >= 5
             ORDER BY ABS(affinity_score) DESC
             LIMIT 100",
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                Ok(TopicAffinity {
                    topic: row.get(0)?,
                    embedding: None,
                    positive_signals: row.get(1)?,
                    negative_signals: row.get(2)?,
                    total_exposures: row.get(3)?,
                    affinity_score: row.get(4)?,
                    confidence: row.get(5)?,
                    last_interaction: row.get(6)?,
                    decay_applied: false,
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    }

    /// Get anti-topics that meet the threshold
    pub fn get_anti_topics(&self, min_rejections: u32) -> Result<Vec<AntiTopic>, String> {
        let conn = self.conn.lock();
        let mut stmt = conn
            .prepare(
                "SELECT topic, rejection_count, confidence, auto_detected, user_confirmed,
                    first_rejection, last_rejection
             FROM anti_topics
             WHERE rejection_count >= ?1
             ORDER BY rejection_count DESC",
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([min_rejections], |row| {
                Ok(AntiTopic {
                    topic: row.get(0)?,
                    rejection_count: row.get(1)?,
                    confidence: row.get(2)?,
                    auto_detected: row.get::<_, i32>(3)? != 0,
                    user_confirmed: row.get::<_, i32>(4)? != 0,
                    first_rejection: row.get(5)?,
                    last_rejection: row.get(6)?,
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    }

    /// Get behavior modifier for an item based on learned preferences
    pub fn get_behavior_modifier(&self, topics: &[String], source: &str) -> Result<f32, String> {
        let conn = self.conn.lock();
        let mut modifier = 0.0;
        let mut count = 0;

        // Get topic affinities
        for topic in topics {
            let result: Result<(f32, f32), _> = conn.query_row(
                "SELECT affinity_score, confidence FROM topic_affinities WHERE topic = ?1",
                [topic],
                |row| Ok((row.get(0)?, row.get(1)?)),
            );
            if let Ok((score, confidence)) = result {
                modifier += score * confidence;
                count += 1;
            }
        }

        // Average topic contribution
        if count > 0 {
            modifier /= count as f32;
        }

        // Add source preference (weighted less)
        let source_score: f32 = conn
            .query_row(
                "SELECT score FROM source_preferences WHERE source = ?1",
                [source],
                |row| row.get(0),
            )
            .unwrap_or(0.0);

        modifier += source_score * 0.3;

        Ok(modifier.clamp(-1.0, 1.0))
    }

    /// Get learned behavior summary
    pub fn get_learned_behavior(&self) -> Result<LearnedBehaviorSummary, String> {
        let affinities = self.get_topic_affinities()?;
        let anti_topics = self.get_anti_topics(5)?;

        let conn = self.conn.lock();

        // Get total interactions
        let total_interactions: u32 = conn
            .query_row("SELECT COUNT(*) FROM interactions", [], |row| row.get(0))
            .unwrap_or(0);

        // Get source preferences
        let mut stmt = conn
            .prepare(
                "SELECT source, score, interactions FROM source_preferences ORDER BY score DESC",
            )
            .map_err(|e| e.to_string())?;

        let source_prefs: Vec<SourcePreferenceSummary> = stmt
            .query_map([], |row| {
                Ok(SourcePreferenceSummary {
                    source: row.get(0)?,
                    score: row.get(1)?,
                    interactions: row.get(2)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        // Compute learning confidence
        let learning_confidence = (total_interactions as f32 / 100.0).min(0.95);

        // Get interests (strong positive affinities)
        let interests: Vec<String> = affinities
            .iter()
            .filter(|a| a.affinity_score > 0.3 && a.confidence > 0.5)
            .map(|a| a.topic.clone())
            .collect();

        Ok(LearnedBehaviorSummary {
            total_interactions,
            learning_confidence,
            interests,
            anti_topics: anti_topics.iter().map(|a| a.topic.clone()).collect(),
            source_preferences: source_prefs,
            top_affinities: affinities.into_iter().take(10).collect(),
        })
    }

    /// Confirm an anti-topic (user explicitly agrees)
    pub fn confirm_anti_topic(&self, topic: &str) -> Result<(), String> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE anti_topics SET user_confirmed = 1, confidence = 1.0, updated_at = datetime('now')
             WHERE topic = ?1",
            [topic],
        ).map_err(|e| format!("Failed to confirm anti-topic: {}", e))?;
        Ok(())
    }

    /// Apply temporal decay to topic affinities
    pub fn apply_behavior_decay(&self) -> Result<usize, String> {
        let conn = self.conn.lock();

        // Apply decay based on last_interaction timestamp
        // Half-life of 30 days
        let updated = conn.execute(
            "UPDATE topic_affinities SET
                affinity_score = affinity_score * POWER(0.5, (julianday('now') - julianday(last_interaction)) / 30.0),
                confidence = confidence * POWER(0.5, (julianday('now') - julianday(last_interaction)) / 30.0),
                decay_applied = 1
             WHERE decay_applied = 0
               AND julianday('now') - julianday(last_interaction) > 1",
            [],
        ).map_err(|e| format!("Failed to apply behavior decay: {}", e))?;

        if updated > 0 {
            println!(
                "[ACE/Behavior] Applied decay to {} topic affinities",
                updated
            );
        }

        Ok(updated)
    }

    // ========================================================================
    // Phase D: Health Monitoring & Validation
    // ========================================================================

    /// Perform a complete health check
    pub fn check_health(&self) -> HealthSnapshot {
        let mut monitor = self.health_monitor.lock();
        let snapshot = monitor.check_health();

        // Update fallback chain based on health
        let mut chain = self.fallback_chain.lock();
        chain.update_from_health(&snapshot);

        // Log to audit trail
        let mut logger = self.audit_logger.lock();
        let _ = logger.log(AuditEntry {
            id: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            entry_type: AuditEntryType::HealthCheck,
            action: format!("Health check: {:?}", snapshot.overall_status),
            reason: format!("Quality: {:?}", snapshot.context_quality),
            contributing_factors: snapshot.components.iter().map(|c| c.name.clone()).collect(),
            before_state: None,
            after_state: Some(format!("{:?}", snapshot.overall_status)),
            confidence: 1.0,
        });

        snapshot
    }

    /// Get current fallback level
    pub fn get_fallback_level(&self) -> FallbackLevel {
        self.fallback_chain.lock().current_level()
    }

    /// Get available features at current fallback level
    pub fn get_available_features(&self) -> Vec<String> {
        self.fallback_chain.lock().available_features()
    }

    /// Get recent health alerts
    pub fn get_health_alerts(&self) -> Vec<HealthAlert> {
        let monitor = self.health_monitor.lock();
        let accuracy = self.accuracy_tracker.lock();

        let mut alerts = monitor.get_recent_alerts();
        alerts.extend(accuracy.check_alerts());
        alerts
    }

    /// Log a context update to audit trail
    pub fn audit_context_update(
        &self,
        action: &str,
        topics: &[String],
        confidence: f32,
    ) -> Result<i64, String> {
        let mut logger = self.audit_logger.lock();
        logger.log_context_update(action, topics, confidence)
    }

    /// Log a relevance decision to audit trail
    pub fn audit_relevance_decision(
        &self,
        item_id: i64,
        score: f32,
        matches: &[String],
        confidence: f32,
    ) -> Result<i64, String> {
        let mut logger = self.audit_logger.lock();
        logger.log_relevance_decision(item_id, score, matches, confidence)
    }

    /// Log an anomaly detection
    pub fn audit_anomaly(
        &self,
        anomaly_type: &str,
        topic: &str,
        confidence: f32,
    ) -> Result<i64, String> {
        let mut logger = self.audit_logger.lock();
        logger.log_anomaly(anomaly_type, topic, confidence)
    }

    /// Get recent audit entries
    pub fn get_audit_entries(&self, limit: usize) -> Vec<AuditEntry> {
        self.audit_logger.lock().get_recent(limit)
    }

    /// Query audit log by type
    pub fn query_audit_log(
        &self,
        entry_type: Option<AuditEntryType>,
        limit: usize,
    ) -> Result<Vec<AuditEntry>, String> {
        self.audit_logger.lock().query(entry_type, limit)
    }

    /// Explain a relevance decision for an item
    pub fn explain_decision(&self, item_id: i64) -> Result<Option<String>, String> {
        self.audit_logger.lock().explain_decision(item_id)
    }

    /// Record feedback for accuracy tracking
    pub fn record_accuracy_feedback(&self, result: FeedbackResult) {
        let mut tracker = self.accuracy_tracker.lock();
        tracker.record_feedback(result);
    }

    /// Get current accuracy metrics
    pub fn get_accuracy_metrics(&self) -> AccuracyMetrics {
        self.accuracy_tracker.lock().compute_metrics()
    }

    /// Get accuracy history
    pub fn get_accuracy_history(&self, days: u32) -> Result<Vec<DailyMetrics>, String> {
        self.accuracy_tracker.lock().get_history(days)
    }

    /// Persist accuracy metrics to database
    pub fn persist_accuracy_metrics(&self) -> Result<(), String> {
        self.accuracy_tracker.lock().persist_metrics()
    }

    /// Check if accuracy targets are being met
    pub fn meets_accuracy_targets(&self) -> bool {
        self.accuracy_tracker.lock().compute_metrics().meets_targets
    }

    /// Get comprehensive system status
    pub fn get_system_status(&self) -> SystemStatus {
        let health = self.check_health();
        let fallback_level = self.get_fallback_level();
        let accuracy = self.get_accuracy_metrics();
        let alerts = self.get_health_alerts();

        SystemStatus {
            health_snapshot: health,
            fallback_level,
            accuracy_metrics: accuracy,
            active_alerts: alerts,
            available_features: self.get_available_features(),
        }
    }

    // ========================================================================
    // Anomaly Detection Methods
    // ========================================================================

    /// Run all anomaly detection checks
    pub fn detect_anomalies(&self) -> Vec<Anomaly> {
        let mut detector = self.anomaly_detector.lock();
        detector.detect_all()
    }

    /// Get unresolved anomalies
    pub fn get_unresolved_anomalies(&self) -> Result<Vec<Anomaly>, String> {
        self.anomaly_detector.lock().get_unresolved()
    }

    /// Resolve an anomaly by ID
    pub fn resolve_anomaly(&self, anomaly_id: i64) -> Result<(), String> {
        self.anomaly_detector.lock().resolve(anomaly_id)
    }

    /// Get recent anomalies
    pub fn get_recent_anomalies(&self, limit: usize) -> Vec<Anomaly> {
        self.anomaly_detector.lock().get_recent(limit)
    }

    // ========================================================================
    // Embedding Methods
    // ========================================================================

    /// Generate embedding for a topic
    pub fn embed_topic(&self, topic: &str) -> Result<Vec<f32>, String> {
        match &self.embedding_service {
            Some(service) => service.lock().embed(topic),
            None => Err("Embedding service not initialized".to_string()),
        }
    }

    /// Find similar topics to a query
    pub fn find_similar_topics(
        &self,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<(String, f32)>, String> {
        let topics = self.get_active_topics()?;
        let topic_strings: Vec<String> = topics.iter().map(|t| t.topic.clone()).collect();

        match &self.embedding_service {
            Some(service) => service.lock().find_similar(query, &topic_strings, top_k),
            None => Err("Embedding service not initialized".to_string()),
        }
    }

    /// Configure embedding provider
    pub fn configure_embedding(&mut self, config: EmbeddingConfig) {
        let service = EmbeddingService::new(config, self.conn.clone());
        self.embedding_service = Some(Mutex::new(service));
    }

    /// Check if embedding service is operational
    pub fn is_embedding_operational(&self) -> bool {
        self.embedding_service
            .as_ref()
            .map(|s| s.lock().is_operational())
            .unwrap_or(false)
    }

    // ========================================================================
    // Watcher Persistence Methods
    // ========================================================================

    /// Save watcher state for restart recovery
    pub fn save_watcher_state(&self) -> Result<(), String> {
        if let (Some(persistence), Some(watcher)) = (&self.watcher_persistence, &self.watcher) {
            let watcher_guard = watcher.lock();
            persistence.save(&watcher_guard)
        } else {
            Err("Watcher or persistence not initialized".to_string())
        }
    }

    /// Restore watcher from saved state
    pub fn restore_watcher_state(&mut self) -> Result<usize, String> {
        if let Some(persistence) = &self.watcher_persistence {
            if self.watcher.is_none() {
                let config = WatcherConfig::default();
                self.watcher = Some(Mutex::new(FileWatcher::new(config)));
            }

            if let Some(watcher) = &self.watcher {
                let mut watcher_guard = watcher.lock();
                persistence.restore(&mut watcher_guard)
            } else {
                Err("Failed to create watcher".to_string())
            }
        } else {
            Err("Watcher persistence not initialized".to_string())
        }
    }

    /// Clear saved watcher state
    pub fn clear_watcher_state(&self) -> Result<(), String> {
        if let Some(persistence) = &self.watcher_persistence {
            persistence.clear()
        } else {
            Err("Watcher persistence not initialized".to_string())
        }
    }
}

/// Comprehensive system status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub health_snapshot: HealthSnapshot,
    pub fallback_level: FallbackLevel,
    pub accuracy_metrics: AccuracyMetrics,
    pub active_alerts: Vec<HealthAlert>,
    pub available_features: Vec<String>,
}

/// Summary of learned behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedBehaviorSummary {
    pub total_interactions: u32,
    pub learning_confidence: f32,
    pub interests: Vec<String>,
    pub anti_topics: Vec<String>,
    pub source_preferences: Vec<SourcePreferenceSummary>,
    pub top_affinities: Vec<TopicAffinity>,
}

/// Source preference summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePreferenceSummary {
    pub source: String,
    pub score: f32,
    pub interactions: u32,
}

/// Result of autonomous context detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousContext {
    pub detected_tech: Vec<DetectedTech>,
    pub active_topics: Vec<ActiveTopic>,
    pub projects_scanned: usize,
    pub context_confidence: f32,
    pub detection_time: String,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Check if a dependency is notable enough to include
fn is_notable_dependency(name: &str) -> bool {
    let notable = [
        // Rust
        "tokio",
        "async-std",
        "serde",
        "reqwest",
        "sqlx",
        "diesel",
        "actix",
        "axum",
        "rocket",
        "tauri",
        "warp",
        "hyper",
        "tonic",
        "prost",
        // JavaScript/TypeScript
        "react",
        "vue",
        "angular",
        "svelte",
        "next",
        "nuxt",
        "express",
        "fastify",
        "nest",
        "prisma",
        "drizzle",
        "typeorm",
        "mongoose",
        "vite",
        "webpack",
        "esbuild",
        // Python
        "django",
        "flask",
        "fastapi",
        "numpy",
        "pandas",
        "tensorflow",
        "pytorch",
        "scikit-learn",
        "sqlalchemy",
        "celery",
        "redis",
        // Go
        "gin",
        "echo",
        "fiber",
        "gorm",
        "cobra",
        "viper",
        // Databases
        "postgresql",
        "mysql",
        "sqlite",
        "mongodb",
        "redis",
        "elasticsearch",
    ];

    notable.iter().any(|n| name.to_lowercase().contains(n))
}

/// Merge duplicate detected technologies, keeping highest confidence
fn merge_detected_tech(tech: Vec<DetectedTech>) -> Vec<DetectedTech> {
    let mut map: HashMap<String, DetectedTech> = HashMap::new();

    for t in tech {
        let key = t.name.to_lowercase();
        if let Some(existing) = map.get_mut(&key) {
            if t.confidence > existing.confidence {
                existing.confidence = t.confidence;
            }
            existing.evidence.extend(t.evidence);
        } else {
            map.insert(key, t);
        }
    }

    let mut result: Vec<_> = map.into_values().collect();
    result.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    result
}

/// Process file changes from the watcher
fn process_file_changes(
    conn: &Arc<Mutex<Connection>>,
    changes: &[FileChange],
) -> Result<(), String> {
    let conn = conn.lock();

    for change in changes {
        let change_type_str = match change.change_type {
            FileChangeType::Created => "created",
            FileChangeType::Modified => "modified",
            FileChangeType::Deleted => "deleted",
        };

        // Extract topics from file content (if not deleted)
        let topics = if change.change_type != FileChangeType::Deleted {
            watcher::extract_topics_from_file(&change.path).unwrap_or_default()
        } else {
            Vec::new()
        };

        let topics_json = serde_json::to_string(&topics).unwrap_or_default();

        // Store file signal
        conn.execute(
            "INSERT INTO file_signals (path, change_type, extracted_topics, timestamp)
             VALUES (?1, ?2, ?3, datetime('now'))",
            rusqlite::params![change.path.to_string_lossy(), change_type_str, topics_json],
        )
        .map_err(|e| format!("Failed to store file signal: {}", e))?;

        // Update active topics from file content
        for topic in &topics {
            conn.execute(
                "INSERT INTO active_topics (topic, weight, confidence, source, last_seen)
                 VALUES (?1, 0.6, 0.7, 'file_content', datetime('now'))
                 ON CONFLICT(topic) DO UPDATE SET
                    weight = MAX(excluded.weight, active_topics.weight),
                    last_seen = datetime('now')",
                rusqlite::params![topic],
            )
            .map_err(|e| format!("Failed to update active topic: {}", e))?;
        }

        println!(
            "[ACE/Watcher] Processed: {} ({}, {} topics)",
            change.path.display(),
            change_type_str,
            topics.len()
        );
    }

    Ok(())
}

/// Apply freshness decay to active topics
/// Topics decay over time - half-life of 7 days
pub fn apply_freshness_decay(conn: &Arc<Mutex<Connection>>) -> Result<usize, String> {
    let conn = conn.lock();

    // Apply decay based on last_seen timestamp
    // weight = weight * 0.5^(days_since_seen / 7)
    let updated = conn
        .execute(
            "UPDATE active_topics SET
            weight = weight * (0.5 * (1.0 + (julianday('now') - julianday(last_seen)) / 7.0)),
            decay_applied = 1
         WHERE decay_applied = 0
           AND julianday('now') - julianday(last_seen) > 1",
            [],
        )
        .map_err(|e| format!("Failed to apply decay: {}", e))?;

    // Remove topics with very low weight
    let removed = conn
        .execute("DELETE FROM active_topics WHERE weight < 0.1", [])
        .map_err(|e| format!("Failed to clean up topics: {}", e))?;

    if updated > 0 || removed > 0 {
        println!(
            "[ACE] Applied freshness decay: {} updated, {} removed",
            updated, removed
        );
    }

    Ok(updated)
}

/// Get real-time context for relevance scoring
/// Combines active topics from files, git, and manifests
pub fn get_realtime_context(conn: &Arc<Mutex<Connection>>) -> Result<RealtimeContext, String> {
    let conn = conn.lock();

    // Get active topics (recent, with decay applied)
    let mut stmt = conn
        .prepare(
            "SELECT topic, weight, confidence, source, last_seen
         FROM active_topics
         WHERE julianday('now') - julianday(last_seen) <= 7
         ORDER BY weight DESC
         LIMIT 50",
        )
        .map_err(|e| e.to_string())?;

    let topics: Vec<ActiveTopic> = stmt
        .query_map([], |row| {
            Ok(ActiveTopic {
                topic: row.get(0)?,
                weight: row.get(1)?,
                confidence: row.get(2)?,
                source: match row.get::<_, String>(3)?.as_str() {
                    "file_content" => TopicSource::FileContent,
                    "git_commit" => TopicSource::GitCommit,
                    "import" => TopicSource::ImportStatement,
                    "manifest" => TopicSource::ProjectManifest,
                    _ => TopicSource::ActivityTracker,
                },
                last_seen: row.get(4)?,
                embedding: None,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // Get detected technologies
    let mut stmt = conn
        .prepare(
            "SELECT name, category, confidence, source
         FROM detected_tech
         ORDER BY confidence DESC
         LIMIT 20",
        )
        .map_err(|e| e.to_string())?;

    let tech: Vec<DetectedTech> = stmt
        .query_map([], |row| {
            let category_str: String = row.get(1)?;
            let source_str: String = row.get(3)?;

            Ok(DetectedTech {
                name: row.get(0)?,
                category: match category_str.as_str() {
                    "language" => TechCategory::Language,
                    "framework" => TechCategory::Framework,
                    "library" => TechCategory::Library,
                    "tool" => TechCategory::Tool,
                    "database" => TechCategory::Database,
                    _ => TechCategory::Platform,
                },
                confidence: row.get(2)?,
                source: match source_str.as_str() {
                    "manifest" => DetectionSource::Manifest,
                    "file_extension" => DetectionSource::FileExtension,
                    "file_content" => DetectionSource::FileContent,
                    "git_history" => DetectionSource::GitHistory,
                    _ => DetectionSource::UserExplicit,
                },
                evidence: Vec::new(),
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // Calculate overall context confidence
    let context_confidence = if topics.is_empty() && tech.is_empty() {
        0.3 // Minimal
    } else {
        let topic_conf = if topics.is_empty() {
            0.0
        } else {
            topics.iter().map(|t| t.confidence).sum::<f32>() / topics.len() as f32
        };
        let tech_conf = if tech.is_empty() {
            0.0
        } else {
            tech.iter().map(|t| t.confidence).sum::<f32>() / tech.len() as f32
        };
        (topic_conf * 0.5 + tech_conf * 0.5).min(0.95)
    };

    Ok(RealtimeContext {
        active_topics: topics,
        detected_tech: tech,
        context_confidence,
        last_updated: chrono::Utc::now().to_rfc3339(),
    })
}

/// Real-time context data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeContext {
    pub active_topics: Vec<ActiveTopic>,
    pub detected_tech: Vec<DetectedTech>,
    pub context_confidence: f32,
    pub last_updated: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exclusion_strength() {
        assert_eq!(ExclusionStrength::Soft.apply_to_score(1.0), 0.5);
        assert_eq!(ExclusionStrength::Hard.apply_to_score(1.0), 0.1);
        assert_eq!(ExclusionStrength::Absolute.apply_to_score(1.0), 0.0);
    }

    #[test]
    fn test_notable_dependency() {
        assert!(is_notable_dependency("react"));
        assert!(is_notable_dependency("tokio"));
        assert!(is_notable_dependency("@prisma/client"));
        assert!(!is_notable_dependency("my-random-lib"));
    }
}
