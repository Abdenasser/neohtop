//! Read-only, driver-reported GPU observations. Missing counters stay missing.
use serde::Serialize;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(all(test, target_os = "macos"))]
#[path = "linux.rs"]
mod linux_fixtures;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(any(target_os = "linux", target_os = "windows"))]
mod nvidia;
#[cfg(target_os = "windows")]
mod windows;

#[derive(Clone, Debug, Serialize)]
pub struct GpuDevice {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub utilization: Option<f64>,
    pub memory_used: Option<u64>,
    pub memory_total: Option<u64>,
    pub memory_kind: String,
    pub temperature: Option<f64>,
    pub power_watts: Option<f64>,
    pub core_clock_mhz: Option<u64>,
    pub cores: Option<u64>,
    pub source: String,
}

pub fn percent(value: f64) -> Option<f64> {
    (value.is_finite() && (0.0..=100.0).contains(&value)).then_some(value)
}

#[derive(Debug, Default)]
pub struct GpuMonitor {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    nvidia: nvidia::Nvidia,
}

impl GpuMonitor {
    pub fn collect(&mut self) -> Vec<GpuDevice> {
        #[cfg(target_os = "macos")]
        let mut devices = macos::collect();
        #[cfg(any(target_os = "linux", target_os = "windows"))]
        let mut devices = {
            let mut devices = self.nvidia.collect();
            #[cfg(target_os = "linux")]
            devices.extend(linux::collect());
            #[cfg(target_os = "windows")]
            {
                let mut covered = devices
                    .iter()
                    .map(|d| d.name.trim_start_matches("NVIDIA ").to_lowercase())
                    .collect::<Vec<_>>();
                for device in windows::collect() {
                    if device.vendor == "NVIDIA" {
                        let name = device.name.trim_start_matches("NVIDIA ").to_lowercase();
                        if let Some(index) = covered.iter().position(|n| *n == name) {
                            covered.remove(index);
                            continue;
                        }
                    }
                    devices.push(device);
                }
            }
            devices
        };
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        let mut devices: Vec<GpuDevice> = Vec::new();
        devices.sort_by(|a, b| a.id.cmp(&b.id));
        devices.dedup_by(|a, b| a.id == b.id);
        devices.truncate(8);
        devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unavailable_is_not_idle() {
        assert_eq!(percent(0.0), Some(0.0));
        assert_eq!(percent(100.0), Some(100.0));
        for value in [-1.0, 101.0, f64::NAN, f64::INFINITY] {
            assert_eq!(percent(value), None);
        }
    }
}
