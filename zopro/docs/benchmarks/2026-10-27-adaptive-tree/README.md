# Adaptive G2 tree-layer records

Primary32 retains a **31.530µs paired median saving** against accepted, about **0.37%**, with9/9 winning blocks. Its median ratio is0.996297, interval[0.987348,0.998537]. The original-eight aggregate improves against both original controls, and the24×2 and54→1 gain checks pass. These are useful positive results, with no minimum gain cutoff.

**The global gate fails; no change is adopted.** Control calibration fails at primary16 and4×2, and4×2 nonregression bounds exceed the declared2% limit. CPU10× rows among the eight supplied shapes remain **0/8**. Production is unchanged.

The candidate chooses prefix/norm records for native G2 layers with at least32 ordinary affine pairs and the original kernel for smaller layers. This threshold was fixed by the preceding [training/heldout component](../2026-10-27-tree-record-crossover/README.md), before any current SDK capture. It does not depend on circuit identity, core count or current timing results. The [unconditional prefix candidate](../2026-10-27-g2-tree-records/README.md) remains a separate, frozen comparator with its prior global failure and retained large-shape positives.

One guard is computed after pair selection and controls the existing forward, record-reservation and reverse branches. Field formulas, scalar tails, complete exceptional-pair compaction, tree topology, tile size, point order, query dispatch and scheduling remain unchanged. G1 and portable/no-IFMA paths retain the original kernel. Only `msm.rs` changes; reversing the new edits recovers the prior candidate, whose existing audit recovers accepted source. The other169 originals are exact. There is no new table, plan, pool, point-input copy, field primitive or runtime experiment selector.

## Complete verified request latency

Ryzen9950X. Nine rotated/reversed fresh-process blocks over all nine circuits at32workers plus primary16 on CPUs0–15. Four variants: exact accepted SDK, reused fresh original baseline, reused unconditional prefix candidate, new adaptive candidate. Their source/library/driver/binary pins are checked; no new control builds are claimed. All use the same native flags, standard SDK driver, fat LTO and one code-generation unit.

Every timed call includes witness work, fresh blinded proving, SDK verification and serialization. Loading and three primes are excluded. Each32-worker process has31 timed calls; primary16 has151. All360 latency processes precede four separate warm-gated199Hz primary32 profiles,251 timed calls after3 primes each. There is no short screen, conditional extension, successful capture repeat, threshold revision or case exclusion.

Displayed times are medians of process medians. Ratios pair blocks geometrically; savings pair block medians and may differ from subtracting displayed times. CPU includes coordination; lifetime RSS includes loading/primes and is not minimum RAM.

|Case|Accepted ms|Fresh ms|Unconditional ms|Adaptive ms|Adaptive/accepted|Adaptive/fresh|Adaptive/unconditional|Paired saving vs accepted µs|Adaptive RSS MiB|Adaptive CPU ms/request|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2-w32|8.628200|8.669467|8.539252|8.602090|0.994471|0.989387|1.002802|31.530|113.00|199.044|
|2x2-w32|11.726902|11.792506|11.825017|11.753543|1.004285|0.998314|0.997617|-11.863|268.50|274.205|
|4x2-w32|16.181883|16.228821|16.219163|16.374466|1.007726|0.999578|1.000710|-177.465|320.50|430.956|
|8x2-w32|24.536646|24.766080|24.155899|24.090376|0.978471|0.976447|0.997832|549.056|497.00|668.736|
|12x2-w32|34.263909|34.511827|34.296020|33.923376|0.988636|0.982491|0.991757|572.821|720.25|946.257|
|16x2-w32|40.899741|41.173699|40.370585|40.679926|0.993090|0.988189|1.005511|219.815|816.25|1141.221|
|24x2-w32|68.547968|68.877007|67.421373|67.421261|0.978921|0.976760|0.999684|1518.555|1348.50|1930.814|
|merge-24x1-w32|67.272049|67.610169|66.892252|66.314153|0.982590|0.977763|0.989800|1295.953|1183.50|1842.157|
|merge-54x1-w32|177.333562|178.373996|163.038432|163.004334|0.922263|0.916946|0.996303|14257.565|1823.54|4480.609|
|1x2-w16|10.491323|10.597142|10.376707|10.453992|0.944286|0.969772|0.998211|82.915|89.50|130.827|

Against accepted, the adaptive24×2 paired median ratio is0.977847 and54→1 is0.919568, each with9/9 wins. The8×2 and24→1 merge cases also win9/9. Those gains do not waive the4×2 check.

The adaptive rule does not establish better primary latency than the unconditional comparator: their paired median ratio is1.001242, interval[0.993345,1.012481]. The component crossover remains valid for its measured caller; it did not guarantee an additional complete-proof gain.

## Frozen qualification

The rule uses individual distribution-free median intervals with at least95% coverage under independent/exchangeable block assumptions. For nine blocks, endpoints are the second-smallest and second-largest ratios, nominal coverage96.09375% under those assumptions. This is not a simultaneous/familywise guarantee. All observations remain and determine the order statistics.

Calibration requires fresh/accepted intervals wholly within factor1.03 for every case. Adaptive nonregression requires upper bounds≤1.02 against accepted, fresh and unconditional in every case. Fixed24×2 and54→1 gain checks require upper bounds<1 against accepted and fresh, to preserve the earlier large-shape signal without a shape-based dispatcher. The original-eight within-block geometric aggregate requires an upper bound≤1 against accepted and fresh. This structural change need not strictly improve primary in isolation; primary and every other case must pass nonregression, while the original eventual all-eight CPU10× objective remains intact. Any passing performance result still needs resource/pressure/compatibility review before adoption.

calibration_failures: `["1x2-w16 fresh/accepted interval [0.7920578904276399,1.022446532440535]","4x2-w32 fresh/accepted interval [0.9948462734528482,1.0337200087855256]"]`.

