# Pulse Key

**Research-stage physiological authentication using an Oura Ring 4 and a local macOS service.**

Pulse Key investigates whether an enrolled wearer's physiological signal can strengthen an authorization decision while retaining explicit user intent, local ring possession and live sensing. This repository contains a **reproducible offline identity research engine**, historical evidence, and a research plan. The original Rust tap/liveness crate source is now under `mac-daemon/crates/pulse-key/`. Its sibling `oura-link` dependency and parent Cargo workspace are still needed for a full Rust build.

> **Security status (20 November 2026):** Identity verification has not been validated prospectively. The offline research engine deliberately returns `authorized: false`. Do not deploy it as an authentication factor or treat the published experiments as a measured false acceptance rate.

## What has actually been built

- A separate local Rust demonstration was previously reported running with the physical old Ring 4: double tap, fresh heartbeat, local vault unlock and relock on ring removal. The original Rust crate source has been imported with device-specific constants removed and private recorded-session fixtures excluded. A complete build still needs the sibling `oura-link` crate and workspace manifest. It showed device custody, intent and pulse liveness, **not wearer identity**.
- A Python research engine parses historical Oura CVA PPG bursts, reconstructs waveforms and extracts optical calibration telemetry; it uses quality gates and a conservative `ACCEPT` / `REJECT` / `INCONCLUSIVE` result.
- The corrected **Phase 1 v2** engine repairs a waveform decoder defect found on 3 November 2026. It remains isolated from the live Mac authorization path.
- Historical ring-pair exploratory data suggested a sign difference in the `N5 - N3` optical calibration feature; testing involved only two wearers and post-hoc analysis. No validated cross-day or population generalization is claimed.

## Repository map

| Folder | Purpose |
|---|---|
| `research/identity/phase1_v2/` | Current corrected **offline** identity prototype and tests |
| `research/identity/archive/phase1_v1/` | Preserved original verifier; **known waveform reconstruction error** |
| `research/benchmarks/public_waveform/` | Historical public-data benchmark; v1-era results are not a valid v2 identity evaluation |
| `docs/research/current/` | Current primary evidence and next experiment protocol (3 Nov 2026) |
| `docs/research/history/` | Earlier research reports and dated limitations |
| `docs/ARCHITECTURE.md` | Existing vs proposed components and trust boundaries |
| `docs/SECURITY.md` | Threat model and private-data rules |
| `docs/MAC_IMPORT.md` | Rust source import status and missing workspace dependencies |
| `evidence/source-archives/` | Original ZIP snapshots to preserve provenance |
| `mac-daemon/crates/pulse-key/` | Imported Rust tap/liveness daemon; requires missing workspace and sibling `oura-link` dependency |
| [`data/`](data/) | Neutral reference-recordings release; externally collected, de-identified research data |

## Run the verified offline tests (macOS Apple Silicon or Linux)

```bash
cd research/identity/phase1_v2
python3 -m venv .venv
source .venv/bin/activate
python -m pip install -r requirements.txt pytest
python -m pytest -q
```

The clean standalone snapshot currently reports **14 passed, 6 skipped**, because those six tests need the private historical owner database. The November 3 source report recorded 20 passing tests when the private database was supplied. Preserve the skipped-test distinction when reporting results.

## Next experimental gate

Inspect `docs/research/current/2026-11-03_NEXT_EXPERIMENT_PROTOCOL.md`. Freeze extractor and enrollment artifacts, obtain prospective same-ring owner reseats, then consented cross-wearer trials. Record all attempted, eligible, rejected and inconclusive sessions. Hold off on identity-based live unlock until tests justify the trust claim.

## Privacy and provenance

All locally captured ring recordings, keys, `oura.db`, biometric templates, personally identifying metadata and human-subject consent records stay off GitHub. Separately licensed and publicly released third-party reference recordings are distributed through [GitHub Releases](https://github.com/Rayan-and-beyond/Pulse_Key/releases/tag/reference-recordings-v1) for non-commercial research with source attribution in `research/benchmarks/public_waveform/README.md`. `.gitignore` has broad protections; review `docs/SECURITY.md` and run `scripts/verify_clean_repo.py` before every push. This repository is public; keep individual enrollment material and personal biometric data strictly local.

The research snapshot is dated 3 November 2026. Packaging occurred 20 November 2026. No new physical-ring tests were run during packaging.
