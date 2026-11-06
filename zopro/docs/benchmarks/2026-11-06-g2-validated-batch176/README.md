# Once-validated borrowed G2 batch qualification

The candidate validates encoded bucket indices once before mutation, then uses fixed-size batch groups and justified unchecked bucket/prefix-record accesses in the existing G2 kernel. **Correctness and source/receipt review pass; no latency benchmark has run.** G2 instruction rows fall 3.62% and bounds-panic call sites fall from 37 to 2, with an additional upfront assertion path. Field arithmetic, retained scratch, input representation and the accepted prover remain unchanged.

## Native qualification

Both cores and actual callers link the original accepted field library coherently. ON passes 128 unfiltered core tests; OFF passes the new shared integration test with 125 unchanged tests filtered. The integration covers 14 empty/full/tail lengths through 511 and 1/3/17 reuse counts. Two additional ON tests cover invalid destinations before any write, empty slices and signed bucket-zero indexing. Original G2 sign-mask, exception, tails, reuse and small-stack pool tests also run in ON. Field suites and prior qualified builds are inherited without repetition.

All eight build/test/link commands succeed. Review checks command argv, exit/output hashes, 176 original source recovery and 173 byte-exact originals among 178 snapshot files. The three changed parents retain reversible deltas; two new children hold the kernel and diagnostic adapters. Arithmetic call-site counts match the original kernel. This source review and testing do not constitute a formal unsafe-code proof.

## Final consumer code

Six newly decoded functions cover fresh OFF/ON G1/G2/vector inverse; three frozen reference decodes are reused with hashes rather than repeated. Counts cover entire static functions, including cold paths. They do not measure executed checks, instruction cost, spills, memory peaks or proof latency.

|Build/function|Instruction rows|IFMA sites|Bytes|Vector stack-operand rows|Bounds-panic call sites (new decodes)|
|---|---:|---:|---:|---:|---:|
|OFF/G1|4425|800|26198|286|24|
|OFF/G2|8590|1450|51830|603|37|
|OFF/inverse|1139|500|6979|67|0|
|ON/G1|4425|800|26198|286|24|
|ON/G2|8279|1450|49994|606|2|
|ON/inverse|1139|500|6979|67|0|
|FROZEN/G1|4425|800|26198|286|inherited|
|FROZEN/G2|8590|1450|51830|603|inherited|
|FROZEN/inverse|1139|500|6979|67|inherited|

## Safety and scope

The immutable batch and exact mutable bucket slice are borrowed for the entire call. Every masked destination is validated before any scratch or bucket write. Full chunks partition the original prefix exactly, and the completed forward loop produces one record for each reverse chunk. Points are copied before writes; no simultaneous mutable point aliases are constructed. Tail storage remains checked. Distinct destinations and ordinary nonzero denominators remain existing arithmetic preconditions, not unsafe-memory assumptions. The added validation has a real scan cost, which must be included in any later caller measurement.

No descriptor array, coordinate validation, hardware gather, point/key copy, lookup table, retained plan, pool or buffer is added. Original field/Fr FFT, arithmetic, inversion, signs, tails, exception guards, queue geometry and production policy remain. This differs from prior gather/descriptor/late-coordinate experiments; no exhaustive novelty claim is made.

Before the first native compile, a cast comparison was parenthesized in generated child and generator source, and only their source/audit pins were updated. Setup was not repeated. The archived setup binary predates that text correction; building the final setup source emits the corrected child directly. No native compile or test failed in this preparation.

The frozen389307 control is an audited accepted-source diagnostic caller, distinct from the production SDK. Its recorded field input is described in the inherited [provenance note](../2026-11-06-joint-pair52-callers176/artifacts/provenance-note.md). Fresh OFF/ON explicitly use the original field1200. Paired calibration is still required before performance claims.

## Next step and accepted baseline

The intended removal is present in the final consumer. Next, freeze a prospective complete-caller comparison with both controls, tails, 1/16/32 workers, unchanged G1 controls and automatic PMU/cycle profiles. Keep validation inside the timer. A caller win would select all-eight plus merge54 SDK, resource/portable and independent Gnark proof checks before adoption.

Accepted local 32-worker 1×2 remains **7.928660 ms**, merge54 **161.883899 ms**. Inherited 132.75 MiB RSS is not a new peak or minimum RAM. Ledger 30846 is unchanged. All-eight CPU 10× remains active and unmet (0/8); inherited [M5 CPU/Metal comparisons](../2026-11-06-joint-pair52-capture176/README.md#accepted-proof-baseline-unchanged) remain current. No new proof, AWS, Aeglos, large-circuit, device, pressure or concurrency measurement is added.

[Qualification](artifacts/qualification.json), [pins](artifacts/qualification-pins.json), [design](artifacts/design.md), [source deltas](artifacts/source-deltas.json), [Rust reviewer/code inspector](artifacts/qualify.rs) and [selected sources/receipts](artifacts/source-and-receipts.json.gz) preserve the implementation. The archive excludes executables, libraries, binary fixtures, keys, witnesses and proofs. Full snapshot and binaries remain local with recorded hashes.
