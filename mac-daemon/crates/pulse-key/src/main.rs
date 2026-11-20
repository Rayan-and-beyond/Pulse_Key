use anyhow::{Context, Result};
use clap::Parser;
use oura_link::ble::BleTransport;
use oura_link::OuraClient;
use pulse_key::config::{FAILED_HOLD, PORT, RECONNECT_EVERY};
use pulse_key::engine::{self, Event, Snapshot};
use pulse_key::ops::RealRingOps;
use pulse_key::oura::{AcmSample, HeartRateSample};
use pulse_key::server::{self, AppState};
use pulse_key::tap::TapDetector;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, watch};


#[derive(Parser, Debug)]
pub struct Args {
    /// Local 16-byte ring application-auth key file. Never store this file in Git.
    #[arg(long)]
    pub key_file: PathBuf,
    /// MAC address of the designated research ring, provided locally at runtime.
    #[arg(long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
    pub expected_mac: String,
    /// Serial number of the designated research ring, provided locally at runtime.
    #[arg(long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
    pub expected_serial: String,
    #[arg(long, default_value_t = PORT)]
    pub port: u16,
    #[arg(long, default_value = "vault.txt")]
    pub vault: PathBuf,
    #[arg(long, default_value = "Oura")]
    pub name: String,
    /// Previously verified transport ID of the retired experiment ring.
    #[arg(long, value_parser = clap::builder::NonEmptyStringValueParser::new())]
    pub address: String,
}

fn require_old_ring_mac(mac: &str, expected_mac: &str) -> Result<()> {
    anyhow::ensure!(mac.eq_ignore_ascii_case(expected_mac), "connected device is not the experiment ring");
    Ok(())
}

fn require_old_ring_serial(serial: &str, expected_serial: &str) -> Result<()> {
    anyhow::ensure!(serial == expected_serial, "authenticated device is not the experiment ring");
    Ok(())
}

fn load_key(path: &PathBuf) -> Result<[u8; 16]> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading key file {}", path.display()))?;
    let bytes = hex::decode(text.trim()).context("key file must be 32 hex chars")?;
    let key: [u8; 16] = bytes.as_slice().try_into().context("key must be 16 bytes")?;
    Ok(key)
}

async fn accel_pump(mut rx: mpsc::Receiver<AcmSample>, cmd: mpsc::Sender<Event>) {
    let mut detector = TapDetector::new();
    let mut stream_start = tokio::time::Instant::now();
    let mut last_arrival = tokio::time::Instant::now();
    let mut idx: u64 = 0;
    while let Some(sample) = rx.recv().await {
        let now = tokio::time::Instant::now();
        if now.duration_since(last_arrival) > std::time::Duration::from_millis(500) {
            detector = TapDetector::new();
            stream_start = now;
            idx = 0;
        }
        last_arrival = now;
        let t = stream_start
            + std::time::Duration::from_secs_f64(idx as f64 / pulse_key::config::ACCEL_RATE_HZ);
        idx += 1;
        if let Some(sig) = detector.feed(sample.magnitude(), t) {
            let _ = cmd.send(Event::Tap(sig)).await;
        }
    }
    let _ = cmd.send(Event::AccelEnded).await;
}

async fn hr_pump(mut rx: mpsc::Receiver<HeartRateSample>, cmd: mpsc::Sender<Event>) {
    while let Some(beat) = rx.recv().await {
        let _ = cmd.send(Event::Beat(beat)).await;
    }
    let _ = cmd.send(Event::HrEnded).await;
}

