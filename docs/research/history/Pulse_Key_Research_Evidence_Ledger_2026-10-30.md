# Pulse Key — Research Evidence Ledger
**Snapshot:** 30 October 2026  
**Scope:** Scientific findings and open questions for wearer verification with a retired Oura Ring 4, incorporating the agent's `multimodal-audit-private.json` and our independent review.  
**Privacy:** Contains derived biometric features and anonymous X/Y labels only. Do not publish the private audit or underlying `oura.db` without the participants' explicit consent.  
**Status:** Research notes; **no deployed wearer-identity authentication has been validated**.

## Executive scientific position

**Original goal:** Enroll Person X from data available through the *same* physical Oura Ring 4, subsequently ACCEPT X and REJECT Person Y or an unfamiliar wearer of that ring. Cross-day persistence is an intended research objective. Same-day matching is an intermediate scientific case. We are studying wearable physiological identity rather than recovering public-dataset participants' names.

**Progress:** The first phase-normalized mean-beat correlation matcher failed: Y's recordings matched X's waveform template at r≈0.994–0.995. Subsequent waveform timing, APG-DCT curvature, and differential optical calibration analyses show multiple **exploratory X/Y separations** in the existing recordings. These changes are real progress in identifying candidate measurements, but published FAR/FRR cannot yet be claimed for this ring. A second person recorded only on one day and all feature candidates have been selected/inspected with knowledge of X/Y data.

**Most interesting candidate signals:**

- `apg_dct_1_16`: a 16-dimensional second-derivative / morphology representation; only representation retaining an owner-versus-Y distance gap when all five available X waveform records in the audit's LOO comparison are included.
- `t10_50_ms`: 10%→50% mid-systolic rise time, preserving group ordering across the four reported segmentation/filter variants; computational interpolation resolution is not sensor resolution.
- `N5 − N3`: difference between optical calibration-step counts associated with path/channel identifiers 5 and 3; all **9 reported X burst differences are nonpositive**, all **6 Y burst differences positive**, including reported Y→X→Y handovers. Its physiological specificity and robustness to position/contact remain unproven.

**Crucial distinction:** The agent's report measures features derived from ring logs and waveform processing. This document checks their internal numerical consistency where possible; it does not independently reproduce the raw CVA decoder or re-run those algorithms on the original waveform bytes.

## 1. Known Ring 4 acquisition and initial failure

- Retired Ring 4, direct BLE via `open_oura` and custom client: on-demand `0x0D` CVA recordings of approximately 30 s, ~1,500 PPG samples, around 50 Hz; one reconstructed optical waveform (`0x81` event family). Accelerometer ~50 Hz is also accessible; tap recognition and live IBI/HR have been exercised.
- Working early end-to-end demo: software-detected double tap → live heartbeat validation → local Mac gate unlock → ring-off relock. This establishes possession/liveness/intent, **not the identity of the wearer**.
- Original phase-normalized Pearson beat-template matcher failed X/Y: Y was *more* similar to X's template than X's held-out recordings. Phase stretching may discard absolute systolic timing information.
- The project contains a CVA mixed absolute/delta-marker boundary reconstruction concern; raw sample validation remains a prerequisite for treating fine waveform features as physical measurements.
- Current relevant experimental signals: pulse morphology and APG, temperature (`0x46`), calibration ASCII logs (`0x43`), binary diagnostic subtype (`0x61`/`0x33`), motion metadata (`0x47`), mid-burst scan indicator (`0x82`), separate pre-CVA tap accelerometer traces.

## 2. Optical calibration finding

The agent claims the firmware emits ASCII end-of-calibration logs of the form `E:3N...A...`, `E:5N...A...`, with step counts `N3`, `N5`. It correlated diagnostic-register traces (subtype `0x33` within event `0x61`) with ASCII step counts, reporting register final values `1 + 4*N` and trace counts generally `N + 1`.

**Verification nuance:** The attached JSON marks **13 of 15** bursts `protocol_verified_exact=true`; two are false, one due to absent register traces and one due to mismatched trace count. This supports strong *empirical correspondence*, not proof of the full firmware state machine or optical interpretation. The records do not directly establish wavelength, emitter/detector geometry, optical path length, tissue scattering coefficients, or cancellation of contact pressure.

All 15 reported burst-level values:

