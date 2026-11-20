# Rust build and source import

The user-supplied `pulse-key.zip` contained a single Rust crate, originally located under `~/Desktop/OuraDeepSeek/ring/crates/pulse-key/`. The original root workspace manifest and sibling `oura-link` source were not included in that ZIP.

## Dependency resolution

The reconstructed workspace now lives at `mac-daemon/Cargo.toml` and consumes the upstream [open_oura](https://github.com/Th0rgal/open_oura) `oura-link` and `oura-protocol` Rust crates as **pinned Git dependencies**. This preserves attribution and avoids copying upstream code into this repo. Their license is MIT. The pinned revision is `e57c43ce5fe0cf9a646afa790bda3731be767507`.

The upstream `OuraClient` API currently has no `stop_realtime` or `stop_live_hr` methods. The daemon now sends the upstream protocol's documented realtime-stop request and restores the live-HR feature mode using `OuraClient::transport()` and the public `Transport` trait.

## macOS build

Install Rust if necessary from [rustup.rs](https://rustup.rs), then:

```bash
cd mac-daemon
cargo test --workspace
cargo build --workspace
```

The default tests use mocks and virtual time; no ring, biometric database, or paired key is needed. The real BLE path requires your private locally stored ring authentication key and runtime-supplied BLE address and ring identifiers.

## Runtime safety

The original ring credentials, MAC address, serial number, and six recorded accelerometer fixtures remain excluded from the public repository. Leave the offline Python identity verifier separate from live authorization, and do not treat a green Rust build as biometric identity validation.

For local hardware use, obtain the actual ring identifiers from your device. Do not publish secrets or reuse test identifiers as if they were real ring identifiers.
