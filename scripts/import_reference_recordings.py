#!/usr/bin/env python3
"""Fetch the publicly released, de-identified ring recordings for archival.

Use only with the separate source's CC BY-NC 4.0 terms.
Source attribution remains in research/benchmarks/public_waveform/README.md.
This script never reads an Oura key, private wearer profile or local ring DB.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import sys
import time
import zipfile
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

from huggingface_hub import hf_hub_download

SOURCE_ID = "snowballlab/Multisite-PPG"
EXPECTED_SUBJECTS = tuple(range(1, 21))


def fetch_one(index: int, output: Path, cache: Path) -> dict[str, object]:
    remote_path = f"raw_data/P{index}/P{index}_Ring_raw.npz"
    destination = output / f"P{index}_Ring_raw.npz"
    for attempt in range(1, 4):
        try:
            downloaded = hf_hub_download(
                repo_id=SOURCE_ID,
                repo_type="dataset",
                filename=remote_path,
                cache_dir=str(cache),
            )
            shutil.copyfile(downloaded, destination)
            with zipfile.ZipFile(destination, "r") as archive:
                if archive.testzip() is not None:
                    raise ValueError(f"Archive integrity failed: {destination.name}")
                if not archive.namelist():
                    raise ValueError(f"Empty archive: {destination.name}")
            digest = hashlib.sha256()
            with destination.open("rb") as source:
                for block in iter(lambda: source.read(1024 * 1024), b""):
                    digest.update(block)
            return {
                "filename": destination.name,
                "bytes": destination.stat().st_size,
                "sha256": digest.hexdigest(),
                "participant_id": f"P{index}",
            }
        except Exception:
            destination.unlink(missing_ok=True)
            if attempt == 3:
                raise
            time.sleep(attempt * 8)
    raise AssertionError("unreachable")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--cache", type=Path)
    args = parser.parse_args()
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=True)
    cache = (args.cache or (out.parent / "_transfer_cache")).resolve()
    cache.mkdir(parents=True, exist_ok=True)

    records = []
    with ThreadPoolExecutor(max_workers=3) as pool:
        futures = {
            pool.submit(fetch_one, index, out, cache): index
            for index in EXPECTED_SUBJECTS
        }
        for future in as_completed(futures):
            result = future.result()
            records.append(result)
            print(f"Verified {result['filename']}: {result['bytes']} bytes", flush=True)

    records.sort(key=lambda v: int(str(v["participant_id"])[1:]))
    total = sum(int(v["bytes"]) for v in records)
    if len(records) != 20:
        raise RuntimeError(f"Expected 20 recordings, got {len(records)}")
    (out / "SHA256SUMS.txt").write_text(
        "".join(f"{r['sha256']}  {r['filename']}\n" for r in records),
        encoding="utf-8",
    )
    # A count-and-checksum manifest; origin and license are documented in repo.
    (out / "MANIFEST.json").write_text(
        json.dumps({"count": len(records), "total_bytes": total, "files": records}, indent=2) + "\n",
        encoding="utf-8",
    )
    print(f"Prepared {len(records)} verified recordings ({total} bytes).", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
