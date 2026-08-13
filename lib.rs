//! # LensOS v0.1 - Integration Module (`integration/`)
//!
//! ## Architectural Overview
//!
//! The `integration/` module serves as the primary external connectivity, developer tooling,
//! and cloud integration subsystem for LensOS v0.1.
//!
//! It is designed to interface seamlessly with other core LensOS modules:
//! - **boot**: Initializes the integration service stack during LensOS startup.
//! - **kernel**: Interacts with IPC, virtual file systems, and task scheduling.
//! - **desktop / ui**: Feeds real-time GitHub status notifications, commit status badges, and branch selectors to the UI.
//! - **system**: Interacts with system user identity, configuration stores, and credentials keyring.
//! - **installer**: Pulls OS update channels or software packages from GitHub releases/repositories.
//! - **files**: Bridges remote Git repositories directly into the LensOS Files VFS (`files_bridge.rs`).
//! - **settings**: Provides user interface bindings for GitHub OAuth, tokens, and sync frequency settings.
//! - **browser**: Shares authentication cookies/OAuth session handlers with the built-in system browser.
//! - **lens_ai**: Empowers the Lens AI engine (`ai_integration.rs`) to search repositories, clone codebases,
//!   generate commits, push updates, and summarize commit histories.
//!
//! ## Submodules
//!
//! - `auth`: GitHub authentication (OAuth2 & PAT token validation).
//! - `repository`: Repository metadata, permissions, searching, and browsing.
//! - `clone`: Repository cloning into target workspace directories.
//! - `commit`: File change staging, commit creation, and log inspection.
//! - `push`: Pushing commits to remote GitHub branches.
//! - `pull`: Fetching and pulling remote changes.
//! - `branch`: Git branch creation, checkout, listing, and deletion.
//! - `sync`: Bidirectional synchronization between local LensOS files and remotes.
//! - `files_bridge`: Virtual File System bridge mounting Git repos into LensOS Files App.
//! - `ai_integration`: Dedicated Lens AI bindings for automated codebase management.
//! - `github`: Facade API client combining authentication, search, and rate limit telemetry.

pub mod ai_integration;
pub mod auth;
pub mod branch;
pub mod clone;
pub mod commit;
pub mod files_bridge;
pub mod github;
pub mod pull;
pub mod push;
pub mod repository;
pub mod sync;

use thiserror::Error;

use crate::{
    ai_integration::LensAiIntegration,
    auth::GitHubAuthenticator,
    branch::{Branch, BranchManager},
    clone::{CloneOptions, ClonedRepository, RepositoryCloner},
    commit::{Commit, CommitAuthor, CommitManager, FileChangeType},
    files_bridge::{FilesBridge, VirtualGitFolder},
    github::GitHubClient,
    pull::{PullManager, PullOptions, PullResult},
    push::{PushManager, PushOptions, PushResult},
    repository::{Repository, RepositoryBrowser, RepositorySearchQuery},
    sync::RepositorySyncer,
};

/// Master error type for all operations within the LensOS `integration/` module.
#[derive(Debug, Error)]
pub enum IntegrationError {
    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Clone failed: {0}")]
    CloneError(String),

    #[error("Commit failed: {0}")]
    CommitError(String),

    #[error("Push operation failed: {0}")]
    PushError(String),

    #[error("Pull operation failed: {0}")]
    PullError(String),

    #[error("Branch operation error: {0}")]
    BranchError(String),

    #[error("Sync operation error: {0}")]
    SyncError(String),

    #[error("Files App Bridge error: {0}")]
    FilesBridgeError(String),

    #[error("Lens AI Integration error: {0}")]
    AiError(String),

    #[error("Network failure: {0}")]
    NetworkError(String),
}

/// Orchestrator for LensOS Integration services.
/// Connects GitHub authentication, repository management, local file system bridging,
/// sync engines, and Lens AI automation.
#[derive(Debug, Clone)]
pub struct IntegrationManager {
    pub authenticator: GitHubAuthenticator,
    pub browser: RepositoryBrowser,
    pub cloner: RepositoryCloner,
    pub commit_manager: CommitManager,
    pub push_manager: PushManager,
    pub pull_manager: PullManager,
    pub branch_manager: BranchManager,
    pub syncer: RepositorySyncer,
    pub files_bridge: FilesBridge,
    pub ai_integration: LensAiIntegration,
}

