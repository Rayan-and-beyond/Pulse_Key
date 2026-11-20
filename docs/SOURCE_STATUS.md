# Included, archived and missing sources

| Component | Status | Evidence |
|---|---|---|
| Corrected Phase 1 Python identity verifier | **Included** | Original v2 archive, extracted source, tests |
| Historic Phase 1 v1 Python verifier | **Archived** | Original 2026-10-30 source; waveform reconstruction error |
| Public PPG waveform benchmark | **Archived** | Research comparison predates decoder correction |
| 3 Nov evidence, errata and protocol | **Included** | Documents and aggregates |
| Original Mac Rust `pulse-key` crate | **Missing from available sources** | Historical project README reported working 126 tests |
| Mac optical telemetry audit tools | **Missing from available sources** | See historical provenance audit |
| Raw ring database / calibration captures / owner enrollment | **Purposely excluded** | Sensitive human biometric material |
| Actual per-device key, profile and session data | **Purposely excluded** | Confidential device/person-related records |

Do not imply the Rust daemon was recompiled or its real-hardware behavior reverified in this repository preparation. The corrected offline Python code passed 14 standalone tests with 6 private-database tests skipped on 20 Nov 2026.
