# Adopt G2 scalar-grain bound after C7a qualification

The exact experimental candidate is **adopted** after **all forty prospective checks pass** across thirteen C7a contexts. At 96 workers, 1×2 improves **5.74% paired** versus accepted, uses **13.29% less CPU**, and wins all 31 blocks. Median process-mean latency is **12.764646 ms versus 13.593476 ms**, with **121.79 MiB** maximum lifetime RSS. At 32 workers the paired latency gain is 0.86%; the best tested candidate median is **11.314984 ms**. This is a useful incremental gain; the full all-eight CPU 10× goal remains unmet.

The adopted runtime file is byte-exact to the earlier experimental source. Its existing `zopro_phi_grain` branch adds `with_min_len(128)` to the in-place G2 `phi_split` iterator. A separate narrow core build-script change registers/emits the cfg under the existing native x86_64 + ifma8 + AVX-512 IFMA/VL/DQ selector without no_ifma. No runtime-body cleanup or new prover/SDK build is substituted. Of 176 production files, 174 remain exact and two change explicitly. Arithmetic, order, representations, allocation capacities, workspace forecasts, ABI and Cargo.lock are preserved; no buffer, lookup table, key plan, pool, shared mutable state, unsafe code or architectural RAM cap is added.

PARENT is the exact retained SDK `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`; REF is accepted `d5e1b2c981fb9bcab2beb69e3b2af5608d298d792c7e3814e87273b04509662e`. Both use the original untraced fixed-placement driver, coherent eight-library builds, Rust 1.98.1, znver4, O3, fat LTO, one codegen unit and the accepted manual flags. The performance artifact remains that exact hand-linked SDK. Ten finite selector contexts and an actual Cargo core check confirm default phi activation without an explicit phi flag, with dense/GPU selection preserved. This is default-selection/type evidence, not a newly timed Cargo release SDK, following the [previous busy-reset promotion](../2026-11-09-busy-reset-adoption176/README.md). The [accepted manifest](artifacts/production-source-manifest.json) records the mapping and all source hashes.

|Context|Build|Proof ms|Whole-loop CPU ms/request|Maximum lifetime RSS MiB|
|:---|:---|---:|---:|---:|
|12x2-w64|REF|34.065482|1694.303|706.35|
|12x2-w64|PARENT|34.079904|1665.323|706.06|
|16x2-w64|REF|39.001713|1912.055|852.12|
|16x2-w64|PARENT|38.788170|1875.559|854.49|
|1x2-w16|REF|16.227177|209.977|85.90|
|1x2-w16|PARENT|16.208512|209.685|83.50|
|1x2-w32|REF|11.454285|250.281|129.36|
|1x2-w32|PARENT|11.314984|247.346|127.80|
|1x2-w64|REF|11.476940|433.958|129.29|
|1x2-w64|PARENT|11.332097|413.744|129.20|
|1x2-w96|REF|13.593476|775.805|120.32|
|1x2-w96|PARENT|12.764646|673.335|121.79|
|24x2-w64|REF|55.004891|2640.009|1350.33|
|24x2-w64|PARENT|54.414317|2598.645|1313.41|
|2x2-w32|REF|15.222006|342.935|269.48|
|2x2-w32|PARENT|15.193170|341.159|259.02|
|2x2-w96|REF|18.242888|1092.138|258.25|
|2x2-w96|PARENT|17.359101|970.440|258.18|
|4x2-w64|REF|19.182293|882.468|301.98|
|4x2-w64|PARENT|18.959916|857.537|307.21|
|8x2-w64|REF|26.162033|1329.002|507.05|
|8x2-w64|PARENT|26.191684|1304.896|499.14|
|merge-24x1-w64|REF|57.750907|2612.846|1205.21|
|merge-24x1-w64|PARENT|57.355090|2560.951|1197.57|
|merge-54x1-w64|REF|123.219319|4743.789|2180.35|
|merge-54x1-w64|PARENT|123.137881|4698.491|2133.20|

Latency is the median of 31 process arithmetic means, with 151 timed requests per 1×2 process and 31 per other-shape process after three primes. Timers include fresh assignment, solving, blinding, proving, SDK verification and serialization; loading, primes, export and external verification are excluded. CPU sums user and system time across threads, including ordinary between-call loop work. RSS is the maximum across process lifetime peaks, including loading and primes, not minimum RAM. Default SDK stacks and allocator settings are retained. Process and individual worker masks are checked before and after each process.

