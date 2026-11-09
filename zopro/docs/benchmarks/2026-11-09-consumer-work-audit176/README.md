# Consumer work audit from retained CPU profiles

**The rejected indexed-wide schedule adds substantial non-product work, while accepted MSM kernels also spend samples outside multiply instructions.** This offline audit adds no optimization, timing run, proof, or adoption. Accepted full-proof1×2 remains **7.857170ms**; the all-eight CPU10× goal remains unmet.

Five existing symbol profiles are annotated at instruction addresses: the [indexed-wide comparison](../2026-11-09-indexed-wide-query176/README.md) OFF/ON G2 reducer, accepted full-proof primary G1/G2 kernels, and accepted merge54's forward FFT leaf. Original profile quality is inherited. The separate reader checks **10,172 sample counts and42,121 instruction rows**, plus source/ELF identities, source-level work and all-eight target gaps. [Analysis](artifacts/analysis.json), [review](artifacts/review.json).

## Work excluded by the field-product model

Across primary's288ordinary cells, the source-shaped census has1,057joint reduce_layer calls. Each call makes two complete scans of1,024group descriptors: pair selection and final compaction. That is **2,164,736group-entry visits**, before short-circuit while tests, final output loops, merging, weighting and carry work. The corresponding population has266,955generic pair operations. These counts describe source loops and generic geometry; they are not measured instruction retirements.

Primary also has502,909signed input-point copies, representing64,372,352bytes of logical point payload. Each leaf requests1,024identity outputs and511weighted slots:294,912+147,168slots across the ordinary windows, or56,586,240bytes of logical initialization payload. The grouped2×2 population has893reduction calls,1,828,864descriptor visits and519,007copies.

These are source operations and initialization semantics, **not actual executed stores, DRAM traffic, peak allocation or RSS**. Compiler elimination, allocator zeroing and cache reuse matter. The previous typed capacity forecast concerned simultaneous ownership, a different quantity. The field-product model explicitly excluded routing, scans and copies; its5%product reduction was never a complete CPU-work estimate.

The OFF and ON reducers each have13,453decoded instruction rows and the same static mnemonic histogram. This does not prove byte identity, equal paths or dynamic instruction frequency. Their sampled populations differ630→2,321. The candidate changed how much and what kind of work reaches the same source kernel; this audit does not isolate a single causal bottleneck or reopen the rejected geometry.

## Instruction-address samples

Each table entry counts cycle samples landing on a mnemonic class, relative to the selected symbol only. IFMA means vpmadd52luq/huq. Moves include register spills and many data purposes; shifts/shuffles include field carry and radix work. Other vector instructions include arithmetic, comparisons and masks. **These are not packing-only costs, removable wall fractions, or retired instruction frequencies.** Sampling skid and inlining limit attribution.

|Selected symbol|Samples|IFMA|Memory move|Shuffle/shift|Other vector|Control|
|---|---:|---:|---:|---:|---:|---:|
|indexed-reduce-OFF|630|19.05%|28.41%|6.35%|18.89%|1.27%|
|indexed-reduce-ON|2321|24.77%|24.82%|8.10%|20.03%|5.08%|
|accepted-primary-G1|2632|40.24%|7.22%|14.48%|31.91%|0.15%|
|accepted-primary-G2|2005|42.79%|9.33%|14.66%|27.93%|1.35%|
|accepted-merge54-FFT|2584|6.04%|53.29%|8.48%|28.29%|0.19%|

Register moves and other scalar instructions make up the unshown remainder. In the accepted primary G1/G2 hot symbols, IFMA samples are40.23%/42.79%, not their complete costs. The selected merge54FFT leaf has a different instruction mix, with many samples on memory moves; that alone does not prove DRAM saturation or a memory-bound critical path. A faster isolated multiply or fewer abstract products cannot be treated as a proportional full-proof gain.

The accepted primary profile contains2,632G1stream and2,005borrowedG2 samples among12,700total. Their sample shares cannot be subtracted from full-proof latency as exclusive time. The earlier [coarse phase diagnostic](../2026-11-06-coarse-proof-phase-universal176/README.md) failed its combined observer qualification at16workers. Its callbacks can help one another through Rayon, so their wall spans are not additive costs; the32worker subchecks do not override the failure. The earlier [causal-profiler configuration](../2026-11-08-causal-profiler176/README.md) also failed qualification and supplies no accepted critical-path estimate.

## Implications for the goal

The remaining wall-time reductions are much larger than recent component gains. These calculations use unchanged accepted local full-proof measurements and supplied M5 references, not new measurements.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|Required wall reduction|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|52.91%|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|46.62%|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|31.67%|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|21.51%|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|16.50%|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|16.52%|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|20.42%|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|22.98%|

Primary needs52.91%less wall time and2×2needs46.62%. No new implementation is selected by this audit. The next investigation must account for complete consumer dataflow and machine work across the dominant G1/G2 paths, while keeping FFT costs relevant for larger shapes. An inverse-only or product-only proxy is insufficient. Retained profiles and source/DWARF data can support this work without another timing run.

Prior GLV, triple/common-Z, layout/packing, independent-chain, alternate-field and indexed/ring negatives remain closed; the [stream work census](../2026-11-06-stream-work-census176/README.md) and recorded source history cover many of them. This audit does not claim an unseen algorithm, new speedup or a way to remove all sampled overhead. The universal memory constraint remains: no large persistent lookup table, duplicate key/solver plan, new pool or artificial RAM cap.

## Reproduction and limits

The offline tool invokes perf annotate on retained data and reads its integer sample column. One exploratory ON annotation was displayed with truncation before the saved count view; that saved view is reused. An initial display command combined mutually exclusive sample/period flags and exited129before decoding; corrected counts-only output succeeds. Original notes remain. No performance recording, prover/field build, native test, SDK request or proof is repeated. The audit and separate reader compile and execute successfully on their first attempts.

The reader independently reparses every annotated address/mnemonic/count, checks annotation row and summary totals, checks IFMA counts and class conservation, recomposes both structural populations and all target reductions, and verifies all176production hashes plus the retained accepted profile executable's frozen hash. Source counts inherit the qualified scalar census and do not observe new exceptional branches. The input pins are posthoc offline evidence bindings, not a new prospective performance protocol.

Production176/SDK6d83 and the40,154verified-proof ledger remain unchanged. Full all-eight plus merge54 SDK/Gnark/resource/portable/concurrency validation remains required for adoption. Goal active and unmet,0/8. No new AWS/Aeglos, mobile/ARM, huge-circuit, pressure, swap, minimum-RAM or throughput result follows. No job or cloud resource remains live.

[Continuation](artifacts/next.md), [tool notes](artifacts/tooling-notes.md), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
