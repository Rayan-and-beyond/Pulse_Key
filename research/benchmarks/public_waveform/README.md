# Pulse Key: frozen Phase-1 public PPG waveform benchmark

Offline research only. This does not issue authentication or call the physical ring.

## Run

Install Python 3.11+, then run `python3 -m pip install -r requirements.txt`.

Download `P1_Ring_raw.npz` ... `P20_Ring_raw.npz` from `https://huggingface.co/datasets/snowballlab/Multisite-PPG` and put the ring-only files in `/path/to/raw-ring-files/`.

Use the Phase-1 owner-enrollment CLI **privately** to create your own reference profile from your Oura database. From the resulting profile, locally construct JSON containing:

```
{"templates": <waveform.templates>, "threshold": <waveform.nn_distance_threshold>, "rise_ms_owner": <waveform.rise_ms_owner>}
```

Protect the private profile and keep it out of Git.

Then run:

```
python3 phase1_aligned_public_benchmark.py \
  --raw /path/to/raw-ring-files \
  --selection selection_manifest.json \
  --output /private/path/public_features.json \
  --owner-template /private/path/owner_template_private.json
```

Results are in `RESULTS.md` and `benchmark_results_2026-10-31.json`. The included `selection_manifest.json` contains **163 30-second window positions** preselected by the previous public-data quality screen, and no waveform contents. On the original run 139 segments passed the frozen Phase-1 `features` quality gate. The source extractor SHA256 is `9bb4609ba8b5a2a2032f5da76367b54e529045c983688ccbd6c5fd8c6fd5d8ee`.

The benchmark uses waveform scores only, since the public ring recordings lack the Oura `N5-N3` calibration data. Do not use this score to unlock a device or claim a real-world impostor false-acceptance rate. See the dataset card for CC BY-NC 4.0 and re-identification restrictions.
