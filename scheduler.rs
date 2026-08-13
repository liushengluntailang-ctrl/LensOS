//! LensOS v0.1 - Task Scheduler Subsystem
//!
//! Implements multi-queue preemptive process scheduling, thread state transitions,
//! and timer tick interrupts for LensOS task context switches.

use std::collections::VecDeque;

/// Task execution state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Task scheduling priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    RealTime = 3,
}

/// Process Control Block (PCB) for a task in LensOS.
#[derive(Debug, Clone)]
pub struct Task {
    pub pid: usize,
    pub name: String,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub cpu_time_ticks: u64,
}

/// Preemptive Multi-Queue Task Scheduler.
pub struct TaskScheduler {
    initialized: bool,
    next_pid: usize,
    active_pid: Option<usize>,
    tasks: Vec<Task>,
    ready_queue: VecDeque<usize>,
}

impl TaskScheduler {
    /// Constructs a new task scheduler.
    pub fn new() -> Self {
        Self {
            initialized: false,
            next_pid: 0,
            active_pid: None,
            tasks: Vec::new(),
            ready_queue: VecDeque::new(),
        }
    }

    /// Initializes the CPU scheduler and spawns the initial kernel idle process (PID 0).
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("[BOOT][SCHEDULER] Initializing preemptive multi-queue process scheduler...");
        println!("[BOOT][SCHEDULER] Configuring Programmable Interval Timer (PIT) at 100 Hz...");

        // Spawn Kernel Idle Task (PID 0)
        let _idle_pid = self.spawn_task_internal("kidle", TaskPriority::Low);
        // Spawn System Init Task (PID 1)
        let init_pid = self.spawn_task_internal("init", TaskPriority::High);

        self.active_pid = Some(init_pid);
        if let Some(init_task) = self.tasks.iter_mut().find(|t| t.pid == init_pid) {
            init_task.state = TaskState::Running;
        }

        self.initialized = true;
        println!(
            "[BOOT][SCHEDULER] Scheduler online. Spawned PID 0 (kidle) and PID 1 (init)."
        );
        Ok(())
    }

    /// Internal helper to construct and register a task.
    fn spawn_task_internal(&mut self, name: &str, priority: TaskPriority) -> usize {
        let pid = self.next_pid;
        self.next_pid += 1;

        let task = Task {
            pid,
            name: name.to_string(),
            state: TaskState::Ready,
            priority,
            cpu_time_ticks: 0,
        };

        self.tasks.push(task);
        self.ready_queue.push_back(pid);
        pid
    }

    /// Spawns a new user or kernel task.
    pub fn spawn_task(&mut self, name: &str, priority: TaskPriority) -> Result<usize, String> {
        if !self.initialized {
            return Err("Task scheduler is not initialized.".to_string());
        }
        let pid = self.spawn_task_internal(name, priority);
        println!(
            "[SCHEDULER] Spawns task '{}' (PID: {}, Priority: {:?})",
            name, pid, priority
        );
        Ok(pid)
    }

    /// Simulates a system timer tick interrupt and context switch.
    pub fn schedule_tick(&mut self) {
        if !self.initialized {
            return;
        }

        // Increment current task tick count
        if let Some(curr_pid) = self.active_pid {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.pid == curr_pid) {
                task.cpu_time_ticks += 1;
            }
        }

        // Context switch rotation
        if let Some(next_pid) = self.ready_queue.pop_front() {
            if let Some(prev_pid) = self.active_pid {
                if let Some(prev_task) = self.tasks.iter_mut().find(|t| t.pid == prev_pid) {
                    if prev_task.state == TaskState::Running {
                        prev_task.state = TaskState::Ready;
                        self.ready_queue.push_back(prev_pid);
                    }
                }
            }

            self.active_pid = Some(next_pid);
            if let Some(next_task) = self.tasks.iter_mut().find(|t| t.pid == next_pid) {
                next_task.state = TaskState::Running;
            }
        }
    }

    /// Terminates a process by PID.
    pub fn terminate_task(&mut self, pid: usize) -> Result<(), String> {
        if !self.initialized {
            return Err("Task scheduler is not initialized.".to_string());
        }

        if let Some(task) = self.tasks.iter_mut().find(|t| t.pid == pid) {
            task.state = TaskState::Terminated;
            println!("[SCHEDULER] Terminated PID {} ({})", pid, task.name);
            Ok(())
        } else {
            Err(format!("Process PID {} not found.", pid))
        }
    }

    /// Returns the active process list.
    pub fn get_tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Returns the number of currently active non-terminated processes.
    pub fn active_task_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.state != TaskState::Terminated).count()
    }

    /// Shuts down scheduler and clears task queues.
    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Ok(());
        }
        println!("[SHUTDOWN][SCHEDULER] Stopping CPU context switching and terminating processes...");
        self.tasks.clear();
        self.ready_queue.clear();
        self.initialized = false;
        Ok(())
    }

    /// Checks if scheduler is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for TaskScheduler {
    fn default() -> Self {
        Self::new()
    }
}
