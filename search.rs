//! LensOS File Search Engine Module (`src/search.rs`)
//!
//! Provides file search capabilities with string matching, extension filtering,
//! depth restrictions, and relevance scoring for LensOS v0.1.

use crate::file::FileInfo;
use std::fs;
use std::path::Path;

/// Filtering rules for constraining file search queries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchFilter {
    pub case_sensitive: bool,
    pub match_extension: Option<String>,
    pub max_depth: usize,
    pub include_hidden: bool,
}

impl Default for SearchFilter {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            match_extension: None,
            max_depth: 5,
            include_hidden: false,
        }
    }
}

/// Individual search result containing file metadata and matching score.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub file_info: FileInfo,
    pub match_score: u32,
    pub matched_in_name: bool,
}

/// Search execution engine for LensOS Files.
#[derive(Debug, Clone, Default)]
pub struct SearchEngine {
    pub last_query: String,
    pub results: Vec<SearchResult>,
    pub is_searching: bool,
}

impl SearchEngine {
    /// Instantiates a clean search engine instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Recursively scans `root_path` searching for files matching `query` and constrained by `filter`.
    pub fn search(
        &mut self,
        root_path: &Path,
        query: &str,
        filter: &SearchFilter,
    ) -> Vec<SearchResult> {
        self.last_query = query.to_string();
        self.is_searching = true;
        self.results.clear();

        if query.trim().is_empty() {
            self.is_searching = false;
            return Vec::new();
        }

        let mut results = Vec::new();
        self.recursive_scan(root_path, query, filter, 0, &mut results);

        // Sort results by relevance score descending
        results.sort_by(|a, b| b.match_score.cmp(&a.match_score));

        self.results = results.clone();
        self.is_searching = false;

        results
    }

    /// Clears the stored search query and result cache.
    pub fn clear(&mut self) {
        self.last_query.clear();
        self.results.clear();
        self.is_searching = false;
    }

    /// Internal recursive traversal method for performing directory inspection.
    fn recursive_scan(
        &self,
        dir: &Path,
        query: &str,
        filter: &SearchFilter,
        current_depth: usize,
        results: &mut Vec<SearchResult>,
    ) {
        if current_depth > filter.max_depth {
            return;
        }

        let read_dir = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return,
        };

        for entry in read_dir.flatten() {
            let path = entry.path();
            if let Ok(file_info) = FileInfo::from_path(&path) {
                if !filter.include_hidden && file_info.is_hidden {
                    continue;
                }

                // Check extension filter
                if let Some(ref required_ext) = filter.match_extension {
                    if file_info
                        .extension
                        .as_ref()
                        .map(|e| e.to_lowercase() != required_ext.to_lowercase())
                        .unwrap_or(true)
                    {
                        if file_info.file_type.is_dir() {
                            self.recursive_scan(&path, query, filter, current_depth + 1, results);
                        }
                        continue;
                    }
                }

                // Match query string against file name
                if let Some(score) = self.calculate_match_score(&file_info.name, query, filter.case_sensitive) {
                    results.push(SearchResult {
                        file_info: file_info.clone(),
                        match_score: score,
                        matched_in_name: true,
                    });
                }

                // Recurse into subdirectories
                if file_info.file_type.is_dir() {
                    self.recursive_scan(&path, query, filter, current_depth + 1, results);
                }
            }
        }
    }

    /// Computes a relevance match score based on substring match and prefix alignment.
    fn calculate_match_score(&self, target: &str, query: &str, case_sensitive: bool) -> Option<u32> {
        let (t, q) = if case_sensitive {
            (target.to_string(), query.to_string())
        } else {
            (target.to_lowercase(), query.to_lowercase())
        };

        if let Some(pos) = t.find(&q) {
            let mut score = 100u32;
            if pos == 0 {
                score += 50; // Exact prefix match bonus
            }
            if t == q {
                score += 100; // Exact name match bonus
            }
            Some(score)
        } else {
            None
        }
    }
}
