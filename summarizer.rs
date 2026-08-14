use serde::{Deserialize, Serialize};

/// Target summary detail level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SummaryLength {
    Short,
    Medium,
    Detailed,
    Executive,
}

impl Default for SummaryLength {
    fn default() -> Self {
        SummaryLength::Medium
    }
}

/// Layout format for the generated summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SummaryFormat {
    Paragraph,
    BulletPoints,
    KeyTakeaways,
}

impl Default for SummaryFormat {
    fn default() -> Self {
        SummaryFormat::BulletPoints
    }
}

/// Options to guide summarization behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryOptions {
    pub length: SummaryLength,
    pub format: SummaryFormat,
    pub max_bullet_points: usize,
    pub include_key_takeaways: bool,
    pub focus_keywords: Vec<String>,
}

impl Default for SummaryOptions {
    fn default() -> Self {
        Self {
            length: SummaryLength::Medium,
            format: SummaryFormat::BulletPoints,
            max_bullet_points: 5,
            include_key_takeaways: true,
            focus_keywords: Vec::new(),
        }
    }
}

/// Result object output by the Summarizer module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryResult {
    pub summary: String,
    pub key_points: Vec<String>,
    pub original_word_count: usize,
    pub summary_word_count: usize,
    pub compression_ratio: f32,
}

/// Summarizer engine for LensAI text and document compression.
pub struct Summarizer;

impl Summarizer {
    pub fn new() -> Self {
        Self
    }

    pub fn summarize(&self, content: &str, options: SummaryOptions) -> Result<SummaryResult, String> {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Err("Input content for summarization cannot be empty.".to_string());
        }

        let words: Vec<&str> = trimmed.split_whitespace().collect();
        let original_word_count = words.len();

        let key_points = self.extract_key_points(content, options.max_bullet_points);

        let summary = match options.format {
            SummaryFormat::Paragraph => format!(
                "Executive Summary ({:?} length): {}",
                options.length,
                if words.len() > 30 {
                    words[..30].join(" ") + "..."
                } else {
                    trimmed.to_string()
                }
            ),
            SummaryFormat::BulletPoints => {
                let mut out = String::from("Key Points:\n");
                for (idx, point) in key_points.iter().enumerate() {
                    out.push_str(&format!("  {}. {}\n", idx + 1, point));
                }
                out
            }
            SummaryFormat::KeyTakeaways => format!(
                "Core Takeaways:\n- {}",
                key_points.join("\n- ")
            ),
        };

        let summary_word_count = summary.split_whitespace().count();
        let compression_ratio = if original_word_count > 0 {
            1.0 - (summary_word_count as f32 / original_word_count as f32)
        } else {
            0.0
        };

        Ok(SummaryResult {
            summary,
            key_points,
            original_word_count,
            summary_word_count,
            compression_ratio,
        })
    }

    pub fn extract_key_points(&self, content: &str, limit: usize) -> Vec<String> {
        let sentences: Vec<&str> = content
            .split(&['.', '!', '?'][..])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if sentences.is_empty() {
            return vec!["No coherent sentences found to summarize.".to_string()];
        }

        sentences
            .iter()
            .take(limit)
            .map(|s| s.to_string())
            .collect()
    }
}

impl Default for Summarizer {
    fn default() -> Self {
        Self::new()
    }
}
