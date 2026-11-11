# Indexed-top elision: fixed-input query diagnostic

Removing indexed-top reduction work saves **21–23% of primary single-worker query time**, but only **5.35–8.50% at 32 workers**, or **0.161–0.260 ms**. This supports deprioritizing top-only changes as the main path toward the remaining primary full-proof gap. It does not establish a whole-proof upper bound or a production speedup.

The native build took **42.559 seconds**. Twelve preflight caller commands took **1.090 seconds**, and 36 timing caller commands took **11.153 seconds** in total. These exclude preparation, orchestration and analysis. All **816 complete queries** match the retained independent Arkworks MSM oracles. No full-SDK campaign was needed to answer this diagnostic question.

## Intervention

The prior [indexed-tile bundle trial](../2026-11-08-indexed-tile-bundles176/README.md) reduced CPU work and single-worker latency without establishing a threaded gain. Rather than repeat that algorithm, this experiment asks what happens when the four signed indexed-top component reductions have already finished.

A private snapshot retains the current original reducer unchanged. During the first excluded prime, both modes save its four outputs. OFF continues calculating every reduction; ELIDE subsequently clones the saved output. Both modes retain the same **32 KiB of saved affine coordinates**. Ordinary windows, grouping, Phi mapping of completed roots, merging, weighting and final normalization continue normally. Four wrapper calls per query and the appropriate original-reducer execution count are checked exactly.

This deliberately uses one immutable fixture per process. The cache is positional, has no general input identity or invalidation, and **must not become a production proving path**. Its purpose is to remove a selected cost experimentally. Every complete query still receives the independent oracle check, and point-input hashes must remain unchanged. The experiment adds no production cache, key table, memory policy or runtime pool.

## Fixed screen

Local Ryzen 9950X, 16 physical/32 logical CPUs. Rust 1.98.1, `znver4`, release optimization level 3, fat LTO, one codegen unit, current native arithmetic cfgs. One executable supplies both modes; three separately copied executable files provide the three fixed blocks. Solo runs pin to CPU2, threaded runs to CPUs0–15 or0–31, with 512 KiB worker stacks.

Six contexts cover primary and grouped 2×2 G2 queries at 1/16/32 workers. All twelve one-round preflights pass before the main screen. Each main process has three excluded primes, seventeen timed queries and one postcheck. Reset copies, grouping, allocation, MSM, pool dispatch and normalization remain timed. Parsing, pool creation and oracle checks are excluded. Ordering rotates/reverses by block; no sample extension, exclusion or favorable-copy selection occurs. Capture is isolated from owned builds, helpers and analysis.

These are three-block directional observations, without confidence intervals or adoption gates. OFF is a fresh diagnostic consumer, not the exact accepted SDK. Wrapper/code-generation overhead relative to that SDK is not measured. Ratios below are paired by block; displayed times are medians of process arithmetic means and need not divide to the median paired ratio.

|Query / workers|OFF ms|ELIDE ms|Paired ELIDE/OFF, blocks 0 / 1 / 2|Median reduction|
|---|---:|---:|---|---:|
|1×2 / 1|36.898287|28.726125|0.770745 / 0.782742 / 0.778522|22.15%|
|1×2 / 16|3.418638|3.316021|0.987246 / 0.962304 / 0.970631|2.94%|
|1×2 / 32|3.014236|2.803129|0.946509 / 0.915045 / 0.931047|6.90%|
|2×2 / 1|39.397281|31.264892|0.785852 / 0.793580 / 0.796397|20.64%|
|2×2 / 16|3.873775|3.794856|1.000728 / 0.976298 / 0.963970|2.37%|
|2×2 / 32|3.560564|3.375621|0.947845 / 0.938568 / 0.960446|5.22%|

The grouped 2×2/16 direction differs across blocks and remains inconclusive under the protocol. All primary contexts and both 32-worker populations improve in every block, but three observations do not qualify a precise population effect.

|Query / workers|Median CPU service OFF / ELIDE ms|Peak process RSS OFF / ELIDE MiB|
|---|---:|---:|
|1×2 / 1|36.897 / 28.725|15.25 / 15.25|
|1×2 / 16|44.099 / 33.944|26.75 / 25.25|
|1×2 / 32|68.483 / 51.174|31.00 / 25.75|
|2×2 / 1|39.397 / 31.263|27.00 / 27.00|
|2×2 / 16|47.147 / 37.422|43.00 / 33.75|
|2×2 / 32|74.789 / 56.830|53.50 / 53.75|

CPU service sums user and system time over the seventeen-call measured loop. RSS is the largest whole-process high-water observation, including loading, primes and allocator retention. It is not incremental workspace or minimum RAM. Avoided allocations, changed scheduler interaction and cache effects are part of this counterfactual; the result does not isolate any one of them. No intrusive phase observer or new hardware-counter profile is required to establish that the selected reductions stopped executing: exact execution counters prove that mechanism directly.

## Decision and verification

Deprioritize another top-only rescue as the main breakthrough strategy. Even complete removal leaves most threaded query latency. The next algorithmic target should remove substantial ordinary-window work across G1 and G2 while preserving independent jobs and bounded memory. This follows the prior [cost map](../2026-11-11-cost-map177/README.md), where streaming additions and affine reduction layers together account for about 46.58% of the sampled CPU residence; that CPU share is not removable wall time.

The separate same-agent Rust reader checks source recovery, native flags, identical executable hashes with distinct file identities, all raw integer samples, paired ratios, cache coverage and unchanged production/ledger bindings. Three original source parents have reversible diagnostic additions; the original top implementation and all other bound files remain exact. The first reader rejected a sub-nanosecond JSON floating-point roundtrip difference. Its retained correction changes only decimal-comparison tolerances; no capture, successful native build or raw data is rerun or edited.

No optimization is adopted. Accepted production and the verified ledger remain unchanged at **53,917 exports**, SHA-256 `a033132ee87cf5d5260c396acf1894bbb1afc95c07d8ba22c55ed9c8e5a85873`. The latest accepted primary full proof remains **7.821726 ms / 127.5 MiB peak RSS** on this machine; the CPU 10× target is **3.7 ms**. All eight CPU/Metal comparisons and their remaining gaps are retained in the [cost-map table](../2026-11-11-cost-map177/README.md#accepted-full-proof-reference). Goal status remains **active and unmet, 0/8**. No AWS/Aeglos, mobile, pressure, concurrency-throughput or new full-proof result is claimed. Nothing is pushed or published.

[Protocol](artifacts/protocol.md), [all paired results](artifacts/result.json), [review](artifacts/review.json), [diagnostic module](artifacts/top_elision.rs), [workflow/reader source](artifacts/manage.rs), [separate review source](artifacts/review.rs), [retained reader correction](artifacts/failures.md), [raw receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS). Reproduction requires pinned local libraries, compiler and private fixtures; the archive contains source/plan bindings and command receipts, not binaries or proving inputs.
