# The zk crates build without features (the no_std CompressedProof that
# on-chain programs link), with only the on-chain verifier, with only the
# circuit DSL, and with every feature set the tests use.
check-zk:
    cargo check -p spr-zk-dsl
    cargo check -p spr-zk-dsl --features verifier
    cargo check -p spr-zk-dsl --features circuit
    cargo check --tests -p spr-zk-dsl --features test-support,setup,parallel,serde,tsify,wasm,verifier
    cargo check --tests -p spr-zk-gadgets -p spr-zk-macros -p spr-zk-macros-core -p spr-zk-standalone-fixture --features spr-zk-standalone-fixture/circuits
    cargo check -p spr-zk-standalone-fixture

# Unit, property and fingerprint tests of the circuit DSL and prover, the
# gadgets, the core derives, and the standalone circuit that uses them. The
# R1CS fingerprints pin every gadget and DSL operation.
test-zk:
    cargo test -p spr-zk-macros-core -p spr-zk-macros -p spr-zk-standalone-fixture --features spr-zk-standalone-fixture/circuits
    cargo test -p spr-zk-dsl --features test-support,setup,verifier
    cargo test -p spr-zk-gadgets

clippy-zk:
    cargo clippy --workspace --all-targets -- -D warnings
