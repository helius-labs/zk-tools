# Default G2 scalar-grain qualification on C7a

The default-build candidate is **not adopted**: **58 of 66 fixed checks pass**, with all 66 required. At 96 workers, 1×2 improves 3.16% paired versus the accepted SDK but is 2.27% slower paired than the earlier experimental build. Production remains unchanged. All fresh exported proofs verify, and all thirteen peak-RSS guards pass.

This follow-up tests the actual unconditional `with_min_len(128)` change to the existing in-place G2 `phi_split` iterator, without its experimental cfg. The retained candidate has 175 exact original files and one reversibly changed file. Arithmetic, item ordering, scalar/point representations, allocations, forecasts, ABI, Cargo.lock and worker pools are unchanged. The edit restricts Rayon partition granularity; it adds no lookup table, key preparation plan, heap buffer or runtime selector.

REF is the accepted Zen4 SDK `d5e1b2c981fb9bcab2beb69e3b2af5608d298d792c7e3814e87273b04509662e`. PARENT is the earlier qualified experimental build `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`. CLEAN is the new default build `96915d56cec6fc3479e155001ac2f2e69acf13895a454a765172bbe6d9bfe22e`. PARENT is an algorithmic control, not a newly rebuilt original iterator. All three use the original untraced accepted SDK driver. The [preceding experiment](../2026-11-11-phi-grain-c7a176/README.md) remains valid for its own binaries and protocol; its result does not automatically qualify this different default artifact.

|Context|Build|Proof ms|Whole-loop CPU ms/request|Maximum lifetime RSS MiB|
|:---|:---|---:|---:|---:|
|12x2-w64|REF|34.051144|1700.220|703.29|
|12x2-w64|PARENT|33.963240|1667.509|702.31|
|12x2-w64|CLEAN|34.158605|1674.638|707.96|
|16x2-w64|REF|38.794493|1907.873|830.81|
|16x2-w64|PARENT|39.129346|1888.268|821.26|
|16x2-w64|CLEAN|38.663417|1880.672|841.10|
|1x2-w16|REF|16.206219|209.965|83.45|
|1x2-w16|PARENT|16.207738|210.110|82.79|
|1x2-w16|CLEAN|16.213291|209.591|83.72|
|1x2-w32|REF|11.436294|249.837|126.41|
|1x2-w32|PARENT|11.324571|247.861|125.48|
|1x2-w32|CLEAN|11.406619|250.370|125.54|
|1x2-w64|REF|11.379093|432.608|127.77|
|1x2-w64|PARENT|11.596753|418.221|128.43|
|1x2-w64|CLEAN|11.555326|416.810|127.12|
|1x2-w96|REF|13.545724|773.781|120.35|
|1x2-w96|PARENT|12.802860|676.106|124.07|
|1x2-w96|CLEAN|13.146839|681.813|124.84|
|24x2-w64|REF|54.147074|2621.148|1285.23|
|24x2-w64|PARENT|54.339839|2595.445|1292.34|
|24x2-w64|CLEAN|54.593465|2592.784|1292.62|
|2x2-w32|REF|15.271225|343.311|259.96|
|2x2-w32|PARENT|15.166862|341.238|266.61|
|2x2-w32|CLEAN|15.192123|341.317|252.88|
|2x2-w96|REF|18.335000|1093.108|244.72|
|2x2-w96|PARENT|17.349968|969.389|249.90|
|2x2-w96|CLEAN|17.401036|975.079|250.71|
|4x2-w64|REF|19.254783|883.536|304.00|
|4x2-w64|PARENT|19.107621|859.955|296.02|
|4x2-w64|CLEAN|19.194692|863.605|303.37|
|8x2-w64|REF|26.365444|1335.196|495.06|
|8x2-w64|PARENT|26.288512|1301.549|497.57|
|8x2-w64|CLEAN|26.116262|1302.731|486.36|
|merge-24x1-w64|REF|57.568461|2615.883|1166.76|
|merge-24x1-w64|PARENT|57.734686|2576.337|1172.55|
|merge-24x1-w64|CLEAN|57.123504|2571.790|1205.92|
|merge-54x1-w64|REF|124.312911|4747.489|2195.74|
|merge-54x1-w64|PARENT|123.169610|4704.368|2130.14|
|merge-54x1-w64|CLEAN|123.377169|4717.417|2150.64|

