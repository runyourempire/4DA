//! Database module for 4DA - Persistence layer for embeddings and sources
//!
//! Uses sqlite-vec for vector similarity search at scale.
//! Designed to handle hundreds of thousands of sources.

use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use rusqlite::{params, Connection, Result as SqliteResult};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::Arc;

// ============================================================================
// Types
// ============================================================================

/// A stored context chunk with its embedding
#[derive(Debug, Clone)]
pub struct StoredContext {
    pub id: i64,
    pub source_file: String,
    pub content_hash: String,
    pub text: String,
    pub embedding: Vec<f32>,
    pub created_at: DateTime<Utc>,
}

/// A stored source item (HN story, arXiv paper, RSS item, etc.)
#[derive(Debug, Clone)]
pub struct StoredSourceItem {
    pub id: i64,
    pub source_type: String, // "hackernews", "arxiv", "rss", etc.
    pub source_id: String,   // External ID (HN id, arXiv id, etc.)
    pub url: Option<String>,
    pub title: String,
    pub content: String,
    pub content_hash: String,
    pub embedding: Vec<f32>,
    pub created_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

/// Similarity result from vector search
#[derive(Debug, Clone)]
pub struct SimilarityResult {
    pub context_id: i64,
    pub source_file: String,
    pub text: String,
    pub distance: f32,
}

// ============================================================================
// Database Manager
// ============================================================================

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Initialize database with sqlite-vec extension
    pub fn new(db_path: &Path) -> SqliteResult<Self> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(db_path)?;

        // Load sqlite-vec extension
        unsafe {
            sqlite_vec::sqlite3_vec_init();
        }

        // Enable extension loading and initialize sqlite-vec
        conn.execute_batch(
            "
            -- Enable foreign keys
            PRAGMA foreign_keys = ON;

            -- WAL mode for better concurrency
            PRAGMA journal_mode = WAL;
        ",
        )?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        // Run migrations
        db.migrate()?;

