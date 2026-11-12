# Solver-phase search participation

**The local screen does not establish a gain; the candidate is not adopted.**
Primary32 is 7.731 ms with the restriction versus 7.754 ms in the same binary's
unrestricted mode. The paired ratio is 0.999933, with 5/9 wins and inconsistent
backing medians. Grouped2x2 is 0.47% slower in paired comparisons, with 0/9 wins.
All 97 fresh exported proofs independently verify with Gnark.

## Intervention

Only during native canonical witness solving, the first ceil(N/3) workers may
search other workers' private deques. All workers retain local pops, external
injection, broadcasts, original arithmetic and task creation. This limits search
participation, not the number of workers executing existing or injected work.
The full pool is restored before scalar preparation, FFT and MSM.

A private Rayon1.13 fork adds one atomic limit and mutex-protected scope count
per existing registry. RAII restoration uses the existing injected-job fence,
sleepy event and wakeup path. Overlapping participating scopes lift the restriction
until all finish. No new pool, table, key plan or unsafe code is introduced.
Unrelated application work can still be affected while the diagnostic is active;
this is not a qualified production API.

The fork passes 113 scheduler tests, with two upstream tests ignored. New checks
cover nested work, 1/4/16/32-worker pools, 512KiB test stacks, panic restoration,
foreign pools, concurrent participating callers and all-worker broadcast after
restoration. Arithmetic is unchanged. Independent SDK proof checks qualify the
tested paths; no new all-shape, portable, phone or memory-pressure claim is made.

## Fixed full-proof screen

Ryzen9950X, fixed affinity. REF is exact accepted SDKb646; OFF/ON share candidate
`1f1f432a7399f82aa98226e7e5c166da08150921e1148eb359a06307fd232579`.
Rust1.98.1, znver4, O3/fatLTO/oneCGU. All retain adopted phi grain and tile batching.
OFF includes the fork's unrestricted limit load. The coherent SDK build uses
the same 48 other registry packages; only Rayon is replaced by its private fork.

Twelve preflights precede 81 main processes: three blocks across three fixed,
independently written backings per consumer, with rotated order. Each main process
times17 complete SDK requests after3 primes; every sample remains. Scope counts,
pool restoration, tile dispatch and placement pass. Timings include scope/transition
costs, solving, blinding, proving, verification and serialization. Loading and primes
are excluded. All recordings precede external verification.

|Case|REF / OFF / ON ms|Paired ON/OFF|ON/OFF wins|REF / OFF / ON peak MiB|
|---|---:|---:|---:|---:|
|1x2 /16|8.885 /8.837 /8.774|0.997122|6/9|75.50 /73.75 /78.25|
|1x2 /32|7.726 /7.754 /7.731|0.999933|5/9|96.00 /95.75 /94.00|
|2x2 /32|10.714 /10.872 /10.939|1.004676|0/9|237.25 /233.50 /237.50|

Times are medians of process means; paired ratios need not divide displayed
medians. RSS is lifetime peak, not minimum RAM. CPU ms/proof OFF→ON is
111.225→110.579, 172.765→171.930 and252.913→249.322 respectively.
The declared screen passes18/21 checks; three primary32 gain checks fail. All
nonregression and RSS guards pass. This is a directional screen without CIs.

REF backing0 is slower than its other copies, especially at16workers; all copies
remain included. Favorable ON/REF aggregates therefore do not establish a gain.
The same-binary ON/OFF comparison also fails to show consistent primary benefit.
See [full comparisons](artifacts/analysis.json) and [assessment](artifacts/assessment.json).

## Post-screen profile and next decision

Four separately frozen warm captures add916 SDK calls and4 fresh verified exports.
Primary32 OFF/ON counters report100%running. User cycles/proof are792.26/781.40
million (−1.37%); instructions1,323.15/1,323.33million (+0.014%). Cache-miss events
are2.778/2.759million. Cycle profiles have12,383/12,249samples,211/134unknown and
zeroLOST; PID, mapping, worker affinity and FIFO gates pass. Absolute sampled-period
coverage is unvalidated. The inherited profile filenames contain `B`, but their
recorded commands execute complete SDK proofs, not G2-only queries.

This single panel suggests a small change in total cycle cost, not a large local
coordination reduction or exclusive critical-path saving. The local screen stays
closed. A matched32/96 physical-core C7a test is a distinct next question motivated
by the historical search-overlap evidence. AWS STS returned expired SSO/refresh
failure this turn; no cloud resource was created. Do not sweep more local limits.

Scheduler test build/run24.387s, coherent libraries39.949s, SDK link16.275s;
preflight/main capture1.935/18.529s, external verifiers0.200/1.402s. Design creation
to main review was about3m38s, excluding earlier history/source work. The profile
workflow took9.809s. Lock preparation initially changed unrelated versions; selective
resolution restored runtime equivalence. Remaining optional/dev lock changes are
not among compiled SDK packages. No successful test or capture was replayed.

Total2,584 SDK calls,97 new distinct Gnark exports; canonical count55,024, ledger
SHA`b015c16fdf71d7caee67a6bc055d66368fe79b01a299c397e9408f94c36b312a`.
All177 production files remain exact. The [accepted all-shape CPU/Metal comparison](../2026-11-12-bundle-adoption178/README.md)
and unmet0/8 goal remain unchanged. This experiment is local and unpushed.

[Source/receipt audit](artifacts/close-review.json), [profiles](artifacts/diagnostic-analysis.json),
[sources, frozen plans, raw text receipts and rows](artifacts/evidence.tar.gz).
Raw perf data, binaries and private proof/input payloads remain under
`/home/ubuntu/src/zopro-perf-work/phase-search178`.