regressions: `["4x2-w32 adaptive/accepted upper=1.0224746726304823","4x2-w32 adaptive/fresh upper=1.0207327561146602"]`.

large_gain_failures: `[]`.

aggregate_failures: `[]`.

The original-eight/accepted aggregate median ratio is0.990347, interval[0.986908,0.994721]; against fresh it is0.985250, interval[0.981833,0.992302]. Both pass their individual checks. For4×2, adaptive/accepted has a median ratio1.010890 and interval[0.996115,1.022475]; its upper bound crosses2%, although the interval also includes improvement. Primary16 candidate nonregression passes; its failed check concerns the original controls' calibration.

[Full analysis](artifacts/analysis.json) retains every block, ratio, interval, saving and failure. Crossing an interval bound indicates insufficient support for the required limit; it is not by itself a causal slowdown measurement. No adverse cell is removed and no gate is relaxed.

## Supplied M5 CPU and Metal comparisons

Historical references supplied by the user, not remeasured. No54reference was supplied. Rows are experimental candidate measurements.

|Shape|Adaptive ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.602090|37|4.301×|24|2.790×|3.7|
|2x2|11.753543|59|5.020×|30|2.552×|5.9|
|4x2|16.374466|109|6.657×|48|2.931×|10.9|
|8x2|24.090376|189|7.845×|74|3.072×|18.9|
|12x2|33.923376|284|8.372×|101|2.977×|28.4|
|16x2|40.679926|335|8.235×|124|3.048×|33.5|
|24x2|67.421261|528|7.831×|176|2.610×|52.8|
|merge-24x1|66.314153|507|7.645×|175|2.639×|50.7|
|merge-54x1|163.004334|—|—|—|—|—|

## Correctness, memory and profiling

All111 native core tests, two raw tests, six cubic tests, generator checks and portable/no-IFMA consumers pass. The existing240-case independent complete-curve layer oracle remains, with its unconditional old-buffer assertion adapted for the new selection. The new test covers31/32/33 ordinary pairs and a growing/shrinking sequence through2048, both inverse modes, every output/count, and retained buffer capacities. Field source and accepted qualification are inherited unchanged.

Both record vectors can retain allocations as layer sizes change. Native G2 below the threshold needs at most three old Group records; larger layers use two-vector prefix records instead of nine-vector Group records. The new test checks the actual combined retained record bytes against the prior maximum exact Group bound through its sequence. This finite coverage is not a universal allocator proof, process-RAM limit or minimum-RAM claim. The additional empty Vec header and struct padding, task metadata, allocation margins and pressure behavior still need formal review before adoption.

Whole-process RSS is not uniformly lower. For54→1, accepted/adaptive peak RSS is1,926.06/1,823.54MiB. For24×2 it is1,249.78/1,348.50MiB, despite the lower record-capacity bound in the tested sequence. These are observed process high-water marks, not an isolated measurement of allocator or scheduler causality. No universal RAM-saving claim follows.

All nine adaptive circuits pass four-worker preflight, one timed call after3 primes:36 SDK calls and nine independent Gnark exports. The matrix adds16,560 SDK calls/360 exports, and profiles add1,016 calls/four exports. Every exported proof passes independent verification,164-byte length, pinned public binding and freshness against7,560 prior identities plus new exports.

Total **17,612 SDK calls**, **373 distinct independently verified exports**, **746 proof/public members**; proof ledger **7,560 → 7,933**. All170 production files and the accepted SDK remain unchanged. The all-eight CPU10× objective remains active and unproven.

Profiles are collected automatically after all latency. Each capture has one sampled process and zero lost records; decoded categories and symbols remain in [rows](artifacts/performance/rows.json). Sampled cycle shares do not give removable wall-time fractions or isolate cache/memory causality. No helper, build, hash, archive or verifier overlaps a timed process. Closing checks rehash all pins and proof members. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, pressure, concurrency or throughput qualification is claimed here.

The four primary32 profiles contain38,193 samples: accepted9,347, fresh9,627, unconditional9,368 and adaptive9,851. They do not profile4×2 and cannot explain that case's interval result.

## Post-capture code-generation audit

The actual adaptive G2 `reduce_layer` symbol occupies81,580bytes, versus53,534 for the unconditional candidate and52,064 for accepted. The runtime guard places both arithmetic branches in that symbol. These are static code sizes, not instruction counts, cache-miss measurements or a causal explanation of latency. [Code-generation audit](artifacts/codegen-audit.json) records all four binary hashes, exact symbols and sizes; inspection occurs after all captures and generates no additional proofs or timing samples.

A distinct next source experiment can keep the same32-pair rule and formulas while separating pair selection/compaction from const-specialized ordinary-pair kernels. That would test implementation of the measured crossover without another threshold sweep, inline annotation, pool change or unchanged-candidate rerun. The current candidate and every positive/negative result remain retained.

accepted executable: `/tmp/zopro-grouping-module/native/candidate-sdk`; SHA-256 `03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d`.

fresh executable: `/tmp/zopro-poseidon-fused-dot/native/fresh-sdk`; SHA-256 `d847ae9fd2ae52c6c74172a95004780eaa5481ca2a5f8889d7c9d0063dcd1eaa`.

tree executable: `/tmp/zopro-g2-tree-records/native/tree-sdk`; SHA-256 `d8dc7ed028c9112f4c20a8b1847c23f27f8593097e12f450bfa5d4d53a22514a`.

adaptive executable: `/tmp/zopro-adaptive-tree/native/adaptive-sdk`; SHA-256 `1074991b86c7ac6709ada47b367732151d0510ea2cd54d300b534660d71dbc19`.

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [edits](artifacts/edits.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
