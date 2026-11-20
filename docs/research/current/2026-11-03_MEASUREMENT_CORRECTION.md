# Pulse Key: measurement corrections and independently reproduced offline results

**Dated:** 3 November 2026. **Status:** new, experimental offline source v2. The v1 code and v1 results are retained for provenance but are superseded for waveform-based analysis.

## A. Decoder defect and correction

The archived `Pulse_Key_Phase1_Identity_Engine_2026-10-30.zip` `feature_extraction.reconstruct()` added each entry of a non-marker packet's `ppg_samples` to a running amplitude. The Rust-event-decoded JSON field already contains **cumulative offsets relative to the beginning of each packet**, creating a double-accumulation error. The corrected version adds each stored offset to the fixed packet-start amplitude, then carries forward only the packet's final absolute value. For stored offsets `[-1, 2]` after sample 102, the correct new samples are `[101,104]`; the previous code instead yielded `[101,103]`.

Astra's report states that it verified the correction via independent raw-byte and decoded-JSON paths on all **nine complete, single-marker bursts**. We independently patched the Library v1 Python source in an isolated environment and reproduced its corrected numerical results against the provided private historical `oura.db`. We did not alter the original Mac's live daemon or local frozen prospective code.

## B. Offline reproduced regression (recomputed from historical DB)

Enrollment: X start IDs `715158`, `2659172`, `2661558`. Optical acceptance interval remains `[-5,0]`, which was chosen after exploration of original X/Y. The v2 profile schema prevents interchange with old v1 profile templates. The same owner-only threshold-selection method now yields **0.517940** rather than the v1 (incorrect-waveform) **0.793096**.

| Burst | Identity | v1 distance (wrong reconstruction) | v2 distance (corrected reconstruction) | v2 waveform verdict | v2 overall decision |
|---|---|---:|---:|---|---|
| 2699937 | X | 0.386680 | **0.170049** | Pass | ACCEPT |
| 2703303 | Y | 0.386518 | **0.188865** | Pass | REJECT (optical) |
| 2708139 | Y | 0.521412 | **0.196780** | Pass | REJECT (optical) |

Chronologically held-out *after last enrollment* X = 1 ACCEPT, 0 REJECT, 2 INCONCLUSIVE; Y = 0 ACCEPT, 2 REJECT, 4 INCONCLUSIVE. Three earlier pre-enrollment non-training X recordings are **not** included in that chronological held-out denominator. The complete 15-burst listing contains 9 X, 6 Y, of which 3 X were used to enroll.

**Consequences:** The prior categorical claim that no scalar waveform distance cutoff can separate the clean observed X/Y scores is **false of the corrected three distances**. A post-hoc cutoff between `0.170049` and `0.188865` separates those *three historical observations*, yet is far tighter than owner-trained `0.517940` and has no independent validity. Both Ys still pass the corrected owner-trained waveform threshold. No improvement in demonstrated genuine/impostor operating performance.

## C. Optical register parser repair

Older verification matched only `0x50/0x51` and `0x61/0x62` and missed some second tuples. The new offline `0x61` subtype `0x33` parser recognizes 8/14-byte debug records with a 2-byte prefix and one/two 6-byte slot/DAC/config tuples. It checks measured step sequences for channel/bank 5 and bank 6, without asserting physical emitter/detector positions. **All 15 historical bursts** match the expected calibration-value multisets after corrected extraction (20 tests passed; 0 failed, with optional actual DB tests enabled). One burst's observed register-write order is reportedly nonmonotonic; multiset equality does not establish chronological physical writes. Firmware slot semantics and physical optical geometry are unproved.

## D. Prior claims reclassified

- The old **82/139 waveform-score passes in cross-device public recordings** are a true description of a historical v1 processing experiment, and **cannot evaluate the corrected v2 owner-profile waveform score**. A revised owner-vs-public comparison requires rebuilding the owner template and rerunning with properly permissioned raw data.
- The *within-public* subject-equal later-day APG-DCT16 AUC **0.570**, rise-time AUC **0.547**, was computed from the public arrays directly. This particular owner-decoder error does not invalidate it. It remains exploratory and exhibits poor generalization in those public comparisons.
- A 15/15 calibration multiset match does not prove a physiological fingerprint, path adjacency, known wavelength/path geometry, or a genuine security verifier.
- The MAX86178F teardown designation belongs to this hardware investigation; do not import MAX86171-specific register interpretations as proven Ring 4 facts. The previously referenced RData failure on Ring 5 firmware 2.1.3 cannot settle Ring 4 firmware 2.12.5 access.
- Oura's 18 selectable optical paths are hardware information; simultaneous raw 18-channel data access was **not demonstrated**.
- Historical `N5−N3` X `[-3,0]` vs Y `[+1,+3]` across 9/6 bursts survives. Threshold selection is post-hoc, and cross-day stranger discrimination is unknown.
- Motion/tap behavioral identity is an untested lead. Knowledge-based secret taps can authenticate an input secret but do not prove physiological identity.

## E. Protection against silent changes

The patched v2 source is distributed as `Pulse_Key_Phase1_Corrected_Offline_v2_2026-11-03.zip`. It is **research-only** and returns `authorized:false`. Older v1 ZIP, previous reports, and original private audit are left unchanged as historical evidence. Reproduced SHA-256 hashes and aggregate metrics appear in `2026-11-03_OFFLINE_REPRODUCTION_AGGREGATES.json`. No raw samples, profile templates, BLE pairing keys or user database are included in the patched archive.
