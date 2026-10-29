use super::{percent, GpuDevice};
use nvml_wrapper::{
    enum_wrappers::device::{Clock, TemperatureSensor},
    Nvml,
};
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Nvidia {
    library: Option<Nvml>,
    last_attempt: Option<Instant>,
}
impl Nvidia {
    pub fn collect(&mut self) -> Vec<GpuDevice> {
        if self.library.is_none()
            && self
                .last_attempt
                .is_none_or(|i| i.elapsed() >= Duration::from_secs(30))
        {
            self.last_attempt = Some(Instant::now());
            self.library = Nvml::init().ok();
        }
        let Some(nvml) = &self.library else {
            return vec![];
        };
        let Ok(count) = nvml.device_count() else {
            self.library = None;
            return vec![];
        };
        (0..count.min(8))
            .filter_map(|i| {
                let device = nvml.device_by_index(i).ok()?;
                let memory = device.memory_info().ok();
                #[cfg(target_os = "linux")]
                let id = device
                    .pci_info()
                    .ok()
                    .and_then(|p| {
                        let parts = p.bus_id.split_once(':')?;
                        let domain = u32::from_str_radix(parts.0, 16).ok()?;
                        Some(format!("pci:{domain:04x}:{}", parts.1.to_lowercase()))
                    })
                    .or_else(|| device.uuid().ok())?;
                #[cfg(not(target_os = "linux"))]
                let id = device.uuid().ok()?;
                Some(GpuDevice {
                    id,
                    name: device.name().unwrap_or_else(|_| "NVIDIA graphics".into()),
                    vendor: "NVIDIA".into(),
                    utilization: device
                        .utilization_rates()
                        .ok()
                        .and_then(|r| percent(r.gpu as f64)),
                    memory_used: memory.as_ref().map(|m| m.used),
                    memory_total: memory.as_ref().map(|m| m.total).filter(|v| *v > 0),
                    memory_kind: "dedicated".into(),
                    temperature: device
                        .temperature(TemperatureSensor::Gpu)
                        .ok()
                        .map(|v| v as f64),
                    power_watts: device.power_usage().ok().map(|v| v as f64 / 1000.0),
                    core_clock_mhz: device.clock_info(Clock::Graphics).ok().map(u64::from),
                    cores: None,
                    source: "NVIDIA NVML driver".into(),
                })
            })
            .collect()
    }
}
