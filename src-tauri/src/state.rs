//! Application state management
//!
//! This module handles the global application state, including system monitoring
//! and process tracking capabilities.

use crate::monitoring::{ProcessMonitor, SystemMonitor};
use std::sync::Mutex;
use sysinfo::{Disks, Networks, System};

/// Global application state
///
/// Maintains thread-safe access to system information and monitoring components
#[derive(Debug)]
pub struct AppState {
    /// System information handler
    pub sys: Mutex<System>,
    /// Networks information handler
    pub networks: Mutex<Networks>,
    /// Networks information handler
    pub disks: Mutex<Disks>,
    /// Process monitoring component
    pub process_monitor: Mutex<ProcessMonitor>,
    /// System statistics monitoring component
    pub system_monitor: Mutex<SystemMonitor>,
}

impl AppState {
    /// Creates a new instance of the application state
    ///
    /// Initializes system monitoring and process tracking components
    ///
    /// # Returns
    ///
    /// A new `AppState` instance with initialized monitors
    pub fn new() -> Self {
        let sys = System::new_all();
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        Self {
            process_monitor: Mutex::new(ProcessMonitor::new()),
            system_monitor: Mutex::new(SystemMonitor::new(&networks)),
            sys: Mutex::new(sys),
            disks: Mutex::new(disks),
            networks: Mutex::new(networks),
        }
    }
}
