//! Context Engine - Personalized Relevance System for 4DA
//!
//! The Context Engine implements a 3-layer "Context Membrane" that transforms
//! generic information filtering into deeply personalized relevance.
//!
//! ## Layer Architecture
//!
//! 1. **Static Identity** (Phase 1) - Explicit user-declared interests
//!    - Role, tech stack, domains
//!    - Explicit topic interests with embeddings
//!    - Exclusion list (topics to never show)
//!
//! 2. **Active Context** (Phase 2) - Real-time awareness
//!    - Watched directories
//!    - Recent file modifications
//!    - Git commit analysis
//!    - Project detection
//!
//! 3. **Learned Behavior** (Phase 3) - Implicit preferences
//!    - Click/save/dismiss tracking
//!    - Topic affinities
//!    - Temporal decay
//!
//! All context data stays LOCAL - privacy is core to 4DA.

use parking_lot::Mutex;
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// Types - Static Identity (Layer 1)
// ============================================================================

/// User's static identity - explicit declarations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StaticIdentity {
    /// Professional role (e.g., "Backend Developer", "Data Scientist")
    pub role: Option<String>,

    /// Tech stack the user works with
    pub tech_stack: Vec<String>,

    /// Domains of interest (e.g., "distributed systems", "ML infrastructure")
    pub domains: Vec<String>,

    /// Explicit topic interests
    pub interests: Vec<Interest>,

    /// Topics to exclude (never show content about these)
    pub exclusions: Vec<String>,
}

/// A single interest with optional embedding for semantic matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interest {
    pub id: Option<i64>,
    pub topic: String,
    pub weight: f32, // 1.0 for explicit, lower for inferred
    pub embedding: Option<Vec<f32>>,
    pub source: InterestSource,
}

/// Where an interest came from
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InterestSource {
    Explicit, // User directly added
    GitHub,   // Imported from GitHub
    Import,   // Imported from other sources
    Inferred, // System inferred from behavior
}

impl Default for InterestSource {
    fn default() -> Self {
        InterestSource::Explicit
    }
}

