# Command-environment receipt normalization

The first offline qualifier stopped before producing qualification output because it compared every declared command variable with the receipt's filtered environment. The receipt intentionally retains only `xtask::config::managed` variables; `LLVM_PROFILE_FILE`, `RUSTC` and `RUSTUP_TOOLCHAIN` are passed to the process but absent from that display. `workflow.rs` passes the full environment to `run`; `process.rs` calls `Command::envs(environment)` and filters only its saved receipt.

Preserve this source/failure. Replace the reviewer's ad hoc two-variable exclusions with the exact managed-variable predicate from `xtask/src/config.rs`. Declared protocol environment equality, actual profile paths, source/library pins and output hashes remain checked. Do not change the process environment or rerun any successful build, test, capture, merge or verifier. This is a receipt-check repair, not an optimization or arithmetic failure.
