# On-demand FFT preparation: full SDK results

**The standalone column gain does not transfer to full proofs.** For 1×2 at 32 workers, accepted/control/candidate medians are **8.013551 / 7.984872 / 8.092982 ms**. Paired candidate/accepted is **1.009522 [1.005782, 1.013462]** and candidate/control **1.011733 [1.008139, 1.015534]**: about 1% slower with calibrated controls. Across the original eight shapes, equal-log paired ratios are **1.004290 [1.001258, 1.006670]** versus accepted and **1.003901 [1.002012, 1.005598]** versus control. **Do not adopt this candidate.**

The experiment integrates the exact qualified root constructor and fixed-factor product into the actual packed FFT column kernel. Only its gap-one stage changes: prepare one root, use it for 16 butterflies and discard it; the other 64 products are ordinary. Both directions and root layouts are enabled. The factor payload is 640 bytes plus arithmetic temporaries; no retained table, key/plan copy, new buffer, heap allocation or production pool is added. Native guards exclude this path on unsupported targets. The generic whole-prover check with no_asm/no_ifma and the experiment flag passes. This is build qualification, not an ARM/mobile or minimum-RAM result.

The [native column experiment](../2026-11-07-fft-column-prep-native176/README.md) measured a 9.31% intended threaded aggregate gain, but failed four latency checks and profile symbol quality. Those failures remain intact. A separately recorded engineering exception selected this full-SDK test before its build and clocks, using the same both-direction/layout implementation. Its negative proof result closes that recipe; no forward-only selection, worker/layout/compiler reroll or favorable-shape promotion follows.

Measurements run locally on an AMD Ryzen 9 9950X, 16 physical cores/32 logical CPUs, Rust 1.98.1, optimized native CPU builds and a fat-LTO/one-codegen-unit SDK link. Accepted binary is 6d83f1d6…, fresh original control 64bfa94a…, candidate 387d9382…. Candidate links eight coherent libraries and the unchanged request driver/allocator. Original scalar/native field and component qualifications are inherited without replay. Fresh integration preflight passes all nine shapes at 32 workers plus 1×2 at 1/16 workers, for both original control and candidate: 88 SDK requests and 22 independently Gnark-verified exports.

The fixed full-proof matrix uses 15 rotated/reversed paired blocks, accepted/control/candidate, all nine shapes at 32 workers and 1×2 at 16 workers. Each process performs three primes, then 151 measured requests for 1×2 or 31 for other shapes. All 450 latency runs finish before 18 automatic diagnostics (stat and user cycles for 1×2 at 32/16 and 24×2 at 32). Every recording finishes before 468 independent export verifications and 45 offline commands. No owned build, hashing, analysis, reporting or push overlaps recording. All 981 commands pass.

Times are median process arithmetic means in milliseconds. Ratios and 95% intervals use paired block means, 20,000 predeclared SplitMix64 resamples. RSS is the maximum whole-process high-water reading across latency processes, including setup.

|Shape / workers|Accepted ms|Control ms|Candidate ms|Candidate/accepted (95% CI)|Candidate/control (95% CI)|Accepted / candidate RSS MiB|
|---|---:|---:|---:|---|---|---:|
|1x2 / 32|8.013551|7.984872|8.092982|1.009522 [1.005782, 1.013462]|1.011733 [1.008139, 1.015534]|136.50 / 133.75|
|2x2 / 32|11.254041|11.215208|11.329081|1.004789 [0.998698, 1.010060]|1.008106 [1.003737, 1.012566]|275.25 / 272.75|
|4x2 / 32|15.963789|16.081629|15.999175|1.002355 [0.997871, 1.006600]|0.998038 [0.992702, 1.003376]|312.00 / 318.50|
|8x2 / 32|24.269224|24.145599|24.504083|1.007028 [0.994322, 1.019131]|1.015755 [1.003797, 1.027495]|512.50 / 513.75|
|12x2 / 32|33.884727|33.928124|33.960243|1.001744 [0.996047, 1.007463]|0.996540 [0.989162, 1.002755]|711.75 / 711.25|
|16x2 / 32|40.199590|40.297841|40.307201|1.002614 [0.996363, 1.008354]|1.000561 [0.994660, 1.006675]|816.75 / 838.50|
|24x2 / 32|65.690345|66.392121|65.994985|1.005263 [0.997581, 1.012557]|0.997615 [0.991949, 1.003397]|1283.25 / 1292.75|
|merge-24x1 / 32|65.555031|65.337461|65.528959|1.001037 [0.995108, 1.005722]|1.003043 [0.998309, 1.007770]|1153.43 / 1177.00|
|merge-54x1 / 32|161.737128|161.787641|161.522020|1.000939 [0.997913, 1.004218]|0.997419 [0.992828, 1.002101]|1824.85 / 1888.90|
|1x2 / 16|9.688970|9.667219|9.590842|1.015891 [0.972429, 1.061190]|1.001006 [0.948643, 1.057249]|91.25 / 91.25|

