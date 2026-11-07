# Borrowed G2 bucket trees: native qualification and SDK pilot

The balanced fill tree is **slower in complete primary proofs**. Paired latency is **3.20% worse than fresh OFF** and **4.38% worse than accepted**, with no winning blocks against either. Calibration passes. The fixed implementation is closed without adoption, an unchanged reroll or a favorable worker/layout/platform subset.

## Algorithm and qualification

The [work screen](../2026-11-07-borrowed-bucket-tree176/README.md) models 43.22%/48.18% fewer ordinary inversion requests on primary/2×2 while preserving logical generic joins. Those are subcomponent counts, not whole-proof savings.

Ten ordinary Phi2 G2 fills route signed u32 indices into the existing 256-bucket groups. Each group reduces high-component leaves as balanced trees, applies the original endomorphism to roots, then reduces low leaves seeded by those roots. Key leaves stay borrowed; two compact buffers and descriptor storage are reused. Generic batches pad to eight lanes with valid pairs in distinct inactive outputs. Identities, opposites and doubling use complete curve operations before batching. Original grouping, two point partitions, 12-bit digits, narrow indexed top, merges and weighting remain.

This adds temporary routing, point moves and padding. No persistent table, key copy, duplicate solver plan, new pool, new unsafe operation or field representation is added. A query-level resource reservation charges additional requested capacities using checked worst-case bounds and the current worker count; the original forecast remains a floor. Failed admission selects the original path before large workspace allocation. This uses the existing OS-headroom policy, not a new fixed RAM cap.

Final source has 178 files: 175 exact accepted parents, one reversible dispatch parent and two children. All 18 field files and original G2 streaming arithmetic remain exact. Corrected native qualification passes **130 OFF and 130 ON core tests**, including five new tests covering all padding sign masks, tails, cancellation/identities/doubling, skew, repeated reuse, immutable inputs, full-query generator-coefficient oracles and 1/4/16/32-worker pools with 512 KiB stacks. Two prover-library and six cubic tests pass. These core oracles share accepted field/curve arithmetic; they are not an independent Arkworks implementation.

Coherent eight-library SDKs use O/native/fat LTO/one codegen unit. Actual final inspection finds identical selected G1 metrics (4,428 instructions/800 IFMAs/4,288-byte prologue) and G2 metrics (8,591/1,450/5,952) across OFF/ON. The selected recursive ON tree bridge reserves 16,600 bytes; append/execute reserve 552/216. OFF excludes tree text. Individual frames and native pool tests are not a formal whole-call-chain stack bound.

## Complete SDK latency

Ryzen 9 9950X, 32 logical workers pinned to CPUs 0–31. Fifteen fixed rotated/reversed blocks; each process performs three primes and 151 measured complete SDK requests. All latency finishes before verification, counters or profiles. No other owned CPU workload overlaps recording. Every block is retained.

|Arm|Median process mean, ms|Median process maximum RSS, MiB|Largest observed process maximum RSS, MiB|
|---|---:|---:|---:|
|Accepted|7.975947|130.75|133.00|
|Fresh OFF|8.050707|131.00|134.50|
|ON|8.332573|133.00|135.50|

|Paired comparison|Median ratio|Interval|Wins of 15|
|---|---:|---:|---:|
|OFF/accepted|1.007745|1.000852–1.018160|3|
|ON/OFF|1.031954|1.026559–1.037058|0|
|ON/accepted|1.043837|1.032485–1.046714|0|

Ratios pair process means by block and need not equal ratios of displayed medians. Sorted ranks 4–12 provide nominal 96.484375% marginal median coverage conditional on independent, exchangeable blocks, without a familywise guarantee. ON must have an upper ratio below 1 and at least 10 wins against both controls. OFF/accepted bounds must lie within [1/1.03, 1.03]. Both gain checks fail and calibration passes. No gain floor, exclusions, extensions or post-result rescue applies.

## Diagnostics and conclusion

