# SIMD bucket admission: complete-proof comparison

**The candidate is slower and is not adopted.** Primary32 measures **7.903204 ms**, versus **7.784088 ms fresh OFF** and **7.770239 ms accepted REF**. Paired ON/OFF is **1.020248572 [1.010576591,1.023997665],0/15 wins**; ON/REF is **1.015006100 [1.011298390,1.026893274],1/15**. All control calibrations pass. Twelve of77 declared checks fail; the fixed recipe is closed without a rerun or parameter rescue.

The [native qualification](../2026-11-10-simd-bucket-admission176/README.md) implemented collective admission for eight ordinary G1/G2 bucket updates. It rejects collisions, pending destinations and conservative first-X-limb exceptions, then appends eligible inputs to the original borrowed batch. Rejected packets retain original scalar transitions and use their already-extracted digits. Ordinary G1, balanced K/H and G2 feeds change; batch/queue boundaries, arithmetic, digits, windows, solver, FFT, allocator and pool stay original. No table, key copy, solver plan, new heap buffer or architectural RAM cap is added. Correctness did not imply speed.

The130 native tests,704 new curve-window cases/45,056 Arkworks bucket outputs,66,626 predicate cases, source reversals, ISA-disabled compile and actual optimized SDK links are inherited, not rerun. Current176 production files remain exact. REF is the unchanged accepted SDK7aff; OFF/ON are the exact qualified b08a/692e consumers, executed at their existing paths without copying or rebuilding. Both retain original field1200 and dense busy reset; only ON enables packet admission. The driver is identical, with actual O3/native/fat-LTO/one-CGU links.

## Fixed complete-proof results

Ryzen9950X,16 physical/32 logical CPUs, shared host. All eight supplied shapes plus merge54 run at32 workers; primary also runs at1/16. Solo pins to CPU2; otherwise worker i pins to CPUi, with default worker stacks. Solving, fresh randomized proving, internal verification and serialization are timed. Key loading, pool creation, three primes and final proof export are excluded. These are complete SDK requests, not an admission microbenchmark or server-throughput test.

**Stack-setting correction:** the original report and frozen protocol incorrectly stated that SDK workers requested 512 KiB stacks. The actual pinned SDK driver calls `ThreadPoolBuilder` without `stack_size`, and its launch environment has no `RUST_MIN_STACK` setting. The explicit 512 KiB request applied to native qualification pools. The SDK driver, recordings, timings and decisions are unchanged; no measurement was repeated for this correction. The [following complete-proof trial](../2026-11-10-retained-tail-sdk176/README.md) uses the same default-stack driver.

Fifteen fixed matched blocks rotate/reverse contexts and arms; each process measures31 requests. Values are medians of arithmetic process means of integer-rounded SDK nanoseconds. Paired median ratios need not equal quotients of displayed medians. Bounds use zero-based sorted ranks3/7/11 of15, nominal96.484375% marginal coverage under exchangeable independent blocks, without familywise or stationarity guarantees. There are no excluded blocks, added samples, favorable subsets, changed thresholds or unchanged recaptures.

|Context|REF ms|OFF ms|ON ms|ON/OFF [bounds], wins|ON/REF [bounds], wins|
|---|---:|---:|---:|---|---|
|1x2-w32|7.770239|7.784088|7.903204|1.020248572 [1.010576591,1.023997665],0/15|1.015006100 [1.011298390,1.026893274],1/15|
|2x2-w32|10.966498|10.881392|11.097811|1.022315192 [1.016149853,1.026233207],2/15|1.013950204 [1.006901109,1.020122026],1/15|
|4x2-w32|15.761756|15.770570|15.856565|0.999137921 [0.994273932,1.007992165],8/15|1.000507590 [0.996437990,1.004027257],7/15|
|8x2-w32|23.933998|23.935388|24.109130|1.007258788 [0.999787559,1.011883906],4/15|1.005926782 [1.001058500,1.010436543],3/15|
|12x2-w32|33.607895|33.829588|33.902003|1.005035571 [0.995974093,1.012402251],6/15|1.007482103 [1.002347814,1.013216006],2/15|
|16x2-w32|39.794609|39.845047|40.003675|1.005098812 [0.999033013,1.013856979],4/15|1.006159285 [0.998819467,1.017203315],4/15|
|24x2-w32|66.142308|66.092124|66.182458|0.998842336 [0.996785023,1.006793060],8/15|1.002621715 [0.993873475,1.009841183],6/15|
|merge-24x1-w32|65.271702|65.576743|66.056834|1.003851029 [0.998502146,1.012634889],5/15|1.007739319 [1.002542129,1.012750140],2/15|
|merge-54x1-w32|162.486028|161.964544|163.112164|1.006094319 [1.001796031,1.012927394],2/15|1.001610849 [0.995610490,1.012101553],6/15|
|1x2-w1|94.257637|93.846001|96.361427|1.027898338 [1.023534678,1.031704299],0/15|1.023979961 [1.019195381,1.028003588],0/15|
|1x2-w16|9.013311|9.002803|9.235241|1.027195651 [1.024040254,1.036578809],0/15|1.026831573 [1.017584557,1.029526154],0/15|


