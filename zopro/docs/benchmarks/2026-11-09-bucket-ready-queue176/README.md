# Bucket-directed collision queue work screen

**The fixed recipe fails its work-count screen and is closed before native benchmarking.** Directly promoting deferred operands for newly freed buckets slightly reduces projective arrivals, but increases additions to existing projective accumulators. No latency or native RAM result is produced; production remains unchanged at its last measured **7.803404 ms** for 1×2 with 32 workers.

The proposed queue replaces a shared newest-first stop rule with per-bucket LIFO heads and intrusive bounded slots. Each original full-batch completion promotes at most one queued operand per freed destination, using the retired batch's storage. Original 20 balanced spans, 200/352 ready capacities, size-minus-one queue bound, overflow spill and final execute/spill remain. It adds no curve formula, transformed key, persistent lookup table or pool. This differs from the previously closed descending scan and guarded affine drain; neither prior trial is repeated.

|Generic primary K/H work|Original saved observation|New model|
|---|---:|---:|
|Signed arrivals|1170733|1170733|
|First affine assignments|70681|70681|
|Ordinary affine updates|1088726|1088835|
|Projective arrivals|11326|11217|
|Projective initializations / normalization buckets|4415|3938|
|Adds to existing projective accumulators|6911|7279|
|Stream inversion stages|3713|3713|
|Queue flushes|52|52|
|Scalar-tail pairs|78|83|

Projective arrivals decrease by 109 (0.96%) and normalization buckets by 477 (10.80%), while projective accumulator additions rise by 368 (5.32%). Inversion stages stay unchanged. These counts do not price normalization, field arithmetic, exceptional points, packing, indexing or cache behavior, and do not establish speedup or slowdown. The declared four-check screen retains all positives but fails the accumulator-add check. No order/size/threshold sweep or favorable-span promotion follows.

The model adds **143,360 logical bytes (140 KiB)** of u16 bucket heads across all 20 actors and **1,085,616 head checks**. Each packed model slot is 16 bytes, with at most 199/351 active slots per actor. Native pointer layout, capacities, allocator overhead, complete admission, stacks, process RSS and minimum RAM remain unqualified. Diagnostic token/oracle vectors are not proposed native storage.

Three focused tests pass: 120 signed boundary/reuse patterns, direct distinct-destination/LIFO promotion and supported-capacity/empty-input edges. One new candidate census uses the retained 30,529 K rows and exact 30,922 H rows. Independent Rust review uses per-bucket vector stacks instead of intrusive slots and a bit-based reader; it reproduces all 20 actor counter/histogram rows, 1,229,020 digits and 71,680 signed bucket sums. This is generic noncancelling occupancy with integer-token conservation, not native curve-exception or proof correctness. The prior baseline, solver and FFT extraction are inherited without replay.

All six serialized Rust1.98.1/xtask commands succeed first execution. The separate reader checks 191 source/input/tool pins and the preceding five receipts; terminal publication checks all six command/stream receipts, including the reader's own. All 176 production-source hashes and the canonical 43,083-proof ledger remain unchanged. Setup/publication helpers are outside that receipt count. There are zero new SDK requests, Gnark exports, performance captures or profiles.

The full CPU10× goal remains active and unmet, **0/8**. The following accepted local 32-worker full-proof observations and supplied M5 references are inherited from the latest unchanged SDK comparison. Cross-machine reference ratios do not establish equal hardware efficiency.

|Shape|Accepted CPU ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.803404|37.0 /4.742×|24.0 /3.076×|3.7|
|2x2|10.940319|59.0 /5.393×|30.0 /2.742×|5.9|
|4x2|15.809849|109.0 /6.894×|48.0 /3.036×|10.9|
|8x2|24.043743|189.0 /7.861×|74.0 /3.078×|18.9|
|12x2|33.720355|284.0 /8.422×|101.0 /2.995×|28.4|
|16x2|39.753010|335.0 /8.427×|124.0 /3.119×|33.5|
|24x2|65.983137|528.0 /8.002×|176.0 /2.667×|52.8|
|merge-24x1|65.378103|507.0 /7.755×|175.0 /2.677×|50.7|

Accepted primary peak RSS remains 132.50 MiB; merge54 remains 162.189200 ms without a supplied M5 reference. No new AWS/Aeglos, phone/ARM, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput evidence is added. Next research should target bulk stream arithmetic or dependencies rather than another collision-queue variant.

[Counts](artifacts/analysis.json), [independent review](artifacts/review-result.json), [decision](artifacts/decision.json), [design](artifacts/design.md), [model](artifacts/model.rs), [review source](artifacts/review.rs), [terminal receipts](artifacts/receipts.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private cache/input/point/scalar/witness/proof bytes, arbitrary token sums, full ledger and executables remain local; reproduction needs the pinned private cache.
