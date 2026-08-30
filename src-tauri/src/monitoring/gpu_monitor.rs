//! GPU statistics monitoring
//!
//! Collects utilization, video memory and sensor readings for the GPUs present
//! in the machine.
//!
//! On Linux the numbers are read straight out of the DRM sysfs interface, which
//! the `amdgpu` driver populates with everything the UI needs, so no additional
//! dependency is required. NVIDIA cards are read through `nvidia-smi` when it is
//! installed, on Linux and Windows alike. Cards that report neither a
//! utilization nor a memory figure are left out, and the panel disappears when
//! nothing at all is found.

use super::GpuInfo;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Root of the DRM class in sysfs, where every card exposes its driver files
const DRM_PATH: &str = "/sys/class/drm";

/// Locations the distributions ship the PCI id database at
const PCI_IDS_PATHS: [&str; 3] = [
    "/usr/share/hwdata/pci.ids",
    "/usr/share/misc/pci.ids",
    "/usr/share/pci.ids",
];

/// Fields requested from `nvidia-smi`, in the order they come back
const NVIDIA_SMI_QUERY: &str =
    "name,utilization.gpu,memory.total,memory.used,temperature.gpu,power.draw,clocks.current.sm";

/// Monitors the GPUs installed in the machine
#[derive(Debug)]
pub struct GpuMonitor {
    /// Cards found at start-up; GPUs are not hot-plugged, so we only look once
    devices: Vec<GpuDevice>,
    /// Whether any card is read through `nvidia-smi`
    uses_nvidia_smi: bool,
}

/// A GPU we know how to read, together with the details that never change
#[derive(Debug)]
struct GpuDevice {
    /// Model name, e.g. "Radeon RX 6600/6600 XT/6600M"
    name: String,
    /// Vendor name, e.g. "AMD"
    vendor: String,
    /// Where this card's live readings come from
    source: Source,
}

/// Backend used to read a card's live values
#[derive(Debug)]
enum Source {
    /// Driver files under `/sys/class/drm/cardN/device`, as exposed by `amdgpu`
    Sysfs {
        /// The card's PCI device directory
        device: PathBuf,
        /// The card's hwmon directory, when it publishes one
        hwmon: Option<PathBuf>,
    },
    /// A row of `nvidia-smi` output, identified by its position
    NvidiaSmi {
        /// Index of the card in the `nvidia-smi` listing
        index: usize,
    },
}

impl GpuMonitor {
    /// Discovers the GPUs that can be monitored
    ///
    /// Enumeration happens once, at start-up: the set of cards does not change
    /// while the application runs, and it keeps the per-refresh work to a few
    /// small file reads.
    pub fn new() -> Self {
        let mut devices = discover_sysfs_devices();
        devices.extend(discover_nvidia_devices());

        let uses_nvidia_smi = devices
            .iter()
            .any(|device| matches!(device.source, Source::NvidiaSmi { .. }));

        Self {
            devices,
            uses_nvidia_smi,
        }
    }

    /// Collects the current readings for every known GPU
    pub fn collect(&self) -> Vec<GpuInfo> {
        // `nvidia-smi` reports every card at once, so it is run a single time
        // per refresh rather than once per card
        let nvidia_rows = if self.uses_nvidia_smi {
            query_nvidia_smi()
        } else {
            Vec::new()
        };

        self.devices
            .iter()
            .map(|device| {
                let mut info = GpuInfo {
                    name: device.name.clone(),
                    vendor: device.vendor.clone(),
                    utilization: None,
                    memory_total: None,
                    memory_used: None,
                    temperature: None,
                    power_watts: None,
                    core_clock_mhz: None,
                };

                match &device.source {
                    Source::Sysfs { device, hwmon } => {
                        read_sysfs_stats(device, hwmon.as_deref(), &mut info)
                    }
                    Source::NvidiaSmi { index } => {
                        if let Some(row) = nvidia_rows.get(*index) {
                            read_nvidia_stats(row, &mut info);
                        }
                    }
                }

                info
            })
            .collect()
    }
}

/// Finds the DRM cards that publish usable statistics
fn discover_sysfs_devices() -> Vec<GpuDevice> {
    let Ok(entries) = fs::read_dir(DRM_PATH) else {
        return Vec::new();
    };

    // Connector directories share the prefix ("card1-DP-1"), so only entries
    // whose suffix is purely numeric are actual cards
    let mut cards: Vec<(u32, PathBuf)> = entries
        .flatten()
        .filter_map(|entry| {
            let index = entry
                .file_name()
                .to_str()?
                .strip_prefix("card")?
                .parse()
                .ok()?;
            Some((index, entry.path()))
        })
        .collect();

    // The kernel does not hand out card numbers in any particular order, and
    // read_dir does not sort, so order them for a stable panel layout
    cards.sort_by_key(|(index, _)| *index);

    cards
        .into_iter()
        .filter_map(|(_, card)| describe_sysfs_device(&card.join("device")))
        .collect()
}

