//! ACE Database Schema and Migrations
//!
//! Implements the full ACE database schema as specified in the stone tablet.

use parking_lot::Mutex;
use rusqlite::Connection;
use std::sync::Arc;

/// Run all ACE database migrations
pub fn migrate(conn: &Arc<Mutex<Connection>>) -> Result<(), String> {
    let conn = conn.lock();

    conn.execute_batch(r#"
        -- ═══════════════════════════════════════════════════════════════
        -- SIGNAL ACQUISITION TABLES
        -- ═══════════════════════════════════════════════════════════════

        -- Detected projects from manifest scanning
        CREATE TABLE IF NOT EXISTS detected_projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            languages TEXT,                -- JSON array
            frameworks TEXT,               -- JSON array
            dependencies TEXT,             -- JSON array
            last_activity TEXT,
            detection_confidence REAL DEFAULT 0.5,
            created_at TEXT DEFAULT (datetime('now')),
            updated_at TEXT DEFAULT (datetime('now'))
        );

        -- Detected technologies (merged from all sources)
        CREATE TABLE IF NOT EXISTS detected_tech (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            category TEXT NOT NULL,        -- 'language', 'framework', 'library', etc.
            confidence REAL DEFAULT 0.5,
            source TEXT NOT NULL,          -- 'manifest', 'file_extension', etc.
            evidence TEXT,                 -- Semicolon-separated evidence strings
            created_at TEXT DEFAULT (datetime('now')),
            updated_at TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_detected_tech_name ON detected_tech(name);
        CREATE INDEX IF NOT EXISTS idx_detected_tech_confidence ON detected_tech(confidence);

        -- File change signals
        CREATE TABLE IF NOT EXISTS file_signals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            change_type TEXT NOT NULL,     -- 'created', 'modified', 'deleted'
            extracted_topics TEXT,         -- JSON array
            content_hash TEXT,
            timestamp TEXT DEFAULT (datetime('now')),
            processed INTEGER DEFAULT 0
        );
        CREATE INDEX IF NOT EXISTS idx_file_signals_timestamp ON file_signals(timestamp);
        CREATE INDEX IF NOT EXISTS idx_file_signals_processed ON file_signals(processed);

        -- Git signals
        CREATE TABLE IF NOT EXISTS git_signals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_path TEXT NOT NULL,
            commit_hash TEXT,
            commit_message TEXT,
            extracted_topics TEXT,         -- JSON array
            files_changed TEXT,            -- JSON array
            timestamp TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_git_signals_timestamp ON git_signals(timestamp);
        CREATE INDEX IF NOT EXISTS idx_git_signals_repo ON git_signals(repo_path);

        -- ═══════════════════════════════════════════════════════════════
        -- ACTIVE CONTEXT TABLES
        -- ═══════════════════════════════════════════════════════════════

        -- Active topics (derived from current work)
        CREATE TABLE IF NOT EXISTS active_topics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            topic TEXT NOT NULL UNIQUE,
            weight REAL DEFAULT 0.5,
            confidence REAL DEFAULT 0.5,
            embedding BLOB,
            source TEXT NOT NULL,          -- 'file_content', 'git_commit', etc.
            last_seen TEXT DEFAULT (datetime('now')),
            decay_applied INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_active_topics_topic ON active_topics(topic);
        CREATE INDEX IF NOT EXISTS idx_active_topics_last_seen ON active_topics(last_seen);

        -- ═══════════════════════════════════════════════════════════════
        -- LEARNED BEHAVIOR TABLES (extensions to existing)
        -- ═══════════════════════════════════════════════════════════════

        -- Anti-topics (learned exclusions from behavior)
        CREATE TABLE IF NOT EXISTS anti_topics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            topic TEXT NOT NULL UNIQUE,
            rejection_count INTEGER DEFAULT 0,
            confidence REAL DEFAULT 0.0,
            auto_detected INTEGER DEFAULT 1,
            user_confirmed INTEGER DEFAULT 0,
            first_rejection TEXT DEFAULT (datetime('now')),
            last_rejection TEXT DEFAULT (datetime('now')),
            created_at TEXT DEFAULT (datetime('now')),
            updated_at TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_anti_topics_topic ON anti_topics(topic);

        -- User interactions (behavior signals)
        CREATE TABLE IF NOT EXISTS interactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item_id INTEGER NOT NULL,
            action_type TEXT NOT NULL,          -- 'click', 'save', 'share', 'dismiss', etc.
            action_data TEXT,                   -- JSON with action-specific data (dwell_time, etc.)
            item_topics TEXT,                   -- JSON array
            item_source TEXT,                   -- 'hackernews', 'arxiv', etc.
            signal_strength REAL NOT NULL,
            timestamp TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_interactions_timestamp ON interactions(timestamp);
        CREATE INDEX IF NOT EXISTS idx_interactions_item ON interactions(item_id);
        CREATE INDEX IF NOT EXISTS idx_interactions_source ON interactions(item_source);

        -- Topic affinities (learned preferences)
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
        CREATE INDEX IF NOT EXISTS idx_topic_affinities_topic ON topic_affinities(topic);
        CREATE INDEX IF NOT EXISTS idx_topic_affinities_score ON topic_affinities(affinity_score);

        -- Source preferences (learned from behavior)
        CREATE TABLE IF NOT EXISTS source_preferences (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source TEXT NOT NULL UNIQUE,
            score REAL DEFAULT 0.0,
            interactions INTEGER DEFAULT 0,
            last_interaction TEXT DEFAULT (datetime('now')),
            created_at TEXT DEFAULT (datetime('now')),
            updated_at TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_source_preferences_source ON source_preferences(source);

        -- Activity patterns (time-based engagement)
        CREATE TABLE IF NOT EXISTS activity_patterns (
            id INTEGER PRIMARY KEY CHECK (id = 1),  -- Singleton
            hourly_engagement TEXT,              -- JSON array [24 floats]
            daily_engagement TEXT,               -- JSON array [7 floats]
            total_tracked INTEGER DEFAULT 0,
            updated_at TEXT DEFAULT (datetime('now'))
        );
        INSERT OR IGNORE INTO activity_patterns (id, hourly_engagement, daily_engagement, total_tracked)
        VALUES (1, '[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]', '[0,0,0,0,0,0,0]', 0);

        -- ═══════════════════════════════════════════════════════════════
        -- VALIDATION & MONITORING TABLES
        -- ═══════════════════════════════════════════════════════════════

        -- Signal validation records
        CREATE TABLE IF NOT EXISTS validated_signals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            signal_type TEXT NOT NULL,
            signal_data TEXT NOT NULL,     -- JSON
            confidence REAL NOT NULL,
            evidence_sources TEXT,         -- JSON array
            contradictions TEXT,           -- JSON array
            freshness REAL,
            timestamp TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_validated_signals_type ON validated_signals(signal_type);
        CREATE INDEX IF NOT EXISTS idx_validated_signals_timestamp ON validated_signals(timestamp);

        -- Audit trail
        CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            entry_type TEXT NOT NULL,
            action TEXT NOT NULL,
            reason TEXT,
            contributing_factors TEXT,     -- JSON array
            before_state TEXT,
            after_state TEXT,
            confidence REAL,
            timestamp TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_audit_log_timestamp ON audit_log(timestamp);
        CREATE INDEX IF NOT EXISTS idx_audit_log_type ON audit_log(entry_type);

        -- Accuracy metrics (daily snapshots)
        CREATE TABLE IF NOT EXISTS accuracy_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            metric_date TEXT NOT NULL UNIQUE,
            precision_score REAL,
            recall_score REAL,
            engagement_rate REAL,
            items_shown INTEGER DEFAULT 0,
            items_clicked INTEGER DEFAULT 0,
            positive_feedback INTEGER DEFAULT 0,
            negative_feedback INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_accuracy_metrics_date ON accuracy_metrics(metric_date);

        -- System health records
        CREATE TABLE IF NOT EXISTS system_health (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            component TEXT NOT NULL,
            status TEXT NOT NULL,          -- 'healthy', 'degraded', 'failed', 'disabled'
            last_success TEXT,
            error_count INTEGER DEFAULT 0,
            last_error TEXT,
            checked_at TEXT DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_system_health_component ON system_health(component);

        -- ═══════════════════════════════════════════════════════════════
        -- COLD START BOOTSTRAP TABLE
        -- ═══════════════════════════════════════════════════════════════

        -- Common project paths to scan on first run
        CREATE TABLE IF NOT EXISTS bootstrap_paths (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            priority INTEGER DEFAULT 0,
            scanned INTEGER DEFAULT 0,
            last_scanned TEXT,
            created_at TEXT DEFAULT (datetime('now'))
        );

        -- Insert default bootstrap paths if not exists
        INSERT OR IGNORE INTO bootstrap_paths (path, priority) VALUES
            ('~/projects', 10),
            ('~/code', 10),
            ('~/dev', 10),
            ('~/src', 10),
            ('~/Documents/GitHub', 8),
            ('~/repos', 8),
            ('~/workspace', 8),
            ('~/work', 7),
            ('~/.config', 3);
    "#).map_err(|e| format!("ACE migration failed: {}", e))?;

    println!("[ACE] Database schema initialized");
    Ok(())
}

/// Get bootstrap paths for initial scan
pub fn get_bootstrap_paths(conn: &Arc<Mutex<Connection>>) -> Result<Vec<String>, String> {
    let conn = conn.lock();
    let mut stmt = conn
        .prepare("SELECT path FROM bootstrap_paths WHERE scanned = 0 ORDER BY priority DESC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let paths: Result<Vec<String>, _> = rows.collect();
    paths.map_err(|e| e.to_string())
}

/// Mark a bootstrap path as scanned
pub fn mark_path_scanned(conn: &Arc<Mutex<Connection>>, path: &str) -> Result<(), String> {
    let conn = conn.lock();
    conn.execute(
        "UPDATE bootstrap_paths SET scanned = 1, last_scanned = datetime('now') WHERE path = ?1",
        rusqlite::params![path],
    )
    .map_err(|e| format!("Failed to mark path scanned: {}", e))?;
    Ok(())
}

/// Record accuracy metrics for the day
pub fn record_accuracy_metrics(
    conn: &Arc<Mutex<Connection>>,
    items_shown: u64,
    items_clicked: u64,
    positive_feedback: u64,
    negative_feedback: u64,
) -> Result<(), String> {
    let conn = conn.lock();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let precision = if items_shown > 0 {
        (items_clicked + positive_feedback) as f32 / items_shown as f32
    } else {
        0.0
    };

    let engagement = if items_shown > 0 {
        items_clicked as f32 / items_shown as f32
    } else {
        0.0
    };

    conn.execute(
        "INSERT INTO accuracy_metrics (metric_date, precision_score, engagement_rate, items_shown, items_clicked, positive_feedback, negative_feedback)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(metric_date) DO UPDATE SET
            precision_score = excluded.precision_score,
            engagement_rate = excluded.engagement_rate,
            items_shown = excluded.items_shown,
            items_clicked = excluded.items_clicked,
            positive_feedback = excluded.positive_feedback,
            negative_feedback = excluded.negative_feedback",
        rusqlite::params![today, precision, engagement, items_shown, items_clicked, positive_feedback, negative_feedback],
    ).map_err(|e| format!("Failed to record accuracy metrics: {}", e))?;

    Ok(())
}

/// Update component health status
pub fn update_component_health(
    conn: &Arc<Mutex<Connection>>,
    component: &str,
    status: &str,
    error: Option<&str>,
) -> Result<(), String> {
    let conn = conn.lock();

    if let Some(err) = error {
        conn.execute(
            "INSERT INTO system_health (component, status, error_count, last_error)
             VALUES (?1, ?2, 1, ?3)
             ON CONFLICT(component) DO UPDATE SET
                status = excluded.status,
                error_count = system_health.error_count + 1,
                last_error = excluded.last_error,
                checked_at = datetime('now')",
            rusqlite::params![component, status, err],
        )
    } else {
        conn.execute(
            "INSERT INTO system_health (component, status, last_success, error_count)
             VALUES (?1, ?2, datetime('now'), 0)
             ON CONFLICT(component) DO UPDATE SET
                status = excluded.status,
                last_success = datetime('now'),
                error_count = 0,
                last_error = NULL,
                checked_at = datetime('now')",
            rusqlite::params![component, status],
        )
    }
    .map_err(|e| format!("Failed to update health: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migration() {
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        assert!(migrate(&conn).is_ok());

        // Verify tables exist
        let conn_guard = conn.lock();
        let tables: Vec<String> = conn_guard
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();

        assert!(tables.contains(&"detected_projects".to_string()));
        assert!(tables.contains(&"detected_tech".to_string()));
        assert!(tables.contains(&"active_topics".to_string()));
        assert!(tables.contains(&"audit_log".to_string()));
    }
}
