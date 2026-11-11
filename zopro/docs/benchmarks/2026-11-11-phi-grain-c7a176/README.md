# G2 scalar-grain candidate on C7a

The candidate improves 1×2 at **96 workers by 5.12% paired versus accepted**, winning all 21 blocks. Median process-mean latency is **12.820886 ms versus 13.426282 ms**; paired CPU use falls 12.62%. All twelve declared target checks pass. At 32 workers latency remains near parity. Production is unchanged pending clean-source/default and all-shape qualification.

This is a fresh comparison on one c7a.24xlarge, 96 physical cores without SMT. REF is the accepted Zen4 SDK, OFF is the newly built original iterator, and ON adds only `with_min_len(128)` to the existing in-place G2 `phi_split` iterator through an experimental compile-time cfg. The other 175 source files and all 176 production hashes remain unchanged. Arithmetic, ordering, eligibility, points, buckets, allocations and forecasts are retained; no runtime selector, new buffer, lookup table or worker pool is added.

|Workers|Build|Proof ms|Whole-loop CPU ms/request|Maximum lifetime RSS MiB|
|---:|:---|---:|---:|---:|
|32|REF|11.468709|250.184|127.77|
|32|OFF|11.453442|250.502|129.32|
|32|ON|11.418734|249.973|124.96|
|96|REF|13.426282|769.841|121.18|
|96|OFF|13.540828|774.449|124.71|
|96|ON|12.820886|673.065|123.39|

Latencies are medians of 21 process arithmetic means, each from 151 SDK requests after three primes. CPU sums user and system time across threads and includes ordinary loop work between SDK timers. RSS is the maximum across the 21 process lifetime peaks, including loading and primes; it is not minimum RAM. Timings include fresh assignment, solving, blinding, proving, SDK verification and serialization, excluding loading, primes and export/reader work. Fixed process and individual worker masks are checked before and after; default stacks and accepted allocation settings are retained. Physical-host exclusivity and frequency locking are not established.

|Workers|Metric and paired comparison|Median ratio|Rank interval|Wins / 21|Pass|
|---:|:---|---:|:---|---:|:---|
|32|latency OFF/REF|0.999072|[0.989927, 1.007104]|11|yes|
|32|latency ON/OFF|0.994070|[0.988465, 1.014533]|13|yes|
|32|latency ON/REF|0.995278|[0.988827, 1.004844]|12|yes|
|32|CPU ON/OFF|0.997103|[0.990470, 1.010136]|13|yes|
|32|CPU ON/REF|0.995372|[0.991161, 1.002533]|13|yes|
|96|latency OFF/REF|1.002023|[0.995079, 1.015855]|10|yes|
|96|latency ON/OFF|0.942474|[0.933841, 0.957409]|21|yes|
|96|latency ON/REF|0.948800|[0.941856, 0.970336]|21|yes|
|96|CPU ON/OFF|0.864975|[0.862822, 0.873659]|21|yes|
|96|CPU ON/REF|0.873787|[0.864885, 0.888297]|21|yes|

Comparisons pair the same worker count and block. Sorted ranks 5/15 of 21 ratios give nominal 97.339630126953125% marginal coverage under independent exchangeable blocks, with no familywise or stationarity guarantee. Fixed rotated/reversed order, counts and gates were frozen before the first container preflight. OFF/REF latency intervals must lie wholly within factor 1.03 at both counts. ON96 must have latency upper bounds below one against both controls; ON32 latency and both-count CPU upper bounds must be at most 1.02. Maximum ON RSS must be no more than max(REF, OFF)×1.05+1 MiB at each count; both guards pass. There is no gain floor, sample exclusion, extension or post-result threshold change. Paired ratios/savings and displayed median timings use different aggregation.

The 32-worker intervals cross one: this campaign does not establish a latency gain there. Among the two tested counts, 32 remains faster than 96; 64 is not remeasured. The result identifies a useful high-core improvement, not the complete explanation of scaling or a measured reduction in job counts. The earlier task observer failed its target overhead check; the present performance run uses no task tracing. The exact minimum-grain idea was previously near parity locally; this new target condition tests C7a with 96 physical cores.