The full latency gate fails eight checks: primary gain against both controls; 8×2 candidate/control nonregression; 16-worker control calibration and candidate nonregression against both controls; and both all-eight aggregate gain checks. Calibration requires the full CI inside [1/1.03, 1.03], nonregression upper bounds <=1.02, and primary/aggregate gain upper bounds <1. No positive gain floor, exclusion, early stopping, extension or reroll is applied. The 16-worker intervals are broad and fail calibration; its lower candidate median is not an established improvement. Small favorable estimates against control in 4×2, 12×2, 24×2 and merge54 have intervals crossing parity and are retained without promotion.

All nine full-proof profiles meet the declared quality checks: **90,029 samples**, zero LOST, >=1,000 samples each, unknown symbols <=5%, valid owned binary MMAP/PID/TID/CPU/event/period/FIFO checks and sampled actual FFT leaves. Grouped PMU panels meet >=99.9% running time. These new profiles qualify independently; they do not repair the failed standalone profiles.

The original-control 1×2/32 profile assigns **230/12,330 samples (1.87%)** to the changed column leaves; 24×2 assigns **1,714/8,829 (19.41%)**. Thus the changed leaves are a small part of primary sampled CPU work. This is a scope/priority observation, not a precise wall-time ceiling. Full-proof symbol attribution is saved, including dominant MSM, inversion and scheduling work. Candidate leaf shares are 258/12,505 and 1,794/8,862. The SDK leaves grow from 15,569/15,612 bytes (inverse/forward) to 25,295/25,119; static IFMA sites grow 200→614 across whole functions including layout branches. More code and a different optimizer/access context can matter, but these measurements do not isolate why the standalone gain disappears.

Single PMU panels are mixed: primary32 candidate instructions are about 1.387 billion/request versus control 1.389 billion, with cycles near parity; primary16 cycles increase; large24×2 instructions and cycles increase. They are diagnostics, not a replicated causal decomposition. The candidate adds no source-level heap/table, while observed RSS varies by shape and allocator history. Primary32 reads 133.75 MiB candidate versus 136.50 accepted; merge54 reads about 1,889 MiB candidate versus 1,825 accepted. These are process peaks, not minimum RAM, proof-only allocations or a demonstrated memory-pressure advantage. Minor faults are retained per request in the summary; they are page mappings/first-touch events, not evidence of swap I/O or a sole latency cause.

All 28,006 new SDK requests include the 88 integration-preflight requests. **490 fresh exported proofs** (22 preflight +468 capture) independently pass Gnark with image/VK/public bindings and uniqueness against the canonical ledger, advancing **33,449→33,939**. Only the exported final proof from each process is independently checked by Gnark. Other requests exercise the SDK's verification path. Review reconciles 1,028 workflow receipts, four new SDK leaf decodes, all 176 accepted-source recoveries, 26,514 capture samples, paired point estimates, aggregate weights and profile counts. It does not replay successful tests, recordings, verifiers, decoders or bootstrap resampling. The initial preparation helper assumed UTF-8 for binary fixtures; recovery was repaired using byte hashes before builds, retaining the original failure and snapshot.

Comparisons below use the supplied M5 references; they are cross-machine elapsed-time ratios, not equal-hardware efficiency. The original CPU 10× goal remains active and unmet: **0/8 shapes**, for accepted and candidate alike. Merge54 has no supplied M5 reference.

|Shape|Accepted / candidate ms|M5 CPU / Metal ms|Candidate speedup vs CPU / Metal|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|8.014 / 8.093|37 / 24|4.572× / 2.966×|3.7|
|2x2|11.254 / 11.329|59 / 30|5.208× / 2.648×|5.9|
|4x2|15.964 / 15.999|109 / 48|6.813× / 3.000×|10.9|
|8x2|24.269 / 24.504|189 / 74|7.713× / 3.020×|18.9|
|12x2|33.885 / 33.960|284 / 101|8.363× / 2.974×|28.4|
|16x2|40.200 / 40.307|335 / 124|8.311× / 3.076×|33.5|
|24x2|65.690 / 65.995|528 / 176|8.001× / 2.667×|52.8|
|merge-24x1|65.555 / 65.529|507 / 175|7.737× / 2.671×|50.7|

The accepted implementation stays unchanged. Its primary was previously below 8 ms and now measures 8.013551 ms in this matched run, using the same binary; the cause of inter-run variation is unestablished. No AWS/Aeglos, phone, huge-circuit, swap or low-RAM execution is measured here. The next experiment must target dominant full-proof work with a distinct work-removal hypothesis, rather than another isolated column variant.

[Latency summary](artifacts/latency-summary.json), [diagnostics](artifacts/diagnostic-analysis.json), [profile attribution](artifacts/profile-attribution.json), [review](artifacts/review.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.json), [source/plan/pin/raw-receipt archive](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
