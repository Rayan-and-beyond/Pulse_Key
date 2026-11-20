# Pulse Key: Phase 1 experimental identity engine

**2026-10-30. Offline replay candidate, not connected to the live Pulse Key daemon.**

This package implements and tests a read-only wearer verification prototype against Oura Ring 4 capture history (`oura.db`). It **never communicates with the ring**, modifies the ring, uses a pairing key, sends network requests, or unlocks anything. The experimental `ACCEPT` output contains `"authorized": false` deliberately. The existing tap/liveness/custody demonstration remains untouched.

The live Rust `ring/crates/pulse-key` repository was not present in the source attachments available to this session. Its Mac source was not modified. The package is portable and can be added under the local repo's `ring/tools/pulsekey_identity_phase1/` for independent testing. Integration should only happen after controlled prospective testing establishes useful discrimination.

## What was implemented

- Read-only SQLite adapter, with automatic CVA burst discovery and absolute-marker/delta waveform reconstruction.
- Independently parsed **both** `0x43` ASCII calibration records (`E:3N`, `E:5N`) and `0x61` subtype `0x33` low-level register traces; both channels' sequences must corroborate the calibration step counts.
- Quality checks for missing optical values, inconsistent register traces, `0x82` mid-burst scans, `0x47` motion, short recordings and unusable waveform features.
- APG-DCT16 morphology descriptor extracted from 50 Hz PPG, resampled for interpolation to 250 Hz and bandpass-filtered 0.5–12 Hz. Pulse segmentation and feature extraction here are **independent implementations**, not verbatim reproduction of the other agent's `multimodal_audit.py`. Interpolation does not create physical sensor resolution.
- Owner-only calibration interval and nearest-neighbor waveform threshold; enrollment requires three distinct quality-passing recordings. No Y values are used in fitting thresholds.
- Per-modality verdicts, a conservative AND gate, and an `INCONCLUSIVE` result for insufficient-quality recordings. Enrollment recordings cannot be self-scored.
- macOS-compatible CLI with `list`, `enroll`, `verify`, and `backtest`, plus unit and raw-database regression tests.

## Installation on Apple Silicon macOS

Requires Python 3.11+ (Homebrew Python works).

```bash
cd ~/Desktop/OuraDeepSeek
mkdir -p ring/tools/pulsekey_identity_phase1
# Copy the contents of the supplied phase1 ZIP into that folder.
cd ring/tools/pulsekey_identity_phase1
python3 -m venv .venv
source .venv/bin/activate
python -m pip install -r requirements.txt
```

The command paths below assume the current directory is the extracted package directory and the local `oura.db` is in the OuraDeepSeek project root.

```bash
python pulsekey_identity.py --db ../../../oura.db list
```

**Example exploratory enrollment** from the three known earlier clean X captures:

```bash
python pulsekey_identity.py --db ../../../oura.db enroll \
  --starts 715158 2659172 2661558 \
  --out owner_profile_private.json
```

The profile contains **sensitive derived biometric data**. Keep it private and out of Git (add `owner_profile_private.json` to `.gitignore`). The CLI creates this file with mode `0600` and refuses to overwrite an existing profile.

**Replay previously unseen captures:**

```bash
python pulsekey_identity.py --db ../../../oura.db verify \
  --profile owner_profile_private.json --start 2699937
python pulsekey_identity.py --db ../../../oura.db verify \
  --profile owner_profile_private.json --start 2703303
```

**Read-only research evaluation with X/Y labels from the private audit** (not a valid population FAR/FRR estimate):

```bash
python pulsekey_identity.py --db ../../../oura.db backtest \
  --audit /path/to/multimodal-audit-private.json
```

The audit is used for known enrollment labels and the benchmark's expected identities. It is not consulted by the verification rule for the test recordings.

## Tests

```bash
python -m pip install pytest
python -m pytest -q
```

Run the private-database regression tests too:

```bash
PULSE_KEY_TEST_DB="$HOME/Desktop/OuraDeepSeek/oura.db" \
PULSE_KEY_TEST_AUDIT="/path/to/multimodal-audit-private.json" \
python -m pytest -q
```

## Actual retrospective outcomes in the supplied Oura database

Owner-only enrollment used X starts `715158`, `2659172`, `2661558`. Both channels and waveform quality had to pass. The optical threshold was `[-5, 0]`, from the enrollment step-difference range `[-3, -2]` plus **two steps of predefined software tolerance**. The DCT descriptor threshold was computed from enrollment-only leave-one-out nearest-neighbor distances (`0.793096` in this run).

| Held-out result | X | Y |
|---|---:|---:|
| `ACCEPT` | 1 | 0 |
| `REJECT` | 0 | 2 |
| `INCONCLUSIVE` | 2 | 4 |

The qualifying later X recording `2699937` had optical difference `0`, waveform distance `0.38668` and passed both checks. The two later Y recordings `2703303` and `2708139` had differences `+3` and `+2`, respectively. **Both Y recordings passed the waveform threshold; optical calibration did the rejecting.** Two early Y captures were short, one had motion, and another lacked binary calibration corroboration. Two later X captures had motion/scan quality exclusions. For inconclusive captures the software may display optical values as diagnostics, but never uses them to issue an identity decision.

**Independent raw-log check:** The earlier agent marked start `2696276` as lacking one binary register step. Our parser found the missing `0x51` register update embedded inside a longer debug packet, which explains the mismatch; this capture still fails the motion/scan quality gate.

**Critical sensitivity finding:** Using zero optical tolerance, the later clean X recording at `2699937` is rejected. The `+2`-step tolerance used in the successful replay was selected after earlier exploratory review of all X/Y recordings. Thus the positive result is **not independent prospective validation**, despite enrollment-only threshold fitting in this particular replay. The tolerance must be frozen before any future fresh ring sessions.

On this independent extractor the waveform-only method does not reliably reject Y. Earlier APG-DCT findings in `multimodal-audit-private.json` refer to a *different* implementation and cannot be assumed reproduced.

**Statistical boundary:** One clean held-out X and two clean held-out Y recordings produce no dependable population-level accuracy, false acceptance rate or false rejection rate. Device contact, position and optical calibration steps may confound identity. Genuine on-body freshness/authenticity and replay resistance require separate security engineering.

## Workflow and safety

1. Record X using the *existing* old ring, with repeated reseating and different days; keep the enrollment profile frozen.
2. Score future captures without seeing the true wearer label at decision time. Record `ACCEPT`, `REJECT`, `INCONCLUSIVE`, including reasons and per-feature scores.
3. Assess how often different people pass under the same physical ring hardware. Keep raw private telemetry local.
4. Only after independent evidence, decide how and whether to integrate into the live Rust daemon's verification registry. Do not auto-enable identity-based unlocking from this prototype.

## Files

- `pulsekey_identity.py`: CLI
- `observations.py`: Oura SQLite read-only adapter, optical diagnostics and quality gates
- `feature_extraction.py`: reconstructed PPG and custom APG-DCT16
- `verifier.py`: owner-only enrollment and per-modality decision logic
- `tests/test_verifier.py`: deterministic tests and optional real-database regression
- `requirements.txt`: NumPy and SciPy
