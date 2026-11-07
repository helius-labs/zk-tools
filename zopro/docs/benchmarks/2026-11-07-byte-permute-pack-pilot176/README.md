# Byte-permutation packing: first complete-SDK pilot

The candidate is **1.33% slower than fresh OFF** and **1.20% slower than accepted** in paired complete-proof latency. Calibration passes, both gain checks fail, and the implementation closes without adoption. The prospective 31-block study retains every observation; no additional blocks or favorable subset follow.

## Implementation and qualification

The [native report](../2026-11-07-byte-permute-pack-native176/README.md) describes the new eight-lane AoS-to-radix52 constructor. Fixed AVX512VBMI byte permutations replace word transpose/shift assembly using the same four input loads. Six selector vectors occupy 384 logical bytes, plus scalar mask/immediate constants. Input API, field arithmetic/radix, point layout, scratch, keys, solver, scheduling and pools stay unchanged. The feature fallback retains the original constructor.

The source operation count falls 22→17 before loads, constant setup, moves and register allocation. Actual linked G1/G2 consumers instead gain instructions and stack operands, with unchanged IFMA counts. The shared inverse grows slightly and the two inspected FFT column consumers retain their metrics. Those static tradeoffs were preserved before timing; whole-function size alone was not a rejection gate.

Inherited native qualification includes the exhaustive 2,048-input-bit basis, 196,928 raw lane outputs, 65,536 constructor roundtrips and 32,768 Arkworks products per configuration/tag population, 125 OFF/125 ON core tests and eight ON prover tests. Repeated tags/configurations are not distinct vectors. The original test-oracle endian repair and supplemental FFT direction selection remain documented. No successful test, model, symbol listing or decoder is repeated in this campaign. All ten saved consumer decodes are reused.

Fresh independent Gnark preflight passes all nine shapes and all three executables. Production and the accepted SDK remain unchanged.

## Complete proof latency

Ryzen 9 9950X, 32 logical workers, process affinity CPUs 0–31. Thirty-one fixed rotated/reversed process blocks compare accepted/OFF/ON. Each process makes three primes and 151 measured complete SDK requests, including fresh proving, verification and serialization. All latency precedes its independent verification and all counters/profiles. No other owned CPU workload overlaps recording; exclusive host use is not claimed.

|Arm|Median process mean, ms|Median process maximum RSS, MiB|Largest process maximum RSS, MiB|
|---|---:|---:|---:|
|Accepted|8.010996|130.50|134.00|
|Fresh OFF|7.988174|130.25|135.25|
|ON|8.083878|130.75|135.25|

|Paired comparison|Median ratio|Interval|Wins / 31|Check|
|---|---:|---:|---:|---|
|OFF / accepted|0.995700546|0.990120140–1.004112327|18|Calibration passes|
|ON / OFF|1.013348504|1.007789025–1.019221868|2|Gain fails|
|ON / accepted|1.012011700|1.005697498–1.017535870|5|Gain fails|

Ratios pair process means by block and need not equal ratios of displayed medians. Sorted ranks 10–22 provide nominal **97.05506265% marginal median coverage**, conditional on independent, exchangeable blocks, without a familywise guarantee. Gain requires upper ratio below one and at least 21 wins against both controls. OFF/accepted must remain within factor 1.03. No percentage gain floor applies.

The block count and endpoints were frozen before this candidate's first clock. They do not extend or replace the previous uniform-prefix trial. No exclusions, threshold changes, additional blocks, worker/layout/compiler sweep or curve-only promotion occur. The failed primary pilot selects no all-shape performance extension or source adoption.

## Automatic CPU diagnostics

After latency, each arm runs one 31-request user-counter panel and one 251-request user-cycle profile, each after three primes. FIFO warm-region gates exclude loading/primes. Sampling period is 16 million cycles with 512 mmap pages. Every counter event reports 100% running time.

|Counter per complete SDK request|OFF|ON|ON / OFF|
|---|---:|---:|---:|
|Cycles|813,763,645|826,618,900|1.015797|
|Instructions|1,389,790,527|1,408,423,789|1.013407|
|Generic cache misses|2,723,084|2,940,167|1.079720|
|Branch misses|906,482|899,463|0.992257|

The single panels support increased whole-proof instruction/cycle work, consistent with the latency regression. They lack repeated-panel confidence intervals and do not isolate byte-permutation latency, a cache level, DRAM, register spills or an exclusive critical path. Fewer source operations do not imply less emitted or executed work; this result concerns the fixed implementation, not every byte-permutation strategy.

Accepted/OFF/ON profiles contain **12,309 / 12,431 / 12,431 samples**, with **1.66% / 1.90% / 1.71% unknown symbols** and zero recorded LOST events. Process/thread/CPU, event/period and metadata checks pass. Saved SDK decodes are reused; no new exact IP/ELF mapping is claimed. Flat samples include inlined work and are not removable wall-time fractions.

RSS includes loading/primes. The observed process high-water values are not minimum RAM, complete allocation bounds or qualification for phones, ARM, concurrent proofs, giant circuits or pressure/swap. No new heap buffer, key-dependent table or synthetic RAM cap is introduced.

## Verification and provenance

Preflight completes 108 SDK requests including primes and 27 fresh independent Gnark exports. Latency adds **14,322 requests/93 exports**; diagnostics add **864 requests/six exports**. Total: **15,294 requests and 126 fresh independently verified exports**. Canonical proof ledger advances **31,777 → 31,804 → 31,897 → 31,903**. Proof/public/image/VK bindings and export freshness pass.

All campaign phases complete once on their first executions. The outcome-neutral reader checks 404 input pins, 213 post-preflight capture/verification/offline receipts and all 99 post-preflight fresh bindings, reconstructing raw means, paired ranks/gates and counter/profile totals. Its final workflow adds five own-source/executable/finished-analysis pins, 409 total. It is not a second cryptographic verifier, disassembler, LOST decoder or exact IP/ELF analyzer. No successful capture or qualification is repeated for bookkeeping.

The same coherently linked OFF/ON SDKs inspected in the native stage run throughout. The prospective source explicitly adapts all block counts, median/rank endpoints, request/export totals and ledger counts to 31 blocks; diagnostic top-15 symbol summaries remain unchanged. Frozen protocols, sources and exact receipts accompany the data.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker complete-SDK observations and supplied M5 references follow. These are not all-shape timings of this candidate. All eight CPU 10× requirements remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.915095|37 / 4.675×|24 / 3.032×|3.7|
|2x2|11.169812|59 / 5.282×|30 / 2.686×|5.9|
|4x2|16.085882|109 / 6.776×|48 / 2.984×|10.9|
|8x2|24.517520|189 / 7.709×|74 / 3.018×|18.9|
|12x2|33.838626|284 / 8.393×|101 / 2.985×|28.4|
|16x2|40.083672|335 / 8.358×|124 / 3.094×|33.5|
|24x2|65.506198|528 / 8.060×|176 / 2.687×|52.8|
|merge-24x1|65.200982|507 / 7.776×|175 / 2.684×|50.7|

Merge54 remains 161.718956 ms without a supplied M5 reference. Accepted production176/SDK6d83 remain unchanged. No new AWS/Aeglos, large-circuit, device, pressure or concurrent-proof comparison is added. Goal remains active and unmet, 0/8.

[Artifacts](artifacts/) retain changed source versions, frozen designs, 31-block protocol/helpers, all latency samples, counter/profile summaries, plans, hashes and exact text/JSON receipts/reviews. Archives are compared with originals. Actual keys/images, requests, witnesses, proof/public-input payloads, executables/libraries and raw perf data remain private.
