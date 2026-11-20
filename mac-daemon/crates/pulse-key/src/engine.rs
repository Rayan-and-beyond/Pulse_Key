use crate::config::*;
use crate::ops::RingOps;
use crate::oura::{AcmSample, HeartRateSample};
use crate::tap::TapSignal;
use crate::verify::{Beat, CeremonyCtx, VerifierRegistry};
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, mpsc, watch};
use tokio::time::Instant;

#[derive(Debug, Clone, Serialize)]
pub struct Snapshot {
    pub state: String,
    pub reason: Option<String>,
    pub bpm: Option<u16>,
}

impl Snapshot {
    pub fn locked() -> Self {
        Self { state: "locked".into(), reason: None, bpm: None }
    }
}

pub enum Event {
    RequestUnlock,
    Tap(TapSignal),
    Beat(HeartRateSample),
    AccelEnded,
    HrEnded,
}

#[derive(Debug, PartialEq)]
pub enum SessionEnd {
    RingLost,
}

enum State {
    Locked,
    Armed { since: Instant },
    Measuring { since: Instant },
    Unlocked,
    Failed { reason: String, since: Instant },
}

enum TickAction {
    ArmTimeout,
    MeasuringEnd,
    PulseLoss,
    FailedExpired,
}

pub struct Engine {
    state: State,
    beats: Vec<Beat>,
    tap_after_arm: bool,
    last_beat_at: Option<Instant>,
    last_bpm: Option<u16>,
    verifiers: VerifierRegistry,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            state: State::Locked,
            beats: Vec::new(),
            tap_after_arm: false,
            last_beat_at: None,
            last_bpm: None,
            verifiers: VerifierRegistry::new(),
        }
    }

    fn snapshot(&self) -> Snapshot {
        match &self.state {
            State::Locked => Snapshot::locked(),
            State::Armed { .. } => Snapshot { state: "armed".into(), reason: None, bpm: None },
            State::Measuring { .. } => Snapshot { state: "measuring".into(), reason: None, bpm: None },
            State::Unlocked => Snapshot { state: "unlocked".into(), reason: None, bpm: self.last_bpm },
            State::Failed { reason, .. } => Snapshot {
                state: "locked".into(),
                reason: Some(reason.clone()),
                bpm: None,
            },
        }
    }

    fn publish(&self, snap: &watch::Sender<Snapshot>) {
        let _ = snap.send(self.snapshot());
    }

    fn fail(&mut self, reason: &str) {
        self.state = State::Failed { reason: reason.to_string(), since: Instant::now() };
    }

    async fn on_event(
        &mut self,
        event: Event,
        ops: &dyn RingOps,
        snap: &watch::Sender<Snapshot>,
        extras: &broadcast::Sender<String>,
        accel_tx: &mpsc::Sender<AcmSample>,
        hr_tx: &mpsc::Sender<HeartRateSample>,
    ) -> Option<SessionEnd> {
        match event {
            Event::RequestUnlock => {
                if matches!(&self.state, State::Locked | State::Failed { .. }) {
                    self.tap_after_arm = false;
                    self.beats.clear();
                    self.last_bpm = None;
                    self.last_beat_at = None;
                    if ops.start_accel(accel_tx.clone()).await.is_err() {
                        self.fail("ring unreachable");
                    } else {
                        self.state = State::Armed { since: Instant::now() };
                    }
                    self.publish(snap);
                }
            }
            Event::Tap(sig) => {
                if matches!(&self.state, State::Armed { .. }) {
                    match sig {
                        TapSignal::DoubleTap { .. } => {
                            self.tap_after_arm = true;
                            self.beats.clear();
                            let _ = ops.stop_accel().await;
                            if ops.start_hr(hr_tx.clone()).await.is_err() {
                                self.fail("ring unreachable");
                                self.publish(snap);
                                return None;
                            }
                            self.last_beat_at = None;
                            self.state = State::Measuring { since: Instant::now() };
                            self.publish(snap);
                        }
                        TapSignal::Tap { .. } => {
                            let _ = extras.send("{\"type\":\"tap\"}".to_string());
                        }
                    }
                }
            }
            Event::Beat(sample) => {
                let measuring = matches!(&self.state, State::Measuring { .. });
                let unlocked = matches!(&self.state, State::Unlocked);
                let beat = Beat { bpm: sample.bpm as f64, ibi_ms: sample.ibi_ms as f64 };
                if measuring {
                    self.beats.push(beat);
                }
                if !(measuring || unlocked) || !beat.is_plausible() {
                    return None;
                }
                self.last_beat_at = Some(Instant::now());
                self.last_bpm = Some(sample.bpm);
                if unlocked {
                    self.publish(snap);
                }
            }
            Event::AccelEnded => {
                if matches!(&self.state, State::Armed { .. }) {
                    self.fail("ring unreachable");
                    self.publish(snap);
                }
            }
            Event::HrEnded => {
                if matches!(&self.state, State::Measuring { .. } | State::Unlocked) {
                    let _ = ops.stop_hr().await;
                    self.beats.clear();
                    self.last_beat_at = None;
                    self.last_bpm = None;
                    self.fail("ring unreachable");
                    self.publish(snap);
                    return Some(SessionEnd::RingLost);
                }
            }
        }
        None
    }

    async fn on_tick(
        &mut self,
        ops: &dyn RingOps,
        snap: &watch::Sender<Snapshot>,
    ) -> Option<SessionEnd> {
        let now = Instant::now();
        let action = match &self.state {
            State::Armed { since } if now.duration_since(*since) >= ARM_TIMEOUT => {
                Some(TickAction::ArmTimeout)
            }
            State::Measuring { since } if now.duration_since(*since) >= MEASURE_WINDOW => {
                Some(TickAction::MeasuringEnd)
            }
            State::Unlocked
                if self.last_beat_at.map_or(true, |b| now.duration_since(b) >= PULSE_LOSS) =>
            {
                Some(TickAction::PulseLoss)
            }
            State::Failed { since, .. } if now.duration_since(*since) >= FAILED_HOLD => {
                Some(TickAction::FailedExpired)
            }
            _ => None,
        };

        match action {
            Some(TickAction::ArmTimeout) => {
                let _ = ops.stop_accel().await;
                self.fail("tap timeout");
                self.publish(snap);
            }
            Some(TickAction::MeasuringEnd) => {
                let ctx = CeremonyCtx {
                    authed: ops.is_authed(),
                    tap_after_arm: self.tap_after_arm,
                    beats: &self.beats,
                };
                let report = self.verifiers.run(&ctx);
                let recent_pulse = self.last_beat_at.is_some_and(|at| now.duration_since(at) < PULSE_LOSS);
                if report.passed && recent_pulse {
                    self.state = State::Unlocked;
                } else {
                    let reason = report.fail_reason.unwrap_or_else(|| "no pulse detected".into());
                    self.beats.clear();
                    let _ = ops.stop_hr().await;
                    self.fail(&reason);
                }
                self.publish(snap);
            }
            Some(TickAction::PulseLoss) => {
                let _ = ops.stop_hr().await;
                self.fail("ring removed");
                self.last_bpm = None;
                self.publish(snap);
            }
            Some(TickAction::FailedExpired) => {
                self.state = State::Locked;
                self.publish(snap);
            }
            None => {}
        }
        None
    }
}

