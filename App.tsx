import React, { useState, useEffect } from 'react';
import { Play, RotateCcw, Code, Terminal, CheckCircle2, Cpu, HardDrive, Layers, Server, Shield, Sparkles, Copy, Check } from 'lucide-react';

// Rust source files stored for viewing in the web inspector
const RUST_FILES: Record<string, string> = {
  'Cargo.toml': `[package]
name = "lensos-boot"
version = "0.1.0"
edition = "2021"
authors = ["LensOS Kernel Core Team"]
description = "Boot module for the LensOS operating system"

[dependencies]
`,
  'src/main.rs': `//! # LensOS Boot Module - Main Entry Point
//!
//! Entry point for the LensOS operating system boot simulation.
//! Coordinates the modular boot process, initializing hardware abstraction,
//! microkernel subsystems, and visual boot screens.

mod animation;
mod bootloader;
mod logo;

use bootloader::Bootloader;
use std::process::ExitCode;

/// Main function for the LensOS boot module.
///
/// Instantiates the LensOS \`Bootloader\` and triggers the ~3-second boot sequence.
/// On completion, displays success messaging and returns clean exit code.
fn main() -> ExitCode {
    let bootloader = Bootloader::new();

    match bootloader.execute_boot() {
        Ok(_) => {
            // Explicit required output upon completion
            println!("Boot completed successfully");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("LensOS Boot Error: Failed during boot sequence: {}", err);
            ExitCode::FAILURE
        }
    }
}
`,
  'src/bootloader.rs': `//! # LensOS Bootloader Module
//!
//! Orchestrates the operating system boot sequence. Simulates low-level
//! kernel initialization, memory management setup, virtual filesystem mounting,
//! driver stack startup, and user-space initialization within a 3-second timing budget.

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::animation::LoadingAnimation;
use crate::logo::{colors, LogoRenderer};

/// Subsystem initialization step in the boot sequence.
pub struct BootStep {
    pub name: &'static str,
    pub duration_ms: u64,
}

pub struct Bootloader {
    logo_renderer: LogoRenderer,
    boot_steps: Vec<BootStep>,
}

impl Bootloader {
    pub fn new() -> Self {
        Self {
            logo_renderer: LogoRenderer::new(),
            boot_steps: vec![
                BootStep { name: "Initializing CPU cores & interrupts", duration_ms: 450 },
                BootStep { name: "Probing physical RAM & mapping page tables", duration_ms: 550 },
                BootStep { name: "Loading LensOS microkernel image", duration_ms: 600 },
                BootStep { name: "Mounting Virtual File System (VFS)", duration_ms: 500 },
                BootStep { name: "Starting Hardware Abstraction Layer (HAL)", duration_ms: 450 },
                BootStep { name: "Launching core system daemons", duration_ms: 450 },
            ],
        }
    }

    pub fn execute_boot(&self) -> io::Result<()> {
        let start_time = Instant::now();
        self.logo_renderer.render()?;

        let animation = LoadingAnimation::new("Starting system...");
        let mut current_pct = 0u32;
        let total_steps = self.boot_steps.len();

        for (idx, step) in self.boot_steps.iter().enumerate() {
            let next_pct = ((idx + 1) * 100 / total_steps) as u32;
            animation.animate_stage(step.duration_ms, current_pct, next_pct)?;
            current_pct = next_pct;
        }

        thread::sleep(Duration::from_millis(150));
        let elapsed = start_time.elapsed();
        println!("   ✓ Boot completed successfully in {:.2?}!", elapsed);
        LogoRenderer::restore_terminal()?;
        Ok(())
    }
}
`,
  'src/logo.rs': `//! # LensOS Logo Module
//!
//! Provides ASCII art rendering and visual display components for the
//! LensOS boot screen. Handles ANSI color codes for glowing blue visuals
//! and black background formatting.

use std::io::{self, Write};

pub mod colors {
    pub const RESET: &str = "\\x1b[0m";
    pub const BG_BLACK: &str = "\\x1b[40m";
    pub const CLEAR_SCREEN: &str = "\\x1b[2J";
    pub const CURSOR_HOME: &str = "\\x1b[1;1H";
    pub const GLOW_CYAN_BLUE: &str = "\\x1b[1;38;5;39m";
    pub const DEEP_BLUE: &str = "\\x1b[1;38;5;33m";
    pub const SOFT_BLUE: &str = "\\x1b[38;5;75m";
    pub const BRIGHT_WHITE: &str = "\\x1b[1;97m";
    pub const DIM_GRAY: &str = "\\x1b[38;5;244m";
}

pub struct LogoRenderer;

impl LogoRenderer {
    pub fn new() -> Self { LogoRenderer }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        write!(stdout, "{}{}{}", colors::CLEAR_SCREEN, colors::CURSOR_HOME, colors::BG_BLACK)?;

        // ASCII Graphic & LensOS Text Output
        writeln!(stdout, "                LensOS v0.1")?;
        writeln!(stdout, "    Next-Gen AI Operating System-")?;
        stdout.flush()?;
        Ok(())
    }
}
`,
  'src/animation.rs': `//! # LensOS Animation Module
//!
//! Provides loading animations and progress indicators during the boot sequence.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use crate::logo::colors;

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct LoadingAnimation {
    label: String,
}

impl LoadingAnimation {
    pub fn new(label: &str) -> Self {
        Self { label: label.to_string() }
    }

    pub fn animate_stage(&self, duration_ms: u64, start_pct: u32, end_pct: u32) -> io::Result<()> {
        let frame_delay = Duration::from_millis(50);
        let total_frames = (duration_ms / 50).max(1);
        let mut stdout = io::stdout();

        for frame in 0..total_frames {
            let spinner = SPINNER_FRAMES[(frame as usize) % SPINNER_FRAMES.len()];
            let progress = start_pct + ((end_pct - start_pct) * (frame as u32 + 1) / total_frames as u32);
            write!(stdout, "\\r   [{}] {}% {}", progress, spinner, self.label)?;
            stdout.flush()?;
            thread::sleep(frame_delay);
        }
        Ok(())
    }
}
`,
};

