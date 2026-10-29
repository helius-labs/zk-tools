# Early canonical output X standard SDK pilot

**The full-proof primary gate fails; production remains unchanged.** The passing caller result does not establish a gain in the standard prover. This exact SDK candidate is not adopted.

ON/OFF paired ratio is 0.999244496 [0.997223650, 1.002353813], 17/31 wins: the saving estimate is 6.021 µs, with uncertainty spanning parity. ON/accepted is 1.004017497 [1.001188080, 1.007720711], 8/31 wins, a 32.431 µs paired regression. Both predeclared primary-gain checks fail; fresh-control calibration passes.

## Candidate and parent evidence

The [actual-caller component](../2026-10-29-early-x175/README.md) remains a valid small gain in its measured context: all 12 contexts improve 0.320% versus fresh OFF, and six logical-32 contexts improve 0.597%, each with 14/15 aggregate wins. Those synthetic valid-point batch measurements are not proof timings. Their positive results are retained without claiming transfer to the SDK.

The clean SDK snapshot removes the diagnostic adapter child and exact module/reexport suffixes. Its 177 files retain 171 byte-exact originals out of 175; four originals have reversible prototype edits and two children are new. All 18 field files and the core arithmetic child match the qualified component byte-for-byte. New private cfg `zopro_early_x` selects ON; OFF retains original arithmetic bodies. There is no trace, hot-loop environment selector, diagnostic export or altered SDK driver. Reversing the prototype edits and removing children recovers all 175 production files.

The arithmetic canonicalizes output X before Y, reuses canonical Ax−X without the generic final 2p correction, then exports X through the original transpose without another p reduction. The typed wrapper has a private constructor. Products, tail/exception/sign handling, buffers/capacities, key arrays, solver, query/window scheduling and allocation forecasts remain unchanged. No per-key table, new heap buffer or pool is added. Existing SIMD export instructions/unsafe block retain their original assumptions. Compiler stack behavior is not qualified as minimum RAM.

Fresh clean OFF/ON pass all 124/125 core tests and eight prover tests each. Parent 33/34 field-test results are inherited from byte-exact field source; the removed diagnostic cfg is absent from those field files. This does not assert arithmetic binary identity. Native flags, frozen transitive libraries, actual linked G1/G2 caller disassembly and the absence of diagnostic adapter symbols are checked. Official generator/template integration, portable/resource checks and final adoption were conditional on proof performance and have not been selected by this failed pilot.

## Verified primary latency

Ryzen 9950X, 16 physical/32 logical CPUs, 32 workers pinned to CPUs 0–31. Thirty-one fixed rotated/reversed blocks compare accepted SDK `20fde…` with freshly built clean OFF/ON. Each process measures 151 requests after three primes. Timed work includes fresh witness solving, randomized proving, SDK verification and serialization; loading and primes are excluded from latency. CPU sums user+system time across threads. RSS is maximum lifetime process high-water mark across the 31 blocks, including load and primes; it is not incremental workspace or minimum RAM.

|Variant|Median latency ms|CPU ms per request|Median / maximum RSS MiB|
|---|---:|---:|---:|
|accepted|8.039747|182.204|131.25 / 134.75|
|OFF|8.083209|181.949|130.25 / 134.50|
|ON|8.067820|181.764|130.50 / 134.25|

Displayed cells are medians of process medians; paired savings/ratios use matching blocks. They need not equal differences/ratios of the marginal medians.

|Comparison|Ratio|Bounds|Wins / 31|Paired saving µs|
|---|---:|---|---:|---:|
|1x2-w32/OFF/accepted|1.004605187|[1.001807420, 1.007465925]|9|-36.890|
|1x2-w32/ON/OFF|0.999244496|[0.997223650, 1.002353813]|17|6.021|
|1x2-w32/ON/accepted|1.004017497|[1.001188080, 1.007720711]|8|-32.431|

The prospective primary gate requires upper paired ratio<1 and at least 21/31 wins against both controls; OFF/accepted calibration bounds must fit [1/1.03,1.03]. Individual order ranks 10–22 of 31 give nominal 97.055% median coverage under independent/exchangeable blocks, not familywise guarantees. There is no minimum-gain floor, post-capture waiver, observation exclusion, sample extension, fastest-layout selection or unchanged reroll. The failed checks stay failed even though the ON/OFF saving estimate is positive.

## Proof verification and automatic profiles

All nine shapes × three variants pass 27 preflights, each one request after three primes and an independent Gnark check, before primary timing. All 93 latency processes precede six separate FIFO-gated 199 Hz cycles:u profiles: primary 16/32 × accepted/OFF/ON, 251 requests after three primes. All 99 latency/profile captures finish before any of their independent verifications, decoding or raw-loss audits. Physical 16 has diagnostic profiles only; it is not a new latency qualification. No owned build/helper/hash/archive or second CPU workload overlaps capture. Offline decoding runs as the file owner after all captures.

|Profile|Samples|
|---|---:|
|cycles-1x2-w16-OFF|5409|
|cycles-1x2-w16-ON|5589|
|cycles-1x2-w16-accepted|5350|
|cycles-1x2-w32-OFF|9232|
|cycles-1x2-w32-ON|8981|
|cycles-1x2-w32-accepted|9285|

Each profile has one sampled SDK PID and zero lost records, including a separate raw-record audit. Complete symbols are retained. Inlining and attribution can change visible weights; these samples are not exclusive critical-path wall time or proof of an arithmetic, cache, clock or scheduler cause.

All **15,954 SDK requests** and **126 distinct independently Gnark-verified exports** pass. Each retained proof is 164 bytes, has the expected fixture/public binding, and has a unique hash disjoint from 20,755 prior exports. The ledger advances to **20,881**, with 252 proof/public members. All 276 capture/offline steps and ten native steps complete. Sources, helper/library/binary/input pins close; source, receipt and proof archives round-trip byte-for-byte with per-member SHA256. No successful capture or test is repeated.

## M5 comparisons and remaining scope

Only primary32 latency is newly measured here. The comparison below uses the fixed new primary measurements and supplied historical M5 references of 37 ms CPU and 24 ms Metal, not fresh Apple measurements.

|Variant|Primary ms|M5 CPU speedup|M5 Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|accepted|8.039747|4.602×|2.985×|3.7|
|OFF|8.083209|4.577×|2.969×|3.7|
|ON|8.067820|4.586×|2.975×|3.7|

All eight accepted-production comparisons remain inherited below. The original full-performance-gate failure/physical16 engineering exception is preserved and not transferred to this candidate. Primary accepted production peak remains 134 MiB.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The original all-eight CPU 10× goal remains active and unmet, 0/8. The candidate remains unadopted; this exact SDK recipe is closed without a subset or compiler-layout rescue. There is no new AWS/Aeglos, ARM/mobile/TEE, forester 250/4999, memory-pressure/swap/minimum-RAM or concurrent-throughput qualification.

[Design](artifacts/design.md), [all blocks and profiles](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [verification inventory](artifacts/proof-inventory.json), [review](artifacts/review.json), [current accepted local/AWS summary](../2026-10-29-current-cpu-summary/README.md).
