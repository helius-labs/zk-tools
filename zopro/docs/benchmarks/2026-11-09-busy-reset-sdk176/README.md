# Bitmap reset: complete SDK comparison

The bitmap-reset candidate passes all **79 declared checks**. Primary 32-worker paired latency improves 0.63% against fresh OFF and 0.56% against accepted REF; the all-eight aggregate improves 0.22% and 0.74%, respectively. The result selects remaining source/resource/admission/portable/concurrency qualification before adoption. Production is unchanged, and the CPU 10× goal remains unmet on all eight shapes.

REF is the unchanged original-field accepted SDK with the fixed-placement driver. OFF/ON reuse the exact qualified cores from the [native Window study](../2026-11-09-busy-reset176/README.md) and coherently rebuild prover/zolana/SDK dependents. Only ON enables the fixed busy-bitmap reset. Both use the original field, native O3, fat LTO, one codegen unit, driver, allocator and worker placement. Source/link checks recover all 176 accepted originals. No field arithmetic, inversion, queue geometry, solver/FFT, configuration lifetime, forecast or resource policy changes. There is no new buffer, retained table/plan, point/key copy, pool or memory cap.

Each SDK request includes witness preparation/solving, fresh randomized proving, internal verification and serialization. Loading, pool creation, three primes and final export are outside latency. Every process exports its final proof/public inputs for independent Gnark verification after all recording. Primary at 16/32 workers uses 31 paired blocks of 151 requests; other conditions use 15 blocks of 31 requests, except primary solo with 11. Case and arm orders rotate/reverse. No observation is excluded, extended or repeated.

|Case/workers|REF ms|OFF ms|ON ms|ON/OFF paired ratio [95%]|Wins|ON/REF paired ratio [95%]|Wins|
|---|---:|---:|---:|---|---:|---|---:|
|1x2-w32|7.832281|7.863572|7.809034|0.993735 [0.991839, 0.995545]|27/31|0.994435 [0.992589, 0.996313]|26/31|
|2x2-w32|10.984795|10.997725|10.940974|0.996438 [0.992363, 1.000648]|10/15|0.993713 [0.990456, 0.996981]|12/15|
|4x2-w32|15.987627|15.781970|15.750370|0.997005 [0.991123, 1.002230]|9/15|0.984411 [0.978606, 0.990740]|12/15|
|8x2-w32|24.249130|23.989433|23.951790|0.995514 [0.987344, 1.002870]|9/15|0.986740 [0.978126, 0.994347]|12/15|
|12x2-w32|34.174517|33.869454|33.857141|1.001254 [0.994628, 1.008331]|7/15|0.990781 [0.981901, 1.000319]|12/15|
|16x2-w32|40.245136|39.815716|39.726526|1.001457 [0.992640, 1.010267]|8/15|0.991775 [0.982742, 1.001770]|11/15|
|24x2-w32|66.278990|65.985093|65.984808|0.999755 [0.995612, 1.003766]|6/15|0.995397 [0.988765, 1.001926]|11/15|
|merge-24x1-w32|65.347397|65.865698|65.660588|0.997620 [0.992385, 1.002316]|8/15|1.003801 [0.998983, 1.008483]|5/15|
|merge-54x1-w32|161.934390|162.240572|163.167792|1.004030 [1.000199, 1.008005]|4/15|1.004018 [0.998418, 1.009210]|6/15|
|1x2-w1|95.634225|95.348134|94.301186|0.990984 [0.988906, 0.993056]|15/15|0.986432 [0.983382, 0.989069]|15/15|
|1x2-w16|9.243506|9.251633|9.179981|0.992318 [0.990905, 0.993712]|31/31|0.991610 [0.990083, 0.993138]|31/31|

Displayed time is the median of process arithmetic means of emitted SDK samples rounded to integer nanoseconds. Paired geometric ratios use corresponding blocks and 20,000 log-bootstrap resamples. Ratios of marginal medians need not equal paired estimates. Intervals assume exchangeable blocks and provide no familywise guarantee. The equal-log all-eight aggregate independently resamples within each shape, with 31 primary and 15 other blocks; it is not synchronized cross-shape sampling.

Primary ON/OFF is 0.993735 [0.991839, 0.995545], with 27/31 wins; ON/REF is 0.994435 [0.992589, 0.996313], with 26/31. The all-eight aggregates are 0.997844 [0.995738, 0.999929] against OFF and 0.992616 [0.990235, 0.994977] against REF. The fresh-control aggregate passes narrowly, under the original rule without a minimum percentage gain. Individual shapes need not all improve.

