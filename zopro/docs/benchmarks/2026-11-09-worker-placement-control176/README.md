# Fixed worker placement as a measurement control

**Fixed worker placement sharply reduces 16-worker variation in this experiment and improves the same-driver paired latency by 7.14%.** At 32 workers, its paired gain is 2.53%. This is a benchmark-control intervention using the unchanged accepted prover libraries. No arithmetic change or production default is adopted. The overall predeclared control screen fails one 16-worker calibration gate, retained below. The all-eight CPU 10× goal remains active and unmet.

The [initialized-prefix experiment](../2026-11-09-weighted-live-prefix176/README.md) had a supported small 32-worker gain but unresolved 16-worker non-regression. Its existing rows associate slower 16-worker processes with many more involuntary switches, without comparable CPU-work or minor-fault movement. Earlier [placement traces](../2026-10-24-worker-placement/README.md) and [neutral paired controls](../2026-10-27-sdk-paired-control/README.md) already established variation, including within one SDK process. Those diagnostics are not repeated here. The older [32-worker pinning optimization](../2026-10-21-ryzen-research/worker-affinity/README.md) remains a separate failed screen with different libraries and gates.

This intervention compares the exact standard SDK with one new driver in two runtime modes. Both driver modes initialize the existing global Rayon pool before loading, with unchanged default stacks and worker counts. Unbound workers retain the inherited broad process mask; fixed workers each receive one CPU in their start callback. Main keeps the broad mask. Both modes link the exact same eight accepted libraries. There is no additional pool, worker, global CPU-policy change, key/witness/point/scalar copy, execution plan or lookup table. No prover library or field test is rebuilt or rerun.

Local hardware is the Ryzen 9 9950X. At 16 workers, CPUs 0–15 cover all 16 physical cores once; 32 workers use CPUs 0–31. The driver records worker index, TID and actual allowed mask before priming and after timing, and checks exactly N+1 process threads. These broadcasts and setup are outside request timing. The complete SDK solve/prove/internal verification/serialization call remains timed, with fresh randomized proofs.

## Timing and variation

Thirty-one matched rotated/reversed blocks per cohort measure 151 requests per arm after three primes. Every observation remains; there are no exclusions, extensions, rerolls or changed gates. CV is the sample standard deviation divided by the mean of 31 process means. Relative MAD is the median absolute deviation divided by their median. These spread estimates are descriptive, without confidence intervals. Displayed latency is the median of process means; paired geometric ratios use matched blocks and need not equal ratios of these medians.

|Workers / arm|Median ms|Process-mean CV|Relative MAD|Peak process MiB|Involuntary switches/request|
|---|---:|---:|---:|---:|---:|
|16 / standard|10.170208|10.157%|7.230%|91.75|838.616|
|16 / unbound|9.614004|7.061%|2.144%|91.25|327.841|
|16 / fixed|9.269595|0.804%|0.221%|88.25|0.603|
|32 / standard|8.038367|0.785%|0.544%|133.00|225.040|
|32 / unbound|8.068147|0.887%|0.371%|135.75|256.563|
|32 / fixed|7.869279|0.632%|0.295%|132.75|3.417|

At 16 workers, fixed/unbound=0.928643, 95% paired bootstrap 0.906543–0.948956, with 31/31 wins. Its CV is 8.78× lower and relative MAD 9.70× lower. The intervention meets both declared spread-halving conditions. Fixed/standard=0.885763, interval 0.854729–0.916695, also 31/31 wins. The switch reduction is large, but counts alone do not identify migration, yields, cache locality, wakeups or their exclusive wall-time costs. Placement changes those conditions together.

At 32 workers, fixed/unbound=0.974734, interval 0.971901–0.977384, 31/31 wins; fixed/standard=0.980564, interval 0.977411–0.983930, 30/31 wins. Unbound/standard calibration passes at 32 workers. Fixed placement meets both ≤1.02 non-regression bounds in both cohorts, and all RSS screens pass.

**The overall neutral-control support criterion nevertheless fails:** 16-worker unbound/standard=0.953825, interval 0.918297–0.989562, outside the frozen calibration range [1/1.03,1.03]. Explicit pool setup and the standard executable cannot be treated as calibrated equivalents there. The same-binary fixed/unbound intervention supports a useful conditional measurement result; it does not waive that calibration failure, replace standard accepted performance, or qualify the earlier weighted-tree candidate. Its original failed 16-worker gates remain unchanged.

The useful next comparison is accepted libraries versus the exact weighted OFF/ON libraries with common, explicit fixed placement. Both cohorts, all eight shapes plus merge 54→1, and resource/admission/portable/concurrent qualification remain needed before adopting prover code. Pinning is not made a universal prover default or assumed available on phones. Placement alone does not reduce the main arithmetic work enough to meet the full objective.

## Proofs and automatic profiles

