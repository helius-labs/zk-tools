# Current-source C7a primary hardware counters

Diagnostic complete: **true**. This compares the exact accepted 172-file Zopro source, freshly built for Zen4, with pinned Aeglos `0290613` Sorted at 32 and 96 workers on one temporary c7a.24xlarge. No source optimization or adoption occurs. These are instrumented warm SDK observations, not a new unprofiled latency qualification.

The protocol uses three fixed rotated/reversed blocks and separate four-event branch/cache groups: 24 counter processes, followed by four separate 199 Hz user-cycle profiles. Each counter process runs 501 requests after three primes; each profile runs 251 after three. Loading and primes precede FIFO enable/ack, and disable/ack precedes proof export. Counters include userland SDK logging, arithmetic, polling and coordination; SDK rusage includes kernel time too. Different panels are different processes.

## Counter observations

Cells are medians of three processes. IPC is the median of per-process ratios; counts are per timed request. No statistical latency guarantee or isolated cause follows from these three diagnostic blocks. Generic cache events are not DRAM bytes, bandwidth or miss latency.

|Workers|Engine|Panel|Cycles/request million|Instructions/request million|IPC|CPU ms/request|Instrumented SDK ms|
|---|---|---|---:|---:|---:|---:|---:|
|32|current|branch|897.195|1444.834|1.610|276.331|12.695|
|32|current|cache|888.278|1441.577|1.622|274.684|12.649|
|32|aeglos|branch|1169.994|1807.959|1.545|331.404|11.970|
|32|aeglos|cache|1168.460|1807.977|1.547|331.724|11.931|
|96|current|branch|2512.418|1842.052|0.734|817.667|15.089|
|96|current|cache|2522.036|1838.041|0.731|818.125|15.305|
|96|aeglos|branch|1677.500|2107.259|1.255|529.581|7.944|
|96|aeglos|cache|1670.713|2106.335|1.261|528.472|7.920|

## Separate warm profiles

All counter captures finish before these profiles. Flat category shares are not removable wall-time fractions or exclusive critical-path costs. Generic categories match Zopro symbols; Aeglos appears as other, a matcher limitation. Its raw flat symbols remain archived.

|Workers|Engine|Samples|CPU ms/request|Profile SDK ms|Epoch %|Scheduler %|
|---|---|---:|---:|---:|---:|---:|
|32|current|14925|272.840|12.537|8.88|5.20|
|32|aeglos|17261|331.899|12.209|—|—|
|96|current|58509|791.810|14.619|48.35|12.44|
|96|aeglos|27615|534.437|8.038|—|—|

## Identity, verification and lifecycle

The current build passes generator consistency, 33 field and 119 core tests plus two raw and six cubic prover tests. Local preflight uses the exact final SDK binaries for eight calls and two independently verified primary exports at four workers. Aeglos retains the qualified archive/driver and original source hash, including its existing shifted-base policy; no latest-release claim is made.

Completed diagnostic: true. SDK calls: 13120 (completed-call lower bound 13120). Distinct independent exports: 30, including 28 cloud exports independently reverified after transport; repeat verification is counted once. Proof/public members: 60. The ledger advances **12,143→12173**. All retained proofs are 164 bytes, fixture-bound and fresh against prior/new identities. Actual counters, hardware, memory events and throttle observations remain in the archive.

Exactly one owned instance uses a 90-minute self-termination timer and encrypted delete-on-termination volume. The measurement container is network-disabled, CPU quota 96, memory 30 GiB and swap disabled. Owned instance/volume, temporary ECR tag, private transfer object/bucket and credential files are removed before closure. No production resource or global perf setting is changed. Zero remaining regional TemporaryProverBenchmark instances is confirmed.

The [accepted all-eight CPU/Metal comparison](../2026-10-27-clean-fft-adoption/README.md) remains the canonical local comparison against supplied historical M5 figures. The [older C7a comparison](../2026-10-27-current-c7a-aeglos/README.md) used previous 171-file source and remains separate. No 54 reference was supplied, and no all-nine unprofiled cloud rerun occurs here. The original all-eight CPU 10× goal remains active and unmet, 0/8. Worker count can change geometry, frequency and coordination; counter changes do not isolate a clock, RAM-capacity, fence or syscall cause.

[Protocol](artifacts/design.md), [analysis and raw rows](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [post-transport verification](artifacts/post-transport-gnark.log), [source identity](artifacts/aws-accepted-manifest.json), [cleanup](artifacts/cleanup-complete.json), [artifact hashes](artifact-index.json).

## Interpretation and scope

At 96 workers Zopro retires about 12.6% fewer instructions than Aeglos but spends about 50% more user cycles per request in the branch panel. Moving Zopro from 32 to 96 workers increases instructions by about 27.5% and cycles by about 180%, while IPC falls from 1.610 to 0.734. This supports investigating parallel coordination and work availability; instruction count alone does not explain the gap. At 32 workers Zopro uses fewer instructions, cycles and CPU milliseconds than Aeglos, yet has slightly higher instrumented wall latency.

The separate current96 profile attributes 48.35% of sampled cycles to epoch symbols and 12.44% to scheduler symbols, versus 8.88% and 5.20% at32. Previous empty-victim and batched-steal trials reduced coordination costs without establishing a broadly faster prover, so these shares do not justify repeating those changes or predicting an equivalent latency saving. A distinct scheduling mechanism must preserve progress and be judged on full proofs. No improvement is adopted by this diagnostic.

Grouped C7a branch counters already existed for a historical 167-file source in the [October25 study](../2026-10-25-c7a-warm-profile/README.md). The [pre-capture audit](artifacts/prior-counter-audit.md) corrects the frozen design's broader absence claim. This study updates the exact 172-file source and adds a separate cache panel; it does not establish the first observation of the scaling issue or a controlled improvement over that older study.

Preparation and offline repairs are retained: [Aeglos identity check](artifacts/preparation-failure.json), [category presentation](artifacts/presentation-repair.json), [build identity metadata](artifacts/identity-presentation-repair.json), [proof-directory selection](artifacts/analysis-failure.json), and [cleanup volume identity](artifacts/cleanup-repair.json). The first post-transport verifier invocation found no proofs before reading any; corrected hash-checked copies then passed verification once. The inherited cleanup verifier initially checked the previous run's volume; the repaired receipt checks this run's recorded volume ID. No successful SDK capture, native build, test or proof verification was repeated for these repairs.
