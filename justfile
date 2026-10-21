# The zk crates build without features (the no_std CompressedProof that
# on-chain programs link), with only the on-chain verifier, with only the
# circuit DSL, and with every feature set the tests use.
check-zk:
    cargo check -p zolana-zk-dsl
    cargo check -p zolana-zk-dsl --features verifier
    cargo check -p zolana-zk-dsl --features circuit
    cargo check --tests -p zolana-zk-dsl --features test-support,setup,parallel,serde,tsify,wasm,verifier
    cargo check --tests -p zolana-zk-gadgets -p zolana-zk-macros -p zolana-zk-macros-core -p zolana-zk-standalone-fixture --features zolana-zk-standalone-fixture/circuits
    cargo check -p zolana-zk-standalone-fixture

# Unit, property and fingerprint tests of the circuit DSL and prover, the
# gadgets, the core derives, and the standalone circuit that uses them. The
# R1CS fingerprints pin every gadget and DSL operation.
test-zk:
    cargo test -p zolana-zk-macros-core -p zolana-zk-macros -p zolana-zk-standalone-fixture --features zolana-zk-standalone-fixture/circuits
    cargo test -p zolana-zk-dsl --features test-support,setup,verifier
    cargo test -p zolana-zk-gadgets

clippy-zk:
    cargo clippy --workspace --all-targets -- -D warnings
