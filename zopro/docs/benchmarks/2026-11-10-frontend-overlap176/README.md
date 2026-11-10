# Canonical front-stage overlap: full-proof comparison

**Primary improves, but the fixed all-shape recipe is not adopted.** At 32 workers, accepted REF measures **7.783594 ms**, fresh sequential OFF **7.776515 ms**, and overlapping ON **7.747712 ms**. The paired median saving is **0.69% versus OFF** and **0.71% versus REF**, with 12/15 wins against each. Both declared primary gain checks pass. The all-eight aggregate remains near parity, and 8×2 fails its nonregression bounds. Four of 77 checks fail.

These are complete SDK proof requests: solving, fresh randomized proving, internal verification and serialization are timed; key loading, pool construction, three primes and final export are excluded. All 535 fresh exported proofs pass independent Gnark verification. Production and the accepted SDK remain unchanged.

The private canonical-wire path moves public correction, in-place witness conversion and A/B scalar gathering into one side of an existing-pool Rayon join; the other side computes the original quotient product from disjoint solved A/B evaluations. Main A/K/H/B2 queries start after both sides finish. Point formulas, field library, scalar grouping, queues, product normalization, fused-KH guard and output assembly stay original. There is no new owned vector, key/solver plan, lookup table, worker pool or architectural RAM cap.

Scope clarification: the public correction itself performs existing point multiplication and can fall back to an MSM (`lagrange.rs::Folded::public_term`). That work and its temporary lifetime can overlap the FFT. The initial design's “all MSMs follow the join” language refers to the main query phase; it is not a claim that public correction uses no point work or scratch. The frozen implementation, protocol, gates and observations are preserved without a post-capture change. This differs from prior four/five-query overlap and solver-to-A actors, which started main point queries early.

Ryzen 9950X, 16 physical/32 logical CPUs. Fifteen fixed rotated/reversed matched blocks run 31 requests per process after three primes. Solo uses CPU2; otherwise worker i uses CPUi, with 512 KiB requested stacks. Times are medians of process means of integer-rounded SDK nanoseconds. Paired ratios need not equal quotients of displayed medians. Lower/median/upper use zero-based sorted ranks 3/7/11 of 15; coverage assumes exchangeable independent blocks and is not familywise. No minimum gain floor, sample extension, excluded block or favorable subset is used.

|Case|REF ms|OFF ms|ON ms|ON/OFF [bounds]|Wins/15|
|---|---:|---:|---:|---|---:|
|1x2-w32|7.783594|7.776515|7.747712|0.993080991 [0.989961540, 0.997861556]|12|
|2x2-w32|10.932731|10.916275|10.777527|0.989169486 [0.980046558, 0.995873627]|12|
|4x2-w32|15.757088|15.700129|15.725524|1.003501806 [0.986420391, 1.007320003]|6|
|8x2-w32|23.973295|23.975235|24.027755|1.003241023 [0.997410272, 1.033587762]|4|
|12x2-w32|33.827269|33.731044|33.699425|1.000125903 [0.995199263, 1.001978921]|7|
|16x2-w32|39.900917|39.684278|39.876159|1.004382703 [0.996523267, 1.013782837]|4|
|24x2-w32|66.041083|66.049717|66.348634|0.998228604 [0.993334168, 1.010145038]|8|
|merge-24x1-w32|65.717994|65.316638|65.199744|0.997440152 [0.991759003, 1.005621914]|8|
|merge-54x1-w32|162.170158|162.466095|162.894605|1.001304497 [0.997276930, 1.007120151]|6|
|1x2-w1|94.302386|94.084023|94.014452|1.001434795 [0.995915498, 1.005412503]|7|
|1x2-w16|9.016864|9.014134|8.978434|0.995959656 [0.991380440, 1.001098189]|10|

Primary ON/REF is **0.992859331 [0.989282098, 0.999881178]**, 12/15 wins. The displayed primary medians differ by **35.9 µs**; that difference and the block-paired percentage summarize different distributions. The measured saving is retained despite rejection of the broader recipe. 2×2 also improves against both controls: ON/OFF **0.989169486 [0.980046558, 0.995873627]**, 12/15 wins; ON/REF **0.982817768 [0.981509625, 0.994661724]**, 14/15 wins.

The equal-log ALL8 block aggregate is **1.000327460 [0.996617963, 1.002950951]** versus OFF, 7/15 wins, and **0.997632232 [0.993294681, 1.000106512]** versus REF, 10/15 wins. Both intervals cross parity; neither establishes an all-eight gain. 8×2 ON/OFF is **1.003241023 [0.997410272, 1.033587762]** and ON/REF **1.000856925 [0.998322124, 1.038292066]**. Its upper bounds exceed the 2% guard, while its lower bounds are below one. This is failed nonregression qualification, not a demonstrated 3.8% slowdown.

The fixed gate requires all eleven OFF/REF calibration intervals within factor 1.03, all 22 ON/control nonregression upper bounds ≤1.02, primary32 and ALL8 gains against both controls with upper<1 and at least 10/15 wins, all 22 RSS comparisons within 5%+1 MiB, and eighteen diagnostic quality checks. Every calibration, RSS and diagnostic check passes. The four failures are:

- `8x2-w32/ON/OFF/nonregression`
- `8x2-w32/ON/REF/nonregression`
- `ALL8/ON/OFF/gain`
- `ALL8/ON/REF/gain`

This closes the fixed universal overlap recipe without adoption or a shape/worker rescue. A small primary benefit is measured; the all-eight improvement and complete nonregression requirement are not established. No unchanged recapture or threshold change follows.

