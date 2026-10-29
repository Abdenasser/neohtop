use super::{percent, GpuDevice};
use std::{collections::HashSet, fs, path::Path};
fn read(path: &Path, name: &str) -> Option<String> {
    fs::read_to_string(path.join(name))
        .ok()
        .map(|s| s.trim().to_owned())
}
fn integer(path: &Path, name: &str) -> Option<u64> {
    read(path, name)?.parse().ok()
}

#[cfg_attr(test, allow(dead_code))]
pub fn collect() -> Vec<GpuDevice> {
    collect_from(Path::new("/sys/class/drm"))
}
fn collect_from(root: &Path) -> Vec<GpuDevice> {
    let Ok(entries) = fs::read_dir(root) else {
        return vec![];
    };
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !name
            .strip_prefix("card")
            .is_some_and(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
        {
            continue;
        }
        let Ok(path) = fs::canonicalize(entry.path().join("device")) else {
            continue;
        };
        if !seen.insert(path.clone()) {
            continue;
        }
        let vendor_id = read(&path, "vendor").unwrap_or_default();
        let vendor = match vendor_id.as_str() {
            "0x1002" => "AMD",
            "0x8086" => "Intel",
            "0x10de" => "NVIDIA",
            _ => "Unknown",
        };
        let amd = fs::read_link(path.join("driver"))
            .ok()
            .is_some_and(|p| p.file_name().is_some_and(|n| n == "amdgpu"));
        let pci = path.file_name().unwrap_or_default().to_string_lossy();
        let name = read(&path, "product_name")
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| {
                format!(
                    "{vendor} graphics ({})",
                    read(&path, "device").unwrap_or_else(|| pci.into_owned())
                )
            });
        let temperature = if amd {
            fs::read_dir(path.join("hwmon")).ok().and_then(|entries| {
                entries.flatten().find_map(|e| {
                    if read(&e.path(), "name").as_deref() != Some("amdgpu") {
                        return None;
                    }
                    integer(&e.path(), "temp1_input")
                        .map(|v| v as f64 / 1000.0)
                        .filter(|v| *v <= 150.0)
                })
            })
        } else {
            None
        };
        result.push(GpuDevice {
            id: format!(
                "pci:{}",
                path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase()
            ),
            name,
            vendor: vendor.into(),
            utilization: if amd {
                integer(&path, "gpu_busy_percent").and_then(|v| percent(v as f64))
            } else {
                None
            },
            memory_used: if amd {
                integer(&path, "mem_info_vram_used")
            } else {
                None
            },
            memory_total: if amd {
                integer(&path, "mem_info_vram_total").filter(|v| *v > 0)
            } else {
                None
            },
            memory_kind: if amd { "dedicated" } else { "unknown" }.into(),
            temperature,
            power_watts: hwmon_number(&path, "power1_average")
                .or_else(|| hwmon_number(&path, "power1_input"))
                .map(|v| v as f64 / 1_000_000.0),
            core_clock_mhz: hwmon_number(&path, "freq1_input").map(|v| v / 1_000_000),
            cores: None,
            source: if amd {
                "Linux amdgpu sysfs"
            } else {
                "Linux DRM inventory; utilization not exposed"
            }
            .into(),
        });
        if result.len() == 8 {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn amd_fixture_deduplicates_cards_and_preserves_missing_counters() {
        use std::os::unix::fs::symlink;
        let root = std::env::temp_dir().join(format!("neohtop-gpu-fixture-{}", std::process::id()));
        let pci = root.join("0000:01:00.0");
        let drm = root.join("drm");
        let driver = root.join("amdgpu");
        fs::create_dir_all(&pci).unwrap();
        fs::create_dir_all(&driver).unwrap();
        for card in ["card0", "card1"] {
            fs::create_dir_all(drm.join(card)).unwrap();
            symlink(&pci, drm.join(card).join("device")).unwrap();
        }
        symlink(&driver, pci.join("driver")).unwrap();
        for (key, value) in [
            ("vendor", "0x1002"),
            ("product_name", "AMD fixture"),
            ("gpu_busy_percent", "0"),
            ("mem_info_vram_used", "0"),
            ("mem_info_vram_total", "8589934592"),
        ] {
            fs::write(pci.join(key), value).unwrap();
        }
        let devices = collect_from(&drm);
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].utilization, Some(0.0));
        assert_eq!(devices[0].memory_used, Some(0));
        assert_eq!(devices[0].memory_total, Some(8589934592));
        fs::write(pci.join("gpu_busy_percent"), "-1").unwrap();
        fs::remove_file(pci.join("mem_info_vram_used")).unwrap();
        let devices = collect_from(&drm);
        assert_eq!(devices[0].utilization, None);
        assert_eq!(devices[0].memory_used, None);
        fs::remove_dir_all(root).unwrap();
    }
}

fn hwmon_number(device: &Path, field: &str) -> Option<u64> {
    fs::read_dir(device.join("hwmon"))
        .ok()?
        .flatten()
        .find_map(|entry| integer(&entry.path(), field))
}
