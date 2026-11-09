# Complete inverse-oriented G2 query

The new query passes correctness checks but **fails its declared performance gate**. For the primary 1×2 query at 32 workers, the paired candidate/fresh-control ratio is **0.988576 [0.974123, 1.003076]**, with 22/31 wins. The point estimate improves 1.14%, but the interval includes a small slowdown. Four original/fresh-control calibration checks also fail. Production is unchanged, and this exact query recipe is closed without a rerun or an additional selection exception.

These are complete **G2 query** measurements, not complete proofs. The earlier accepted full-proof measurements below 8 ms remain valid: **7.915095 ms** under the historical standard arrangement and **7.869302 ms** with fixed worker placement. The separate latest standard observation was 8.075143 ms. These different campaigns do not establish a code regression or a new proof record. This experiment adds no SDK proof or Gnark verification.

## Implementation and scope

For the existing signed split `s = k1 + lambda*k2`, the identity `s = lambda*(k2 + lambda²*k1)` permits swapping the two component pairs in the existing 32-byte rows. The high buckets use the qualified inverse cubic map, with positive 191-bit multiplier beta². A final forward map corrects the projective result using two scalar Fq products. The [preceding transform study](../2026-11-09-inverse-phi-small176/README.md) provides the exact field library; it is neither rebuilt nor retested here. Its failed calibration checks and explicit exception selecting this distinct query experiment remain part of the history.

The candidate includes indexed top windows, ordinary borrowed windows, owned fallback, grouping and grouping declines, resource admission, tails, weighting, spills and final normalization. High affine buckets use the four-point SIMD inverse map; projective spills use scalar beta². Swapping component streams can change collisions and spills; the measurement includes those effects. Width 12, two parts, component magnitudes/signs, queue capacities and representations are retained.

Original query functions remain byte-exact in a private snapshot. Of 176 original source members, 174 remain exact and two parents have reversible appended declarations. Three new children hold the candidate and tests. A probe bridge selects either implementation; the production entry is not switched. The source audit recovers every original member. No input-sized point/scalar array, retained key/witness copy, per-key table or execution plan, production pool, artificial memory cap, or global field dispatch change is introduced. The reused field has 64 logical factor/quotient bytes; compiler constants and stack space are additional.

## Correctness and actual controls

Three new native tests pass, with 125 unchanged tests filtered and inherited:

- 40 raw/owned complete-query cases against independent Arkworks arithmetic, including lengths 0, 7, 1,023, 1,024, 1,025, 4,096, 30,089, 65,535 and 65,536; full-width and short scalars; zeros; identity, equal, opposite and repeated points; ordinary and top windows; and owned fallback.
- 21 grouping/admission cases, including accepted grouping, small effective groups, all-zero input, existing size cutoffs and reservation denial.
- 106 nested complete queries on all 53 actual workers of 1/4/16/32-worker Rayon pools with 512 KiB requested stacks. Expected results use independent Arkworks generator/scalar arithmetic. Inputs remain unchanged.

The expected length-guard assertion is caught by a passing test; its stderr is retained. The generic no-ASM/no-IFMA guard compiles successfully. This is a compilation check, not execution on a mobile device. All five native build/test steps and two actual caller links finish successfully on their first workflow executions.

REF links the original frozen field and original query. OFF and ON share one fresh binary and the qualified new field/core: OFF selects the exact original query; ON selects the inverse query. Both actual linked consumers contain the expected query symbols. REF is not a byte-identical executable copy of OFF. A new field link and fat LTO can change generated code even when original query source is unchanged; the observed OFF/REF difference has no established cause.

Saved real B-query fixtures contain 30,089 points for 1×2 and 52,274 for 2×2. Their SHA-256 values are `421d6ba37e8b3a6f5feb23829c174d11bc034cb9bd8874adf672d65038fb0ad0` and `2b325a5850f1fc9ea21e715e6027f14cc7a29d2c602d0acc683bbfb8377921bc`. Existing independent complete-MSM oracle receipts also bind subgroup/on-curve points and canonical scalars. The original solves and oracles are not rerun. Every measured output matches the saved independent oracle, and key digests remain unchanged.

## Measurement and results

Ryzen 9950X, 16 physical/32 logical CPUs; Rust 1.98.1, native O3, fat LTO, one codegen unit and debug information. Each process creates one global Rayon pool with default stacks before any requests. Workers pin to CPU 2 for the solo case or their index in CPUs 0–15/0–31. Before/after task counts are N+1 and final worker CPUs agree. This is the same conditional placement for all arms, not an adopted production default or concurrent-proof benchmark.

