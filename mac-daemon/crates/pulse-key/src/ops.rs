use crate::engine::Event;
use oura_link::ble::BleTransport;
use oura_link::client::{AcmSample, HeartRateSample};
use oura_link::OuraClient;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

pub trait RingOps: Send + Sync {
    fn start_accel(
        &self,
        sink: mpsc::Sender<AcmSample>,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>>;

    fn stop_accel(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>>;

    fn start_hr(
        &self,
        sink: mpsc::Sender<HeartRateSample>,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>>;

    fn stop_hr(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>>;

    fn is_authed(&self) -> bool;
}

pub struct RealRingOps {
    client: Arc<OuraClient<BleTransport>>,
    authed: bool,
    cmd: mpsc::Sender<Event>,
    accel_task: Mutex<Option<JoinHandle<()>>>,
    hr_task: Mutex<Option<JoinHandle<()>>>,
}

impl RealRingOps {
    pub fn new(client: Arc<OuraClient<BleTransport>>, authed: bool, cmd: mpsc::Sender<Event>) -> Self {
        Self { client, authed, cmd, accel_task: Mutex::new(None), hr_task: Mutex::new(None) }
    }
}

impl RingOps for RealRingOps {
    fn start_accel(
        &self,
        sink: mpsc::Sender<AcmSample>,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>> {
        Box::pin(async move {
            if let Some(h) = self.accel_task.lock().unwrap().take() {
                h.abort();
            }
            let client = self.client.clone();
            let cmd = self.cmd.clone();
            let handle = tokio::spawn(async move {
                let _ = client
                    .stream_accelerometer(Duration::from_secs(crate::config::ACCEL_ARM_SECS), move |s| {
                        let _ = sink.try_send(s);
                    })
                    .await;
                let _ = cmd.clone().send(Event::AccelEnded).await;
            });
            *self.accel_task.lock().unwrap() = Some(handle);
            Ok(())
        })
    }

    fn stop_accel(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>> {
        Box::pin(async move {
            if let Some(h) = self.accel_task.lock().unwrap().take() {
                h.abort();
            }
            Ok(self.client.stop_realtime().await?)
        })
    }

    fn start_hr(
        &self,
        sink: mpsc::Sender<HeartRateSample>,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>> {
        Box::pin(async move {
            if let Some(h) = self.hr_task.lock().unwrap().take() {
                h.abort();
            }
            let client = self.client.clone();
            let cmd = self.cmd.clone();
            let handle = tokio::spawn(async move {
                let _ = client
                    .live_heart_rate(Duration::from_secs(3600), false, move |s| {
                        let _ = sink.try_send(s);
                    })
                    .await;
                let _ = cmd.clone().send(Event::HrEnded).await;
            });
            *self.hr_task.lock().unwrap() = Some(handle);
            Ok(())
        })
    }

    fn stop_hr(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + '_>> {
        Box::pin(async move {
            if let Some(h) = self.hr_task.lock().unwrap().take() {
                h.abort();
            }
            Ok(self.client.stop_live_hr().await?)
        })
    }

    fn is_authed(&self) -> bool {
        self.authed
    }
}
