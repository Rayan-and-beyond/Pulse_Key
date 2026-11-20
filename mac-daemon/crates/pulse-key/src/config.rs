use std::time::Duration;

pub const PORT: u16 = 8787;

pub const ACCEL_RATE_HZ: f64 = 49.5;
pub const ACCEL_ARM_SECS: u64 = 35;

pub const ARM_TIMEOUT: Duration = Duration::from_secs(30);
pub const MEASURE_WINDOW: Duration = Duration::from_secs(18);
pub const MIN_BEATS: usize = 4;

pub const IBI_MIN_MS: f64 = 350.0;
pub const IBI_MAX_MS: f64 = 1900.0;
pub const BPM_MIN: f64 = 40.0;
pub const BPM_MAX: f64 = 200.0;

pub const PULSE_LOSS: Duration = Duration::from_secs(10);
pub const FAILED_HOLD: Duration = Duration::from_secs(5);
pub const RECONNECT_EVERY: Duration = Duration::from_secs(5);

pub const DOUBLE_TAP_WINDOW: Duration = Duration::from_millis(1000);
pub const CLUSTER_GAP: Duration = Duration::from_millis(450);
pub const MIN_DOUBLE_GAP: Duration = Duration::from_millis(150);
pub const REBOUND_RATIO: f64 = 0.35;

pub const TAP_MIN_JERK: f64 = 150.0;
pub const TAP_NOISE_MULT: f64 = 5.0;
pub const NOISE_WINDOW: Duration = Duration::from_secs(5);
