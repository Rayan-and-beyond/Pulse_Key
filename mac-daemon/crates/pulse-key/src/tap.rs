use crate::config::*;
use std::collections::VecDeque;
use tokio::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapSignal {
    Tap { at: Instant },
    DoubleTap { at: Instant },
}

struct Cluster {
    start: Instant,
    last_spike: Instant,
    peak: f64,
}

pub struct TapDetector {
    mags: VecDeque<f64>,
    jerks: VecDeque<(Instant, f64)>,
    cluster: Option<Cluster>,
}

impl TapDetector {
    pub fn new() -> Self {
        Self { mags: VecDeque::new(), jerks: VecDeque::new(), cluster: None }
    }

    pub fn feed(&mut self, mag: f64, t: Instant) -> Option<TapSignal> {
        self.mags.push_back(mag);
        if self.mags.len() > 4 {
            self.mags.pop_front();
        }
        let jerk = if self.mags.len() == 4 { mag - self.mags[0] } else { 0.0 };

        self.jerks.push_back((t, jerk.abs()));
        while let Some(&(ts, _)) = self.jerks.front() {
            if t.duration_since(ts) > NOISE_WINDOW {
                self.jerks.pop_front();
            } else {
                break;
            }
        }

        if let Some(c) = &self.cluster {
            if t.duration_since(c.last_spike) > CLUSTER_GAP {
                self.cluster = None;
            }
        }

        let threshold = TAP_MIN_JERK.max(TAP_NOISE_MULT * Self::median(&self.jerks));
        if jerk <= threshold {
            return None;
        }

        match &mut self.cluster {
            Some(c) => {
                let gap = t.duration_since(c.last_spike);
                if gap < MIN_DOUBLE_GAP || (gap <= CLUSTER_GAP && jerk <= REBOUND_RATIO * c.peak) {
                    c.last_spike = t;
                    c.peak = c.peak.max(jerk);
                    None
                } else {
                    let prev_start = c.start;
                    *c = Cluster { start: t, last_spike: t, peak: jerk };
                    let start_gap = t.duration_since(prev_start);
                    Some(if start_gap >= MIN_DOUBLE_GAP && start_gap <= DOUBLE_TAP_WINDOW {
                        TapSignal::DoubleTap { at: t }
                    } else {
                        TapSignal::Tap { at: t }
                    })
                }
            }
            None => {
                self.cluster = Some(Cluster { start: t, last_spike: t, peak: jerk });
                Some(TapSignal::Tap { at: t })
            }
        }
    }

