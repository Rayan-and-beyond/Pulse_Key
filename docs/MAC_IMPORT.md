# Rust source import status

Source ZIP: user-supplied `pulse-key.zip` on 20 November 2026.

The Rust crate is now included at `mac-daemon/crates/pulse-key/`. The original project workspace was previously documented under `~/Desktop/OuraDeepSeek/ring/` on macOS.

To build and test it on a Mac, restore the original parent Cargo workspace manifest and the sibling `oura-link` crate in the expected layout:

```text
ring/
  Cargo.toml                 # workspace manifest; not in supplied ZIP
  crates/
    oura-link/               # required Rust BLE dependency; not in supplied ZIP
    pulse-key/               # imported here
```

Inspect and preserve `oura-link`'s copyright, dependencies, license, and original source provenance before uploading it.

The public `pulse-key` source replaces hardcoded device identifiers with required CLI arguments:

```bash
cargo run -p pulse-key -- \
  --address "<local-BLE-transport-id>" \
  --expected-mac "<local-ring-mac>" \
  --expected-serial "<local-ring-serial>" \
  --key-file "$HOME/.config/pulse-key/ring.key"
```

These values must come from your own local ring and should never be committed. Six original raw acceleration fixtures are intentionally excluded from the public repo; to run their regression tests locally, restore them into `crates/pulse-key/tests/fixtures/` and enable `--features private-fixtures`.

The Rust crate's live hardware behavior has **not** been independently re-tested as part of this upload. The Python identity verifier remains offline and deliberately separate from the live authorization path.
