# Borrowed-pair G2 reducer: complete-query screen

**The candidate fails its latency screen and is not adopted.** Primary32 complete
G2 query medians are **2.917 ms streaming, 3.092 ms staged tree and 3.118 ms new
borrowed-pair tree**. The new candidate is slower than streaming in every block
of every context. Removing a left-point staging copy does not recover the tree's
performance. No full-proof qualification or further variant follows this screen.

## Change and correctness

The [ordinary-stage diagnostic](../2026-11-12-ordinary-elision178/README.md)
identified substantial local headroom in ordinary fills. The earlier
[borrowed tree](../2026-11-07-borrowed-bucket-tree-pilot176/README.md)
exposed independent bucket groups but regressed. Its generic pair operation first
copies the left point into an output slot, then invokes an in-place affine adder.

The new private reducer borrows both operands. A24-byte descriptor holds two
references, destination and two sign bits; the old descriptor is16bytes. Generic
results are written directly without a staged left point. Initialized destination
buffers remain live across levels and groups instead of being cleared and
reinitialized on every level. New/growing slots are still initialized. Exceptional
identities, equal-X cases and odd leaves retain complete curve handling.

The tree's routing and window bodies,256-bucket groups, pair order, vector padding,
point partitions, top path and final merge/weight operations are unchanged. The
additional descriptor and retained buffer capacities are covered by checked
reservation accounting before scalar mutation; denial selects original streaming.
No new unsafe code, persistent table, transformed key, solver plan or pool is added.

Five focused native tests pass: both operand sign masks with padded/unpadded tails
and unrelated prefilled destinations; tree exceptions/skew/reuse; complete-query
generator-scalar oracle/dispatch;1/4/16/32-worker512KiB stacks; and checked memory
bounds/actual retained capacities. The sign test covers all256 masks independently
for each operand through a bijective pairing, not all65,536 joint mask pairs.
These tests share accepted curve/field arithmetic. Real complete queries additionally
match the retained independent Arkworks MSM oracles, with inputs unchanged.

All19 preflight processes pass, including primary32 with optional workspace
allowance zero and confirmed fallback. This is configured admission denial, not
physical RAM pressure. Native tests compile/run in59.672s, the caller builds in
44.302s, and preflight/main caller commands take1.758/18.118s. Protocol creation to
review is about7m30s by filesystem timestamps, excluding preceding source review
and subsequent profiling/reporting. There are no failed builds or tests to rerun.

## Frozen query screen

Ryzen9950X,16 physical/32 logical CPUs. Current177-source baseline, Rust1.98.1,
znver4,O3,fatLTO,oneCGU, native phi grain and indexed tile batching. Four production
parents receive reversible private edits; the other173 files match. Streaming,
staged tree and borrowed-pair tree share one process-fixed-mode executable:
`80dec42895cabe28dbc33476eb2b1a2cccaf39b328c2040695b67696ad8d0218`.
Streaming is a fresh experimental consumer, not the exact accepted SDK.

Six contexts cover1x2 and grouped2x2 at1/16/32workers. Three fixed blocks use three
distinct byte-identical executable backings with rotated/reversed order. Each main
process has3excluded primes,17timed queries and1postcheck. Affinity is per worker;
scalar reset, grouping, allocation/drop, reservations, routing and all query work
remain timed. All54 main processes finish before analysis or profiling. Every
sample is retained; there are no extensions or favorable-copy selections.

Times are medians of3process means. Ratios pair corresponding blocks, so they need
not equal ratios of displayed medians. This is a directional screen without CIs.

|Query/workers|Streaming ms|Staged tree ms|Borrowed pair ms|New/streaming|New/staged|
|---|---:|---:|---:|---:|---:|
|1x2/1|32.592|37.645|37.487|1.1529|0.9964|
|1x2/16|3.323|3.529|3.563|1.0723|1.0073|
|1x2/32|2.917|3.092|3.118|1.0682|1.0084|
|2x2/1|35.213|40.863|40.608|1.1569|0.9959|
|2x2/16|3.892|4.144|4.126|1.0574|1.0007|
|2x2/32|3.438|3.893|4.021|1.1668|1.0327|