| Start index | Wearer | N3 | N5 | N5−N3 | Both ADC 509800? | Exact binary corroboration? | Mid-scan | Motion seconds |
|---:|:---:|---:|---:|---:|:---:|:---:|---:|---:|
| 715158 | X | 16 | 14 | -2 | no | yes | 0 | 0 |
| 955573 | X | 25 | 23 | -2 | yes | yes | 1 | 0 |
| 957675 | X | 22 | 22 | +0 | no | yes | 1 | 10 |
| 962505 | X | 22 | 19 | -3 | yes | yes | 0 | 3 |
| 2659172 | X | 12 | 10 | -2 | yes | yes | 0 | 0 |
| 2661558 | X | 14 | 11 | -3 | no | yes | 0 | 0 |
| 2664026 | X | 11 | 10 | -1 | no | yes | 0 | 3 |
| 2675324 | Y | 17 | 18 | +1 | yes | yes | 0 | 0 |
| 2676726 | Y | 13 | 14 | +1 | no | yes | 0 | 0 |
| 2682959 | Y | 23 | 24 | +1 | yes | yes | 0 | 4 |
| 2688161 | Y | 20 | 21 | +1 | no | no | 0 | 0 |
| 2696276 | X | 15 | 15 | +0 | yes | no | 1 | 8 |
| 2699937 | X | 16 | 16 | +0 | yes | yes | 0 | 0 |
| 2703303 | Y | 20 | 23 | +3 | yes | yes | 0 | 0 |
| 2708139 | Y | 20 | 22 | +2 | yes | yes | 0 | 0 |

**Derived from the JSON:** X `N5−N3` ranges from −3 to 0 (9/9 observations). Y ranges from +1 to +3 (6/6). The sign separation remains in **5 X and 4 Y recordings** for which *both* final reported ADC values equal 509800; further restricting to exact binary corroboration leaves **4 X and 4 Y** observations.

**Physical hypothesis (unproven):** Different optical calibration steps could reflect finger anatomy, pigmentation, perfusion, placement and contact. Subtracting steps might reduce a common additive calibration effect. The step difference is **not** demonstrably an optical attenuation ratio or an invariant identifying the wearer. Data from one other person on one day cannot establish impostor rejection across the population.

## 3. Time-domain and derivative waveform findings

From the agent's four processing variants (five X waveforms, two Y waveforms), the median `t10_50_ms` upstroke values place X below Y in each variant. The following row computes `min(Y median) − max(X median)` on **reported burst medians**, not on future independent samples:

| Processing variant | Max X median (ms) | Min Y median (ms) | Median separation (ms) |
|---|---:|---:|---:|
| Literal 50 Hz, 0.5–6 Hz | 46.507 | 49.133 | +2.627 |
| Interpolated 500 Hz, 0.5–6 Hz | 46.182 | 48.331 | +2.148 |
| Modified Oura, 500 Hz, 0.5–6 Hz | 46.594 | 48.331 | +1.736 |
| Modified Oura, 500 Hz, 0.5–12 Hz | 32.327 | 36.717 | +4.390 |

**Important:** Interpolation from a 50 Hz sensor does not recreate missing high-frequency samples. Millisecond bootstrap precision based on tens of beats from the *same burst* does not measure variation between separate recording days, ring orientations or people. X's cold-hand burst notably destabilized TrueHeart peak/crest-time (`Tc`) and second-derivative magnitude ratio (`|b|/a`); initial reports that both perfectly separate X/Y across days were revised after bootstrap assessment.

In the agent's selected `0.5–12 Hz` modified pipeline, upper X *within-burst bootstrap* bound for `t10_50_ms` is 33.2 ms and lower Y bound is 35.8 ms. This remains exploratory and conditional on processing method and sample selection.

High correlations among the 4 scalar upstroke metrics and harmonic magnitudes (e.g. X-only r≈−0.998 between mid-upstroke time and slope/amplitude) suggest redundancy; these are not independent authentication factors. Five X bursts are too few to establish statistical independence of optical asymmetry from waveform morphology (X-only r≈−0.04 with `t10_50_ms`).

## 4. Owner-only leave-one-out discrimination in audit

**Larger distance indicates poorer match.** For each comparison the report lists the range of genuine X leave-one-burst-out distances and impostor Y distances. A positive separation margin means `min(Y distance)>max(X distance)` in these *specific* observations, not that the classifier will generalize. Table below faithfully transcribes audit aggregates:

| Representation | Four scan-clean X: X range | Y range | Margin | All five X: X range | Y range | Margin |
|---|---|---|---:|---|---|---:|
| APG-DCT 16D | 1.307–1.602 | 2.083–3.001 | +0.481 | 0.943–1.990 | 2.165–3.141 | +0.176 |
| TrueHeart/upstroke 4D | 0.530–2.189 | 3.687–6.151 | +1.498 | 0.307–4.323 | 1.767–5.053 | -2.556 |
| Waveform combined 20D | 1.193–1.827 | 2.667–3.869 | +0.840 | 0.890–2.639 | 2.172–3.119 | -0.468 |
| Waveform 20D + optical 2D | 1.139–2.079 | 2.655–4.068 | +0.576 | 0.849–2.519 | 2.297–3.274 | -0.222 |

