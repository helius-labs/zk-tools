# Bounded Euler output compaction

The candidate failed the declared caller screen and was not adopted. On captured 1×2 input it is about 11% slower than the old arithmetic at the same 256-row task size, and 26% slower than the accepted caller with one worker. The smaller-task control is faster than the accepted caller at 16/32 workers. That observation concerns this component; no full-proof improvement has been established.

The [preceding census](../2026-10-28-euler-factor-census/README.md) identified exact reductions: when original row values satisfy `A=B`, `DA*B+A*DB=A*(DA+DB)`, preserving unequal derivatives. Zero inputs also eliminate products. This implementation classifies each fixed 256-row tile, gathers packed derivative lanes, evaluates one- or two-product groups, then scatters results. Its two fixed u16 index arrays occupy 1 KiB per active tile; compiler stack and field temporaries are additional. It adds no key table, retained plan or input-sized side buffer. Tiles without an arithmetic-count advantage fall back to the old arithmetic after classification.

The arithmetic savings did not cover classification, gathers and scatter. At 16/32 workers compaction beats the accepted task decomposition, but the old-arithmetic tile control beats compaction. This rejects the tested compaction caller. It does not establish that the underlying algebra can never help. No unchanged gather/layout/tile sweep or SDK integration is justified by this result. A separate audit must establish whether the scheduling observation is distinct from prior task-granularity experiments before any full-proof test.

The actual-32-worker fresh/accepted calibration also fails its declared ±3% interval bound (lower ratio 0.964955). It remains in the report. The negative same-tile comparisons and single-worker regressions independently fail their gates.

| Fixture | Workers | Accepted µs | Fresh µs | Tiled old µs | Compact µs |
|---|---:|---:|---:|---:|---:|
| actual | 1 | 221.790 | 221.125 | 220.575 | 279.724 |
| actual | 16 | 63.173 | 62.806 | 45.499 | 50.461 |
| actual | 32 | 86.277 | 84.905 | 50.353 | 55.907 |
| general | 1 | 222.119 | 221.723 | 220.764 | 267.984 |
| general | 16 | 62.885 | 62.669 | 45.321 | 49.992 |
| general | 32 | 85.897 | 85.602 | 49.648 | 52.708 |

| Fixture | Workers | Paired ratio | Median | Conditional interval | Wins | Check |
|---|---:|---|---:|---:|---:|---|
| actual | 1 | fresh/accepted | 0.996764 | [0.995013, 0.998418] | 22/31 | pass |
| actual | 1 | compact/accepted | 1.259294 | [1.257533, 1.263987] | 0/31 | fail |
| actual | 1 | compact/fresh | 1.265502 | [1.260841, 1.270322] | 0/31 | fail |
| actual | 1 | compact/tiled | 1.267518 | [1.264948, 1.270937] | 0/31 | descriptive |
| actual | 16 | fresh/accepted | 1.003579 | [0.990428, 1.015711] | 15/31 | pass |
| actual | 16 | compact/accepted | 0.797502 | [0.788123, 0.807331] | 31/31 | pass |
| actual | 16 | compact/fresh | 0.795671 | [0.787898, 0.806105] | 31/31 | pass |
| actual | 16 | compact/tiled | 1.109523 | [1.087228, 1.119271] | 3/31 | fail |
| actual | 32 | fresh/accepted | 0.983400 | [0.964955, 1.008854] | 21/31 | fail |
| actual | 32 | compact/accepted | 0.645665 | [0.638407, 0.655959] | 31/31 | pass |
| actual | 32 | compact/fresh | 0.656037 | [0.649922, 0.663193] | 31/31 | pass |
| actual | 32 | compact/tiled | 1.110230 | [1.077754, 1.133067] | 3/31 | fail |
| general | 1 | fresh/accepted | 0.997989 | [0.995793, 0.999608] | 22/31 | pass |
| general | 1 | compact/accepted | 1.208180 | [1.203930, 1.211852] | 0/31 | fail |
| general | 1 | compact/fresh | 1.209920 | [1.206781, 1.212746] | 0/31 | fail |
| general | 1 | compact/tiled | 1.213968 | [1.210183, 1.217375] | 0/31 | descriptive |
| general | 16 | fresh/accepted | 0.993308 | [0.982979, 1.018513] | 19/31 | pass |
| general | 16 | compact/accepted | 0.792909 | [0.784018, 0.812579] | 31/31 | pass |
| general | 16 | compact/fresh | 0.795498 | [0.791726, 0.804348] | 31/31 | pass |
| general | 16 | compact/tiled | 1.090261 | [1.079610, 1.116017] | 1/31 | descriptive |
| general | 32 | fresh/accepted | 0.999389 | [0.978282, 1.020367] | 16/31 | pass |
| general | 32 | compact/accepted | 0.616780 | [0.603026, 0.621287] | 31/31 | pass |
| general | 32 | compact/fresh | 0.613544 | [0.600068, 0.627029] | 31/31 | pass |
| general | 32 | compact/tiled | 1.077116 | [1.037928, 1.095005] | 5/31 | descriptive |