Earlier pinned Aeglos measured 7.902802 ms at 96 workers in the [previous all-shape AWS comparison](../2026-11-10-accepted-current-c7a176/README.md). No fresh Aeglos run is performed here, so that is historical context rather than a new matched comparison. This candidate has not demonstrated beating that result. The supplied M5 1×2 CPU/Metal references are 37/24 ms; ON32 is about 3.24×/2.10× faster respectively, still short of the CPU 10× target of 3.7 ms. The original all-eight objective remains active and unmet. This primary-only timing campaign supplies no fresh other-shape latency, phone/ARM/TEE, memory-pressure, minimum-RAM, huge-circuit or concurrent-throughput result.

Both fresh native builds pass 127 core, two prover and six cubic tests and coherently rebuild eight libraries with Rust 1.98.1, znver4, O3, fat LTO, one codegen unit and accepted manual flags including dense busy reset. Candidate generic x86-64/noIFMA/noASM validation passes 102 core and two prover tests; the Zolana test target compiles with zero tests enabled. Field source/qualification is inherited. Three independent reader models cover ties, nonproportional pairing and exact binomial coverage. Native SDK identities and frozen protocol remain unchanged after generic validation.

Local functional preflight covers all nine candidate shapes plus primary REF/OFF at four workers: 44 SDK calls and eleven fresh independently Gnark-verified 164-byte exports. Cloud preflight and main make 19,428 SDK calls and 132 fresh exports, all Gnark verified on the server. After SHA/size-bound transport, all 132 are verified once more locally without increasing the distinct count. The independent reader checks 19,032 timed samples, command argv/environment, stream hashes, fixed placement, means, bindings and freshness. A separate publication reader recomputes all ratio/RSS gates from producer summaries. The cumulative verified-export ledger advances **50,208 → 50,219 → 50,351**.

All major-fault counts are zero. Container CPU throttling, memory-pressure/OOM and swap-event deltas are zero. The 96-CPU/30 GiB/no-swap/network-disabled container is a benchmark setting, not a prover architectural RAM cap. Minor faults remain in the raw resource rows. During all preflight/recording only the confirmed live measurement handle is polled; no owned helper/build/hash/analysis/report overlaps captures. All main timings finish before main-export verifiers. No successful science is replayed.

The initial nested-xtask host-lock failure occurred before Cargo; its correction runs native workflows directly. Expired SSO initially stopped image publication before creating resources. After renewed login, the existing image and completed tests/proofs are reused, with the scientific protocol unchanged. One owned server is launched with a 90-minute termination timer. It is now terminated, its encrypted delete-on-termination volume and temporary ECR tag are absent, and the private transfer object/bucket are deleted; no temporary benchmark instance remains in the checked region.

[Target analysis](artifacts/target-analysis.json), [all resource rows](artifacts/resource-rows.json), [quality](artifacts/target-quality.json), [independent proof/sample review](artifacts/remote-review.json), [protocol](artifacts/target-protocol.md), [cleanup](artifacts/cleanup.json), [target publication checks](artifacts/target-publication.json), [raw target results, sources and public proofs](artifacts/target-results-sources-public-proofs.tar.gz), [native review](artifacts/native-review.json), [generic review](artifacts/portable-review.json), [preparation sources and receipts](artifacts/preparation-sources-receipts-public-proofs.tar.gz), [generic sources and receipts](artifacts/portable-sources-receipts.tar.gz), [checksums](SHA256SUMS). Archives round-trip exact bytes and hashes. SDK executables, compiled libraries, production inputs and credential-bearing files are excluded. Reproduction requires pinned tools and private inputs.

Local experiment directory: `/home/ubuntu/src/zopro-perf-work/phi-grain-c7a176`; executables: `sdk-control-znver4` and `sdk-candidate-znver4`.
