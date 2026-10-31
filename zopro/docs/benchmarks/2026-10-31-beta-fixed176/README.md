# Fixed-factor G2 bucket transforms

The declared complete-transform gate is **false**. This component experiment changes no production source and establishes no new proof latency. All six contexts and all failed checks remain below.

The fixed-factor transform clearly beats scalar multiplication in all six contexts, with 15/15 wins against both scalar controls. At 2,048 points and 32 workers its paired ratio is 0.277533, about 3.60× faster than fresh scalar. It also beats ordinary SIMD by 11.4–12.0% at one worker. However, ordinary SIMD wins at 32 workers: fixed-factor is 14.4% slower for 2,048 points and 51.9% slower for 64 points. Only those two required comparisons fail; all scalar calibration checks pass. These positive caller results are retained without claiming a proving gain or adopting the older SIMD implementation.

## Arithmetic and scope

The candidate specializes the [qualified fixed-factor quotient kernel](../2026-10-24-fixed-multiplier-component/README.md) to the G2 cube-root constant beta. Five radix-52 factor words and five quotient words are computed at compile time: 80 logical bytes, independent of keys and circuit size. No per-key table, input-point copy, dynamic preparation, heap buffer or execution plan is added to the transform. Broadcast registers, object-file constants and stack frames can exceed that logical count.

For B=2^52, R=B^5 and c=floor(beta*R/p), the high-product approximation omits less than 8/B. For normalized a<2p it yields 0≤a*beta−q*p<(1+a/R+8/B)*p<2p. Two low products modulo R recover the nonnegative remainder without a final correction. Numerical R256 representation is preserved. The general truncated arithmetic and signed-coefficient bound below 2^57 are inherited; the new constant specialization has independent native output checks. This is separate from the [older ordinary-SIMD bucket transform](../2026-10-24-g2-phi-vector/README.md), whose failed full-proof gate remains unchanged.

The complete in-place caller includes input packing, multiplication, unpacking, identity branches and X stores; Y is unchanged. Scalar tails retain the original formula. Public known-generator subgroup points and deterministic identity slots model 64/2048-point bucket arrays. These are synthetic caller buffers, not captured proof buckets. Worker-owned input copies belong to the benchmark fixture; they are not a proposed prover cache.

## Fixed caller measurements

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. FROZEN uses the exact accepted field library; OFF is fresh scalar; SIMD and FIXED use ordinary and fixed-factor eight-lane arithmetic. The three fresh modes share one executable. Both final links use -O/native/fat-LTO/single-CGU. Actual scalar and ordinary-SIMD static function counts match across the frozen and fresh callers; calibration is still measured.

Fifteen fixed paired blocks rotate/reverse six contexts and four variants. Each worker has 512 KiB stack and individual affinity: CPU 2 solo, CPUs 0–15 or 0–31. Three checked primes precede 192 measured cycles of three transforms. beta cubed is one, so every prime-pass and final worker output must exactly equal its original coordinates, with unchanged capacity. Intermediate timed transforms are not individually checked. Input generation, thread startup, barriers before the measured worker loops and final checks are outside the per-worker clock. The wave statistic is the maximum worker elapsed time divided by 576 transforms. It is an independent transform wave, not proof scheduling, single-proof latency or concurrent proof throughput.

Every FIXED/scalar comparison must have upper<1 and at least 10/15 wins against both frozen and fresh controls in all six contexts. Both 32-worker FIXED/SIMD comparisons must also improve. All OFF/frozen scalar intervals must fit factor 1.03. Sorted ranks 4–12 of 15 give nominal 96.484375% marginal median bounds, without familywise guarantees. No gain floor, exclusion, extension, width/iteration sweep, favorable subset or unchanged reroll. A pass only selects separately declared complete-query/full-proof qualification.

