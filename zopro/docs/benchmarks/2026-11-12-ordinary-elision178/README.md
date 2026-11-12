# Ordinary G2 fill and reduction elision

Replaying ordinary bucket fills reduces primary 32-worker complete-query latency
from **2.993 to 1.407 ms** in this fixed-input diagnostic. Replaying ordinary
part merges and weighted reductions instead gives **2.667 ms**; replaying both
gives **1.244 ms**. Ordinary filling offers substantially more observed headroom
than the downstream reductions. This is **not a production speedup**, a full-proof
bound, or a measurement of C7a behavior.

The native build took 42.882 seconds; preflight and main caller commands took
1.638 and 14.567 seconds. Filesystem timestamps put protocol creation to the
verified result at approximately 3 minutes 27 seconds, including preparation,
build and orchestration, but excluding preceding history review and this report.
All **1,632 complete queries** match retained independent Arkworks MSM oracles.
No full-SDK campaign or new proof export was needed.

## Intervention and fixed screen

A private snapshot starts from all 177 adopted production files. Three parents
receive reversible diagnostic wrappers and one child is added. The arithmetic
implementations remain unchanged. The current native selector enables both the
adopted phi grain and indexed tile batching. Rust 1.98.1, znver4, O3, fat LTO,
one codegen unit; diagnostic executable SHA-256
`7e7aa24140745a47c4fec0b3342b1b8b6eb0b52365541416c8f6d1b9bd1fcfb7`.

Four modes share one executable:

- **Normal:** execute all original fills and reductions.
- **Fill replay:** clone saved ordinary bucket arrays in place of twenty fills.
- **Reduction replay:** still execute the fills, then return ten saved window
  sums instead of their part merges and weighted reductions.
- **Both:** replay both stages.

All modes save the same first-prime outputs: twenty 2,048-point affine arrays
and ten projective sums, **5,244,800 bytes** of coordinate payload. Allocation
overhead is additional. Cache identity is positional and valid for only one
immutable fixture per process. This cache must never become a proving path.
Fill cloning and dropping buffers remain timed, including in reduction replay.
The outer window/part task geometry, indexed top, grouping, scalar split, final
fold and normalization remain. Exact call/execution/slot/byte counters verify
which work actually stopped executing.

Ryzen 9950X, 16 physical/32 logical CPUs. Two retained fixtures at 1/16/32 workers,
individual worker affinity to CPU2 or CPUs0–N−1, and 512 KiB worker stacks.
Twenty-four one-round preflights precede 72 main processes. Each main process
has three excluded primes, seventeen timed queries and one checked postcall.
Three fixed blocks use three distinct, byte-identical executable backings;
context and mode order rotate/reverse. Every output and point-input immutability
check passes. No sample is excluded or added, and no owned work overlaps capture.

## Complete-query observations

Times are medians of three process means. The final columns are medians of
within-block ratios to Normal; they need not equal ratios of displayed medians.
These are directional observations without confidence intervals or adoption gates.

|Query / workers|Normal ms|Fill replay ms|Reduction replay ms|Both ms|Fill / Normal|Reduction / Normal|Both / Normal|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2 / 1|32.834|8.835|28.850|4.737|0.2691|0.8787|0.1443|
|1x2 / 16|3.330|1.319|3.059|1.038|0.3982|0.9172|0.3132|
|1x2 / 32|2.993|1.407|2.667|1.244|0.4795|0.8860|0.4115|
|2x2 / 1|35.488|11.122|31.454|7.003|0.3134|0.8863|0.1975|
|2x2 / 16|3.902|1.993|3.634|1.817|0.5089|0.9277|0.4639|
|2x2 / 32|3.529|2.012|3.250|1.737|0.5701|0.9210|0.5069|

All three replay modes are faster than Normal in every corresponding block.
For primary32, fill-replay ratios range 0.4556–0.4827; reduction-replay ratios
range 0.8824–0.9362. Per-block data and nonadditivity are retained in
[result.json](artifacts/result.json). Individual savings must not be added as
independent proof-time fractions.

