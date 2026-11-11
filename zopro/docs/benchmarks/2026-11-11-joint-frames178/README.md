# Joint query window scheduling

**Reject this recipe locally.** A shared set of A, K/H and G2 window/reduction
jobs makes primary proofs slower at both worker counts, despite slightly lower
CPU time. Production remains unchanged.

|Case|Accepted REF ms|Fresh OFF ms|Joint ON ms|Paired ON/OFF|ON wins|
|---|---:|---:|---:|---:|---:|
|1×2, 16 workers|8.940118|8.979293|9.434469|1.051108|0/9|
|1×2, 32 workers|7.740051|7.769949|8.833823|1.136310|0/9|
|2×2, 32 workers, unchanged-path control|10.850950|10.939006|10.862772|0.994909|7/9|

The primary candidate also loses every pair against accepted REF. All three
backing medians regress against both controls. At 32 workers, OFF/ON CPU time
is 176.862/175.751 ms per request; maximum lifetime RSS is 97.75/95.75 MiB.
At 16 workers, CPU is 114.966/113.566 ms and RSS is 76.75/75.00 MiB.
Lower aggregate CPU use does not establish a shorter dependency path.

Times are medians of nine process means, each containing 17 fresh requests after
three primes. Ratios pair processes by block and executable backing; they need
not equal ratios of displayed medians. RSS includes loading and primes and does
not establish minimum RAM or behavior under pressure. No samples were extended
or discarded. The 2×2 control is not evidence of an arithmetic improvement.

## Change and evidence

The prototype prepares grouped A/G2 inputs, then lets fixed drainers on the
existing pool claim window fills and ready 256-point reductions across all three
queries. Drainers return when no ready work is claimable; in-flight producers
continue draining after publication. No application-level spinning, new pool,
expanded key table or unsafe code is introduced. Existing internal Rayon work
in top windows and bucket merging remains.

Point arithmetic, A width12/one-part decomposition, balanced K/H windows,
G2 phi width12/two-part decomposition and final folds are preserved. A/G2
preparation now precedes the joint phase. The new preparation barrier, retained
bucket arrays/partials and per-drainer scratch are charged to timing. A G2 scalar
copy is needed when grouping does not supply owned storage. This is not a
memory-admitted production implementation. The screen does not isolate which
of these scheduling and lifetime changes causes the regression.

The history audit reused the [query-work removal study](../2026-11-09-query-neutralization-sdk176/README.md):
K/H and G2 already materially affect proof latency. The
[query helping analysis](../2026-10-29-query-help175/README.md) also shows why
callback return order cannot identify exclusive query service. Those diagnostics
were not repeated. Unlike the earlier scoped-continuation prototype, reduction
chunks here have no individual Rayon job. Unlike the preceding
[K/H-only frames](../2026-11-11-window-frames177/README.md), the job set spans all
three queries and drainers do not wait for unfinished frames.

Independent generator-weighted checks validate 63 three-query outputs and 15
unsupported grouped-geometry decisions across 1/2/4/16/32 workers. Checks include
zero/one/negative/dense/repeated scalars, identity/signed/duplicate points, nested
joins and four OS callers sharing a four-worker pool. The source audit recovers
all 176 originals: 170 exact, six with reversible edits or appendages, one new
child. This is focused native qualification, not a full release test suite.

An initial compile failure used a private accessor; the existing crate-visible
accessor fixes it. Subsequent integration review caught a potential double H
conversion when grouped geometry falls back. The final wrapper converts H once
on that branch. It was rebuilt before any SDK request; the unchanged arithmetic
adapter's qualification was reused. The synthetic test checks the decision to
fall back, not that complete SDK fallback branch. Both repairs are retained.

Twelve preflight and 81 main processes complete **1,668 SDK calls**, including
primes, and 1,389 timed samples. All **93 fresh exported proofs** independently
pass Gnark verification and expected image/public binding checks. Reviews check
271 pin entries, receipts, placement, executable identities and sample counts.
Fresh assignment, solving, randomness, proving, SDK verification and serialization
are timed; loading, primes and export are excluded. No helper/build/profile work
overlaps capture. There is no new PMU profile or runtime dispatch trace.

REF is accepted `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`.
OFF/ON share `e618f14945c2f64c40b7a9f490be897dd01e084267e3c59e0246a959ea5d4866`.
Each SDK has three independently written byte-identical backings. Accepted
Rust1.98.1/znver4 flags and allocator settings, fixed local Ryzen9950X CPU masks
and default SDK worker stacks are retained.

Preparation from protocol creation to final linked SDK takes about277 seconds;
main capture18.653 s and independent verification1.382 s. The canonical ledger
advances **54,100 → 54,112 → 54,193**, at
`/home/ubuntu/src/zopro-perf-work/joint-frames178/proof-sha256-after-main.json`,
SHA-256 `075b7efb0a64d9c00013d4286c93b1149a6469c404ca9db9c7feddfaee25aa8b`.

## Decision

Do not sweep constants or expand this candidate into a public scheduler. The
fixed screen rejects this implementation; it does not reject every work-sharing
design. Two-bank accumulation, K/H frames and joint frames complete the current
three-experiment review. Their common lesson is that caller latency, CPU work and
full-proof latency can move differently. The next mechanism must target a
distinct reduction in arithmetic or data movement, informed by existing negative
results, rather than another rearrangement of the same job boundaries.

The [accepted all-eight M5 CPU/Metal comparison](../2026-11-11-cost-map177/README.md#accepted-full-proof-reference)
is unchanged; the goal remains active and unmet, 0/8. No new AWS, Aeglos, phone,
pressure, large-circuit or throughput result is claimed. No push is authorized.

[Protocol](artifacts/protocol.md), [analysis](artifacts/analysis.json),
[source audit](artifacts/source-audit.json), [source and receipts](artifacts/evidence.tar.gz),
[checksums](SHA256SUMS). The archive excludes binaries, full source snapshots,
private inputs/proof payloads and full ledgers; reproduction requires pinned
local inputs and dependencies.
