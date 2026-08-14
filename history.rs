//! # Browsing History Manager (`history.rs`)
//!
//! Log visits, track visit frequencies, query history by keyword or time range,
//! compute top visited sites, and perform privacy data purges.

/// Unique ID for a history entry.
pub type HistoryItemId = u64;

/// Represents a visited web page entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryItem {
    pub id: HistoryItemId,
    pub url: String,
    pub title: String,
    pub last_visit_time: u64,
    pub visit_count: u32,
    pub typed_count: u32,
    pub favicon_url: Option<String>,
}

impl HistoryItem {
    pub fn new(id: HistoryItemId, url: impl Into<String>, title: impl Into<String>, timestamp: u64) -> Self {
        Self {
            id,
            url: url.into(),
            title: title.into(),
            last_visit_time: timestamp,
            visit_count: 1,
            typed_count: 0,
            favicon_url: None,
        }
    }
}

/// Filter options for history queries.
#[derive(Debug, Clone, Default)]
pub struct HistoryQuery {
    pub text_filter: Option<String>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: usize,
}

impl HistoryQuery {
    pub fn new() -> Self {
        Self {
            text_filter: None,
            start_time: None,
            end_time: None,
            limit: 50,
        }
    }

    pub fn with_filter(mut self, filter: impl Into<String>) -> Self {
        self.text_filter = Some(filter.into());
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

/// Time ranges for history privacy cleanup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearHistoryRange {
    LastHour,
    Last24Hours,
    Last7Days,
    AllTime,
}

/// Manages browsing history logs, search index, and privacy operations.
#[derive(Debug, Default)]
pub struct HistoryManager {
    items: Vec<HistoryItem>,
    next_id: u64,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }

    /// Logs a visit to a URL. Increments visit count if URL already exists.
    pub fn record_visit(&mut self, url: impl Into<String>, title: impl Into<String>, is_typed: bool, timestamp: u64) -> &HistoryItem {
        let url_str = url.into();
        let title_str = title.into();

        if let Some(index) = self.items.iter().position(|item| item.url == url_str) {
            let item = &mut self.items[index];
            item.last_visit_time = timestamp;
            item.visit_count += 1;
            if !title_str.is_empty() {
                item.title = title_str;
            }
            if is_typed {
                item.typed_count += 1;
            }
            &self.items[index]
        } else {
            let id = self.next_id;
            self.next_id += 1;
            let mut item = HistoryItem::new(id, url_str, title_str, timestamp);
            if is_typed {
                item.typed_count = 1;
            }
            self.items.push(item);
            self.items.last().unwrap()
        }
    }

    /// Queries history items based on search filters and time range.
    pub fn query(&self, query: &HistoryQuery) -> Vec<&HistoryItem> {
        let text_lower = query.text_filter.as_ref().map(|s| s.to_lowercase());

        let mut results: Vec<&HistoryItem> = self
            .items
            .iter()
            .filter(|item| {
                if let Some(ref text) = text_lower {
                    if !item.url.to_lowercase().contains(text) && !item.title.to_lowercase().contains(text) {
                        return false;
                    }
                }
                if let Some(start) = query.start_time {
                    if item.last_visit_time < start {
                        return false;
                    }
                }
                if let Some(end) = query.end_time {
                    if item.last_visit_time > end {
                        return false;
                    }
                }
                true
            })
            .collect();

        // Sort by most recent first
        results.sort_by(|a, b| b.last_visit_time.cmp(&a.last_visit_time));

        if query.limit > 0 && results.len() > query.limit {
            results.truncate(query.limit);
        }

        results
    }

    /// Returns top visited sites sorted by visit count.
    pub fn top_sites(&self, limit: usize) -> Vec<&HistoryItem> {
        let mut sorted: Vec<&HistoryItem> = self.items.iter().collect();
        sorted.sort_by(|a, b| b.visit_count.cmp(&a.visit_count));
        sorted.truncate(limit);
        sorted
    }

    /// Clears history items matching the specified time range.
    pub fn clear_history(&mut self, range: ClearHistoryRange, current_time: u64) -> usize {
        let initial_len = self.items.len();

        match range {
            ClearHistoryRange::AllTime => {
                self.items.clear();
            }
            ClearHistoryRange::LastHour => {
                let cutoff = current_time.saturating_sub(3600);
                self.items.retain(|item| item.last_visit_time < cutoff);
            }
            ClearHistoryRange::Last24Hours => {
                let cutoff = current_time.saturating_sub(86400);
                self.items.retain(|item| item.last_visit_time < cutoff);
            }
            ClearHistoryRange::Last7Days => {
                let cutoff = current_time.saturating_sub(86400 * 7);
                self.items.retain(|item| item.last_visit_time < cutoff);
            }
        }

        initial_len - self.items.len()
    }

    /// Removes a specific URL from history log.
    pub fn remove_url(&mut self, url: &str) -> bool {
        let initial = self.items.len();
        self.items.retain(|item| item.url != url);
        self.items.len() < initial
    }

    /// Total history records count.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
