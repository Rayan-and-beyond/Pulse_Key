# Rust Pulse Key daemon

This directory contains the actual Ring 4 tap/liveness authorization demonstration, imported from a user-supplied ZIP.

## Build

```bash
cd mac-daemon
cargo test --workspace
cargo build --workspace
```

The Rust Cargo workspace and pinned upstream `oura-link`/ `oura-protocol` dependencies are defined here. The live BLE client requires your Mac and local ring hardware at runtime; software-only tests use `MockRingOps` and Tokio virtual time.

## Structure

- `crates/pulse-key/src/engine.rs`: authorization state machine
- `crates/pulse-key/src/ops.rs`: real BLE implementation behind the `RingOps` trait
- `crates/pulse-key/src/tap.rs`: gesture extraction
- `crates/pulse-key/src/verify.rs`: possession/gesture/pulse checks; identity currently skipped
- `crates/pulse-key/src/server.rs`: loopback web API and vault demonstration

See `../docs/MAC_IMPORT.md` for implementation and privacy caveats.
