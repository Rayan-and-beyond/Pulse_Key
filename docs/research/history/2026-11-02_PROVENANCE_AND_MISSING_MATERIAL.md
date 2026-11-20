# Pulse Key Library: Provenance, Completeness and Import Plan

**Prepared:** 2026-11-02. The purpose is to make future Astra conclusions auditable.

## Existing files already preserved in the actual Library

Location `/Pulse Key/`:
- `README.md` (2026-10-31 status; keep as historical baseline).

Location `/Pulse Key/Research/`:
- `Pulse_Key_Research_Evidence_Ledger_2026-10-30.md` (history and methodological limits).
- `multimodal-audit-private.json` (sensitive derived values, not for public sharing).
- `Pulse_Key_Public_Dataset_Experiment_2026-10-30.md`.
- `Pulse_Key_Phase1_Implementation_2026-10-30.md`.
- `Pulse_Key_Phase1_Identity_Engine_2026-10-30.zip`.
- `Pulse_Key_Exact_Phase1_Public_Cohort_Benchmark_2026-10-31.md`.
- `Pulse_Key_Public_Waveform_Benchmark_2026-10-31.zip`.
- `Oura_Pulse_Key_Agent_Handoff_2026-10-31.md`.
- `Pulse_Key_Oura_Updated_Handoff_2026-10-31.zip`.

## Reported local Mac files NOT available in ChatGPT Library at archive time

Agent's actual six audit documents (do not invent them or imply that they were transferred):

1. `docs/research/optical-telemetry-audit/INVENTORY.md`
2. `docs/research/optical-telemetry-audit/DECODING_EVIDENCE.md`
3. `docs/research/optical-telemetry-audit/FEATURES.csv` (**sensitive; keep local by default**)
4. `docs/research/optical-telemetry-audit/ANALYSIS.md`
5. `docs/research/optical-telemetry-audit/NEXT_EXPERIMENT.md`
6. `docs/research/optical-telemetry-audit/RECOMMENDATION.md`

Also reported but not present here: `tools/optical_telemetry_audit.py`, six deterministic tests, `tools/optical_calibration_verifier.py` (current Mac version), `docs/experiments/optical-calibration-prospective-v1.json`, `run_prospective_attempt.py`, `retrospective-baseline.json`, `data/sessions/prospective-optical-2026-10-31/attempts.jsonl`. Existing Library Phase 1 ZIP is a **different earlier** independently implemented offline verifier and should not be mistaken for the Mac agent's new script.

A concise agent report supplied in conversation has been preserved as `OPTICAL_TELEMETRY_AUDIT_SUMMARY_2026-11-02.md`. The new actual source artifacts must be uploaded for byte-for-byte verification and reproducibility. Any local user-authored documents can be shared separately, with sensitive CSV and keys omitted. **No pairing key and no raw `oura.db` upload is necessary for Astra's initial review.**

## Recommended safe transfer from Mac

Ask the local agent to export a review bundle containing the six `.md` files, `optical_telemetry_audit.py`, safe test vectors with synthetic/non-sensitive data, and an aggregate-only feature summary. The actual `FEATURES.csv` can remain local or have sensitive per-person/per-session rows removed; never expose BLE key, raw pulse waveform, enrollment template or private absolute identifiers. Include a SHA-256 manifest of all exported files. Put archive in ChatGPT Library `/Pulse Key/Research/` and link it to these source records.

## Evidence tiers

- **Tier P1 (direct Library artifact):** older ledger, private JSON, Phase 1 code ZIP, exact public benchmark and its scripts.
- **Tier P2 (user-pasted Mac agent statement):** decoder details, 15/15 agreement, 28/39 tag inventory, 48-column feature table, newly found raw register semantics and 6 tests. Numerical claims are not independently rerun in ChatGPT.
- **Tier P3 (peer-reviewed outside research):** hardware and published biometric studies on other devices; guide hypotheses, not Oura performance.
- **Tier P4 (hypothesis):** optical calibration, path index and APG as stable wearer-specific biometric signature; untested prospectively.

## Verification checklist for Astra

- Confirm each source file exists and read it.
- Obtain raw-byte fixture samples for 0x61(0x33) and 0x35, and list expected decoder output.
- Compare new parser output per burst to the 15-row archived baseline, naming the exact discrepancies and reasons.
- Verify MAX86178F vs MAX86171 documentation applicability.
- Confirm frozen SHA-256 hashes and show any new prospective attempt with its actual time.
- Preserve original records, never overwrite bad or historical results with favorable reinterpretations.
