//! Experimental ANE power sampling for the validated M6 driver and macOS build.
mod ioreport;

use serde::Serialize;
use std::time::{Duration, Instant};

const WINDOW_MS: u64 = 60_000;

#[derive(Clone, Copy, Debug)]
pub struct Reading {
    joules: f64,
    duration: Duration,
}

impl Reading {
    fn watts(self) -> f64 {
        self.joules / self.duration.as_secs_f64()
    }
}

#[derive(Debug, Default)]
pub struct Monitor {
    previous: Option<(Instant, u64, u64)>,
}

impl Monitor {
    pub fn sample(&mut self, cadence: u64) -> Option<Reading> {
        let energy = ioreport::read_energy();
        self.record(energy, Instant::now(), cadence)
    }

    pub fn reset(&mut self) {
        self.previous = None;
    }

    fn record(&mut self, energy: Option<u64>, now: Instant, cadence: u64) -> Option<Reading> {
        let previous = std::mem::replace(&mut self.previous, energy.map(|n| (now, n, cadence)));
        let (then, previous_energy, previous_cadence) = previous?;
        let duration = now.checked_duration_since(then)?;
        let limit = Duration::from_millis((cadence.max(previous_cadence) * 5 / 2).min(WINDOW_MS));
        if duration.is_zero() || duration > limit {
            return None;
        }
        let delta = energy?.checked_sub(previous_energy)?;
        Some(Reading {
            joules: delta as f64 / 1e9,
            duration,
        })
    }
}

#[derive(Serialize)]
pub struct PowerPoint {
    timestamp: u64,
    watts: Option<f64>,
}

#[derive(Serialize)]
pub struct Overview {
    pub window_ms: u64,
    pub covered_ms: u64,
    pub watts: Option<f64>,
    pub average_watts: Option<f64>,
    pub peak_watts: Option<f64>,
    pub joules: Option<f64>,
    pub points: Vec<PowerPoint>,
}

impl Overview {
    pub fn from_history(
        end: u64,
        latest: Option<Reading>,
        history: impl Iterator<Item = (u64, Option<Reading>)>,
    ) -> Option<Self> {
        ioreport::supported().then(|| Self::summarize(end, latest, history))
    }

    fn summarize(
        end: u64,
        latest: Option<Reading>,
        history: impl Iterator<Item = (u64, Option<Reading>)>,
    ) -> Self {
        let start = end.saturating_sub(WINDOW_MS);
        let mut points = Vec::new();
        let mut energy = 0.0;
        let mut coverage = Duration::ZERO;
        let mut peak: Option<f64> = None;

        for (timestamp, reading) in history.filter(|(time, _)| (start..=end).contains(time)) {
            let watts = reading.map(Reading::watts);
            points.push(PowerPoint { timestamp, watts });
            if let Some(watts) = watts {
                peak = Some(peak.map_or(watts, |current| current.max(watts)));
            }
            // Sum only intervals wholly inside the window. A partially overlapping
            // counter delta cannot tell us when its energy was consumed.
            if let Some(reading) =
                reading.filter(|r| r.duration.as_millis() <= (timestamp - start) as u128)
            {
                energy += reading.joules;
                coverage += reading.duration;
            }
        }

        Self {
            window_ms: WINDOW_MS,
            covered_ms: coverage.as_millis() as u64,
            watts: latest.map(Reading::watts),
            average_watts: (!coverage.is_zero()).then(|| energy / coverage.as_secs_f64()),
            peak_watts: peak,
            joules: (!coverage.is_zero()).then_some(energy),
            points,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn powers_use_counter_deltas_and_keep_valid_zeroes() {
        let mut monitor = Monitor::default();
        let start = Instant::now();
        assert!(monitor.record(Some(9), start, 3000).is_none());
        let idle = monitor
            .record(Some(9), start + Duration::from_secs(3), 3000)
            .unwrap();
        assert_eq!(idle.watts(), 0.0);
        let active = monitor
            .record(Some(7_500_000_009), start + Duration::from_secs(6), 3000)
            .unwrap();
        assert_eq!(active.watts(), 2.5);
        assert_eq!(active.joules, 7.5);
    }

    #[test]
    fn reset_failed_and_stale_samples_do_not_fill_gaps() {
        let mut monitor = Monitor::default();
        let start = Instant::now();
        monitor.record(Some(10), start, 3000);
        assert!(monitor
            .record(Some(5), start + Duration::from_secs(2), 3000)
            .is_none());
        assert!(monitor
            .record(None, start + Duration::from_secs(4), 3000)
            .is_none());
        assert!(monitor
            .record(Some(9), start + Duration::from_secs(6), 3000)
            .is_none());
        assert!(monitor
            .record(Some(12), start + Duration::from_secs(20), 3000)
            .is_none());
        monitor.reset();
        assert!(monitor
            .record(Some(14), start + Duration::from_secs(21), 3000)
            .is_none());
    }

    #[test]
    fn energy_excludes_partial_and_missing_intervals() {
        let reading = Reading {
            joules: 6.0,
            duration: Duration::from_secs(3),
        };
        let history = [
            (12_000, Some(reading)),
            (15_000, Some(reading)),
            (18_000, None),
            (21_000, Some(reading)),
            (71_000, None),
        ];
        let summary = Overview::summarize(70_000, None, history.into_iter());
        assert_eq!(summary.points.len(), 4);
        assert!(summary.points[2].watts.is_none());
        assert_eq!(summary.covered_ms, 6000);
        assert_eq!(summary.joules, Some(12.0));
        assert_eq!(summary.average_watts, Some(2.0));
        assert_eq!(summary.peak_watts, Some(2.0));
        assert!(summary.watts.is_none());
    }
}
