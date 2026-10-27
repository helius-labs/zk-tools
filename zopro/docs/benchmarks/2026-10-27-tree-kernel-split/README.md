# Specialized ordinary-pair tree kernels

Complete SDK gate passes: **false**. Supplied CPU10× rows: **0/8**. No change is adopted; accepted production remains unchanged. Small reliable gains have no minimum cutoff.

Splitting the kernels does not establish an additional latency improvement over the adaptive parent: every split/parent median interval includes parity. Gains over the original prover remain, including 24×2 and 54→1 and the original-eight aggregate. The overall gate fails control calibration in three cases and the 16×2 nonregression bound against the parent. This closes the function-layout experiment; further work should remove arithmetic operations rather than adjust this split.

The earlier adaptive study's **31.530 µs (0.032 ms, about 0.37%)** primary saving, with nine of nine paired wins, remains a positive result in that study. It is not zero and is not discarded because it is small. Its unresolved qualification checks are retained separately; the new split is a different candidate and measurement.

The candidate preserves the [parent adaptive rule](../2026-10-27-adaptive-tree/README.md): native G2 layers use prefix records from32 actual ordinary pairs, otherwise the original kernel. Pair selection and complete exceptional compaction remain in the caller. Ordinary-pair forward/inverse/reverse/scalar-tail work moves into a private module with a compile-time boolean, yielding separately compiled kernels. The threshold, formulas, topology, tile size, data order, storage and scheduler remain fixed.

There are no inline annotations, new LLVM/ISA flags, key tables, execution plans, pools, point-input copies, runtime experiment flags or circuit/core selectors. Reversing the caller edits recovers the exact parent; arithmetic tokens differ only in predicate spelling and function boundary. The prior audit recovers accepted source. One original file changes and one private module is added;169 originals and all170 production files remain unchanged. Existing111 core tests cover the moved work, so no tests mirroring the factor are added.

## Verified request latency

Ryzen9950X, nine rotated/reversed fresh-process blocks over all nine shapes at32workers plus primary16 on CPUs0–15. Controls are the exact accepted SDK, exact reused fresh original baseline and exact parent adaptive SDK; their source/library/consumer/binary pins are rechecked, with no new control build claimed. The new candidate uses the same compiler/native flags/fat LTO/one code-generation unit/standard SDK driver.

Each32-worker process runs31 timed complete requests after3 primes; primary16 runs151 after3. Timed work includes witness construction, fresh blinded proving, SDK verification and serialization; loading/primes are excluded. All360 latency processes precede eight separate warm199Hz profiles, both primary32 and4×2, all four variants,251 timed calls after3 primes. There is no short screen, extension, threshold revision, exclusion or repeated successful capture.

Times are medians of process medians; ratios are medians of within-block ratios, and savings are medians of within-block differences. These summaries need not equal ratios or differences of displayed times. Lifetime RSS includes loading/primes and is not minimum RAM. CPU includes coordination.

|Case|Accepted ms|Fresh ms|Parent ms|Split ms|Split/accepted|Split/fresh|Split/parent|Paired saving vs accepted µs|Split RSS MiB|Split CPU ms/request|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2-w32|8.636699|8.673857|8.606502|8.601051|1.003789|0.986529|0.998271|-32.281|111.00|198.909|
|2x2-w32|11.715993|11.787568|11.781846|11.805784|1.005812|1.003248|0.998073|-67.950|271.75|274.516|
|4x2-w32|16.108469|16.194141|16.147230|16.222611|0.999186|0.994781|1.004290|13.233|316.00|424.499|
|8x2-w32|24.897283|24.641225|24.240085|24.377798|0.967099|0.987130|1.002980|826.049|511.75|672.453|
|12x2-w32|34.713975|34.785423|34.141535|34.209087|0.989153|0.980836|1.001835|379.626|704.25|946.941|
|16x2-w32|41.309212|40.667285|40.222098|40.496994|0.991411|0.995587|1.005634|354.812|807.50|1136.083|
|24x2-w32|68.458550|68.281446|67.237206|66.936831|0.981399|0.978294|0.996092|1266.576|1329.01|1922.500|
|merge-24x1-w32|67.302685|67.112920|66.069487|66.354462|0.987127|0.989430|1.004861|863.304|1154.31|1846.226|
|merge-54x1-w32|175.768723|176.039642|163.055570|163.703940|0.927104|0.933659|1.002369|12753.121|1822.43|4492.285|
|1x2-w16|10.907017|10.708150|10.548617|10.581158|0.979879|0.991625|0.995254|219.116|88.00|131.044|

For primary32, the split/parent paired median ratio is 0.998271, interval [0.994820, 1.006609]. Against accepted, the split wins four of nine blocks; its paired median saving is −32.281 µs despite the lower displayed median time. For the original-eight within-block geometric aggregate, split/accepted has median 0.991394, interval [0.986534, 0.996239], and split/fresh has median 0.988561, interval [0.985615, 0.992652]. These aggregate gains do not establish an additional benefit from splitting the adaptive kernels.

## Frozen qualification

Individual distribution-free median intervals have at least95% coverage under independent/exchangeable block assumptions. With nine blocks, endpoints2..8 give96.09375% nominal coverage under those assumptions; no simultaneous/familywise guarantee is claimed. All observations remain and determine the ranks.

