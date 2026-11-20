# Pulse Key: next executable experiments (3 November 2026)

## Objective and governance

Determine whether the actual OLD Oura Ring 4 can identify enrolled owner X through independent wearing sessions, across days, and reject other consenting individuals using that **same physical ring**. Existing historical X/Y feature-selection is contaminated by post-hoc development; predeclare the next tests and preserve raw logs. The live `ring/crates/pulse-key` Mac daemon remains unchanged; research outputs must never unlock it.

## Gate 0: local engineering repair (agent; no new ring capture required)

1. Compare Astra's raw-byte decoder audit with the corrected v2 ZIP and actual local Rust `ring/crates/oura-protocol/src/events.rs`. Add fixtures verifying non-marker packet-relative offsets, packet partition changes, absolute-marker boundaries, truncation and marker reset.
2. Audit `0x61` subtype `0x33` parser for both 6-byte tuples, slot variants and expected register values. Recognize that 15/15 *multiset* agreement does not prove chronological hardware writes. Investigate the 1/15 nonmonotonic trace; abstain on unsupported instrument states.
3. Run tests with private database read-only. Verify expected source SHA-256 and corrected scores: threshold **0.517940**; clean X **0.170049**, two clean Y **0.188865**, **0.196780**. V2 schema cannot ingest v1 owner template. Investigate any mismatch before collecting data.
4. Preserve frozen Mac optical-only `[-5,0]` baseline and its existing hashes. Stage v2 only as a separate offline shadow scorer, with version/hash manifest. Ensure any new code explicitly reports `authorized:false` and cannot affect the live unlock. No raw biometric or key uploads, no public PR/commits of private features.
5. Validate acquired data freshness in offline harness: device identity, timestamp boundaries, duplicate burst ID detection, replay/old-history refusal, missing binary provenance => `INCONCLUSIVE`. Existing host BLE pairing is not sensor-signed freshness proof; threat-model replay separately.

**Gate 0 success:** deterministic test suite passes, archived raw-derived waveforms match independent checks, immutable trial logging and versioned models are ready. If any unexplained packet inconsistency remains, stop biometric interpretation until fixed.

## Gate 1: owner repeatability (10 independent ordinary reseats, two days)

- Day 1: **5 fresh captures**. Day 2 or later: **5 fresh captures**, different day. Owner wears the SAME OLD ring on the habitual enrolled finger. Full removal and comfortable reseat between each. Allow ~45–60s settling; account for 4.9–10.6s of optical calibration **before** the 30s CVA waveform. Document actual latency. No deliberate stress rotations in these ten ordinary trials.
- Assign `D1-A01`...`D1-A05`, `D2-A01`...`D2-A05`; store timestamps with Asia/Riyadh offset, ring firmware and stable device ID locally, forearm/finger and orientation notes, optional temperature/fit, calibration N3/N5/difference, active slot, register multiset status, scan/motion, waveform quality, v1 optical-only verdict, v2 shadow verdict and errors. Log failed attempts.
- Freeze both pipelines **before** recording. No threshold retuning after any trial. Separate all-attempts denominator from quality-passing conditional denominator and annotate reasons for `INCONCLUSIVE`. Do not replace failed acquisitions with extra successful ones.
- **Engineering continue screen:** >=9/10 attempts independently `ACCEPT` under the frozen optical baseline, with acceptable measurement coverage. This is a *screen*, not an accuracy certificate. Repeated eligible owner dN >0, >=2 attempts failing to accept, or frequent capture-quality failures warrant investigation/stop for the current optical-only version. Capture failures differ from genuine negative outcomes.
- If the first 5 already show multiple clean owner rejections, don't keep collecting dozens to hunt a passing result. Diagnose pressure/orientation/temperature and reconsider candidate while preserving this test unchanged.

## Gate 2: stranger falsification (requires consenting volunteers)

- **Five unfamiliar wearers**, three independently donned attempts on each of two days => **30 stranger attempts**, using the same old ring and same protocol. Finger compatibility, comfortable fit, orientation and ethics/consent are required. Rotate collection order and interleave return-to-owner recordings.
- Score blindly using the same frozen baseline, separately scoring v2 shadow model. No supervised tuning on these five people. Keep source user labels private; do not publish identifiable waveform records.
- **Any clean stranger `ACCEPT`** falsifies the strict *zero accept among these strangers* outcome for that method. No clean stranger accepts is an exploratory reason to expand testing, not proof of 'reject anybody else.' Track person-level ever-accepted, number of accepted attempts, number inconclusive and operational usability. If >20% inconclusive, acquisition quality must be solved before security claims.
- Include different days and reseats; session-independent negative trial counts do not translate into independent population samples. Five strangers are five person-level units, regardless of number of beats/attempts.

## Gate 3: compare and freeze improved identity candidate

Only after Gates 1/2, use discovery data to compare: optical difference baseline; full `(N3,N5)` with documented instrument state; corrected waveform-only; small regularized optical+waveform fusion; optionally constrained motion challenge with consent. Split by **person, day and donning** before feature selection. Choose an operating point using training/validation data, freeze it, then recruit **new** people and new sessions for confirmation. No deep-learning search on the same 2-person history.

For security-grade targets choose an explicit application-level acceptable FAR, FRR, INCONCLUSIVE rate and latency; estimate with person-/session-cluster-aware uncertainty. Zero observed false accepts in 30 attempts cannot bound real-world FAR below 1%; under independent trial assumptions its one-sided 95% upper bound is about 9.5%, and actual repeated-wearer samples are correlated. Security threats (replay, stale logs, host compromise, relay, removal/handoff) require dedicated evaluation separately from statistical scoring.

## Alternative system path

If identity fails, preserve the existing custody/liveness/tap demo and evaluate explicit tap-entered secret or strong Mac Touch ID/passkey at initial donning with conservative ring-wear continuity. That is a practical authentication design **but does not meet ring-only physiological X identity**. Multiwavelength optical access is a conditional hardware-development path, not a demonstrated data stream.

## What user and agent should do next

**Agent now:** complete Gate 0 on the local Mac, preserve hash manifest, and prepare a single-command, read-only attempt runner that logs every attempt.

**User after Gate 0 verified:** wear the OLD ring, remove/reseat normally, and capture five owner trials with the existing safe method. Leave the ring untouched until the agent cues movement. The following day run five more. Start recruiting strangers only if owner stability clears the engineering screen.
