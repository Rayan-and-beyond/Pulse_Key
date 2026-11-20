# Pulse Key — Exploratory SNOWBALL Multisite-PPG Experiment

**Date:** 2026-10-30  
**Research question:** Do the pulse morphology and optical intensity features proposed for Pulse Key show person-associated stability in a public ring PPG dataset?

## Source and scope

- Public source: https://huggingface.co/datasets/snowballlab/Multisite-PPG
- Downloaded individually the `raw_data/P1/` through `raw_data/P20/` `P<N>_Ring_raw.npz` files (~1,326,368,955 bytes combined) on an authorized Linux research VM.
- Each raw ring file includes timestamped green PPG, infrared PPG, 3-axis acceleration, and temperature sampled at 100 Hz.
- **18 of 20 participant files yielded selected windows under this specific exploratory processing pipeline**; P14 and P16 yielded no selected usable windows. This is an algorithm-specific exclusion, not a claim that their recordings contain no valid physiological signal.
- **163 nonoverlapping 30-second candidate recordings** remained after spectral quality checks, with selected recordings spaced at least 30 minutes apart within a calendar day. Multiple recordings per participant are correlated and are **not** independent participants.
- Samples were resampled from 100 to 50 Hz for rough Oura sampling-rate comparability. Green and IR used the same extraction logic; IR produced usable feature extraction for **38 of the 163 selected green-PPG windows**.

## Experimental implementation

A fresh **approximate** extraction pipeline was implemented (not the original Oura `multimodal_audit.py` or its exact `apg_dct_1_16` implementation):

1. Validate each 30-second segment's timing continuity; use a spectral concentration filter and a periodicity check.
2. Downsample to 50 Hz. Use a third-order 0.5–12 Hz Butterworth bandpass; detect systolic peaks, estimate previous valleys and per-beat 10–50% rise time.
3. Phase-normalize accepted beats to 128 samples, calculate a median template, use the second derivative of a smoothed template, and retain normalized DCT coefficients 1–16 as a **proxy APG-DCT** embedding.
4. Extract median raw green and IR intensities from each 30-second segment; compare log-intensities and the log green/IR ratio as **optical proxies** (not Oura LED drive/current register values).
5. Use each participant's first selected recording as enrollment reference. Compare different recordings from the same participant with reference recordings from other participants. Separate same-calendar-day from later-day genuine probes; use pooled threshold-free AUC (`1.0`=perfect, `0.5`=chance) for exploratory comparisons.

**Sample sizes for AUC calculations:** 36 same-day genuine comparisons, 109 later-day genuine comparisons, and 306 cross-participant reference comparisons in each modality with complete readings (except IR/fused subsets). These pairwise comparisons share enrollment references and are **not independent Bernoulli trials**, so naive confidence intervals and population-level FAR claims are inappropriate.

## Results: actual analyses performed

| Feature / distance | Same-day AUC | Later-day AUC | Interpretation |
|---|---:|---:|---|
| Mid-upstroke 10–50% rise time (green) | 0.412 | 0.403 | Poor discriminator with this processing pipeline |
| 16D normalized APG-DCT-style green embedding | 0.491 | 0.463 | Approximately chance, with this independently implemented extractor |
| Log median green DC intensity | 0.714 | 0.719 | Some participant-associated variation, potentially device-confounded |
| Log median IR DC intensity | **0.803** | 0.704 | Strongest same-day exploratory discrimination, but potentially device-confounded |
| Log median green/IR DC ratio | 0.698 | 0.651 | Moderate discrimination; **inferior to the individual channels in this evaluation** |
| Heuristically combined APG morphology + optical ratio | 0.660 | 0.634 | The added morphology did **not** improve the optical ratio alone here |

IR *waveform* morphology was available for only a small subset (38/163 windows, seven participants with sufficient repeat captures): single-channel IR morphology AUC 0.305 same-day and 0.195 later-day; green+IR morphology fusion AUC 0.333 same-day and 0.420 later-day in that limited sample. These small-sample results are highly unstable and do not prove that multispectral physiology cannot help.

### Additional observation

The public green-waveform-derived mid-upstroke measurements had large intra-participant variation, frequently changing by tens of milliseconds between retained sessions. One participant had a measured 31.9 ms window, near the range reported in the Oura owner's previous audit; this is **not** evidence that the participant could impersonate the owner because the sensor, firmware, segmentation, and filtering are different.

## Scientific interpretation

1. **The public dataset supports studying waveform variability, and the naive feature extractors did not generalize reliably across independent free-living recordings.** This does not refute TrueHeart, Oura-specific CVA features, or a better-quality extractor.
2. **Relative optical signal intensity is worth researching**, but absolute green/IR levels are especially vulnerable to device-specific gains, LED currents, geometry, contact, skin temperature, and arbitrary sensor scales. The dataset does not establish whether all participants used the same physical ring or the same calibration. The observed AUCs could reflect device identity rather than wearer identity.
3. The proposed differential `N5−N3` feature from Oura firmware **cannot be tested directly**: this dataset does not include Oura `0x43` ASCII calibration logs or `0x61 subtype 0x33` LED-control diagnostic packets. The green/IR DC ratio is **not a substitute for N5−N3**.
4. The optical ratio did not outperform the individual-channel intensities and fusion with the current APG representation weakened the observed discrimination. So **normalizing or fusing optical channels does not automatically remove confounding or improve authentication**.
5. The original Oura audit reported X/Y separation for the owner with its APG-DCT and `N5−N3` features. This public-data test neither validates nor contradicts that **same-physical-ring** finding directly.

## Reproducibility and limits

- Experimental scripts and aggregate JSON output were created on the authorized research VM in `/tmp/pulsekey_public_ppg/`: `public_biosignal_probe.py`, `evaluate_features.py`, `add_ir.py`, `evaluate_2channel.py`, `add_optical_ratio.py`, `evaluate_optical.py`, `evaluate_three_optical.py`, `public_features_optical.json`.
- Raw public data remain on the VM's temporary filesystem; no public individual physiological streams are included in this report. The scripts are standalone exploratory variants rather than published-paper reproductions.
- The VM had ~1 GiB RAM. A first version that loaded all channels simultaneously was stopped due to heavy swapping; final extraction loaded green and IR sequentially.
- Data were free-living; chosen windows were determined by an exploratory signal-quality method, and the polarity/peak heuristic was not independently validated against ECG-derived systolic events. This strongly limits interpretation of negative waveform results.
- The AUC comparisons use participant labels only in their anonymized form. They were not used to infer real-world identities. See dataset documentation for applicable usage terms, particularly its separate-permission rule for re-identification attempts.

## Decision-relevant outcome

**Actual finding:** On this public dataset and extraction pipeline, raw optical intensities yielded more apparent subject discrimination than the PPG waveform representations, but device-level calibration confounding is unresolved. **Next scientific question:** whether the Oura `N5−N3` sign remains wearer-specific after controlling ring placement and sensor calibration using the same physical Oura Ring 4.

No validated FAR, FRR, EER, or deployment authentication guarantee follows from these experiments.