pub async fn run(
    ops: Arc<dyn RingOps>,
    rx: &mut mpsc::Receiver<Event>,
    snap: &watch::Sender<Snapshot>,
    extras: &broadcast::Sender<String>,
    accel_tx: mpsc::Sender<AcmSample>,
    hr_tx: mpsc::Sender<HeartRateSample>,
) -> SessionEnd {
    let mut engine = Engine::new();
    let mut tick = tokio::time::interval(Duration::from_millis(500));
    engine.publish(snap);

    loop {
        tokio::select! {
            maybe = rx.recv() => {
                let Some(event) = maybe else { return SessionEnd::RingLost };
                if let Some(end) =
                    engine.on_event(event, ops.as_ref(), snap, extras, &accel_tx, &hr_tx).await
                {
                    return end;
                }
            }
            _ = tick.tick() => {
                if let Some(end) = engine.on_tick(ops.as_ref(), snap).await {
                    return end;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::RingOps;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Mutex;
    use tokio::sync::{broadcast, mpsc, watch};

    #[derive(Default)]
    struct MockRingOps {
        calls: Mutex<Vec<String>>,
        authed: Mutex<bool>,
        accel_sink: Mutex<Option<mpsc::Sender<crate::oura::AcmSample>>>,
        hr_sink: Mutex<Option<mpsc::Sender<crate::oura::HeartRateSample>>>,
        fail_accel: Mutex<bool>,
        hr_running: Mutex<bool>,
    }

    impl MockRingOps {
        fn new() -> Self {
            Self { authed: Mutex::new(true), ..Default::default() }
        }

        fn record(&self, s: &str) {
            self.calls.lock().unwrap().push(s.to_string());
        }

        fn calls(&self) -> Vec<String> {
            self.calls.lock().unwrap().clone()
        }

        fn hr_running(&self) -> bool {
            *self.hr_running.lock().unwrap()
        }
    }

    impl RingOps for MockRingOps {
        fn start_accel(
            &self,
            sink: mpsc::Sender<crate::oura::AcmSample>,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>> {
            Box::pin(async move {
                self.record("start_accel");
                if *self.fail_accel.lock().unwrap() {
                    anyhow::bail!("mock accel failure");
                }
                *self.accel_sink.lock().unwrap() = Some(sink);
                Ok(())
            })
        }

        fn stop_accel(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>> {
            Box::pin(async move {
                self.record("stop_accel");
                self.accel_sink.lock().unwrap().take();
                Ok(())
            })
        }

        fn start_hr(
            &self,
            sink: mpsc::Sender<crate::oura::HeartRateSample>,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>> {
            Box::pin(async move {
                self.record("start_hr");
                *self.hr_sink.lock().unwrap() = Some(sink);
                *self.hr_running.lock().unwrap() = true;
                Ok(())
            })
        }

        fn stop_hr(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>> {
            Box::pin(async move {
                self.record("stop_hr");
                self.hr_sink.lock().unwrap().take();
                *self.hr_running.lock().unwrap() = false;
                Ok(())
            })
        }

        fn is_authed(&self) -> bool {
            *self.authed.lock().unwrap()
        }
    }

    struct Harness {
        ops: Arc<MockRingOps>,
        cmd: mpsc::Sender<Event>,
        snap: watch::Receiver<Snapshot>,
        task: tokio::task::JoinHandle<SessionEnd>,
    }

    fn harness(fail_accel: bool) -> Harness {
        let ops = Arc::new(MockRingOps::new());
        *ops.fail_accel.lock().unwrap() = fail_accel;
        let (cmd, mut rx) = mpsc::channel(16);
        let (snap_tx, snap) = watch::channel(Snapshot::locked());
        let (extras, _) = broadcast::channel(16);
        let (accel_tx, _accel_rx) = mpsc::channel(8);
        let (hr_tx, _hr_rx) = mpsc::channel(8);
        let ops_clone = ops.clone();
        let task = tokio::spawn(async move {
            run(ops_clone, &mut rx, &snap_tx, &extras, accel_tx, hr_tx).await
        });
        Harness { ops, cmd, snap, task }
    }

    async fn settle() {
        tokio::time::advance(std::time::Duration::from_millis(50)).await;
    }

    fn beat() -> crate::oura::HeartRateSample {
        crate::oura::HeartRateSample { bpm: 72, ibi_ms: 833 }
    }

    async fn begin_measurement(h: &Harness) {
        h.cmd.send(Event::RequestUnlock).await.unwrap();
        settle().await;
        h.cmd
            .send(Event::Tap(TapSignal::DoubleTap { at: tokio::time::Instant::now() }))
            .await
            .unwrap();
        settle().await;
    }

    async fn unlock_via_tap(h: &Harness, beats: usize) {
        begin_measurement(h).await;
        tokio::time::advance(MEASURE_WINDOW - std::time::Duration::from_secs(2)).await;
        for _ in 0..beats {
            h.cmd.send(Event::Beat(beat())).await.unwrap();
        }
        settle().await;
        tokio::time::advance(std::time::Duration::from_millis(2600)).await;
        settle().await;
    }

    #[tokio::test(start_paused = true)]
    async fn happy_path_double_tap() {
        let h = harness(false);
        h.cmd.send(Event::RequestUnlock).await.unwrap();
        settle().await;
        assert_eq!(h.snap.borrow().state, "armed");
        assert!(h.ops.calls().contains(&"start_accel".to_string()));

        h.cmd
            .send(Event::Tap(TapSignal::DoubleTap { at: tokio::time::Instant::now() }))
            .await
            .unwrap();
        settle().await;
        assert_eq!(h.snap.borrow().state, "measuring");
        assert!(h.ops.calls().contains(&"stop_accel".to_string()));

        tokio::time::advance(MEASURE_WINDOW - std::time::Duration::from_secs(2)).await;
        for _ in 0..5 {
            h.cmd.send(Event::Beat(beat())).await.unwrap();
        }
        settle().await;
        tokio::time::advance(std::time::Duration::from_millis(2600)).await;
        settle().await;
        assert_eq!(h.snap.borrow().state, "unlocked");
        assert_eq!(h.snap.borrow().bpm, Some(72));
        assert!(h.ops.calls().contains(&"start_hr".to_string()));
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn measured_hr_starts_at_measuring_entry() {
        let h = harness(false);
        h.cmd.send(Event::RequestUnlock).await.unwrap();
        settle().await;
        h.cmd
            .send(Event::Tap(TapSignal::DoubleTap { at: tokio::time::Instant::now() }))
            .await
            .unwrap();
        settle().await;
        assert_eq!(h.snap.borrow().state, "measuring");
        assert!(h.ops.hr_running());
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn failed_measurement_stops_hr() {
        let h = harness(false);
        unlock_via_tap(&h, 0).await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("no pulse detected"));
        assert!(!h.ops.hr_running());
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn tap_timeout_shows_reason_then_clears() {
        let h = harness(false);
        h.cmd.send(Event::RequestUnlock).await.unwrap();
        settle().await;
        tokio::time::advance(ARM_TIMEOUT + std::time::Duration::from_millis(600)).await;
        settle().await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("tap timeout"));
        assert!(h.ops.calls().contains(&"stop_accel".to_string()));

        tokio::time::advance(FAILED_HOLD + std::time::Duration::from_millis(700)).await;
        settle().await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert!(h.snap.borrow().reason.is_none());
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn no_beats_fails_no_pulse() {
        let h = harness(false);
        unlock_via_tap(&h, 0).await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("no pulse detected"));
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn too_few_beats_fails_implausible() {
        let h = harness(false);
        unlock_via_tap(&h, 2).await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("pulse implausible"));
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn pulse_loss_relocks_after_unlock() {
        let h = harness(false);
        unlock_via_tap(&h, 5).await;
        assert_eq!(h.snap.borrow().state, "unlocked");

        for _ in 0..3 {
            h.cmd.send(Event::Beat(beat())).await.unwrap();
        }
        settle().await;

        tokio::time::advance(PULSE_LOSS + std::time::Duration::from_millis(1200)).await;
        settle().await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("ring removed"));
        assert!(h.ops.calls().contains(&"stop_hr".to_string()));

        tokio::time::advance(FAILED_HOLD + std::time::Duration::from_millis(700)).await;
        settle().await;
        assert!(h.snap.borrow().reason.is_none());
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn accel_failure_reports_ring_unreachable() {
        let h = harness(true);
        h.cmd.send(Event::RequestUnlock).await.unwrap();
        settle().await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("ring unreachable"));
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn hr_end_in_measuring_aborts_even_after_enough_beats() {
        let h = harness(false);
        begin_measurement(&h).await;
        for _ in 0..MIN_BEATS {
            h.cmd.send(Event::Beat(beat())).await.unwrap();
        }
        settle().await;
        h.cmd.send(Event::HrEnded).await.unwrap();
        assert_eq!(h.task.await.unwrap(), SessionEnd::RingLost);
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("ring unreachable"));
        assert!(!h.ops.hr_running());
    }

    #[tokio::test(start_paused = true)]
    async fn stale_beats_cannot_unlock() {
        let h = harness(false);
        begin_measurement(&h).await;
        for _ in 0..MIN_BEATS {
            h.cmd.send(Event::Beat(beat())).await.unwrap();
        }
        settle().await;
        tokio::time::advance(MEASURE_WINDOW + std::time::Duration::from_millis(600)).await;
        settle().await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("no pulse detected"));
        assert!(!h.ops.hr_running());
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn invalid_late_beat_cannot_refresh_stale_evidence() {
        let h = harness(false);
        begin_measurement(&h).await;
        for _ in 0..MIN_BEATS {
            h.cmd.send(Event::Beat(beat())).await.unwrap();
        }
        settle().await;
        tokio::time::advance(MEASURE_WINDOW - std::time::Duration::from_secs(1)).await;
        h.cmd.send(Event::Beat(HeartRateSample { bpm: 210, ibi_ms: 285 })).await.unwrap();
        settle().await;
        tokio::time::advance(std::time::Duration::from_millis(1600)).await;
        settle().await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("no pulse detected"));
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn unlocking_preserves_actual_last_beat_time() {
        let h = harness(false);
        begin_measurement(&h).await;
        tokio::time::advance(MEASURE_WINDOW - std::time::Duration::from_secs(3)).await;
        for _ in 0..MIN_BEATS {
            h.cmd.send(Event::Beat(beat())).await.unwrap();
        }
        settle().await;
        tokio::time::advance(std::time::Duration::from_millis(3600)).await;
        settle().await;
        assert_eq!(h.snap.borrow().state, "unlocked");
        tokio::time::advance(PULSE_LOSS - std::time::Duration::from_secs(2)).await;
        settle().await;
        assert_eq!(h.snap.borrow().state, "locked");
        assert_eq!(h.snap.borrow().reason.as_deref(), Some("ring removed"));
        h.task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn hr_end_while_unlocked_ends_session() {
        let h = harness(false);
        unlock_via_tap(&h, 5).await;
        assert_eq!(h.snap.borrow().state, "unlocked");
        h.cmd.send(Event::HrEnded).await.unwrap();
        let end = h.task.await.unwrap();
        assert!(matches!(end, SessionEnd::RingLost));
    }
}
