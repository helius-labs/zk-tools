# SIMD coordinates retained across binary-tree levels

The complete-tree callers improve, but the full prospective gate fails and this implementation is not adopted. At 32 workers, the equal-log aggregate over 256 and 1,024 points is **2.55% faster for G1 and 7.35% faster for G2 against fresh OFF**. Both aggregates win all 15 blocks against both controls. All individual ON/control latency bounds are below parity. Nine process-RSS guards and one solo control calibration fail; those failures remain in the result. This is a component improvement, not a verified proof-latency improvement or the requested 10× result.

The candidate packs the input once and retains intermediate native SIMD coordinates across binary-tree levels. A two-input lane permutation exposes adjacent pairs without converting points to canonical host words. Existing prefix/norm inversion and affine formulas produce each level into a second reusable buffer. The final eight points convert back and use the existing complete reducer. Identity inputs, equal-X pairs, non-power-of-two lengths and short inputs request the original complete path, preserving the immutable input even after earlier candidate levels have finished. The benchmark measures initial packing, all buffer operations, inversions, the final reduction and output.

No field arithmetic primitive or representation, scalar algorithm, key, solver, persistent table, retained key cache, execution plan or production dispatch changes. New field helpers move lanes and test normalized values below 2p for the encodings 0/p; existing arithmetic range invariants are required. The new helper source is separately owned and the existing generator includes it under a private native configuration. Generator consistency passes. The earlier packed tree-store, scalar/SIMD overlap and host-prefix-only results remain separate; this candidate changes the representation across levels. The general dependency-graph approach in the [SQIsign authors’ paper](https://arxiv.org/abs/2608.13948v2) was reviewed previously; no SQIsign implementation, modulus assumption or claimed speedup is transferred here.

Fifteen fixed rotated/reversed blocks compare a separate original build with OFF and ON in one candidate executable. Final links are release/native CPU/fat LTO/one codegen unit. Each process owns one fixture per configured worker, performs three eight-round primes, then 4,096 timed complete reductions per fixture through Rayon. Input slices alternate between two independently known generator sums; black-box inputs and outputs prevent hoisting. Both final outputs of every prime and timed chain are checked outside timing. This is not a check of every intermediate iteration. Timed allocations are zero; capacities remain stable and every measured ON call follows the ordinary packed route without fallback. Exceptional fallback is covered by native tests.

Ryzen 9950X, 16 physical cores / 32 logical CPUs, shared host. Workers bind to CPU 2, CPUs 0–15 or CPUs 0–31 for 1/16/32 workers with 512 KiB stacks. Fixtures are Rayon tasks; no per-round barrier or guaranteed fixture-to-core assignment is implied. Original source 176 is recovered exactly from both 178-file private snapshots after removing five parent additions. Production remains unchanged.

The declared 32-worker two-size aggregate must have upper ratio below one and at least 10/15 wins against both controls, separately for each curve. Every individual ON/control upper bound must be at most 1.03; OFF/original intervals must fit factor 1.03; every maximum ON/control process RSS ratio must be at most 1.05. Bounds are ranks 4–12 of 15, nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise coverage. There is no gain floor, excluded row, extension, favorable subset, threshold rescue or unchanged reroll. Before any caller capture, the first plan’s 128 timed rounds were increased to 4,096 based on prior threaded calibration dispersion. Both declarations/helper versions are retained; algorithms, cases, blocks and gates did not change.

|Curve / 32-worker aggregate|ON/control ratio [bounds]|Wins|
|---|---:|---:|
|G1 / frozen|0.967162521 [0.961318446, 0.978367808]|15/15|
|G1 / OFF|0.974547186 [0.966810865, 0.981247702]|15/15|
|G2 / frozen|0.922083642 [0.915448310, 0.927587082]|15/15|
|G2 / OFF|0.926465564 [0.921448959, 0.929327658]|15/15|

|Curve/points/workers|Original / OFF / ON µs per tree wave|ON/OFF ratio [bounds],wins|ON/original ratio [bounds],wins|
|---|---:|---:|---:|
|G1-256-w1|11.285 / 11.244 / 10.939|0.973204 [0.960318, 0.984313],15/15|0.967430 [0.959777, 0.979116],15/15|
|G1-256-w16|12.417 / 12.364 / 12.027|0.971967 [0.969871, 0.976418],15/15|0.968900 [0.964267, 0.971467],15/15|
|G1-256-w32|17.291 / 17.150 / 16.660|0.970627 [0.964195, 0.977940],14/15|0.962162 [0.952064, 0.968697],15/15|
|G1-1024-w1|29.997 / 29.783 / 28.759|0.964447 [0.960269, 0.970304],15/15|0.958197 [0.953632, 0.965258],15/15|
|G1-1024-w16|33.860 / 33.538 / 32.337|0.965127 [0.961480, 0.969248],15/15|0.956499 [0.953105, 0.959326],15/15|
|G1-1024-w32|47.861 / 47.720 / 46.636|0.978285 [0.970689, 0.987954],15/15|0.976229 [0.969757, 0.979330],15/15|
|G2-256-w1|19.141 / 19.555 / 17.931|0.915977 [0.908367, 0.922900],15/15|0.933439 [0.927081, 0.942698],15/15|
|G2-256-w16|21.733 / 21.822 / 20.382|0.936489 [0.924575, 0.938079],15/15|0.939040 [0.927521, 0.940613],15/15|
|G2-256-w32|31.485 / 31.263 / 29.243|0.935386 [0.926345, 0.944670],15/15|0.926546 [0.920579, 0.933372],15/15|
|G2-1024-w1|57.287 / 58.828 / 52.707|0.896166 [0.890033, 0.901951],15/15|0.919509 [0.913152, 0.925688],15/15|
|G2-1024-w16|65.474 / 66.678 / 60.643|0.909402 [0.902355, 0.914893],15/15|0.926071 [0.923725, 0.926765],15/15|
|G2-1024-w32|100.159 / 99.619 / 91.799|0.918907 [0.912260, 0.924815],15/15|0.917643 [0.910777, 0.924952],15/15|

The sole calibration failure is G2/1024/one-worker OFF/original: 1.025468822 [1.019061541, 1.032135248], whose upper bound exceeds 1.03. All 32-worker calibrations pass. Full gate: false, with nine RSS failures and this calibration failure; all latency gain and nonregression checks pass. The gain and the failed guards remain distinct facts.

The [source-level model](artifacts/work-model.json) reconciles every reported ordinary retained heap capacity. Coordinate new/get calls fall 64.5–66.1% in G1 and 72.7–74.5% in G2. These counts exclude inverse bundles and scalar tails; they are not cycles, physical cache traffic or a latency prediction. The original G2 path already uses prefix/norm records at 32 or more pairs; this comparison does not claim to remove a product at every G2 level.

|Curve / points|Original / candidate retained scratch bytes per fixture|Change bytes|
|---|---:|---:|
|G1-256|43008 / 40992|-2016|
|G1-1024|172032 / 163872|-8160|
|G2-256|49792 / 71712|21920|
|G2-1024|181888 / 286752|104864|

At 1,024 G2 points, 181,888→286,752 bytes is approximately 177.6→280.0 KiB, an increase of 104,864 bytes per fixture. These are retained vector capacities after ordinary calls, excluding inputs, inline/stack storage, allocation size classes and rare fallback retaining additional buffers. They are not a complete optional-resource forecast or minimum RAM. At 32 workers, maximum whole-process RSS rises 7.75→14.50 MiB against OFF in that context. A bounded temporary allocation still has a real footprint; no memory-pressure or universal-device qualification follows from the local speedup. Raw CPU service, process RSS, minor faults and context switches remain in every caller receipt.

Native qualification passes 36 field and 128 core tests, including the new all-field/radix lane/zero-mask test and three complete-tree tests. They cover 0/1/7/8/9/15/16/17/31/32/33/63/64/127/128/256/1024 points, identities, doubling, inverses, intermediate cancellation, input immutability, scratch reuse and 1/4/16/32-worker 512 KiB pools. All suites finish once without a build/test failure or rerun. No fresh portable, resource-pressure, SDK or Gnark qualification is claimed.

All 540 latency processes finish before six grouped PMU panels and six fixed 16M-user-cycle profiles. PMU running time is 100%. Independent review validates 11,519 samples, expected PID/TID/CPU membership, event configuration and zero lost records; each profile exceeds the declared 400-sample floor and has less than 5% unknown symbols. The smallest profile has 596 samples; the largest unknown fraction is 115/3120=3.69%. These small sample sets do not provide precise per-function causal shares or absolute cycle coverage. Static counts include inlining and code outside a single hot path; RSP operands are not dynamic spill counts.

|32-worker counter context|OFF / ON instructions per tree wave|OFF / ON cycles per wave|OFF / ON cache-miss events per wave|
|---|---:|---:|---:|
|G1-256-w32|5062133 / 4676536|2454700 / 2393241|40.20 / 42.03|
|G2-1024-w32|23596291 / 21076999|12893536 / 12261548|2340.46 / 12445.49|

Instructions decrease about 7.6% for G1 and 10.7% for G2 in these panels. G2’s cache-miss event count increases about 5.3× despite its lower instructions/cycles. This supports examining the extra buffer and working set; it does not establish a cache level, DRAM limit or a causal wall-time cost.

All 609 workflow commands succeed, including 588 caller processes and eighteen offline decoders. The planned access repair changes ownership of the six task-owned perf files after capture; before/after content hashes match. No native test suite or timed capture is repeated. 391 source/compiler/executable/log pins bracket capture. The independent reader recovers 352 original source files, checks all command exits and stream hashes, recomputes every paired decision and aggregate, and qualifies counters/profiles. Raw perf.data, executables, libraries and private key/witness inputs remain local. Four archives are byte-roundtrip verified.

The table below retains the previously qualified accepted local proof results and supplied M5 CPU/Metal references. This candidate generated zero proofs and leaves the independent proof ledger at 27,851. These are not candidate full-proof results. No fresh AWS, Aeglos, phone, cold-start, huge-circuit or concurrent-proof comparison is added.

|Shape|Accepted CPU ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|10× CPU target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37|4.667×|24|3.027×|3.7|
|2x2|11.151932|59|5.291×|30|2.690×|5.9|
|4x2|16.086386|109|6.776×|48|2.984×|10.9|
|8x2|24.179453|189|7.817×|74|3.060×|18.9|
|12x2|33.884455|284|8.381×|101|2.981×|28.4|
|16x2|40.077381|335|8.359×|124|3.094×|33.5|
|24x2|65.850777|528|8.018×|176|2.673×|52.8|
|merge-24x1|65.060937|507|7.793×|175|2.690×|50.7|

Accepted primary remains 7.928660 ms / 132.75 MiB peak RSS. Merge54 remains 161.883899 ms / 1876.96 MiB, without a supplied M5 reference. The original CPU 10× goal is unmet: 0/8 shapes meet it. [Accepted qualification](../2026-10-30-g2-vector-sign-adoption176/README.md), including its physical-16 calibration exception, remains authoritative.

The positive caller results justify a distinct memory-reducing follow-up, not adoption or a rerun of this two-buffer implementation. A prospective in-place design could recover all inverse factors before writing outputs, then compact forward because each write precedes unread pairs. Compact native coordinate storage may further reduce capacity; its extra passes/codecs must be charged and measured, not assumed free. Any complete-query integration also needs bounded admission and correct exception fallback. The current gate stays false.

[Final protocol](artifacts/declaration.json), [first pre-capture declaration](artifacts/declaration-v1.json), [all latency/CPU/RSS rows and caller receipts](artifacts/latency-analysis.json), [independent review and profile bins](artifacts/review.json), [work model](artifacts/work-model.json), [workflow](artifacts/plan.json), [source/compiler pins](artifacts/pins.json), [original pins](artifacts/original-pins.json), [exact source snapshots](artifacts/source.tar.gz), [Rust driver/helpers](artifacts/workflow.tar.gz), [raw receipts/build logs](artifacts/receipts.tar.gz), [counter files](artifacts/counters.tar.gz), [host](artifacts/host.txt).
