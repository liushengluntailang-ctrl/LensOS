//! # Search Integration Engine (`search.rs`)
//!
//! Manages search engine providers (Lens AI Search, DuckDuckGo, Google, Custom),
//! auto-completion suggestion aggregators, and search URL formatters.

use crate::bookmarks::BookmarkManager;
use crate::history::HistoryManager;
use crate::{BrowserError, BrowserResult};

/// Supported search engine provider types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchEngineType {
    LensAI,
    DuckDuckGo,
    Google,
    Bing,
    Custom,
}

/// Metadata and URL query templates for a search provider.
#[derive(Debug, Clone)]
pub struct SearchEngine {
    pub id: String,
    pub name: String,
    pub keyword: String,
    pub search_url_template: String,
    pub suggest_url_template: Option<String>,
    pub engine_type: SearchEngineType,
}

impl SearchEngine {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        keyword: impl Into<String>,
        url_template: impl Into<String>,
        engine_type: SearchEngineType,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            keyword: keyword.into(),
            search_url_template: url_template.into(),
            suggest_url_template: None,
            engine_type,
        }
    }

    /// Replaces `{searchTerms}` placeholder in template with query string.
    pub fn build_search_url(&self, query: &str) -> String {
        let encoded_query = query.replace(' ', "+");
        self.search_url_template.replace("{searchTerms}", &encoded_query)
    }
}

/// Source categorization for address bar suggestions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuggestionSource {
    History,
    Bookmark,
    SearchEngine,
    LensAiContext,
}

/// An inline auto-complete or search result suggestion.
#[derive(Debug, Clone)]
pub struct SearchSuggestion {
    pub text: String,
    pub url: Option<String>,
    pub source: SuggestionSource,
    pub relevance_score: u32,
}

/// Manages configured search engines and aggregates intelligent address bar suggestions.
#[derive(Debug)]
pub struct SearchManager {
    engines: Vec<SearchEngine>,
    default_engine_id: String,
}

impl Default for SearchManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchManager {
    /// Initializes SearchManager with standard default search engines (Lens AI Search primary).
    pub fn new() -> Self {
        let lens_search = SearchEngine::new(
            "lens-ai",
            "Lens AI Search",
            "@lens",
            "https://search.lensos.org/?q={searchTerms}",
            SearchEngineType::LensAI,
        );

        let duckduckgo = SearchEngine::new(
            "duckduckgo",
            "DuckDuckGo",
            "@ddg",
            "https://duckduckgo.com/?q={searchTerms}",
            SearchEngineType::DuckDuckGo,
        );

        let google = SearchEngine::new(
            "google",
            "Google",
            "@g",
            "https://www.google.com/search?q={searchTerms}",
            SearchEngineType::Google,
        );

        Self {
            engines: vec![lens_search, duckduckgo, google],
            default_engine_id: "lens-ai".to_string(),
        }
    }

    /// Returns reference to default search engine.
    pub fn default_engine(&self) -> &SearchEngine {
        self.engines
            .iter()
            .find(|e| e.id == self.default_engine_id)
            .unwrap_or(&self.engines[0])
    }

    /// Sets default search engine by ID.
    pub fn set_default_engine(&mut self, id: &str) -> BrowserResult<()> {
        if self.engines.iter().any(|e| e.id == id) {
            self.default_engine_id = id.to_string();
            Ok(())
        } else {
            Err(BrowserError::SearchError(format!("Search engine '{}' not registered", id)))
        }
    }

    /// Registers a custom search engine provider.
    pub fn add_engine(&mut self, engine: SearchEngine) {
        self.engines.retain(|e| e.id != engine.id);
        self.engines.push(engine);
    }

    /// Builds a search URL for query using default or keyword-specified search engine.
    pub fn format_query_url(&self, query: &str) -> String {
        let trimmed = query.trim();

        // Check if query starts with engine shortcut keyword (e.g. "@ddg rust")
        for engine in &self.engines {
            if trimmed.starts_with(&engine.keyword) {
                let actual_query = trimmed[engine.keyword.len()..].trim();
                return engine.build_search_url(actual_query);
            }
        }

        self.default_engine().build_search_url(trimmed)
    }

    /// Aggregates address bar suggestions from bookmarks, history, and search engine.
    pub fn get_suggestions(
        &self,
        input: &str,
        history: &HistoryManager,
        bookmarks: &BookmarkManager,
    ) -> Vec<SearchSuggestion> {
        let mut suggestions = Vec::new();
        if input.trim().is_empty() {
            return suggestions;
        }

        // 1. Search Bookmarks
        for bm in bookmarks.search(input).iter().take(3) {
            suggestions.push(SearchSuggestion {
                text: bm.title.clone(),
                url: Some(bm.url.clone()),
                source: SuggestionSource::Bookmark,
                relevance_score: 90,
            });
        }

        // 2. Search History
        let hist_query = crate::history::HistoryQuery::new().with_filter(input).with_limit(3);
        for item in history.query(&hist_query) {
            suggestions.push(SearchSuggestion {
                text: item.title.clone(),
                url: Some(item.url.clone()),
                source: SuggestionSource::History,
                relevance_score: 80,
            });
        }

        // 3. Add default search engine query suggestion
        let default_engine = self.default_engine();
        suggestions.push(SearchSuggestion {
            text: format!("Search {} for \"{}\"", default_engine.name, input),
            url: Some(default_engine.build_search_url(input)),
            source: SuggestionSource::SearchEngine,
            relevance_score: 100,
        });

        // Sort by relevance score
        suggestions.sort_by(|a, b| b.relevance_score.cmp(&a.relevance_score));
        suggestions
    }
}