|Points/workers|Frozen µs|OFF µs|SIMD µs|FIXED µs|FIXED/OFF [bounds], wins|FIXED/SIMD [bounds], wins|
|---|---:|---:|---:|---:|---|---|
|64/1|1.326911|1.323990|0.367030|0.321543|0.242113 [0.241596, 0.245672], 15/15|0.879793 [0.873168, 0.890932], 14/15|
|64/16|1.426665|1.431082|0.396530|0.372767|0.258044 [0.245651, 0.275418], 15/15|0.931377 [0.907789, 0.996722], 12/15|
|64/32|2.369970|2.374127|0.548116|0.842856|0.355233 [0.335815, 0.373112], 15/15|1.518693 [1.486417, 1.562094], 1/15|
|2048/1|42.105665|42.060703|11.680661|10.324859|0.246006 [0.244440, 0.247363], 15/15|0.886281 [0.882625, 0.894170], 15/15|
|2048/16|46.146894|46.400984|12.800550|12.946608|0.276927 [0.264420, 0.287025], 15/15|1.000691 [0.947505, 1.050108], 7/15|
|2048/32|79.897280|79.878599|19.254469|22.129280|0.277533 [0.267555, 0.285127], 15/15|1.143933 [1.101790, 1.157546], 0/15|

|Points/workers|FIXED/frozen [bounds], wins|OFF/frozen calibration [bounds]|Frozen/OFF/SIMD/FIXED max process RSS MiB|
|---|---|---|---:|
|64/1|0.242053 [0.240025, 0.244911], 15/15|0.997644 [0.988423, 1.002937], 10/15|2.00/2.00/2.00/2.00|
|64/16|0.258089 [0.253383, 0.277672], 15/15|1.004523 [0.997428, 1.006981], 4/15|2.00/2.00/2.00/2.00|
|64/32|0.354773 [0.335934, 0.372545], 15/15|0.999362 [0.997643, 1.002185], 8/15|2.25/2.25/2.25/2.25|
|2048/1|0.245325 [0.244791, 0.246313], 15/15|0.997862 [0.996869, 1.001090], 11/15|2.50/2.50/2.50/2.50|
|2048/16|0.278326 [0.264756, 0.288624], 15/15|1.004183 [1.002069, 1.012191], 2/15|6.25/6.25/6.25/6.25|
|2048/32|0.276706 [0.267418, 0.285584], 15/15|1.000001 [0.998770, 1.001603], 7/15|10.50/10.50/10.50/10.50|

Failed checks: `["n64-w32/fixed/simd/gain","n2048-w32/fixed/simd/gain"]`. CPU service per wave, minor faults and context switches are retained for every process. RSS is lifetime process high-water including fixture generation, worker copies and primes, not prover peak or minimum RAM. Kernel source has no heap allocation; there is no measured allocator-call count.

|Points/workers|OFF CPU ms/wave|SIMD CPU ms/wave|FIXED CPU ms/wave|
|---|---:|---:|---:|
|64/1|0.001345|0.000387|0.000340|
|64/16|0.022179|0.006260|0.005729|
|64/32|0.073727|0.017910|0.025345|
|2048/1|0.042075|0.011694|0.010345|
|2048/16|0.712464|0.197682|0.188396|
|2048/32|2.470804|0.594231|0.669564|

The transform adds no per-key cache. All variants have the same observed process RSS at each context; that does not isolate cache behavior. Its weaker wide-worker result versus ordinary SIMD remains unexplained. At 32 workers the diagnostic instruction count is 163,375 versus 127,016 per transform, despite fewer static IFMA sites. The single counter panels do not reproduce every latency direction and do not isolate a cache, spill, clock or scheduling cause.

## Code and automatic diagnostics

The fresh ordinary SIMD transform has 100 IFMA sites, 278 static instructions and 12 RSP-operand instructions; fixed has 84, 356 and 87. These are full transform-symbol static counts, not dynamic spills, cycles or memory bounds. The fixed kernel removes multiplications while adding surrounding instructions and stack operands.

All latency precedes six FIFO-gated four-event counter panels and six separate 16-million-user-cycle profiles. Diagnostics run 32,768 cycles per worker after excluded primes, 2048 points at 16/32 workers, OFF/SIMD/FIXED. Each event must report at least 99.9% running coverage. Profile PID, registered worker TID/CPU, binary MMAP, worker FORK, fixed period and zero loss are checked. Attribution requires at least 1,000 samples and at most 5% unknown share; small/failed panels are retained. Absolute sample coverage, exclusive symbol costs, local CPI and removable proof wall time remain unvalidated.