const STAGES = [
  { name: 'Initializing CPU cores & interrupts', icon: Cpu, duration: 500 },
  { name: 'Probing physical RAM & mapping page tables', icon: HardDrive, duration: 600 },
  { name: 'Loading LensOS microkernel image', icon: Shield, duration: 650 },
  { name: 'Mounting Virtual File System (VFS)', icon: Layers, duration: 500 },
  { name: 'Starting Hardware Abstraction Layer (HAL)', icon: Server, duration: 450 },
  { name: 'Launching core system daemons', icon: Sparkles, duration: 450 },
];

const SPINNER_FRAMES = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

export default function App() {
  const [activeTab, setActiveTab] = useState<'monitor' | 'code'>('monitor');
  const [isBooting, setIsBooting] = useState(false);
  const [bootProgress, setBootProgress] = useState(0);
  const [currentStageIdx, setCurrentStageIdx] = useState(0);
  const [bootLogs, setBootLogs] = useState<string[]>([]);
  const [bootComplete, setBootComplete] = useState(false);
  const [selectedFile, setSelectedFile] = useState<string>('src/main.rs');
  const [copied, setCopied] = useState(false);
  const [spinnerIdx, setSpinnerIdx] = useState(0);
  const [elapsedTime, setElapsedTime] = useState('0.00s');

  const startBoot = () => {
    setIsBooting(true);
    setBootProgress(0);
    setCurrentStageIdx(0);
    setBootLogs(['[LENSOS KERNEL] Cold boot sequence initiated...']);
    setBootComplete(false);
  };

  useEffect(() => {
    startBoot();
  }, []);

  // Spinner animation loop
  useEffect(() => {
    if (!isBooting || bootComplete) return;
    const interval = setInterval(() => {
      setSpinnerIdx((prev) => (prev + 1) % SPINNER_FRAMES.length);
    }, 80);
    return () => clearInterval(interval);
  }, [isBooting, bootComplete]);

  // Boot sequence simulation matching Rust timing (~3 seconds)
  useEffect(() => {
    if (!isBooting) return;

    let startTime = Date.now();
    let currentStep = 0;
    let progress = 0;

    const timer = setInterval(() => {
      const elapsed = Date.now() - startTime;
      setElapsedTime((elapsed / 1000).toFixed(2) + 's');

      if (currentStep < STAGES.length) {
        const stagePct = Math.min(100, Math.floor(((currentStep + 1) / STAGES.length) * 100));
        progress = Math.min(100, progress + 3);
        setBootProgress(progress);

        if (progress >= stagePct) {
          const stageName = STAGES[currentStep].name;
          setBootLogs((prev) => [...prev, `[OK] ${stageName}`]);
          currentStep++;
          setCurrentStageIdx(Math.min(currentStep, STAGES.length - 1));
        }
      } else {
        setBootProgress(100);
        setBootComplete(true);
        setIsBooting(false);
        setBootLogs((prev) => [...prev, '✓ Boot completed successfully in 3.18s']);
        clearInterval(timer);
      }
    }, 90);

    return () => clearInterval(timer);
  }, [isBooting]);

  const copyCode = () => {
    navigator.clipboard.writeText(RUST_FILES[selectedFile]);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans antialiased selection:bg-cyan-500/30 selection:text-cyan-200">
      {/* Header Bar */}
      <header className="border-b border-slate-800 bg-slate-900/80 backdrop-blur px-6 py-3.5 flex items-center justify-between sticky top-0 z-30">
        <div className="flex items-center gap-3">
          <div className="relative flex items-center justify-center w-8 h-8 rounded-full bg-cyan-950 border border-cyan-500/40 shadow-[0_0_15px_rgba(6,182,212,0.35)]">
            <div className="w-3.5 h-3.5 rounded-full border-2 border-cyan-400 bg-cyan-500 animate-pulse" />
          </div>
          <div>
            <h1 className="font-semibold text-lg tracking-tight text-slate-100 flex items-center gap-2">
              LensOS <span className="text-xs px-2 py-0.5 rounded-full bg-cyan-950 border border-cyan-500/30 text-cyan-400 font-mono">v0.1 Rust Core</span>
            </h1>
            <p className="text-xs text-slate-400 font-mono">Next-Gen AI Operating System — Boot Module</p>
          </div>
        </div>

        {/* View Switcher */}
        <div className="flex items-center gap-2 bg-slate-950 p-1 rounded-lg border border-slate-800">
          <button
            onClick={() => setActiveTab('monitor')}
            className={`flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium transition-all ${
              activeTab === 'monitor'
                ? 'bg-cyan-950 text-cyan-300 border border-cyan-500/40 shadow-sm'
                : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'
            }`}
          >
            <Terminal className="w-3.5 h-3.5" />
            Live Boot Monitor
          </button>
          <button
            onClick={() => setActiveTab('code')}
            className={`flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium transition-all ${
              activeTab === 'code'
                ? 'bg-cyan-950 text-cyan-300 border border-cyan-500/40 shadow-sm'
                : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'
            }`}
          >
            <Code className="w-3.5 h-3.5" />
            Rust Cargo Project Code
          </button>
        </div>
      </header>

      {/* Main Body */}
      <main className="flex-1 p-4 md:p-6 max-w-6xl w-full mx-auto flex flex-col gap-6">
        {activeTab === 'monitor' ? (
          <div className="flex flex-col gap-4">
            {/* Monitor Frame */}
            <div className="relative rounded-2xl border border-slate-800 bg-black shadow-2xl shadow-cyan-950/20 overflow-hidden flex flex-col min-h-[500px]">
              {/* Screen Display */}
              <div className="flex-1 p-6 md:p-10 flex flex-col items-center justify-between relative bg-radial from-slate-950 via-black to-black">
                {/* Glowing Lens Ring Graphic */}
                <div className="relative my-6 flex flex-col items-center">
                  {/* Outer Pulsing Glow */}
                  <div className="absolute -inset-8 rounded-full bg-cyan-500/10 blur-2xl animate-pulse" />
                  
                  {/* Optical Lens Ring */}
                  <div className="relative w-36 h-36 rounded-full border-4 border-cyan-500/80 shadow-[0_0_50px_rgba(6,182,212,0.6)] flex items-center justify-center bg-gradient-to-tr from-cyan-950/80 via-blue-950/40 to-slate-950">
                    <div className="w-24 h-24 rounded-full border-2 border-blue-400/60 flex items-center justify-center shadow-[inset_0_0_20px_rgba(59,130,246,0.5)]">
                      <div className="w-14 h-14 rounded-full border border-cyan-300/80 bg-gradient-to-br from-cyan-400/30 to-blue-600/50 flex items-center justify-center">
                        <div className="w-6 h-6 rounded-full bg-cyan-200 shadow-[0_0_15px_rgba(165,243,252,0.9)] animate-ping" />
                      </div>
                    </div>
                  </div>

                  {/* OS Branding Text */}
                  <div className="text-center mt-6">
                    <h2 className="text-4xl md:text-5xl font-extrabold tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-cyan-300 via-blue-400 to-cyan-200 drop-shadow-[0_0_25px_rgba(6,182,212,0.5)] font-mono">
                      LensOS
                    </h2>
                    <p className="text-cyan-400/90 text-sm font-mono tracking-widest mt-1">
                      LensOS v0.1
                    </p>
                    <p className="text-blue-300/60 text-xs font-mono tracking-wider mt-0.5">
                      Next-Gen AI Operating System-
                    </p>
                  </div>
                </div>

                {/* Progress & Status Section */}
                <div className="w-full max-w-xl mx-auto flex flex-col gap-3 font-mono">
                  {/* Status Indicator */}
                  <div className="flex items-center justify-between text-xs text-slate-400 px-1">
                    <span className="flex items-center gap-2 text-cyan-300">
                      <span className="text-cyan-400 text-sm font-bold">
                        {bootComplete ? '✓' : SPINNER_FRAMES[spinnerIdx]}
                      </span>
                      {bootComplete ? 'System Ready' : 'Starting system...'}
                    </span>
                    <span>{bootProgress}%</span>
                  </div>

                  {/* Progress Bar */}
                  <div className="h-2.5 w-full bg-slate-900 rounded-full overflow-hidden border border-slate-800 p-0.5">
                    <div
                      className="h-full bg-gradient-to-r from-blue-600 via-cyan-400 to-cyan-300 rounded-full transition-all duration-100 ease-out shadow-[0_0_12px_rgba(6,182,212,0.8)]"
                      style={{ width: `${bootProgress}%` }}
                    />
                  </div>

                  {/* Current Active Kernel Step */}
                  <div className="text-xs text-slate-400 text-center py-1">
                    {!bootComplete && STAGES[currentStageIdx] && (
                      <span className="text-cyan-400/90 animate-pulse">
                        [BOOT] {STAGES[currentStageIdx].name}
                      </span>
                    )}
                    {bootComplete && (
                      <span className="text-emerald-400 font-semibold flex items-center justify-center gap-1.5">
                        <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                        Boot completed successfully ({elapsedTime})
                      </span>
                    )}
                  </div>
                </div>

                {/* Live Console Output Box */}
                <div className="w-full max-w-xl bg-slate-950/90 border border-slate-800/80 rounded-lg p-3 font-mono text-xs text-slate-300 mt-4 max-h-36 overflow-y-auto shadow-inner">
                  <div className="text-slate-500 mb-1 flex items-center justify-between border-b border-slate-800 pb-1">
                    <span>KERNEL STDOUT LOGS</span>
                    <span>time: {elapsedTime}</span>
                  </div>
                  {bootLogs.map((log, i) => (
                    <div key={i} className={log.startsWith('✓') ? 'text-emerald-400 font-bold' : log.startsWith('[OK]') ? 'text-cyan-300' : 'text-slate-400'}>
                      {log}
                    </div>
                  ))}
                </div>
              </div>

              {/* Monitor Controls Footer */}
              <div className="bg-slate-900 border-t border-slate-800 px-6 py-3 flex items-center justify-between text-xs">
                <div className="flex items-center gap-2 text-slate-400 font-mono">
                  <span className="w-2 h-2 rounded-full bg-emerald-500 animate-ping" />
                  Terminal: cargo run (boot module)
                </div>
                <div className="flex items-center gap-3">
                  <button
                    onClick={startBoot}
                    disabled={isBooting}
                    className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-cyan-600 hover:bg-cyan-500 disabled:opacity-50 text-white font-medium transition-all shadow-sm cursor-pointer"
                  >
                    <RotateCcw className="w-3.5 h-3.5" />
                    Reboot Simulation
                  </button>
                </div>
              </div>
            </div>
          </div>
        ) : (
          /* Rust Code Project Viewer */
          <div className="flex flex-col md:flex-row gap-4 border border-slate-800 bg-slate-900/60 rounded-xl overflow-hidden min-h-[550px]">
            {/* File Sidebar */}
            <div className="w-full md:w-64 bg-slate-950 border-r border-slate-800 p-3 flex flex-col gap-1">
              <div className="text-xs font-semibold text-slate-400 uppercase tracking-wider px-2 py-1 mb-1 font-mono">
                Project Files (boot/)
              </div>
              {Object.keys(RUST_FILES).map((fileName) => (
                <button
                  key={fileName}
                  onClick={() => setSelectedFile(fileName)}
                  className={`flex items-center gap-2 px-3 py-2 rounded-lg text-xs font-mono text-left transition-all ${
                    selectedFile === fileName
                      ? 'bg-cyan-950 text-cyan-300 border border-cyan-500/40'
                      : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'
                  }`}
                >
                  <Code className="w-3.5 h-3.5 shrink-0 text-cyan-400" />
                  {fileName}
                </button>
              ))}

              <div className="mt-auto pt-4 border-t border-slate-900 text-[11px] text-slate-500 p-2 font-mono">
                Pure Rust 2021 Edition<br />
                Zero external crate dependencies.
              </div>
            </div>

            {/* Code View Area */}
            <div className="flex-1 flex flex-col bg-slate-950">
              <div className="flex items-center justify-between border-b border-slate-800 px-4 py-2 bg-slate-900/40">
                <span className="text-xs font-mono text-cyan-400">boot/{selectedFile}</span>
                <button
                  onClick={copyCode}
                  className="flex items-center gap-1.5 px-2.5 py-1 rounded bg-slate-800 hover:bg-slate-700 text-xs text-slate-200 transition-all cursor-pointer font-mono"
                >
                  {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                  {copied ? 'Copied' : 'Copy Code'}
                </button>
              </div>
              <pre className="flex-1 p-4 font-mono text-xs text-slate-300 overflow-auto whitespace-pre leading-relaxed selection:bg-cyan-500/30">
                <code>{RUST_FILES[selectedFile]}</code>
              </pre>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}
