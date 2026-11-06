# Split REDC planes: actual point-caller qualification

The fresh ON core passes **126 unfiltered tests**. The OFF core passes the shared signed-tail/reuse oracle, and matching OFF/ON field, core and driver identities are frozen and verified. The actual consumer code is emitted and inspected. **No caller timing, full-proof gain or production adoption is claimed.**

The [native field qualification](../2026-11-06-split-redc-native176/README.md) and [mathematical/dependency screen](../2026-11-06-split-redc-planes176/README.md) are reused without rerunning field tests or models. The 178-file snapshot copies the qualified native source, changes two core parents only by diagnostic module/export suffixes, and adds the existing caller adapter. Production core arithmetic remains byte-exact. The adapter calls the original G1 stream and signed borrowed G2 routine; no rejected validated-G2 kernel or dispatch is copied. Both eight-lane fields/radices retain the new global candidate scope.

## Actual linked code

Six newly decoded consumers cover G1, G2 and vector inversion in both drivers. Three frozen-control decodes are inherited unchanged. Nominal IFMA counts survive, while additional additions and vector stack operands remain visible. Counts are static instruction locations, not retired work, measured spills, memory peaks or minimum RAM.

| Arm / consumer | Instruction rows | IFMA sites | Vector adds | Vector stack operands | Code bytes | Partial register-chain score |
|---|---:|---:|---:|---:|---:|---:|
| OFF / G1 | 4425 | 800 | 252 | 286 | 26198 | 92 |
| OFF / G2 | 8590 | 1450 | 481 | 603 | 51830 | 114 |
| OFF / inverse | 1139 | 500 | 95 | 67 | 6979 | 92 |
| ON / G1 | 4574 | 800 | 316 | 298 | 27001 | 70 |
| ON / G2 | 8855 | 1450 | 577 | 646 | 53319 | 96 |
| ON / inverse | 1219 | 500 | 135 | 67 | 7457 | 70 |

The partial register-chain score uses the previously checked graph helper byte-for-byte with calibrated weights. It omits unknown/masked/memory/copy/inter-block edges. The lower ON scores may partly reflect extra memory boundaries breaking those graphs; they do **not** prove shorter whole-kernel hardware latency or compiler realization of every ideal source dependency. Actual matched measurements must decide whether the dependence change outweighs added work and stack traffic. No old profile is transferred to these binaries.

## Validation and retained repair

Four initial workflow stages succeeded once: OFF core build/freeze, test build and the shared tail/reuse test (one passed, 125 inherited tests filtered). Driver linking then failed because its inherited search path did not contain the new transitive field library. The failed command and original protocol are retained. A separate eight-stage continuation adds only the matching field search path and explicit field extern to the failed/unexecuted driver builds; all other remaining arguments stay identical. The four successful stages are reused. The continuation succeeds, including the full ON suite of 126 tests with zero filtered. All 12 unique successful stages and the one failed link are reconciled.

A pre-build helper correction gave frozen core libraries valid `libzopro_core-HASH.rlib` names; no preparation or native work was repeated for it. Source inverse edits, actual command arguments, compiled-arm cfg, field/core/driver hashes and the inherited generator oracle are checked. Both caller executables are frozen immediately after compilation. The retained frozen accepted-source comparator is `/home/ubuntu/src/zopro-perf-work/word-export176/caller-frozen`; it is not an SDK or a claim of original field-library binary identity.

## Next measurement

Declare a complete matched caller protocol before clocks: frozen accepted-source control, fresh OFF and ON; both curves; 200/201/352/353 points; 1/16/32 workers; fixed paired blocks with signed/tail/generator/output/capacity/zero-allocation guards. Retain every context and control-calibration failure. No unchanged width/layout/attribute or favorable-subset rescue. Only a positive complete-caller result permits SDK/all-shape/Gnark/resource/portable qualification.

Accepted production remains unchanged: local 1x2 is 7.928660 ms and merge54 is 161.883899 ms. The inherited 132.75 MiB RSS is not a new measurement or minimum RAM. The [all-eight CPU/Metal comparison](../2026-11-06-split-redc-planes176/README.md#unchanged-full-proof-baseline) remains unchanged; the CPU 10× goal is active and unmet on all eight shapes. Proof ledger remains 30,846. There is no new AWS/Aeglos, SDK/Gnark proof, portability, pressure or mobile result.

[Qualification and all static rows](artifacts/qualification.json), [publication review](artifacts/review.json), [design](artifacts/design.md), [qualifier/codegen source](artifacts/qualify.rs), [connected-chain analysis](artifacts/consumer-chains.rs), [reused graph helper](artifacts/graph.rs), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [source deltas](artifacts/source-deltas.json), [original build plan](artifacts/build-plan.json), [continuation plan](artifacts/repair-plan.json), [repair notes](artifacts/repair-field-search.md), [input/receipt pins](artifacts/qualification-pins.json), [source/receipt archive](artifacts/source-and-receipts.json.gz) and [member hashes](artifacts/archive-index.json). The archive includes all six new consumer decodes, full chain output, changed/added text sources and both successful and failed command records; binary/key/witness/proof blobs are not redistributed.
