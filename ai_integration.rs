//! # LensOS v0.1 Integration Module - Lens AI Integration (`ai_integration.rs`)
//!
//! Provides dedicated interfaces for `lens_ai` (Lens AI Assistant) in LensOS.
//!
//! Lens AI is empowered to:
//! 1. Search GitHub repositories
//! 2. Clone repositories into local workspace directories
//! 3. Create structured git commits with author details and messages
//! 4. Push local changes to GitHub remotes
//! 5. Summarize commit history, diffs, and codebase structure for intelligence tasks

use crate::{
    auth::GitHubAuthenticator,
    clone::{CloneOptions, ClonedRepository, RepositoryCloner},
    commit::{Commit, CommitAuthor, CommitManager, FileChangeType},
    push::{PushManager, PushOptions, PushResult},
    repository::{Repository, RepositoryBrowser, RepositorySearchQuery},
    IntegrationError,
};

/// High-level Lens AI integration engine component.
#[derive(Debug, Clone)]
pub struct LensAiIntegration {
    authenticator: GitHubAuthenticator,
    browser: RepositoryBrowser,
    cloner: RepositoryCloner,
    commit_manager: CommitManager,
    push_manager: PushManager,
}

impl LensAiIntegration {
    /// Create a new `LensAiIntegration` engine instance.
    pub fn new(
        authenticator: GitHubAuthenticator,
        browser: RepositoryBrowser,
        cloner: RepositoryCloner,
        commit_manager: CommitManager,
        push_manager: PushManager,
    ) -> Self {
        Self {
            authenticator,
            browser,
            cloner,
            commit_manager,
            push_manager,
        }
    }

    /// 1. Lens AI capability: Search GitHub repositories by keywords or query criteria.
    pub fn search_repositories(&self, query: &str) -> Vec<Repository> {
        let search_query = RepositorySearchQuery {
            query: query.to_string(),
            language: None,
            owner: None,
            include_private: self.authenticator.validate_session(),
            limit: 10,
        };
        self.browser.search(&search_query)
    }

    /// 2. Lens AI capability: Clone a repository into target workspace directory.
    pub fn clone_repository(
        &mut self,
        repo_url_or_name: &str,
        target_path: &str,
    ) -> Result<ClonedRepository, IntegrationError> {
        let repo = if let Some(r) = self.browser.list_all().into_iter().find(|r| {
            r.clone_url == repo_url_or_name || r.name == repo_url_or_name || r.full_name == repo_url_or_name
        }) {
            r
        } else {
            // Construct fallback repository structure if URL specified
            let name = repo_url_or_name.split('/').last().unwrap_or("repo").trim_end_matches(".git");
            Repository {
                id: 999,
                name: name.to_string(),
                full_name: format!("ai_discovered/{}", name),
                owner: "ai_discovered".to_string(),
                description: Some("Discovered by Lens AI Assistant".to_string()),
                default_branch: "main".to_string(),
                visibility: crate::repository::RepositoryVisibility::Public,
                clone_url: repo_url_or_name.to_string(),
                ssh_url: format!("git@github.com:{}.git", name),
                stargazers_count: 0,
                forks_count: 0,
                updated_at: chrono::Utc::now().to_rfc3339(),
                permissions: crate::repository::RepositoryPermissions::default(),
            }
        };

        let options = CloneOptions {
            target_directory: target_path.to_string(),
            ..Default::default()
        };

        self.cloner.clone_repository(&repo, &options)
    }

    /// 3. Lens AI capability: Create commits automatically with staged/modified file lists.
    pub fn create_commit(
        &mut self,
        repo_path: &str,
        message: &str,
        files: &[&str],
    ) -> Result<Commit, IntegrationError> {
        for file in files {
            self.commit_manager.stage_file(repo_path, file, FileChangeType::Modified)?;
        }

        let author = CommitAuthor {
            name: "Lens AI Assistant".to_string(),
            email: "lens-ai@lensos.org".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.commit_manager.create_commit(repo_path, author, message)
    }

    /// 4. Lens AI capability: Push commits to remote branch.
    pub fn push_changes(
        &mut self,
        repo_path: &str,
        remote: &str,
        branch: &str,
    ) -> Result<PushResult, IntegrationError> {
        let token = self.authenticator.get_token();
        let options = PushOptions {
            remote_name: remote.to_string(),
            target_branch: branch.to_string(),
            force: false,
            tags: false,
        };

        self.push_manager.push(repo_path, &options, token)
    }

    /// 5. Lens AI capability: Summarize commit history and recent diffs for reasoning.
    pub fn summarize_commit_history(
        &self,
        repo_path: &str,
        count: usize,
    ) -> Result<String, IntegrationError> {
        let commits = self.commit_manager.get_commit_history(repo_path, count);

        if commits.is_empty() {
            return Ok(format!(
                "Lens AI Summary for {}: Repository is initialized. No commits recorded yet.",
                repo_path
            ));
        }

        let mut summary = format!(
            "Lens AI Commit History Summary for {} (Latest {} commits):\n",
            repo_path,
            commits.len()
        );

        for (idx, commit) in commits.iter().enumerate() {
            summary.push_str(&format!(
                "{}. [{}] - {} (by {} at {})\n   Files changed: {}\n",
                idx + 1,
                &commit.hash[0..8.min(commit.hash.len())],
                commit.message,
                commit.author.name,
                commit.timestamp,
                commit.changed_files.len()
            ));
        }

        Ok(summary)
    }

    /// Suggest an AI generated commit message based on staged or recent file changes.
    pub fn suggest_commit_message(&self, repo_path: &str) -> String {
        let staged = self.commit_manager.get_staged(repo_path);
        if staged.is_empty() {
            "feat: update workspace files".to_string()
        } else {
            let file_names: Vec<&str> = staged.iter().map(|f| f.file_path.as_str()).collect();
            format!("feat: update {}", file_names.join(", "))
        }
    }
}
