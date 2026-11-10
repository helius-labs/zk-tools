# Wide approximate GCD: complete-query result

**Rejected for adoption.** The 62-bit/u128 inversion is correct but makes primary A at 32 workers **5.32% slower** than fresh OFF. Primary G2 at 32 workers is near parity, with no established gain. All control calibrations, memory guards and diagnostic quality checks pass; nine of 52 fixed checks fail. Production and the accepted SDK remain unchanged. These are complete MSM query timings, not proof timings.

The candidate replaces two positive 31-bit approximate binary-GCD blocks with one 62-bit block using 66 leading bits and 62 low bits in u128. Packed matrix rows use `f + 2^64*g` modulo 2^128, with row L1 norm at most 2^62. One exact full-integer matrix update replaces the intermediate update and second extraction. Accepted 62-bit modular coefficient reduction and the exact one-word Euclidean finish remain byte-identical. This instantiates the production [Pornin-style method](https://eprint.iacr.org/2020/972) more widely; it copies no external implementation and claims no algorithmic novelty. Prior packed31, signed small-quotient jumps, signed safegcd and Lehmer studies retain their outcomes.

The wider block removes an intermediate update but adds 128-bit comparison, subtraction, encoded-row arithmetic and extraction. That tradeoff was measured in complete callers rather than selected using inverse-only throughput. There is no width sweep, favorable fixture subset or post-hoc rescue.

Ryzen 9950X, 16 physical/32 logical CPUs. REF is the source-verified accepted complete-query executable; OFF is fresh reversible source with the selector absent; ON enables the new inversion. Driver, point formulas, field multiplication, recoding, grouping, queue, pool and scratch policies remain fixed. Inversion changes both G1 and G2, so A queries are affected callers. The initial design's unchanged-A label was superseded by the explicit 52-check protocol and both frozen readers **before any preflight**. No unchanged-A condition was removed after seeing data.

Thirty-one rotated/reversed matched blocks run 51 queries per process after three checked primes, followed by one postcheck. Every timed output matches a retained independent Arkworks complete-MSM oracle. Loading and pool construction are excluded; scalar reset, grouping/admission, windows/merges/weighting, allocations/drop and final affine normalization are included. Solo uses CPU2; other worker i uses CPUi, with 512 KiB requested stacks.

Times are medians of process medians. Ratios pair blocks and need not equal quotients of displayed medians. Lower bound, median and upper bound use zero-based sorted ranks 9, 15 and 21 of 31, conditional on independent exchangeable blocks and without familywise coverage. All observations, including individual positive ratios, remain in the data.

|Case|REF ms|OFF ms|ON ms|ON/OFF [bounds]|Wins/31|
|---|---:|---:|---:|---|---:|
|1x2-A-w32|1.169665|1.185014|1.247673|1.053229600 [1.046199502, 1.059249460]|1|
|1x2-B-w1|36.677044|37.192386|37.917629|1.019954035 [1.018515590, 1.020570055]|0|
|1x2-B-w16|3.642467|3.653217|3.688203|1.007442678 [1.006177904, 1.013065567]|4|
|1x2-B-w32|3.229247|3.236371|3.239958|1.002712182 [0.999414187, 1.006363187]|10|
|2x2-A-w32|1.502603|1.501471|1.520246|1.011496726 [1.007723430, 1.017850492]|3|
|2x2-B-w1|39.314077|39.786670|40.556288|1.019354305 [1.018412725, 1.019983143]|0|
|2x2-B-w16|4.213974|4.211449|4.254120|1.012881373 [1.008716974, 1.016442992]|5|
|2x2-B-w32|3.779345|3.793782|3.808630|1.002596803 [0.997641994, 1.008730418]|12|

Primary B32 ON/REF is **1.005272873 [0.999060696, 1.006315710]**, with 12/31 wins. The equal-log two-shape B32 aggregate is **1.003860754 [0.997753724, 1.006521950]** versus OFF (11/31 wins) and **1.005126097 [1.000900057, 1.010157175]** versus REF (7/31 wins). The primary B32 and aggregate intervals cross parity versus OFF; neither establishes a speedup. Primary A32 clearly regresses, and both solo B populations lose every block versus both controls.

The prospective gate requires all eight OFF/REF calibration intervals within factor 1.03; all sixteen ON/control nonregression upper bounds ≤1.02; primary B32 and two-shape B32 aggregate gain upper bounds <1 and at least 21/31 wins against both controls; eight RSS guards; and sixteen diagnostic quality checks. There is no minimum gain percentage. Five nonregression and all four gain checks fail. The exact failed checks are:

- `1x2-A-w32/ON/OFF/nonregression`
- `1x2-A-w32/ON/REF/nonregression`
- `1x2-B-w1/ON/OFF/nonregression`
- `1x2-B-w1/ON/REF/nonregression`
- `1x2-B-w32/ON/OFF/gain`
- `1x2-B-w32/ON/REF/gain`
- `2x2-B-w1/ON/REF/nonregression`
- `two-shape-G2-32/ON/OFF/gain`
- `two-shape-G2-32/ON/REF/gain`

All 824 capture commands and 784 checked caller processes succeed once, covering 47,504 complete-query invocations including primes/postchecks. The frozen analyzer and separate same-agent raw reader also pass on their first executions. The reader reconstructs process medians, paired rank ratios, wins, resource observations and all 52 decisions. It shares PMU/sample text-schema parsers with the producer. Independent fixture/oracle generation is inherited and hash-bound, not rerun. These are not new Groth16 proofs.

Eight separately gated PMU panels and eight profiles follow latency. Every recording completes before all offline decoding/analysis. Panels use grouped user cycles, instructions, generic cache misses and branch misses at full scheduled coverage. They are single diagnostic panels, not repeated-trial confidence intervals.

|Context|Instructions ON/OFF|Cycles ON/OFF|Branch misses ON/OFF|
|---|---:|---:|---:|
|1x2-B-w16|1.025557|1.016357|0.823321|
|1x2-B-w32|1.023450|1.025150|0.901828|
|2x2-B-w32|1.023901|1.015490|0.880862|
|2x2-A-w32|1.032861|1.016302|0.886312|

Fewer full-width update boundaries did not reduce executed instructions. Panels retire roughly 2.3–3.3% more instructions and 1.6–2.5% more cycles despite fewer branch misses. All eight profiles qualify: 50,581 samples, fixed 16M-cycle period, zero reported loss, allowed PID/TID/CPU ownership and matching executable MMAP metadata. The widest unknown share is below 1.24%; absolute whole-budget sample coverage is not validated. ON profiles contain `inverse::wide62::inverse`; inverse shares rise in all four paired contexts. Flat samples do not establish exclusive critical-path costs, the exact cause of the slowdown, or a DRAM-bandwidth limit.

All eight RSS guards pass. Primary B32 REF/OFF/ON peaks are **39.0/36.5/39.0 MiB**; 2x2 B32 peaks are **73.0/69.75/64.0 MiB**. Primary A32 OFF/ON both peak at **26.0 MiB** while latency regresses. These diagnostic-process peaks include input/runtime/setup and exclude the full proving key; they are not whole-prover memory or minimum RAM. Raw CPU service, minor faults and switches remain in the reader's 24 resource cells. Major faults are not exposed by the unchanged driver. No table, key/point/solver-plan copy, heap scratch, pool or architectural RAM cap is introduced by the new fixed-stack routine.

Qualification passes 131,072 matrices against a separately written branch-based u128 step reference and 262,144 full-integer rows against signed BigInt cancellation/division/sign checks. Pure 2^62 row-coefficient boundaries and zero/equal/random 256-bit inputs are included. New complete Fq and Fr cohorts each pass 5,884 Arkworks inverse comparisons, including raw Montgomery boundaries, powers/neighbors, random values and output-fed chains; nonzero outputs also satisfy x·inverse=1. Counts include repeated values. All 38 native field, 11 generic field and 127 core tests pass. Native field qualification enables overflow checks. Finite tests are not a formal proof for every input or ARM/mobile execution.

Two reversible parents and three private children recover all 176 accepted originals: inverse selector/module, private query API, the new inversion child and two test children. Both field libraries are reused across test-only repairs. Twelve successful build/qualification commands and two retained test-build failures cover a module-path issue and BigUint shift inference. A direct capture-preparation helper stopped on overlapping path replacement order; its continuation reused generated capture/review/cases. Unused-code/import warnings are retained. There are 846 workflow commands overall: 844 succeed and those two compilation attempts fail. Direct helper compiler invocations are tool-transcript observations, without complete xtask receipts. All scientific successes, capture data, raw decisions and failures are preserved; no capture or successful qualification is repeated.

The publisher checks all 589 final pins, argv/exit/stream receipts, archive roundtrip and report links. Public analysis omits only duplicated raw caller receipt rows; the exact original remains in the archive with its hash. Binaries, libraries, raw perf data, unchanged production sources, private input arrays, keys, witnesses, proofs and the full proof ledger are excluded. Raw perf hashes and source/binary/library identities remain published.

Accepted local 32-worker full-proof observations and supplied historical M5 references remain unchanged:

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37 / 4.726×|24 / 3.065×|3.7|
|2x2|10.979335|59 / 5.374×|30 / 2.732×|5.9|
|4x2|15.803969|109 / 6.897×|48 / 3.037×|10.9|
|8x2|23.944017|189 / 7.893×|74 / 3.091×|18.9|
|12x2|33.703579|284 / 8.426×|101 / 2.997×|28.4|
|16x2|39.858478|335 / 8.405×|124 / 3.111×|33.5|
|24x2|66.186709|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.489902|507 / 7.742×|175 / 2.672×|50.7|

Primary remains **7.829286 ms / 133.50 MiB**; earlier 7.803404 ms used the same executable. Primary at 16 workers remains 9.192237 ms. Merge 54 remains 162.541995 ms without a supplied M5 reference. Accepted production/SDK and the canonical 43,730 verified proofs are unchanged. CPU AVX ≥10× M5 CPU on all eight shapes remains **active and unmet, 0/8**. This adds no SDK trial, Gnark proof export, fresh AWS/Aeglos/M5, phone/ARM, huge-circuit, memory-pressure/swap, minimum-RAM or concurrent-throughput result.

[Analysis](artifacts/analysis.json), [independent raw reader](artifacts/review.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.md), [initial design](artifacts/design.md), [build qualification](artifacts/build-review.json), [pre-clock review](artifacts/prospective-review.json), [artifact identities](artifacts/identities.json), [raw perf identities](artifacts/raw-perf-identities.json), [continuation/autoresearch loop](artifacts/next.md), [sources and exact receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).

Private experiment root: `/home/ubuntu/src/zopro-perf-work/wide-gcd62-176`.