// ============================================================================
// Types - Active Context (Layer 2) - Placeholder for Phase 2
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActiveContext {
    pub watched_dirs: Vec<WatchedDirectory>,
    pub active_topics: Vec<TopicWeight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedDirectory {
    pub path: String,
    pub enabled: bool,
    pub last_indexed: Option<String>,
    pub chunk_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicWeight {
    pub topic: String,
    pub weight: f32,
}

// ============================================================================
// Types - Learned Behavior (Layer 3) - Placeholder for Phase 3
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LearnedBehavior {
    pub topic_affinities: HashMap<String, TopicAffinity>,
    pub anti_topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicAffinity {
    pub topic: String,
    pub positive_signals: u32,
    pub negative_signals: u32,
    pub total_exposures: u32,
    pub affinity_score: f32,
}

// ============================================================================
// Types - Interaction Tracking
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub source_item_id: i64,
    pub action: InteractionType,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InteractionType {
    Click,   // Opened the item
    Save,    // Bookmarked
    Dismiss, // Marked not relevant
    Ignore,  // Scrolled past
}

// ============================================================================
// Context Membrane - The unified context model
// ============================================================================

/// The complete context membrane combining all three layers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextMembrane {
    pub static_identity: StaticIdentity,
    pub active_context: ActiveContext,
    pub learned_behavior: LearnedBehavior,
}

impl ContextMembrane {
    /// Compute relevance score for an item embedding against user context
    pub fn compute_relevance(&self, item_embedding: &[f32], item_topics: &[String]) -> f32 {
        // Check exclusions first (hard filter)
        for topic in item_topics {
            let topic_lower = topic.to_lowercase();
            for exclusion in &self.static_identity.exclusions {
                if topic_lower.contains(&exclusion.to_lowercase()) {
                    return 0.0;
                }
            }
        }

        // Layer 1: Static identity match (explicit interests)
        let static_score = self.compute_static_score(item_embedding);

        // Layer 2: Active context match (current work - placeholder)
        let active_score = self.compute_active_score(item_topics);

        // Layer 3: Learned behavior match (placeholder)
        let learned_score = self.compute_learned_score(item_topics);

        // Anti-topic penalty
        let anti_penalty = self.compute_anti_penalty(item_topics);

        // Weighted combination
        // Phase 1: Static gets full weight, others minimal until implemented
        let combined = static_score * 0.7      // Explicit intent (primary for now)
                     + active_score * 0.2      // Current work
                     + learned_score * 0.1     // Behavioral signal
                     - anti_penalty;

        combined.clamp(0.0, 1.0)
    }

    /// Compute score from static identity (explicit interests)
    fn compute_static_score(&self, item_embedding: &[f32]) -> f32 {
        if self.static_identity.interests.is_empty() {
            return 0.5; // Neutral if no interests defined
        }

        let mut max_score: f32 = 0.0;

        for interest in &self.static_identity.interests {
            if let Some(ref interest_embedding) = interest.embedding {
                let similarity = cosine_similarity(item_embedding, interest_embedding);
                let weighted = similarity * interest.weight;
                max_score = max_score.max(weighted);
            }
        }

        max_score
    }

    /// Compute score from active context (Phase 2 placeholder)
    fn compute_active_score(&self, item_topics: &[String]) -> f32 {
        if self.active_context.active_topics.is_empty() {
            return 0.0;
        }

        let mut max_score: f32 = 0.0;

        for active_topic in &self.active_context.active_topics {
            for item_topic in item_topics {
                if item_topic
                    .to_lowercase()
                    .contains(&active_topic.topic.to_lowercase())
                {
                    max_score = max_score.max(active_topic.weight);
                }
            }
        }

        max_score
    }

    /// Compute score from learned behavior (Phase 3 placeholder)
    fn compute_learned_score(&self, item_topics: &[String]) -> f32 {
        if self.learned_behavior.topic_affinities.is_empty() {
            return 0.0;
        }

        let mut total_score: f32 = 0.0;

        for topic in item_topics {
            let topic_lower = topic.to_lowercase();
            for (affinity_topic, affinity) in &self.learned_behavior.topic_affinities {
                if topic_lower.contains(&affinity_topic.to_lowercase())
                    && affinity.affinity_score > 0.0
                {
                    total_score += affinity.affinity_score;
                }
            }
        }

        total_score.min(1.0)
    }

    /// Compute anti-topic penalty
    fn compute_anti_penalty(&self, item_topics: &[String]) -> f32 {
        let mut penalty: f32 = 0.0;

        for topic in item_topics {
            let topic_lower = topic.to_lowercase();
            for anti in &self.learned_behavior.anti_topics {
                if topic_lower.contains(&anti.to_lowercase()) {
                    penalty += 0.3;
                }
            }
        }

        penalty.min(1.0)
    }
}

/// Cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

// ============================================================================
// Context Engine - Database-backed context management
// ============================================================================

pub struct ContextEngine {
    conn: Arc<Mutex<Connection>>,
}

impl ContextEngine {
    /// Create a new context engine using an existing database connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> SqliteResult<Self> {
        let engine = Self { conn };
        engine.migrate()?;
        Ok(engine)
    }

    /// Run database migrations for context engine tables
    fn migrate(&self) -> SqliteResult<()> {
        let conn = self.conn.lock();

        conn.execute_batch("
            -- User identity and role
            CREATE TABLE IF NOT EXISTS user_identity (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                role TEXT,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            );

            -- Tech stack
            CREATE TABLE IF NOT EXISTS tech_stack (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                technology TEXT NOT NULL UNIQUE,
                created_at TEXT DEFAULT (datetime('now'))
            );

            -- Domains of interest
            CREATE TABLE IF NOT EXISTS domains (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                domain TEXT NOT NULL UNIQUE,
                created_at TEXT DEFAULT (datetime('now'))
            );

            -- Explicit interests with embeddings
            CREATE TABLE IF NOT EXISTS explicit_interests (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                topic TEXT NOT NULL UNIQUE,
                weight REAL DEFAULT 1.0,
                embedding BLOB,
                source TEXT DEFAULT 'explicit',
                created_at TEXT DEFAULT (datetime('now'))
            );
            CREATE INDEX IF NOT EXISTS idx_interests_topic ON explicit_interests(topic);

            -- Exclusions (topics to never show)
            CREATE TABLE IF NOT EXISTS exclusions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                topic TEXT NOT NULL UNIQUE,
                created_at TEXT DEFAULT (datetime('now'))
            );
            CREATE INDEX IF NOT EXISTS idx_exclusions_topic ON exclusions(topic);

            -- Watched directories (Phase 2)
            CREATE TABLE IF NOT EXISTS watched_directories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                enabled INTEGER DEFAULT 1,
                last_indexed TEXT,
                chunk_count INTEGER DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now'))
            );

            -- Interactions for learning (Phase 3 + ACE compatible)
            CREATE TABLE IF NOT EXISTS interactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_item_id INTEGER,
                item_id INTEGER,
                action TEXT,
                action_type TEXT,
                action_data TEXT,
                item_topics TEXT,
                item_source TEXT,
                signal_strength REAL DEFAULT 0.5,
                timestamp TEXT DEFAULT (datetime('now'))
            );
            CREATE INDEX IF NOT EXISTS idx_interactions_item ON interactions(source_item_id);
            CREATE INDEX IF NOT EXISTS idx_interactions_action ON interactions(action);
            CREATE INDEX IF NOT EXISTS idx_interactions_source ON interactions(item_source);
            CREATE INDEX IF NOT EXISTS idx_interactions_timestamp ON interactions(timestamp);

            -- Learned topic affinities (Phase 3 + ACE compatible)
            CREATE TABLE IF NOT EXISTS topic_affinities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                topic TEXT NOT NULL UNIQUE,
                embedding BLOB,
                positive_signals INTEGER DEFAULT 0,
                negative_signals INTEGER DEFAULT 0,
                total_exposures INTEGER DEFAULT 0,
                affinity_score REAL DEFAULT 0.0,
                confidence REAL DEFAULT 0.0,
                last_interaction TEXT DEFAULT (datetime('now')),
                decay_applied INTEGER DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            );
            CREATE INDEX IF NOT EXISTS idx_affinities_topic ON topic_affinities(topic);
            CREATE INDEX IF NOT EXISTS idx_topic_affinities_score ON topic_affinities(affinity_score);

            -- Initialize singleton user identity if not exists
            INSERT OR IGNORE INTO user_identity (id) VALUES (1);
        ")?;

        println!("[4DA/Context] Context engine tables initialized");
        Ok(())
    }

    // ========================================================================
    // Static Identity Operations (Layer 1)
    // ========================================================================

    /// Get user role
    pub fn get_role(&self) -> SqliteResult<Option<String>> {
        let conn = self.conn.lock();
        conn.query_row("SELECT role FROM user_identity WHERE id = 1", [], |row| {
            row.get(0)
        })
    }

    /// Set user role
    pub fn set_role(&self, role: Option<&str>) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE user_identity SET role = ?1, updated_at = datetime('now') WHERE id = 1",
            params![role],
        )?;
        Ok(())
    }

    /// Get tech stack
    pub fn get_tech_stack(&self) -> SqliteResult<Vec<String>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT technology FROM tech_stack ORDER BY technology")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        rows.collect()
    }

    /// Add technology to stack
    pub fn add_technology(&self, tech: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT OR IGNORE INTO tech_stack (technology) VALUES (?1)",
            params![tech],
        )?;
        Ok(())
    }

    /// Remove technology from stack
    pub fn remove_technology(&self, tech: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "DELETE FROM tech_stack WHERE technology = ?1",
            params![tech],
        )?;
        Ok(())
    }

    /// Get domains
    pub fn get_domains(&self) -> SqliteResult<Vec<String>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT domain FROM domains ORDER BY domain")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        rows.collect()
    }

    /// Add domain
    pub fn add_domain(&self, domain: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT OR IGNORE INTO domains (domain) VALUES (?1)",
            params![domain],
        )?;
        Ok(())
    }

    /// Remove domain
    pub fn remove_domain(&self, domain: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute("DELETE FROM domains WHERE domain = ?1", params![domain])?;
        Ok(())
    }

    /// Get all explicit interests
    pub fn get_interests(&self) -> SqliteResult<Vec<Interest>> {
        let conn = self.conn.lock();
        let mut stmt =
            conn.prepare("SELECT id, topic, weight, embedding, source FROM explicit_interests")?;

        let rows = stmt.query_map([], |row| {
            let embedding_blob: Option<Vec<u8>> = row.get(3)?;
            let source_str: String = row.get(4)?;
            let source = match source_str.as_str() {
                "github" => InterestSource::GitHub,
                "import" => InterestSource::Import,
                "inferred" => InterestSource::Inferred,
                _ => InterestSource::Explicit,
            };

            Ok(Interest {
                id: Some(row.get(0)?),
                topic: row.get(1)?,
                weight: row.get(2)?,
                embedding: embedding_blob.map(|blob| blob_to_embedding(&blob)),
                source,
            })
        })?;

        rows.collect()
    }

    /// Add an interest
    pub fn add_interest(
        &self,
        topic: &str,
        weight: f32,
        embedding: Option<&[f32]>,
        source: InterestSource,
    ) -> SqliteResult<i64> {
        let conn = self.conn.lock();
        let embedding_blob = embedding.map(|e| embedding_to_blob(e));
        let source_str = match source {
            InterestSource::Explicit => "explicit",
            InterestSource::GitHub => "github",
            InterestSource::Import => "import",
            InterestSource::Inferred => "inferred",
        };

        conn.execute(
            "INSERT INTO explicit_interests (topic, weight, embedding, source)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(topic) DO UPDATE SET
                weight = excluded.weight,
                embedding = excluded.embedding,
                source = excluded.source",
            params![topic, weight, embedding_blob, source_str],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Remove an interest
    pub fn remove_interest(&self, topic: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "DELETE FROM explicit_interests WHERE topic = ?1",
            params![topic],
        )?;
        Ok(())
    }

    /// Get all exclusions
    pub fn get_exclusions(&self) -> SqliteResult<Vec<String>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT topic FROM exclusions ORDER BY topic")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        rows.collect()
    }

    /// Add an exclusion
    pub fn add_exclusion(&self, topic: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT OR IGNORE INTO exclusions (topic) VALUES (?1)",
            params![topic],
        )?;
        Ok(())
    }

    /// Remove an exclusion
    pub fn remove_exclusion(&self, topic: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute("DELETE FROM exclusions WHERE topic = ?1", params![topic])?;
        Ok(())
    }

    /// Get the complete static identity
    pub fn get_static_identity(&self) -> SqliteResult<StaticIdentity> {
        Ok(StaticIdentity {
            role: self.get_role()?,
            tech_stack: self.get_tech_stack()?,
            domains: self.get_domains()?,
            interests: self.get_interests()?,
            exclusions: self.get_exclusions()?,
        })
    }

    // ========================================================================
    // Interaction Tracking (Layer 3 - Foundation)
    // ========================================================================

    /// Record an interaction
    pub fn record_interaction(
        &self,
        source_item_id: i64,
        action: InteractionType,
    ) -> SqliteResult<()> {
        let conn = self.conn.lock();
        let action_str = match action {
            InteractionType::Click => "click",
            InteractionType::Save => "save",
            InteractionType::Dismiss => "dismiss",
            InteractionType::Ignore => "ignore",
        };

        conn.execute(
            "INSERT INTO interactions (source_item_id, action) VALUES (?1, ?2)",
            params![source_item_id, action_str],
        )?;
        Ok(())
    }

    /// Get interaction counts for an item
    pub fn get_interaction_counts(
        &self,
        source_item_id: i64,
    ) -> SqliteResult<HashMap<String, u32>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT action, COUNT(*) FROM interactions WHERE source_item_id = ?1 GROUP BY action",
        )?;

        let mut counts = HashMap::new();
        let rows = stmt.query_map(params![source_item_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?))
        })?;

        for row in rows {
            let (action, count) = row?;
            counts.insert(action, count);
        }

        Ok(counts)
    }

    // ========================================================================
    // Full Context Membrane
    // ========================================================================

    /// Load the complete context membrane
    pub fn get_context_membrane(&self) -> SqliteResult<ContextMembrane> {
        let static_identity = self.get_static_identity()?;

        // Active context and learned behavior are placeholders for now
        let active_context = ActiveContext::default();
        let learned_behavior = LearnedBehavior::default();

        Ok(ContextMembrane {
            static_identity,
            active_context,
            learned_behavior,
        })
    }

    /// Get interest count
    pub fn interest_count(&self) -> SqliteResult<i64> {
        let conn = self.conn.lock();
        conn.query_row("SELECT COUNT(*) FROM explicit_interests", [], |row| {
            row.get(0)
        })
    }

    /// Get exclusion count
    pub fn exclusion_count(&self) -> SqliteResult<i64> {
        let conn = self.conn.lock();
        conn.query_row("SELECT COUNT(*) FROM exclusions", [], |row| row.get(0))
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert f32 embedding to blob for storage
fn embedding_to_blob(embedding: &[f32]) -> Vec<u8> {
    embedding.iter().flat_map(|f| f.to_le_bytes()).collect()
}

/// Convert blob back to f32 embedding
fn blob_to_embedding(blob: &[u8]) -> Vec<f32> {
    blob.chunks_exact(4)
        .map(|chunk| {
            let arr: [u8; 4] = chunk.try_into().unwrap();
            f32::from_le_bytes(arr)
        })
        .collect()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);

        let c = vec![0.0, 1.0, 0.0];
        assert!(cosine_similarity(&a, &c).abs() < 1e-6);

        let d = vec![-1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &d) - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_exclusion_filter() {
        let mut membrane = ContextMembrane::default();
        membrane.static_identity.exclusions = vec!["crypto".to_string(), "nft".to_string()];

        let embedding = vec![0.5; 384];
        let topics_clean = vec!["rust".to_string(), "programming".to_string()];
        let topics_excluded = vec!["cryptocurrency".to_string(), "programming".to_string()];

        // Clean topics should get a non-zero score
        let score_clean = membrane.compute_relevance(&embedding, &topics_clean);
        assert!(score_clean > 0.0);

        // Excluded topics should get zero
        let score_excluded = membrane.compute_relevance(&embedding, &topics_excluded);
        assert_eq!(score_excluded, 0.0);
    }
}