/// Builds a device entry for a DRM card, if it reports anything we can show
fn describe_sysfs_device(device: &Path) -> Option<GpuDevice> {
    // Drivers publishing neither utilization nor video memory (Intel's i915 and
    // xe, nouveau, and NVIDIA's proprietary DRM node) have nothing to offer
    // here, so they are skipped rather than shown as an empty panel
    if !device.join("gpu_busy_percent").exists() && !device.join("mem_info_vram_total").exists() {
        return None;
    }

    let vendor_id = read_hex_id(&device.join("vendor"));
    let device_id = read_hex_id(&device.join("device"));

    let name = vendor_id
        .zip(device_id)
        .and_then(|(vendor, model)| lookup_pci_name(vendor, model))
        .unwrap_or_else(|| match (vendor_id, device_id) {
            (Some(vendor), Some(model)) => format!("{vendor:04x}:{model:04x}"),
            _ => "Unknown GPU".to_string(),
        });

    Some(GpuDevice {
        name,
        vendor: vendor_name(vendor_id),
        source: Source::Sysfs {
            device: device.to_path_buf(),
            hwmon: find_hwmon(device),
        },
    })
}

/// Locates the hwmon directory a card publishes its sensors under
fn find_hwmon(device: &Path) -> Option<PathBuf> {
    // The hwmon index is handed out at probe time, so it has to be looked up
    // rather than assumed
    fs::read_dir(device.join("hwmon"))
        .ok()?
        .flatten()
        .find(|entry| entry.file_name().to_string_lossy().starts_with("hwmon"))
        .map(|entry| entry.path())
}

/// Fills in the live readings a DRM card publishes in sysfs
fn read_sysfs_stats(device: &Path, hwmon: Option<&Path>, info: &mut GpuInfo) {
    info.utilization = read_number(&device.join("gpu_busy_percent")).map(|value| value as f32);
    info.memory_total = read_number(&device.join("mem_info_vram_total"));
    info.memory_used = read_number(&device.join("mem_info_vram_used"));

    let Some(hwmon) = hwmon else {
        return;
    };

    info.temperature = read_temperature(hwmon);

    // hwmon reports power in microwatts; `power1_average` is what amdgpu fills
    // in, while some cards only provide the instantaneous `power1_input`
    info.power_watts = read_number(&hwmon.join("power1_average"))
        .or_else(|| read_number(&hwmon.join("power1_input")))
        .map(|value| value as f32 / 1_000_000.0);

    // freq1_input is the shader clock, in hertz
    info.core_clock_mhz =
        read_number(&hwmon.join("freq1_input")).map(|value| (value / 1_000_000) as u32);
}

/// Reads a card's edge temperature in degrees Celsius
fn read_temperature(hwmon: &Path) -> Option<f32> {
    // amdgpu exposes edge, junction and memory sensors in no fixed order; the
    // edge reading is the one normally quoted as "the GPU temperature"
    let labelled_edge = (1..=3).find(|index| {
        fs::read_to_string(hwmon.join(format!("temp{index}_label")))
            .is_ok_and(|label| label.trim() == "edge")
    });

    let input = hwmon.join(format!("temp{}_input", labelled_edge.unwrap_or(1)));

    // hwmon reports temperatures in millidegrees
    read_number(&input).map(|value| value as f32 / 1000.0)
}

/// Reads a single unsigned integer out of a sysfs file
fn read_number(path: &Path) -> Option<u64> {
    fs::read_to_string(path).ok()?.trim().parse().ok()
}

/// Reads a sysfs PCI id such as "0x1002"
fn read_hex_id(path: &Path) -> Option<u16> {
    let raw = fs::read_to_string(path).ok()?;
    u16::from_str_radix(raw.trim().trim_start_matches("0x"), 16).ok()
}

/// Maps a PCI vendor id to a display name
fn vendor_name(vendor_id: Option<u16>) -> String {
    match vendor_id {
        Some(0x1002) => "AMD",
        Some(0x10de) => "NVIDIA",
        Some(0x8086) => "Intel",
        _ => "Unknown",
    }
    .to_string()
}

