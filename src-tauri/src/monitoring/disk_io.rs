use super::{system_monitor::bytes_per_second, DiskIoRates};
use crate::macos_io_registry::{dictionary_number, IoServices};
use core_foundation::dictionary::CFDictionary;
use std::{collections::BTreeMap, time::Instant};

#[derive(Clone, Copy, Debug)]
struct DiskCounters {
    read: u64,
    written: u64,
}

type DiskSnapshot = BTreeMap<u64, DiskCounters>;

#[derive(Debug, Default)]
pub struct DiskIoMonitor {
    last: Option<(Instant, DiskSnapshot)>,
}

impl DiskIoMonitor {
    pub fn sample(&mut self) -> DiskIoRates {
        self.record(collect_counters(), Instant::now())
    }

    fn record(&mut self, current: Option<DiskSnapshot>, now: Instant) -> DiskIoRates {
        let previous = std::mem::replace(&mut self.last, current.map(|disks| (now, disks)));
        match (previous, self.last.as_ref()) {
            (Some((before, previous)), Some((_, current))) => {
                disk_rates(&previous, current, now.duration_since(before).as_secs_f64())
            }
            _ => DiskIoRates::default(),
        }
    }
}

fn collect_counters() -> Option<DiskSnapshot> {
    // A block-storage driver accounts for a whole disk, including its volumes.
    let mut services = IoServices::matching(c"IOBlockStorageDriver")?;
    let counters = services
        .by_ref()
        .map(|disk| {
            let id = disk.id()?;
            let stats = disk.property("Statistics")?.downcast::<CFDictionary>()?;
            let counter = |key| u64::try_from(dictionary_number(&stats, key)?.to_i64()?).ok();
            Some((
                id,
                DiskCounters {
                    read: counter("Bytes (Read)")?,
                    written: counter("Bytes (Write)")?,
                },
            ))
        })
        .collect::<Option<DiskSnapshot>>()?;
    // A hot-plug event can invalidate an iterator mid-scan.
    services.is_valid().then_some(counters)
}

fn disk_rates(previous: &DiskSnapshot, current: &DiskSnapshot, elapsed: f64) -> DiskIoRates {
    if current.is_empty() || !current.keys().eq(previous.keys()) {
        // Wait for two readings of the same set of devices.
        return DiskIoRates::default();
    }

    let rate = |counter: fn(&DiskCounters) -> u64| {
        let delta = current.iter().try_fold(0_u64, |total, (id, after)| {
            let before = previous.get(id)?;
            total.checked_add(counter(after).checked_sub(counter(before))?)
        })?;
        bytes_per_second(delta, elapsed)
    };

    DiskIoRates {
        read_bytes_per_second: rate(|disk| disk.read),
        write_bytes_per_second: rate(|disk| disk.written),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn disks(values: &[(u64, u64, u64)]) -> DiskSnapshot {
        values
            .iter()
            .map(|&(id, read, written)| (id, DiskCounters { read, written }))
            .collect()
    }

    #[test]
    fn totals_device_deltas_over_the_actual_sample_interval() {
        let before = disks(&[(1, 900, 800), (2, 100, 50)]);
        let after = disks(&[(1, 1050, 900), (2, 200, 50)]);
        let rates = disk_rates(&before, &after, 2.5);
        assert_eq!(rates.read_bytes_per_second, Some(100));
        assert_eq!(rates.write_bytes_per_second, Some(40));
        let idle = disk_rates(&after, &after, 2.5);
        assert_eq!(idle.read_bytes_per_second, Some(0));
        assert_eq!(idle.write_bytes_per_second, Some(0));
    }

    #[test]
    fn device_changes_and_counter_resets_do_not_create_spikes() {
        let before = disks(&[(1, 900, 800), (2, 100, 50)]);
        let replaced = disks(&[(1, 1000, 900), (3, 1_000_000, 1_000_000)]);
        assert_eq!(disk_rates(&before, &replaced, 1.0), DiskIoRates::default());
        let reset = disks(&[(1, 1000, 900), (2, 0, 50)]);
        let rates = disk_rates(&before, &reset, 1.0);
        assert_eq!(rates.read_bytes_per_second, None);
        assert_eq!(rates.write_bytes_per_second, Some(100));
    }

    #[test]
    fn a_failed_sample_requires_a_new_baseline() {
        let mut monitor = DiskIoMonitor::default();
        let start = Instant::now();
        let first = monitor.record(Some(disks(&[(1, 100, 200)])), start);
        let failed = monitor.record(None, start + Duration::from_secs(1));
        let resumed = monitor.record(
            Some(disks(&[(1, 300, 400)])),
            start + Duration::from_secs(2),
        );
        assert_eq!(first, DiskIoRates::default());
        assert_eq!(failed, DiskIoRates::default());
        assert_eq!(resumed, DiskIoRates::default());
        let next = monitor.record(
            Some(disks(&[(1, 400, 400)])),
            start + Duration::from_secs(3),
        );
        assert_eq!(next.read_bytes_per_second, Some(100));
        assert_eq!(next.write_bytes_per_second, Some(0));
    }
}
