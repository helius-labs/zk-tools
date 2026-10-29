# GLS4 deferred-point census

All **384 feed records** pass independent flow checks. The diagnostic covers four complete primary SDK requests (three primes and one observed request); the final fresh export independently verifies with Gnark. Counts match across all four requests: **true**. This instrumented run establishes no latency or speedup. The [parent GLS4 implementation](../2026-10-29-g2-gls4-horner175/README.md) remains rejected.

## Observed request

The displayed fourth request follows the declared SDK boundary. All four requests are retained in [analysis](artifacts/analysis.json) and [raw structured records](artifacts/records.json). Each window row sums four partitions and four component passes. Distinct destinations are the minimum/maximum per partition/component; they are not one combined set.

| Window (low to high) | Nonzero arrivals | Affine batch entries | Queue-flush entries | Projective initializations | Existing projective updates | Distinct destinations | Max absolute digit |
|---|---:|---:|---:|---:|---:|---|---:|
| 0 | 113343 | 106796 | 2451 | 605 | 1846 | 1020–1024 | 1024 |
| 1 | 112745 | 107089 | 1560 | 397 | 1163 | 1021–1024 | 1024 |
| 2 | 112751 | 106736 | 1919 | 601 | 1318 | 1019–1024 | 1024 |
| 3 | 112773 | 106743 | 1934 | 621 | 1313 | 1018–1024 | 1024 |
| 4 | 112748 | 106966 | 1686 | 443 | 1243 | 1020–1024 | 1024 |
| 5 | 112552 | 70001 | 40530 | 1688 | 38842 | 222–475 | 544 |

The highest window contributes **38842/45725 (84.947%)** of existing projective updates in these feeds. The census also records equal-point deferrals, opposite cancellations, initializer counts and batch sizes separately. An initializer stores a projective point; an existing update uses the original mixed-add operation. Final normalization/combining, weighted reductions and transforms are outside these feed counters. Counts cannot isolate cycle cost or explain the entire parent latency regression.

This supports testing a hybrid algorithm: retain Horner accumulation for the lower five windows and use indexed affine reductions for the highest window's four components. The accepted two-component path already handles its narrow top separately. Such a hybrid would change the reduction algorithm and needs a new design, admission calculation, correctness tests and full-proof comparison. The counters do not establish that it will beat the accepted implementation; the six ordinary-window version remains closed.

## Observer fidelity and scope

The observer wraps original add/execute/flush methods and follows the original LIFO drain order. It reads a fixed seen bitmap to count repeated flush destinations before calling the original flush once. Scoped thread-local context identifies nested worker jobs. Each record checks input/skip flow, direct/queued flow, queue pop/flush conservation, insertion cases, executed affine entries and projective initialization/update totals. The analyzer independently repeats the checks, verifies all query/window/partition/component tuples and bound checks, and checks batch histograms.

Two fresh observer tests pass; 128 existing tests are filtered out. The tests compare exact affine/projective state with original feeds through all four map passes and check an independently specified doubling/cancellation/queue example and its final generator sum. Parent 128 core, eight prover and 33 field tests are inherited, with inverse edits recovering all 177 parent files and all 17 field files exact. The new snapshot contains 178 files; 176 parent files remain byte-identical. Production source 175 remains untouched. No new field, queue, decomposition, width/partition or input algorithm is selected here.

The standard SDK driver retains raw elapsed/usage output. Those numbers include counters and synchronized output and are excluded from performance conclusions. There is no new PMU/profile capture, unchanged latency rerun, all-shape candidate performance, AWS/Aeglos, phone, pressure, giant-circuit or minimum-RAM claim.

## Inherited CPU and Metal comparison

These are the accepted source-175 timings, not new measurements. The all-eight CPU 10× objective remains active and unmet, 0/8.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | CPU 10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.022165 | 37 | 4.612× | 24 | 2.992× | 3.7 |
| 2x2 | 11.335969 | 59 | 5.205× | 30 | 2.646× | 5.9 |
| 4x2 | 15.945544 | 109 | 6.836× | 48 | 3.010× | 10.9 |
| 8x2 | 24.090485 | 189 | 7.845× | 74 | 3.072× | 18.9 |
| 12x2 | 33.844855 | 284 | 8.391× | 101 | 2.984× | 28.4 |
| 16x2 | 39.931557 | 335 | 8.389× | 124 | 3.105× | 33.5 |
| 24x2 | 66.233309 | 528 | 7.972× | 176 | 2.657× | 52.8 |
| merge-24x1 | 65.273224 | 507 | 7.767× | 175 | 2.681× | 50.7 |

Accepted primary remains 8.022165 ms / 134 MiB peak RSS; merge54 remains 162.250464 ms with no supplied M5 reference. The accepted physical-16 baseline-calibration engineering exception is retained. This census does not qualify a product change.

## Provenance

Four SDK requests verify internally; one fresh 164-byte export independently passes Gnark with exact image/public binding and freshness against 17,551 prior identities. The ledger is 17,552; two archived proof/public members represent one proof. Source, image, library/driver/helper and observation pins close. Native binaries/rlibs remain local with hashes. [Design](artifacts/design.md), [observer](artifacts/source/crates/zopro-core/src/msm_affine/g2_borrowed/gls4/census.rs), [source audit](artifacts/source-audit.json), [new tests](artifacts/native/census-tests/tests.json), [workflow](artifacts/runs/result.json), [proof inventory](artifacts/proof-inventory.json).

Publication checks retain two whitespace findings byte-for-byte: the test runner's final blank line and an inherited source manifest's final blank line. Neither is newly authored source. The report interpretation was added after generation; captured data and pinned implementation files are unchanged.