/// Resolves a PCI vendor/device pair to a model name through the system's
/// `pci.ids` database, the same one `lspci` reads
///
/// The file lists vendors at column zero, their devices one tab in, and
/// subsystem overrides two tabs in.
fn lookup_pci_name(vendor_id: u16, device_id: u16) -> Option<String> {
    let database = PCI_IDS_PATHS
        .iter()
        .find_map(|path| fs::read_to_string(path).ok())?;

    let vendor_prefix = format!("{vendor_id:04x}");
    let device_prefix = format!("{device_id:04x}");
    let mut in_vendor = false;

    for line in database.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        match line.strip_prefix('\t') {
            // A device (or, with a further tab, a subsystem) of the current vendor
            Some(device) if in_vendor => {
                if let Some(name) = device.strip_prefix(&device_prefix) {
                    if name.starts_with(' ') {
                        return Some(shorten_model_name(name.trim()));
                    }
                }
            }
            Some(_) => {}
            // A vendor line: once we have walked past our own block, give up
            None => {
                if in_vendor {
                    return None;
                }
                in_vendor = line
                    .strip_prefix(&vendor_prefix)
                    .is_some_and(|rest| rest.starts_with(' '));
            }
        }
    }

    None
}

/// Trims a `pci.ids` entry down to the marketing name where it carries one, so
/// that "Navi 23 [Radeon RX 6600/6600 XT/6600M]" becomes
/// "Radeon RX 6600/6600 XT/6600M"
fn shorten_model_name(name: &str) -> String {
    match (name.find('['), name.rfind(']')) {
        (Some(open), Some(close)) if close > open + 1 => name[open + 1..close].to_string(),
        _ => name.to_string(),
    }
}

/// Lists the NVIDIA GPUs `nvidia-smi` can see, if it is installed at all
fn discover_nvidia_devices() -> Vec<GpuDevice> {
    query_nvidia_smi()
        .into_iter()
        .enumerate()
        .map(|(index, row)| GpuDevice {
            name: row
                .first()
                .filter(|name| !name.is_empty())
                .cloned()
                .unwrap_or_else(|| "NVIDIA GPU".to_string()),
            vendor: "NVIDIA".to_string(),
            source: Source::NvidiaSmi { index },
        })
        .collect()
}

/// Runs `nvidia-smi` and returns one row of fields per GPU
///
/// An absent or failing `nvidia-smi` simply yields no rows, which is the normal
/// case on machines without an NVIDIA card.
fn query_nvidia_smi() -> Vec<Vec<String>> {
    let mut command = Command::new("nvidia-smi");
    command.args([
        format!("--query-gpu={NVIDIA_SMI_QUERY}").as_str(),
        "--format=csv,noheader,nounits",
    ]);
    hide_console_window(&mut command);

    let Ok(output) = command.output() else {
        return Vec::new();
    };

    if !output.status.success() {
        return Vec::new();
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().to_string())
                .collect()
        })
        .collect()
}

/// Applies one row of `nvidia-smi` output to a card's readings
///
/// Values the driver cannot supply come back as "[N/A]" and fail to parse,
/// which leaves the corresponding field empty.
fn read_nvidia_stats(row: &[String], info: &mut GpuInfo) {
    let field = |index: usize| row.get(index).and_then(|value| value.parse::<f64>().ok());

    info.utilization = field(1).map(|value| value as f32);
    // `nvidia-smi` reports memory in mebibytes
    info.memory_total = field(2).map(|value| (value * 1024.0 * 1024.0) as u64);
    info.memory_used = field(3).map(|value| (value * 1024.0 * 1024.0) as u64);
    info.temperature = field(4).map(|value| value as f32);
    info.power_watts = field(5).map(|value| value as f32);
    info.core_clock_mhz = field(6).map(|value| value as u32);
}

/// Keeps a console window from flashing up on Windows when `nvidia-smi` runs
#[cfg(target_os = "windows")]
fn hide_console_window(command: &mut Command) {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}

/// No-op outside Windows, where spawning a process shows nothing
#[cfg(not(target_os = "windows"))]
fn hide_console_window(_command: &mut Command) {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Discovery must succeed on every platform, with or without a GPU
    #[test]
    fn test_gpu_monitor_creation() {
        let monitor = GpuMonitor::new();
        assert_eq!(monitor.devices.len(), monitor.collect().len());
    }

    /// Readings must stay inside their documented ranges
    #[test]
    fn test_collected_stats_are_plausible() {
        for gpu in GpuMonitor::new().collect() {
            assert!(!gpu.name.is_empty());
            assert!(gpu.utilization.is_some() || gpu.memory_total.is_some());

            if let Some(utilization) = gpu.utilization {
                assert!((0.0..=100.0).contains(&utilization));
            }
            if let (Some(used), Some(total)) = (gpu.memory_used, gpu.memory_total) {
                assert!(used <= total);
            }
        }
    }

    /// The marketing name in brackets is preferred over the code name
    #[test]
    fn test_shorten_model_name() {
        assert_eq!(
            shorten_model_name("Navi 23 [Radeon RX 6600/6600 XT/6600M]"),
            "Radeon RX 6600/6600 XT/6600M"
        );
        assert_eq!(shorten_model_name("Some Plain Name"), "Some Plain Name");
        assert_eq!(shorten_model_name("Empty []"), "Empty []");
    }
}