CPU service below is median user+system milliseconds per query. RSS is the
largest lifetime process peak across blocks, including load, primes and replay
storage. Each four-value cell follows Normal / Fill / Reduction / Both order.

|Query / workers|CPU ms/query|Peak RSS MiB|
|---|---|---|
|1x2 / 1|32.83 / 8.83 / 28.85 / 4.74|20.25 / 20.25 / 20.25 / 20.25|
|1x2 / 16|39.53 / 13.01 / 32.47 / 7.35|31.25 / 30.50 / 29.25 / 31.25|
|1x2 / 32|63.61 / 23.29 / 49.43 / 15.51|32.25 / 34.75 / 34.50 / 34.50|
|2x2 / 1|35.49 / 11.12 / 31.45 / 7.00|33.75 / 33.75 / 33.75 / 33.75|
|2x2 / 16|42.88 / 17.32 / 36.50 / 11.54|40.50 / 41.50 / 47.25 / 40.25|
|2x2 / 32|70.04 / 32.08 / 56.93 / 21.90|58.25 / 65.00 / 55.75 / 51.50|

## Decision and limits

Prioritize ordinary filling over another reduction-only or indexed-top-only
variant. Eliminating fills changes both single-worker work and threaded latency;
it is not merely an aggregate-CPU saving. The primary both-stage residual is
slower at32 than16 in these medians and consumes more than twice the CPU service.
That residual includes indexed top, grouping/splitting, final work, replay copies,
allocation and coordination. It must not be labeled pure scheduler overhead.

The intervention does not distinguish arithmetic, memory movement and dependency
cost within the fills. Nor does it establish an attainable 52% query gain: replay
changes work availability and cache/allocator behavior. Normal is a fresh
instrumented diagnostic consumer, not the exact accepted SDK; wrapper overhead
relative to that SDK is unmeasured. No C7a causal conclusion follows from this
local screen. A read-only authentication check failed with expired SSO; no cloud
resource was created.

The next structural question concerns exposing independent ordinary-fill work
without paying for extra bucket copies and merges. The existing
[independent-plane trial](../2026-10-31-g2-independent-planes176/README.md) and
[borrowed-tree pilot](../2026-11-07-borrowed-bucket-tree-pilot176/README.md)
already measured regressions; this diagnostic does not reopen their unchanged
recipes. One concrete source-level cost to screen is the borrowed tree's
`append`: it writes each generic pair's left point into the next output buffer
before the ordinary in-place adder reads and overwrites it. Later levels repeat
this staging. Test whether a reducer borrowing both inputs and writing each
result once can remove that traffic without adding more descriptor/sign work.
This would be a distinct data representation, not another tree partition sweep;
it still needs a complete-caller measurement. No production candidate is adopted
by this report.

A separate same-agent Rust reader reconstructs all ratios from raw integer
samples, checks receipts and stream hashes, verifies distinct executable copies,
recovers the three edited parents and checks the other174 production bindings.
It is not a second cryptographic oracle. No timing or successful test was replayed.
[Protocol](artifacts/protocol.md), [review](artifacts/review.json),
[diagnostic](artifacts/phase_elision.rs), [raw evidence](artifacts/evidence.tar.gz).
Private fixtures, binaries, full source snapshots and proof ledgers are excluded.

Accepted primary full-proof latency remains **7.788384 ms / 98.25 MiB peak RSS**
locally. All eight supplied M5 CPU/Metal comparisons remain those in the
[adoption report](../2026-11-12-bundle-adoption178/README.md); none was remeasured
here. The complete CPU10x goal remains active and unmet, **0/8**. Canonical ledger
stays **54,927** verified exports with SHA-256
`48e866327435bbc85f7c8d0433115337d7d903481594f307bab4373c39ec3119`.
No AWS/Aeglos, mobile, pressure, huge-circuit or throughput result is added.
