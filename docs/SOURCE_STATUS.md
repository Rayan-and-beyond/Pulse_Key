# Included, archived and missing sources

| Component | Status | Evidence |
|---|---|---|
| Corrected Phase 1 Python identity verifier | **Included** | Original v2 archive, extracted source, tests |
| Historic Phase 1 v1 Python verifier | **Archived** | Original 2026-10-30 source; waveform reconstruction error |
| Public PPG waveform benchmark | **Archived** | Research comparison predates decoder correction |
| 3 Nov evidence, errata and protocol | **Included** | Documents and aggregates |
| Original Mac Rust `pulse-key` crate | **Included after source review** | Uploaded `pulse-key.zip`; identifiers removed; motion recordings excluded |
| Parent Rust Cargo workspace and sibling `oura-link` crate | **Not supplied** | Required to compile the Rust crate |
| Mac optical telemetry audit tools | **Missing from available sources** | See historical provenance audit |
| Raw ring database / calibration captures / owner enrollment | **Purposely excluded** | Sensitive human biometric material |
| Actual per-device key, profile and session data | **Purposely excluded** | Confidential device/person-related records |

The Rust crate source is included, but it has not been compiled or hardware-tested in this import because the workspace and the sibling `oura-link` crate were not supplied. Six private accelerometer fixtures were withheld. The corrected offline Python code passed 14 standalone tests with 6 private-database tests skipped on 20 Nov 2026.