Latency is the median of nine process arithmetic means, with 151 timed requests for 1×2 and 31 for other shapes after three primes. Each timer covers fresh assignment, solving, blinding, proving, SDK verification and serialization. Loading, primes, export and external verification are excluded. CPU sums user and system time across threads and includes ordinary loop work between SDK timers. RSS is the maximum process lifetime peak, including loading and primes; it is not a minimum-RAM bound. Process masks and individual worker masks are checked before and after each run. Default SDK stacks and accepted allocator settings are retained.

The fixed protocol covers nine shapes at the previously accepted best worker counts, plus 1×2 at 16/64/96 and 2×2 at 96. Context order rotates and reverses across nine blocks; arm order also rotates/reverses. Pairing uses the same context and block. Sorted ratio ranks 1/7 (zero based) give nominal 96.09375% marginal coverage under independent exchangeable blocks, without familywise or stationarity guarantees. All 52 latency/CPU nonregression upper bounds must be at most 1.02. All thirteen RSS guards require CLEAN ≤ max(REF, PARENT)×1.05+1 MiB; the primary 96-worker latency upper bound versus REF must be below one. There are no exclusions, extensions, gain floors, threshold changes or subset rescues. Paired ratios and displayed medians use different aggregation.

|Context|Check|Ratio|Lower|Median|Upper|Wins / 9|Pass|
|:---|:---|:---|---:|---:|---:|---:|:---|
|12x2-w64|latency|CLEAN/REF|0.990712|1.003156|1.019296|4|yes|
|12x2-w64|CPU|CLEAN/REF|0.984700|0.985205|0.988566|9|yes|
|12x2-w64|latency|CLEAN/PARENT|1.000889|1.006469|1.010272|1|yes|
|12x2-w64|CPU|CLEAN/PARENT|0.996624|1.008836|1.010053|3|yes|
|16x2-w64|latency|CLEAN/REF|0.987690|1.000378|1.012301|4|yes|
|16x2-w64|CPU|CLEAN/REF|0.980509|0.987459|0.988636|9|yes|
|16x2-w64|latency|CLEAN/PARENT|0.973605|0.992468|1.008115|6|yes|
|16x2-w64|CPU|CLEAN/PARENT|0.989494|0.999307|1.001622|7|yes|
|1x2-w16|latency|CLEAN/REF|0.994948|0.998962|1.002476|5|yes|
|1x2-w16|CPU|CLEAN/REF|0.996506|0.998211|0.998538|9|yes|
|1x2-w16|latency|CLEAN/PARENT|0.995616|0.999028|1.001958|5|yes|
|1x2-w16|CPU|CLEAN/PARENT|0.997169|0.998310|0.999060|9|yes|
|1x2-w32|latency|CLEAN/REF|0.984829|0.991531|1.012218|6|yes|
|1x2-w32|CPU|CLEAN/REF|0.980507|0.996759|1.006153|5|yes|
|1x2-w32|latency|CLEAN/PARENT|0.996180|1.001900|1.020177|3|no|
|1x2-w32|CPU|CLEAN/PARENT|0.995863|1.010127|1.013130|3|yes|
|1x2-w64|latency|CLEAN/REF|0.960015|0.996561|1.021493|5|no|
|1x2-w64|CPU|CLEAN/REF|0.947032|0.959770|0.970509|9|yes|
|1x2-w64|latency|CLEAN/PARENT|0.949771|0.996467|1.041379|5|no|
|1x2-w64|CPU|CLEAN/PARENT|0.980977|0.998227|1.014507|6|yes|
|1x2-w96|latency|CLEAN/REF|0.946880|0.968449|0.978734|9|yes|
|1x2-w96|CPU|CLEAN/REF|0.865886|0.881043|0.888797|9|yes|
|1x2-w96|latency|CLEAN/PARENT|1.004103|1.022718|1.034023|1|no|
|1x2-w96|CPU|CLEAN/PARENT|0.998085|1.000751|1.014989|3|yes|
|24x2-w64|latency|CLEAN/REF|0.995751|1.005360|1.018100|3|yes|
|24x2-w64|CPU|CLEAN/REF|0.982590|0.991375|1.000570|7|yes|
|24x2-w64|latency|CLEAN/PARENT|0.982907|1.005614|1.029198|4|no|
|24x2-w64|CPU|CLEAN/PARENT|0.990170|1.001321|1.018433|4|yes|
|2x2-w32|latency|CLEAN/REF|0.987989|0.996552|1.000252|6|yes|
|2x2-w32|CPU|CLEAN/REF|0.990855|0.994193|0.998448|8|yes|
|2x2-w32|latency|CLEAN/PARENT|0.991255|1.002636|1.011141|3|yes|
|2x2-w32|CPU|CLEAN/PARENT|0.996353|1.000231|1.005086|4|yes|
|2x2-w96|latency|CLEAN/REF|0.938251|0.948717|0.957292|9|yes|
|2x2-w96|CPU|CLEAN/REF|0.886688|0.889736|0.902279|9|yes|
|2x2-w96|latency|CLEAN/PARENT|0.986861|0.998018|1.012451|5|yes|
|2x2-w96|CPU|CLEAN/PARENT|0.993999|0.999241|1.025009|5|no|
|4x2-w64|latency|CLEAN/REF|0.985639|0.996427|1.005167|5|yes|
|4x2-w64|CPU|CLEAN/REF|0.968472|0.973400|0.995310|9|yes|
|4x2-w64|latency|CLEAN/PARENT|0.986054|1.001132|1.012880|4|yes|
|4x2-w64|CPU|CLEAN/PARENT|0.987746|1.005697|1.016497|3|yes|
|8x2-w64|latency|CLEAN/REF|0.987817|0.995580|1.005936|5|yes|
|8x2-w64|CPU|CLEAN/REF|0.969762|0.978413|0.984610|9|yes|
|8x2-w64|latency|CLEAN/PARENT|0.993134|0.999480|1.010028|5|yes|
|8x2-w64|CPU|CLEAN/PARENT|0.994880|0.997826|1.010199|5|yes|
|merge-24x1-w64|latency|CLEAN/REF|0.987971|0.992681|0.995438|9|yes|
|merge-24x1-w64|CPU|CLEAN/REF|0.969167|0.977714|0.996056|9|yes|
|merge-24x1-w64|latency|CLEAN/PARENT|0.976096|0.996877|1.006442|6|yes|
|merge-24x1-w64|CPU|CLEAN/PARENT|0.991675|0.994311|1.008674|6|yes|
|merge-54x1-w64|latency|CLEAN/REF|0.965763|0.990505|1.025105|5|no|
|merge-54x1-w64|CPU|CLEAN/REF|0.978922|0.993871|1.008229|7|yes|
|merge-54x1-w64|latency|CLEAN/PARENT|0.984425|1.002859|1.012285|3|yes|
|merge-54x1-w64|CPU|CLEAN/PARENT|0.980795|0.997010|1.024686|5|no|
|1x2-w96|primary96_gain|CLEAN/REF|0.946880|0.968449|0.978734|9|yes|

