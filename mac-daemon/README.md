# Rust Pulse Key crate imported

The original `pulse-key.zip` Rust source has been imported into `mac-daemon/crates/pulse-key/`.

Included: Cargo manifest, Rust daemon and state machine, tap detector, verification logic, BLE integration hooks, local web UI, example tool, and deterministic Rust unit tests.

**Build dependency still missing:** This source archive is one crate from a larger Cargo workspace. Its manifest inherits `version`, `edition`, `license`, and `repository` from `[workspace.package]`, and requires a sibling `../oura-link` crate. The root workspace manifest and `oura-link` implementation were not supplied. Import them from your Mac before expecting `cargo test -p pulse-key` to run.

**Public release safety edits:**
- The original device MAC address and serial were removed from `src/main.rs`. Supply `--expected-mac`, `--expected-serial`, `--address`, and `--key-file` locally when running the crate.
- The key file contents were absent from the uploaded archive and remain excluded.
- Six historical accelerometer `.jsonl` fixture recordings were excluded. The six fixture-dependent tests are behind the `private-fixtures` feature, with instructions in `tests/fixtures/README.md`.
- macOS metadata was excluded.

The Rust service is a research demonstration of device possession, tap intent, and fresh heartbeat sensing. It does **not** incorporate the offline Python wearer-identity research verifier.

See `../../docs/MAC_IMPORT.md` for the rest of the workspace import procedure.