fn set_snapshot(snap: &watch::Sender<Snapshot>, s: Snapshot) {
    let _ = snap.send(s);
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,pulse_key=info,btleplug=off".into()),
        )
        .with_target(false)
        .init();

    let args = Args::parse();
    let key = load_key(&args.key_file)?;
    if !args.vault.exists() {
        std::fs::write(
            &args.vault,
            "Welcome, verified human.\nEverything after the pulse check is yours.\n",
        )
        .with_context(|| format!("creating {}", args.vault.display()))?;
    }

    let (snap_tx, snap_rx) = watch::channel(Snapshot::locked());
    let (cmd_tx, mut cmd_rx) = mpsc::channel(32);
    let (extras_tx, _) = broadcast::channel(64);
    let (accel_tx, accel_rx) = mpsc::channel(256);
    let (hr_tx, hr_rx) = mpsc::channel(64);

    tokio::spawn(accel_pump(accel_rx, cmd_tx.clone()));
    tokio::spawn(hr_pump(hr_rx, cmd_tx.clone()));
    server::spawn_broadcaster(snap_rx.clone(), extras_tx.clone());

    let ops_cmd_tx = cmd_tx.clone();
    let app = AppState {
        snapshot: snap_rx.clone(),
        cmd: cmd_tx,
        extras: extras_tx.clone(),
        vault: args.vault.clone(),
    };
    let listener = TcpListener::bind(("127.0.0.1", args.port)).await?;
    tracing::info!("pulse-key listening on http://127.0.0.1:{}", args.port);
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, server::router(app)).await {
            tracing::error!("server stopped: {e}");
        }
    });

    loop {
        match connect_and_run(
            &args,
            &key,
            &mut cmd_rx,
            &snap_tx,
            &extras_tx,
            accel_tx.clone(),
            hr_tx.clone(),
            ops_cmd_tx.clone(),
        )
        .await
        {
            Ok(()) => {}
            Err(e) => tracing::info!("ring session ended: {e:#}"),
        }
        set_snapshot(
            &snap_tx,
            Snapshot { state: "locked".into(), reason: Some("ring unreachable".into()), bpm: None },
        );
        tokio::time::sleep(FAILED_HOLD).await;
        set_snapshot(&snap_tx, Snapshot::locked());
        tokio::time::sleep(RECONNECT_EVERY).await;
    }
}

async fn connect_and_run(
    args: &Args,
    key: &[u8; 16],
    cmd_rx: &mut mpsc::Receiver<Event>,
    snap_tx: &watch::Sender<Snapshot>,
    extras_tx: &broadcast::Sender<String>,
    accel_tx: mpsc::Sender<AcmSample>,
    hr_tx: mpsc::Sender<HeartRateSample>,
    cmd_tx: mpsc::Sender<Event>,
) -> Result<()> {
    let transport = BleTransport::connect(&args.name, Some(&args.address), Duration::from_secs(10))
        .await
        .context("connecting to ring")?;
    let client = Arc::new(OuraClient::new(transport));
    let info = client.firmware().await.context("checking experiment ring before authentication")?;
    require_old_ring_mac(&info.mac, &args.expected_mac)?;
    client.authenticate(key).await.context("authenticating")?;
    let serial = client.serial().await.context("checking experiment ring before starting sensors")?;
    require_old_ring_serial(&serial, &args.expected_serial)?;
    let ops = Arc::new(RealRingOps::new(client, true, cmd_tx));
    let end = engine::run(ops, cmd_rx, snap_tx, extras_tx, accel_tx, hr_tx).await;
    tracing::info!("ring session ended: {end:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ARGS: &[&str] = &[
        "pulse-key",
        "--address", "verified-transport-id",
        "--key-file", "local-ring.key",
        "--expected-mac", "aa:bb:cc:dd:ee:ff",
        "--expected-serial", "TEST-DEVICE-001",
    ];

    #[test]
    fn device_identifiers_and_key_path_are_required() {
        assert!(Args::try_parse_from(["pulse-key"]).is_err());
        assert!(Args::try_parse_from([
            "pulse-key", "--address", "",
            "--key-file", "local-ring.key",
            "--expected-mac", "aa:bb:cc:dd:ee:ff",
            "--expected-serial", "TEST-DEVICE-001",
        ]).is_err());
        assert!(Args::try_parse_from(TEST_ARGS).is_ok());
    }

    #[test]
    fn explicit_transport_address_is_retained() {
        let args = Args::try_parse_from(TEST_ARGS).unwrap();
        assert_eq!(args.address, "verified-transport-id");
    }

    #[test]
    fn device_mac_must_match_explicit_ring() {
        assert!(require_old_ring_mac("aa:bb:cc:dd:ee:ff", "AA:BB:CC:DD:EE:FF").is_ok());
        assert!(require_old_ring_mac("00:00:00:00:00:00", "AA:BB:CC:DD:EE:FF").is_err());
    }

    #[test]
    fn serial_must_match_explicit_ring() {
        assert!(require_old_ring_serial("TEST-DEVICE-001", "TEST-DEVICE-001").is_ok());
        assert!(require_old_ring_serial("unrecognized-device", "TEST-DEVICE-001").is_err());
    }
}
