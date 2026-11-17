# G1 mixed Jacobian addition screen

The HMV mixed-addition formula passes correctness checks but produces **no
primary full-proof gain** in this local screen. Paired median latency versus
accepted227 is **2.18% slower at 16 workers** and **1.39% slower at 32 workers**,
with 0/3 and 1/3 wins. The exact candidate is closed and is not adopted.
Accepted scaled-prefix227 remains in production. Small gains remain eligible;
this screen has no minimum improvement floor.

## Change and linked implementation

The prototype substitutes the standard HMV formula for ordinary G1 mixed
Jacobian addition, after the existing identity/equality/opposite guards. It
retains G2 arithmetic. For scalar Fq, square and multiplication use the same
product backend. The old formula has 7M+4S; HMV has 8M+3S, so both require eleven
product-backend calls. Linear field operations fall from fourteen to seven.
Ordinary outputs differ only by projective scale: old=(4X,8Y,2Z).
See the primary [EFD formulas](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html).
No mathematical novelty or external implementation contribution is claimed.

The frozen protocol mistakenly stated eight fewer linear operations. A retained
[correction](artifacts/pre-capture-cost-correction.md), written after plan review
and **before any SDK capture**, corrects this to seven. Formulas, binaries,
ordering, sample counts, gates and analysis did not change. The original
[protocol](artifacts/protocol.md) and pins remain intact.

The private source has 187 files: all 186 accepted files, two modified parents
and one test child. Inverting those edits recovers all 186 accepted files;
184 original files are byte-identical. Production source is unchanged. The
private cfg adds no runtime option, heap buffer, table, key copy, pool or retained
state. Generic field tests also exercise the new formula without ASM or IFMA.

|Linked scalar method|Accepted REF / OFF|Candidate ON|
|---|---:|---:|
|G1 symbol bytes|12,957|12,020|
|G1 static instruction sites|2,479|2,295|
|G1 static MULX sites|352|352|
|G2 symbol bytes|36,371|36,371|

G1 REF/OFF symbol bytes are identical. G2 REF/ON symbol bytes are identical.
These are static linked counts, not executed instruction counts, spill costs
or speed predictions. Smaller correct code did not yield a full-proof gain.
[Codegen evidence](artifacts/codegen.json).

## Full-proof results

Ryzen 9 9950X, Rust 1.98.1, znver4, O3/fat-LTO/one codegen unit; coherent native
libraries and the exact accepted SDK driver/support. REF is retained accepted227;
OFF is freshly linked with the cfg absent; ON enables the new formula. Three
independent executable backings participate for each arm, one per matched block.
Cases and arms rotate/reverse under the frozen plan.

The SDK interval includes solving, randomized proving, internal verification
and serialization. Loading and proof export are outside it. Each process uses
three primes and then 251 timed primary calls, 51 for 2×2 or seven for merge54.
The benchmark uses default worker stacks, fixed worker/main affinities and the
same accepted three opt-ins and allocator. This does not change production
affinity policy or memory admission.

Times are medians of three process means, in ms. Paired ratios are medians of
matched ON/control ratios; they are a different estimator from ratios of these
marginal medians.

|Case|REF ms|OFF ms|ON ms|ON/REF|Wins|ON/OFF|Wins|
|---|---:|---:|---:|---:|---:|---:|---:|
|1×2, 16 workers|8.554279|8.577064|8.741190|1.021850|0/3|1.019135|0/3|
|1×2, 32 workers|7.476443|7.462430|7.580376|1.013901|1/3|1.015812|1/3|
|2×2, 32 workers|10.404818|10.422289|10.539384|1.013853|1/3|1.011761|0/3|
|Merge54, 32 workers|163.219758|163.834601|163.055960|1.001600|1/3|0.999012|2/3|

The three ON/REF primary ratios are:

- 16 workers: **1.218589322, 1.021849993, 1.002374189**.
- 32 workers: **1.118324879, 1.013901343, 0.995380410**.

