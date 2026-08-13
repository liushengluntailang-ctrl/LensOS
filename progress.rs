//! Installation progress tracking and event notification for LensOS v0.1.
//!
//! Provides granular state tracking for asynchronous installation workflows,
//! percentage completion calculations, stage transitions, time estimation,
//! and subscriber callbacks for integration with LensOS UI modals and desktop widgets.

use serde::{Deserialize, Serialize};

/// Task status flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Cancelled,
}

/// Description of the current active installation sub-stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressStage {
    pub stage_id: String,
    pub stage_description: String,
}

/// Snapshot of current task progress state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressState {
    pub task_id: String,
    pub task_title: String,
    pub percentage: f32,
    pub current_stage: ProgressStage,
    pub status: TaskStatus,
    pub elapsed_seconds: u32,
    pub estimated_remaining_seconds: Option<u32>,
}

/// Dynamic event listener interface for UI progress binding.
pub trait ProgressSubscriber: Send + Sync {
    fn on_progress_updated(&self, state: &ProgressState);
}

/// Active progress tracker object managing state updates for an installer operation.
#[derive(Debug, Clone)]
pub struct ProgressTracker {
    pub state: ProgressState,
}

impl ProgressTracker {
    pub fn new(task_id: impl Into<String>, task_title: impl Into<String>) -> Self {
        Self {
            state: ProgressState {
                task_id: task_id.into(),
                task_title: task_title.into(),
                percentage: 0.0,
                current_stage: ProgressStage {
                    stage_id: "init".to_string(),
                    stage_description: "Initializing operation...".to_string(),
                },
                status: TaskStatus::Pending,
                elapsed_seconds: 0,
                estimated_remaining_seconds: None,
            },
        }
    }

    /// Sets the active stage description and transitions status to InProgress.
    pub fn set_stage(&mut self, stage_id: impl Into<String>, description: impl Into<String>) {
        self.state.status = TaskStatus::InProgress;
        self.state.current_stage = ProgressStage {
            stage_id: stage_id.into(),
            stage_description: description.into(),
        };
    }

    /// Updates current completion percentage (0.0 to 100.0).
    pub fn update(&mut self, percentage: f32) {
        self.state.status = TaskStatus::InProgress;
        self.state.percentage = percentage.clamp(0.0, 100.0);
    }

    /// Marks the task as successfully completed.
    pub fn complete(&mut self, message: impl Into<String>) {
        self.state.percentage = 100.0;
        self.state.status = TaskStatus::Completed;
        self.state.current_stage = ProgressStage {
            stage_id: "completed".to_string(),
            stage_description: message.into(),
        };
    }

    /// Marks the task as failed with an error message.
    pub fn fail(&mut self, error_message: impl Into<String>) {
        let msg = error_message.into();
        self.state.status = TaskStatus::Failed(msg.clone());
        self.state.current_stage = ProgressStage {
            stage_id: "failed".to_string(),
            stage_description: format!("Failed: {}", msg),
        };
    }

    /// Cancels the running task.
    pub fn cancel(&mut self) {
        self.state.status = TaskStatus::Cancelled;
        self.state.current_stage = ProgressStage {
            stage_id: "cancelled".to_string(),
            stage_description: "Operation cancelled by user".to_string(),
        };
    }

    /// Checks if operation finished (completed, failed, or cancelled).
    pub fn is_finished(&self) -> bool {
        matches!(
            self.state.status,
            TaskStatus::Completed | TaskStatus::Failed(_) | TaskStatus::Cancelled
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_tracking_lifecycle() {
        let mut tracker = ProgressTracker::new("task_001", "Install App");
        assert_eq!(tracker.state.status, TaskStatus::Pending);

        tracker.set_stage("download", "Downloading binary...");
        tracker.update(25.0);
        assert_eq!(tracker.state.status, TaskStatus::InProgress);
        assert_eq!(tracker.state.percentage, 25.0);

        tracker.complete("App installed successfully");
        assert_eq!(tracker.state.status, TaskStatus::Completed);
        assert_eq!(tracker.state.percentage, 100.0);
        assert!(tracker.is_finished());
    }
}
