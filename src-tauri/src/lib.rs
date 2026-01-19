use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Listener, Manager};

mod ace;
mod context_engine;
mod db;
mod llm;
mod monitoring;
mod settings;
mod sources;

use context_engine::{ContextEngine, ContextMembrane, InteractionType, InterestSource};
use db::Database;
use llm::RelevanceJudge;
use settings::{LLMProvider, RerankConfig, SettingsManager};
use sources::{
    arxiv::ArxivSource, hackernews::HackerNewsSource, reddit::RedditSource, SourceRegistry,
};

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFile {
    pub path: String,
    pub content: String,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSettings {
    pub configured_dirs: Vec<String>,
    pub active_dirs: Vec<String>,
    pub using_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HNItem {
    pub id: u64,
    pub title: String,
    pub url: Option<String>,
    pub content: String, // Scraped article content or HN text
}

#[derive(Debug, Deserialize)]
struct HNStory {
    id: u64,
    title: Option<String>,
    url: Option<String>,
    text: Option<String>, // For Ask HN / Show HN / text posts
}

/// A chunk of text with its embedding
#[derive(Debug, Clone)]
struct EmbeddedChunk {
    source_file: String,
    text: String,
    embedding: Vec<f32>,
}

/// Relevance match between an HN item and context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevanceMatch {
    pub source_file: String,
    pub matched_text: String,
    pub similarity: f32,
}

/// Full relevance result for an HN item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HNRelevance {
    pub id: u64,
    pub title: String,
    pub url: Option<String>,
    pub top_score: f32,
    pub matches: Vec<RelevanceMatch>,
    pub relevant: bool,
    /// Score from context files (what you're working on)
    #[serde(default)]
    pub context_score: f32,
    /// Score from explicit interests (what you care about)
    #[serde(default)]
    pub interest_score: f32,
    /// Whether this item was filtered by an exclusion
    #[serde(default)]
    pub excluded: bool,
    /// The exclusion that blocked this item (if any)
    #[serde(default)]
    pub excluded_by: Option<String>,
}

/// Status update for the UI (sent via events)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStatus {
    pub stage: String,
    pub progress: f32,
    pub message: String,
    pub items_processed: usize,
    pub items_total: usize,
}

/// Background analysis state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisState {
    pub running: bool,
    pub completed: bool,
    pub error: Option<String>,
    pub results: Option<Vec<HNRelevance>>,
}

/// LLM judgment attached to a relevance result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMJudgment {
    pub relevant: bool,
    pub confidence: f32,
    pub reasoning: String,
    pub key_connections: Vec<String>,
}

/// Enhanced relevance result with optional LLM judgment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedRelevance {
    pub id: u64,
    pub title: String,
    pub url: Option<String>,
    pub embedding_score: f32,
    pub matches: Vec<RelevanceMatch>,
    pub embedding_relevant: bool,
    /// LLM re-ranking judgment (if enabled)
    pub llm_judgment: Option<LLMJudgment>,
    /// Final relevance after both stages
    pub final_relevant: bool,
}

// ============================================================================
// Global Embedding Model (Lazy Initialized)
// ============================================================================

static EMBEDDING_MODEL: OnceCell<Mutex<TextEmbedding>> = OnceCell::new();

fn get_embedding_model() -> Result<&'static Mutex<TextEmbedding>, String> {
    EMBEDDING_MODEL.get_or_try_init(|| {
        println!("[4DA] Initializing embedding model (MiniLM-L6-v2)...");
        println!("[4DA] This may take a moment on first run (downloading model)...");

        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
        )
        .map_err(|e| format!("Failed to initialize embedding model: {}", e))?;

        println!("[4DA] Embedding model ready (384 dimensions)");
        Ok(Mutex::new(model))
    })
}

/// Generate embeddings for a list of texts
fn embed_texts(texts: &[String]) -> Result<Vec<Vec<f32>>, String> {
    if texts.is_empty() {
        return Ok(vec![]);
    }

    let model = get_embedding_model()?;
    let model_guard = model.lock();

    // Convert to &str for the API
    let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

    model_guard
        .embed(text_refs, None)
        .map_err(|e| format!("Embedding failed: {}", e))
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

/// Extract topics/keywords from text for context matching
/// Returns lowercase keywords suitable for exclusion/interest matching
fn extract_topics(title: &str, content: &str) -> Vec<String> {
    let mut topics = Vec::new();

    // Combine title and first part of content
    let text = format!(
        "{} {}",
        title,
        content.chars().take(500).collect::<String>()
    );
    let text_lower = text.to_lowercase();

    // Technology keywords to look for
    let tech_keywords = [
        "rust",
        "python",
        "javascript",
        "typescript",
        "go",
        "golang",
        "java",
        "c++",
        "cpp",
        "react",
        "vue",
        "angular",
        "svelte",
        "node",
        "deno",
        "bun",
        "ai",
        "ml",
        "machine learning",
        "deep learning",
        "neural",
        "gpt",
        "llm",
        "transformer",
        "database",
        "sql",
        "postgresql",
        "postgres",
        "mysql",
        "mongodb",
        "redis",
        "sqlite",
        "kubernetes",
        "k8s",
        "docker",
        "container",
        "aws",
        "azure",
        "gcp",
        "cloud",
        "api",
        "rest",
        "graphql",
        "grpc",
        "microservice",
        "crypto",
        "cryptocurrency",
        "bitcoin",
        "ethereum",
        "blockchain",
        "nft",
        "web3",
        "defi",
        "startup",
        "vc",
        "funding",
        "acquisition",
        "open source",
        "oss",
        "github",
        "git",
        "security",
        "vulnerability",
        "hack",
        "breach",
        "performance",
        "optimization",
        "scale",
        "scalability",
        "frontend",
        "backend",
        "fullstack",
        "devops",
        "sre",
        "linux",
        "unix",
        "windows",
        "macos",
        "mobile",
        "ios",
        "android",
        "flutter",
        "react native",
        "game",
        "gaming",
        "gamedev",
        "hardware",
        "chip",
        "semiconductor",
        "cpu",
        "gpu",
        "climate",
        "sustainability",
        "energy",
        "sports",
        "football",
        "basketball",
        "soccer",
        "politics",
        "election",
        "government",
    ];

    for keyword in &tech_keywords {
        if text_lower.contains(keyword) {
            topics.push(keyword.to_string());
        }
    }

    // Also extract capitalized words from title as potential topics
    for word in title.split_whitespace() {
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
        if clean.len() > 2
            && clean
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
        {
            let lower = clean.to_lowercase();
            if !topics.contains(&lower)
                && !["the", "and", "for", "how", "why", "what", "show", "ask"]
                    .contains(&lower.as_str())
            {
                topics.push(lower);
            }
        }
    }

    topics
}

/// Check if an item should be excluded based on user exclusions
/// Returns Some(exclusion) if blocked, None if allowed
fn check_exclusions(topics: &[String], exclusions: &[String]) -> Option<String> {
    for topic in topics {
        let topic_lower = topic.to_lowercase();
        for exclusion in exclusions {
            let exclusion_lower = exclusion.to_lowercase();
            if topic_lower.contains(&exclusion_lower) || exclusion_lower.contains(&topic_lower) {
                return Some(exclusion.clone());
            }
        }
    }
    None
}

/// Compute interest score by comparing item embedding against interest embeddings
fn compute_interest_score(item_embedding: &[f32], interests: &[context_engine::Interest]) -> f32 {
    if interests.is_empty() {
        return 0.0;
    }

    let mut max_score: f32 = 0.0;

    for interest in interests {
        if let Some(ref interest_embedding) = interest.embedding {
            let similarity = cosine_similarity(item_embedding, interest_embedding);
            let weighted = similarity * interest.weight;
            max_score = max_score.max(weighted);
        }
    }

    max_score
}

// ============================================================================
// ACE Context Integration
// ============================================================================

/// ACE-discovered context for relevance scoring
#[derive(Debug, Default)]
struct ACEContext {
    /// Active topics detected from project manifests and git history
    active_topics: Vec<String>,
    /// Detected tech stack (languages, frameworks)
    detected_tech: Vec<String>,
    /// Anti-topics (topics user has consistently rejected)
    anti_topics: Vec<String>,
    /// Topic affinities from behavior learning (topic -> boost factor)
    topic_affinities: std::collections::HashMap<String, f32>,
}

/// Fetch ACE-discovered context for relevance scoring
fn get_ace_context() -> ACEContext {
    let ace = match get_ace_engine() {
        Ok(engine) => engine,
        Err(_) => return ACEContext::default(),
    };

    let mut ctx = ACEContext::default();

    // Get active topics
    if let Ok(topics) = ace.get_active_topics() {
        ctx.active_topics = topics
            .iter()
            .filter(|t| t.weight >= 0.3)
            .map(|t| t.topic.to_lowercase())
            .collect();
    }

    // Get detected tech
    if let Ok(tech) = ace.get_detected_tech() {
        ctx.detected_tech = tech.iter().map(|t| t.name.to_lowercase()).collect();
    }

    // Get anti-topics from behavior learning
    if let Ok(anti_topics) = ace.get_anti_topics(3) {
        ctx.anti_topics = anti_topics
            .iter()
            .filter(|a| a.user_confirmed || a.confidence >= 0.7)
            .map(|a| a.topic.to_lowercase())
            .collect();
    }

    // Get topic affinities
    if let Ok(affinities) = ace.get_topic_affinities() {
        for aff in affinities {
            // Only use positive affinities with enough interactions
            if aff.affinity_score > 0.0 && aff.total_exposures >= 3 {
                ctx.topic_affinities
                    .insert(aff.topic.to_lowercase(), aff.affinity_score);
            }
        }
    }

    ctx
}

/// Check if item should be excluded by ACE anti-topics
fn check_ace_exclusions(topics: &[String], ace_ctx: &ACEContext) -> Option<String> {
    for topic in topics {
        let topic_lower = topic.to_lowercase();
        for anti_topic in &ace_ctx.anti_topics {
            if topic_lower.contains(anti_topic) || anti_topic.contains(&topic_lower) {
                return Some(format!("ACE anti-topic: {}", anti_topic));
            }
        }
    }
    None
}

/// Compute ACE context boost for an item based on topic matches
fn compute_ace_boost(topics: &[String], ace_ctx: &ACEContext) -> f32 {
    let mut boost: f32 = 0.0;

    for topic in topics {
        let topic_lower = topic.to_lowercase();

        // Boost for matching active topics (project context)
        for active_topic in &ace_ctx.active_topics {
            if topic_lower.contains(active_topic) || active_topic.contains(&topic_lower) {
                boost += 0.15;
                break;
            }
        }

        // Boost for matching detected tech stack
        for tech in &ace_ctx.detected_tech {
            if topic_lower.contains(tech) || tech.contains(&topic_lower) {
                boost += 0.1;
                break;
            }
        }

        // Apply learned affinity boost
        if let Some(&affinity) = ace_ctx.topic_affinities.get(&topic_lower) {
            boost += affinity * 0.2; // Scale affinity contribution
        }
    }

    // Cap boost at 0.3 to avoid overwhelming other signals
    boost.min(0.3)
}

// ============================================================================
// Global Database (Lazy Initialized)
// ============================================================================

static DATABASE: OnceCell<Arc<Database>> = OnceCell::new();

fn get_database() -> Result<&'static Arc<Database>, String> {
    DATABASE.get_or_try_init(|| {
        let mut db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        db_path.pop();
        db_path.push("data");
        db_path.push("4da.db");

        println!("[4DA] Initializing database at {:?}...", db_path);

        let db =
            Database::new(&db_path).map_err(|e| format!("Failed to initialize database: {}", e))?;

        // Register default sources
        db.register_source("hackernews", "Hacker News").ok();
        db.register_source("arxiv", "arXiv").ok();
        db.register_source("reddit", "Reddit").ok();

        println!("[4DA] Database ready");
        Ok(Arc::new(db))
    })
}

// ============================================================================
// Global Context Engine (Lazy Initialized)
// ============================================================================

static CONTEXT_ENGINE: OnceCell<Arc<ContextEngine>> = OnceCell::new();

