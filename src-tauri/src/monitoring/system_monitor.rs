//! System statistics monitoring
//!
//! This module handles collection and monitoring of system-wide statistics
//! including CPU, memory, network, and disk usage.

use super::SystemStats;
use std::fmt::Debug;
use std::path::Path;
use std::time::Instant;
use sysinfo::{Disk, Disks, Networks, System};

/// Monitors system-wide statistics
#[derive(Debug)]
pub struct SystemMonitor {
    /// Tracks network usage between updates
    last_network_update: (Instant, u64, u64),
}

impl SystemMonitor {
    /// Creates a new system monitor instance
    ///
    /// # Arguments
    ///
    /// * `sys` - System information provider for initial readings
    pub fn new(networks: &Networks) -> Self {
        let (initial_rx, initial_tx) =
            networks
                .iter()
                .fold((0, 0), |(initial_rx, initial_tx), (_, data)| {
                    (
                        initial_rx + data.total_received(),
                        initial_tx + data.total_transmitted(),
                    )
                });

        Self {
            last_network_update: (Instant::now(), initial_rx, initial_tx),
        }
    }

    /// Collects current system statistics
    ///
    /// # Arguments
    ///
    /// * `sys` - System information provider
    pub fn collect_stats(
        &mut self,
        sys: &System,
        networks: &Networks,
        disks: &Disks,
    ) -> SystemStats {
        let (network_rx, network_tx) = self.calculate_network_stats(networks);
        let (disk_total, disk_used, disk_free) = self.calculate_disk_stats(disks);
        let load_average = System::load_average();

        SystemStats {
            cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
            memory_total: sys.total_memory(),
            memory_used: sys.used_memory(),
            memory_free: sys.total_memory() - sys.used_memory(),
            memory_cached: sys.total_memory()
                - (sys.used_memory() + (sys.total_memory() - sys.used_memory())),
            uptime: System::uptime(),
            load_avg: [load_average.one, load_average.five, load_average.fifteen],
            network_rx_bytes: network_rx,
            network_tx_bytes: network_tx,
            disk_total_bytes: disk_total,
            disk_used_bytes: disk_used,
            disk_free_bytes: disk_free,
        }
    }

    /// Filters disks based on platform-specific criteria
    #[cfg(not(target_os = "windows"))]
    fn filter_disks(disks: &[Disk]) -> impl Iterator<Item = &Disk> {
        disks
            .iter()
            .filter(|disk| disk.mount_point() == Path::new("/"))
    }

    /// Windows-specific disk filtering
    #[cfg(target_os = "windows")]
    fn filter_disks(disks: &[Disk]) -> impl Iterator<Item = &Disk> {
        disks.iter()
    }

    /// Calculates network usage rates
    fn calculate_network_stats(&mut self, networks: &Networks) -> (u64, u64) {
        let (current_rx, current_tx) =
            networks
                .iter()
                .fold((0, 0), |(current_rx, current_tx), (_, data)| {
                    (
                        current_rx + data.total_received(),
                        current_tx + data.total_transmitted(),
                    )
                });

        let elapsed = self.last_network_update.0.elapsed().as_secs_f64();
        let rx_rate = ((current_rx - self.last_network_update.1) as f64 / elapsed) as u64;
        let tx_rate = ((current_tx - self.last_network_update.2) as f64 / elapsed) as u64;

        self.last_network_update = (Instant::now(), current_rx, current_tx);
        (rx_rate, tx_rate)
    }

    /// Calculates disk usage statistics and returns `(total, used, free)`.
    fn calculate_disk_stats(&self, disks: &Disks) -> (u64, u64, u64) {
        Self::filter_disks(disks).fold((0, 0, 0), |(total, used, free), disk| {
            (
                total + disk.total_space(),
                used + (disk.total_space() - disk.available_space()),
                free + disk.available_space(),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    /// Tests creation of system monitor
    #[test]
    fn test_system_monitor_creation() {
        let networks = Networks::new();
        let monitor = SystemMonitor::new(&networks);
        assert!(monitor.last_network_update.1 >= 0);
        assert!(monitor.last_network_update.2 >= 0);
    }

    /// Tests system statistics collection
    #[test]
    fn test_stats_collection() {
        let mut networks = Networks::new();
        let mut monitor = SystemMonitor::new(&networks);
        networks.refresh(true);
        let sys = System::new_all();
        let disks = Disks::new_with_refreshed_list();

        let stats = monitor.collect_stats(&sys, &networks, &disks);
        assert!(!stats.cpu_usage.is_empty());
        assert!(stats.memory_total > 0);
    }
}
