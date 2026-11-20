# Local acceleration test fixtures

The original archive contained six raw accelerometer recordings under `tests/fixtures/`.
These are human-motion traces, so the recordings are intentionally excluded from the
public repository. Original ZIP remains with the project owner.

The default test suite covers deterministic/synthetic tap tests. To enable the six
recorded-session regression tests locally, copy the original `.jsonl` files into
this directory and run:

```bash
cargo test -p pulse-key --features private-fixtures
```

Never commit the recordings.