fn get_context_engine() -> Result<&'static Arc<ContextEngine>, String> {
    CONTEXT_ENGINE.get_or_try_init(|| {
        // Context engine shares the database connection
        let mut db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        db_path.pop();
        db_path.push("data");
        db_path.push("4da.db");

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database for context engine: {}", e))?;

        let engine = ContextEngine::new(Arc::new(parking_lot::Mutex::new(conn)))
            .map_err(|e| format!("Failed to initialize context engine: {}", e))?;

        println!("[4DA/Context] Context engine ready");
        Ok(Arc::new(engine))
    })
}

// ============================================================================
// Global ACE Instance (Lazy Initialized with RwLock for mutable access)
// ============================================================================

static ACE_ENGINE: OnceCell<Arc<parking_lot::RwLock<ace::ACE>>> = OnceCell::new();

fn init_ace_engine() -> Result<Arc<parking_lot::RwLock<ace::ACE>>, String> {
    // ACE shares the database connection
    let mut db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    db_path.pop();
    db_path.push("data");
    db_path.push("4da.db");

    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database for ACE: {}", e))?;

    let engine = ace::ACE::new(Arc::new(parking_lot::Mutex::new(conn)))
        .map_err(|e| format!("Failed to initialize ACE: {}", e))?;

    println!("[ACE] Autonomous Context Engine ready");
    Ok(Arc::new(parking_lot::RwLock::new(engine)))
}

fn get_ace_engine() -> Result<parking_lot::RwLockReadGuard<'static, ace::ACE>, String> {
    let engine = ACE_ENGINE.get_or_try_init(init_ace_engine)?;
    Ok(engine.read())
}

fn get_ace_engine_mut() -> Result<parking_lot::RwLockWriteGuard<'static, ace::ACE>, String> {
    let engine = ACE_ENGINE.get_or_try_init(init_ace_engine)?;
    Ok(engine.write())
}

// ============================================================================
// Global Source Registry (Lazy Initialized)
// ============================================================================

static SOURCE_REGISTRY: OnceCell<Mutex<SourceRegistry>> = OnceCell::new();

fn get_source_registry() -> &'static Mutex<SourceRegistry> {
    SOURCE_REGISTRY.get_or_init(|| {
        println!("[4DA] Initializing source registry...");
        let mut registry = SourceRegistry::new();

        // Register default sources
        registry.register(Box::new(HackerNewsSource::new()));
        registry.register(Box::new(ArxivSource::new()));
        registry.register(Box::new(RedditSource::new()));

        println!("[4DA] Source registry ready ({} sources)", registry.count());
        Mutex::new(registry)
    })
}

// ============================================================================
// Global Settings Manager
// ============================================================================

static SETTINGS_MANAGER: OnceCell<Mutex<SettingsManager>> = OnceCell::new();

fn get_settings_manager() -> &'static Mutex<SettingsManager> {
    SETTINGS_MANAGER.get_or_init(|| {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.pop();
        data_path.push("data");

        println!("[4DA] Initializing settings manager...");
        let manager = SettingsManager::new(&data_path);
        println!(
            "[4DA] Settings loaded. LLM re-ranking: {}",
            if manager.is_rerank_enabled() {
                "enabled"
            } else {
                "disabled"
            }
        );
        Mutex::new(manager)
    })
}

// ============================================================================
// Global Analysis State
// ============================================================================

static ANALYSIS_STATE: OnceCell<Mutex<AnalysisState>> = OnceCell::new();

fn get_analysis_state() -> &'static Mutex<AnalysisState> {
    ANALYSIS_STATE.get_or_init(|| {
        Mutex::new(AnalysisState {
            running: false,
            completed: false,
            error: None,
            results: None,
        })
    })
}

// ============================================================================
// Global Monitoring State
// ============================================================================

static MONITORING_STATE: OnceCell<Arc<monitoring::MonitoringState>> = OnceCell::new();

fn get_monitoring_state() -> &'static Arc<monitoring::MonitoringState> {
    MONITORING_STATE.get_or_init(|| Arc::new(monitoring::MonitoringState::new()))
}

// ============================================================================
// Configuration
// ============================================================================

/// Get context directories from settings (no fallback - empty means no context)
fn get_context_dirs() -> Vec<PathBuf> {
    let settings = get_settings_manager().lock();
    let dirs = settings.get().context_dirs.clone();
    drop(settings);

    dirs.into_iter().map(PathBuf::from).collect()
}

/// Legacy function for single directory (uses first configured dir)
fn get_context_dir() -> Option<PathBuf> {
    get_context_dirs().into_iter().next()
}

/// File extensions we care about for Phase 0
const SUPPORTED_EXTENSIONS: &[&str] = &["md", "txt", "rs", "ts", "js", "py"];

/// Relevance threshold for "relevant" classification (lowered for content-based matching)
const RELEVANCE_THRESHOLD: f32 = 0.30;

/// Maximum content length for embedding (roughly 1000 words)
const MAX_CONTENT_LENGTH: usize = 5000;

/// Maximum chunk size in characters (roughly 100-150 words)
const MAX_CHUNK_SIZE: usize = 500;

// ============================================================================
// Text Processing
// ============================================================================

/// Split text into chunks for embedding
fn chunk_text(text: &str, source_file: &str) -> Vec<(String, String)> {
    let mut chunks = Vec::new();
    let paragraphs: Vec<&str> = text.split("\n\n").collect();

    let mut current_chunk = String::new();

    for para in paragraphs {
        let para = para.trim();
        if para.is_empty() {
            continue;
        }

        if current_chunk.len() + para.len() > MAX_CHUNK_SIZE {
            if !current_chunk.is_empty() {
                chunks.push((source_file.to_string(), current_chunk.clone()));
                current_chunk.clear();
            }
        }

        if !current_chunk.is_empty() {
            current_chunk.push_str("\n\n");
        }
        current_chunk.push_str(para);
    }

    if !current_chunk.is_empty() {
        chunks.push((source_file.to_string(), current_chunk));
    }

    // If no chunks were created, use the whole text
    if chunks.is_empty() && !text.trim().is_empty() {
        chunks.push((source_file.to_string(), text.trim().to_string()));
    }

    chunks
}

// ============================================================================
// Content Scraping
// ============================================================================

/// Scrape article content from a URL
async fn scrape_article_content(url: &str) -> Option<String> {
    use scraper::{Html, Selector};

    // Skip non-HTTP URLs and known problematic domains
    if !url.starts_with("http") {
        return None;
    }

    // Create client with timeout
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;

    // Fetch the page
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (compatible; 4DA/0.1)")
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let html = response.text().await.ok()?;
    let document = Html::parse_document(&html);

    // Try multiple content selectors in order of preference
    let selectors = [
        "article",
        "main",
        "[role='main']",
        ".post-content",
        ".article-content",
        ".entry-content",
        ".content",
        "body",
    ];

    for selector_str in selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                let text: String = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");

                // Only use if we got meaningful content (at least 100 chars)
                if text.len() > 100 {
                    // Truncate to max length
                    let truncated = if text.len() > MAX_CONTENT_LENGTH {
                        text.chars().take(MAX_CONTENT_LENGTH).collect()
                    } else {
                        text
                    };
                    return Some(truncated);
                }
            }
        }
    }

    None
}

/// Build embedding text from HN item (title + content)
fn build_embedding_text(title: &str, content: &str) -> String {
    if content.is_empty() {
        title.to_string()
    } else {
        // Combine title and content with clear separation
        format!("{}\n\n{}", title, content)
    }
}

/// Emit a progress event to the frontend
fn emit_progress(
    app: &AppHandle,
    stage: &str,
    progress: f32,
    message: &str,
    processed: usize,
    total: usize,
) {
    let status = AnalysisStatus {
        stage: stage.to_string(),
        progress,
        message: message.to_string(),
        items_processed: processed,
        items_total: total,
    };
    let _ = app.emit("analysis-progress", &status);
}

// ============================================================================
// Commands
// ============================================================================

#[tauri::command]
async fn get_context_files() -> Result<Vec<ContextFile>, String> {
    let context_dir = match get_context_dir() {
        Some(dir) => dir,
        None => {
            println!("[4DA] No context directory configured");
            return Ok(vec![]);
        }
    };
    println!("[4DA] Reading context files from: {:?}", context_dir);

    if !context_dir.exists() {
        println!("[4DA] Context directory does not exist");
        return Ok(vec![]);
    }

    let mut files = Vec::new();
    let entries = fs::read_dir(&context_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !SUPPORTED_EXTENSIONS.contains(&ext) {
            continue;
        }

        match fs::read_to_string(&path) {
            Ok(content) => {
                let lines = content.lines().count();
                let path_str = path.to_string_lossy().to_string();
                println!("[4DA] Loaded: {} ({} lines)", path_str, lines);
                files.push(ContextFile {
                    path: path_str,
                    content,
                    lines,
                });
            }
            Err(e) => {
                println!("[4DA] Failed to read {:?}: {}", path, e);
            }
        }
    }

    println!("[4DA] Total context files loaded: {}", files.len());
    Ok(files)
}

/// Clear all indexed context chunks from the database
#[tauri::command]
async fn clear_context() -> Result<String, String> {
    println!("[4DA] Clearing indexed context...");

    // Use the singleton database connection (same one used by analysis)
    let db = get_database()?;

    let cleared = db
        .clear_contexts()
        .map_err(|e| format!("Failed to clear context: {}", e))?;

    println!(
        "[4DA] Context cleared successfully ({} chunks removed)",
        cleared
    );
    Ok(format!(
        "Context cleared successfully ({} chunks removed)",
        cleared
    ))
}

/// Index context files - read, chunk, embed, and store in database
#[tauri::command]
async fn index_context() -> Result<String, String> {
    println!("[4DA] Indexing context files...");

    let db = get_database()?;

    // First clear existing context to avoid duplicates
    let _ = db.clear_contexts();

    // Read context files from configured directories
    let context_files = get_context_files().await?;
    if context_files.is_empty() {
        return Err("No context files found. Add files to your context directory.".to_string());
    }

    // Chunk the files
    let mut all_chunks: Vec<(String, String)> = Vec::new();
    for file in &context_files {
        let filename = file
            .path
            .split('/')
            .last()
            .and_then(|s| s.split('\\').last())
            .unwrap_or(&file.path);
        let chunks = chunk_text(&file.content, filename);
        println!("[4DA]   {} -> {} chunks", filename, chunks.len());
        all_chunks.extend(chunks);
    }

    if all_chunks.is_empty() {
        return Err("No content to index from context files.".to_string());
    }

    // Generate embeddings
    println!(
        "[4DA] Generating embeddings for {} chunks...",
        all_chunks.len()
    );
    let chunk_texts: Vec<String> = all_chunks.iter().map(|(_, text)| text.clone()).collect();
    let chunk_embeddings = embed_texts(&chunk_texts)?;

    // Store in database
    println!(
        "[4DA] Storing {} context chunks in database...",
        all_chunks.len()
    );
    for ((source, text), embedding) in all_chunks.iter().zip(chunk_embeddings.iter()) {
        db.upsert_context(source, text, embedding)
            .map_err(|e| format!("Failed to store context: {}", e))?;
    }

    let msg = format!(
        "Indexed {} files ({} chunks)",
        context_files.len(),
        all_chunks.len()
    );
    println!("[4DA] {}", msg);
    Ok(msg)
}

/// Get current context directory settings
#[tauri::command]
async fn get_context_settings() -> Result<ContextSettings, String> {
    let dirs = get_context_dirs();
    let settings = get_settings_manager().lock();
    let configured = settings.get().context_dirs.clone();
    drop(settings);

    let using_default = configured.is_empty();
    Ok(ContextSettings {
        configured_dirs: configured,
        active_dirs: dirs
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect(),
        using_default,
    })
}

/// Set context directories
#[tauri::command]
async fn set_context_dirs(dirs: Vec<String>) -> Result<String, String> {
    println!("[4DA] Setting context directories: {:?}", dirs);

    // Validate directories exist
    for dir in &dirs {
        let path = PathBuf::from(dir);
        if !path.exists() {
            return Err(format!("Directory does not exist: {}", dir));
        }
        if !path.is_dir() {
            return Err(format!("Path is not a directory: {}", dir));
        }
    }

    let mut settings = get_settings_manager().lock();
    settings.get_mut().context_dirs = dirs.clone();
    settings.save()?;
    drop(settings);

    println!("[4DA] Context directories updated: {:?}", dirs);
    Ok(format!(
        "Context directories updated: {} directories configured",
        dirs.len()
    ))
}