        Ok(db)
    }

    /// Run database migrations
    fn migrate(&self) -> SqliteResult<()> {
        let conn = self.conn.lock();

        conn.execute_batch(
            "
            -- Context chunks table (your local files)
            CREATE TABLE IF NOT EXISTS context_chunks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_file TEXT NOT NULL,
                content_hash TEXT NOT NULL UNIQUE,
                text TEXT NOT NULL,
                embedding BLOB NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            CREATE INDEX IF NOT EXISTS idx_context_source ON context_chunks(source_file);
            CREATE INDEX IF NOT EXISTS idx_context_hash ON context_chunks(content_hash);

            -- Source items table (HN, arXiv, RSS, etc.)
            CREATE TABLE IF NOT EXISTS source_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_type TEXT NOT NULL,
                source_id TEXT NOT NULL,
                url TEXT,
                title TEXT NOT NULL,
                content TEXT NOT NULL DEFAULT '',
                content_hash TEXT NOT NULL,
                embedding BLOB NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_seen TEXT NOT NULL DEFAULT (datetime('now')),
                UNIQUE(source_type, source_id)
            );
            CREATE INDEX IF NOT EXISTS idx_source_type ON source_items(source_type);
            CREATE INDEX IF NOT EXISTS idx_source_hash ON source_items(content_hash);
            CREATE INDEX IF NOT EXISTS idx_source_seen ON source_items(last_seen);

            -- Sources registry (track what sources we monitor)
            CREATE TABLE IF NOT EXISTS sources (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_type TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                config TEXT,  -- JSON config for the source
                last_fetch TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            -- User feedback for learning
            CREATE TABLE IF NOT EXISTS feedback (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_item_id INTEGER NOT NULL,
                relevant INTEGER NOT NULL,  -- 1 = relevant, 0 = not relevant
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (source_item_id) REFERENCES source_items(id)
            );
            CREATE INDEX IF NOT EXISTS idx_feedback_item ON feedback(source_item_id);

            -- Schema version for future migrations
            CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY
            );
            INSERT OR IGNORE INTO schema_version (version) VALUES (1);
        ",
        )?;

        println!("[4DA/DB] Database schema initialized");
        Ok(())
    }

    // ========================================================================
    // Context Operations
    // ========================================================================

    /// Store a context chunk with its embedding
    pub fn upsert_context(
        &self,
        source_file: &str,
        text: &str,
        embedding: &[f32],
    ) -> SqliteResult<i64> {
        let conn = self.conn.lock();
        let content_hash = hash_content(text);
        let embedding_blob = embedding_to_blob(embedding);

        conn.execute(
            "INSERT INTO context_chunks (source_file, content_hash, text, embedding, updated_at)
             VALUES (?1, ?2, ?3, ?4, datetime('now'))
             ON CONFLICT(content_hash) DO UPDATE SET
                source_file = excluded.source_file,
                updated_at = datetime('now')",
            params![source_file, content_hash, text, embedding_blob],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Get all context embeddings
    pub fn get_all_contexts(&self) -> SqliteResult<Vec<StoredContext>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, source_file, content_hash, text, embedding, created_at
             FROM context_chunks",
        )?;

        let rows = stmt.query_map([], |row| {
            let embedding_blob: Vec<u8> = row.get(4)?;
            Ok(StoredContext {
                id: row.get(0)?,
                source_file: row.get(1)?,
                content_hash: row.get(2)?,
                text: row.get(3)?,
                embedding: blob_to_embedding(&embedding_blob),
                created_at: Utc::now(), // Simplified for now
            })
        })?;

        rows.collect()
    }

    /// Clear all context chunks (for re-indexing)
    pub fn clear_contexts(&self) -> SqliteResult<usize> {
        let conn = self.conn.lock();
        conn.execute("DELETE FROM context_chunks", [])
    }

    /// Get context count
    pub fn context_count(&self) -> SqliteResult<i64> {
        let conn = self.conn.lock();
        conn.query_row("SELECT COUNT(*) FROM context_chunks", [], |row| row.get(0))
    }

    // ========================================================================
    // Source Item Operations
    // ========================================================================

    /// Store or update a source item
    pub fn upsert_source_item(
        &self,
        source_type: &str,
        source_id: &str,
        url: Option<&str>,
        title: &str,
        content: &str,
        embedding: &[f32],
    ) -> SqliteResult<i64> {
        let conn = self.conn.lock();
        let content_hash = hash_content(&format!("{}{}", title, content));
        let embedding_blob = embedding_to_blob(embedding);

        conn.execute(
            "INSERT INTO source_items (source_type, source_id, url, title, content, content_hash, embedding, last_seen)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))
             ON CONFLICT(source_type, source_id) DO UPDATE SET
                url = excluded.url,
                title = excluded.title,
                content = excluded.content,
                content_hash = excluded.content_hash,
                embedding = excluded.embedding,
                last_seen = datetime('now')",
            params![source_type, source_id, url, title, content, content_hash, embedding_blob],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Check if a source item exists (for incremental updates)
    pub fn source_item_exists(&self, source_type: &str, source_id: &str) -> SqliteResult<bool> {
        let conn = self.conn.lock();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM source_items WHERE source_type = ?1 AND source_id = ?2",
            params![source_type, source_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Get a single source item by type and id (for cache lookup)
    pub fn get_source_item(
        &self,
        source_type: &str,
        source_id: &str,
    ) -> SqliteResult<Option<StoredSourceItem>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, source_type, source_id, url, title, content, content_hash, embedding, created_at, last_seen
             FROM source_items
             WHERE source_type = ?1 AND source_id = ?2"
        )?;

        let mut rows = stmt.query_map(params![source_type, source_id], |row| {
            let embedding_blob: Vec<u8> = row.get(7)?;
            Ok(StoredSourceItem {
                id: row.get(0)?,
                source_type: row.get(1)?,
                source_id: row.get(2)?,
                url: row.get(3)?,
                title: row.get(4)?,
                content: row.get(5)?,
                content_hash: row.get(6)?,
                embedding: blob_to_embedding(&embedding_blob),
                created_at: Utc::now(),
                last_seen: Utc::now(),
            })
        })?;

        match rows.next() {
            Some(Ok(item)) => Ok(Some(item)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    /// Update last_seen timestamp for an existing item
    pub fn touch_source_item(&self, source_type: &str, source_id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE source_items SET last_seen = datetime('now') WHERE source_type = ?1 AND source_id = ?2",
            params![source_type, source_id],
        )?;
        Ok(())
    }

    /// Get source items by type
    pub fn get_source_items(
        &self,
        source_type: &str,
        limit: usize,
    ) -> SqliteResult<Vec<StoredSourceItem>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, source_type, source_id, url, title, content, content_hash, embedding, created_at, last_seen
             FROM source_items
             WHERE source_type = ?1
             ORDER BY last_seen DESC
             LIMIT ?2"
        )?;

        let rows = stmt.query_map(params![source_type, limit as i64], |row| {
            let embedding_blob: Vec<u8> = row.get(7)?;
            Ok(StoredSourceItem {
                id: row.get(0)?,
                source_type: row.get(1)?,
                source_id: row.get(2)?,
                url: row.get(3)?,
                title: row.get(4)?,
                content: row.get(5)?,
                content_hash: row.get(6)?,
                embedding: blob_to_embedding(&embedding_blob),
                created_at: Utc::now(),
                last_seen: Utc::now(),
            })
        })?;

        rows.collect()
    }

    /// Get all recent source items across all types
    pub fn get_recent_items(&self, limit: usize) -> SqliteResult<Vec<StoredSourceItem>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, source_type, source_id, url, title, content, content_hash, embedding, created_at, last_seen
             FROM source_items
             ORDER BY last_seen DESC
             LIMIT ?1"
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            let embedding_blob: Vec<u8> = row.get(7)?;
            Ok(StoredSourceItem {
                id: row.get(0)?,
                source_type: row.get(1)?,
                source_id: row.get(2)?,
                url: row.get(3)?,
                title: row.get(4)?,
                content: row.get(5)?,
                content_hash: row.get(6)?,
                embedding: blob_to_embedding(&embedding_blob),
                created_at: Utc::now(),
                last_seen: Utc::now(),
            })
        })?;

        rows.collect()
    }

    /// Count items by source type
    pub fn source_item_count(&self, source_type: &str) -> SqliteResult<i64> {
        let conn = self.conn.lock();
        conn.query_row(
            "SELECT COUNT(*) FROM source_items WHERE source_type = ?1",
            params![source_type],
            |row| row.get(0),
        )
    }

    /// Count total items
    pub fn total_item_count(&self) -> SqliteResult<i64> {
        let conn = self.conn.lock();
        conn.query_row("SELECT COUNT(*) FROM source_items", [], |row| row.get(0))
    }

    // ========================================================================
    // Feedback Operations
    // ========================================================================

    /// Record user feedback
    pub fn record_feedback(&self, source_item_id: i64, relevant: bool) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT INTO feedback (source_item_id, relevant) VALUES (?1, ?2)",
            params![source_item_id, relevant as i32],
        )?;
        Ok(())
    }

    // ========================================================================
    // Source Registry
    // ========================================================================

    /// Register a source
    pub fn register_source(&self, source_type: &str, name: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT OR IGNORE INTO sources (source_type, name) VALUES (?1, ?2)",
            params![source_type, name],
        )?;
        Ok(())
    }

    /// Update last fetch time for a source
    pub fn update_source_fetch_time(&self, source_type: &str) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE sources SET last_fetch = datetime('now') WHERE source_type = ?1",
            params![source_type],
        )?;
        Ok(())
    }

    /// Get enabled sources
    pub fn get_enabled_sources(&self) -> SqliteResult<Vec<(String, String)>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT source_type, name FROM sources WHERE enabled = 1")?;

        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

        rows.collect()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Hash content for deduplication
fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}

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
    use std::path::PathBuf;

    #[test]
    fn test_hash_content() {
        let hash1 = hash_content("hello world");
        let hash2 = hash_content("hello world");
        let hash3 = hash_content("different content");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_embedding_roundtrip() {
        let embedding = vec![0.1, 0.2, 0.3, -0.5, 1.0];
        let blob = embedding_to_blob(&embedding);
        let recovered = blob_to_embedding(&blob);

        assert_eq!(embedding.len(), recovered.len());
        for (a, b) in embedding.iter().zip(recovered.iter()) {
            assert!((a - b).abs() < 1e-6);
        }
    }
}
