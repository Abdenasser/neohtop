use super::{percent, GpuDevice};
use core_foundation::{
    base::{CFAllocatorRef, CFType, CFTypeRef, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use std::ffi::{c_char, c_void, CStr};

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOServiceMatching(name: *const c_char) -> CFDictionaryRef;
    fn IOServiceGetMatchingServices(
        port: u32,
        matching: CFDictionaryRef,
        iterator: *mut u32,
    ) -> i32;
    fn IOIteratorNext(iterator: u32) -> u32;
    fn IOObjectRelease(object: u32) -> i32;
    fn IORegistryEntryGetRegistryEntryID(entry: u32, id: *mut u64) -> i32;
    fn IORegistryEntryCreateCFProperty(
        entry: u32,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: u32,
    ) -> CFTypeRef;
    fn IORegistryEntryGetName(entry: u32, name: *mut c_char) -> i32;
}

// Every iterator and service returned with an owned reference is released,
// including early returns and devices without a statistics dictionary.
struct IoObject(u32);
impl Drop for IoObject {
    fn drop(&mut self) {
        unsafe {
            IOObjectRelease(self.0);
        }
    }
}
fn property(entry: u32, key: &str) -> Option<CFType> {
    let key = CFString::new(key);
    let value = unsafe {
        IORegistryEntryCreateCFProperty(entry, key.as_concrete_TypeRef(), std::ptr::null(), 0)
    };
    (!value.is_null()).then(|| unsafe { CFType::wrap_under_create_rule(value) })
}
fn number(dict: &CFDictionary, key: &str) -> Option<f64> {
    let key = CFString::new(key);
    let pointer = *dict.find(key.as_concrete_TypeRef() as *const c_void)?;
    let value = unsafe { CFType::wrap_under_get_rule(pointer) };
    value.downcast::<CFNumber>()?.to_f64()
}
fn bytes(dict: &CFDictionary, key: &str) -> Option<u64> {
    number(dict, key)
        .filter(|v| v.is_finite() && *v >= 0.0)
        .map(|v| v as u64)
}

pub fn collect() -> Vec<GpuDevice> {
    let mut result = Vec::new();
    let matching = unsafe { IOServiceMatching(c"IOAccelerator".as_ptr()) };
    if matching.is_null() {
        return result;
    }
    let mut raw = 0;
    // IOServiceGetMatchingServices consumes the matching dictionary.
    if unsafe { IOServiceGetMatchingServices(0, matching, &mut raw) } != 0 || raw == 0 {
        return result;
    }
    let iterator = IoObject(raw);
    while result.len() < 8 {
        let raw = unsafe { IOIteratorNext(iterator.0) };
        if raw == 0 {
            break;
        }
        let entry = IoObject(raw);
        let mut id = 0;
        if unsafe { IORegistryEntryGetRegistryEntryID(entry.0, &mut id) } != 0 {
            continue;
        }
        let mut name_buffer = [0 as c_char; 128];
        let fallback = if unsafe { IORegistryEntryGetName(entry.0, name_buffer.as_mut_ptr()) } == 0
        {
            unsafe { CStr::from_ptr(name_buffer.as_ptr()) }
                .to_string_lossy()
                .into_owned()
        } else {
            "Graphics adapter".into()
        };
        let name = property(entry.0, "model")
            .and_then(|v| v.downcast::<CFString>())
            .map(|s| s.to_string())
            .unwrap_or(fallback);
        let apple = name.starts_with("Apple ");
        let stats =
            property(entry.0, "PerformanceStatistics").and_then(|v| v.downcast::<CFDictionary>());
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
        let cores = property(entry.0, "gpu-core-count")
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
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
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