#[tauri::command]
async fn get_hn_top_stories() -> Result<Vec<HNItem>, String> {
    println!("[4DA] Fetching HN top stories...");
    let client = reqwest::Client::new();

    let top_ids: Vec<u64> = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch top stories: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse top stories: {}", e))?;

    println!("[4DA] Got {} story IDs, fetching top 30...", top_ids.len());

    let mut items = Vec::new();
    for id in top_ids.into_iter().take(30) {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        match client.get(&url).send().await {
            Ok(response) => match response.json::<HNStory>().await {
                Ok(story) => {
                    let title = story.title.unwrap_or_else(|| "[No title]".to_string());

                    // Get content: prefer HN text field, otherwise scrape URL
                    let content = if let Some(text) = story.text {
                        // Ask HN / Show HN / text posts have content directly
                        println!("[4DA] HN #{}: {} (has text)", id, title);
                        text
                    } else if let Some(ref article_url) = story.url {
                        // Link posts - scrape the article
                        print!("[4DA] HN #{}: {} - scraping...", id, title);
                        match scrape_article_content(article_url).await {
                            Some(scraped) => {
                                println!(" got {} chars", scraped.len());
                                scraped
                            }
                            None => {
                                println!(" failed, using title only");
                                String::new()
                            }
                        }
                    } else {
                        println!("[4DA] HN #{}: {} (no content)", id, title);
                        String::new()
                    };

                    items.push(HNItem {
                        id: story.id,
                        title,
                        url: story.url,
                        content,
                    });
                }
                Err(e) => println!("[4DA] Failed to parse story {}: {}", id, e),
            },
            Err(e) => println!("[4DA] Failed to fetch story {}: {}", id, e),
        }
    }

    println!("[4DA] Loaded {} HN stories", items.len());
    Ok(items)
}