Nine grouped PMU panels and nine cycle profiles run after all latency. All recording finishes before offline decoding and external proof verification. PMU events run at the required ≥99.9% coverage. Profiles contain **118,916 samples**, with fixed 16M-cycle period, zero reported loss, exact executable/PID/TID/CPU ownership checks, and maximum unknown share below 2.27%. Absolute whole-budget sample coverage is not validated. Panels are single diagnostic observations, not replicated counter confidence intervals.

|Context|Instructions ON/OFF|Cycles ON/OFF|Generic misses ON/OFF|
|---|---:|---:|---:|
|1x2-w32|0.999930|1.001914|0.992368|
|1x2-w16|0.999663|0.997292|0.989838|
|2x2-w32|0.994573|0.985071|0.980772|

Primary32 instructions are essentially unchanged while its single cycle panel is slightly higher; 2×2 retires fewer instructions and cycles. Overlapping stages does not remove their arithmetic. These panels and flat profiles do not identify an exclusive critical path, explain 8×2 variability, or establish cache level/DRAM bandwidth as a cause. Full symbol distributions and raw resource observations remain available.

Primary32 REF/OFF/ON peak RSS is **111.25/108.25/111.00 MiB** in this 31-request-per-process campaign. Merge54 peaks are **1995.93/1934.16/1908.45 MiB**. All 22 RSS comparisons pass. These are whole-process peaks including setup/loading/primes, not minimum RAM, compiler stack bounds, memory-pressure qualification or evidence of a universal memory reduction. Earlier same-SDK primary 133.50 MiB remains a valid observation from a different campaign. Longer overlap of existing buffers can still affect peak lifetime. No phone/ARM or huge 250/4999 result is added.

All 127 native core tests pass. One reversible `lib.rs` parent recovers all 176 production originals; the other 175 originals and the accepted field library `1200e635…` remain exact. Ten coherent native/downstream build commands pass without rebuilding the field. Both fresh binaries pass all nine functional shapes at 32 workers plus primary at 1/16 workers: 88 internally checked preflight requests and 22 fresh Gnark exports. The full capture completes **1,071 commands** once: 495 latency processes, eighteen diagnostics, 45 offline commands and 513 independent Gnark verifications. Capture adds 19,422 internally checked SDK requests and 513 fresh unique exports. Totals are **19,510 requests / 535 unique exports**, and the canonical inventory advances **43,730→43,752→44,265**. Export count is distinct from all internally generated proofs.

The frozen analyzer and separate same-agent raw reader pass on their first executions. The reader independently reconstructs integer process means, medians, paired rank ratios, peaks and all 77 decisions. SDK structure/proof-binding and detailed diagnostic parsers are shared; this is not an external audit. All 423 final pins and exact argv/exit/stream receipts are checked. One helper build fails on a duplicate median and omitted demangler dependency; original source/receipts are retained, and the repaired helper builds succeed. Native code and timing protocol are unchanged by that repair. There are **1,131 workflow commands: 1,130 successful and that one failed compile**; direct preparer/compiler/checkpoint commands do not all have xtask receipts. No successful build, qualification, preflight, capture or proof stage is rerun.

Fresh accepted REF and unadopted ON comparisons against supplied historical M5 references follow. M5 hardware and timings are not remeasured here.

|Shape|REF ms|ON ms|M5 CPU ms / REF speedup / ON speedup|M5 Metal ms / REF speedup / ON speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|7.783594|7.747712|37 / 4.754× / 4.776×|24 / 3.083× / 3.098×|3.7|
|2x2|10.932731|10.777527|59 / 5.397× / 5.474×|30 / 2.744× / 2.784×|5.9|
|4x2|15.757088|15.725524|109 / 6.918× / 6.931×|48 / 3.046× / 3.052×|10.9|
|8x2|23.973295|24.027755|189 / 7.884× / 7.866×|74 / 3.087× / 3.080×|18.9|
|12x2|33.827269|33.699425|284 / 8.396× / 8.427×|101 / 2.986× / 2.997×|28.4|
|16x2|39.900917|39.876159|335 / 8.396× / 8.401×|124 / 3.108× / 3.110×|33.5|
|24x2|66.041083|66.348634|528 / 7.995× / 7.958×|176 / 2.665× / 2.653×|52.8|
|merge-24x1|65.717994|65.199744|507 / 7.715× / 7.776×|175 / 2.663× / 2.684×|50.7|

Latest accepted primary is **7.783594 ms**, using the same accepted executable as prior **7.829286 ms** and **7.803404 ms** observations. No replacement is adopted. Merge 54 REF is 162.170158 ms without a supplied M5 reference. The all-eight CPU AVX ≥10× M5 CPU objective remains **active and unmet, 0/8**, for both REF and this unadopted candidate. No fresh AWS/Aeglos, mobile/ARM/TEE, huge-circuit, swap/pressure, minimum-RAM or concurrent-throughput claim follows.

[Analysis](artifacts/analysis.json), [independent raw reader](artifacts/review-result.json), [decision](artifacts/decision.json), [CPU/Metal comparisons](artifacts/fresh-comparison.json), [initial design](artifacts/design.md), [frozen protocol](artifacts/protocol.md), [build review](artifacts/link-review.json), [preflight proof inventory](artifacts/preflight-review.json), [pre-clock review](artifacts/prospective-review.json), [artifact identities](artifacts/identities.json), [source recovery](artifacts/source-audit.json), [continuation/autoresearch loop](artifacts/next.md), [sources and exact receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).

Private experiment root: `/home/ubuntu/src/zopro-perf-work/frontend-overlap176`.
