# C7a task interval diagnostic

The accepted Zopro build measures **11.429457 ms at 32 workers** and **13.854492 ms at 96 workers** for 1×2. The compact interval observer fails three of six target calibration/overhead checks, all at 96 workers. No optimization is adopted, and the traces do not establish the cause of the scaling loss.

One c7a.24xlarge provides 96 physical cores without SMT. Nine rotated/reversed matched blocks run REF (accepted Zen4 build), OFF (new observer build disabled) and ON (enabled), at 32 and 96 workers. Each process runs three primes and 31 measured requests. Timings include assignment, solving, blinding, proving, internal verification and serialization, excluding loading, primes and observer setup/checking. Masks and individual worker placement are checked before and after. Physical-host exclusivity is not established.

The new build uses Rust 1.98.1, znver4, O3, fat LTO, one codegen unit and the accepted flags, including dense busy reset. All eight linked libraries are rebuilt coherently. The observer and driver retain the previously qualified compact-interval implementation. Production source and arithmetic remain unchanged.

|Workers|Build|Proof ms|Whole-loop CPU ms/request|Peak RSS MiB|Minor faults/request|
|---:|:---|---:|---:|---:|---:|
|32|REF|11.429457|250.148|94.79|442.52|
|32|OFF|11.491669|251.124|96.38|412.39|
|32|ON|11.583473|266.345|173.55|1068.97|
|96|REF|13.854492|779.926|88.73|464.06|
|96|OFF|13.386323|699.790|85.69|449.84|
|96|ON|14.679793|871.397|328.82|2703.10|

Values are medians of nine process summaries. CPU includes observer work between SDK calls and is not SDK-only CPU service. RSS includes loading, preparation and primes and does not establish minimum RAM. All process major-fault counts are zero. Container CPU throttling, memory-pressure/OOM and swap-event deltas are zero. The 30 GiB container setting is a harness limit, not a prover architectural cap.

|Workers|Paired ratio|Median|Rank interval|Pass|
|---:|:---|---:|:---|:---|
|32|OFF/REF|1.008822|[0.987049, 1.017637]|yes|
|32|ON/OFF|1.008051|[0.997516, 1.019989]|yes|
|32|ON/REF|1.016033|[1.003603, 1.028097]|yes|
|96|OFF/REF|0.967828|[0.947436, 0.975830]|no|
|96|ON/OFF|1.095031|[1.084351, 1.102771]|no|
|96|ON/REF|1.061875|[1.032994, 1.069515]|no|

Intervals use sorted paired ratios at ranks 1 and 7 (zero based), nominal 96.09375% marginal coverage under independent exchangeable blocks; there is no familywise or stationarity guarantee. OFF/REF must lie wholly within [1/1.03, 1.03]; ON/OFF and ON/REF upper bounds must be at most 1.05 at both worker counts. All six are required. The disabled observer build's positive 96-worker estimate is retained, but its calibration falls outside the declared range; this campaign does not attribute it to an arithmetic or scheduling improvement. The exact target observer recipe is not qualified, and the passing 32-worker subset is not promoted.

The following phase medians describe all 54 retained main **instrumented** graphs. They are exploratory because the overall target overhead screen failed; aggregate overhead also does not bound individual-phase perturbation.

|Phase|32-worker ms|96-worker ms|Published jobs 32 / 96|
|:---|---:|---:|---:|
|Solver|1.956411|3.020608|183 / 183|
|Scalar preparation|0.409373|1.178726|8232 / 24516|
|FFT|0.781805|1.388888|7813 / 11955|
|MSM queries|7.165933|7.646556|1547 / 16454|
|Verification|0.583283|0.645024|3 / 3|

All published job generations in the retained graphs match unambiguously. Instrumented scalar preparation publishes about three times as many jobs at 96 workers; instrumented MSM queries publish about eleven times as many. These observations motivate a controlled granularity intervention rather than another flat-profile audit. They do not demonstrate those counts in the uninstrumented build or prove causality. Missing queued jobs do not imply no available arithmetic; direct inline work is not queued. Nested job spans are not exclusive useful CPU, and victim-search wall residence is not CPU idle time.

The next hypothesis is a bounded minimum grain for the existing A/B scalar gathers. Source review identifies unrestricted parallel gathers in `solve_canonical_wires`; canonical scalar conversion already has a minimum length of 128. Test the isolated gather change against the accepted build without tracing, including 32/96-core latency, CPU, RAM, all shapes and fresh proof verification before any adoption. Previous broader dataflow changes remain separate negative results. No large lookup tables, expanded proving-key plans or new production worker pool are proposed.

The supplied M5 references for 1×2 are CPU 37 ms and Metal 24 ms: the fresh accepted 32-worker observation is 3.24× and 2.10× faster respectively, below the CPU 10× target of 3.7 ms. The [accepted all-shape AWS comparison](../2026-11-10-accepted-current-c7a176/README.md) remains the all-eight-shape CPU/Metal reference. This diagnostic adds no fresh Aeglos measurement, other-shape latency claim, phone/ARM/TEE result, memory-pressure qualification or concurrent-proof throughput measurement. The full objective remains unmet.

Native field/core/prover/cubic qualification and both reader model suites passed before container preflight. Local preflight completes 44 SDK calls and 11 fresh independent Gnark exports across all nine shapes. Cloud preflight and main complete 1,860 calls and 60 fresh exports; all are independently verified on the server, then verified once more after SHA-bound transport without increasing the distinct count. The independent reader checks 1,680 timed cloud samples and 56 retained cloud graphs. The cumulative verified-export ledger advances 50137 → 50148 → 50208.

The first container attempt fails output-directory access before any SDK call; its receipt is retained. A preparation helper also had an initial borrow-check compile error, corrected before preflight; direct preparation/compiler invocations are outside the command-receipt inventory. No successful proof or recording is replayed. During preflight and recording only the live measurement handle is polled; no owned helper/build/analysis overlaps.

Before publication/launch, an [operational amendment](artifacts/operational-amendment.md) shortens automatic termination to 25 minutes with at least 35 minutes of role lifetime, because SSO token refresh is unavailable. No scientific inputs, ordering, counts or gates change. The single owned instance is terminated; its encrypted delete-on-termination volume and temporary ECR tag are absent, and the private transfer object/bucket are deleted. No live temporary benchmark instance remains in the checked region.

[Target controls](artifacts/target-analysis.json), [phase rows](artifacts/phase-summary.json), [resources](artifacts/resources.json), [independent review](artifacts/remote-review.json), [protocol](artifacts/design.md), [cleanup](artifacts/cleanup.json), [publication checks](artifacts/publication.json), [sources and receipts](artifacts/sources-and-receipts.tar.gz), [raw traces and public exports](artifacts/traces-and-public-exports.tar.gz), [checksums](SHA256SUMS). Sources/receipts include exact commands, frozen input hashes and native source; production keys, images, requests, executables and credential-bearing files are excluded.