The primary displayed ON/OFF medians differ by119.1 µs; that difference and the2.025% paired slowdown summarize different distributions. Equal-log ALL8 ON/OFF is **1.006421726 [1.005047318,1.009719220],0/15**; ON/REF is **1.007198825 [1.004366005,1.010190710],1/15**. Both aggregate intervals are above parity.

The fixed gate requires11 OFF/REF calibrations within factor1.03,22 ON/control nonregression upper bounds≤1.02, primary32 and ALL8 gain bounds<1 with≥10/15 wins against both controls,22 RSS guards and18 diagnostic quality checks. All calibrations, RSS and diagnostic checks pass. The failures are eight primary1/16/32 and2×2 nonregressions, both primary gain checks and both ALL8 gain checks. Failed nonregression bounds are retained; a bound above1.02 is not by itself a precisely measured slowdown of that amount. Primary slowdown and ALL8 slowdown are separately supported by intervals above parity.

## CPU profiles and memory

All nine grouped-counter panels run at≥99.9% scheduled coverage. All nine fixed16M-cycle profiles pass PID/TID/CPU/executable-MMAP/event checks,≥1000 samples,≤5% unknown and zero LOST. There are **120,025 samples**, with maximum unknown share below2.06%. Absolute period coverage is unvalidated. Panels are single descriptive measurements, not replicated causal estimates.

|Context|ON/OFF instructions|ON/OFF cycles|ON/OFF branch misses|ON/OFF generic cache misses|ON G1 admission samples|
|---|---:|---:|---:|---:|---:|
|1x2-w32|1.027058|1.026279|1.195828|1.150858|422|
|1x2-w16|1.029650|1.024792|1.233421|1.167750|341|
|2x2-w32|1.028008|1.024888|1.180304|1.134201|581|


The G1 admission function appears in ON’s actual proof profiles and is absent from both controls. Its samples include useful dispatch work as well as added checks; they cannot be subtracted as removable overhead. G2’s changed admission is inlined into the existing feed function. Instructions, user cycles and branch misses all increase against OFF in these panels. The intended reduction in scalar dispatch work does not produce a net instruction or proof-latency saving. The measurements do not isolate a particular comparison, gather, spill, cache level or page-fault service as the cause. No new profiling is needed to repeat this fixed negative.

|Context|REF / OFF / ON peak RSS MiB|REF / OFF / ON CPU ms per proof|REF / OFF / ON minor faults per proof|
|---|---:|---:|---:|
|1x2-w1|34.25 / 34.25 / 34.25|94.255 / 93.843 / 96.359|0.000 / 0.000 / 0.000|
|1x2-w16|81.25 / 79.25 / 79.75|115.857 / 115.729 / 118.609|205.581 / 207.065 / 199.613|
|1x2-w32|111.00 / 107.25 / 110.25|178.040 / 177.556 / 181.699|408.774 / 408.613 / 421.645|
|2x2-w32|272.75 / 268.25 / 275.50|258.154 / 256.558 / 261.514|1282.355 / 1226.194 / 1306.355|
|24x2-w32|1296.75 / 1292.74 / 1285.52|1926.899 / 1924.649 / 1931.083|4245.194 / 6199.645 / 4179.290|
|merge-54x1-w32|1991.16 / 1999.43 / 1980.28|4523.259 / 4514.795 / 4540.898|22155.742 / 22418.290 / 22192.935|


All22 comparative RSS guards pass: ON maximum≤1.05×control maximum+1 MiB. Process peaks include loading, primes, allocator history and retained buffers. They are not minimum RAM, a whole-program stack bound, pressure/swap qualification or a universal memory reduction. The driver reports minor faults but not major faults; counts do not measure fault service time or establish disk paging. Fixed admission arrays remain stack-local; this trial creates no input-sized table or retained cache.

## Verification and provenance