Six latency and two CPU checks fail. Most failed intervals straddle parity; those failures do not establish a regression by themselves. The primary CLEAN/PARENT latency interval is wholly above one: [1.004103, 1.034023], median 1.022718. The effective source operation is the same, but these are different compiled artifacts; compiler layout, placement interactions and run variation have not been isolated. The observed discrepancy must not be attributed to a specific cause from this experiment alone. The default build retains most of the high-core CPU saving versus REF: paired CPU ratio 0.881043, or 11.90% less. This still does not improve the best tested 1×2 latency, which is 11.406619 ms at 32 workers for CLEAN.

|Shape|CLEAN workers|CLEAN ms|Peak RSS MiB|Supplied M5 CPU ms|CPU speedup|Supplied M5 Metal ms|Metal speedup|10× CPU target ms|
|:---|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2|32|11.406619|125.54|37|3.244×|24|2.104×|3.7|
|2x2|32|15.192123|252.88|59|3.884×|30|1.975×|5.9|
|4x2|64|19.194692|303.37|109|5.679×|48|2.501×|10.9|
|8x2|64|26.116262|486.36|189|7.237×|74|2.833×|18.9|
|12x2|64|34.158605|707.96|284|8.314×|101|2.957×|28.4|
|16x2|64|38.663417|841.10|335|8.665×|124|3.207×|33.5|
|24x2|64|54.593465|1292.62|528|9.671×|176|3.224×|52.8|
|merge-24x1|64|57.123504|1205.92|507|8.876×|175|3.064×|50.7|