#[tauri::command]
async fn compute_relevance() -> Result<Vec<HNRelevance>, String> {
    println!("\n[4DA] ═══════════════════════════════════════════════════════════");
    println!("[4DA] COMPUTING RELEVANCE SCORES (Phase 1 - with persistence)");
    println!("[4DA] ═══════════════════════════════════════════════════════════\n");

    let db = get_database()?;

    // Step 1: Load and cache context embeddings
    println!("[4DA] Step 1: Loading context...");
    let cached_context_count = db.context_count().map_err(|e| e.to_string())?;

    let embedded_chunks: Vec<EmbeddedChunk> = if cached_context_count > 0 {
        // Use cached context
        println!(
            "[4DA]   Using {} cached context chunks",
            cached_context_count
        );
        db.get_all_contexts()
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|ctx| EmbeddedChunk {
                source_file: ctx.source_file,
                text: ctx.text,
                embedding: ctx.embedding,
            })
            .collect()
    } else {
        // NO auto-reindex - user must explicitly index context
        println!("[4DA]   No context indexed. Scores will be 0 without context.");
        println!("[4DA]   To index context: add files to context directory and reload.");
        vec![]
    };

    println!("[4DA]   Context ready: {} chunks", embedded_chunks.len());

    // Step 2: Fetch HN story IDs and process incrementally
    println!("[4DA] Step 2: Fetching HN stories (incremental)...");

    let client = reqwest::Client::new();
    let top_ids: Vec<u64> = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch top stories: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse top stories: {}", e))?;

    println!(
        "[4DA]   Got {} story IDs, processing top 30...",
        top_ids.len()
    );

    // Categorize: cached vs new
    let mut cached_items: Vec<(HNItem, Vec<f32>)> = Vec::new();
    let mut new_items: Vec<HNItem> = Vec::new();

    for id in top_ids.into_iter().take(30) {
        let id_str = id.to_string();

        // Check cache first
        if let Ok(Some(cached)) = db.get_source_item("hackernews", &id_str) {
            println!(
                "[4DA]   HN #{}: {} (cached)",
                id,
                &cached.title[..cached.title.len().min(40)]
            );
            db.touch_source_item("hackernews", &id_str).ok();
            cached_items.push((
                HNItem {
                    id,
                    title: cached.title,
                    url: cached.url,
                    content: cached.content,
                },
                cached.embedding,
            ));
        } else {
            // Need to fetch from API
            let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            match client.get(&url).send().await {
                Ok(response) => match response.json::<HNStory>().await {
                    Ok(story) => {
                        let title = story.title.unwrap_or_else(|| "[No title]".to_string());

                        // Get content: prefer HN text field, otherwise scrape URL
                        let content = if let Some(text) = story.text {
                            println!(
                                "[4DA]   HN #{}: {} (NEW - has text)",
                                id,
                                &title[..title.len().min(40)]
                            );
                            text
                        } else if let Some(ref article_url) = story.url {
                            print!(
                                "[4DA]   HN #{}: {} (NEW - scraping...)",
                                id,
                                &title[..title.len().min(35)]
                            );
                            match scrape_article_content(article_url).await {
                                Some(scraped) => {
                                    println!(" {} chars)", scraped.len());
                                    scraped
                                }
                                None => {
                                    println!(" failed)");
                                    String::new()
                                }
                            }
                        } else {
                            println!(
                                "[4DA]   HN #{}: {} (NEW - no content)",
                                id,
                                &title[..title.len().min(40)]
                            );
                            String::new()
                        };

                        new_items.push(HNItem {
                            id: story.id,
                            title,
                            url: story.url,
                            content,
                        });
                    }
                    Err(e) => println!("[4DA]   Failed to parse story {}: {}", id, e),
                },
                Err(e) => println!("[4DA]   Failed to fetch story {}: {}", id, e),
            }
        }
    }

    println!(
        "[4DA]   Found {} cached, {} new items",
        cached_items.len(),
        new_items.len()
    );

    // Step 3: Generate embeddings only for NEW items
    let new_embeddings = if !new_items.is_empty() {
        println!(
            "[4DA] Step 3: Generating embeddings for {} NEW items...",
            new_items.len()
        );
        let with_content = new_items.iter().filter(|i| !i.content.is_empty()).count();
        println!("[4DA]   {} items have scraped content", with_content);

        let new_texts: Vec<String> = new_items
            .iter()
            .map(|item| build_embedding_text(&item.title, &item.content))
            .collect();
        let embeddings = embed_texts(&new_texts)?;

        // Cache new items in database
        println!(
            "[4DA]   Caching {} new items in database...",
            new_items.len()
        );
        for (item, embedding) in new_items.iter().zip(embeddings.iter()) {
            db.upsert_source_item(
                "hackernews",
                &item.id.to_string(),
                item.url.as_deref(),
                &item.title,
                &item.content,
                embedding,
            )
            .ok();
        }

        embeddings
    } else {
        println!("[4DA] Step 3: All items cached, no embedding needed!");
        vec![]
    };

    db.update_source_fetch_time("hackernews").ok();

    // Combine cached and new items
    let mut all_items_with_embeddings: Vec<(HNItem, Vec<f32>)> = cached_items;
    for (item, embedding) in new_items.into_iter().zip(new_embeddings.into_iter()) {
        all_items_with_embeddings.push((item, embedding));
    }

    if all_items_with_embeddings.is_empty() {
        return Err("No HN stories fetched".to_string());
    }

    // Step 4: Load user context for personalized scoring
    println!("[4DA] Step 4: Loading user context...");
    let context_engine = get_context_engine()?;
    let static_identity = context_engine
        .get_static_identity()
        .map_err(|e| format!("Failed to load context: {}", e))?;

    let interest_count = static_identity.interests.len();
    let exclusion_count = static_identity.exclusions.len();
    println!(
        "[4DA]   {} explicit interests, {} exclusions loaded",
        interest_count, exclusion_count
    );

    if !static_identity.exclusions.is_empty() {
        println!(
            "[4DA]   Exclusions: [{}]",
            static_identity.exclusions.join(", ")
        );
    }
    if !static_identity.interests.is_empty() {
        let topics: Vec<&str> = static_identity
            .interests
            .iter()
            .map(|i| i.topic.as_str())
            .collect();
        println!("[4DA]   Interests: [{}]", topics.join(", "));
    }

    // Step 4b: Load ACE-discovered context
    println!("[4DA] Step 4b: Loading ACE discovered context...");
    let ace_ctx = get_ace_context();
    println!(
        "[4DA]   ACE: {} active topics, {} detected tech, {} anti-topics, {} affinities",
        ace_ctx.active_topics.len(),
        ace_ctx.detected_tech.len(),
        ace_ctx.anti_topics.len(),
        ace_ctx.topic_affinities.len()
    );

    if !ace_ctx.active_topics.is_empty() {
        println!(
            "[4DA]   ACE Topics: [{}]",
            ace_ctx
                .active_topics
                .iter()
                .take(5)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    if !ace_ctx.detected_tech.is_empty() {
        println!(
            "[4DA]   ACE Tech: [{}]",
            ace_ctx
                .detected_tech
                .iter()
                .take(5)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    // Step 5: Compute similarity scores with context integration
    println!(
        "[4DA] Step 5: Computing personalized relevance for {} items...\n",
        all_items_with_embeddings.len()
    );
    let mut results: Vec<HNRelevance> = Vec::new();
    let mut excluded_count = 0;

    for (item, item_embedding) in &all_items_with_embeddings {
        // Extract topics from this item
        let topics = extract_topics(&item.title, &item.content);

        // Check exclusions FIRST (hard filter)
        let excluded_by = check_exclusions(&topics, &static_identity.exclusions)
            .or_else(|| check_ace_exclusions(&topics, &ace_ctx));

        if let Some(ref exclusion) = excluded_by {
            println!(
                "[4DA] ✗ EXCLUDED: {} (matched: \"{}\")",
                &item.title[..item.title.len().min(50)],
                exclusion
            );
            excluded_count += 1;

            // Still add to results but marked as excluded
            results.push(HNRelevance {
                id: item.id,
                title: item.title.clone(),
                url: item.url.clone(),
                top_score: 0.0,
                matches: vec![],
                relevant: false,
                context_score: 0.0,
                interest_score: 0.0,
                excluded: true,
                excluded_by: Some(exclusion.clone()),
            });
            continue;
        }

        // Compute context file score (what you're working on)
        let mut matches: Vec<RelevanceMatch> = Vec::new();
        for chunk in &embedded_chunks {
            let similarity = cosine_similarity(item_embedding, &chunk.embedding);
            matches.push(RelevanceMatch {
                source_file: chunk.source_file.clone(),
                matched_text: if chunk.text.len() > 100 {
                    format!("{}...", &chunk.text[..100])
                } else {
                    chunk.text.clone()
                },
                similarity,
            });
        }
        matches.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        matches.truncate(3);

        let context_score = matches.first().map(|m| m.similarity).unwrap_or(0.0);

        // Compute interest score (what you care about)
        let interest_score = compute_interest_score(item_embedding, &static_identity.interests);

        // Compute ACE boost (what you're working on, discovered automatically)
        let ace_boost = compute_ace_boost(&topics, &ace_ctx);

        // Combined score: weighted average of context, interest scores, plus ACE boost
        // If user has interests defined, give them significant weight
        // If no interests, rely solely on context files + ACE
        let base_score = if interest_count > 0 {
            // 50% context (what you're working on) + 50% interests (what you care about)
            context_score * 0.5 + interest_score * 0.5
        } else {
            // No interests defined, use context score only
            context_score
        };

        // Add ACE boost to base score
        let combined_score = (base_score + ace_boost).min(1.0);

        let relevant = combined_score >= RELEVANCE_THRESHOLD;

        // Console output with personalization info
        let status = if relevant { "RELEVANT" } else { "not relevant" };
        println!("[4DA] ┌─────────────────────────────────────────────────────────────");
        println!("[4DA] │ HN #{}: {}", item.id, item.title);
        if ace_boost > 0.0 {
            println!(
                "[4DA] │ Combined: {:.3} (context: {:.3}, interest: {:.3}, ACE: +{:.3}) -> {}",
                combined_score, context_score, interest_score, ace_boost, status
            );
        } else {
            println!(
                "[4DA] │ Combined: {:.3} (context: {:.3}, interest: {:.3}) -> {}",
                combined_score, context_score, interest_score, status
            );
        }
        if !topics.is_empty() {
            println!(
                "[4DA] │ Topics: [{}]",
                topics
                    .iter()
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        for (i, m) in matches.iter().take(2).enumerate() {
            println!(
                "[4DA] │   {}. {:.3} | {} | \"{}\"",
                i + 1,
                m.similarity,
                m.source_file,
                m.matched_text
                    .replace('\n', " ")
                    .chars()
                    .take(40)
                    .collect::<String>()
            );
        }
        println!("[4DA] └─────────────────────────────────────────────────────────────\n");

        results.push(HNRelevance {
            id: item.id,
            title: item.title.clone(),
            url: item.url.clone(),
            top_score: combined_score,
            matches,
            relevant,
            context_score,
            interest_score,
            excluded: false,
            excluded_by: None,
        });
    }

    // Sort by relevance score descending (excluded items go to bottom)
    results.sort_by(|a, b| {
        if a.excluded && !b.excluded {
            return std::cmp::Ordering::Greater;
        }
        if !a.excluded && b.excluded {
            return std::cmp::Ordering::Less;
        }
        b.top_score.partial_cmp(&a.top_score).unwrap()
    });

    // Summary
    let relevant_count = results.iter().filter(|r| r.relevant && !r.excluded).count();
    let db_item_count = db.total_item_count().unwrap_or(0);
    println!("[4DA] ═══════════════════════════════════════════════════════════");
    println!("[4DA] PERSONALIZED ANALYSIS COMPLETE");
    println!(
        "[4DA] Total items: {} | Relevant: {} | Excluded: {}",
        results.len(),
        relevant_count,
        excluded_count
    );
    println!(
        "[4DA] User context: {} interests, {} exclusions",
        interest_count, exclusion_count
    );
    println!("[4DA] Threshold: {:.2}", RELEVANCE_THRESHOLD);
    println!("[4DA] Database: {} total items cached", db_item_count);
    println!("[4DA] ═══════════════════════════════════════════════════════════\n");

    Ok(results)
}

// ============================================================================
// App Entry
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    println!("╔════════════════════════════════════════════════╗");
    println!("║  4DA Home - Personalized Intelligence          ║");
    println!("║     The internet searches for you.             ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();
    println!("Context directory: {:?}", get_context_dir());
    println!("Embedding model: all-MiniLM-L6-v2 (384 dimensions)");
    println!("Relevance threshold: {:.2}", RELEVANCE_THRESHOLD);

    // Initialize database early
    match get_database() {
        Ok(db) => {
            let ctx_count = db.context_count().unwrap_or(0);
            let item_count = db.total_item_count().unwrap_or(0);
            println!(
                "Database: {} context chunks, {} source items cached",
                ctx_count, item_count
            );
        }
        Err(e) => {
            println!("Database: initialization failed - {}", e);
        }
    }

    // Initialize context engine
    match get_context_engine() {
        Ok(engine) => {
            let interest_count = engine.interest_count().unwrap_or(0);
            let exclusion_count = engine.exclusion_count().unwrap_or(0);
            if let Ok(identity) = engine.get_static_identity() {
                let role_str = identity.role.as_deref().unwrap_or("Not set");
                println!(
                    "Context Engine: {} interests, {} exclusions, role: {}",
                    interest_count, exclusion_count, role_str
                );
                if !identity.tech_stack.is_empty() {
                    println!("  Tech Stack: [{}]", identity.tech_stack.join(", "));
                }
                if !identity.domains.is_empty() {
                    println!("  Domains: [{}]", identity.domains.join(", "));
                }
            }
        }
        Err(e) => {
            println!("Context Engine: initialization failed - {}", e);
        }
    }

    // Initialize source registry
    let registry = get_source_registry();
    let source_count = registry.lock().count();
    let source_names: Vec<String> = registry
        .lock()
        .sources()
        .iter()
        .map(|s| s.name().to_string())
        .collect();
    println!(
        "Sources: {} registered [{}]",
        source_count,
        source_names.join(", ")
    );
    println!();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            get_context_files,
            clear_context,
            index_context,
            get_context_settings,
            set_context_dirs,
            get_hn_top_stories,
            compute_relevance,
            get_database_stats,
            get_sources,
            start_background_analysis,
            get_analysis_status,
            // Settings commands (Phase 2)
            get_settings,
            set_llm_provider,
            set_rerank_config,
            test_llm_connection,
            get_usage_stats,
            // Monitoring commands (Phase 3)
            get_monitoring_status,
            set_monitoring_enabled,
            set_monitoring_interval,
            trigger_notification_test,
            // Context Engine commands
            get_user_context,
            set_user_role,
            add_tech_stack,
            remove_tech_stack,
            add_domain,
            remove_domain,
            add_interest,
            remove_interest,
            add_exclusion,
            remove_exclusion,
            record_interaction,
            get_context_stats,
            // ACE (Autonomous Context Engine) commands - Phase A
            ace_detect_context,
            ace_get_detected_tech,
            ace_get_active_topics,
            ace_get_health,
            // ACE Phase B: Real-Time Context
            ace_analyze_git,
            ace_get_realtime_context,
            ace_apply_decay,
            ace_full_scan,
            // ACE Phase C: Behavior Learning
            ace_record_interaction,
            ace_get_topic_affinities,
            ace_get_anti_topics,
            ace_confirm_anti_topic,
            ace_get_behavior_modifier,
            ace_get_learned_behavior,
            ace_apply_behavior_decay,
            // ACE Phase D: Health Monitoring & Validation
            ace_check_health,
            ace_get_system_status,
            ace_get_fallback_level,
            ace_get_alerts,
            ace_get_audit_log,
            ace_explain_decision,
            ace_get_accuracy_metrics,
            ace_get_accuracy_history,
            ace_record_accuracy_feedback,
            ace_persist_accuracy,
            ace_check_accuracy_targets,
            // ACE Phase E: Anomaly Detection
            ace_detect_anomalies,
            ace_get_unresolved_anomalies,
            ace_resolve_anomaly,
            ace_get_recent_anomalies,
            // ACE Phase E: Embedding
            ace_embed_topic,
            ace_find_similar_topics,
            ace_embedding_status,
            // ACE Phase E: Watcher Persistence
            ace_save_watcher_state,
            ace_get_watcher_state,
            ace_clear_watcher_state,
            // ACE Phase E: Rate Limiting
            ace_get_rate_limit_status,
            // ACE Watcher Control
            ace_start_watcher,
            ace_stop_watcher,
            ace_is_watching
        ])
        .setup(|app| {
            // Set up system tray
            let tray = monitoring::setup_tray(app.handle()).expect("Failed to set up system tray");

            // Store tray handle for later updates
            app.manage(std::sync::Mutex::new(Some(tray)));

            // Load monitoring settings from persistence
            let monitoring_state = get_monitoring_state().clone();
            {
                let settings = get_settings_manager().lock();
                let config = settings.get_monitoring_config();
                monitoring_state.set_enabled(config.enabled);
                monitoring_state.set_interval(config.interval_minutes * 60);
                println!(
                    "[4DA/Monitor] Loaded saved settings: enabled={}, interval={}min",
                    config.enabled, config.interval_minutes
                );
            }

            // Start background scheduler
            let app_handle = app.handle().clone();
            monitoring::start_scheduler(app_handle.clone(), monitoring_state.clone());

            // Listen for tray events
            let app_handle_analyze = app_handle.clone();
            app.listen("tray-analyze", move |_| {
                println!("[4DA/Tray] Manual analysis triggered from tray");
                let _ = app_handle_analyze.emit("start-analysis-from-tray", ());
            });

            let app_handle_toggle = app_handle.clone();
            app.listen("tray-toggle-monitoring", move |_| {
                let state = get_monitoring_state();
                let new_enabled = !state.is_enabled();
                state.set_enabled(new_enabled);
                println!(
                    "[4DA/Monitor] Monitoring {}",
                    if new_enabled { "enabled" } else { "disabled" }
                );
                let _ = app_handle_toggle.emit("monitoring-toggled", new_enabled);
            });

            // Listen for scheduled analysis events
            let app_handle_scheduled = app_handle.clone();
            app.listen("scheduled-analysis", move |_| {
                println!("[4DA/Monitor] Scheduled analysis starting...");
                // Trigger analysis via the app handle
                let handle = app_handle_scheduled.clone();
                tauri::async_runtime::spawn(async move {
                    // Run the analysis
                    match run_background_analysis(&handle).await {
                        Ok(results) => {
                            let relevant_count = results.iter().filter(|r| r.relevant).count();
                            let state = get_monitoring_state();
                            monitoring::complete_scheduled_check(
                                &handle,
                                &state,
                                relevant_count,
                                results.len(),
                            );
                            // Emit results to frontend if window is visible
                            let _ = handle.emit("analysis-complete", results);
                        }
                        Err(e) => {
                            println!("[4DA/Monitor] Scheduled analysis failed: {}", e);
                            let state = get_monitoring_state();
                            state
                                .is_checking
                                .store(false, std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                });
            });

            println!("[4DA/Tray] System tray and monitoring initialized");

            // Initialize ACE with configured directories (runs async in background)
            initialize_ace_on_startup(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Get registered sources
#[tauri::command]
async fn get_sources() -> Result<Vec<serde_json::Value>, String> {
    let registry = get_source_registry();
    let guard = registry.lock();

    let sources: Vec<serde_json::Value> = guard
        .sources()
        .iter()
        .map(|s| {
            serde_json::json!({
                "type": s.source_type(),
                "name": s.name(),
                "enabled": s.config().enabled,
                "max_items": s.config().max_items,
                "fetch_interval_secs": s.config().fetch_interval_secs
            })
        })
        .collect();

    Ok(sources)
}

/// Get database statistics
#[tauri::command]
async fn get_database_stats() -> Result<serde_json::Value, String> {
    let db = get_database()?;

    let context_count = db.context_count().map_err(|e| e.to_string())?;
    let hn_count = db
        .source_item_count("hackernews")
        .map_err(|e| e.to_string())?;
    let total_count = db.total_item_count().map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "context_chunks": context_count,
        "hackernews_items": hn_count,
        "total_items": total_count
    }))
}

/// Start background analysis - returns immediately, emits progress events
#[tauri::command]
async fn start_background_analysis(app: AppHandle) -> Result<(), String> {
    // Check if already running
    {
        let state = get_analysis_state();
        let guard = state.lock();
        if guard.running {
            return Err("Analysis already running".to_string());
        }
    }

    // Mark as running
    {
        let state = get_analysis_state();
        let mut guard = state.lock();
        guard.running = true;
        guard.completed = false;
        guard.error = None;
        guard.results = None;
    }

    // Spawn background task
    tokio::spawn(async move {
        let result = run_background_analysis(&app).await;

        // Update state with result
        let state = get_analysis_state();
        let mut guard = state.lock();
        guard.running = false;

        match result {
            Ok(results) => {
                guard.completed = true;
                guard.results = Some(results.clone());

                // Emit completion event
                let _ = app.emit("analysis-complete", &results);
            }
            Err(e) => {
                guard.error = Some(e.clone());

                // Emit error event
                let _ = app.emit("analysis-error", &e);
            }
        }
    });

    Ok(())
}

/// The actual background analysis work
async fn run_background_analysis(app: &AppHandle) -> Result<Vec<HNRelevance>, String> {
    println!("\n[4DA] ═══════════════════════════════════════════════════════════");
    println!("[4DA] BACKGROUND ANALYSIS STARTED");
    println!("[4DA] ═══════════════════════════════════════════════════════════\n");

    emit_progress(app, "init", 0.0, "Initializing...", 0, 0);

    let db = get_database()?;

    // Step 1: Load context
    emit_progress(app, "context", 0.05, "Loading context...", 0, 0);
    let cached_context_count = db.context_count().map_err(|e| e.to_string())?;

    let embedded_chunks: Vec<EmbeddedChunk> = if cached_context_count > 0 {
        println!("[4DA] Using {} cached context chunks", cached_context_count);
        emit_progress(
            app,
            "context",
            0.1,
            &format!("Using {} cached context chunks", cached_context_count),
            0,
            0,
        );
        db.get_all_contexts()
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|ctx| EmbeddedChunk {
                source_file: ctx.source_file,
                text: ctx.text,
                embedding: ctx.embedding,
            })
            .collect()
    } else {
        // NO auto-reindex - user must explicitly index context
        println!("[4DA] No context indexed. Running without context-based scoring.");
        emit_progress(
            app,
            "context",
            0.1,
            "No context indexed - add files to context directory",
            0,
            0,
        );
        vec![]
    };

    // Step 2: Fetch HN story IDs
    emit_progress(app, "fetch", 0.15, "Fetching story IDs...", 0, 30);

    let client = reqwest::Client::new();
    let top_ids: Vec<u64> = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch top stories: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse top stories: {}", e))?;

    let total_items = 30.min(top_ids.len());
    emit_progress(
        app,
        "fetch",
        0.2,
        &format!("Processing {} stories...", total_items),
        0,
        total_items,
    );

    // Step 3: Process items incrementally with progress updates
    let mut cached_items: Vec<(HNItem, Vec<f32>)> = Vec::new();
    let mut new_items: Vec<HNItem> = Vec::new();

    for (idx, id) in top_ids.into_iter().take(total_items).enumerate() {
        let id_str = id.to_string();
        let progress = 0.2 + (0.5 * (idx as f32 / total_items as f32));

        // Check cache first
        if let Ok(Some(cached)) = db.get_source_item("hackernews", &id_str) {
            emit_progress(
                app,
                "fetch",
                progress,
                &format!("Cached: {}", &cached.title[..cached.title.len().min(35)]),
                idx + 1,
                total_items,
            );
            db.touch_source_item("hackernews", &id_str).ok();
            cached_items.push((
                HNItem {
                    id,
                    title: cached.title,
                    url: cached.url,
                    content: cached.content,
                },
                cached.embedding,
            ));
        } else {
            // Fetch from API
            let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            match client.get(&url).send().await {
                Ok(response) => match response.json::<HNStory>().await {
                    Ok(story) => {
                        let title = story.title.unwrap_or_else(|| "[No title]".to_string());
                        emit_progress(
                            app,
                            "fetch",
                            progress,
                            &format!("Fetching: {}", &title[..title.len().min(35)]),
                            idx + 1,
                            total_items,
                        );

                        let content = if let Some(text) = story.text {
                            text
                        } else if let Some(ref article_url) = story.url {
                            emit_progress(
                                app,
                                "scrape",
                                progress,
                                &format!("Scraping: {}", &title[..title.len().min(35)]),
                                idx + 1,
                                total_items,
                            );
                            scrape_article_content(article_url)
                                .await
                                .unwrap_or_default()
                        } else {
                            String::new()
                        };

                        new_items.push(HNItem {
                            id: story.id,
                            title,
                            url: story.url,
                            content,
                        });
                    }
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
    }

    // Step 4: Embed new items
    let new_embeddings = if !new_items.is_empty() {
        emit_progress(
            app,
            "embed",
            0.75,
            &format!("Embedding {} new items...", new_items.len()),
            cached_items.len(),
            total_items,
        );

        let new_texts: Vec<String> = new_items
            .iter()
            .map(|item| build_embedding_text(&item.title, &item.content))
            .collect();
        let embeddings = embed_texts(&new_texts)?;

        for (item, embedding) in new_items.iter().zip(embeddings.iter()) {
            db.upsert_source_item(
                "hackernews",
                &item.id.to_string(),
                item.url.as_deref(),
                &item.title,
                &item.content,
                embedding,
            )
            .ok();
        }

        embeddings
    } else {
        emit_progress(
            app,
            "embed",
            0.75,
            "All items cached!",
            total_items,
            total_items,
        );
        vec![]
    };

    db.update_source_fetch_time("hackernews").ok();

    // Combine all items
    let mut all_items_with_embeddings: Vec<(HNItem, Vec<f32>)> = cached_items;
    for (item, embedding) in new_items.into_iter().zip(new_embeddings.into_iter()) {
        all_items_with_embeddings.push((item, embedding));
    }

    // Step 5: Load user context for personalized scoring
    emit_progress(
        app,
        "relevance",
        0.82,
        "Loading user context...",
        0,
        all_items_with_embeddings.len(),
    );

    let context_engine = get_context_engine()?;
    let static_identity = context_engine
        .get_static_identity()
        .map_err(|e| format!("Failed to load context: {}", e))?;

    let interest_count = static_identity.interests.len();
    let exclusion_count = static_identity.exclusions.len();
    println!(
        "[4DA]   {} explicit interests, {} exclusions loaded",
        interest_count, exclusion_count
    );

    // Step 6: Compute personalized relevance
    emit_progress(
        app,
        "relevance",
        0.85,
        "Computing personalized relevance...",
        0,
        all_items_with_embeddings.len(),
    );

    let mut results: Vec<HNRelevance> = Vec::new();
    let mut excluded_count = 0;

    for (idx, (item, item_embedding)) in all_items_with_embeddings.iter().enumerate() {
        let progress = 0.85 + (0.10 * (idx as f32 / all_items_with_embeddings.len() as f32));

        // Extract topics for exclusion checking
        let topics = extract_topics(&item.title, &item.content);

        // Check exclusions FIRST (hard filter)
        let excluded_by = check_exclusions(&topics, &static_identity.exclusions);

        if let Some(ref exclusion) = excluded_by {
            excluded_count += 1;
            emit_progress(
                app,
                "relevance",
                progress,
                &format!(
                    "Excluded: {} ({})",
                    &item.title[..item.title.len().min(30)],
                    exclusion
                ),
                idx + 1,
                all_items_with_embeddings.len(),
            );

            results.push(HNRelevance {
                id: item.id,
                title: item.title.clone(),
                url: item.url.clone(),
                top_score: 0.0,
                matches: vec![],
                relevant: false,
                context_score: 0.0,
                interest_score: 0.0,
                excluded: true,
                excluded_by: Some(exclusion.clone()),
            });
            continue;
        }

        emit_progress(
            app,
            "relevance",
            progress,
            &format!("Scoring: {}", &item.title[..item.title.len().min(35)]),
            idx + 1,
            all_items_with_embeddings.len(),
        );

        // Compute context file score
        let mut matches: Vec<RelevanceMatch> = Vec::new();
        for chunk in &embedded_chunks {
            let similarity = cosine_similarity(item_embedding, &chunk.embedding);
            matches.push(RelevanceMatch {
                source_file: chunk.source_file.clone(),
                matched_text: if chunk.text.len() > 100 {
                    format!("{}...", &chunk.text[..100])
                } else {
                    chunk.text.clone()
                },
                similarity,
            });
        }
        matches.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        matches.truncate(3);

        let context_score = matches.first().map(|m| m.similarity).unwrap_or(0.0);

        // Compute interest score
        let interest_score = compute_interest_score(item_embedding, &static_identity.interests);

        // Combined score
        let combined_score = if interest_count > 0 {
            context_score * 0.5 + interest_score * 0.5
        } else {
            context_score
        };

        let relevant = combined_score >= RELEVANCE_THRESHOLD;

        results.push(HNRelevance {
            id: item.id,
            title: item.title.clone(),
            url: item.url.clone(),
            top_score: combined_score,
            matches,
            relevant,
            context_score,
            interest_score,
            excluded: false,
            excluded_by: None,
        });
    }

    // Sort: excluded items last, then by score
    results.sort_by(|a, b| {
        if a.excluded && !b.excluded {
            return std::cmp::Ordering::Greater;
        }
        if !a.excluded && b.excluded {
            return std::cmp::Ordering::Less;
        }
        b.top_score.partial_cmp(&a.top_score).unwrap()
    });

    if excluded_count > 0 {
        println!(
            "[4DA]   {} items excluded by user preferences",
            excluded_count
        );
    }

    // Step 6: LLM Re-ranking (if enabled)
    let settings_manager = get_settings_manager();
    let (rerank_enabled, llm_settings, rerank_config) = {
        let mut guard = settings_manager.lock();
        let is_enabled = guard.is_rerank_enabled() && guard.within_daily_limits();
        let llm = guard.get().llm.clone();
        let rerank = guard.get().rerank.clone();
        (is_enabled, llm, rerank)
    };

    if rerank_enabled {
        emit_progress(
            app,
            "rerank",
            0.92,
            "LLM re-ranking enabled, filtering candidates...",
            0,
            0,
        );

        // Get items that pass the minimum embedding score
        let candidates: Vec<&mut HNRelevance> = results
            .iter_mut()
            .filter(|r| r.top_score >= rerank_config.min_embedding_score)
            .take(rerank_config.max_items_per_batch)
            .collect();

        let candidate_count = candidates.len();

        if candidate_count > 0 {
            println!(
                "[4DA] LLM Re-ranking: {} candidates above {:.2} threshold",
                candidate_count, rerank_config.min_embedding_score
            );

            emit_progress(
                app,
                "rerank",
                0.93,
                &format!("Sending {} items to LLM for re-ranking...", candidate_count),
                0,
                candidate_count,
            );

            // Build comprehensive context summary from ALL context chunks
            // This gives the LLM a complete picture of user's interests
            let context_summary: String = embedded_chunks
                .iter()
                .map(|c| {
                    format!(
                        "[{}]\n{}",
                        c.source_file,
                        c.text.chars().take(600).collect::<String>()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n---\n\n");

            // Prepare items for LLM with more context
            // Include all top matches to give LLM full picture
            let items_for_llm: Vec<(String, String, String)> = candidates
                .iter()
                .map(|r| {
                    // Combine all match texts for richer context
                    let content_snippet = r
                        .matches
                        .iter()
                        .map(|m| format!("Matched '{}': {}", m.source_file, m.matched_text))
                        .collect::<Vec<_>>()
                        .join(" | ");
                    (r.id.to_string(), r.title.clone(), content_snippet)
                })
                .collect();

            // Create judge and run
            let judge = RelevanceJudge::new(llm_settings);

            match judge.judge_batch(&context_summary, items_for_llm).await {
                Ok((judgments, input_tokens, output_tokens)) => {
                    let cost_cents = judge.estimate_cost_cents(input_tokens, output_tokens);

                    println!(
                        "[4DA] LLM Re-ranking complete: {} judgments, {} tokens, ~{} cents",
                        judgments.len(),
                        input_tokens + output_tokens,
                        cost_cents
                    );

                    // Record usage
                    {
                        let mut guard = settings_manager.lock();
                        guard.record_usage(input_tokens + output_tokens, cost_cents);
                    }

                    // Apply LLM judgments with confidence threshold
                    // Only demote if LLM is confident (>= 0.7) to avoid over-filtering
                    const DEMOTION_CONFIDENCE_THRESHOLD: f32 = 0.7;
                    let mut llm_relevant_count = 0;
                    let mut demoted_count = 0;
                    let mut kept_by_low_confidence = 0;

                    let mut no_match_count = 0;
                    for result in results.iter_mut() {
                        let result_id_str = result.id.to_string();
                        if let Some(judgment) =
                            judgments.iter().find(|j| j.item_id == result_id_str)
                        {
                            if judgment.relevant {
                                // LLM confirms relevance
                                llm_relevant_count += 1;
                                println!(
                                    "[4DA]   LLM confirmed: {} (conf: {:.2})",
                                    &result.title[..result.title.len().min(40)],
                                    judgment.confidence
                                );
                            } else if result.relevant {
                                // LLM says not relevant - check confidence before demoting
                                if judgment.confidence >= DEMOTION_CONFIDENCE_THRESHOLD {
                                    println!(
                                        "[4DA]   LLM demoted: {} (conf: {:.2}) - {}",
                                        &result.title[..result.title.len().min(35)],
                                        judgment.confidence,
                                        &judgment.reasoning[..judgment.reasoning.len().min(50)]
                                    );
                                    result.relevant = false;
                                    demoted_count += 1;
                                } else {
                                    // Low confidence - keep as relevant (benefit of doubt)
                                    println!(
                                        "[4DA]   LLM uncertain, keeping: {} (conf: {:.2})",
                                        &result.title[..result.title.len().min(40)],
                                        judgment.confidence
                                    );
                                    llm_relevant_count += 1;
                                    kept_by_low_confidence += 1;
                                }
                            }
                        } else if result.relevant {
                            // No matching judgment found - item keeps embedding relevance
                            no_match_count += 1;
                            if no_match_count <= 3 {
                                println!(
                                    "[4DA]   No LLM judgment for: {} (id={})",
                                    &result.title[..result.title.len().min(40)],
                                    result_id_str
                                );
                            }
                        }
                    }

                    if no_match_count > 0 {
                        println!(
                            "[4DA] Warning: {} items had no matching LLM judgment",
                            no_match_count
                        );
                    }

                    println!(
                        "[4DA] LLM summary: {} confirmed, {} demoted, {} kept (low confidence)",
                        llm_relevant_count - kept_by_low_confidence,
                        demoted_count,
                        kept_by_low_confidence
                    );

                    emit_progress(
                        app,
                        "rerank",
                        0.98,
                        &format!(
                            "LLM kept {} of {} as relevant",
                            llm_relevant_count, candidate_count
                        ),
                        candidate_count,
                        candidate_count,
                    );
                }
                Err(e) => {
                    println!(
                        "[4DA] LLM Re-ranking failed: {}. Using embedding scores only.",
                        e
                    );
                    emit_progress(
                        app,
                        "rerank",
                        0.98,
                        "LLM re-ranking failed, using embeddings only",
                        0,
                        0,
                    );
                }
            }
        } else {
            println!("[4DA] LLM Re-ranking: No candidates above threshold, skipping");
            emit_progress(
                app,
                "rerank",
                0.98,
                "No candidates for LLM re-ranking",
                0,
                0,
            );
        }
    } else {
        println!("[4DA] LLM Re-ranking: Disabled or limit reached");
    }

    emit_progress(
        app,
        "complete",
        1.0,
        "Analysis complete!",
        results.len(),
        results.len(),
    );

    let relevant_count = results.iter().filter(|r| r.relevant && !r.excluded).count();
    let final_excluded = results.iter().filter(|r| r.excluded).count();
    println!("[4DA] ═══════════════════════════════════════════════════════════");
    println!("[4DA] PERSONALIZED ANALYSIS COMPLETE");
    println!(
        "[4DA] Total: {} | Relevant: {} | Excluded: {}",
        results.len(),
        relevant_count,
        final_excluded
    );
    println!(
        "[4DA] User context: {} interests, {} exclusions",
        interest_count, exclusion_count
    );
    println!("[4DA] ═══════════════════════════════════════════════════════════\n");

    Ok(results)
}

/// Get current analysis state
#[tauri::command]
async fn get_analysis_status() -> Result<AnalysisState, String> {
    let state = get_analysis_state();
    let guard = state.lock();
    Ok(guard.clone())
}

// ============================================================================
// Settings Commands
// ============================================================================

/// Get current settings
#[tauri::command]
async fn get_settings() -> Result<serde_json::Value, String> {
    let manager = get_settings_manager();
    let guard = manager.lock();
    let settings = guard.get();

    Ok(serde_json::json!({
        "llm": {
            "provider": settings.llm.provider,
            "model": settings.llm.model,
            "has_api_key": !settings.llm.api_key.is_empty(),
            "base_url": settings.llm.base_url
        },
        "rerank": {
            "enabled": settings.rerank.enabled,
            "max_items_per_batch": settings.rerank.max_items_per_batch,
            "min_embedding_score": settings.rerank.min_embedding_score,
            "daily_token_limit": settings.rerank.daily_token_limit,
            "daily_cost_limit_cents": settings.rerank.daily_cost_limit_cents
        },
        "usage": {
            "tokens_today": settings.usage.tokens_today,
            "cost_today_cents": settings.usage.cost_today_cents,
            "tokens_total": settings.usage.tokens_total,
            "items_reranked": settings.usage.items_reranked
        },
        "embedding_threshold": settings.embedding_threshold
    }))
}

/// Update LLM provider settings
#[tauri::command]
async fn set_llm_provider(
    provider: String,
    api_key: String,
    model: String,
    base_url: Option<String>,
) -> Result<(), String> {
    let manager = get_settings_manager();
    let mut guard = manager.lock();

    let llm_provider = LLMProvider {
        provider,
        api_key,
        model,
        base_url,
    };

    guard.set_llm_provider(llm_provider)?;
    println!("[4DA/Settings] LLM provider updated");
    Ok(())
}

/// Update re-ranking configuration
#[tauri::command]
async fn set_rerank_config(
    enabled: bool,
    max_items: usize,
    min_score: f32,
    daily_token_limit: u64,
    daily_cost_limit: u64,
) -> Result<(), String> {
    let manager = get_settings_manager();
    let mut guard = manager.lock();

    let config = RerankConfig {
        enabled,
        max_items_per_batch: max_items,
        min_embedding_score: min_score,
        daily_token_limit,
        daily_cost_limit_cents: daily_cost_limit,
    };

    guard.set_rerank_config(config)?;
    println!(
        "[4DA/Settings] Re-rank config updated. Enabled: {}",
        enabled
    );
    Ok(())
}

/// Test LLM connection
#[tauri::command]
async fn test_llm_connection() -> Result<serde_json::Value, String> {
    let manager = get_settings_manager();
    let settings = {
        let guard = manager.lock();
        guard.get().clone()
    };

    if settings.llm.provider == "none" || settings.llm.api_key.is_empty() {
        return Err("No LLM provider configured".to_string());
    }

    println!(
        "[4DA] Testing LLM connection to {}...",
        settings.llm.provider
    );

    let judge = RelevanceJudge::new(settings.llm.clone());

    // Simple test - ask for a short response
    let test_items = vec![(
        "test".to_string(),
        "Test Item".to_string(),
        "This is a test.".to_string(),
    )];

    match judge
        .judge_batch("User is testing the connection.", test_items)
        .await
    {
        Ok((_, input_tokens, output_tokens)) => {
            let cost = judge.estimate_cost_cents(input_tokens, output_tokens);
            println!(
                "[4DA] LLM test successful! Tokens: {} in, {} out, ~{} cents",
                input_tokens, output_tokens, cost
            );

            Ok(serde_json::json!({
                "success": true,
                "input_tokens": input_tokens,
                "output_tokens": output_tokens,
                "cost_cents": cost,
                "message": format!("Connection successful! Test used {} tokens.", input_tokens + output_tokens)
            }))
        }
        Err(e) => {
            println!("[4DA] LLM test failed: {}", e);
            Err(format!("Connection failed: {}", e))
        }
    }
}

/// Get usage statistics
#[tauri::command]
async fn get_usage_stats() -> Result<serde_json::Value, String> {
    let manager = get_settings_manager();
    let mut guard = manager.lock();
    let within_limits = guard.within_daily_limits();
    let summary = guard.usage_summary();
    let settings = guard.get();

    Ok(serde_json::json!({
        "tokens_today": settings.usage.tokens_today,
        "cost_today_cents": settings.usage.cost_today_cents,
        "tokens_total": settings.usage.tokens_total,
        "items_reranked": settings.usage.items_reranked,
        "daily_token_limit": settings.rerank.daily_token_limit,
        "daily_cost_limit_cents": settings.rerank.daily_cost_limit_cents,
        "within_limits": within_limits,
        "summary": summary
    }))
}

// ============================================================================
// Monitoring Commands (Phase 3)
// ============================================================================

/// Get monitoring status
#[tauri::command]
async fn get_monitoring_status() -> Result<serde_json::Value, String> {
    let state = get_monitoring_state();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let last_check = state.last_check.load(std::sync::atomic::Ordering::Relaxed);
    let secs_since_check = if last_check > 0 { now - last_check } else { 0 };

    Ok(serde_json::json!({
        "enabled": state.is_enabled(),
        "interval_secs": state.get_interval(),
        "interval_mins": state.get_interval() / 60,
        "is_checking": state.is_checking.load(std::sync::atomic::Ordering::Relaxed),
        "last_check": last_check,
        "secs_since_check": secs_since_check,
        "last_relevant_count": state.last_relevant_count.load(std::sync::atomic::Ordering::Relaxed),
        "total_checks": state.total_checks.load(std::sync::atomic::Ordering::Relaxed)
    }))
}

/// Enable or disable monitoring
#[tauri::command]
async fn set_monitoring_enabled(enabled: bool) -> Result<serde_json::Value, String> {
    let state = get_monitoring_state();
    state.set_enabled(enabled);

    if enabled {
        // Set last_check to 0 to trigger immediate check
        state
            .last_check
            .store(0, std::sync::atomic::Ordering::Relaxed);
    }

    // Persist to settings
    {
        let mut settings = get_settings_manager().lock();
        let interval = state.get_interval() / 60;
        let _ = settings.set_monitoring_config(settings::MonitoringConfig {
            enabled,
            interval_minutes: interval,
        });
    }

    println!(
        "[4DA/Monitor] Monitoring {} (persisted)",
        if enabled { "enabled" } else { "disabled" }
    );

    Ok(serde_json::json!({
        "enabled": enabled,
        "message": if enabled { "Monitoring started" } else { "Monitoring stopped" }
    }))
}

/// Set monitoring interval
#[tauri::command]
async fn set_monitoring_interval(minutes: u64) -> Result<serde_json::Value, String> {
    let state = get_monitoring_state();
    let secs = minutes * 60;
    state.set_interval(secs);

    // Persist to settings
    {
        let mut settings = get_settings_manager().lock();
        let _ = settings.set_monitoring_config(settings::MonitoringConfig {
            enabled: state.is_enabled(),
            interval_minutes: minutes,
        });
    }

    println!(
        "[4DA/Monitor] Interval set to {} minutes (persisted)",
        minutes
    );

    Ok(serde_json::json!({
        "interval_mins": minutes,
        "interval_secs": secs
    }))
}

/// Test notification delivery
#[tauri::command]
async fn trigger_notification_test(app: AppHandle) -> Result<serde_json::Value, String> {
    monitoring::send_notification(&app, 3, 30);
    Ok(serde_json::json!({
        "success": true,
        "message": "Test notification sent"
    }))
}

// ============================================================================
// Context Engine Commands
// ============================================================================

/// Get the user's static identity (interests, exclusions, role, etc.)
#[tauri::command]
async fn get_user_context() -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;

    let identity = engine
        .get_static_identity()
        .map_err(|e| format!("Failed to get identity: {}", e))?;

    let interest_count = engine.interest_count().unwrap_or(0);
    let exclusion_count = engine.exclusion_count().unwrap_or(0);

    Ok(serde_json::json!({
        "role": identity.role,
        "tech_stack": identity.tech_stack,
        "domains": identity.domains,
        "interests": identity.interests.iter().map(|i| serde_json::json!({
            "id": i.id,
            "topic": i.topic,
            "weight": i.weight,
            "source": i.source,
            "has_embedding": i.embedding.is_some()
        })).collect::<Vec<_>>(),
        "exclusions": identity.exclusions,
        "stats": {
            "interest_count": interest_count,
            "exclusion_count": exclusion_count
        }
    }))
}

/// Set the user's role
#[tauri::command]
async fn set_user_role(role: Option<String>) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;
    engine
        .set_role(role.as_deref())
        .map_err(|e| format!("Failed to set role: {}", e))?;

    println!("[4DA/Context] Role updated to: {:?}", role);

    Ok(serde_json::json!({
        "success": true,
        "role": role
    }))
}

/// Add a technology to the user's tech stack
#[tauri::command]
async fn add_tech_stack(technology: String) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;
    engine
        .add_technology(&technology)
        .map_err(|e| format!("Failed to add technology: {}", e))?;

    println!("[4DA/Context] Added technology: {}", technology);

    Ok(serde_json::json!({
        "success": true,
        "technology": technology
    }))
}

/// Remove a technology from the user's tech stack
#[tauri::command]
async fn remove_tech_stack(technology: String) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;
    engine
        .remove_technology(&technology)
        .map_err(|e| format!("Failed to remove technology: {}", e))?;

    println!("[4DA/Context] Removed technology: {}", technology);

    Ok(serde_json::json!({
        "success": true
    }))
}

/// Add a domain of interest
#[tauri::command]
async fn add_domain(domain: String) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;
    engine
        .add_domain(&domain)
        .map_err(|e| format!("Failed to add domain: {}", e))?;

    println!("[4DA/Context] Added domain: {}", domain);

    Ok(serde_json::json!({
        "success": true,
        "domain": domain
    }))
}

/// Remove a domain of interest
#[tauri::command]
async fn remove_domain(domain: String) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;
    engine
        .remove_domain(&domain)
        .map_err(|e| format!("Failed to remove domain: {}", e))?;

    println!("[4DA/Context] Removed domain: {}", domain);

    Ok(serde_json::json!({
        "success": true
    }))
}

/// Add an explicit interest (with embedding generation)
#[tauri::command]
async fn add_interest(topic: String, weight: Option<f32>) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;
    let weight = weight.unwrap_or(1.0);

    // Generate embedding for the topic
    let embedding = embed_texts(&[topic.clone()])?;
    let emb = embedding.first().map(|e| e.as_slice());

    let id = engine
        .add_interest(&topic, weight, emb, InterestSource::Explicit)
        .map_err(|e| format!("Failed to add interest: {}", e))?;

    println!(
        "[4DA/Context] Added interest: {} (weight: {}, embedding: {})",
        topic,
        weight,
        emb.is_some()
    );

    Ok(serde_json::json!({
        "success": true,
        "id": id,
        "topic": topic,
        "weight": weight,
        "has_embedding": emb.is_some()
    }))
}

/// Remove an interest
#[tauri::command]
async fn remove_interest(topic: String) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;
    engine
        .remove_interest(&topic)
        .map_err(|e| format!("Failed to remove interest: {}", e))?;

    println!("[4DA/Context] Removed interest: {}", topic);

    Ok(serde_json::json!({
        "success": true
    }))
}

/// Add an exclusion (topic to never show)
#[tauri::command]
async fn add_exclusion(topic: String) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;
    engine
        .add_exclusion(&topic)
        .map_err(|e| format!("Failed to add exclusion: {}", e))?;

    println!("[4DA/Context] Added exclusion: {}", topic);

    Ok(serde_json::json!({
        "success": true,
        "topic": topic
    }))
}

/// Remove an exclusion
#[tauri::command]
async fn remove_exclusion(topic: String) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;
    engine
        .remove_exclusion(&topic)
        .map_err(|e| format!("Failed to remove exclusion: {}", e))?;

    println!("[4DA/Context] Removed exclusion: {}", topic);

    Ok(serde_json::json!({
        "success": true
    }))
}

/// Record a user interaction (click, save, dismiss)
#[tauri::command]
async fn record_interaction(
    source_item_id: i64,
    action: String,
) -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;

    let action_type = match action.to_lowercase().as_str() {
        "click" => InteractionType::Click,
        "save" => InteractionType::Save,
        "dismiss" => InteractionType::Dismiss,
        "ignore" => InteractionType::Ignore,
        _ => return Err(format!("Unknown action type: {}", action)),
    };

    engine
        .record_interaction(source_item_id, action_type)
        .map_err(|e| format!("Failed to record interaction: {}", e))?;

    println!(
        "[4DA/Context] Recorded {} for item {}",
        action, source_item_id
    );

    Ok(serde_json::json!({
        "success": true
    }))
}

/// Get context engine statistics
#[tauri::command]
async fn get_context_stats() -> Result<serde_json::Value, String> {
    let engine = get_context_engine()?;

    let interest_count = engine.interest_count().unwrap_or(0);
    let exclusion_count = engine.exclusion_count().unwrap_or(0);

    let identity = engine
        .get_static_identity()
        .map_err(|e| format!("Failed to get identity: {}", e))?;

    Ok(serde_json::json!({
        "interests": interest_count,
        "exclusions": exclusion_count,
        "tech_stack": identity.tech_stack.len(),
        "domains": identity.domains.len(),
        "has_role": identity.role.is_some()
    }))
}

// ============================================================================
// ACE (Autonomous Context Engine) Commands
// ============================================================================

/// Trigger autonomous context detection
/// Scans specified paths for project manifests and extracts tech stack
#[tauri::command]
async fn ace_detect_context(paths: Vec<String>) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;

    // Convert string paths to PathBuf, expanding ~ to home directory
    let scan_paths: Vec<PathBuf> = paths
        .iter()
        .map(|p| {
            if p.starts_with("~") {
                if let Some(home) = dirs::home_dir() {
                    home.join(&p[2..]) // Skip "~/"
                } else {
                    PathBuf::from(p)
                }
            } else {
                PathBuf::from(p)
            }
        })
        .collect();

    // If no paths provided, use default locations
    let paths_to_scan = if scan_paths.is_empty() {
        get_default_scan_paths()
    } else {
        scan_paths
    };

    let context = ace.detect_context(&paths_to_scan)?;

    Ok(serde_json::json!({
        "success": true,
        "detected_tech": context.detected_tech,
        "active_topics": context.active_topics,
        "projects_scanned": context.projects_scanned,
        "context_confidence": context.context_confidence,
        "detection_time": context.detection_time
    }))
}

