# G2 index-only gather and compact tree buffers

The component does not pass the frozen gate; stream calibration fails. Production remains unchanged. These are raw-query measurements, not whole-proof or M5 speed evidence.

Primary32 gather takes 3.332390 ms versus fresh stream 3.002297 ms and unchanged prefix tree 3.070977 ms. Within-block ratios are 1.109162 against stream and 1.077549 against prefix, with zero wins in seven blocks against either. Its requested peak is 18.44% below the old tree, but about 2.31 times streaming. Median aggregate CPU per query is 95.56 ms versus prefix 88.64 ms and stream 76.07 ms; median allocation counts are 6727/6727/4543 respectively. Smaller buffers did not reduce allocation counts or total CPU work here.

All primary32 control checks pass. Three primary16 blocks fail the declared complete calibration, so the full qualification remains failed. [Interpretation](artifacts/observations.md) and [CPU/allocation observations](artifacts/observations.json) retain the limits of this conclusion.

The previous prefix tree cloned every signed input point and stored another affine point in each ordinary pair. This private variant gathers the first level directly from the key through signed indices, uses 12-byte index pairs, and alternates two compact, nonaliasing output buffers. It keeps the existing prefix/norm arithmetic, complete-curve exceptions, 256-bucket groups, width12, parts2, phi decomposition, top window and weighting. It changes data movement and allocation shapes, not point-addition counts. No new pool or per-key table. The unchanged prefix tree and fresh stream share the new binary; frozen stream is the exact previous prefix-study binary.

Actual 1x2/2x2 B2 points and fixture scalars, workers1/16/32 pinned to CPU0/CPUs0–15/CPUs0–31. Seven rotated/reversed fresh-process blocks, 17 timed requests after2 primes and1 reference setup per process. Timers include scalar clone, admission, decomposition, routing, allocations, all reductions and merges, result normalization/equality and temporary drops. They exclude key load, witness solving, reference setup, scalar grouping, other queries, blinding, SDK verification and serialization. Both drivers keep allocator-accounting atomics.

Times below are medians of process medians; ratios are geometric means of within-block ratios.

|Case|Frozen ms|Fresh stream ms|Prefix ms|Gather ms|Gather/stream|Gather/frozen|Gather/prefix|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2-w1|40.997694|40.954052|44.608501|50.514695|1.233821|1.232260|1.132604|
|1x2-w16|3.658295|3.718710|3.856620|4.170913|1.069682|1.123473|1.009739|
|1x2-w32|3.008389|3.002297|3.070977|3.332390|1.109162|1.110160|1.077549|
|2x2-w1|65.758034|65.737943|68.647219|78.421018|1.193771|1.192387|1.141865|
|2x2-w16|6.038991|6.011505|5.859241|6.412705|1.089452|1.087231|1.150430|
|2x2-w32|4.860414|4.861606|4.841570|5.272713|1.080078|1.079950|1.083141|

|Case|Stream/prefix/gather requested peak MiB|Stream/prefix/gather RSS MiB|Gather forecast MiB|
|---|---:|---:|---:|
|1x2-w1|1.556/2.199/2.048|60.75/60.75/60.75|87.67|
|1x2-w16|7.639/16.755/14.409|63.75/75.25/74.25|107.12|
|1x2-w32|8.856/25.120/20.487|64.75/84.20/79.15|127.87|
|2x2-w1|2.252/3.283/2.984|94.53/94.53/94.53|131.73|
|2x2-w16|8.652/22.953/18.549|100.08/106.42/104.23|151.18|
|2x2-w32|9.807/35.836/26.828|99.89/122.38/114.50|171.93|

Requested bytes are above prepared inputs, with scalar clone included; RSS includes load/reference/primes. Neither is minimum RAM. Optional admission conservatively includes both compact buffers, index pairs, prefix records/tails, vector/stack/allocation margins and all22 suspended parents above the original raw-query floor. Denial is an explicit error before candidate scalar mutation. No universal memory cap.

Fresh/frozen calibration requires factor1.05 per block and1.03 paired median for all6 cases. Primary32 gather must improve against both streams and unchanged prefix, with4/7 wins against each. 2x32 must stay within1.02 against both streams. Other core counts are descriptive. Any calibration failure withholds fine speed qualification. No gain floor, exclusions, successful rerolls or posthoc core selection.

calibration failures:

- 1x2/w16/b1 stream/frozen0.6122420283221998
- 1x2/w16/b5 stream/frozen1.5947343687993716
- 1x2/w16/b6 stream/frozen1.4017200776479914

failures:

- 1x2/w32 gather_stream ratio1.1091619137624193 wins0/7
- 1x2/w32 gather_frozen ratio1.1101604664927498 wins0/7
- 1x2/w32 gather_prefix ratio1.0775492609747026 wins0/7
- 2x2/w32 gather_stream ratio1.0800775005953436
- 2x2/w32 gather_frozen ratio1.0799502901341382

Before timing, the first core run passed121/122: the capacity-bound test incorrectly rounded an even count up by one. Only that test calculation changed to div_ceil; the kernel and forecast did not change. The failed run and sources are preserved in pre-capture-v1. The corrected122 native core tests, raw2/cubic6 and generator checks pass. Field source is unchanged; the pinned33-test field qualification is inherited. New tests cover signed inputs, identity/doubling/cancellation, empty/odd/skewed buckets, the256-group boundary, independent generator sums, actual30089-point queries and checked geometry/overflow. Every timed output matches its stream reference; points agree with the compressed key and requested peaks fit the typed envelope plus scalar clone. Production170 sources are checked unchanged.

All168 latency processes finish before three primary32 profiles at199Hz,64timed+2primes+1reference each, gated after input preparation. perf uses --no-buildid-cache. These are descriptive user-cycle samples, not critical-path fractions.

|Mode|Samples|Lost|Sampled PIDs|
|---|---:|---:|---:|
|stream|2906|0|1|
|prefix|1684|0|1|
|gather|1727|0|1|

Total3561 raw-query calls includes171 reference setups. No new SDK/Gnark proofs; prior6562 exports unchanged. [Latest whole-SDK local report](../2026-10-26-g1-quarter-current/README.md) retains all supplied CPU/Metal references; [latest C7a comparison](../2026-10-26-g2-tree-sdk/README.md) covers Aeglos. This component does not qualify full proofs, all circuits, AWS, phones, TEE, giant circuits, swap, memory pressure or throughput. No adoption; the original all-eight CPU10× goal remains active and unmet.

All helper binaries/builds/hashes complete before timing or after all latency/profiles. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [jobs](artifacts/jobs.json), [artifact index](artifacts/artifact-index.json) preserve all results and sources. Raw profiles/binaries/large inputs remain local with hashes.
