# Pulse Key: Prospective Experiments and Go/No-Go Gates

**Status:** Proposal, no new captures recorded in this archive update. This is a preregistration template, not a claim the protocol has been executed.

## Research priorities

**Priority A: Prove a signal exists and is repeatable on owner X.** Keep frozen older optical interval `[-5,0]` and existing quality gates; run no post-hoc adjustment on new attempts. Use the existing Mac-side runner, with complete attempt logs and immutable hashes.

**Priority B: Determine whether the same signal distinguishes people rather than contact geometry.** If owner stability survives, test multiple consenting unfamiliar wearers with the *same physical ring*. Vary the order of people (X→Y1→X→Y2→X...) so slow hardware drift is not confounded with identity. Each volunteer contributes multiple independent re-seats, ideally on more than one day. Collect consent and delete raw data on request.

**Priority C: Evaluate actual usability and security.** Gate liveness, on-wear custody, deliberate tap and identity separately. A frozen statistical classifier is never a cryptographically authenticated signal. Need later replay/relay and handoff adversarial tests before real unlock use.

## Stage 0: Integrity and instrument check

- Inspect actual Mac paths, installed CLI, new script and proposed `docs/research/optical-telemetry-audit/*` files.
- Record SHA-256 of code/config/profile and offline DB snapshot. Preserve frozen defaults for the whole experiment.
- Reproduce the baseline 15-record decoder and its quality gates; reconcile historic 13/15 exact match vs claimed 15/15 new parse.
- Verify sensor type and byte decoding against primary documentation, using MAX86178F teardown as a red flag for any MAX86171-based assumption.
- Verify repeated extracted values and BLE event timestamps are genuinely from fresh acquisitions rather than a stale sync or duplicate burst.

## Stage 1: Minimal owner feasibility

**Option chosen by user:** Start with **5 independent X re-seats now** under normal orientation and usual finger; repeat **5 on a different day** if results justify it. This is a cheap, early falsification check. A larger preregistered alternative proposed by the Mac agent is **7 ordinary re-seats + 3 controlled rotations (+25°, −25°, 0°)** in one 25-minute session. These are complementary, not interchangeable: same-day rotation tests contact sensitivity; separate days test physiological/session drift. Run ordinary re-seats before intentional orientation perturbation and don't pool those conditions into an unqualified number.

Each attempt: unique ID, local timestamp Asia/Riyadh, wearer code, same finger, ring-off period, orientation, fit, skin/ambient temperature if measured, firmware, acquisition diagnostics, exact calibration traces, `N3`, `N5`, `dN`, chosen path and scan scores if parsed, waveform quality and morphology, ACCEPT/REJECT/INCONCLUSIVE, failure reason, raw local reference and all tool errors. Never replace failed attempts with successes.

**Early stopping evidence:** one or more repeated *quality-passing* genuine X rejections or positive dN observations require investigation. Multiple failures across normal re-seats/days are strong evidence that the current frozen threshold is unreliable. Unusable recordings at high frequency are an operational failure even when optical values look good. Five or ten accepts are a reason to continue, not proof of robust acceptance. Analyze quality gates independently of biometric class.

## Stage 2: Same-ring unfamiliar wearer crossover

- At least **5 consenting new individuals** as an exploratory next step, each with independent re-seats; avoid using their sessions to alter the frozen model or selected features.
- Include owner X returns interleaved with stranger tests and repeat on a later day wherever practical.
- Evaluate subject-level *ever-accepted* risk, per-session error, quality-gate failure, and distribution overlap; don't count INCONCLUSIVE as impostor REJECT.
- Zero false accepts from 5 people is weak security evidence; more people/sessions will be required for tight statistical bounds. If any stranger cleanly passes, this is direct counterevidence to high-assurance ring-only identity with the current classifier, even if numerical average accuracy looks strong.
- For optical path index / `0x35` scores / morphology hypotheses discovered from historic X/Y, select a frozen new experimental version and test on completely different wearers; never relabel old post-hoc findings as prospective.

## Stage 3: Model and feature research, only when justified

Ranking:

1. **Read-only calibration control trajectory and path-scan feature vectors:** potentially new information if actual raw records vary independently and exist across sessions; first validate packet semantics and stability. Engineer robust parsers and use strict per-person, per-day held-out splits.
2. **PPG + optical diagnostic fusion:** possible if each contributes genuine held-out discrimination; avoid combining highly correlated features as independent votes. Include quality, motion and temperature confounds explicitly.
3. **Better pretrained waveform embeddings:** secondary research only if prospectively trained/evaluated on distinct individuals and days. Prior same-ring and public exact-extractor results make a waveform-only quick fix unlikely.
4. **Additional passive modalities:** live HR/IBI and IMU may aid motion/quality, continuity and deliberate gestures; identity specificity remains unproven.
5. **Multiwavelength raw channels:** attractive scientifically but currently not demonstrated obtainable via safe read-only BLE capture. Do not plan around hypothetical raw IR/red streams.

## Security architecture if ring-only identity stays weak

Use an actual Mac-bound passkey/Touch ID or user-approved credential once at wear/session establishment, then ring-based continuous on-wear custody + live HR + deliberate tap + timeout as an *extra factor*. A wearer change that goes undetected during inactive HR / off-wear gaps is a known hazard to test; challenge at risk boundaries. Secure channel/session binding is a separate implementation requirement. Do not make biometric classifier output directly authorize the existing live vault until independent security validation.

## Definition of honest result

- **Experiment reproducibility:** original raw data and hashed code can reproduce exact features.
- **Usability:** good fraction of all genuine attempts yield an eligible decision, with measured INCONCLUSIVE rate.
- **Owner stability:** stable acceptance across normal re-seats and several days.
- **Impostor specificity:** independent unfamiliar wearers tested on the SAME ring, with realistic crossovers.
- **Security:** replay, relay, handoff and state-machine tests; calibration scores by themselves do not provide cryptographic identity.

## No-go triggers

- High-quality X regularly falls outside the frozen boundary or readings move with mild rotation/pressure rather than wearer.
- More than trivial collision rates across different wearers in the small observed discrete feature range.
- Path-scan or `0x35` values turn out to encode only firmware settings, or cannot be acquired consistently.
- Improvements disappear when evaluated on new people/days or when acquisition order and contact are controlled.
- Autonomous tuning on the test set, cherry-picked quality gates or inaccurate packet semantics invalidate a positive performance report.
