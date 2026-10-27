# G1 reverse-group overlap: complete SDK requests

Full performance gate: **false**. Supplied CPU10× rows: **0/8**. Accepted production remains unchanged; no source is adopted by this report. Small reliable gains have no minimum cutoff.

The component gain does not establish an end-to-end improvement. Primary32 measures **8.629174 ms**, versus **8.627601 ms** accepted and **8.688925 ms** fresh. Its paired candidate/accepted median is 1.000182, interval [0.981168, 1.009418], with 4/9 wins. The original-eight aggregate intervals also include parity. Two control-calibration checks, three nonregression bounds and both primary-gain/aggregate requirements fail. This implementation is closed without adoption or an unchanged retry.

Positive estimates remain: primary32 saves a paired median **62.769 µs** against fresh, with 6/9 wins, but its interval [0.985210, 1.002591] includes parity. Primary16 and 16×2 show paired savings of 84.369 and 298.403 µs versus accepted, also with intervals spanning parity. The earlier component's 0.817–0.818% gain remains valid in its measured caller. These observations are retained alongside every failed check; small gains are not treated as zero.

The standard SDK changes only the G1 streaming reverse loop to the exact body measured in the [preceding component](../2026-10-27-g1-reverse-overlap/README.md). Two consecutive groups expose the next slope before finishing the current point outputs, preserving the original single inverse chain and descending output order. Forward records, scalar tails, signed/canonical guards, field formulas, storage, capacities, point order, scheduler, G2 and affine trees remain unchanged. There is no diagnostic reference/probe/export in this standard build, second inversion chain, new field helper, table, cache, execution plan, point copy or heap storage. Extra live vectors and code size remain possible register/stack/cache tradeoffs.

The component's targeted signed-batch aggregate improved0.817–0.818%, with7/7 winning block aggregates and all40cases calibrated/nonregressing. That supports this whole-proof study without predicting its gain. Independent111core/raw2/cubic6/generator/portable/no-IFMA results from that exact stream body are pinned and **reused**, not freshly rerun for the standard SDK. All169other original files and all170production files remain unchanged. Inverse source edits recover accepted bytes.

## Verified request latency

Ryzen9950X. Nine rotated/reversed fresh-process blocks, all nine shapes at32workers plus primary16 on CPUs0–15. Three variants: exact accepted SDK, exact reused fresh original baseline and new standard candidate. All use the existing native compiler/runtime configuration and fat-LTO/one-codegen-unit driver; controls are reused with source/library/consumer/binary hashes. Each32-worker process runs31timed requests after3primes; primary16 runs151after3. Timed work includes witness solving, fresh blinding/proving, SDK verification and serialization; loading/primes are excluded. All270latency processes precede six separate profiles. There is no short screen, extension, successful repeat, case exclusion or postcapture source/runtime change.

Times are medians of process medians; paired ratios and savings use within-block observations. Ratios/differences of displayed medians need not match paired estimates. RSS is the maximum lifetime process peak across blocks, including loading/primes; it is not minimum RAM or incremental scratch. CPU includes coordination.

|Case|Accepted ms|Fresh ms|Overlap ms|Paired /accepted|Individual median interval|Wins /accepted|Paired saving µs|Accepted / overlap RSS MiB|Overlap CPU ms/request|
|---|---:|---:|---:|---:|---|---:|---:|---:|---:|
|1x2-w32|8.627601|8.688925|8.629174|1.000182|[0.981168, 1.009418]|4/9|-1.573|110.25 / 111.00|199.414|
|2x2-w32|11.690138|11.739374|11.733970|1.002730|[1.000120, 1.011128]|1/9|-31.850|274.50 / 266.00|272.778|
|4x2-w32|16.148044|16.230559|16.153785|1.005310|[0.983788, 1.006344]|4/9|-87.916|315.00 / 309.50|424.682|
|8x2-w32|24.807932|25.040778|24.981203|1.017711|[0.979337, 1.023174]|4/9|-436.093|516.50 / 519.50|691.496|
|12x2-w32|34.808308|34.763122|34.730652|1.000522|[0.984930, 1.015084]|4/9|-18.134|718.25 / 718.50|965.281|
|16x2-w32|40.928505|40.950975|40.966982|0.992709|[0.984434, 1.015676]|6/9|298.403|840.75 / 849.25|1143.811|
|24x2-w32|68.861844|68.750797|69.298346|1.000615|[0.993339, 1.011734]|4/9|-43.010|1318.86 / 1309.77|1986.012|
|merge-24x1-w32|67.293156|68.054612|67.410162|1.001493|[0.996943, 1.011238]|4/9|-100.480|1155.25 / 1183.00|1866.506|
|merge-54x1-w32|175.217487|176.972746|175.723065|1.007932|[0.986980, 1.010948]|3/9|-1377.929|1896.64 / 1899.10|4839.044|
|1x2-w16|10.609489|10.509091|10.526781|0.992048|[0.950373, 1.039199]|5/9|84.369|90.50 / 89.00|131.375|

## Frozen qualification

Individual distribution-free median intervals have at least95% nominal coverage under independent/exchangeable block assumptions. Nine-block endpoints2..8 have96.09375% coverage under those assumptions; no simultaneous/familywise guarantee is claimed. Fresh/accepted calibration intervals must lie wholly within factor1.03 in all10cases. Candidate nonregression upper bounds must be≤1.02 against accepted/fresh in all10. Primary32 gain upper bounds must be<1 against both controls. Original-eight within-block geometric aggregate upper bounds must be≤1 against both. Primary16 requires nonregression, not strict gain. There is no minimum gain floor or waiver of a failed bound. An unmet bound can indicate insufficient precision and is not by itself proof of a causal regression. Performance success would still require resource/pressure/compatibility review before adoption.