Each entry is the median of 31 process means. Every process has three primes and 256 individually timed output calls; restoring A occurs outside each call timer. The complete output invocation includes dispatch, classification, gathers, arithmetic, scatter and tails. Blocks use a fixed rotated/reversed order. Paired ratios use corresponding process means. The 10th–22nd ordered ratios have nominal 97.055063% coverage under independent exchangeable blocks, without a familywise guarantee. Actual 16/32-worker compaction must beat all three controls with upper ratio below one and at least 21 wins. Actual single-worker and all general-control comparisons against accepted/fresh require upper ratios at most 1.02. No gain floor, exclusions, extensions or parameter sweep were used.

All 744 latency processes completed before four separate 199 Hz cycles:u profiles, each with 16,384 calls after three primes. Profile scope includes input restoration and timing/accounting. All have one sampled PID and zero lost records.

| Workers | Variant | Samples | Lost |
|---|---|---:|---:|
| 1 | fresh | 763 | 0 |
| 1 | compact | 951 | 0 |
| 32 | fresh | 9230 | 0 |
| 32 | compact | 7525 | 0 |


At one worker the largest resolved symbol is the inlined output caller. At 32 workers, epoch pinning and advancement are prominent in both profiles. These samples do not isolate classification/gather instruction costs or establish removable wall-time fractions. No tiled-control profile was declared, so the scheduling result has no matched profile attribution. Detailed demangled symbols, allocation counts, CPU usage and process RSS are in [analysis.json](artifacts/analysis.json). Actual-fixture median process RSS is 11.75 MiB at 1/16 workers and 12 MiB at 32 across all variants; this includes frame/reference buffers and thread setup, not a full prover or minimum RAM. CPU accounting includes restoration; individual-call timers exclude it.

The trusted local frame contains original A/B and exact packed derivative bytes: 4,076,208 bytes, SHA-256 `eb37042ac1435fa232c384fd504bf4c7e16eb088870296864cca5f65a324e78c`. Raw field values stay local. One observed proof after three primes verifies internally and independently with Gnark, public binding and freshness against 12,685 prior identities. The proof ledger is 12,686. The general control deterministically makes A/B nonzero and unequal while retaining derivative data; it is a caller fixture, not a second proof.

Original field function bodies remain byte-identical. The native field suite passes 35 tests, including two new four/eight-lane gather/mask cases covering fields, radix modes, permutations and masks. Caller tests pass every equality mask with unequal derivatives, mixed zeros, partial tiles/B extents, reuse, per-row write conservation and 777 full-width Arkworks outputs; two inherited FIFO tests also pass. Separate real-frame tests pass against accepted and fresh libraries. These establish the tested numerical behavior, not universal platform qualification.

Failures are preserved. The initial parallel core suite passed 114 cases and failed five process-global allocator/FFT resource cases. Those five pass in separate processes with exact filters; successful cases were not repeated. This is per-case recovery, not a passed original parallel suite. A metadata helper rejected an irrelevant duplicate dependency before probe compilation; selecting its six required dependencies repaired linking. The first benchmark process then stopped at a mistyped frame header before loading, priming or timing. Correcting only the reader and adding real-frame tests repaired it. That failed schedule has zero completed timed processes. The final schedule completed all 744 latency and four profile processes once. No successful measurement was repeated. All capture pins match; no owned CPU helper, build, verifier, hash or archive process overlapped captures.

The study timed 190,464 output calls and profiled 65,536 calls. Those calls are not proofs. Production and the accepted SDK are unchanged. No new full-proof, all-eight/54, mobile/TEE, memory-pressure, concurrency or AWS qualification was performed. The original all-eight 10× CPU goal remains active and unmet (0/8). The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its engineering exception and false full-performance/qualified-scope gates.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |


These are inherited comparisons, not component-trial measurements. Accepted primary peak RSS remains 134.50 MiB. Merge 54→1 remains 161.341835 ms with historical 1,910.1875 MiB peak RSS and no supplied M5 reference. The latest AWS comparison predates the accepted FFT source; this study makes no new AWS/Aeglos claim.