Progression required new/streaming<1 in every primary32 block and≤1.03 in every
16/32-worker block for both shapes. It fails. Primary32 new/streaming ranges
1.0400–1.0722; new/staged ranges0.9606–1.0522, so that latter threaded direction
is inconclusive. Small single-worker improvements over the old tree do not pass
the comparison against streaming.

Three-value cells below follow streaming/staged/new order. CPU is median
user+system service per query; RSS is maximum lifetime peak across blocks.

|Query/workers|CPU ms/query|Peak RSS MiB|
|---|---|---|
|1x2/1|32.59 /37.64 /37.49|15.25 /15.50 /15.25|
|1x2/16|39.34 /46.62 /46.86|27.25 /35.00 /34.50|
|1x2/32|61.31 /74.24 /77.03|29.50 /47.25 /49.00|
|2x2/1|35.21 /40.86 /40.61|27.00 /27.25 /27.75|
|2x2/16|43.10 /52.61 /52.57|43.50 /46.75 /46.50|
|2x2/32|67.68 /87.93 /89.85|53.75 /69.50 /83.50|

## Diagnosis after the closed screen

A separately frozen supplement uses the same binary/fixture at32workers: one
201-query counter panel and one251-query cycle profile per arm, gated after
primes. All six captures finish before decoding. These panels cannot reopen the
latency gate. All counters report100% running; profiles have4336/5124/5121samples,
zero LOST records and44/101/109unknown samples. PID, worker affinity, event period,
mapping metadata and gate checks pass. Absolute sampled-period coverage is unverified.

|Counter per query|Streaming|Staged tree|Borrowed pair|
|---|---:|---:|---:|
|Instructions|513,229,584|572,307,697|577,912,619|
|User cycles|279,002,086|325,731,058|326,205,218|
|Cache-miss events|1,201,915|3,605,864|4,096,728|
|Branch-miss events|257,707|557,320|579,549|

New/staged instructions rise0.98%, cache-miss events13.61%, and cycles0.15%.
The tree append helper's samples fall459→273 while the selected affine arithmetic
helper's samples rise2217→2391. Work moved between consumers without a clear total
benefit. One panel supplies no confidence interval, exclusive wall-time accounting
or attribution to a particular cache/memory level.

Source inspection suggests a tradeoff: staging put the left operand into a compact
output buffer for later passes; direct borrowing replaces that with repeated
referenced-input loads, larger descriptors and sign handling in the arithmetic
consumer. This is an explanation consistent with the data, not an isolated causal
proof. Eliminating a source-level copy did not eliminate its net cost here.

## Disposition and evidence

Close this representation change. Do not sweep tree partition sizes or instruction
annotations to rescue it. Next research should change ordinary-fill arithmetic or
dependency structure while charging routing, locality and merging together. The
positive G1 two-bank caller also already has an
[inconclusive SDK follow-up](../2026-11-11-two-bank-sdk177/README.md); it is not an
unmeasured candidate waiting to be rediscovered.

The source/raw reader checks177production bindings, restores4parents, confirms the
staged body and shared routing/window code, reconstructs all ratios and the failed
progression decision, and verifies receipts/copy identities. The separate profile
analysis checks21receipts and all additional oracle outputs. Both are same-agent
checks, not an external code audit. Total **2,609 complete queries** pass independent
retained MSM oracles:1,229screen/preflight plus1,380diagnostic. No SDK proof or new
Gnark export is generated. Raw perf binaries, private fixtures, full source trees
and proof ledgers remain private; source, hashes and text receipts are archived.

[Protocol](artifacts/protocol.md), [all results](artifacts/result.json),
[review](artifacts/review.json), [profiles/counters](artifacts/diagnostic-analysis.json),
[reducer](artifacts/g2_pair.rs), [tree](artifacts/direct_tree.rs),
[evidence](artifacts/evidence.tar.gz).

Production remains the [bounded-batching adoption](../2026-11-12-bundle-adoption178/README.md):
primary full proof **7.788384ms /98.25MiB**, with all eight supplied M5 CPU/Metal
comparisons unchanged there. The full CPU10x goal remains active and unmet **0/8**.
Canonical ledger stays **54,927** exports, SHA-256
`48e866327435bbc85f7c8d0433115337d7d903481594f307bab4373c39ec3119`.
No new AWS, Aeglos, mobile, physical-pressure, huge-circuit or throughput result.