Fresh/accepted calibration intervals must lie wholly within factor1.03 for every case. Split nonregression upper bounds must be≤1.02 against accepted, fresh and parent in every case. Fixed24×2 and54→1 gain upper bounds must be<1 against accepted/fresh. Original-eight within-block geometric aggregate upper bounds must be≤1 against both original controls. Primary must pass nonregression; the eventual all-eight CPU10× goal is unchanged. Any performance pass still requires resource/pressure/compatibility review before adoption.

calibration_failures: `["16x2-w32 fresh/accepted interval [0.9659229291192061,1.0178575180761136]","1x2-w16 fresh/accepted interval [0.949810293008569,1.0337066600003815]","8x2-w32 fresh/accepted interval [0.9654661302963747,1.019915424903416]"]`.

regressions: `["16x2-w32 split/parent upper=1.024450059972918"]`.

large_gain_failures: `[]`.

aggregate_failures: `[]`.

[Analysis](artifacts/analysis.json) retains every block ratio, interval, estimate and failure. Crossing a bound indicates insufficient support for the required limit and does not by itself establish a causal slowdown. No adverse case is dropped or gate relaxed.

## Supplied M5 references

Historical supplied CPU/Metal values, not fresh Apple measurements. No54reference was supplied. Experimental candidate values follow.

|Shape|Split ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.601051|37|4.302×|24|2.790×|3.7|
|2x2|11.805784|59|4.998×|30|2.541×|5.9|
|4x2|16.222611|109|6.719×|48|2.959×|10.9|
|8x2|24.377798|189|7.753×|74|3.036×|18.9|
|12x2|34.209087|284|8.302×|101|2.952×|28.4|
|16x2|40.496994|335|8.272×|124|3.062×|33.5|
|24x2|66.936831|528|7.888×|176|2.629×|52.8|
|merge-24x1|66.354462|507|7.641×|175|2.637×|50.7|
|merge-54x1|163.703940|—|—|—|—|—|

## Code generation, correctness and memory

Before any SDK capture, the actual split G2 caller is2,531bytes and references both outlined helper instantiations: original49,309bytes and prefix50,576bytes. The parent combined G2 function is81,580bytes. Total code size is larger; an individual selected kernel is smaller. This confirms structure, not fewer dynamic instructions, fewer cache misses or latency causality. The unchanged G1 choice was inlined by the compiler. [Code-generation audit](artifacts/codegen-audit.json), symbol listings and caller disassembly retain addresses, sizes and binary hashes. No annotations force this layout.

All111 native core tests, raw2/cubic6, generator and portable/no-IFMA consumers pass. They include the independent complete-curve tree oracle and the31/32/33 boundary plus growing/shrinking scratch-capacity tests. Formula/scalar-tail/compaction bodies remain unchanged. Field source and accepted qualification are inherited unchanged. No component probe or recorder is linked into SDK proving.

The factor adds no allocations or record storage. Both inherited record buffers can retain capacity; the existing test checks actual combined retained record bytes through its sequence against the original maximum exact Group bound. That finite coverage is not a universal allocator proof or process-RAM bound. Extra metadata, task stacks, allocator retention and pressure behavior still need formal qualification before adoption. Lower code/record bytes do not imply a universal RSS benefit or minimum RAM.

Preflight covers all nine candidate circuits at4workers, one timed call after3 primes:36 SDK calls/nine Gnark exports. Matrix:16,560 SDK calls/360 exports. Eight profiles:2,032 calls/eight exports. Every exported proof passes independent verification,164-byte length, pinned public binding and freshness against7,933 prior identities plus new exports.

Total **18,628 SDK calls**, **377 distinct independently verified exports**, **754 proof/public members**; proof ledger **7,933 → 8,310**. All170 production files and the exact accepted SDK remain unchanged. The original all-eight CPU10× objective remains active and unproven.

Profiles are automatic and follow all latency. Every capture samples one own process with zero lost records. Decoded categories/symbols and resource receipts remain in [rows](artifacts/performance/rows.json). Sampled periods do not give removable wall fractions or establish cache causality. No helper/build/hash/archive/verifier overlaps a timed capture. Closing checks rehash all pins and proof members. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, pressure, concurrency or throughput qualification is claimed.

The eight profiles contain 116,948 samples: 38,529 on primary32 and 78,419 on 4×2. Parent/split G2 period shares are 28.22%/28.53% on primary32 and 18.41%/17.97% on 4×2. These small share changes do not identify a causal speedup or slowdown; no hardware instruction-cache or instruction-count comparison was captured.

accepted executable: `/tmp/zopro-grouping-module/native/candidate-sdk`; SHA-256 `03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d`.

fresh executable: `/tmp/zopro-poseidon-fused-dot/native/fresh-sdk`; SHA-256 `d847ae9fd2ae52c6c74172a95004780eaa5481ca2a5f8889d7c9d0063dcd1eaa`.

parent executable: `/tmp/zopro-adaptive-tree/native/adaptive-sdk`; SHA-256 `1074991b86c7ac6709ada47b367732151d0510ea2cd54d300b534660d71dbc19`.

split executable: `/tmp/zopro-tree-kernel-split/native/split-sdk`; SHA-256 `27acad0463fae3f7a2475ab217d31bb244885d5114aaeb44e8f27a1c05bf1b8e`.

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [edits](artifacts/edits.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