**Critical finding from independent review:** On all five X recordings, the **APG-DCT 16D representation alone** has positive separation (+0.176; ratio 1.088). Both combined waveform and waveform+optical configurations exhibit negative margins (overlap). Thus adding optical calibration to the agent's combined-distance representation has **not** demonstrated improved observed discrimination. With four scan-clean X recordings, all methods separate, but inclusion of the fifth challenges the more complicated approaches. Note the audit labels its five-X condition `all_5_strict_X_bursts` even though one recording (start 955573) is marked mid-scan. Define and justify each quality exclusion before evaluating new data.

**Potential technical confound:** Owner LOO scores and Y-to-owner scores need to be computed using comparable template-size, centering, and scale estimators; the JSON contains only summary ranges, so complete equality of those calculations has not been established. The feature family was selected while looking at X/Y; using only X to fit a distance scale does not retroactively remove feature-selection leakage.

## 5. Tap kinematics and modality independence

Six owner double-tap sessions appear in the audit. Reported double-tap gaps across sessions range ~263–394 ms, peak2/peak1 acceleration ratios ~0.52–1.59, and transverse/axial ratios ~0.74–2.57. Measured gravity vectors also vary substantially with hand orientation. There are no labeled tap gestures from Y in this report. Therefore tap timing/direction alone is currently a **weak, unvalidated biometric hypothesis**.

Two optical features, `N5−N3` and `(N5−N3)/(N5+N3)`, derive from the *same calibration step pair*. Treat them as related transformations of one modality, not two independent factors. Small Pearson correlations within five samples do not prove independence, and three-modality false-acceptance rates cannot be obtained by multiplying marginal rates without joint error modeling.

## 6. Published scientific context (paper's result ≠ Ring 4 result)

