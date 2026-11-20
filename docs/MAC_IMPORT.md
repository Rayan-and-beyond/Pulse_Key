# Import the live Mac Rust project

The source for the previously reported working `ring/crates/pulse-key` daemon is **not present** in the archival Library files or connected Linux VM. Historical Mac workspace path was `~/Desktop/OuraDeepSeek`, which must be verified on the Mac. This placeholder tracks the missing component.

## Intended layout

```text
mac-daemon/
  Cargo.toml              # only after verifying the actual source/workspace
  crates/pulse-key/        # actual source imported after a secret and binary review
  docs/                   # source-specific status, decisions and demo instructions
```

Before import, make a **copy** of the original local workspace, inspect `git status`, identify the real Cargo workspace root, preserve upstream licensing and attribution for any vendored `open_oura`, then scan the files. Avoid exporting any `.key`, `oura.db`, `data/sessions`, enrollment profiles, ring identifiers, build products or historical raw packet traces.

Do not auto-merge the corrected Python identity model into the Rust authorization path. Compare decoders, capture invariants and independent study evidence first.
