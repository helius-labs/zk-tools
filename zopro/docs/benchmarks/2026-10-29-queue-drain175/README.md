# Drain collision queues through existing affine batches

29 October 2026. Private source-175 native experiment. Suitable queued points use the original affine batch before retaining the original projective fallback. This is a complete verified SDK pilot; production remains unchanged pending all required qualification.

Declared pilot gate: **false**. Failed checks: `["1x2-w16/ON/accepted","1x2-w16/ON/OFF"]`.

**Primary32 improves against both controls, with 15/15 paired wins.** ON measures **7.889011 ms**, versus 8.035988 ms accepted and 8.069801 ms OFF. Its paired gain is **1.710% against accepted**, ratio 0.982904529 [0.979309999, 0.985773825], and **2.356% against OFF**, ratio 0.976444150 [0.972827853, 0.980220282]. Against the supplied primary M5 references, ON is 4.690× faster than CPU and 3.042× faster than Metal; these are reference ratios, not new Apple measurements.

The full pilot remains unqualified because both physical16 nonregression intervals cross the declared 1.02 upper bound. ON/accepted is 1.006360183 [0.987416279, 1.064764849]; ON/OFF is 0.993655827 [0.972210529, 1.038989634]. These wide intervals do not establish a 6.5% slowdown or a physical16 gain. Both calibration intervals and all relative RSS guards pass. The two physical16 checks are the only failures; the demonstrated primary32 gain is retained without adopting the candidate or repeating the failed screen.

Diagnostic instruction counts fall about 3.3% at both worker counts versus OFF. Ordinary CPU service also falls: 182.714→177.806 ms/request at 32 and 119.900→116.343 at 16. Peak RSS rises slightly, 133.75→134.25 MiB and 89.50→91.00 MiB. These observations are not a throughput or memory-saving qualification.

## Change

Borrowed G1/G2 Window::flush_queue originally sends all queued contributions to scalar projective accumulators. The candidate first drains when the last SIMD-width queued destinations are distinct and the current pending batch is empty or at least SIMD width. It executes any qualifying pending batch, repeatedly calls the original take_from_queue/execute, then applies the original projective loop to the remainder. At the end of input, the preceding execute has already cleared pending busy bits. At overflow, early execution can release them.

The guard masks the sign bit when comparing destinations and uses the existing native SIMD width. It allocates nothing and adds no Window field, buffer, point copy, key table, sorting, plan or pool. Every successful iteration pops at least SIMD width entries; the queue is smaller than original batch capacity. Identity/equal/opposite inputs keep complete handling and can leave a partial arithmetic batch. This is not a guarantee of full ordinary lanes. Batch fragmentation and additional inversions can outweigh reduced scalar projective work.

Scalar decomposition, widths, query readiness, field arithmetic, Rayon and workspace forecasts remain exact. Existing scratch growth is still bounded by original batch geometry and typed capacity-growth allowances; actual retained capacities/RSS can differ. No minimum-RAM or pressure claim follows. This is distinct from the rejected shared-point windows, mixed binary/triple formulas and component partitioning. [History](artifacts/history-audit.md), [predicate](artifacts/queue_drain.rs), [G1 integration and oracle](artifacts/g1-suffix.rs), [G2 integration and oracle](artifacts/g2-suffix.rs).

## Complete verified request latency

Ryzen 9 9950X,16physical/32logical CPUs. Primary32 uses CPUs0–31; primary16 uses0–15. Fifteen fixed blocks rotate/reverse variant order, with topology order32then16. Accepted is the exact retained SDK, OFF a coherent fresh build, ON the new path. Every process measures151 requests after3primes: fresh solving, randomized proving, SDK verification and serialization are timed; loading is excluded. All9shapes×3variants plus physical16×3 pass1+3preflight and independent Gnark before latency.

CPU is summed user+system service per request, summarized by median process mean. RSS is maximum lifetime process high-water including loading/primes, not minimum RAM. Latency is median of process medians.

|Case|Variant|Median ms|CPU ms/request|Peak RSS MiB|
|---|---|---:|---:|---:|
|1x2-w16|OFF|9.691869|119.900|89.50|
|1x2-w16|ON|9.729750|116.343|91.00|
|1x2-w16|accepted|9.649268|120.078|90.00|
|1x2-w32|OFF|8.069801|182.714|133.75|
|1x2-w32|ON|7.889011|177.806|134.25|
|1x2-w32|accepted|8.035988|183.289|132.00|

|Paired comparison|Median ratio|Bounds|Wins/15|Pass|
|---|---:|---|---:|---|
|1x2-w16/OFF/accepted|0.998556562|[0.976902476, 1.021414098]|8|true|
|1x2-w16/ON/OFF|0.993655827|[0.972210529, 1.038989634]|9|false|
|1x2-w16/ON/accepted|1.006360183|[0.987416279, 1.064764849]|6|false|
|1x2-w32/OFF/accepted|1.004981508|[1.001341395, 1.007805600]|1|true|
|1x2-w32/ON/OFF|0.976444150|[0.972827853, 0.980220282]|15|true|
|1x2-w32/ON/accepted|0.982904529|[0.979309999, 0.985773825]|15|true|