| Source | Relevant message | What does not transfer directly |
|---|---|---|
| [Sancho et al. 2018](https://pmc.ncbi.nlm.nih.gov/articles/PMC5981424/) | Finger PPG can support verification; error rates worsen under time-separated conditions (roughly 19–23% EER reported for selected cases). | Different sensor and data; no Oura FAR. |
| [Seeing Red 2020](https://arxiv.org/abs/2004.07088) | Finger camera-PPG authentication; stronger within-session than across-session average performance. | Different optical modality. |
| [Yadav et al. 2018](https://arxiv.org/abs/1712.08583) | Wavelet + learned discriminant PPG biometric methods under varied physiological states. | Published low EER figures not guaranteed replicable. |
| [TrueHeart (Zhao et al., INFOCOM 2020)](https://web.eecs.utk.edu/~jliu/publications/zhao2020trueheart.pdf) | Single-channel optical waveform fiducials, constrained valley segmentation, crest time and APG; low sampling-rate ablation. Binary classifier uses labeled impostors in training. | Reported 90%+ within-sitting accuracy and ~88% at 25Hz are for that device/protocol; TrueHeart's ~3.9–4.2% 'false detection' is FRR, and ~88–90% attack detection implies ~10–12% attacker pass at the reported operating point. Agent's earlier `[120,300]ms` claimed literal pre-peak window was corrected to paper's `[150,260]ms` with `[440,740]ms` post-peak window. |
| [Know Me by My Pulse (NDSS 2026)](https://www.ndss-symposium.org/ndss-paper/know-me-by-my-pulse-toward-practical-continuous-authentication-on-wearable-devices-via-wrist-worn-ppg/) | Four-channel 25 Hz PPG showed useful verification; one-green-channel ablation substantially weaker (~43.5% classification accuracy). | One-channel Oura CVA cannot inherit multi-channel reported error rates. |
| [Fusion-ID (Apple ICASSP 2022)](https://machinelearning.apple.com/research/photoplethysmography-motion-sensor-fusion) | Combining physiological and inertial information can improve wearable authentication. | Sensors/configuration and training differ. |
| [H-PPG (2026)](https://www.mdpi.com/1999-4893/19/1/83) | PPG + 6-axis IMU and learned identity representations on a custom prototype. | Oura streams are sequential; its 3-axis accel and one CVA stream are not identical to simultaneous custom sensors. |
| [M-Ring (2026)](https://www.researchgate.net/publication/398020601_M-Ring_A_Lightweight_Ring_for_Gesture_Interaction_and_User_Authentication_with_Multimodal_Fusion) | Multimodal ring research. | Uses bio-impedance + IMU, NOT Oura's PPG+tap. |
| [PPGTransID (2026)](https://arxiv.org/html/2602.08972) | Cross-device simultaneous wearable PPG / phone-camera rPPG for physiological co-presence. | Additional phone measurement; different security goal from ring-only identity. |
| [RingAuth](https://arxiv.org/abs/2301.03594) | Gesture-specific authentication with inertial sensors on a ring. | Its sensor set, gestures and protocol are not the same as Oura accel taps. |
| [Alecci et al. 2025](https://pmc.ncbi.nlm.nih.gov/articles/PMC12736706/) | Highlights time-series leakage and poor chronological PPG-biometric generalization in some methods. | Caution, not proof all PPG identity matching is impossible. |
| [Pulse-PPG](https://github.com/maxxu05/pulseppg) / [PaPaGei](https://github.com/Nokia-Bell-Labs/papagei-foundation-model) | Downloadable pretrained PPG encoders could expose signal structure missed by coarse mean-beat Pearson. | Neither is a validated wearer authenticator for Oura 50 Hz CVA; PaPaGei reference operates with different sample rate. |

## 7. Public dataset relevance and use constraints

- [SNOWBALL Multisite-PPG](https://huggingface.co/datasets/snowballlab/Multisite-PPG): 20 participants, custom wearable devices including ring, green/infrared PPG and motion data, multiple days. Useful for waveform feature stability, sampling-rate ablations and sensor-confound checks.
- **Important limitation:** Its public files do not document Oura firmware's N3/N5 calibration steps or ring double-tap enrollment ceremonies; public ring waveform observations must not be substituted for Oura optical calibration logs.
- Previous X-vs-public hardware comparison rejected all 20 in its report, but agent's same-hardware control demonstrated that a cross-device analog/digital signal difference can dominate this outcome. It is a sensor-confounded test, not evidence of near-perfect identity rejection.
- Adjacent dataset windows overlap heavily; keep entire source sessions together for leakage-safe checks.
- The dataset's stated ban on re-identification without authorization remains a **terms-of-use ambiguity** for anonymous subject-level impostor benchmarking against X. The user views their protocol as X-verification with anonymous negatives, rather than discovering public participants' actual real-world identities. This document makes **no legal determination**; any use should respect dataset terms.

## 8. Scientific unknowns and claim boundaries

| Claim / question | Current status |
|---|---|
| Ring 4 supplies 50 Hz raw CVA PPG and live HR/accelerometer | Reported established on this hardware in project logs |
| Coarse Pearson waveform-template recognizes X vs Y | **Failed** in first pilot |
| APG-DCT16 separates existing X/Y measurements in agent audit | **Yes**, with narrow +0.176 margin across all five X waveform recordings in the audit |
| Mid-upstroke timing medians order X < Y in 4 processing variants | **Yes**, under the agent's computed feature extraction |
| N5−N3 sign differs in all 15 reported X/Y calibration bursts | **Yes**, as decoded from the attached JSON |
| Optical calibration is a stable physiological fingerprint | **Unknown**; may be position/contact/firmware-specific |
| Optical features have proven independent information beyond waveforms | **Unknown**; five-X Pearson r≈0 does not prove it |
| Adding optical to the combined matcher improves accuracy | **Not shown** in the current reported LOO margin comparison |
| Double-tap biomechanics distinguish X from Y | **Untested** in this audit (no Y tap cohort) |
| System will reject 95–99% of strangers | **Unsubstantiated**; no generalizable FAR estimate |
| Fresh same-ring recordings of X accepted, Y rejected | **Not yet evaluated prospectively with frozen features/threshold** |
| Reliable cross-day X verification | **Open**, especially since Y has only one measured day |
| Authenticity/freshness of ring waveforms against local replay | **Unsolved**; host timestamps or nonces do not cryptographically authenticate acquisition at sensor |

## 9. Source provenance and reproducibility

1. **Primary file for numeric findings:** `multimodal-audit-private.json` (agent-generated, private). The accompanying `source_sha256` field is the agent's declared hash of its input source; it has **not** been independently matched to `oura.db` within this report. JSON's own file SHA-256 is: `cd0119edac72e3b8a4706c22ecbff6493834fb73c3de5d744d843344ce700818`.
2. **Original project state:** earlier library `Pulse_Key_PPG_Biometric_Research_Update.md` (18 Oct) and `README(9).md` / `STATUS` project handoffs. These predate the new multimodal analysis and contain some historical-only statements.
3. **Evidence type:** JSON measurements/LOO/correlations are **third-party agent calculations** inspected for internal consistency. Prior chat research critiques and links are context, not an independently replicated end-to-end analysis.
4. **Privacy boundary:** `oura.db`, BLE key files, captured waveforms and tap session raw logs should remain private, local-only and out of source-control publication. This ledger intentionally contains aggregate feature values and anonymous wearer labels.

## 10. What this record is for

Preserve exactly what we know, what was discovered later, and where earlier claims were corrected. Any future agent resuming Pulse Key should begin with this document and the JSON, then check the original capture data/code before making new security claims. No project files, authenticator behavior or enrollment thresholds were modified as part of this archival work.
