# Reproduction notes

Verify report SHA256SUMS before using artifacts. Work in a new root; do not overwrite or repeat the completed experiment for bookkeeping.

`receipts.json.gz` decodes to JSON with `members` containing `path`, `utf8`, `bytes`, `sha256`. Preserve UTF-8 contents including final whitespace, check lengths/hashes and require relative safe paths. Original historical command paths remain as provenance and require relocation for a new run.

The integer model needs pinned num-bigint, num-traits and serde_json libraries, listed in the model compile receipt and parent source pins. `model.rs` is the initial compile-failed source; `model-fixed.rs` is authoritative, differing only by explicit BigUint/usize annotations. `prepare.rs` and `repair.rs` produce their separate plans/pins; `model-fixed-plan.json` contains the successful compile and execution commands. Do not run the initial failed plan as a prerequisite to reproduction.

Recover the 176-file accepted source using its linked manifest/current repository revision and compare every original hash. Apply only `source-audit.json`'s field-lib suffix and copy `kernel.rs` to its indicated child. Compare the resulting177file hashes with `snapshot-hashes.json`; public binary source fixture bytes must be preserved rather than decoded as UTF-8.

`native-plan.json` contains both native Cargo test commands, explicit compiler/toolchain declarations, accepted cfgs, native CPU flags and separate target directories. Checked mode adds overflow/debug assertions. The existing release profile configures optimization/LTO/CGU. These are correctness tests, not measured native performance drivers. Only the two new `signed_square::tests::` tests run;35 inherited tests are filtered. All test vectors are generated in the published source.

`review.rs` is an independent read-only reviewer using serde_json and sha2, with no model/kernel includes. It intentionally uses historical absolute paths and refuses to overwrite a successful review. Adapt output paths for a new study. `review-initial.rs` is the preserved reviewer before correcting managed-environment receipt expectations. The accompanying runner/config source documents why `command.json` omits RUSTC/RUSTUP_TOOLCHAIN while protocol declarations and execution retain them.

The model and native source qualifications do not authorize a speedup claim. A separate complete-caller protocol, coherent original/fresh binaries, correctness checks and optimized-link inspection are required before timing. Preserve the original OFF expression byte-for-byte. Complete-caller performance, full SDK/Gnark/all-eight/54/portable/memory admission and pressure fallback remain unqualified.
