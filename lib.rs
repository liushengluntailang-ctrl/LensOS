//! # LensOS v0.1 - Files Application Module (`apps/files/`)
//!
//! ## Architecture Overview
//! The LensOS Files application provides a modular, high-performance file manager built specifically
//! for the LensOS desktop ecosystem. It enforces strict separation of concerns across dedicated modules
//! handling file representation, folder navigation, sidebar quick-access, search filtering, recent access tracking,
//! and asynchronous-safe disk operations.
//!
//! ### Module Organization
//! - [`files`]: Contains the main [`FilesApp`] orchestrator and LensOS desktop integration interface.
//! - [`explorer`]: Manages path navigation, location history (back/forward/up), sorting, and view modes.
//! - [`folder`]: Encapsulates directory content representation, item counts, and spatial metadata.
//! - [`file`]: Encapsulates individual file metadata, file types, formatted sizes, and icon associations.
//! - [`sidebar`]: Manages system quick-access locations (Home, Desktop, Documents, Downloads, Pictures) and custom locations.
//! - [`search`]: Provides file search capabilities with string matching, extension filtering, and recursive scan limits.
//! - [`operations`]: Implements filesystem mutation handlers (create folder, rename, delete, copy, move) with error handling.
//! - [`recent`]: Tracks recently accessed and pinned items for instant recovery.
//!
//! ### Design Philosophy
//! - **Elegant Dark Theme**: Primary `#0D0F12` void canvas with `#161920` container surfaces.
//! - **Frosted Glass Interface**: Glassmorphism attributes (background blur, 1px subtle borders, semi-transparent opacity).
//! - **Minimalist Layout**: Clean visual hierarchy, zero noise, high contrast typography.
//! - **Smooth Navigation**: Non-blocking navigation history and cached directory states.
//! - **Future LensOS Desktop Integration**: Exposes unified UI state payloads (`FilesAppRenderState`) for the LensOS compositor.

pub mod explorer;
pub mod file;
pub mod files;
pub mod folder;
pub mod operations;
pub mod recent;
pub mod search;
pub mod sidebar;

// Re-exports for convenience
pub use explorer::{FileExplorer, SortBy, SortOrder, ViewMode};
pub use file::{FileInfo, FileMetadata, FileType};
pub use files::{FilesApp, FilesAppRenderState, ThemeConfig};
pub use folder::FolderInfo;
pub use operations::{FileOperation, FileOperationHandler, OperationResult};
pub use recent::{RecentItem, RecentTracker};
pub use search::{SearchEngine, SearchFilter, SearchResult};
pub use sidebar::{Sidebar, SidebarItem, SidebarLocation};