/// Get detected technologies from ACE
#[tauri::command]
async fn ace_get_detected_tech() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let tech = ace.get_detected_tech()?;

    Ok(serde_json::json!({
        "technologies": tech
    }))
}

/// Get active topics from ACE
#[tauri::command]
async fn ace_get_active_topics() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let topics = ace.get_active_topics()?;

    Ok(serde_json::json!({
        "topics": topics
    }))
}

/// Get ACE system health status
#[tauri::command]
async fn ace_get_health() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let health = ace.get_health();

    Ok(serde_json::json!({
        "project_scanner": health.project_scanner,
        "file_watcher": health.file_watcher,
        "git_analyzer": health.git_analyzer,
        "behavior_learner": health.behavior_learner,
        "overall_status": health.overall_status,
        "context_quality": health.context_quality
    }))
}

/// Get default paths to scan for projects
fn get_default_scan_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(home) = dirs::home_dir() {
        // Common project locations
        let candidates = [
            "projects",
            "code",
            "dev",
            "src",
            "Documents/GitHub",
            "repos",
            "workspace",
            "work",
        ];

        for candidate in candidates {
            let path = home.join(candidate);
            if path.exists() {
                paths.push(path);
            }
        }
    }

    // Also check current working directory parent (for dev scenarios)
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(parent) = cwd.parent() {
            if !paths.contains(&parent.to_path_buf()) {
                paths.push(parent.to_path_buf());
            }
        }
    }

    paths
}