After latency, each arm runs one 31-request user-counter panel and one 251-request user-cycle profile, with three primes and FIFO warm gates. Sampling period is 16 million cycles with 512 mmap pages. All counters report 100% running.

|Counter per request|OFF|ON|ON/OFF|
|---|---:|---:|---:|
|Cycles|816,917,724|874,785,111|1.070836|
|Instructions|1,389,029,346|1,445,255,861|1.040479|
|Cache misses|2,809,684|3,981,766|1.417158|
|Branch misses|912,417|1,207,995|1.323950|

These single diagnostic panels support increased whole-proof CPU work; they do not isolate causality or a memory level. Reducing inversion requests leaves the logical point-join count unchanged and adds routing, exception checks, point moves and padding. The modeled reduction is insufficient to establish lower total work or latency.

Accepted/OFF/ON profiles contain 12,406/12,375/13,174 samples, zero recorded LOST events and 1.52%/1.66%/2.25% unknown symbols. ON records 520 samples in the tree bridge, 487 in append and 294 in routing, confirming execution of the new path. Flat scalar Fq-inverse samples fall from 894 OFF to 839 ON; those totals are not inversion-call counts. Sample shares are not exclusive wall time or removable critical-path fractions. No exact IP/ELF mapping or causal DRAM attribution is claimed.

All nine shapes and three arms pass independent Gnark preflight: 108 SDK requests including primes and 27 fresh exports. Latency adds 6,930 requests/45 exports; diagnostics add 864 requests/six exports. **Total: 7,902 requests and 78 fresh independently verified exports.** Canonical unique-proof count advances 31,621→31,699. The independent reader checks 226 pins, 117 capture/verification/offline receipts, 51 post-preflight proof bindings, paired gates and diagnostic totals. It is not a second cryptographic verifier or LOST decoder.

Observed process maximum RSS grows modestly here. It is not minimum RAM or qualification for phones, ARM, giant circuits, pressure/swap or concurrent throughput. No AWS/Aeglos remeasurement is added.

## Preparation repairs and provenance

The initial native OFF build succeeds and four new tests pass, then the 512 KiB pool test aborts with stack overflow. The inspected recursive producer reserves 110,728 bytes per frame while returning large point arrays. The corrected implementation returns compact slot indices and writes roots into existing output storage; stack limits, arithmetic, worker counts and geometry stay fixed. New OFF/ON suites qualify that runtime repair. The failed source, executable hash and receipts remain preserved.

First SDK symbol selection then finds retained tree code in OFF behind a runtime boolean branch. Explicit compile-time guards isolate the final control before any timing. Removing those guards reproduces the qualified ON parent byte-for-byte; the other 177 files are exact. A new 130-test OFF suite checks the changed control; qualified ON core/prover suites are inherited rather than replayed. Fresh coherent final libraries and links receive their own inspection and proof preflight.

The first isolation reviewer mistakes an xtask parser flag for raw test filtering on env-wrapped commands. Its repair reads the retained raw summaries: two/six passed, zero failed/ignored/filtered. No tests are replayed for that metadata repair. The initial failed symbol selector starts no disassembly commands. Seven final consumer decodes run once and are reused after timing. Unchanged mathematical models, field qualifications and successful proof/capture stages are not replayed for bookkeeping.

All failed stages precede performance capture. Their repairs do not change the frozen performance gate. Accepted production and SDK remain unchanged.

## Accepted CPU and Metal comparisons, unchanged

These inherited local 32-worker full-SDK observations use the exact accepted SDK. The supplied M5 CPU/Metal references are not new Apple measurements or measurements of this candidate. All eight CPU 10× targets remain unmet.

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

Accepted merge54 remains 161.718956 ms without a supplied M5 reference. The goal remains active and unmet, 0/8.

[Artifacts](artifacts/) retain source versions, helpers, designs, plans, pins, analyses and raw text/JSON/CSV receipts. Archives are compared with originals; checksums cover published files. Actual keys/images, requests, witnesses, proofs, public inputs, executables/libraries and raw perf data remain private.
