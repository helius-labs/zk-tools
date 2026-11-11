# Bounded G2 batching: SDK screen and profiles

**Continue qualification of this exact candidate.** The local 16-worker primary
screen improves against accepted production across all nine pairs. Hardware
counters support reduced CPU work. The faster 32-worker context remains
inconclusive against accepted production. No production change or breakthrough
is established.

|Context|Accepted REF ms|Candidate OFF ms|Candidate ON ms|Paired ON/REF|ON/REF wins|
|---|---:|---:|---:|---:|---:|
|1×2, 16 workers|8.969920|9.013759|8.788301|0.982089|9/9|
|1×2, 32 workers|7.744617|7.791784|7.734493|0.997536|6/9|
|2×2, 32 workers|10.906112|10.797861|10.773151|0.989349|7/9|

These are medians of nine process means, each with 17 timed fresh requests after
three primes. Paired ratios need not equal ratios of displayed medians. The
16-worker medians differ by 0.182 ms. All three executable-backing medians improve
against both REF and OFF there; the other contexts have mixed signs against REF.
The 2×2 path also executes the candidate and is not an unchanged-path control.
This fixed short screen supplies no adoption confidence interval.

## Mechanism and source binding

The exact [existing reducer](../2026-11-08-indexed-tile-bundles176/README.md)
bundles independent G2 bucket tiles into jobs containing at most 1,024 points.
It shares affine batch inversions across unchanged pair trees, preserving point
order and outputs. It adds job metadata and worker offsets/counts, with checked
optional admission and fallback before input mutation. It introduces no key
tables, expanded solver plans, extra full point arrays or production pool.

The earlier standalone threaded gate remains failed. Later
[BUNDLE-only control evidence](../2026-11-09-paired-top-query176/README.md)
motivated this first prospective full-proof screen. The failed PAIR/BOTH
compositions remain closed. This is follow-through on useful evidence, not a
new algorithm or a revision of an old gate.

The child SHA-256 is
`862a9ce36635a2c02f14529b3a39f259ef3b7fd6c24cbd0565b112ef703caba7`.
Source review binds the original reducer, 18 unchanged field files and all 176
accepted source files: 174 exact, two instrumented parents, one added child.
Native core tests, generator/edge checks and complete-query Arkworks comparisons
are inherited under those bindings. Current accepted phi-grain remains enabled.
New SDK counters check zero indexed calls in OFF and four per timed request in
ON. The prior denial test injects an absent charge; complete SDK resource-denial
fallback remains to be tested.

## Profiles and resources

The same SDK and three retained executable backings supply separate plain,
counter and sampled-profile panels at 16 workers, 251 requests per process.
Across the three counter pairs, instructions decrease **3.718–3.746%** and user
cycles **3.967–4.071%**. Cache-miss counts increase; no cache improvement is
claimed. Inverse-symbol samples decrease from 2,285 to 1,881 across the six
profiles, consistent with the mechanism, without establishing exclusive wall
time or exact operation counts.

All event-running, sample-count, loss, attribution and unknown-symbol checks
pass. Instrumented/plain median ratios range from 1.0063 to 1.0090, within the
declared factor-1.05 guard. These checks do not prove unbiased observation.
Each backing has one counter panel per arm, without repeated-panel uncertainty.

|Screen context|REF / OFF / ON CPU ms per request|Maximum lifetime RSS MiB, REF / OFF / ON|
|---|---:|---:|
|1×2, 16 workers|114.982 / 115.041 / 110.760|76.00 / 75.75 / 77.25|
|1×2, 32 workers|177.262 / 177.439 / 172.110|94.25 / 95.50 / 98.25|
|2×2, 32 workers|256.526 / 255.255 / 250.177|223.25 / 241.00 / 239.25|

Additional reservation is 270,872 bytes per indexed call at 16 workers and
537,880 at 32, not measured allocation or a complete-process bound. RSS includes
loading and primes; it is not minimum RAM or memory-pressure qualification.

## Validation and next decision

The screen and profile campaigns complete 6,240 SDK calls, including primes,
5,907 timed samples and **111 fresh independently Gnark-verified exports**.
Receipts, input bindings, placement and executable identities pass. Recording
and other owned work do not overlap. SDK timing includes fresh assignment,
solving, randomness, proving, verification and serialization, excluding loading,
primes and export. Host is Ryzen 9950X with fixed affinity, default SDK stacks,
Rust 1.98.1 and accepted znver4/allocator settings.

Accepted REF is `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`;
shared OFF/ON is `b646c7b8c1d8a26a0db45fb3f67c453dd616bd9f02e25c36192721ae02855abe`.
Preparation to linked SDK took about 79 seconds; main capture took 18.345 seconds
and its independent verifiers 1.361 seconds. These are not total research time.

Next, freeze and run all-shape performance/resource qualification against REF
on this exact build, including the surviving 16-worker and best-latency
32-worker contexts. Check actual denied-admission fallback, applicable portable
and concurrent behavior, and production activation. No cleanup relink is needed
to qualify this artifact. Adoption is pending; no second candidate is selected.

The canonical ledger is **54,304**, SHA-256
`a53b7f952c40444ab5dc5f88e38d1689716b2deb2e0cf465021784b8c7eadcf5`, at
`/home/ubuntu/src/zopro-perf-work/bundle-profile178/proof-sha256-after-profile.json`.
The [accepted all-eight CPU/Metal comparison](../2026-11-11-cost-map177/README.md#accepted-full-proof-reference)
is unchanged. The full goal remains unmet, 0/8. There is no new C7a, Aeglos,
ARM, huge-circuit or concurrent-throughput result. No push is authorized.

[Screen](artifacts/screen-analysis.json), [profiles](artifacts/profile-summary.json),
[source audit](artifacts/source-audit.json), [screen protocol](artifacts/screen-protocol.md),
[profile protocol](artifacts/profile-protocol.md), [reducer](artifacts/indexed_tiles.rs),
[source and receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
Private inputs, proof payloads, binaries and raw perf recordings remain local.