The [protocol](artifacts/protocol.md) was frozen before the first new SDK preflight. It covers nine shapes at prior best counts plus 1×2 at 16/64/96 and 2×2 at 96, using only the retained candidate and accepted baseline. Context order rotates/reverses and arm order reverses on odd blocks. Ratios pair the same context and block. Sorted ranks 9/21 of 31 ratios give nominal 97.055062651634% marginal coverage under independent exchangeable blocks, without joint-confidence or stationarity guarantees. All thirteen latency and thirteen CPU upper bounds must be ≤1.02, all thirteen RSS maxima must satisfy PARENT ≤ REF×1.05+1 MiB, and primary96 latency upper must be below one. There is no gain floor, exclusion, result-dependent extension, threshold change or subset rescue. Paired ratios/savings and displayed medians use different aggregation.

|Context|Metric|Lower ratio|Median ratio|Upper ratio|Wins / 31|Pass|
|:---|:---|---:|---:|---:|---:|:---|
|12x2-w64|latency|0.993215|0.998952|1.002517|18|yes|
|12x2-w64|CPU|0.980045|0.981834|0.983057|31|yes|
|16x2-w64|latency|0.988057|0.995513|0.998686|23|yes|
|16x2-w64|CPU|0.977744|0.982406|0.986932|31|yes|
|1x2-w16|latency|0.997770|0.998653|1.001788|20|yes|
|1x2-w16|CPU|0.997674|0.998648|0.999736|26|yes|
|1x2-w32|latency|0.982097|0.991375|0.995351|27|yes|
|1x2-w32|CPU|0.985392|0.988485|0.995781|29|yes|
|1x2-w64|latency|0.977493|0.983723|0.999319|23|yes|
|1x2-w64|CPU|0.948218|0.951921|0.960342|31|yes|
|1x2-w96|latency|0.934115|0.942633|0.955246|31|yes|
|1x2-w96|CPU|0.860927|0.867149|0.871092|31|yes|
|24x2-w64|latency|0.978760|0.991050|1.001585|20|yes|
|24x2-w64|CPU|0.975276|0.984607|0.996380|26|yes|
|2x2-w32|latency|0.993630|0.997101|1.000394|20|yes|
|2x2-w32|CPU|0.991992|0.993138|0.997204|26|yes|
|2x2-w96|latency|0.942701|0.948952|0.955607|31|yes|
|2x2-w96|CPU|0.886327|0.889724|0.897059|31|yes|
|4x2-w64|latency|0.985448|0.989603|0.993513|26|yes|
|4x2-w64|CPU|0.969024|0.972916|0.977256|30|yes|
|8x2-w64|latency|0.990787|0.998819|1.001905|18|yes|
|8x2-w64|CPU|0.974683|0.978032|0.982427|31|yes|
|merge-24x1-w64|latency|0.984995|0.995414|1.001200|21|yes|
|merge-24x1-w64|CPU|0.975859|0.982663|0.987925|27|yes|
|merge-54x1-w64|latency|0.990809|0.997933|1.005357|17|yes|
|merge-54x1-w64|CPU|0.976852|0.989111|0.996336|24|yes|
|1x2-w96|primary96_gain|0.934115|0.942633|0.955246|31|yes|

All thirteen RSS guards pass. The previously uncertain primary64 latency now has an upper ratio of 0.999319 and merge54 has 1.005357, both within the fixed nonregression bound. Primary64's paired gain is 1.63%; 2×2 at 96 workers improves 5.10% paired. These are fresh prospective results. The earlier [12-check primary campaign](../2026-11-11-phi-grain-c7a176/README.md) remains valid; the [separate unconditional default build](../2026-11-11-phi-grain-clean176/README.md) remains failed at 58/66 and unadopted. Neither its gates nor its nine blocks were changed. Retrospective control analysis supplied motivation, not retroactively passed adoption gates.

|Shape|Candidate workers|Candidate ms|Peak RSS MiB|Supplied M5 CPU ms|CPU speedup|Supplied M5 Metal ms|Metal speedup|10× CPU target ms|
|:---|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2|32|11.314984|127.80|37|3.270×|24|2.121×|3.7|
|2x2|32|15.193170|259.02|59|3.883×|30|1.975×|5.9|
|4x2|64|18.959916|307.21|109|5.749×|48|2.532×|10.9|
|8x2|64|26.191684|499.14|189|7.216×|74|2.825×|18.9|
|12x2|64|34.079904|706.06|284|8.333×|101|2.964×|28.4|
|16x2|64|38.788170|854.49|335|8.637×|124|3.197×|33.5|
|24x2|64|54.414317|1313.41|528|9.703×|176|3.234×|52.8|
|merge-24x1|64|57.355090|1197.57|507|8.840×|175|3.051×|50.7|

