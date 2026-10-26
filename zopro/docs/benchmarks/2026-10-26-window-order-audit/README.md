# Top-window callback ordering audit

The old 32-worker traces do not support a late-release explanation for K/H's short top window. It starts a median **3.827 µs** after K/H kernel entry and **2.605 µs** after the first window callback. Reordering it to start first is therefore not supported as a large latency optimization by these observations. No top-first candidate is built or measured. This does not rule out other scheduling effects.

This is a new analysis of the existing readiness captures. All 102 matrix graphs are retained: 51 requests each at 16 and 32 workers. The separate four-worker preflight graph is outside the matrix. Graph and protocol hashes match the previous archive's `SHA256SUMS`; event schemas, IDs, worker ranges, parents, interval containment, window counts, widths and kernel/window identities pass validation. The six summaries cover 5,406 window intervals.

|Workers|Query|Top start from kernel ms|Top start from first window ms|Start rank (0-based)|Top last /51|Top inclusive ms|Gap after ordinary callbacks ms|
|---|---|---:|---:|---:|---:|---:|---:|
|32|a|1.169726|1.167703|19|0|0.806651|0.000000|
|32|kh|0.003827|0.002605|18|24|1.679137|0.000000|
|32|g2|1.491062|0.962496|9|6|1.517733|0.000000|
|16|a|4.459803|4.458971|19|9|0.590043|0.000000|
|16|kh|2.111433|2.110691|17|9|4.994472|0.000000|
|16|g2|3.621091|3.417357|8|18|3.223240|0.000000|

Each table value is a separate median, except the last-window count. All times are inclusive instrumented callback intervals. A late start rank can coexist with a very short absolute delay: at 32 workers, K/H's top starts 19th out of 20 callbacks, yet only 2.605 µs after the first. K/H's 24 last completions do not demonstrate that its own arithmetic is the bottleneck. Nested Rayon work and waits can extend a callback. Its gap after ordinary callbacks has median zero but p90 2.615744 ms; that gap is not isolated CPU service, removable latency or a schedule-replay speedup.

At 32 workers, A's top starts a median 1.169726 ms after kernel entry but is never last in 51 requests. G2's top starts at 1.491062 ms and is last in six requests. At 16 workers the three top windows often start later; their pooled median gaps after ordinary callbacks are still zero, with broad distributions retained in the analysis. These data do not establish an ordering change that helps all cases.

The captures predate the new borrowed pending-buffer candidates. They cannot explain or qualify the latest candidate's slow 16-worker process. No fresh proof, export, latency sample, profile, cloud resource, field/MSM implementation or production change occurs here. The proof ledger remains **6,928**. Two focused analyzer tests check that an early-starting top may finish last and that invalid parents, labels and intervals are rejected. All 170 production hashes and the accepted SDK hash are checked unchanged. The 4.7 MiB raw graph file is archived losslessly compressed with a verified byte-for-byte round trip.

The next source question is whether existing independent solver tasks offer useful across-hash SIMD. Current `Schedule` epochs contain independent tasks whose within-task order must remain intact; refined waves have their own barriers. Previous rejected Poseidon experiments pack operations within one hash. A new across-hash design first needs a census of same-width ready tasks, chain lengths, useful lane occupancy, tails and available task parallelism. No speedup or implementation is claimed. It must reuse the existing plan and bounded scratch rather than add a large table or another execution plan.

The [latest borrowed-G1 report](../2026-10-26-borrowed-g1/README.md) retains the experimental primary result and its failed 16-worker check. The [latest full SDK matrix](../2026-10-26-pairing-x-chain-sdk/README.md) retains all supplied M5 CPU/Metal comparisons. This audit changes neither comparison. The original all-eight CPU 10× goal remains active and unmet.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [per-request metrics](artifacts/rows.json), [analyzer](artifacts/analyze.rs), [raw graphs](artifacts/graphs.json.gz), [input pins](artifacts/input-pins.json), [artifact hashes](artifact-index.json).