The large first-block slowdowns remain in the result. No observations were
discarded, and no cache, frequency, stack or scheduler explanation is established.
Three blocks do not establish a precise general penalty. All four OFF/REF median
calibrations pass the declared factor1.03 guard. The useful-signal rule requires
a median below one and at least two wins against both controls at either primary
count. Neither primary meets it. Merge54's small ON/OFF improvement does not
reproduce against REF. No confidence interval or all-shape improvement is claimed.

|Case|ON/REF paired CPU time|Peak RSS MiB REF / OFF / ON|
|---|---:|---:|
|1×2, 16 workers|1.006485|83.00 / 82.50 / 81.00|
|1×2, 32 workers|1.002007|131.00 / 131.75 / 130.25|
|2×2, 32 workers|1.004208|285.25 / 281.50 / 280.00|
|Merge54, 32 workers|1.002341|1640.58 / 1607.88 / 1588.98|

All prospective RSS guards pass: ON ≤ each control×1.10+8 MiB. Lower observed
RSS does not establish structural memory savings. RSS includes loading and
priming; it is not minimum RAM, allocation accounting or swap/pressure evidence.
The full [assessment](artifacts/assessment.json) preserves every paired ratio.

## Validation, provenance and decision

Native ON passes **41 field and 138 core tests**, unfiltered. Generic ON passes
**14 field tests**, unfiltered. The two new field test functions independently
compare against Arkworks over 384 full-width scalar pairs at three projective
scales, exceptional/sign cases, and 64 changing 48-step chains. They check affine
outputs, raw coordinate equations, ordinary Z and unchanged inputs. Native and
generic use the same fixtures, so they are not distinct mathematical coverage.
All 193 test executions pass. OFF libraries/link and both ON/OFF SDK links pass.
Unchanged qualification is inherited by source identity; no all-shape adoption
qualification was run for this negative screen.

All **39 fresh exported proofs** pass independent Gnark verification. Both raw
readers agree on **5,160 SDK calls, 5,043 timed records, 78 receipts and 156 stream
hashes**, source identity and the decision. Both smoke reviews pass before the
guarded main launch. Captures precede late verification and analysis, with no
owned CPU cojobs during SDK recordings. Build, test, SDK, verifier and analysis
executions succeed on their first attempt; no successful capture is replayed.

Exact SDK SHA-256 bindings:

```text
REF b70a9472fa64ac6b6b41b227ca64ddfcf61c6443879c9187331eae6d5e22c153
OFF 6af7a60c1a54df209f7b5b2f05ee059b337c78b37d3ca6f0fdd671d8ab1fdb0b
ON  100b0629207b7c4b276716ddf13ccbb97f707124154d0acb9f1fe66dfc104962
```

The canonical inventory advances 61,104→61,143; final SHA-256 is
`ab290bf3ef3d535745621f21aa914423ff5d578abfb30f44ed80f2f6998104f8`.
Raw keys, requests, proofs, complete inventories, binaries and raw disassembly
remain private under `/home/ubuntu/src/zopro-perf-work/unscaled-mixed230`.

Preparation took 472.05s; smoke/main capture spans were 1.10s/64.98s. Active SDK
commands total 66.05s and Gnark commands 4.23s. Final review followed the last
verifier by 7.14s; protocol creation to review was 565.97s. These descriptive
spans use filesystem timestamps; active durations use xtask elapsed receipts,
not instrumented monotonic phase timers. Both declared time allowances were met.

The exact candidate is closed without formula/layout/parameter rescue or an
unchanged reroll. The next experiment needs a distinct work-removing mechanism.
Accepted full all-eight CPU/Metal comparisons remain in
[228](../2026-11-17-scaled-prefix-all228/README.md); the all-eight CPU 10× goal
remains active and unmet, **0/8**. No new AWS, phone, huge-circuit, memory-pressure
or throughput result is claimed. No cloud resource was created. This evidence
is committed locally; publication awaits approval.

[Source audit](artifacts/source-audit.json), [build review](artifacts/build-review.json),
[independent main review](artifacts/independent-main.json),
[timing](artifacts/timing.json), [sources/plans](artifacts/source-and-plans.tar.gz),
[checksums](SHA256SUMS).
