# Accepted SDK scaling on Ryzen 9950X

The accepted build continues improving through 32 workers, but with diminishing
returns. Primary retired instructions grow only about **5.4%** from one to 32
workers, while aggregate user cycles grow about **62%**. The much larger CPU-time
increase does not establish proportionally more executed instruction work.
No implementation changes or new speedup are adopted.

This diagnostic uses retained SDK `b646c7b8`, the exact
[accepted build](../2026-11-12-bundle-adoption178/README.md), without rebuilding.
It adds counts 1/2/4/8 to the latest 16/32 comparison. Each worker i is pinned to
CPU i. CPUs 0–15 are distinct physical cores; 16–31 are their SMT siblings.
Three blocks use three independently written, byte-identical executable backings,
with rotated/reversed context order. Backing and block are confounded; this is
an exploratory curve, not a precise small-effect or adoption qualification.

Each process measures seventeen complete SDK requests after three excluded
primes. Loading and preparation are excluded; fresh solving, randomness,
proving, internal verification and serialization are included. Displayed latency
and CPU are medians of three process means. CPU sums user and system thread time.
RSS is the maximum lifetime process value, including loading and primes; it is
not minimum RAM. All raw process means are retained.

|Workers|1×2 ms|1×2 CPU ms|1×2 RSS MiB|2×2 ms|2×2 CPU ms|2×2 RSS MiB|
|---:|---:|---:|---:|---:|---:|---:|
|1|87.937607|87.934|34.00|115.183943|115.180|59.50|
|2|45.801097|89.754|37.75|60.054360|118.697|69.25|
|4|24.396427|91.877|47.00|32.155443|122.937|87.75|
|8|13.674606|95.630|58.25|18.008014|130.151|119.00|
|16|8.759072|110.093|74.00|12.261015|155.688|167.50|
|32|7.710565|173.505|97.75|10.707751|249.211|244.75|

The 32-worker primary process means are 7.762677, 7.676307 and 7.710565 ms.
The previously accepted 7.788384 ms came from a different campaign using this
same binary; the difference is not a newly demonstrated optimization.

## Follow-up counters

The latency curve selected a narrower question: does increased CPU time reflect
more instructions or more cycles per instruction? A separate frozen follow-up
runs nine primary panels: 1/16/32 workers, three blocks/backings, 101 measured
requests after three primes. The retained SDK gates a simultaneous user-event
group through perf control FIFOs. Loading and primes are outside the counters.
Every event reports 100% running, and all enable/disable acknowledgments pass.
No cycle-sample recording or new observer-overhead qualification is claimed.

|Workers|Instructions/proof, billion|User cycles/proof, million|Instructions/cycle|
|---:|---:|---:|---:|
|1|1.255055|480.234|2.6134|
|16|1.282680|544.001|2.3583|
|32|1.322806|782.249|1.6930|

Medians of paired 32/1 ratios are 1.053987 for instructions, 1.624961 for cycles,
and 1.541857 for cycles/instruction. Across all three blocks, instruction growth
is 5.39–5.52%, versus cycle growth of 61.54–63.13%. At 16 workers, the corresponding
ranges are 2.20–2.22% and 12.82–13.28%. Aggregate CPU occupancy is not useful-work
occupancy. Frequency, SMT contention, cache behavior, dependencies and changes
in selected work are not separately attributed by these counters. Generic
cache misses identify neither a cache level nor a DRAM-capacity bottleneck.

This changes the local priority: another scheduler tweak justified solely by
rising aggregate CPU time is insufficient. Most of the observed proportional
cycle growth is in cycles/instruction, not instruction count. It does not imply
that scheduler costs are absent or that all slowdown is caused by SMT. One-worker
work divided by worker count is not an attainable latency bound.

The historical [C7a comparison](../2026-11-10-accepted-current-c7a176/README.md)
uses additional physical cores and shows sharply growing scheduler/epoch sample
residence. This local curve does not reproduce or explain that mechanism. Resume
the [prepared C7a intervention](../2026-11-12-phase-search-c7a178/README.md) when
credentials are valid. A fresh STS request this turn failed with expired SSO;
no cloud resource was created. No new local scheduler parameter sweep follows.

## Validation and continuation

All 57 SDK processes and 57 independent Gnark verifications succeed: 12
preflights, 36 latency processes and nine counter panels. There are **1,704 SDK
calls**, including primes, and **57 unique exported proofs**. Every timed request
is internally verified. Review checks executable/input hashes, distinct backing
inodes, exact command receipts, stream hashes, worker placement before/after,
raw samples, public/image bindings and proof freshness. All indexed-reduction
counters show four entries/request, including lower worker counts. This is not
proof that every internal execution choice is invariant with worker count.

Preparation from the first protocol to frozen latency pins took 67 seconds;
preflight commands total 2.43 seconds, main commands 28.47 seconds. Counter
preparation took under one second from its written protocol to pins; its command
duration was 33.22 seconds. The latency reader completed about
45 seconds after pinning; the counter reader completed about 41 seconds after
its pinning. These exclude earlier research/history review, protocol drafting
and later report packaging. No successful capture or verifier was rerun.

Canonical exports advance **55,028 → 55,085**, at
`/home/ubuntu/src/zopro-perf-work/accepted-scaling179/proof-sha256-after-stat.json`,
SHA-256 `946f2b40315004bed6c4fe61f84b8d5f70fff27bb0c206924e58db0a055207f2`.
The frozen C7a package retains its historical 55,028 seed. Preserve its pins;
after its existing target-local ledger reaches 55,152, require all 124 target
exports to be disjoint from the latest canonical ledger and merge both chains.
With no intervening exports, the resulting canonical count would be 55,209.
Do not overwrite the package seed or replay its completed local preflights.

The all-eight CPU10× goal remains active and unmet, **0/8**. The full CPU/Metal
comparison remains in the accepted report. Fresh descriptive 32-worker ratios
are 37/7.710565 = 4.799× CPU and 24/7.710565 = 3.113× Metal for 1×2;
59/10.707751 = 5.510× CPU and 30/10.707751 = 2.802× Metal for 2×2.
Supplied M5 timings are historical, on different hardware. No AWS, Aeglos,
mobile, pressure/swap, minimum-RAM or throughput result is added.

[Latency analysis](artifacts/analysis.json), [counter analysis](artifacts/stat-analysis.json),
[paired counter ratios](artifacts/counter-ratios.json),
[preflight review](artifacts/preflight-review.json), [main review](artifacts/main-review.json),
[source, frozen protocols and raw receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
