# Pulse Key | Frozen Phase-1 waveform vs. public ring PPG cohort

**Experiment:** 2026-10-31 (Asia/Riyadh)  
**Status:** Experiment executed; exploratory, cross-device challenge.  
**Core decision:** **Current APG-DCT16 waveform score cannot independently authenticate the Oura owner.** Preserve the Oura optical-calibration check as a separate research signal. A controlled physical-ring impostor test remains necessary.

## What was actually run

- **Source:** SNOWBALL Lab, *Multisite-PPG*, `snowballlab/Multisite-PPG`, ring-only raw recordings. Attribution: https://huggingface.co/datasets/snowballlab/Multisite-PPG . Twenty anonymized participants, 100 Hz green/IR PPG, other sensor streams. Only green PPG was used for this waveform-only experiment.
- **Acquisition:** All 20 ring-only raw `.npz` files (1,326,368,955 bytes) were present on the connected research VM from the preceding experiment. An existing, identity-independent screening script had selected **163 distinct 30-second windows** from **18** coded participants; P14 and P16 yielded no selected windows in that earlier screen. Selected windows were spaced at least 30 minutes apart within each day. The precise selected `[participant_code, timestamp_ms]` list is supplied in `selection_manifest.json`. Existing selection quality criteria were exploratory and could exclude valid data.
- **Signal conversion:** Each selected 100 Hz green PPG window was resampled to 50 Hz using `scipy.signal.resample_poly(data, 1, 2)`.
- **Extractor:** The **exact** frozen Phase 1 `feature_extraction.py` used for the owner's Oura CVA PPG, confirmed identical on the VM and in the package by SHA-256 `9bb4609ba8b5a2a2032f5da76367b54e529045c983688ccbd6c5fd8c6fd5d8ee`. It reconstructs normalized aligned beats and obtains 16 normalized coefficients from the second derivative of the median waveform, after a 0.5–12 Hz bandpass at interpolated 250 Hz. It selects polarity and applies its built-in minimum beat-count and coherence gates. Interpolation is not an increase in sensor resolution.
- **Frozen X enrollment:** Oura recording starts `715158`, `2659172`, `2661558`. Nearest-reference APG-DCT16 Euclidean acceptance threshold **0.793096**, derived from owner-only training (but its feature design and slack settings were chosen during earlier explorations, so this is not an untouched evaluation). The private derived owner template was held on the research VM temporarily, chmod `0600`, and excluded from this deliverable.
- **No optical spoofing by absence:** SNOWBALL's raw ring files do **not** provide the proprietary Oura `N5−N3` diagnostic/LED step-count data. The full Phase 1 AND-verifier was **not** run on these files as if missing Oura calibration were a biometric failure. Public recordings were tested only against the **waveform score**.
- **Hardware:** These public ring records originate from a **different ring design** from the user's Oura Ring 4. Whether they used exactly the same physical device across people was not verified. Equal 50 Hz sampling and identical code do not eliminate hardware, optical geometry, firmware, filter, posture, and contact differences.

## Primary results: owner vs. anonymous public recordings

| Measure | Measured result |
|---|---:|
| Selection-screened 30 s public windows | **163** |
| Passed the exact Phase-1 feature-quality gates | **139** |
| Failed feature-quality gates | **24** |
| Public participants with usable windows | **18** |
| Passed owner's frozen waveform threshold (distance ≤ 0.793096) | **82 / 139** |
| Did not pass that threshold | **57 / 139** |
| Participants with at least one passing waveform window | **17 / 18** |
| Minimum public window distance to X enrollment | **0.238851** |
| Median public window distance to X enrollment | **0.700626** |

**82/139 = 59.0% waveform-score pass rate in this cross-device, non-independent candidate-window sample. It is NOT a true Oura false-acceptance rate, NOT a prediction of how many people could unlock the ring, and NOT representative of all 20 participants or future recordings.** Optical calibration remained completely untested for public participants.

### Tightening the threshold was insufficient on these samples

| Waveform distance cutoff | Public windows scoring within cutoff | Participants with any such window |
|---:|---:|---:|
| 0.300000 | 1 / 139 | 1 |
| 0.386680 | **8 / 139** | **6** |
| 0.500000 | 30 / 139 | 14 |
| 0.600000 | 49 / 139 | 16 |
| 0.700000 | 69 / 139 | 16 |
| 0.793096 (frozen Phase-1 cutoff) | **82 / 139** | **17** |

The clean, later owner (X) ring recording `2699937` scored **0.386680**. One already observed Y ring recording `2703303` scored **0.386518**, numerically *closer* to the X enrollment than X's own later recording, while the second Y `2708139` scored **0.521412**. Both Y waveform scores passed the Phase-1 0.793096 cutoff; Y's positive optical differences of `+3` and `+2` caused the full retrospective verifier to reject them. Both measurements come from the same physical Oura Ring; this is stronger direct evidence of waveform-score weakness than any cross-device public comparison.

