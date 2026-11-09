# Original-order G2 query with a short forward multiplier

The full query **fails its declared performance gate** and is not adopted. At 32 workers, the primary paired candidate/fresh-control estimate is **0.977754 [0.956035, 1.000362]**, with 22/31 wins: a 2.22% positive estimate whose interval narrowly includes parity. Against the original-field control it is **1.004650 [0.989710, 1.022035]**, also 22/31 wins, without an established gain. Seven checks fail, including three control calibrations. Correctness, all RSS checks and diagnostics pass.

The [preceding transform's 12.39% gain](../2026-11-09-negative-phi-caller176/README.md) remains valid for that endpoint. It does not establish a query or proof gain. This exact query recipe closes without another exception, unchanged rerun, threshold change, favorable subset or SDK trial. Production and accepted proof performance remain unchanged.

## Implementation and qualification

The private candidate replaces only three high-affine β transforms: indexed leading buckets, owned windows and borrowed windows. It reuses the exact qualified [short-negative forward-map field library](../2026-11-09-negative-phi-short176/README.md), without rebuilding or retesting it. The map uses `β = −(β²+1) mod p`, a 191-bit factor, signed residual and zero-lane correction; its standalone caller had 72 IFMA sites versus ordinary SIMD's 100. Those are inherited map counts, not a new count of complete-query machine instructions.

Original scalar splitting, component/point order, projective β spill maps, width 12/two partitions, grouping/admission, queues, tails, weighting and final result remain unchanged. There is no component swap or final query correction. This differs from the [closed inverse-oriented query](../2026-11-09-inverse-phi-query176/README.md). No new input-sized array, retained key/witness/point/scalar copy, per-key table/plan, production pool, artificial memory cap or global field dispatch is introduced. The reused field's factor/quotient occupy 64 logical bytes; emitted constants and compiler stack are additional.

The snapshot has 179 members: 174 accepted originals remain byte-exact, two original parents have reversible appended declarations, and three children contain the query, borrowed windows and tests. Original functions remain exact; a probe selects original or candidate without switching the production entry. Prospective source recovery independently reverses the cloned-prefix edits and checks all three new affine calls, original scalar split, projective β maps and absence of final correction.

Three focused new native tests pass, with 125 unchanged core tests filtered and inherited. They cover 40 raw/owned cases through the 65,536 cutoff; 21 grouping/admission/fallback cases; and 106 nested queries on all 53 actual Rayon workers of 1/4/16/32-worker pools with 512 KiB requested stacks. Independent Arkworks generator/scalar arithmetic checks complete outputs, including identities, repeated/opposite points, full-width and short scalars, every ordinary/top path and immutable inputs. The expected length-guard assertion is caught by a passing test; its stderr is retained. The generic no-ASM/no-IFMA guard compiles, without a mobile-runtime claim.

All five native steps, two actual consumer links and three controller/reader builds finish successfully on their first executions. Unchanged field/model/old suite qualification is inherited. Copied harness receipt labels were corrected before linking/freezing. An initial patch-format tool rejection occurred before any edit/build; no scientific command, arithmetic execution or test failed.

## Complete-query comparison

Ryzen 9950X, 16 physical/32 logical CPUs; Rust 1.98.1, native O3, fat LTO, one codegen unit and debug information. REF links the original field and original query through the new bridge. OFF and ON share one fresh binary/new field/core; OFF selects the exact original query, ON the new query. Source identity does not imply byte-identical executables or identical generated code. The control discrepancy below has no established cause.

Saved B-query fixtures contain 30,089 points for 1×2 and 52,274 for 2×2. They bind to existing independent complete Arkworks MSM receipts with subgroup, on-curve and canonical-scalar checks. Original solves/oracles are not rerun. Every measured output matches the saved independent oracle; key points remain unchanged.

One global Rayon pool is initialized before requests, using default worker stacks. Workers pin to CPU 2 solo or their index in CPUs 0–15/0–31. Before/after task counts are N+1 and final worker CPUs agree. This conditional placement is common to all arms, not an adopted production default or a concurrent-proof test.

The timer includes scalar reset into reused input, grouping/sample/admission/point sums, original splitting, allocation/drop, queues, ordinary/indexed-top/weighted processing, forward maps and affine normalization. Loading, pool startup, three primes, independent output checks and one postcheck query are excluded. No query output cache is used. The metric is total integer elapsed divided by requests in each process; displayed values are medians of process means.

The primary 1×2/32-worker case has 31 fixed rotated/reversed matched blocks of 151 requests per process. Other cases have 15 blocks of 51 requests, except solo with 11. All 18 preflights pass, followed by 318 latency processes, nine PMU panels and nine profiles. **354 callers perform 28,140 complete queries**, including primes/postchecks. All recording precedes 45 offline steps, for 399 capture/offline commands. Only actual live handle 26942 is polled through terminal exit 0; no owned build/helper/hash/decoder/analyzer/verifier/report/archive/push overlaps recording.

|Shape/workers|REF ms|OFF ms|ON ms|Paired ON/OFF [95% interval]|Wins|
|---|---:|---:|---:|---:|---:|
|1×2 / 1|37.030809|37.575719|36.910678|0.982088 [0.979047, 0.985035]|15/15|
|1×2 / 16|3.546379|3.536148|3.469359|0.973647 [0.952387, 0.993984]|13/15|
|1×2 / 32|3.143465|3.160846|3.102894|0.977754 [0.956035, 1.000362]|22/31|
|2×2 / 1|39.826971|40.176110|39.372210|0.981360 [0.978560, 0.984508]|15/15|
|2×2 / 16|4.270588|4.093911|4.011871|0.982196 [0.962443, 1.003783]|12/15|
|2×2 / 32|3.640051|3.696421|3.628760|0.960434 [0.925740, 0.985439]|14/15|

Paired geometric means need not divide marginal medians. Thus the primary ON median can be lower than REF while its paired geometric estimate is above one. All blocks, including slower candidate blocks, are retained. The paired-log bootstrap uses 20,000 resamples, seed `0x551c36a48ee2d971`, sorted ranks 499/19,499, exchangeable independent blocks and no familywise coverage claim. No minimum percentage gain is required.

All six within-binary ON/OFF point estimates improve. Solo improvements and the physical-16 primary comparison pass their individual gain interpretation, as does 2×2/32 against OFF. The equal-log two-shape 32-worker aggregate is **0.969056 [0.948821, 0.986567]** against OFF, but **0.998657 [0.984165, 1.014132]** against REF. Resampling is independent within each shape's 31/15 blocks; cross-shape block correlation is not assumed. The positive OFF aggregate does not override the required primary or original-control checks.

## Seven failed checks

All 37 checks were frozen before recording. The required control interval is within factor 1.03; candidate nonregression upper ≤1.02; primary gain upper <1 and ≥21/31 wins against both controls; both shape aggregates improve; all maximum-RSS ratios ≤1.05; and all diagnostics qualify.

|Failed check(s)|Paired ratio [95% interval]|Requirement|
|---|---:|---|
|1×2 / 32 OFF/REF calibration|1.027507 [1.011733, 1.045152]|Entire interval within [1/1.03, 1.03]|
|1×2 / 32 ON/OFF primary gain|0.977754 [0.956035, 1.000362]|Upper <1|
|1×2 / 32 ON/REF nonregression and primary gain|1.004650 [0.989710, 1.022035]|Upper ≤1.02 and <1, respectively|
|2×2 / 16 OFF/REF calibration|0.985244 [0.961780, 1.009098]|Entire interval within [1/1.03, 1.03]|
|2×2 / 32 OFF/REF calibration|1.033595 [0.993996, 1.079093]|Entire interval within [1/1.03, 1.03]|
|Two-shape 32 ON/REF aggregate gain|0.998657 [0.984165, 1.014132]|Upper <1|

The third row contains two failures. Both primary win counts pass at 22/31; all twelve RSS checks, eleven of twelve nonregression checks and diagnostics pass. No exclusion, extension, reroll, margin change, favorable subset or additional selection exception follows. This is a failed selection gate, not proof that the arithmetic is universally slower. Its correct algebra, transform gain, query estimates and instruction savings remain in the evidence.

## Resources and CPU profiles

|Shape/workers|Maximum process RSS REF/OFF/ON MiB|Median CPU ms/query OFF/ON|Median minor faults/query OFF/ON|
|---|---:|---:|---:|
|1×2 / 1|13 / 13 / 13|37.575091 / 36.909909|0.090909 / 0.090909|
|1×2 / 16|23.25 / 23.5 / 23.25|45.910569 / 45.082176|18.450980 / 16.294118|
|1×2 / 32|27 / 27.5 / 27|70.413841 / 69.434272|8.807947 / 8.900662|
|2×2 / 1|23.25 / 23.25 / 23.25|40.175636 / 39.371909|0.090909 / 0.090909|
|2×2 / 16|32.75 / 32.5 / 32.75|49.163647 / 48.380922|13.784314 / 14.568627|
|2×2 / 32|35.25 / 35.25 / 34.75|76.314627 / 75.161431|13.176471 / 11.803922|

RSS includes fixture loading, startup, primes and allocator retention. It is neither full-prover peak nor minimum RAM. Equal or lower observations do not establish allocation-free execution or pressure/swap/mobile behavior. Minor faults count mappings without disk reads, not memory-bandwidth stalls. Allocation counts and major faults are not instrumented.

All nine profiles qualify: **82,385 samples**, zero LOST, registered PID/TID/CPU/MMAP/FORK and fixed 16M user-cycle period checked, at most 0.948% unknown. All 36 PMU event values run 100%. Absolute sample-period coverage is not independently validated; single counter panels are descriptive.

|Query/workers|Instructions OFF/ON per query|Cycles OFF/ON per query|
|---|---:|---:|
|1×2 / 16|550,551,244 / 537,371,395|223,404,331 / 218,806,159|
|1×2 / 32|568,716,270 / 555,316,606|319,862,177 / 311,987,151|
|2×2 / 32|598,638,239 / 583,445,881|350,300,489 / 341,374,912|

The primary 32-worker panel has about **2.36% fewer instructions and 2.46% fewer cycles**. Cache changes are mixed; these panels do not identify a unique cache/DRAM, spill, clock or scheduler cause. Borrowed point-update arithmetic still receives about 39.9%/41.4% of OFF/ON samples, reduction 14.9%/14.2%, inversion 7.2%/7.8% and epoch handling 9.7%/9.9%. These are instruction-pointer sample shares, not exclusive wall time or predicted removable proof time. The map improvement touches a limited part of the complete query.

## Review and conclusion

The frozen reader passes independent raw process-mean reconstruction, 318 latency rows, paired comparisons, both shape aggregates, all 37 checks and all seven failures. It verifies 412 pins, original176 recovery, seven native/caller receipts and all 399 capture/offline command stream hashes. Three tool builds also complete; the experiment has 409 successful workflow commands. The reader shares bootstrap routines and detailed PMU/profile parsers with the producer and independently implements the two-shape aggregate. This is same-agent review, not external audit. Float comparisons use 1e−12 relative tolerance; counts, integers and hashes are exact.

The exact query recipe closes without production adoption or whole-SDK selection. A new search must reduce substantial real G2/KH arithmetic or data work rather than infer a large proof improvement from this map. No replacement candidate is selected here. Full SDK, ALL8 plus Merge54, independent Gnark, resource/admission, portable and concurrent-proof qualification remain necessary for any future adoption.

## Accepted CPU and Metal comparison, inherited

These accepted-library full-proof observations use fixed worker placement, not a production pinning default. Supplied M5 references use different hardware. Historical standard 1×2 **7.915095 ms** remains valid; latest standard **8.075143 ms** and fixed **7.869302 ms** retain their own conditions. This query experiment establishes no accepted code regression, new proof record or full-proof speed change.

|Shape|Accepted fixed ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.869302|37 /4.702×|24 /3.050×|3.7|
|2×2|10.984868|59 /5.371×|30 /2.731×|5.9|
|4×2|15.734200|109 /6.928×|48 /3.051×|10.9|
|8×2|23.929023|189 /7.898×|74 /3.092×|18.9|
|12×2|33.722646|284 /8.422×|101 /2.995×|28.4|
|16×2|39.776845|335 /8.422×|124 /3.117×|33.5|
|24×2|66.183856|528 /7.978×|176 /2.659×|52.8|
|Merge24×1|65.448733|507 /7.747×|175 /2.674×|50.7|

Source: [preceding full campaign](../2026-11-09-weighted-prefix-fixed-sdk176/README.md). Merge54 remains 162.489760 ms without an M5 reference. Accepted176/SDK6d83 and canonical **38,668** verified exports remain unchanged. Zero new SDK requests/Gnark exports, AWS resources, Aeglos runs, mobile/huge-circuit/pressure/swap/minimum-RAM or concurrent-proof measurements are added. The full all-eight verified CPU10× goal remains active and unmet, **0/8**.

[Design](artifacts/design.md), [capture protocol](artifacts/capture-protocol.md), [source audit](artifacts/source-audit.json), [candidate query](artifacts/g2_forward_query.rs), [borrowed windows](artifacts/borrowed.rs), [native tests](artifacts/query-tests.rs), [prospective review](artifacts/prospective-review.json), [all observations](artifacts/analysis.json.gz), [result review](artifacts/review-result.json), [decision](artifacts/decision.json), [continuation loop](artifacts/next.md), [source/receipts/decoded profiles](artifacts/evidence.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS). The archive retains member hashes and exact UTF8/gzip roundtrip. Executables/libraries, raw perf.data, unchanged full source, keys/witnesses/proofs, the private proof inventory and FIFO contents are excluded. Historical prelaunch notes are not current liveness evidence.
