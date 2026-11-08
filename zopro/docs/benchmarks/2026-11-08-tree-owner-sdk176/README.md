# Operation-owned tree option: local SDK pilot

The fixed pilot did not establish a repeatable full-proof gain. The candidate is not adopted, and this implementation is closed. Production remains accepted source176 and SDK `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`.

The earlier local 1×2 observations really were below 8 ms: 7.915095, 7.928660 and 7.995682 ms. A subsequent observation was 8.002770 ms. This pilot measures the **same accepted executable at 8.010122 ms**. The cause of the variation is unconfirmed; these observations do not establish a code regression. Describe accepted local latency as approximately 7.9–8.0 ms, retaining the actual observations rather than replacing them with the fastest result.

## Change and hypothesis

The preceding configuration census counted 1,101 `TREE_VECTOR_INVERSE` reads among 1,893 option lookups for primary32. That census did not establish their CPU cost. The installed Rust1.98.1 standard-library implementation takes `ENV_LOCK.read()` around `getenv`, including missing variables. This motivated testing whether moving the actual option read to an existing operation owner helps. A read lock in source is not evidence that contention dominates the proof.

Only `group_affine_impl`, `reduce_indexed_buckets` and `weighted_affine` change in a private accepted-source snapshot. An operation captures the tree option once and shares its immutable value with existing Rayon jobs. Scratch construction, arithmetic, trees, windows, queries, allocator, key and solver remain unchanged. Changes to the option between operations remain observable; simultaneous changes within one operation now use one captured value. This is not a transactional process-wide configuration interface.

The small `Factory` holds `Option<bool>` and adds no table, key copy, retained solver plan, heap storage or production pool. Its expected one-byte payload is not a measured stack or RAM bound. Private `OnceLock`s latch only the experimental selector and optional preflight trace selector, not the actual arithmetic option across operations. Tracing is absent during performance capture. A production implementation would require removal of the experimental selector and fresh artifact qualification.

## Full-proof pilot

Local AMD Ryzen9 9950X, 32 Rayon workers, CPU affinity0–31. Accepted is the immutable SDK above. OFF and ON use the same new executable, SHA256 `75141c950035da49596dd7872591ab07ffb43190428f73ff1e62f6bc9d1a9696`, with the private selector disabled/enabled. The standard SDK driver and perf support are byte-exact copies of the previous pipeline. Proof timing includes fresh solve, randomized proof, internal verification and serialization; loading and three priming requests are excluded.

| Arm | Median of 31 process means, ms | Maximum process RSS, MiB | Median minor faults/request |
|---|---:|---:|---:|
| Accepted SDK | 8.010122 | 134.000 | 129.954 |
| New executable OFF | 8.000403 | 134.000 | 128.364 |
| New executable ON | 7.995993 | 134.750 | 129.781 |

| Paired ratio | Median | Fixed rank interval | Faster blocks /31 |
|---|---:|---:|---:|
| ON / OFF | 0.998483 | [0.993645, 1.005056] | 19 |
| ON / accepted | 0.997542 | [0.993661, 1.002603] | 19 |
| OFF / accepted | 0.997240 | [0.994353, 1.005088] | 18 |

Each of31 rotated/reversed blocks contains all three arms, with151 measured requests per process. Ratios pair process means within a block; their medians can differ from quotients of the displayed arm medians. Intervals use sorted ratios at zero-based ranks10 and22, with rank15 as median. No nominal confidence level is claimed.

The declared selection requires an upper ratio below1 and at least21/31 faster blocks against **both** controls. All four gain checks fail. Fresh OFF/accepted calibration passes its factor1.03 interval check, RSS passes its relative screen, and diagnostic quality passes. There is no minimum percentage-gain floor. Every block is retained; no exclusion, extension, subset selection or timing rerun. Passing would have selected full all-eight+merge54/primary16 performance/resource qualification before adoption. That extension was not selected.

## Automatic diagnostics

Three separate FIFO-gated PMU panels,31 requests each, run after the latency captures. All four grouped events report100% scheduled time.

| Arm | User cycles/request, million | Instructions/request, million | Cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| Accepted | 812.721 | 1387.687 | 2.716 | 0.897 |
| OFF | 815.133 | 1387.070 | 2.931 | 0.901 |
| ON | 812.601 | 1386.645 | 2.785 | 0.895 |

These are single diagnostic panels per arm, not repeated estimates or a causal explanation of the small timing differences. Summed CPU cycles across workers are not wall-clock latency.