impl IntegrationManager {
    /// Create a fully initialized `IntegrationManager` instance for LensOS v0.1.
    pub fn new() -> Self {
        let authenticator = GitHubAuthenticator::new();
        let browser = RepositoryBrowser::new();
        let cloner = RepositoryCloner::new();
        let commit_manager = CommitManager::new();
        let push_manager = PushManager::new();
        let pull_manager = PullManager::new();
        let branch_manager = BranchManager::new();
        let syncer = RepositorySyncer::new();
        let files_bridge = FilesBridge::new();

        let ai_integration = LensAiIntegration::new(
            authenticator.clone(),
            browser.clone(),
            cloner.clone(),
            commit_manager.clone(),
            push_manager.clone(),
        );

        Self {
            authenticator,
            browser,
            cloner,
            commit_manager,
            push_manager,
            pull_manager,
            branch_manager,
            syncer,
            files_bridge,
            ai_integration,
        }
    }

    /// Authenticate LensOS user with GitHub token.
    pub fn authenticate(&mut self, token: &str, username: &str) -> Result<(), IntegrationError> {
        self.authenticator.authenticate_with_token(token, username)?;
        self.sync_ai_engine();
        Ok(())
    }

    /// Search GitHub repositories.
    pub fn search_repositories(&self, query: &str) -> Vec<Repository> {
        let q = RepositorySearchQuery {
            query: query.to_string(),
            ..Default::default()
        };
        self.browser.search(&q)
    }

    /// Clone a GitHub repository into LensOS workspace directory.
    pub fn clone_repository(
        &mut self,
        repo: &Repository,
        target_dir: &str,
    ) -> Result<ClonedRepository, IntegrationError> {
        let options = CloneOptions {
            target_directory: target_dir.to_string(),
            ..Default::default()
        };
        let cloned = self.cloner.clone_repository(repo, &options)?;

        // Auto-mount in Files Bridge
        let _ = self.files_bridge.mount_repository(
            &cloned.local_path,
            &cloned.info.name,
            &cloned.info.owner,
            &cloned.active_branch,
        );

        self.sync_ai_engine();
        Ok(cloned)
    }

    /// Stage modified files and create a commit.
    pub fn stage_and_commit(
        &mut self,
        repo_path: &str,
        message: &str,
        file_paths: &[&str],
        author: Option<CommitAuthor>,
    ) -> Result<Commit, IntegrationError> {
        for path in file_paths {
            self.commit_manager.stage_file(repo_path, path, FileChangeType::Modified)?;
        }

        let author = author.unwrap_or_default();
        let commit = self.commit_manager.create_commit(repo_path, author, message)?;
        self.sync_ai_engine();
        Ok(commit)
    }

    /// Push local repository commits to GitHub remote.
    pub fn push(
        &mut self,
        repo_path: &str,
        remote: &str,
        branch: &str,
    ) -> Result<PushResult, IntegrationError> {
        let token = self.authenticator.get_token();
        let options = PushOptions {
            remote_name: remote.to_string(),
            target_branch: branch.to_string(),
            ..Default::default()
        };
        self.push_manager.push(repo_path, &options, token)
    }

    /// Pull remote repository changes into local clone.
    pub fn pull(
        &mut self,
        repo_path: &str,
        remote: &str,
        branch: &str,
    ) -> Result<PullResult, IntegrationError> {
        let token = self.authenticator.get_token();
        let options = PullOptions {
            remote_name: remote.to_string(),
            source_branch: branch.to_string(),
            ..Default::default()
        };
        self.pull_manager.pull(repo_path, &options, token)
    }

    /// Switch active branch in a local repository clone.
    pub fn switch_branch(
        &mut self,
        repo_path: &str,
        branch_name: &str,
    ) -> Result<Branch, IntegrationError> {
        self.branch_manager.switch_branch(repo_path, branch_name)
    }

    /// Mount repository in LensOS Files App virtual file system.
    pub fn mount_in_files_app(
        &mut self,
        repo_path: &str,
        repo_name: &str,
        owner: &str,
        branch: &str,
    ) -> Result<VirtualGitFolder, IntegrationError> {
        self.files_bridge.mount_repository(repo_path, repo_name, owner, branch)
    }

    /// Access GitHub Client Facade instance.
    pub fn github_client(&self) -> GitHubClient {
        GitHubClient::new(self.authenticator.clone(), self.browser.clone())
    }

    /// Sync updated references into Lens AI integration instance.
    fn sync_ai_engine(&mut self) {
        self.ai_integration = LensAiIntegration::new(
            self.authenticator.clone(),
            self.browser.clone(),
            self.cloner.clone(),
            self.commit_manager.clone(),
            self.push_manager.clone(),
        );
    }
}

impl Default for IntegrationManager {
    fn default() -> Self {
        Self::new()
    }
}