// ============================================================================
// ACE Phase B: Real-Time Context Commands
// ============================================================================

/// Analyze git repositories for context extraction
#[tauri::command]
async fn ace_analyze_git(paths: Vec<String>) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;

    let scan_paths: Vec<PathBuf> = if paths.is_empty() {
        get_default_scan_paths()
    } else {
        paths
            .iter()
            .map(|p| {
                if p.starts_with("~") {
                    if let Some(home) = dirs::home_dir() {
                        home.join(&p[2..])
                    } else {
                        PathBuf::from(p)
                    }
                } else {
                    PathBuf::from(p)
                }
            })
            .collect()
    };

    let signals = ace.analyze_git_repos(&scan_paths)?;

    Ok(serde_json::json!({
        "success": true,
        "repos_analyzed": signals.len(),
        "signals": signals.iter().map(|s| serde_json::json!({
            "repo_name": s.repo_name,
            "repo_path": s.repo_path,
            "commit_count": s.recent_commits.len(),
            "branch_count": s.active_branches.len(),
            "commit_frequency": s.commit_frequency,
            "topics": s.extracted_topics,
            "confidence": s.confidence
        })).collect::<Vec<_>>()
    }))
}

/// Get real-time context (active topics + detected tech)
#[tauri::command]
async fn ace_get_realtime_context() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;

    // Get the connection from ACE (we need to access it)
    let mut db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    db_path.pop();
    db_path.push("data");
    db_path.push("4da.db");

    let conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    let conn = Arc::new(parking_lot::Mutex::new(conn));

    let context = ace::get_realtime_context(&conn)?;

    Ok(serde_json::json!({
        "active_topics": context.active_topics,
        "detected_tech": context.detected_tech,
        "context_confidence": context.context_confidence,
        "last_updated": context.last_updated
    }))
}