Three additional gated251-request profiles use a16-million user-cycle period and512 mmap pages. Accepted/OFF/ON have12,257/12,193/12,282 samples; unknown1.403%/1.575%/1.783%; zero LOST records. All samples belong to the SDK main thread and its32 Rayon threads on assigned CPUs. FFT leaf samples are present. Separate `tree_options` symbols receive zero flat samples; nonzero sampling of such a small helper was deliberately not a quality requirement. Flat samples do not establish exclusive wall time.

## Correctness, failures and evidence

Native qualification tests immutable option ownership and fresh reads by a subsequent operation. Separate tree-option1/0 processes compare the three actual G1/G2 reductions against generator oracles, including zero, identity, repeated/equal/opposite, ragged and boundary inputs in1/4/16/32-worker512KiB-stack pools. The generic x86-64 configuration compiles; it is not a non-AVX runtime measurement. Unchanged field and prover suites are inherited.

The initial new curve test incorrectly supplied a non-power-of-two bucket count to the existing weighted reduction. Only the test was repaired by padding with identities to the required power of two; the initial ownership test had passed and was inherited. A later SDK link used the default Rust1.99.1 against pinned1.98.1 libraries and failed. The continuation corrected the toolchain environment and linked the already-built libraries; successful tests and library builds were not repeated. Initial failures, original test source, repair source and receipts are retained. Neither failure occurred during timing. Actual SDK symbols and one full disassembly were inspected; selected option-owner bodies are archived. This is not a measured dynamic lookup census or a whole-executable equivalence claim.

Fresh all-nine/primary1/primary16 preflight covers accepted/OFF/ON, plus ON with the actual tree option0:34 exports,136 SDK requests. Preflight traces confirm owner activation and both actual option values. Preflight times are not performance evidence. The pilot adds99 exports and15,186 requests. All133 fresh exported proofs are independently verified by Gnark and checked for image/public/VK bindings and non-reuse against the preceding35,181-proof inventory. The complete canonical inventory is now35,314, privately retained at `/home/ubuntu/src/zopro-perf-work/tree-owner-sdk176/proof-sha256-after-capture.json`.

Independent offline review rechecks source and receipt bindings, all proof unions and paired timing estimates without rerunning requests or verifiers. It inherits native arithmetic qualification and the detailed PMU/profile parser; it does not independently reimplement every diagnostic parser. Native5, preflight68 and capture213 successful receipts are retained, together with the failed native attempts. All captures precede external verifiers and offline decoding. No owned build, helper, hashing, analysis, verification or publication runs during any capture, including preflight.

[Latency and resources](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [protocol](artifacts/protocol.json), [offline review](artifacts/review.json), and [source audit](artifacts/source-audit.json) are directly readable. [Evidence archive](artifacts/evidence.json.gz) is a gzip-compressed JSON map of relative names to UTF8 payloads, lengths and SHA256 hashes, including raw timing streams, PMU CSV, decoded profiles, plans, pins, source fragments and receipts. [Archive index](artifacts/archive-index.json) and [SHA256SUMS](SHA256SUMS) bind the publication. Executables, libraries, raw perf binaries, unchanged full source snapshots, actual keys/images/requests/witnesses/proof/public arrays and full proof ledgers are excluded.

## Accepted comparisons and limits

The qualified all-shape comparison below is inherited, including its primary8.002770-ms observation. The separate fresh accepted primary pilot is8.010122 ms, as reported above. No other shape is timed anew in this pilot. M5 reference values are supplied measurements, not new measurements on this machine.

| Shape | Accepted local CPU, ms | Supplied M5 CPU, ms | Supplied M5 Metal, ms | CPU10× target, ms |
|---|---:|---:|---:|---:|
| 1×2 | 8.002770 | 37 | 24 | 3.7 |
| 2×2 | 11.241463 | 59 | 30 | 5.9 |
| 4×2 | 15.933816 | 109 | 48 | 10.9 |
| 8×2 | 24.234311 | 189 | 74 | 18.9 |
| 12×2 | 33.855565 | 284 | 101 | 28.4 |
| 16×2 | 40.071563 | 335 | 124 | 33.5 |
| 24×2 | 65.917462 | 528 | 176 | 52.8 |
| Merge24×1 | 65.305023 | 507 | 175 | 50.7 |

Inherited merge54×1 is162.240109 ms, without a supplied M5 reference. The verified all-eight CPU10× goal remains active and unmet,0/8. No new AWS, Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or concurrent-proof throughput measurement is added. RSS includes loading and priming and is not minimum required RAM. This result closes this fixed operation-owned option implementation; it does not establish that every configuration access pattern has negligible cost.