The clock includes scalar reset into reused input, preparation/grouping/point sums, split/swap, allocation and drop, queues, ordinary/top/weighted processing, map correction and affine normalization. Fixture loading, pool creation, three priming queries, output hash checks and one postcheck query are outside the clock. No completed query result is reused. The statistic is total measured integer nanoseconds divided by requests in each process; the table displays medians of those process means.

The frozen schedule has 31 rotated/reversed matched blocks with 151 requests per primary 1×2/32-worker process. Other cases have 15 blocks and 51 requests per process, except solo processes with 11. There are 18 preflights, 318 latency processes, nine separate PMU panels and nine separate profiles: **354 caller processes and 28,140 complete queries**, including primes and postchecks. All recording precedes 45 offline steps, for 399 capture/offline commands. The confirmed live handle 94821 was polled through terminal exit 0; no owned helper, build, hash, decoder, analyzer, report or push overlaps recording.

|Shape/workers|REF ms|OFF ms|ON ms|Paired ON/OFF [95% interval]|Wins|
|---|---:|---:|---:|---:|---:|
|1×2 / 1|38.469262|37.608417|37.063668|0.983721 [0.981823, 0.985806]|15/15|
|1×2 / 16|3.643822|3.564846|3.515045|0.995293 [0.970767, 1.018491]|9/15|
|1×2 / 32|3.292683|3.159463|3.092633|0.988576 [0.974123, 1.003076]|22/31|
|2×2 / 1|40.968683|40.323363|39.441712|0.981521 [0.976822, 0.986160]|15/15|
|2×2 / 16|4.129645|4.086055|4.018908|0.984526 [0.971439, 0.999082]|13/15|
|2×2 / 32|3.662203|3.664849|3.617436|0.983062 [0.959310, 1.006574]|11/15|

Ratios are paired geometric means and need not divide the marginal medians. The paired-log bootstrap uses 20,000 resamples, seed `0x551c36a48ee2d971`, and sorted ranks 499/19,499. Intervals assume exchangeable independent blocks and do not provide familywise coverage. The primary ON/REF ratio is 0.948214 [0.935136, 0.961857], 25/31 wins; it cannot attribute that entire apparent improvement to the candidate because OFF/REF itself is 0.959172.

The equal-log two-shape 32-worker aggregate is 0.985815 [0.971903, 0.999788] against OFF and 0.957625 [0.940957, 0.975000] against REF. Resampling is independent within each shape; cross-shape block correlation is not assumed. Both aggregate checks pass. This two-shape result is not an all-eight or full-proof result and does not override the required primary check.

All 37 checks were declared before recording. All 12 candidate/control nonregression checks, all 12 maximum-RSS checks, both primary win counts, the primary ON/REF gain, both two-shape aggregates and diagnostics pass. The five failures are:

|Check|Ratio [95% interval]|Required|
|---|---:|---|
|1×2 / 16 OFF/REF calibration|0.984001 [0.968625, 1.001928]|Entire interval within [1/1.03, 1.03]|
|1×2 / 32 OFF/REF calibration|0.959172 [0.946054, 0.972261]|Entire interval within [1/1.03, 1.03]|
|2×2 / 16 OFF/REF calibration|0.985138 [0.968541, 1.000603]|Entire interval within [1/1.03, 1.03]|
|2×2 / 32 OFF/REF calibration|0.983793 [0.961152, 1.006403]|Entire interval within [1/1.03, 1.03]|
|1×2 / 32 ON/OFF primary gain|0.988576 [0.974123, 1.003076]|Upper bound <1|

Both solo calibrations pass. There is no minimum gain floor: the primary fails because its interval crosses 1, not because a 1.14% improvement is deemed too small. No sample exclusion, extension, favorable subset, threshold change, unchanged rerun or additional engineering exception is selected.

## Resource observations and profiles

|Shape/workers|Maximum RSS REF/OFF/ON MiB|Median process CPU time OFF/ON ms per query|Median minor faults OFF/ON per query|
|---|---:|---:|---:|
|1×2 / 1|13 / 13 / 13|37.607818 / 37.063364|0.090909 / 0.090909|
|1×2 / 16|23.5 / 23.5 / 22.75|45.906235 / 45.193333|17.058824 / 14.568627|
|1×2 / 32|27.25 / 27 / 25|71.021444 / 69.916384|8.682119 / 5.642384|
|2×2 / 1|23.25 / 23.25 / 23.25|40.321636 / 39.441091|0.090909 / 0.090909|
|2×2 / 16|32.75 / 32.75 / 32.75|49.275490 / 48.542647|13.901961 / 13.392157|
|2×2 / 32|35.25 / 35 / 35.5|77.044902 / 75.832039|12.372549 / 12.372549|