**Consequently, the currently implemented waveform distance cannot accept clean held-out X while rejecting both of those known clean Y examples using one threshold.** A threshold below X's 0.386680 would reject that legitimate X test, and a threshold at or above it would accept the Y score of 0.386518.

## Within-public-cohort comparison using the same frozen extractor

For each public participant with at least two usable windows, we treated the earliest as a reference and compared later same-person windows with references from other coded participants. Comparisons were separated by calendar day. These are threshold-free descriptive AUCs, not independently held-out FAR/FRR. The following *subject-equal* AUC averages each reference person's AUC once, preventing subjects with many windows from dominating. The 95% intervals resample reference subjects with replacement (`numpy` RNG seed 20261031; 5,000 draws) and remain exploratory.

| Feature and comparison | Subject-equal AUC | 95% subject bootstrap | Reference participants | Same-person probes |
|---|---:|---|---:|---:|
| APG-DCT16, same day | 0.600 | 0.502–0.701 | 10 | 25 |
| APG-DCT16, later day | **0.570** | **0.497–0.657** | 15 | 96 |
| Mid-upstroke rise time, same day | 0.658 | 0.541–0.763 | 10 | 25 |
| Mid-upstroke rise time, later day | **0.547** | **0.455–0.643** | 15 | 96 |

The corresponding pooled, pair-weighted AUCs were APG-DCT16 same-day 0.5957, later-day 0.4676, rise time same-day 0.6669, later-day 0.4436. The differences from the subject-equal estimates show the importance of repeated-window and participant weighting. AUC 0.5 means chance ordering; later-day estimates are weak and their subject-level intervals straddle 0.5.

## What this changes

1. **Do not promote the current waveform-only matcher to an authorization factor.** Independent public pulses often look close to X under this metric, and clean Y already ties or beats clean X on the actual Oura hardware. The current waveform model can be retained as a research comparator or signal-quality check without granting access based on its score alone.
2. **Keep Oura `N5−N3` differential calibration separate.** That measure distinguished X and Y across the already analyzed same-ring captures, and it decided the two clean Y rejections. Its stability across future reseats, temperatures, unknown people, and days is still unknown. Missing values must mean **INCONCLUSIVE**, never impostor rejection.
3. **Optical intensity in the public data is a research lead, not a drop-in substitute.** The preceding exploratory benchmark observed 0.803 same-day AUC for raw IR intensity versus 0.704 later-day, with hardware/gain confounds. The public data lack Oura LED register calibration and cannot directly test `N5−N3`.
4. **More waveform feature tuning on these same 139 windows risks overfitting.** Any new representation needs predeclared data splits and later held-out recordings, especially cross-day. Better pulse fiducial detection, genuine signal quality, waveform reconstruction and temperature controls are reasonable research avenues; changes must beat the frozen APG-DCT16 baseline without using the final evaluation set in tuning.
5. **A fresh Oura test with X and any consenting new person remains decisive.** It need not be Y specifically. Use the same physical OLD Oura Ring 4, separate reseats, and frozen enrollment and thresholds. Report ACCEPT, REJECT and INCONCLUSIVE independently. Leave the working custody/liveness/tap demo untouched.

## Reproduce

The companion `Pulse_Key_Public_Waveform_Benchmark_2026-10-31.zip` contains the benchmark source, the 163-window selection manifest, and the frozen Phase-1 extractor. Obtain the **raw ring-only** `P1_Ring_raw.npz` ... `P20_Ring_raw.npz` files at the source URL and place them in a local folder. The package intentionally excludes the user's owner biometric template and all raw recordings. Dependencies: NumPy and SciPy on Python 3.11+.

- Use the Phase-1 CLI to enroll the three X CVA bursts and export a private owner profile. Extract its `waveform.templates`, `waveform.nn_distance_threshold`, and `waveform.rise_ms_owner` into a local-only JSON file with keys `templates`, `threshold`, `rise_ms_owner` for the benchmark. Do not place this file in a public repository.
- Run `phase1_aligned_public_benchmark.py` with `--raw <folder> --selection selection_manifest.json --output <private-output.json> --owner-template <private-owner-template.json>`.
- To avoid exporting individual public participant score records, publish only the aggregate results. The raw output contains sampled participant codes and derived waveforms; treat it as restricted research material.

Dataset terms: CC BY-NC 4.0; the card permits noncommercial research and derived preprocessing with attribution, while explicitly requiring separate permission for re-identification attempts. This test keeps participant codes anonymous, does not search for identities, and provides only aggregate research results. Research and deployment should comply with the dataset's terms.

## Integrity and implementation scope

**Replicated actual work:** 17/17 existing Phase-1 tests passed again on the mounted private Oura database. The phase-1 waveform extractor's source SHA-256 was verified identical on the VM and in the existing ZIP. The public analysis processed 139 qualifying 30-second windows and printed results; the subject bootstrap was then computed on those fixed features. This report is not a deployment certificate or a claim of independently validated biometric security.