The tradeoff is retained: merge54 is 0.403% slower against OFF, ratio 1.004030 [1.000199, 1.008005], with 4/15 wins. Its accepted-reference ratio is 1.004018 [0.998418, 1.009210]. Both remain within the declared 3% nonregression bound. No favorable shape subset is selected.

All 11 OFF/REF calibration, 22 ON/control nonregression, 22 RSS, four primary gain/win, two aggregate gain and 18 diagnostic checks pass. The full result adds 635 fresh unique independently Gnark-verified exports: 26 preflight plus 609 main-run exports. Canonical inventory advances **41,104 → 41,130 → 41,739**. There are 44,210 SDK requests including primes; those requests are not independent exported proofs. All proof/image/public bindings, 164-byte lengths and freshness checks pass. This SDK stage required no source or reader repair; the native parent's test-only fixture repair is inherited, not repeated.

Maximum whole-process primary32 RSS is REF/OFF/ON **134.25/133.25/132.50 MiB**. Primary16 is 88.50/89.50/87.75 MiB. Not every shape has a lower peak: 4x2 ON is 319.25 MiB versus REF 306.75; 16x2 ON 834.75 versus REF 810.00. Every declared 5% RSS check passes. These peaks include image loading, primes and allocator state and do not establish minimum RAM, incremental scratch, stack peaks or memory-pressure behavior. Logical buffers and forecasts are unchanged.

All nine grouped PMU panels meet 99.9% running coverage. All nine fixed-period profiles qualify: **119,686 samples**, minimum 8,869, maximum unknown 2.151%, zero lost records, and checked actual executable/PID/TID/CPU ownership. Recording uses acknowledged FIFO warm gating, cycles:u period16M, 512 mmap pages, CPU/time/period/IP and no callgraph. Absolute period coverage is unvalidated. Flat sample shares are not wall-time critical-path fractions.

The single primary32 PMU panels show REF/OFF/ON user cycles 814.329/807.873/806.085 million and instructions 1386.992/1385.630/1369.339 million per proof. Cache misses are 2.683/2.680/2.909 million. These descriptive panels have no replication intervals and do not establish a unique cache, bandwidth or scheduling cause for the gain.

Six actual downstream links and four prospective helper builds complete before 26 functional SDK preflights and their 26 Gnark checks. The main workflow then completes all 1263 commands: 591 latency processes, 18 instrumented processes, 45 offline profile commands and 609 Gnark verifications. All recording precedes offline work. A separate same-agent reader checks 602 capture pins, source/link identities, all receipts, raw process means, proof bindings and all 79 decisions. It shares bootstrap/aggregate statistics and PMU/profile parsers; this is not an external audit. Native Window and generic-compile qualification are inherited without replay.

The following fresh local 32-worker figures compare the candidate with supplied historical M5 CPU and Metal references on different hardware. M5 was not remeasured. Candidate timings are not yet adopted production results. None meets the CPU 10× requirement.

|Shape|REF CPU ms|Candidate CPU ms|M5 CPU ms|M5 Metal ms|Candidate/CPU speedup|Candidate/Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|7.832281|7.809034|37|24|4.738×|3.073×|3.7|
|2x2|10.984795|10.940974|59|30|5.393×|2.742×|5.9|
|4x2|15.987627|15.750370|109|48|6.920×|3.048×|10.9|
|8x2|24.249130|23.951790|189|74|7.891×|3.090×|18.9|
|12x2|34.174517|33.857141|284|101|8.388×|2.983×|28.4|
|16x2|40.245136|39.726526|335|124|8.433×|3.121×|33.5|
|24x2|66.278990|65.984808|528|176|8.002×|2.667×|52.8|
|merge-24x1|65.347397|65.660588|507|175|7.722×|2.665×|50.7|

The accepted prover was already below 8 ms: the prior matched reference was 7.832708 ms; historical 7.699867 ms used a different campaign and median of process medians. This is a supported small full-proof gain, not a new sub-8-ms milestone or a twofold breakthrough. No new AWS/Aeglos, ARM/phone/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput measurement is added.

[Results](artifacts/analysis.json), [protocol](artifacts/protocol.md), [raw-data review](artifacts/review-result.json), [prospective review](artifacts/prospective-review.json), [preflight review](artifacts/preflight-review.json), [proof inventory](artifacts/proof-inventory.json), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Executables/libraries, unchanged full source snapshots, raw perf recordings, key/image/witness/proof bytes and full historical proof ledgers remain private. Large decoded logs are hash-indexed locally; summaries, readers and command receipts are archived.