    fn median(jerks: &VecDeque<(Instant, f64)>) -> f64 {
        if jerks.is_empty() {
            return 0.0;
        }
        let mut v: Vec<f64> = jerks.iter().map(|&(_, j)| j).collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v[v.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::Instant;

    fn quiet(d: &mut TapDetector, t: &mut Instant, n: usize) {
        for _ in 0..n {
            d.feed(1000.0, *t);
            *t += Duration::from_millis(20);
        }
    }

    #[test]
    fn single_impulse_is_one_tap_no_double() {
        let mut d = TapDetector::new();
        let mut t = Instant::now();
        let mut sigs: Vec<Option<TapSignal>> = Vec::new();
        quiet(&mut d, &mut t, 300);
        sigs.push(d.feed(2600.0, t));
        t += Duration::from_millis(20);
        quiet(&mut d, &mut t, 200);
        let taps = sigs.iter().flatten().filter(|s| matches!(s, TapSignal::Tap { .. })).count();
        let doubles = sigs.iter().flatten().filter(|s| matches!(s, TapSignal::DoubleTap { .. })).count();
        assert_eq!((taps, doubles), (1, 0));
    }

    #[test]
    fn impulse_plus_weaker_rebound_is_one_tap() {
        let mut d = TapDetector::new();
        let mut t = Instant::now();
        let mut doubles = 0;
        quiet(&mut d, &mut t, 300);
        if let Some(TapSignal::DoubleTap { .. }) = d.feed(2600.0, t) { doubles += 1; }
        t += Duration::from_millis(330);
        if let Some(TapSignal::DoubleTap { .. }) = d.feed(1500.0, t) { doubles += 1; }
        t += Duration::from_millis(20);
        quiet(&mut d, &mut t, 200);
        assert_eq!(doubles, 0);
    }

    #[test]
    fn two_strong_spikes_350ms_apart_is_double_tap() {
        let mut d = TapDetector::new();
        let mut t = Instant::now();
        quiet(&mut d, &mut t, 300);
        assert!(matches!(d.feed(2600.0, t), Some(TapSignal::Tap { .. })));
        t += Duration::from_millis(350);
        assert!(matches!(d.feed(2600.0, t), Some(TapSignal::DoubleTap { .. })));
    }

    #[test]
    fn two_soft_spikes_250ms_apart_is_double_tap() {
        let mut d = TapDetector::new();
        let mut t = Instant::now();
        quiet(&mut d, &mut t, 300);
        assert!(matches!(d.feed(2600.0, t), Some(TapSignal::Tap { .. })));
        quiet(&mut d, &mut t, 12);
        t += Duration::from_millis(10);
        assert!(matches!(d.feed(1800.0, t), Some(TapSignal::DoubleTap { .. })));
    }

    #[test]
    fn two_calm_taps_two_seconds_apart_are_two_singles() {
        let mut d = TapDetector::new();
        let mut t = Instant::now();
        quiet(&mut d, &mut t, 300);
        assert!(matches!(d.feed(2600.0, t), Some(TapSignal::Tap { .. })));
        t += Duration::from_secs(2);
        assert!(matches!(d.feed(2600.0, t), Some(TapSignal::Tap { .. })));
    }

    #[test]
    fn below_threshold_jerk_is_ignored() {
        let mut d = TapDetector::new();
        let mut t = Instant::now();
        quiet(&mut d, &mut t, 300);
        assert!(d.feed(1100.0, t).is_none());
    }

    #[derive(serde::Deserialize)]
    #[cfg(feature = "private-fixtures")]
    struct FixtureSample {
        #[allow(dead_code)]
        i: u32,
        x: i16,
        y: i16,
        z: i16,
    }

    #[cfg(feature = "private-fixtures")]
    fn replay(fixture: &str) -> (usize, usize) {
        let mut d = TapDetector::new();
        let t0 = Instant::now();
        let (mut taps, mut doubles) = (0, 0);
        for (n, line) in fixture.lines().enumerate() {
            let s: FixtureSample = serde_json::from_str(line).unwrap();
            let mag = ((s.x as f64).powi(2) + (s.y as f64).powi(2) + (s.z as f64).powi(2)).sqrt();
            let t = t0 + Duration::from_secs_f64(n as f64 / ACCEL_RATE_HZ);
            match d.feed(mag, t) {
                Some(TapSignal::Tap { .. }) => taps += 1,
                Some(TapSignal::DoubleTap { .. }) => doubles += 1,
                None => {}
            }
        }
        (taps, doubles)
    }

    #[test]
    #[cfg(feature = "private-fixtures")]
    fn fixture_stillness_has_no_double_taps() {
        assert_eq!(replay(include_str!("../tests/fixtures/tap_test1.jsonl")), (4, 0));
    }

    #[test]
    #[cfg(feature = "private-fixtures")]
    fn fixture_tap_session_two_counts() {
        assert_eq!(replay(include_str!("../tests/fixtures/tap_test2.jsonl")), (5, 1));
    }

    #[test]
    #[cfg(feature = "private-fixtures")]
    fn fixture_tap_session_three_counts() {
        assert_eq!(replay(include_str!("../tests/fixtures/tap_test3.jsonl")), (20, 18));
    }

    #[test]
    #[cfg(feature = "private-fixtures")]
    fn fixture_2026_10_19_singles_counts() {
        assert_eq!(replay(include_str!("../tests/fixtures/doubletap_s1.jsonl")), (9, 0));
    }

    #[test]
    #[cfg(feature = "private-fixtures")]
    fn fixture_2026_10_19_doubles_counts() {
        assert_eq!(replay(include_str!("../tests/fixtures/doubletap_s2.jsonl")), (16, 5));
    }

    #[test]
    #[cfg(feature = "private-fixtures")]
    fn fixture_2026_10_19_stillness_counts() {
        assert_eq!(replay(include_str!("../tests/fixtures/doubletap_s3.jsonl")), (0, 0));
    }
}
