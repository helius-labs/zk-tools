# Matched C7a phase timings for 1×2

On a fresh c7a.24xlarge, the exact accepted Zopro build took **12.306 ms** and Aeglos **7.731 ms** at 96 physical cores. At 32 cores Zopro led, **10.914 versus 11.940 ms**. Each value averages three independent process means of 151 warm, sequential, verified requests. The two untraced engines used the same input, same machine, same affinity policy and same measurement blocks.

Two effects explain the change in ordering. Zopro paid 1.82 ms more in solving, preparation and FFT as it moved to 96 cores. Its post-FFT query section recovered just 0.39 ms. On the same host Aeglos’s MSM fell from **8.333 to 3.810 ms**, saving 4.52 ms.

## Zopro phase boundaries

Sparse entry and return probes on the exact executable cover the SDK prove call, canonical proof path, solver, FFT product, A query and G2 query. Times below are disjoint elapsed intervals in the instrumented process. The post-FFT row includes queries, joins and canonical output work. The preparation row includes everything between solver return and FFT entry. The Aeglos MSM timer has different boundaries.

|Timed interval|32 cores, ms|96 cores, ms|96−32 ms|
|---|---:|---:|---:|
|Witness solve|1.917|2.649|0.733|
|Between solve return and FFT entry|0.235|0.546|0.311|
|FFT product|0.774|1.550|0.775|
|After FFT through canonical return|7.041|6.650|-0.391|
|Canonical entry to solver|0.003|0.003|-0.000|
|Outside canonical call|1.164|1.277|0.113|
|Traced request|11.134|12.676|1.542|

At 96 cores G2 returned a mean **2.190 ms** after A, counting zero when G2 returned first. At 32 cores the same value was **1.301 ms**. After the later of A and G2 returned, just **0.025 ms** remained before the canonical call returned on 96 cores. This localizes a late dependency in the current build. It does not convert the 2.190 ms into an achievable saving. Rayon can execute other tasks during either callback. K/H is not independently probed.

The paired median traced/plain ratio was **1.0179** at 32 cores and **1.0346** at 96, within the preregistered factor of 1.05 for both. Every timed trace had correct request counts, six entry/return pairs per request, matching entry and exit threads, a single process, legal CPU bindings, the expected nesting, monotonic request boundaries and zero reported loss or throttling. An aggregate overhead check cannot establish equal perturbation in every individual phase.

## Resource and engine comparison

|Engine and sample|32-core CPU ms/request|96-core CPU ms/request|32-core peak RSS MiB|96-core peak RSS MiB|
|---|---:|---:|---:|---:|
|Zopro, plain|229.82|686.55|123.32|120.31|
|Aeglos, plain|328.17|514.57|424.97|481.50|

Zopro uses less whole-process peak RSS at both counts. CPU here is summed user and system time per proof. It includes scheduling and worker search. The companion [41-shape analysis](../2026-11-18-c7a-gap250/README.md) shows Aeglos MSM gains across every shape and resolves Zopro’s high-core epoch hotspots. The unprofiled stage records and sparse span timings support investigating useful parallel work and synchronization. Neither measures a DRAM-bandwidth limit.

A concrete next test is to give each final G2 bucket one reduction tree across the existing two point partitions. The two source halves can still route in parallel. Each final bucket would need one high-component endomorphism and avoid merging two finished bucket arrays. The complete query must pay for indexing, any larger tree scratch and reduced final-job parallelism. A separate experiment can test a smaller set of active workers for pre-query work and release remaining workers when the queries become ready. The 1.82 ms is an arithmetic estimate of the scope of that opportunity on this run.

## Scope and verification

Accepted SDK d523 and retained Aeglos eab9 were hash-checked before launch. Six smoke processes passed independent Gnark and span checks before the three main blocks. Each main block used a separate byte-identical executable backing and all six processes. All SDK captures preceded late verification and decoding. There were 2,796 calls including three primes per process, 2,724 timed requests and 24 new exported proofs. The host independently verified all 24 after SHA-checked retrieval. The unique ledger reached 64,445. The 30 GiB, no-swap unit was a harness setting. The instance, tagged volume and temporary transfer bucket were removed.

At 96 cores Zopro was **3.01×** the supplied M5 CPU reference and **1.95×** the Metal reference for 1×2. Its CPU 10× target is 3.7 ms. The other seven reference shapes were outside the scope of this phase measurement. Production remains at the previously accepted build.

[Disjoint and overlapping intervals](artifacts/phases.json) · [All block results](artifacts/blocks.json) · [Measurement protocol](artifacts/protocol.md) · [Reader](artifacts/reader.rs)