These are descriptive selections among tested worker counts, not a proven global optimum. Merge 54×1 at 64 workers measures CLEAN 123.377169 ms / 2,150.64 MiB, REF 124.312911 ms / 2,195.74 MiB and PARENT 123.169610 ms / 2,130.14 MiB; no supplied M5 reference exists for this shape. All eight CPU 10× targets remain unmet. No fresh Aeglos measurement is made. Its earlier pinned 1×2 result of 7.902802 ms at 96 workers in the [previous all-shape comparison](../2026-11-10-accepted-current-c7a176/README.md) remains historical context, not a new matched comparison; CLEAN has not demonstrated beating it.

The clean native build passes 127 core, two prover and six cubic tests, coherently rebuilding eight libraries with Rust 1.98.1, znver4, O3, fat LTO, one codegen unit and the accepted manual flags including dense busy reset. Separate generic x86-64/noIFMA/noASM validation passes 102 core and two prover tests; the Zolana target compiles with zero tests enabled. Four independent reader models cover ties, pairing, context isolation and exact binomial coverage. Field source/qualification is inherited. This experiment adds no concurrent SDK workload or throughput claim; because performance qualification fails, no production release qualification is claimed.

The local functional preflight makes 44 SDK calls and eleven fresh Gnark-verified 164-byte exports across all nine CLEAN shapes plus primary REF/PARENT at four workers. Target preflight plus nine main blocks make 25,050 SDK calls and 390 fresh exports, all externally verified on the server. After size/SHA-bound retrieval, all 390 verify once more locally without increasing the distinct count. The independent reader validates 23,880 timed samples, argv/environment, stream hashes, fixed worker placement, means, bindings and freshness. A separate publication reader recomputes all 66 gates from producer summaries and checks 780 server command receipts and 390 local transport-verification receipts. The canonical verified-export ledger advances **50,351 → 50,362 → 50,752**. Combined local/target work is 25,094 calls and 401 distinct fresh exports. No successful science is replayed for bookkeeping.

One c7a.24xlarge supplies 96 physical cores without SMT. The network-disabled container has a 96-CPU quota, 30 GiB memory/no swap, PID limit 2,048, dropped capabilities and no-new-privileges. This is an experimental container setting, not an architectural RAM cap. All observed major-fault counts are zero; CPU throttling, memory-pressure/OOM and swap-event counters do not increase. Minor-fault rows are preserved. Physical-host exclusivity and frequency locking are not established. During every SDK preflight/recording only the confirmed live handle is polled; no owned helper/build/hash/analysis/report overlaps capture. All main captures finish before main-export verification.

The owned VM is terminated, its encrypted delete-on-termination volume and temporary ECR tag are absent, and the private transfer object/bucket and secret URL file are deleted. No live temporary benchmark instance remains in the checked region. The fixed 90-minute termination timer is retained throughout. There are no new phone/ARM/TEE, huge-circuit, memory-pressure, swap or minimum-RAM results.

Next work should distinguish reproducible scalar-grain benefits from artifact-specific effects before another adoption attempt, using retained binaries and source/build evidence first. The failed 66-check qualification is closed; its bounds will not be relaxed or its nine blocks extended. The broader high-core scaling gap also remains, so this partial improvement is not a complete explanation.

[Analysis and all 66 checks](artifacts/target-analysis.json), [resource rows](artifacts/resource-rows.json), [protocol](artifacts/target-protocol.md), [proof/sample review](artifacts/remote-review.json), [native review](artifacts/native-review.json), [generic review](artifacts/portable-review.json), [source audit](artifacts/source-audit.json), [release decision](artifacts/release-review.json), [quality](artifacts/target-quality.json), [cleanup](artifacts/cleanup.json), [publication checks](artifacts/publication.json), [raw results, sources, receipts and public proofs](artifacts/results-sources-public-proofs.tar.gz), [checksums](SHA256SUMS). The archive round-trips exact bytes and hashes. Executables, compiled libraries, private production inputs, credential files and unfiltered container environment are excluded. Reproduction requires the pinned toolchain/dependencies and private inputs.

Local directory: `/home/ubuntu/src/zopro-perf-work/phi-grain-clean176`; clean SDK: `sdk-candidate-znver4`.