|Panel|Cycles/transform|Instructions/transform|Qualified|
|---|---:|---:|---|
|stat-n2048-w16-fixed-b0|56021.656|163374.696|true|
|stat-n2048-w16-off-b0|230880.054|720611.761|true|
|stat-n2048-w16-simd-b0|63305.602|127015.698|true|
|stat-n2048-w32-fixed-b0|83081.970|163374.714|true|
|stat-n2048-w32-off-b0|385387.752|720611.822|true|
|stat-n2048-w32-simd-b0|85985.820|127015.711|true|

|Profile|Samples|Unknown share|Attribution qualified|
|---|---:|---:|---|
|cycles-n2048-w16-fixed-b0|5479|0.0365%|true|
|cycles-n2048-w16-off-b0|22690|0.0220%|true|
|cycles-n2048-w16-simd-b0|6211|0.0483%|true|
|cycles-n2048-w32-fixed-b0|16310|0.0613%|true|
|cycles-n2048-w32-off-b0|75762|0.0449%|true|
|cycles-n2048-w32-simd-b0|16912|0.0650%|true|

## Validation and provenance

All 37 unfiltered native field tests pass, including 104,544 independently Arkworks-checked lazy-lane outputs over changing chains and a complete transform test covering identities and 16 lengths/tails through 2049 points, and threefold restoration. The first native compile stopped on an include-file inner-doc marker before tests executed. Only that marker changes to an ordinary comment in a separate corrected snapshot. Initial source, pins and compiler failure are retained. The corrected suite, library build, two final links and fixed capture each run once. No core/prover, portable or complete-proof qualification is claimed.

The private snapshot has 177 files, with 175 byte-exact originals, one append-only parent and one arithmetic child. Independent stripping recovers all 176 originals in both snapshots; every production source hash remains accepted. Review independently closes 431 raw commands, including one preserved compile failure, 2014 pin entries, 396 caller receipts, all 30 paired comparisons and gate decisions, every PMU value and every profile bin. Total transform calls including primes: 32644248; profile samples: 143364. Final worker outputs are checked, not every intermediate timed state. The first offline reader stopped on a last-bit JSON float comparison. Separate corrected analysis/publication copies use relative 1e-12 for display-value equality; integer durations, raw data, gates, original capture executable and source pins are unchanged. No capture repeats. No new witness solve, proof, Gnark export or verifier count is added; ledger 27,353 remains unchanged.

The gate outcome controls next action: a failure closes this exact recipe before full-query or SDK timing; a pass permits a separate integration study and still supplies no prover speedup. Fewer static multiply sites alone cannot establish the requested end state. No other owned build/helper/hash/archive or benchmark overlaps capture.

## Accepted proof comparisons — inherited, unchanged

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge 24→1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Accepted merge 54→1 is 161.883899 ms without a supplied M5 reference. Accepted primary process RSS is 132.75 MiB. The [adoption report](../2026-10-30-g2-vector-sign-adoption176/README.md) retains its physical-16 calibration engineering exception and qualification scope. No new memory or platform result follows.

Latest AWS primary remains historical source 175 on c7a.24xlarge: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, each campaign's best tested configuration. That does not compare current source 176. No AWS resources were created here.

All eight CPU 10× targets remain unmet, 0/8. Primary remains 7.928660 ms versus 3.7 ms target. No new AWS/Aeglos, mobile/ARM/TEE, huge-circuit, memory-pressure, swap, minimum-RAM or concurrent-proof result follows.

[Design](artifacts/design.md), [all observations](artifacts/analysis.json), [independent review](artifacts/review.json), [symbols](artifacts/symbols.json), [static code](artifacts/codegen.json), [field specialization](artifacts/beta-fixed.rs), [identities](artifacts/identities.json). Archives: [private source](artifacts/source.tar.gz), [workflow sources and protocols](artifacts/workflow.tar.gz), [raw command receipts](artifacts/receipts.tar.gz), [disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS).

Local root: `/home/ubuntu/src/zopro-perf-work/beta-fixed176`. Binaries, libraries and raw perf.data remain local with hashes.
