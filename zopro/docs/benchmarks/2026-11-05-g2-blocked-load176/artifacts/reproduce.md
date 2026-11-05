# Reproduction

This stage qualifies a codec; it does not benchmark proof latency.

The local root is `/home/ubuntu/src/zopro-perf-work/g2-blocked-load176`. `setup.rs` constructs a 177-file snapshot from the accepted 176-file manifest, appends a guarded field-module export, and adds the published `blocked_input.rs` child. It preserves binary fixture bytes. Original arithmetic, generated files, core, and prover are unchanged. Do not rerun successful work for bookkeeping.

`native-plan.json` records exact tool paths, argv, environment, timeouts, required focused tests, and pins. Native Rust is 1.98.1, with `target-cpu=native` and the accepted cfg flags plus `zopro_blocked_load_probe`. Run the published xtask workflow against that protocol in a fresh destination if independently reproducing. Its three steps execute the two named tests, build coherent libraries, and freeze transitive dependencies using `native.rs`. The 35 other field tests are filtered. No final timing executable is linked or run in this stage.

`review.rs` checks protocol defaults and managed-environment behavior, all three command and stream receipts, source/library pins, the two test results, append-only recovery of all 176 accepted source files, loader source guards, and the unchanged accepted SDK/29,953-proof ledger. It does not import the loader as its own oracle and is not a formal verification tool. `bounds.md` explains storage and intrinsic address bounds.

The coherent frozen rlibs and executable helpers remain local and are excluded from the archive. `frozen-candidate.json` records their exact pins. Extract `source.tar.gz` and compare against `source-members.json`. Decompress `receipts.json.gz`; each member contains its relative path, exact UTF-8 content, byte count, and SHA256. Some reproduction paths refer to retained private fixtures and local xtask/tool dependencies; those inputs are not redistributed.

Complete-distribution codec timing and complete curve/MSM/SDK/resource/device qualification remain future work. The accepted implementation is unchanged.
