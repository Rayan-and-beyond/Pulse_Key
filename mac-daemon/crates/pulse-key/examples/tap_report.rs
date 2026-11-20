fn main() {
    for path in std::env::args().skip(1) {
        let text = std::fs::read_to_string(&path).unwrap();
        #[derive(serde::Deserialize)]
        struct S {
            #[allow(dead_code)]
            i: u32,
            x: i16,
            y: i16,
            z: i16,
        }
        let mut d = pulse_key::tap::TapDetector::new();
        let t0 = tokio::time::Instant::now();
        let (mut taps, mut doubles) = (0, 0);
        for (n, line) in text.lines().enumerate() {
            let s: S = serde_json::from_str(line).unwrap();
            let mag = ((s.x as f64).powi(2) + (s.y as f64).powi(2) + (s.z as f64).powi(2)).sqrt();
            let t = t0 + std::time::Duration::from_secs_f64(
                n as f64 / pulse_key::config::ACCEL_RATE_HZ,
            );
            match d.feed(mag, t) {
                Some(pulse_key::tap::TapSignal::Tap { .. }) => taps += 1,
                Some(pulse_key::tap::TapSignal::DoubleTap { .. }) => doubles += 1,
                None => {}
            }
        }
        println!("{path}: taps={taps} doubles={doubles}");
    }
}
