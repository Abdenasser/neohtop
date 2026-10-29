use super::GpuDevice;
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, IDXGIFactory1, DXGI_ADAPTER_FLAG_SOFTWARE,
};

pub fn collect() -> Vec<GpuDevice> {
    let mut result = Vec::new();
    // Enumeration only. DXGI capacity is not used as a utilization counter.
    unsafe {
        let Ok(factory) = CreateDXGIFactory1::<IDXGIFactory1>() else {
            return result;
        };
        for index in 0..32 {
            let Ok(adapter) = factory.EnumAdapters1(index) else {
                break;
            };
            let Ok(desc) = adapter.GetDesc1() else {
                continue;
            };
            if desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32 != 0 {
                continue;
            }
            let end = desc
                .Description
                .iter()
                .position(|v| *v == 0)
                .unwrap_or(desc.Description.len());
            result.push(GpuDevice {
                id: format!(
                    "luid:{:x}:{:x}",
                    desc.AdapterLuid.HighPart, desc.AdapterLuid.LowPart
                ),
                name: String::from_utf16_lossy(&desc.Description[..end]),
                vendor: match desc.VendorId {
                    0x1002 => "AMD",
                    0x8086 => "Intel",
                    0x10de => "NVIDIA",
                    _ => "Unknown",
                }
                .into(),
                utilization: None,
                memory_used: None,
                memory_total: (desc.DedicatedVideoMemory > 0)
                    .then_some(desc.DedicatedVideoMemory as u64),
                memory_kind: if desc.DedicatedVideoMemory > 0 {
                    "dedicated"
                } else {
                    "shared"
                }
                .into(),
                temperature: None,
                cores: None,
                source: "Windows DXGI inventory; live counters unavailable for this adapter".into(),
            });
            if result.len() == 8 {
                break;
            }
        }
    }
    result
}
