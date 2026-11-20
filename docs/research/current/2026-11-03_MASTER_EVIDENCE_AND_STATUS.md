# Pulse Key: current scientific and engineering state

**Authoritative status date: 3 November 2026**. Read `2026-11-03_SUPERSESSION_AND_ERRATA.md` before using historical reports. The scientific target is a one-to-one **owner-X ACCEPT / unfamiliar wearer REJECT** policy on the **same physical OLD Oura Ring 4**, across independent donnings and days. Reliable open-set acceptance/rejection is unproved.

## Evidence ledger

| Item | Finding | Confidence and applicability |
|---|---|---|
| Existing Rust demo | Double tap + fresh live-heartbeat gate + wear-removal relock; identity verifier is still a stub | Working custody/intent/liveness prototype as described in repo; **not wearer ID**; removal latency around 11 seconds reported, not instant |
| Old-ring CVA signals | One recovered ~50 Hz CVA PPG waveform (~1,500 samples per complete burst), optical ASCII/BLE diagnostics, accel, temperature, IBI | Captured historical data, hardware and firmware state must be audited per session |
| Historic same-ring optical sign | X all 9 bursts `N5−N3 ∈ [-3,0]`; Y all 6 `∈ [+1,+3]` | Confirmed from historical DB by Astra; decoded values also independently corroborated from included source; **post-hoc, 2 people, Y one day** |
| Binary calibration parsing | Refined subtype 0x33 parser matches step-count multisets in **15/15** bursts; v1 limited parser matched 13/15 | Isolated offline rerun confirms 15/15 multiset; chronology is not fully monotone in one burst; path physical geometry remains unknown |
| Original Phase 1 v1 | 17 tests passed, but reconstructed Oura PPG incorrectly | **Superseded** for waveform science. Keep original ZIP unchanged for audit |
| Corrected Phase 1 v2 | 20 tests pass; owner-only fitted waveform threshold `0.517940`; X clean score `0.170049`, Y `0.188865` and `0.196780`; both Y waveforms still pass | Independent offline replay of private historical DB (verified SHA-256); **no prospective results** and no actual FAR |
| Optical + waveform offline replay | Quality-passing later X ACCEPT, two Y REJECT by optical gate; X other later 2 INCONCLUSIVE, Y other 4 INCONCLUSIVE | Historical post-hoc results; small clean sample; `authorized:false` |
| Public owner-template benchmark | Prior v1 score pass 82/139 windows from other ring hardware | **Obsolete as a v2 comparison** due to wrong v1 owner template; even as v1 it was not same-ring FAR |
| Within-public PPG baseline | Subject-equal later-day APG AUC 0.570 and rise time 0.547, exploratory confidence bands straddle chance | Different device and dataset, not directly affected by this specific v1 Oura decoder bug; no strong cross-day discrimination established |
| Optical diagnostic physiology | LED control effort may encode tissue/contact response; final ADC near controller target | Plausible instrument-response hypothesis; pressure, fit, gain, orientation, temperature and session effects remain uncontrolled |
| Ring motion behavioral ID | Live accelerometer exists; behavioral authentication demonstrated on different rings in literature | Untested here, possible small parallel study |
| Multiwavelength/18 paths | Ring 4 sensor hardware supports multiple wavelengths and selectable paths | No confirmed access to concurrent individual raw PPG channels; hardware capabilities are not public BLE outputs |
| ECG/BioZ/vein imaging | No verified usable onboard electrodes/interfaces or such raw traces | Do not plan an Oura biometric algorithm around imagined streams |

## Key reported corrections from Astra beyond the reproduced decoder repair

- Optical calibration spans about **4.9 to 10.6 seconds** in observed timestamp differences; any movement before calibration finishes can confound measurement.
- The Ring 4 teardown identifies **MAX86178F** as the AFE. Prior references to MAX86171-register semantics require explicit mapping before use.
- A past RData probe was on a **different ring generation** and should not be used to declare this Ring 4 capability unavailable.
- The fact that optical slot identifiers are numerically consecutive does **not** establish physically adjacent paths or an angular derivative.
- Earlier slot index / borderline zero association and published-looking collision/feasibility percentages were unsupported.
- 4-second overlapping PPG windows and non-session-disjoint train/test methods in a promising published paper can inflate evaluation; review the original paper before importing its error rates.

## Scientific interpretation

**Promising lead:** combine a correctly reconstructed pulse representation with `(N3,N5)` and explicit instrumentation/quality covariates; compare against single-modality baselines. Predefine all feature families and avoid fitting on the two-person historical audit. Source and physical calibration interpretation must be independently checked for any new algorithm.

**Current limiting evidence:** No prospective cross-day owner-repeatability results and no blinded, multiple-stranger, same-physical-ring crossover. No defensible FAR, FRR, EER or rejection-of-*anybody-else* guarantee. Software-passing test counts are decoder/regression evidence, not biometric-validation evidence.

**Architecture separation:** Existing frozen Mac optical-only prospective verifier `[-5,0]` is a historical baseline and must retain its code/profile hashes. Newly patched Library Phase 1 v2 is a **separate offline shadow research engine** and must be independently frozen before new evaluation. No live Mac vault changes or private credential uploads.

## Available evidence and provenance

- Full user-provided Astra assessment: `2026-11-03_ASTRA_INDEPENDENT_REPORT_SOURCE.md`.
- Local reproduction aggregates: `2026-11-03_OFFLINE_REPRODUCTION_AGGREGATES.json`.
- Corrected research code: `Pulse_Key_Phase1_Corrected_Offline_v2_2026-11-03.zip`.
- Correction rationale and historical-claim map: `2026-11-03_MEASUREMENT_CORRECTION.md` and `2026-11-03_SUPERSESSION_AND_ERRATA.md`.
- Next acquisition and decision protocol: `2026-11-03_NEXT_EXPERIMENT_PROTOCOL.md`.
- Preserve original 2026-10-30 to 2026-11-02 records, including `multimodal-audit-private.json`, as historical evidence.

**Privacy:** No raw `oura.db`, keys, BLE pairing secrets or owner's biometric enrollment templates stored in the corrected public-style archive; the original data must remain local and private.