RSS is the whole benchmark process peak, including fixtures, startup, priming and allocator retention. It is neither prover peak nor minimum RAM. Minor faults are mapping events that do not require disk I/O; these counts do not establish a memory-bandwidth bottleneck or pressure/swap behavior. Major faults and allocation counts are not instrumented by this driver. More workers reduce wall time while increasing aggregate process CPU time; this supports investigating parallel overhead, without identifying its cause.

All nine profiles qualify: **83,496 samples**, zero LOST, registered PID/TID/CPU ownership and 16M cycle period checked, with at most 0.963% unknown. All 36 PMU event values run 100%. Absolute cycle-period coverage is not independently validated. Single counter panels are descriptive, not repeated causal measurements.

|Query/workers|Instructions OFF/ON per query|Cycles OFF/ON per query|
|---|---:|---:|
|1×2 / 16|550,807,850 / 537,717,671|223,833,265 / 218,906,708|
|1×2 / 32|567,698,286 / 555,598,823|319,953,329 / 316,021,367|
|2×2 / 32|597,061,726 / 584,067,112|352,290,434 / 346,044,629|

The 1×2/32 panel has approximately 2.13% fewer instructions and 1.23% fewer cycles, while cache misses increase from 1,142,441 to 1,208,422 per query. There is no uniform cache improvement. In OFF/ON profiles at 32 workers, borrowed point-update arithmetic accounts for about 40.2%/42.0% of samples, reduction 15.0%/15.3%, inversion 7.7%/7.5%, and Crossbeam epoch handling 9.6%/9.8%. At 16 workers, epoch handling is about 3.9%/4.1%. The inverse map does not remove the dominant arithmetic or parallel overhead. These are instruction-pointer sample shares, not exclusive wall-time phases or a prediction of removable proof time.

## Review and decision

The frozen result reader reconstructs all raw process means, 318 latency rows, paired comparisons, both shape aggregates, all 37 checks and all five failures. It verifies all 412 pins, original-source recovery and stream hashes for seven setup and 399 capture/offline commands. Review passes while the performance gate remains false. It shares bootstrap/statistical routines and detailed profile/PMU parsers with the producer; this is same-agent review, not an external audit. Float comparison tolerance is 1e−12 relative; hashes, integers and counts are exact. Successful qualification, caller links, captures, decoders and the previous field work are not replayed.

The exact query recipe is closed and not adopted. Its same-binary positive estimates and lower instruction counts are retained as measured evidence. No full-proof speedup is established. A future experiment must introduce a distinct work reduction and its own prospective comparison; this report selects no replacement candidate. Full SDK, ALL8 plus Merge54, Gnark, resource/admission, portable execution and concurrent-proof qualification remain necessary before production adoption.

## Accepted full-proof CPU and Metal comparison, inherited

These accepted-library observations come from the preceding full campaign with fixed worker placement. They are not this query's timings or a production pinning default. The supplied M5 references use different hardware. Historical standard 1×2 **7.915095 ms** remains valid; latest standard **8.075143 ms** and fixed **7.869302 ms** retain their own conditions.

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

Source: [preceding full campaign](../2026-11-09-weighted-prefix-fixed-sdk176/README.md). Merge54 remains 162.489760 ms without an M5 reference. Accepted176/SDK6d83 and the canonical 38,668 verified exports are unchanged. This experiment adds zero SDK requests/Gnark exports, AWS resources, Aeglos runs, mobile/huge-circuit/pressure/swap/minimum-RAM or concurrent-proof measurements. The verified all-eight CPU10× goal remains active and unmet, **0/8**.

[Design](artifacts/design.md), [prospective protocol](artifacts/capture-protocol.md), [source audit](artifacts/source-audit.json), [candidate query](artifacts/g2_inverse_query.rs), [borrowed windows](artifacts/borrowed.rs), [native tests](artifacts/query-tests.rs), [prospective review](artifacts/prospective-review.json), [all observations](artifacts/analysis.json.gz), [result review](artifacts/review-result.json), [decision](artifacts/decision.json), [continuation loop](artifacts/next.md), [source/receipts/decoded profiles](artifacts/evidence.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS). The archive retains member hashes and exact gzip roundtrip. Executables/libraries, raw perf.data, unchanged full source trees, keys/witnesses/proofs and the private proof inventory are excluded. Historical pending-capture notes inside the archive describe their original time, not current liveness.
