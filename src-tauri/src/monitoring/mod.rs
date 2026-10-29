//! System monitoring functionality
//!
//! This module provides types and functionality for monitoring system resources
//! and processes. It includes process monitoring, system statistics collection,
//! and data structures for representing system state.

mod gpu_process_monitor;
mod process_monitor;
pub use gpu_process_monitor::GpuProcessMonitor;
mod system_monitor;
mod types;

pub use process_monitor::ProcessMonitor;
pub use system_monitor::SystemMonitor;
pub use types::*; // Re-export all types
