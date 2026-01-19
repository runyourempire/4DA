//! Hacker News source implementation
//!
//! Fetches top stories from Hacker News API and scrapes article content.

use async_trait::async_trait;
use scraper::{Html, Selector};
use serde::Deserialize;

use super::{Source, SourceConfig, SourceError, SourceItem, SourceResult};

// ============================================================================
// HN API Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct HNStory {
    id: u64,
    title: Option<String>,
    url: Option<String>,
    text: Option<String>, // For Ask HN / Show HN posts
    score: Option<i32>,
    by: Option<String>,
}

// ============================================================================
// Hacker News Source
// ============================================================================

/// Hacker News source - fetches top stories and scrapes article content
pub struct HackerNewsSource {
    config: SourceConfig,
    client: reqwest::Client,
}

impl HackerNewsSource {
    /// Create a new HN source with default config
    pub fn new() -> Self {
        Self {
            config: SourceConfig {
                enabled: true,
                max_items: 30,
                fetch_interval_secs: 300,
                custom: None,
            },
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Create with custom config
    pub fn with_config(config: SourceConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }
}

impl Default for HackerNewsSource {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Source for HackerNewsSource {
    fn source_type(&self) -> &'static str {
        "hackernews"
    }

    fn name(&self) -> &'static str {
        "Hacker News"
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

        println!("[4DA/HN] Fetching top stories...");

        // Fetch top story IDs
        let top_ids: Vec<u64> = self
            .client
            .get("https://hacker-news.firebaseio.com/v0/topstories.json")
            .send()
            .await
            .map_err(|e| SourceError::Network(e.to_string()))?
            .json()
            .await
            .map_err(|e| SourceError::Parse(e.to_string()))?;

        println!(
            "[4DA/HN] Got {} story IDs, fetching top {}...",
            top_ids.len(),
            self.config.max_items
        );

        let mut items = Vec::new();

        for id in top_ids.into_iter().take(self.config.max_items) {
            let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);

            match self.client.get(&url).send().await {
                Ok(response) => match response.json::<HNStory>().await {
                    Ok(story) => {
                        let title = story.title.unwrap_or_else(|| "[No title]".to_string());

                        // For Ask HN / Show HN, content is in the text field
                        let content = story.text.unwrap_or_default();

                        let mut item = SourceItem::new("hackernews", &id.to_string(), &title)
                            .with_url(story.url.clone())
                            .with_content(content);

                        // Add metadata
                        if let (Some(score), Some(by)) = (story.score, story.by) {
                            item = item.with_metadata(serde_json::json!({
                                "score": score,
                                "author": by,
                            }));
                        }

                        items.push(item);
                    }
                    Err(e) => {
                        println!("[4DA/HN] Failed to parse story {}: {}", id, e);
                    }
                },
                Err(e) => {
                    println!("[4DA/HN] Failed to fetch story {}: {}", id, e);
                }
            }
        }

        println!("[4DA/HN] Fetched {} stories", items.len());
        Ok(items)
    }

    async fn scrape_content(&self, item: &SourceItem) -> SourceResult<String> {
        // If item already has content (Ask HN / Show HN), return it
        if !item.content.is_empty() {
            return Ok(item.content.clone());
        }

        // If no URL, nothing to scrape
        let url = match &item.url {
            Some(u) => u,
            None => return Ok(String::new()),
        };

        // Skip non-HTTP URLs
        if !url.starts_with("http") {
            return Ok(String::new());
        }

        // Create a scraping client with shorter timeout
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| SourceError::Other(e.to_string()))?;

        // Fetch the page
        let response = client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (compatible; 4DA/0.1)")
            .send()
            .await
            .map_err(|e| SourceError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SourceError::Network(format!("HTTP {}", response.status())));
        }

        let html = response
            .text()
            .await
            .map_err(|e| SourceError::Parse(e.to_string()))?;

        // Extract content using multiple selectors
        let document = Html::parse_document(&html);
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

                    // Only use if we got meaningful content
                    if text.len() > 100 {
                        // Truncate to reasonable length
                        let max_len = 5000;
                        let truncated = if text.len() > max_len {
                            text.chars().take(max_len).collect()
                        } else {
                            text
                        };
                        return Ok(truncated);
                    }
                }
            }
        }

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
    fn test_hn_source_defaults() {
        let source = HackerNewsSource::new();
        assert_eq!(source.source_type(), "hackernews");
        assert_eq!(source.name(), "Hacker News");
        assert!(source.config().enabled);
        assert_eq!(source.config().max_items, 30);
    }
}