/// Apply freshness decay to active topics
#[tauri::command]
async fn ace_apply_decay() -> Result<serde_json::Value, String> {
    let mut db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    db_path.pop();
    db_path.push("data");
    db_path.push("4da.db");

    let conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    let conn = Arc::new(parking_lot::Mutex::new(conn));

    let updated = ace::apply_freshness_decay(&conn)?;

    Ok(serde_json::json!({
        "success": true,
        "topics_updated": updated
    }))
}

/// Run full autonomous context detection (manifests + git)
#[tauri::command]
async fn ace_full_scan(paths: Vec<String>) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;

    let scan_paths: Vec<PathBuf> = if paths.is_empty() {
        get_default_scan_paths()
    } else {
        paths
            .iter()
            .map(|p| {
                if p.starts_with("~") {
                    if let Some(home) = dirs::home_dir() {
                        home.join(&p[2..])
                    } else {
                        PathBuf::from(p)
                    }
                } else {
                    PathBuf::from(p)
                }
            })
            .collect()
    };

    println!("[ACE] Starting full scan of {} paths...", scan_paths.len());

    // Phase 1: Manifest scanning
    let manifest_context = ace.detect_context(&scan_paths)?;

    // Phase 2: Git analysis
    let git_signals = ace.analyze_git_repos(&scan_paths)?;

    // Combine results
    let total_topics: std::collections::HashSet<String> = manifest_context
        .active_topics
        .iter()
        .map(|t| t.topic.clone())
        .chain(git_signals.iter().flat_map(|s| s.extracted_topics.clone()))
        .collect();

    println!(
        "[ACE] Full scan complete: {} tech, {} topics, {} git repos",
        manifest_context.detected_tech.len(),
        total_topics.len(),
        git_signals.len()
    );

    Ok(serde_json::json!({
        "success": true,
        "manifest_scan": {
            "projects_scanned": manifest_context.projects_scanned,
            "detected_tech": manifest_context.detected_tech.len(),
            "confidence": manifest_context.context_confidence
        },
        "git_scan": {
            "repos_analyzed": git_signals.len(),
            "total_commits": git_signals.iter().map(|s| s.recent_commits.len()).sum::<usize>()
        },
        "combined": {
            "total_topics": total_topics.len(),
            "topics": total_topics.into_iter().collect::<Vec<_>>()
        }
    }))
}

// ============================================================================
// ACE Phase C: Behavior Learning Commands
// ============================================================================

/// Record a user interaction for behavior learning
#[tauri::command]
async fn ace_record_interaction(
    item_id: i64,
    action_type: String,
    action_data: Option<serde_json::Value>,
    item_topics: Vec<String>,
    item_source: String,
) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;

    // Parse action type into BehaviorAction
    let action = match action_type.as_str() {
        "click" => {
            let dwell_time = action_data
                .and_then(|d| d.get("dwell_time_seconds").and_then(|v| v.as_u64()))
                .unwrap_or(0);
            ace::BehaviorAction::Click {
                dwell_time_seconds: dwell_time,
            }
        }
        "save" => ace::BehaviorAction::Save,
        "share" => ace::BehaviorAction::Share,
        "dismiss" => ace::BehaviorAction::Dismiss,
        "mark_irrelevant" => ace::BehaviorAction::MarkIrrelevant,
        "scroll" => {
            let visible_seconds = action_data
                .and_then(|d| d.get("visible_seconds").and_then(|v| v.as_f64()))
                .unwrap_or(0.0) as f32;
            ace::BehaviorAction::Scroll { visible_seconds }
        }
        "ignore" => ace::BehaviorAction::Ignore,
        _ => return Err(format!("Unknown action type: {}", action_type)),
    };

    ace.record_interaction(
        item_id,
        action.clone(),
        item_topics.clone(),
        item_source.clone(),
    )?;

    Ok(serde_json::json!({
        "success": true,
        "recorded": {
            "item_id": item_id,
            "action": action_type,
            "topics": item_topics,
            "source": item_source
        }
    }))
}

/// Get learned topic affinities
#[tauri::command]
async fn ace_get_topic_affinities() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let affinities = ace.get_topic_affinities()?;

    Ok(serde_json::json!({
        "affinities": affinities,
        "count": affinities.len()
    }))
}