Primary32 requires an upper ON/control ratio below1 and at least10/15wins against both controls. Physical16 requires upper ratios≤1.02, without a strict gain requirement. Both OFF/accepted calibration intervals must fit factor1.03; ON maximum RSS must be≤1.05times each control in both cases. These criteria were declared for this new mechanism before capture and do not reinterpret any earlier failed experiment. Sorted ranks4–12of15 give nominal96.484375%individual median coverage under exchangeable independent blocks, without familywise coverage. No gain floor, exclusions, extensions, threshold changes, successful reruns or subset adoption. A passing pilot still needs separate all9 performance/resource/portable qualification.

## Explicit counters and automatic profiles

After all 90 latency processes, six FIFO-gated grouped counter panels measure 31+3 requests and six fixed-period cycles:u panels 251+3. All remaining independent proof checks and decoding follow captures. No owned compiler, analysis/hash/archive helper or second CPU workload overlaps capture. The original capture/analyzer/publisher/summary helpers compile beforehand; a later publication-only destination correction is disclosed below. The host is shared.

Counter rows are single diagnostic observations, not repeated-trial confidence intervals or an isolated cause. Counts require≥99.9%runningcoverage.

|Panel|Instructions/request, million|Cycles/request, million|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|1381.090|585.717|true|
|stat-1x2-w16-ON|1334.904|570.455|true|
|stat-1x2-w16-accepted|1380.554|582.603|true|
|stat-1x2-w32-OFF|1420.449|827.722|true|
|stat-1x2-w32-ON|1373.982|802.733|true|
|stat-1x2-w32-accepted|1420.457|835.076|true|

Profiles use period 16 million, explicit CPU/time/period and executable mappings, one PID/zero lost records, ≥1,000 samples and ≤5% unknown-period attribution checks. The earlier measured absolute-period undercoverage remains: flat shares are qualitative, not exclusive costs, local CPI or removable wall fractions. The drain method is inlined in the linked SDK: no standalone drain symbol or name-attributed period appears. Zero in the next table therefore does not measure zero drain execution or quantify its cost. Compiled-route correctness assertions pass in native tests.

|Profile|Samples|Drain-method context period|Attribution qualified|Lost|
|---|---:|---:|---|---:|
|cycles-1x2-w16-OFF|9046|0|true|0|
|cycles-1x2-w16-ON|8834|0|true|0|
|cycles-1x2-w16-accepted|8898|0|true|0|
|cycles-1x2-w32-OFF|12584|0|true|0|
|cycles-1x2-w32-ON|12276|0|true|0|
|cycles-1x2-w32-accepted|12620|0|true|0|

## Correctness and provenance

Fresh OFF/ON each pass all127unfiltered native core tests and8prover tests. Three added tests cover the signed-destination/SIMD/pending guards and independent G1/G2 generator bucket sums over pending0/7/8/32, empty/short/full-boundary queues, distinct/repeated/opposite/signed destinations, identity/doubling/cancellation and reuse through repeated drains. Local state assertions check the compiled route. Original complete-MSM tests and all-nine proof preflights remain required. All17field files are exact;33field tests are inherited without a rerun.

Source176keeps172of175originals exact; three cfg-only parent edits and one predicate child recover all175 originals. Window fields/workspace forecasts remain exact. Original function line positions are preserved in OFF. Immediate coherent freezing includes actual flags/features and shared proc macros; the original SDK consumer links frozen dependencies. Source, libraries, inputs and all helper pins close.

All294workflowsteps complete. **15,708 SDK requests** verify internally. **132 fresh164-byte exports** independently pass Gnark, exact public/image bindings, common VK identities per shape and uniqueness against22,366prior identities: ledger**22,498**,264proof/publicmembers. These counts include primes/preflights/diagnostics but no repeated successful work. There is no new AWS/Aeglos, phone/ARM/TEE, giant-circuit, pressure/swap/minimum-RAM, concurrent-throughput or portable qualification.

[Source audit](artifacts/source-audit.json), [final source/linked review](artifacts/final-source-review.json), [complete source](artifacts/source.json.gz), [all observations](artifacts/rows.json), [analysis](artifacts/analysis.json), [capture plan](artifacts/plan.json), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [verification inventory](artifacts/proof-inventory.json), [archive review](artifacts/publication-review.json). Raw perf/binaries/libraries stay local with hashes.

## Inherited CPU and Metal comparison

The original publisher inherited the preceding experiment's destination. Its create-new guard rejected the attempt before writing any file, preserving that report. A separate [publication-only correction](artifacts/publish-fixed.rs) changes only the destination string and compiles after capture. All original pinned helpers, measurements and successful tests/verifications are retained; no runtime work is repeated.

Accepted source175measurements remain unchanged; M5 values are supplied historical references, not new matched-machine captures. The accepted physical16 calibration engineering exception and separate final-artifact qualification remain.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|507.0|7.767×|175.0|2.681×|50.7|

Accepted primary remains8.022165ms/134MiB;54→1remains162.250464ms without a supplied M5reference. LatestAWSc7a.24xlarge remains Zopro11.915591ms at32workers versus AeglosSorted7.959184ms at96. The all-eight CPU10×goal stays active/unmet,**0/8**; no production adoption occurs in this pilot.
