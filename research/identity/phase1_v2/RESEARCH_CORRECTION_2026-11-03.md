# Experimental Phase 1 v2: corrected waveform reconstruction (2026-11-03)

**Offline research code only. Never authorize a real Mac unlock using these outcomes.**

## Changes

- `feature_extraction.reconstruct`: corrected double accumulation. The Rust-derived JSON `ppg_samples` for non-marker packets contain **cumulative values relative to the beginning of the packet**, so each is added to that packet's starting amplitude. The old v1 loop accumulated these offsets twice.
- `observations.register_tuples`: parse 8/14-byte `0x33 0x03` payloads into all 6-byte register tuples with observed bank 5/6 identifiers and configuration bytes; no physical optical geometry assumed. The older v1 parser missed some non-default slot IDs and second tuples.
- Offline profile schema bumped to `pulse-key-offline-identity-v2-corrected-ppg`; v1 profile templates/thresholds are incompatible. Regenerate from the **original three owner enrollment bursts** before evaluating in offline mode.
- Unit tests now check packet-relative decoded input semantics, both register tuples, and wrong marker ordering. Optional regression test checks all 15 archived bursts' binary calibration multiset correspondence.

## Scientific boundaries

- This patch changes measurement processing, not the ring firmware, live daemon, or old v1 source/records.
- The optical decision interval `[-5,0]` and slack settings remain **historical, post-hoc experimental settings**. Recomputed waveform threshold under the same enrollment algorithm is expected around `0.517940`; do not override it with the older `0.793096` value.
- Register trace multiset agreement is not proof of chronological register-write order or physical adjacent optical paths. A timestamp-order reversal has been reported for one historical burst.
- Earlier cross-device owner-vs-public waveform score pass rates must be marked **obsolete for a v2 owner profile** and rerun only after obtaining appropriate raw data and permissions.
- This offline experiment carries `authorized:false`, preserves private raw biometrics locally, and reports `INCONCLUSIVE` on quality/protocol failures.
