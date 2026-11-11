# Ready window reductions: caller and SDK screen

**The complete K/H caller improves at 16 workers, but the full-proof screen is
inconclusive. No production change is adopted.** This experiment replaces nested
reduction jobs with fixed scoped workers claiming chunks from ready windows.

|Complete K/H caller|Original ms|Candidate ms|Paired ON/OFF|Wins|CPU ms OFF/ON|Peak RSS MiB OFF/ON|
|---|---:|---:|---:|---:|---:|---:|
|16 workers|3.507721|3.218104|0.917105|9/9|38.848/51.035|28.00/26.25|
|32 workers|2.478706|2.507409|1.007700|1/9|55.141/69.702|32.00/32.00|

The 16-worker gain is 8.29% by median paired ratio, with 31.37% more CPU time.
All three backing medians improve at 16 workers and regress at 32. Single-worker
arms both execute the original path: 34.718464/34.702119 ms is control variation.
The inherited analyzer incorrectly labels that row as selecting a screen; the
protocol requires a **multithreaded** result. Selection uses only the 16-worker
result. The original analyzer/output are retained without changing frozen pins.

|Verified full proof|Accepted REF ms|Fresh OFF ms|Candidate ON ms|Paired ON/OFF|Paired ON/REF|
|---|---:|---:|---:|---:|---:|
|1×2, 16 workers|8.926604|8.971556|8.921822|0.996762|0.996884|
|1×2, 32 workers|7.813028|7.750907|7.794099|1.008305|1.002796|
|2×2, 32 workers|10.944306|10.923388|10.976297|1.001343|1.003623|

Displayed times are medians of nine process means, each with 17 requests after
three primes. Ratios pair processes by block and executable backing, so they
need not equal ratios of displayed medians. Both primary contexts have mixed
backing signs against both comparators. Neither meets the prospective screen
selection rule; this establishes neither a reliable gain nor a definite slowdown.
The fixed sample count was not extended. The unchanged 2×2 path is a control.

At primary 32 workers, REF/OFF/ON CPU time is 177.596/178.018/182.892 ms per
request; maximum lifetime RSS is 96.50/98.25/98.00 MiB. At 16 workers, RSS is
75.00/77.00/75.75 MiB. These include loading and primes, are not minimum RAM,
and do not establish memory behavior under pressure or concurrent proof load.

## Implementation and scope

The prototype retains twenty balanced windows, the original fill and weighted
point arithmetic, and final Horner fold. Workers on the existing Rayon pool
claim fill jobs, then 256-point reduction chunks from published bucket arrays.
The last chunk finishes its window. Workers yield without holding unfinished
chunks; a panic flag lets other workers exit before scoped joining.

All window arrays remain alive until query completion, and each worker reuses
weighted-reduction scratch. There is no expanded key table, new pool, unsafe
code, two-bank accumulation or public library change. This is a bounded
prototype, not the full scheduler described by the motivating
[work-assisting paper](https://doi.org/10.1145/3652586.3663313). No upstream code
was copied. The history audit distinguishes this from earlier Rayon continuation,
worker-group, queue and minimum-grain experiments.

Caller validation includes 71 generator-oracle cases over 1/2/4/16/32 workers,
nested joins, shared-pool callers, and injected panic followed by pool reuse.
All 1,104 real-fixture outputs match the retained independent Arkworks oracle.
The source snapshot binds 176 original files; private fixture/oracle identities
and append-only changes are recorded. The single-worker path is inherited.

The SDK uses a diagnostic environment switch at the concrete G1 fused K/H
entry, under its original eligibility guard. Source inspection confirms width
13 at the primary caller; no execution trace was collected. REF is accepted
`c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`.
OFF/ON share `9f9a9e326ceac88a831eb3d4b816cacf37a520db4cf5b2f99ad6db0993de0942`.
Each executable has three independently written, byte-identical backings.
Rust 1.98.1, znver4 and accepted runtime/build settings are retained. CPU masks
are fixed on the local Ryzen 9950X; SDK worker stacks retain their default size.

SDK timing includes fresh assignment, solving, randomness, proving, SDK
verification and serialization. Loading, primes and final export are outside.
Twelve preflight and 81 main processes complete 1,668 requests, including primes,
with 1,389 timed samples. All 93 fresh exported proofs pass independent Gnark
verification and expected public-input/image binding checks. Reviews check
264 pin entries, receipts, executable identity, sample counts and placement.
No helper/build/profile work overlaps capture.

The canonical ledger advances **54,007 → 54,019 → 54,100**. The new ledger is
`/home/ubuntu/src/zopro-perf-work/window-frames-sdk177/proof-sha256-after-main.json`,
SHA-256 `88fbb44d60da915f835fd7f3a5b09eaff14df230f140029bee24414a30d0c6bd`.
This is not all-shape, ARM/mobile, pressure, throughput, AWS or release qualification.

Caller capture takes 16.356 s; SDK main capture 18.432 s and verification 1.382 s.
An initial SDK build fails because a G1-specific route was inserted in a generic
function. Moving that route to the concrete G1 wrapper resolves the type error;
the successful native build takes 34.076 s and link 16.613 s. Both attempts are
preserved. This build failure is separate from the passing correctness checks.

## Decision

Defer this recipe: its isolated benefit does not reliably survive integration.
Preserve the candidate and evidence; do not start a chunk-size or worker-count
sweep. The next discriminating question concerns interference and completion
order among proof queries, rather than another isolated K/H optimization.
Added CPU work is a possible cost, not an established explanation for the result.

Accepted production and the [all-eight M5 CPU/Metal comparisons](../2026-11-11-cost-map177/README.md#accepted-full-proof-reference)
remain unchanged. The research goal remains unmet, 0/8. No push is authorized.

The artifact archive contains protocols, source additions, readers, frozen plans,
build/capture receipts and proof identities. It excludes binaries, private
inputs, proof payloads, full source snapshots and inherited proof ledgers.
Reproduction requires the pinned local inputs and dependencies.

[Caller analysis](artifacts/caller-analysis.json),
[SDK analysis](artifacts/sdk-analysis.json),
[evidence](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
