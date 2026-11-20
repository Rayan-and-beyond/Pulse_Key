# For Astra: Independent Scientific and Engineering Review of Pulse Key

## Mission

You are a skeptical, technically demanding research partner. Determine whether owner-vs-unfamiliar-wearer verification can work on **one physical OLD Oura Ring 4** with accessible read-only BLE telemetry, and propose the highest-information route to either a credible prototype or a decisive falsification. Optimize correctness over optimism. Preserve the working live custody/tap/liveness daemon.

## Review inputs in Library

- `START_HERE_2026-11-02.md` and all new `Research/2026-11-02_*` documents.
- `Research/Pulse_Key_Research_Evidence_Ledger_2026-10-30.md` and `Research/multimodal-audit-private.json` (sensitive derived data).
- `Research/Pulse_Key_Phase1_Implementation_2026-10-30.md` and phase1 source ZIP.
- `Research/Pulse_Key_Public_Dataset_Experiment_2026-10-30.md`.
- `Research/Pulse_Key_Exact_Phase1_Public_Cohort_Benchmark_2026-10-31.md` and benchmark ZIP.
- Prior `Research/Oura_Pulse_Key_Agent_Handoff_2026-10-31.md`.
- Reported Mac audit artifacts: `INVENTORY.md`, `DECODING_EVIDENCE.md`, `FEATURES.csv`, `ANALYSIS.md`, `NEXT_EXPERIMENT.md`, `RECOMMENDATION.md`, `tools/optical_telemetry_audit.py`. THESE MAC ARTIFACTS ARE NOT YET IN LIBRARY; request user or local agent to supply their actual contents before claiming to have independently inspected them.

## Core observed facts

15 historical same-Oura-ring bursts X9/Y6: dN `N5−N3` X −3..0; Y +1..+3; feature and tolerance selected with knowledge of this dataset. One clean X accepted and two clean Y rejected by offline phase1, but many inconclusives. A waveform-only model failed to discriminate at least one clean Y from X on actual Oura hardware. Exact public comparison: 82/139 quality-selected *different-hardware* windows pass waveform threshold, with subject-equal cross-day AUC 0.570. User Mac agent reports 15/15 packet calibration correspondence after revised parsing, 28 tag types, 39 debug subtypes, 15×48 feature table, more path and optical scan data, and clean X/Y upstroke separation. No fresh independent prospective performance is reported.

## Specific questions to answer, with evidence

1. Is `0x61(0x33)` really a `[slot, led_dac, cfg4]` register tuple? Test all 657 rows and raw byte examples; resolve older 13/15 versus newer 15/15 agreement. Clarify bitfield interpretation.
2. What is the actual analog front end installed in this hardware? TechInsights teardown names **MAX86178F**, while Mac audit cites **MAX86171** datasheet. Verify whether inferred control registers/ADC behavior are transferable.
3. Are `0x82`, `0x83`, `0x61(0x35)` scores and operating DAC information accessible for future independent CVA sessions? Are scan-score differences physiological, position dependent, or selection bias? Identify what feature vectors truly contain new independent data.
4. Is 7-state-ish dN separation too low-dimensional to survive multiple unfamiliar people? Provide information-theoretic caution without inventing collision probabilities.
5. Can pulse morphology add new discriminative information given failure of frozen APG-DCT16 and confounding by preprocessing, heart rate and short bursts? Require cross-day holdout and subject-level metrics.
6. What single *safe* next read-only measurement offers the highest information gain? Owner rotation vs multi-day vs unfamiliar-wearer interleaving are different axes. Give a principled order.
7. Compare the realistic scientific and engineering potential of richer calibration trajectories, relative optical path scores, morphology, tap dynamics, custody binding and Mac Touch ID/passkeys. Distinguish biometric identity from the ability to verify the same ring remains worn.
8. Audit prospective study integrity: frozen interval `[-5,0]`, code SHA-256, acquisition freshness, append-only logs, class-balanced quality gates and handling of INCONCLUSIVE.
9. What evidence would justify switching from research to a real authentication factor? Specify adequate sample sizes and uncertainty bounds at the desired false-accept target, and replay/relay/handoff requirements.
10. What should be **stopped** immediately, including implausible raw channel assumptions, post-hoc tuning and confusion between vendor hardware and decoded firmware outputs?

## Required outputs

- A ranked factual finding matrix: **verified from raw source**, **Mac agent reported**, **literature-supported on another device**, **physiological hypothesis**, **disconfirmed**.
- A detailed byte-level/feature-level technical review; identify exact code to inspect and minimal test vectors.
- A small, preregistered fresh-data protocol with failure gates and complete per-attempt logging, treating Y as unavailable and asking no third party without consent.
- A distinct prospective multi-person protocol for genuine impostor specificity, ideally X↔multiple strangers crossover on the same hardware across days.
- A realistic pathway toward a useful product: if ring-only biometric identity is weak, explicit user authentication at wear/start and continuous custody/liveness/tap thereafter. Account for ring-handoff windows without active HR.
- A falsifiable recommendation: **GO / REVISE / STOP**, with stated uncertainty and concrete reason.
- No modifications to actual local devices, live daemon, raw DB or pairing secrets without the user's explicit request and access.

## Scientific traps

- Do not treat 17 unit tests as evidence of biometric recognition accuracy.
- Do not equate physically supported 18 optical pathways with 18 raw telemetry channels accessible through BLE.
- Do not use the Mac agent's guesses `70–80%`, `<15%`, `30–50%` as quantitative estimates; dataset is too small.
- Do not multiply probabilities of correlated features to imply high security.
- Do not confuse the public dataset cross-device score pass rate with real Oura FAR.
- Do not claim the Deep Research report's literature findings were reproduced on Oura hardware.
- Do not assume any prospective capture was conducted until you see original attempted-session records and hashes.
