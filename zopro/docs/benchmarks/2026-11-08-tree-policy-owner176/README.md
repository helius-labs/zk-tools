# Tree-operation option snapshot: native qualification

The private candidate passes complete G1/G2 tree-output checks and reads the tree-inverse option once per nonempty operation. Across 272 matched instrumented operations, reads fall **512 → 256**. **No latency improvement is measured and production is unchanged.** These counts do not supply a twofold proof-speed forecast.

The preceding [complete-request census](../2026-11-08-config-lookup-census176/README.md) observed 1,101 tree-vector-inverse reads in one primary32 request, but did not time their cost. This experiment changes lookup ownership rather than arithmetic, window geometry, key storage or worker scheduling. It does not repeat either failed affine-offset implementation or the earlier lookup observer.

## Implementation

`group_affine_impl` and `weighted_affine` read the existing `cpu_option` function at operation entry and pass a copied immutable boolean to their parallel scratch constructors. Each subsequent operation reads again. There is no process-global memoization, new heap buffer, retained plan, key/point copy or production pool. Empty operations avoid the lookup even if Rayon constructs an unused initializer. Scratch fields, point order, arithmetic and result formulas retain the accepted source. Other entry points continue using the original Default constructor.

The policy is fixed during one operation, so all its tasks agree. Changes between operations remain visible. Existing unset, empty and non-UTF8 interpretation is inherited by calling the same option function; the new tests explicitly exercise repeated 0/1 changes. Concurrent changes to configuration during an operation are not sampled by each scratch separately.

The snapshot contains 177 sources: 175 accepted files are byte-exact, one parent is reversible, and one private child contains the owner constructor and tests. Original native field library SHA-256 `1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713` is reused. Native field arithmetic and its qualification are inherited. The unchanged generic field and selector-enabled generic core compile; this is not ARM/mobile execution. Additional runtime state is a copied boolean, but compiler stack behavior and whole-prover resources remain unmeasured.

## Qualification and observation

Each arm checks 8048 serial generator-oracle outputs and 5,760 parallel outputs, plus all 53 actual workers across 1/4/16/32-worker pools requesting 512 KiB stacks. Tests exercise both curves, zero/identity/signed points, group boundaries, immutable inputs, scalar/vector inverse settings, nested parallel calls and configuration changes between operations. Weighted tests pad only invalid tail-tile lengths with identity points to satisfy the internal power-of-two tile contract without changing the intended sum. Grouped cases retain arbitrary lengths. The oracle uses generator coefficients and unchanged scalar multiplication; it is not independent Gnark proof verification.

Read counters and true-result counters exist only in test builds and call the original option function. Their instrumentation can affect task decomposition; these are observed counts, not uninstrumented SDK frequencies, lock time, allocations or CPU cycles. Every nonempty ON operation has one read and every empty ON operation has zero. All observed option values agree with the setting selected before that operation. Raw matched rows remain available in the analysis.

Two test-fixture mistakes are retained. The initial unfiltered OFF suite passes 127 tests and fails the new serial test because it assumed an empty Rayon iterator cannot run an initializer; the observed OFF read count was one. The first focused repair allows that behavior but then reaches a synthetic weighted length outside the reducer's power-of-two tile contract. Its point-oracle failure is retained. A second test-only repair pads invalid tail tiles with identities. Neither repair changes non-test runtime code.

The final failed-case-only OFF run passes, with 127 tests filtered; the previous 127 successful tests are not replayed. ON then passes all 128 tests unfiltered. Initial and intermediate snapshots, stdout/stderr, failed exits and repaired sources remain in the archive. The repaired serial cases preserve between-operation flag checks, while the already successful 2,048-point parallel and 257-point small-stack inputs remain unchanged. OFF's runtime library is reused from the initial snapshot, whose non-test child body matches the final source. ON and generic core use the final snapshot.

There are ten successful workflow commands and two retained failed test commands across the three serialized workflows. The qualification reader checks source/pin recovery, command argv/exits/stream hashes, test populations, every matched lookup row and the test-only repair chain. It does not independently implement standard-library environment handling, Rayon or the field arithmetic. Helper compiler invocations do not have complete workflow receipts. No old successful native suite, proof, profile or decoder is rerun.

## Next measurement and unchanged proof comparison

This selects a separate coherent fresh-OFF/ON SDK build and a prospectively frozen complete-proof comparison. The timer must include fresh solving, blinding, proving, internal verification and serialization, with automatic PMU/profiles and controls. All original eight shapes, merge54 and primary16 belong in the complete qualification; caller lookup counts alone cannot select production adoption. No benchmark clock or profile recording exists for this candidate yet.

Canonical verified-proof inventory remains 36,477, with zero new SDK requests or Gnark exports. Accepted production176/SDK6d83 are unchanged. Historical accepted 1×2 was already 7.915095 ms; private runs reached 7.859756/7.869400 ms. Latest accepted full-proof observations below are inherited from the [full-proof campaign](../2026-11-08-uniform-carry-default-performance183/README.md), not timings of this candidate. Supplied M5 timings use different hardware.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|8.018438|37 / 4.614×|24 / 2.993×|3.7|
|2×2|11.199751|59 / 5.268×|30 / 2.679×|5.9|
|4×2|15.957517|109 / 6.831×|48 / 3.008×|10.9|
|8×2|24.250900|189 / 7.794×|74 / 3.051×|18.9|
|12×2|33.735233|284 / 8.418×|101 / 2.994×|28.4|
|16×2|40.165709|335 / 8.340×|124 / 3.087×|33.5|
|24×2|65.802777|528 / 8.024×|176 / 2.675×|52.8|
|merge24×1|64.926690|507 / 7.809×|175 / 2.695×|50.7|

Merge54 remains 162.442299 ms without a supplied M5 reference. The full verified single-proof CPU AVX 10× objective remains active and unmet, 0/8. No AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM, whole-prover RSS or concurrent-throughput result is added.

[Qualification](artifacts/qualification.json), [matched read counts](artifacts/lookup-analysis.json), [source recovery](artifacts/source-audit.json), [artifact identities](artifacts/identities.json), [source and complete receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
