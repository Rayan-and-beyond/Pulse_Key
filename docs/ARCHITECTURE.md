# Architecture and trust boundaries

## State of implementation

The **reported local Mac Rust demo** used an Oura Ring 4, a local daemon, tap detector, fresh live-beat acquisition, and local vault state. This part was reported functional and had 126 workspace tests at the last documented milestone, and its crate source is now included at `mac-daemon/crates/pulse-key/`, pending restoration of the full Cargo workspace. The **offline Python verifier** in `research/identity/phase1_v2` is independently implemented and has not been wired to the daemon.

The design below distinguishes reported demo behavior from proposed security hardening.

## Current reported demo flow

```text
User wears Ring 4
      | BLE (ring-side authentication, separate from wearer identity)
      v
macOS Rust service <-- ring's motion / tap / heartbeat events
      | double-tap intent + fresh heartbeat + wear/custody state
      v
local demonstration vault state (unlock / relock)
```

**What this proves:** local device access, a deliberate interaction and live sensing during the request. **What it does not prove:** which person is wearing the ring or how resistant the full channel is to replay, relay and spoofing.

## Offline identity research engine

```text
Read-only historical Oura SQLite
      v
CVA PPG burst reconstruction (corrected v2)
      v
Signal quality checks and calibrated optical telemetry
      v
APG-DCT16 morphology + wearer enrollment profile
      v
ACCEPT / REJECT / INCONCLUSIVE (`authorized: false`)
```

The optical feature `N5 - N3` has shown retrospective separation between two wearers, with substantial confounders. Waveform-only discrimination remains weak under currently owner-fitted thresholds. Post-hoc results cannot substitute for frozen, independent evaluations.

## Proposed future security boundary

A production-capable signing/authorization gate would require a fresh host-generated challenge, trusted fresh sensor acquisition, local verification under explicit quality gates, deliberate consent/tap, and secure hardware-protected signing key usage with anti-replay state. The signature would bind to the exact challenge/request, timestamp/expiry, and device or session context. A timeout, ring removal, or inconclusive biometric measurement would withhold authorization. It should retain a separate recovery factor.

Device BLE authentication is **distinct** from biometric wearer identification; do not treat the ring's transport-layer challenge-response as a complete end-to-end freshness or biometric-proof primitive. Establish and test each trust boundary independently.

## Engineering gaps

1. Restore the sibling `oura-link` crate and Cargo workspace, then compile and verify the imported Rust daemon and tests.
2. Compare Python and Rust decoders against independent raw-byte fixtures.
3. Freeze the corrected offline model before prospective trials.
4. Validate owner acceptability across days before measuring blinded multiple-wearer rejection.
5. Formally test replay/relay, adversarial waveform injection, secret storage, downgrade and recovery behavior.

The project must remain a research prototype while these gaps persist.