/// Get detected anti-topics
#[tauri::command]
async fn ace_get_anti_topics(min_rejections: Option<u32>) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let threshold = min_rejections.unwrap_or(5);
    let anti_topics = ace.get_anti_topics(threshold)?;

    Ok(serde_json::json!({
        "anti_topics": anti_topics,
        "count": anti_topics.len(),
        "threshold": threshold
    }))
}

/// Confirm an auto-detected anti-topic
#[tauri::command]
async fn ace_confirm_anti_topic(topic: String) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    ace.confirm_anti_topic(&topic)?;

    Ok(serde_json::json!({
        "success": true,
        "confirmed": topic
    }))
}

/// Get behavior modifier for an item
#[tauri::command]
async fn ace_get_behavior_modifier(
    topics: Vec<String>,
    source: String,
) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let modifier = ace.get_behavior_modifier(&topics, &source)?;

    Ok(serde_json::json!({
        "modifier": modifier,
        "topics": topics,
        "source": source
    }))
}

/// Get full learned behavior summary
#[tauri::command]
async fn ace_get_learned_behavior() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let summary = ace.get_learned_behavior()?;

    Ok(serde_json::json!(summary))
}

/// Apply temporal decay to behavior learning
#[tauri::command]
async fn ace_apply_behavior_decay() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let updated = ace.apply_behavior_decay()?;

    Ok(serde_json::json!({
        "success": true,
        "affinities_updated": updated
    }))
}

// ============================================================================
// ACE Phase D: Health Monitoring & Validation Commands
// ============================================================================

/// Perform a complete health check
#[tauri::command]
async fn ace_check_health() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let snapshot = ace.check_health();

    Ok(serde_json::json!(snapshot))
}

/// Get current system status (health + accuracy + alerts)
#[tauri::command]
async fn ace_get_system_status() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let status = ace.get_system_status();

    Ok(serde_json::json!(status))
}

/// Get current fallback level
#[tauri::command]
async fn ace_get_fallback_level() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let level = ace.get_fallback_level();
    let features = ace.get_available_features();

    Ok(serde_json::json!({
        "level": level,
        "available_features": features
    }))
}

/// Get active health alerts
#[tauri::command]
async fn ace_get_alerts() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let alerts = ace.get_health_alerts();

    Ok(serde_json::json!({
        "alerts": alerts,
        "count": alerts.len()
    }))
}

/// Get recent audit entries
#[tauri::command]
async fn ace_get_audit_log(
    entry_type: Option<String>,
    limit: Option<usize>,
) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let limit = limit.unwrap_or(50);

    let type_filter = entry_type.map(|t| match t.as_str() {
        "context_update" => ace::AuditEntryType::ContextUpdate,
        "relevance_decision" => ace::AuditEntryType::RelevanceDecision,
        "exclusion_applied" => ace::AuditEntryType::ExclusionApplied,
        "feedback_received" => ace::AuditEntryType::FeedbackReceived,
        "anomaly_detected" => ace::AuditEntryType::AnomalyDetected,
        "fallback_activated" => ace::AuditEntryType::FallbackActivated,
        "health_check" => ace::AuditEntryType::HealthCheck,
        _ => ace::AuditEntryType::ConfigChange,
    });

    let entries = ace.query_audit_log(type_filter, limit)?;

    Ok(serde_json::json!({
        "entries": entries,
        "count": entries.len()
    }))
}

/// Explain a relevance decision for an item
#[tauri::command]
async fn ace_explain_decision(item_id: i64) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let explanation = ace.explain_decision(item_id)?;

    Ok(serde_json::json!({
        "item_id": item_id,
        "explanation": explanation
    }))
}

/// Get current accuracy metrics
#[tauri::command]
async fn ace_get_accuracy_metrics() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let metrics = ace.get_accuracy_metrics();

    Ok(serde_json::json!(metrics))
}

/// Get accuracy history
#[tauri::command]
async fn ace_get_accuracy_history(days: Option<u32>) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let days = days.unwrap_or(30);
    let history = ace.get_accuracy_history(days)?;

    Ok(serde_json::json!({
        "history": history,
        "days": days
    }))
}

/// Record accuracy feedback
#[tauri::command]
async fn ace_record_accuracy_feedback(
    item_id: i64,
    predicted_score: f32,
    feedback_type: String,
) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;

    let feedback = match feedback_type.as_str() {
        "click" => ace::FeedbackType::Click,
        "save" => ace::FeedbackType::Save,
        "share" => ace::FeedbackType::Share,
        "thumbs_up" => ace::FeedbackType::ThumbsUp,
        "thumbs_down" => ace::FeedbackType::ThumbsDown,
        "dismiss" => ace::FeedbackType::Dismiss,
        "ignore" => ace::FeedbackType::Ignore,
        _ => return Err(format!("Unknown feedback type: {}", feedback_type)),
    };

    ace.record_accuracy_feedback(ace::FeedbackResult {
        item_id,
        predicted_score,
        feedback,
        timestamp: chrono::Utc::now().to_rfc3339(),
    });

    Ok(serde_json::json!({
        "success": true,
        "recorded": {
            "item_id": item_id,
            "feedback": feedback_type
        }
    }))
}

/// Persist accuracy metrics to database
#[tauri::command]
async fn ace_persist_accuracy() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    ace.persist_accuracy_metrics()?;

    Ok(serde_json::json!({
        "success": true
    }))
}

/// Check if accuracy targets are met
#[tauri::command]
async fn ace_check_accuracy_targets() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let meets_targets = ace.meets_accuracy_targets();
    let metrics = ace.get_accuracy_metrics();

    Ok(serde_json::json!({
        "meets_targets": meets_targets,
        "targets": {
            "min_precision": 0.85,
            "min_engagement": 0.30,
            "max_calibration_error": 0.10
        },
        "current": metrics
    }))
}

// ============================================================================
// ACE Phase E: Anomaly Detection Commands
// ============================================================================

/// Detect anomalies in context data
#[tauri::command]
async fn ace_detect_anomalies() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let anomalies = ace.detect_anomalies();

    Ok(serde_json::json!({
        "anomalies": anomalies,
        "count": anomalies.len()
    }))
}

/// Get unresolved anomalies
#[tauri::command]
async fn ace_get_unresolved_anomalies() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let anomalies = ace.get_unresolved_anomalies()?;

    Ok(serde_json::json!({
        "anomalies": anomalies,
        "count": anomalies.len()
    }))
}

/// Resolve an anomaly
#[tauri::command]
async fn ace_resolve_anomaly(anomaly_id: i64) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    ace.resolve_anomaly(anomaly_id)?;

    Ok(serde_json::json!({
        "resolved": true,
        "anomaly_id": anomaly_id
    }))
}

/// Get recent anomalies
#[tauri::command]
async fn ace_get_recent_anomalies(limit: Option<usize>) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let limit = limit.unwrap_or(10);
    let anomalies = ace.get_recent_anomalies(limit);

    Ok(serde_json::json!({
        "anomalies": anomalies,
        "count": anomalies.len()
    }))
}

// ============================================================================
// ACE Phase E: Embedding Commands
// ============================================================================

/// Get embedding for a topic
#[tauri::command]
async fn ace_embed_topic(topic: String) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let embedding = ace.embed_topic(&topic)?;

    Ok(serde_json::json!({
        "topic": topic,
        "embedding": embedding,
        "dimension": embedding.len()
    }))
}

/// Find similar topics using embeddings
#[tauri::command]
async fn ace_find_similar_topics(
    query: String,
    top_k: Option<usize>,
) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let top_k = top_k.unwrap_or(5);
    let results = ace.find_similar_topics(&query, top_k)?;

    Ok(serde_json::json!({
        "query": query,
        "results": results.iter().map(|(topic, score)| {
            serde_json::json!({
                "topic": topic,
                "similarity": score
            })
        }).collect::<Vec<_>>()
    }))
}

/// Check if embedding service is operational
#[tauri::command]
async fn ace_embedding_status() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let operational = ace.is_embedding_operational();

    Ok(serde_json::json!({
        "operational": operational
    }))
}

// ============================================================================
// ACE Phase E: Watcher Persistence Commands
// ============================================================================

/// Save watcher state for persistence
#[tauri::command]
async fn ace_save_watcher_state() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    ace.save_watcher_state()?;

    Ok(serde_json::json!({
        "saved": true
    }))
}

/// Restore watcher state from persistence
/// Note: This returns the saved state info. Actual restoration happens on app restart.
#[tauri::command]
async fn ace_get_watcher_state() -> Result<serde_json::Value, String> {
    // Watcher state is restored automatically on ACE initialization
    // This command provides info about the current state
    Ok(serde_json::json!({
        "info": "Watcher state is restored automatically on app startup. Use ace_save_watcher_state to persist current state."
    }))
}

/// Clear watcher state
#[tauri::command]
async fn ace_clear_watcher_state() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    ace.clear_watcher_state()?;

    Ok(serde_json::json!({
        "cleared": true
    }))
}

// ============================================================================
// ACE Phase E: Rate Limiting Commands
// ============================================================================

/// Get rate limit status for a source
#[tauri::command]
async fn ace_get_rate_limit_status(source: String) -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let status = ace.get_rate_limit_status(&source);

    Ok(serde_json::json!(status))
}

// ============================================================================
// ACE Watcher Control Commands
// ============================================================================

/// Start file watching on specified directories
#[tauri::command]
async fn ace_start_watcher(paths: Vec<String>) -> Result<serde_json::Value, String> {
    let mut ace = get_ace_engine_mut()?;

    let watch_paths: Vec<PathBuf> = paths
        .iter()
        .map(|p| {
            if p.starts_with("~") {
                if let Some(home) = dirs::home_dir() {
                    home.join(&p[2..])
                } else {
                    PathBuf::from(p)
                }
            } else {
                PathBuf::from(p)
            }
        })
        .filter(|p| p.exists())
        .collect();

    if watch_paths.is_empty() {
        return Ok(serde_json::json!({
            "success": false,
            "error": "No valid paths to watch",
            "watching": 0
        }));
    }

    ace.start_watching(&watch_paths)?;

    Ok(serde_json::json!({
        "success": true,
        "watching": watch_paths.len(),
        "paths": watch_paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>()
    }))
}

/// Stop file watching
#[tauri::command]
async fn ace_stop_watcher() -> Result<serde_json::Value, String> {
    let mut ace = get_ace_engine_mut()?;
    ace.stop_watching();

    Ok(serde_json::json!({
        "success": true,
        "watching": false
    }))
}

/// Check if watcher is active
#[tauri::command]
async fn ace_is_watching() -> Result<serde_json::Value, String> {
    let ace = get_ace_engine()?;
    let watching = ace.is_watching();

    Ok(serde_json::json!({
        "watching": watching
    }))
}

// ============================================================================
// Startup Initialization
// ============================================================================

/// Initialize ACE on startup with configured directories
fn initialize_ace_on_startup(app_handle: tauri::AppHandle) {
    // Get configured context directories
    let context_dirs = get_context_dirs();

    if context_dirs.is_empty() {
        println!("[4DA/Startup] No context directories configured, skipping ACE initialization");
        return;
    }

    println!(
        "[4DA/Startup] Found {} configured directories, initializing ACE...",
        context_dirs.len()
    );

    // Spawn async task for ACE initialization
    tauri::async_runtime::spawn(async move {
        // Small delay to let the app fully initialize
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        let paths: Vec<String> = context_dirs
            .iter()
            .map(|p| p.display().to_string())
            .collect();

        // Run full scan
        println!("[4DA/Startup] Running ACE full scan...");
        match ace_full_scan(paths.clone()).await {
            Ok(result) => {
                println!("[4DA/Startup] ACE scan complete: {}", result);
                // Emit event to frontend
                let _ = app_handle.emit("ace-scan-complete", result);
            }
            Err(e) => {
                println!("[4DA/Startup] ACE scan failed: {}", e);
            }
        }

        // Start file watcher
        println!("[4DA/Startup] Starting FileWatcher...");
        match ace_start_watcher(paths).await {
            Ok(result) => {
                println!("[4DA/Startup] FileWatcher started: {}", result);
                let _ = app_handle.emit("ace-watcher-started", result);
            }
            Err(e) => {
                println!("[4DA/Startup] FileWatcher failed: {}", e);
            }
        }
    });
}