Worker selection is descriptive among tested counts, not a proven global optimum. Candidate primary32/64 medians are close, 11.314984/11.332097 ms; this does not establish a significant ordering between those counts. Merge54 at 64 workers is PARENT 123.137881 ms / 2,133.20 MiB versus REF 123.219319 ms / 2,180.35 MiB, without a supplied M5 reference. All eight CPU 10× targets remain unmet, 0/8. No fresh Aeglos measurement is made. The earlier pinned 7.902802 ms primary96 result in the [previous all-shape AWS comparison](../2026-11-10-accepted-current-c7a176/README.md) remains historical context; this campaign does not demonstrate beating it. No local-machine speedup is newly measured.

Native 127 core, two prover and six cubic tests, generic x86-64/noIFMA/noASM 102 core and two prover tests, and prior all-nine functional SDK coverage are inherited after hash checks. The unchanged field qualification is inherited. Source review of Rayon's minimum-grain producer and the pure disjoint mutable map preserves accepted functional concurrency qualification: two shared-image cases and one mixed-key case in a single 32-worker pool. No new concurrent workload or throughput claim is made. Ten finite selector contexts plus an actual Cargo native core check qualify default activation. Post-adoption review checks all 176 production hashes, exact measured runtime source, the two explicit edits and preserved originals; promotion adds no proof or performance capture. The manifest's primary timing is normalized to the existing numeric `ms` field; this is metadata only.

Local harness preflight makes eight calls and two fresh independently Gnark-verified 164-byte exports. Target preflight plus main makes 57,268 calls and 832 fresh exports, all externally verified on the server. After size/SHA-bound retrieval, all 832 are independently verified once more locally without increasing the distinct count. The post reader checks 54,772 timed samples, exact argv/environment, stream hashes, placement, means, bindings and freshness. The adoption reader independently reproduces all forty gates from producer summaries; publication checks 1,664 server command receipts and 832 transport-verification receipts. Combined new work is **57,276 calls / 834 fresh verified exports**; the canonical verified-export ledger advances **50,752 → 50,754 → 51,586**.

The target is one c7a.24xlarge, 96 physical cores without SMT. The network-disabled container has 96 CPUs, 30 GiB memory with no swap, PID limit 2,048, dropped capabilities and no-new-privileges. These are harness settings, not prover requirements. All major-fault counts are zero; CPU-throttle, memory-pressure/OOM and swap-event counters do not increase. Minor faults are preserved in the resource rows. Physical-host exclusivity and frequency locking are not established. No phone/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result is added. During every SDK preflight/capture only the exact live handle is polled; no owned helper/build/hash/analysis/report overlaps. All main captures precede main-export verification.

The owned VM is terminated, its encrypted delete-on-termination volume and temporary ECR tag are absent, and the private transfer object/bucket and secret URL file are deleted. No live temporary benchmark remains in the checked region. The 90-minute termination timer stayed active. A cloned publication guard initially expected the older local count and rejected before AWS mutation; it was corrected to the frozen two-export/eight-call protocol without repeating SDK work or changing the image. Its legacy success text still says 11/44; actual validated image metadata is 2/8. Fresh pre-launch region-wide benchmark and security-group observations were not executed by the cloned launcher. Post-run observations confirm zero temporary benchmarks and no ingress rules, and are explicitly labeled post-run rather than presented as missing pre-launch receipts. This operational deviation is retained with the scientific results; future launches now validate and save those observations before resource creation. The lifecycle version actually executed is archived separately from the corrected helper. An adoption helper delimiter error was corrected before any source mutation. Earlier preparation failures and their receipts remain in the preparation archive. These are harness/helper failures, not failed proofs or relaxed performance gates.

The next research step uses this adopted source/artifact as the baseline and targets the remaining full-proof coordination gap. Small verified improvements are retained, while no twofold or full-goal breakthrough is claimed.

[Decision](artifacts/decision.json), [accepted manifest](artifacts/production-source-manifest.json), [post-adoption review](artifacts/post-adoption-review.json), [all forty checks and all-eight comparisons](artifacts/target-analysis.json), [resource rows](artifacts/resource-rows.json), [protocol](artifacts/protocol.md), [proof/sample review](artifacts/remote-review.json), [quality](artifacts/target-quality.json), [cleanup](artifacts/cleanup.json), [target publication checks](artifacts/target-publication.json), [raw target results, sources and public proofs](artifacts/target-results-sources-public-proofs.tar.gz), [preparation/source/default-selection review](artifacts/preparation-publication.json), [preparation sources and receipts](artifacts/preparation-sources-receipts-public-proofs.tar.gz), [retrospective review](artifacts/retrospective.json), [checksums](SHA256SUMS). Archives round-trip exact bytes and hashes; executables/libraries, private key images/inputs, credentials and unfiltered container environment are excluded. Reproduction requires pinned dependencies and private inputs.

Local root: `/home/ubuntu/src/zopro-perf-work/phi-grain-parent176`; adopted performance SDK: `/home/ubuntu/src/zopro-perf-work/phi-grain-c7a176/sdk-candidate-znver4`.