All44 functional-preflight commands succeed:22 OFF/ON SDK processes covering all nine shapes and primary1/16, then22 independent Gnark verifications. They contribute88 internally verified requests and22 unique fresh exports. Their clocks select no performance result. The canonical inventory advances44,265→44,287 only after verification and hash-freshness checks.

All **1,071 capture commands** complete once:495 latency processes,18 diagnostic processes,45 offline commands and513 independent Gnark verifications. Capture adds19,422 internally checked requests and513 unique exports. The stage totals **19,510 requests /535 fresh exported proofs**, each with image/public/proof bindings. Proofs are164 bytes. The canonical inventory reaches **44,800**. Export count differs from the number of all internally generated proofs. The private key/request/witness/proof payloads remain local.

The producer and separate same-agent raw reader are built and frozen before the first preflight. Both post-capture executions pass on their first runs. The reader reconstructs495 raw integer process means, medians/rank ratios,33 resource summaries and all77 decisions, checks444 final capture pins and source recovery, and reconfirms the513 exported-proof bindings/freshness records. SDK structure and detailed PMU/profile parsers are shared; this is not an external audit or replay of cryptographic verification.

One setup-helper compile fails on borrowing a moved String before any SDK request. A separate source fixes the borrow and adds its own source pin; all five continuation steps pass. The original source, failed stderr and plan remain archived. No native build, test, successful capture, proof verification or decoder is rerun for that repair. Source and timing gates do not change. During preflight and capture, only the exact confirmed live handles are polled until terminal; no owned build/helper/hash/analysis/report overlaps recording.

The accepted SDK/source remain unchanged. No new AWS/Aeglos, ARM/mobile/TEE, huge250/4999, minimum-RAM, pressure/swap or concurrent-throughput result is claimed. The CD-disabled compilation inherited from qualification does not constitute an older-CPU or mobile execution test.

## Supplied CPU and Metal comparisons

Fresh local accepted REF and unadopted ON proof observations follow. M5 CPU and Metal timings were supplied separately and are not remeasured; ratios do not establish equal hardware efficiency.

|Shape|Accepted REF ms|Unadopted ON ms|M5 CPU ms / REF speedup / ON speedup|M5 Metal ms / REF speedup / ON speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|7.770239|7.903204|37 / 4.762× / 4.682×|24 / 3.089× / 3.037×|3.7|
|2x2|10.966498|11.097811|59 / 5.380× / 5.316×|30 / 2.736× / 2.703×|5.9|
|4x2|15.761756|15.856565|109 / 6.915× / 6.874×|48 / 3.045× / 3.027×|10.9|
|8x2|23.933998|24.109130|189 / 7.897× / 7.839×|74 / 3.092× / 3.069×|18.9|
|12x2|33.607895|33.902003|284 / 8.450× / 8.377×|101 / 3.005× / 2.979×|28.4|
|16x2|39.794609|40.003675|335 / 8.418× / 8.374×|124 / 3.116× / 3.100×|33.5|
|24x2|66.142308|66.182458|528 / 7.983× / 7.978×|176 / 2.661× / 2.659×|52.8|
|merge-24x1|65.271702|66.056834|507 / 7.768× / 7.675×|175 / 2.681× / 2.649×|50.7|


Accepted primary is now **7.770239 ms**, the same executable previously observed at7.783594 ms. No replacement or new sub-8-ms milestone is adopted. Merge54 REF is162.486028ms without a supplied M5 reference. Both accepted and candidate meet **0/8** CPU10× targets. The original verified CPU AVX≥10×M5 CPU objective remains **active and unmet**.

This closes the eight-input admission recipe. A further implementation must remove materially different main work or dependencies; changing packet width or tuning the same mask path is not selected by these results. Keep the qualification and this full-proof negative together. No production default changes.

[Results](artifacts/analysis.json), [raw reader](artifacts/review-result.json), [decision](artifacts/decision.json), [fresh CPU/Metal comparisons](artifacts/fresh-comparison.json), [diagnostic summary](artifacts/diagnostic-summary.json), [protocol](artifacts/protocol.md), [identities](artifacts/identities.json), [inherited link review](artifacts/link-review.json), [preflight inventory](artifacts/preflight-review.json), [prospective review](artifacts/prospective-review.json), [proof inventory summary](artifacts/proof-inventory.json), [commands](artifacts/receipts.json), [continuation](artifacts/next.md), [public bindings](artifacts/public-views.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). The indexed UTF-8 archive retains full raw rows, helper versions, commands/streams, decoded profiles and new-export hash bindings. Executables/libraries, raw binary perf, private inputs/outputs, unchanged snapshots and full canonical-ledger payloads are excluded.
