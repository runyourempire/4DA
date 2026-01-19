//! Reddit source implementation
//!
//! Fetches top posts from tech-related subreddits using Reddit's JSON API.

use async_trait::async_trait;
use serde::Deserialize;

use super::{Source, SourceConfig, SourceError, SourceItem, SourceResult};

// ============================================================================
// Reddit API Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct RedditListing {
    data: RedditListingData,
}

#[derive(Debug, Deserialize)]
struct RedditListingData {
    children: Vec<RedditChild>,
}

#[derive(Debug, Deserialize)]
struct RedditChild {
    data: RedditPost,
}

#[derive(Debug, Deserialize)]
struct RedditPost {
    id: String,
    title: String,
    selftext: Option<String>,
    url: Option<String>,
    permalink: String,
    score: i32,
    author: String,
    subreddit: String,
    num_comments: i32,
    is_self: bool,
}

// ============================================================================
// Reddit Source
// ============================================================================

/// Reddit source - fetches top posts from tech subreddits
pub struct RedditSource {
    config: SourceConfig,
    client: reqwest::Client,
    subreddits: Vec<&'static str>,
}

impl RedditSource {
    /// Create a new Reddit source with default config
    pub fn new() -> Self {
        Self {
            config: SourceConfig {
                enabled: true,
                max_items: 30,
                fetch_interval_secs: 600, // 10 minutes
                custom: None,
            },
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("4DA/0.1 (Ambient Intelligence)")
                .build()
                .expect("Failed to create HTTP client"),
            subreddits: vec![
                "programming",
                "technology",
                "machinelearning",
                "rust",
                "typescript",
                "webdev",
                "datascience",
            ],
        }
    }

    /// Fetch posts from a single subreddit
    async fn fetch_subreddit(
        &self,
        subreddit: &str,
        limit: usize,
    ) -> SourceResult<Vec<SourceItem>> {
        let url = format!(
            "https://www.reddit.com/r/{}/hot.json?limit={}",
            subreddit, limit
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| SourceError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SourceError::Network(format!(
                "Reddit API error: HTTP {}",
                response.status()
            )));
        }

        let listing: RedditListing = response
            .json()
            .await
            .map_err(|e| SourceError::Parse(e.to_string()))?;

        let items: Vec<SourceItem> = listing
            .data
            .children
            .into_iter()
            .map(|child| {
                let post = child.data;
                let content = post.selftext.unwrap_or_default();

                // For link posts, use the linked URL; for self posts, use Reddit URL
                let url = if post.is_self {
                    format!("https://reddit.com{}", post.permalink)
                } else {
                    post.url
                        .unwrap_or_else(|| format!("https://reddit.com{}", post.permalink))
                };

                SourceItem::new("reddit", &post.id, &post.title)
                    .with_url(Some(url))
                    .with_content(content)
                    .with_metadata(serde_json::json!({
                        "score": post.score,
                        "author": post.author,
                        "subreddit": post.subreddit,
                        "comments": post.num_comments,
                        "is_self": post.is_self,
                    }))
            })
            .collect();

        Ok(items)
    }
}

impl Default for RedditSource {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Source for RedditSource {
    fn source_type(&self) -> &'static str {
        "reddit"
    }

    fn name(&self) -> &'static str {
        "Reddit"
    }

    fn config(&self) -> &SourceConfig {
        &self.config
    }

    fn set_config(&mut self, config: SourceConfig) {
        self.config = config;
    }

    async fn fetch_items(&self) -> SourceResult<Vec<SourceItem>> {
        if !self.config.enabled {
            return Err(SourceError::Disabled);
        }

        println!(
            "[4DA/Reddit] Fetching from {} subreddits...",
            self.subreddits.len()
        );

        let mut all_items = Vec::new();
        let items_per_sub = (self.config.max_items / self.subreddits.len()).max(5);

        for subreddit in &self.subreddits {
            match self.fetch_subreddit(subreddit, items_per_sub).await {
                Ok(items) => {
                    println!(
                        "[4DA/Reddit] Got {} posts from r/{}",
                        items.len(),
                        subreddit
                    );
                    all_items.extend(items);
                }
                Err(e) => {
                    println!("[4DA/Reddit] Failed to fetch r/{}: {:?}", subreddit, e);
                }
            }

            // Small delay between subreddits to be respectful
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        // Sort by score and take top items
        all_items.sort_by(|a, b| {
            let score_a = a
                .metadata
                .as_ref()
                .and_then(|m| m.get("score"))
                .and_then(|s| s.as_i64())
                .unwrap_or(0);
            let score_b = b
                .metadata
                .as_ref()
                .and_then(|m| m.get("score"))
                .and_then(|s| s.as_i64())
                .unwrap_or(0);
            score_b.cmp(&score_a)
        });

        all_items.truncate(self.config.max_items);

        println!(
            "[4DA/Reddit] Total: {} posts across all subreddits",
            all_items.len()
        );
        Ok(all_items)
    }

    async fn scrape_content(&self, item: &SourceItem) -> SourceResult<String> {
        // Self posts already have content
        if !item.content.is_empty() {
            return Ok(item.content.clone());
        }

        // For link posts, we could scrape the linked page
        // but for now, just return the title (Reddit discussions are the value)
        Ok(String::new())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reddit_source_defaults() {
        let source = RedditSource::new();
        assert_eq!(source.source_type(), "reddit");
        assert_eq!(source.name(), "Reddit");
        assert!(source.config().enabled);
        assert_eq!(source.config().max_items, 30);
    }
}
