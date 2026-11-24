use super::{percent, GpuDevice};
use crate::macos_io_registry::{dictionary_number, IoServices};
use core_foundation::{dictionary::CFDictionary, number::CFNumber, string::CFString};

fn number(dict: &CFDictionary, key: &str) -> Option<f64> {
    dictionary_number(dict, key)?.to_f64()
}
fn bytes(dict: &CFDictionary, key: &str) -> Option<u64> {
    number(dict, key)
        .filter(|v| v.is_finite() && *v >= 0.0)
        .map(|v| v as u64)
}

pub fn collect() -> Vec<GpuDevice> {
    let mut result = Vec::new();
    let Some(services) = IoServices::matching(c"IOAccelerator") else {
        return result;
    };
    for entry in services {
        let Some(id) = entry.id() else {
            continue;
        };
        let name = entry
            .property("model")
            .and_then(|v| v.downcast::<CFString>())
            .map(|s| s.to_string())
            .or_else(|| entry.name())
            .unwrap_or_else(|| "Graphics adapter".into());
        let apple = name.starts_with("Apple ");
        let stats = entry
            .property("PerformanceStatistics")
            .and_then(|v| v.downcast::<CFDictionary>());
        let utilization = stats
            .as_ref()
            .and_then(|d| number(d, "Device Utilization %"))
            .and_then(percent);
        let memory_used = stats.as_ref().and_then(|d| {
            bytes(
                d,
                if apple {
                    "In use system memory"
                } else {
                    "vramUsedBytes"
                },
            )
        });
        let cores = entry
            .property("gpu-core-count")
            .and_then(|v| v.downcast::<CFNumber>())
            .and_then(|v| v.to_i64())
            .and_then(|v| u64::try_from(v).ok());
        result.push(GpuDevice {
            id: format!("registry:{id:x}"),
            name: name.clone(),
            vendor: if apple {
                "Apple"
            } else if name.to_lowercase().contains("intel") {
                "Intel"
            } else if name.to_lowercase().contains("amd") || name.to_lowercase().contains("radeon")
            {
                "AMD"
            } else {
                "Unknown"
            }
            .into(),
            utilization,
            memory_used,
            memory_total: None,
            memory_kind: if apple { "shared" } else { "unknown" }.into(),
            temperature: None,
            power_watts: None,
            core_clock_mhz: None,
            cores,
            source: "macOS IORegistry driver statistics".into(),
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
    use core_foundation::base::TCFType;
    #[test]
    fn driver_properties_preserve_zero_and_reject_missing_or_wrong_types() {
        let values = CFDictionary::from_CFType_pairs(&[
            (
                CFString::new("Device Utilization %").as_CFType(),
                CFNumber::from(0_i64).as_CFType(),
            ),
            (
                CFString::new("In use system memory").as_CFType(),
                CFNumber::from(123456_i64).as_CFType(),
            ),
            (
                CFString::new("negative").as_CFType(),
                CFNumber::from(-1_i64).as_CFType(),
            ),
            (
                CFString::new("text").as_CFType(),
                CFString::new("unknown").as_CFType(),
            ),
        ])
        .into_untyped();
        assert_eq!(number(&values, "Device Utilization %"), Some(0.0));
        assert_eq!(bytes(&values, "In use system memory"), Some(123456));
        assert_eq!(bytes(&values, "negative"), None);
        assert_eq!(number(&values, "text"), None);
        assert_eq!(number(&values, "missing"), None);
    }
}