All 12 preflights pass: primary and merge 54→1 at both worker counts in all three arms. There are 186 latency processes and 12 later PMU/profile processes, followed by 210 independent Gnark verifications and 30 offline ownership/decode/event/metadata/loss reads. All 450 commands, the one driver link, one prospective review and two result analysis/review commands complete successfully on their first executions. Existing field/core/prover qualification is inherited with byte-identical accepted libraries; there is no new native arithmetic qualification claim.

All recording precedes external verification and offline analysis. Only the confirmed-live handle is polled during recording; no owned build/helper/hash/analysis/report/push overlaps it. Total SDK requests, including primes: **31,920**. All **210 fresh unique exported proofs** have matching image/VK/public/proof bindings and pass pinned independent Gnark. The canonical inventory advances 37,864→38,074. The same-agent result reader reconstructs raw means, spread, resources, schedule, declared gates, placement cohorts and proof bindings. Bootstrap, binding and detailed profile parsers are shared; this is not an external audit.

All 140 driver processes retain identical before/after worker TIDs and affinity masks, unique indices and N+1 tasks. In the two fixed profiles, all sampled worker TIDs execute on their assigned CPUs: 17,802 and 25,270 worker samples. The main thread accounts for the remaining 59 and 62 samples. This validates observed fixed placement, not every unsampled scheduling event.

All six profiles pass: 128,313 samples total, 100% assigned execution, 1.26–1.74% unknown, zero LOST. All 24 grouped PMU event values run 100% of enabled time. Sampling uses cycles:u with period 16,000,000 and 512 mmap pages, 501 timed requests per profile; grouped PMU uses 31 requests. FIFO gates exclude priming.

|Workers / arm|Instructions/proof, billions|Cycles/proof, millions|User CPU ms/proof|Kernel CPU ms/proof|Minor faults/proof|
|---|---:|---:|---:|---:|---:|
|16 / standard|1.349129|568.396|113.863|2.220|58.146|
|16 / unbound|1.344072|548.595|116.014|2.194|61.192|
|16 / fixed|1.348641|569.934|116.582|1.915|56.470|
|32 / standard|1.389823|818.274|174.685|5.017|128.252|
|32 / unbound|1.389504|804.890|175.008|4.999|129.364|
|32 / fixed|1.385888|809.267|175.105|4.562|131.026|

Instructions and summed user CPU stay roughly unchanged; the 16-worker fixed PMU panel even has more cycles than unbound. One counter panel per condition does not provide repeated-counter uncertainty or causal/exclusive cycle fractions. Timed major faults are zero in both driver modes; the standard SDK does not expose that field. Process RSS includes loading, worker and allocator state and is not minimum RAM or memory-pressure/phone/swap/TEE qualification.

## Accepted CPU / Metal comparison

Only standard accepted primary at 32 workers is fresh here. The remaining seven shapes are inherited from the preceding accepted observations. The fixed-placement 7.869279 ms is a different measurement condition and is not substituted into this standard accepted table. M5 values are supplied references on different hardware.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1x2|8.038367|37 / 4.603×|24 / 2.986×|3.7|
|2x2|11.149097|59 / 5.292×|30 / 2.691×|5.9|
|4x2|16.054442|109 / 6.789×|48 / 2.990×|10.9|
|8x2|24.131178|189 / 7.832×|74 / 3.067×|18.9|
|12x2|33.940628|284 / 8.368×|101 / 2.976×|28.4|
|16x2|40.164083|335 / 8.341×|124 / 3.087×|33.5|
|24x2|66.187704|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.361571|507 / 7.757×|175 / 2.677×|50.7|

Merge 54→1 retains its inherited 160.890372 ms observation without an M5 reference; the new runs add correctness only. Historical same-standard-build primary 7.915095 ms remains valid. Fresh 8.038367 ms does not establish an adopted regression; fixed-placement 7.869279 ms and the preceding unbound weighted candidate 8.018331 ms must not be compared as matched arithmetic results. All eight CPU 10× targets remain unmet, 0/8. No fresh AWS/Aeglos/Gnark latency, concurrent-proof throughput or cloud resource is added.

[Analysis](artifacts/analysis.json), [profiles/counters](artifacts/diagnostic-analysis.json), [result review](artifacts/review.json), [prospective review](artifacts/plan-review.json), [protocol](artifacts/protocol.json), [design](artifacts/design.md), [driver](artifacts/driver.rs), [placement callbacks](artifacts/driver-extra.rs), [accepted references](artifacts/accepted-references.json), [continuation loop](artifacts/next.md), [source/logs/receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS). The evidence archive retains individual UTF8-member SHA256 hashes and verified gzip roundtrip. Executables/libraries, perf.data, keys/witnesses/proofs and full private ledger payloads are excluded; verifier results and proof hashes remain. Reproduction requires pinned dependencies/inputs and adapted absolute paths.