Calibration failures: `["8x2-w32 fresh/accepted interval [0.9898328946159883,1.0366119386057266]","merge-54x1-w32 fresh/accepted interval [0.9926892279551756,1.03301388528976]"]`.

Nonregression failures: `["1x2-w16 overlap/accepted upper=1.039198613338495","1x2-w16 overlap/fresh upper=1.0482560870410769","8x2-w32 overlap/accepted upper=1.0231739939707893"]`.

Primary gain failures: `["1x2-w32 overlap/accepted upper=1.009417610857831","1x2-w32 overlap/fresh upper=1.002590675657351"]`.

Original-eight aggregate failures: `["overlap/accepted original8 upper=1.0062685223151762","overlap/fresh original8 upper=1.0016750692327725"]`.

Original-eight aggregate intervals: `{"overlap/accepted":{"all_values":[1.0073784685379927,1.0048656384569454,0.9916737888092756,1.0041039968134462,1.0009852923278657,0.9980946805644396,1.0049455782948742,1.0062685223151762,0.9967132882860306],"coverage_under_independent_exchangeable_blocks":0.9609375,"lower":0.9967132882860306,"median":1.0041039968134462,"n":9,"rank_one_based":2,"upper":1.0062685223151762},"overlap/fresh":{"all_values":[1.0004496394761029,1.000447581551459,0.9915428210703506,1.0005406827606995,1.0016750692327725,0.9957557073030546,1.0066119915727765,0.9917265595046088,0.9958058922999664],"coverage_under_independent_exchangeable_blocks":0.9609375,"lower":0.9917265595046088,"median":1.000447581551459,"n":9,"rank_one_based":2,"upper":1.0016750692327725}}`.

[Analysis](artifacts/analysis.json) retains every paired ratio, saving, interval, block and failure. Positive signals and adverse cases remain together.

## Supplied M5 references

These CPU/Metal timings were supplied historically and are not fresh Apple measurements. No54→1reference was supplied. Candidate times remain experimental.

|Shape|Overlap ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.629174|37|4.288×|24|2.781×|3.7|
|2x2|11.733970|59|5.028×|30|2.557×|5.9|
|4x2|16.153785|109|6.748×|48|2.971×|10.9|
|8x2|24.981203|189|7.566×|74|2.962×|18.9|
|12x2|34.730652|284|8.177×|101|2.908×|28.4|
|16x2|40.966982|335|8.177×|124|3.027×|33.5|
|24x2|69.298346|528|7.619×|176|2.540×|52.8|
|merge-24x1|67.410162|507|7.521×|175|2.596×|50.7|
|merge-54x1|175.723065|—|—|—|—|—|

## Correctness, resources and profiles

All nine candidate shapes pass a four-worker functional preflight, one request after three primes:36SDKcalls/nine fresh independently Gnark-verified exports. The latency matrix adds12,420calls/270exports. After all latency, primary32 and4×2 profiles run all three variants,251timed requests after3primes:1,524calls/six exports. Total **13,980SDK calls**, **285distinct independently verified exports**, **570proof/public members**; independent proof ledger **8,310→8,595**. Every export has164-byte proof length, pinned public binding and freshness against all prior/new identities. No subinventory is counted twice. SDK verification also runs on every measured request.

The exact component stream body passed independent complete-curve output/reuse checks across signed/canonical/inverse/storage modes, empty/full/odd/even boundaries and tails. These component receipts and unchanged field qualification are inherited; actual standard-SDK correctness is established here by the all-shape preflight and every fresh request/export. No new record fields, record allocations or heap arrays are introduced. Whole-process RSS and CPU remain measured outcomes, not formal stack, allocator or universal RAM guarantees. Phone, memory-pressure, concurrent-caller, giant-circuit and compatibility qualification beyond the pinned component receipts remain unclaimed.

Six warm199Hzcycles:u profiles follow every latency capture; each samples one own process with zero lost records. Decoded categories, symbols and resource receipts are retained in [rows](artifacts/performance/rows.json); raw profiles remain local with hashes. Sampled periods are not removable proof-wall fractions or cache/spill causality. No helper/build/hash/archive/verifier overlaps a timed capture. Closing checks rehash all source/library/tool/helper/control pins and all proof/public members. No fresh AWS/Aeglos, phone, TEE, giant, swap, minimum-RAM, pressure, throughput or concurrency result is claimed.

The six profiles contain **86,524 samples**: 29,042 on primary32 and 57,482 on 4×2. Accepted/candidate G1 period shares are 39.29%/39.58% on primary32 and 34.98%/35.56% on 4×2. These shares do not identify a causal change in G1 cost. A separate static audit **after all captures** finds the actual standard-SDK canonical signed G1 function grows from 26,282 to 38,462 bytes; [ELF hashes and symbols](artifacts/sdk-codegen-audit.json) retain the evidence. No dynamic instruction, spill or cache-miss comparison was measured. Code growth is a tradeoff, not an established explanation for the latency result.

accepted executable: `/tmp/zopro-grouping-module/native/candidate-sdk`; SHA-256 `03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d`.

fresh executable: `/tmp/zopro-poseidon-fused-dot/native/fresh-sdk`; SHA-256 `d847ae9fd2ae52c6c74172a95004780eaa5481ca2a5f8889d7c9d0063dcd1eaa`.

overlap executable: `/tmp/zopro-g1-reverse-sdk/native/overlap-sdk`; SHA-256 `5481bbb43b184578b5b5ff7fc7a5a0f09c3ea913ae469299f3bf24a7baa3ffb7`.

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [edits](artifacts/edits.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
