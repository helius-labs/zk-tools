# Scaled prefixes in the current CPU pipeline

The new composition passes its exploratory screen: paired median 1×2 latency
improves **1.20% at 16 workers** and **0.49% at 32 workers** versus accepted194.
All three pairs improve against both controls at both counts. It is **pending
full qualification, not adopted**. Three blocks do not establish a precise small
gain. The all-eight CPU 10× goal remains unmet, 0/8.

This integrates the previously tested uniform scaled-prefix and typed uncarried
X-sum arithmetic into the current accepted pipeline. Current indexed G2 tile
reduction, fused preparation, generated-output certificates and all other
accepted source remain. Prefix arithmetic avoids some intermediate normalization;
the inverse bridge restores the scale before final coordinates. It adds no
lookup table, key copy, heap buffer, retained plan or pool. The older183/186
full-scope failures remain unchanged; this is a new implementation artifact.

## Retained full-proof results

Local Ryzen 9 9950X. Values below are medians of three process means, in ms.
Each process makes three untimed primes, then251 primary requests,51 for2×2,
or7 for merge54. The SDK interval includes solving, randomized proving,
internal verification and serialization; loading and proof export are outside.
One fixed global Rayon pool, default worker stacks, worker i pinned to CPU i;
main affinity covers the corresponding0–15 or0–31 CPUs. This is a benchmark
condition, not a new production affinity policy.

|Case|Accepted REF|Accepted-source OFF|New ON|ON/REF paired median|ON/OFF paired median|
|---|---:|---:|---:|---:|---:|
|1×2,16 workers|8.795949|8.886501|8.763309|0.987975|0.985226|
|1×2,32 workers|7.581865|7.645877|7.571195|0.995077|0.989030|
|2×2,32 workers|10.570131|10.681818|10.535341|0.981846|0.985185|
|Merge54,32 workers|161.766546|162.119693|161.562982|0.999506|0.990325|

Median process means and median paired ratios are different estimators; do not
derive the paired improvement from the ratio of the displayed marginal medians.
Primary and2×2 win3/3 against both controls; merge54 wins2/3. All observations
and individual ratios remain in [assessment](artifacts/assessment.json).
No confidence interval, new absolute record, AWS improvement or all-shape gain
is claimed by this short screen.

|Case|ON/REF paired CPU time|Peak RSS MiB REF / OFF / ON|
|---|---:|---:|
|1×2,16 workers|0.980816|83.00 /81.00 /81.75|
|1×2,32 workers|0.982785|130.75 /129.25 /129.25|
|2×2,32 workers|0.979374|287.00 /283.00 /281.75|
|Merge54,32 workers|0.993227|1650.88 /1622.20 /1756.13|

Merge54 ON peaks **8.25% above OFF** and6.38% above REF. It passes this screen's
prospective limit of each control×1.10+8MiB. The source-level scratch geometry
is unchanged and the reused capacity/small-stack tests pass; that does not prove
equal process RSS or explain the increase. RSS includes loading and priming.
It is not minimum RAM, private allocation accounting or pressure/swap evidence.
All four OFF/REF median calibration checks pass factor1.03.

## Qualification and provenance

The source snapshot contains186 files:179 accepted files, seven changed parents
and seven added children.172 accepted files remain byte-identical; arithmetic
parents/children match the previously qualified default182 source, with only
current indexed-tiles dispatch/module restored in the MSM parent. Current
inverse, G2 phi, solver, preparation and certificate source remain accepted.
The exact194 SDK driver/support, Rust1.98.1, znver4, O3/fat-LTO/one-codegen-unit
configuration and eight coherent direct libraries are retained or rebuilt.

Native field **39 tests pass**, and native core **138 tests pass**, including
independent field/point oracles, signs/tails, inverse branches and small-stack
worker cases. The initial core compilation failed because a test-only included
fixture was omitted. Its receipt and original preparation files are retained;
the exact existing fixture was added in a second preparation record. No
arithmetic changed, no core test ran in the failed compile, and the successful
field suite was inherited without replay. Fresh generic/integration/resource
qualification remains required for adoption. See [correction](artifacts/repair.md)
and [source audit](artifacts/source-audit-v2.json).

REF is exact accepted194, SHA `bcc8e077…01fa05b1`; OFF is retained fresh
accepted-source222, `fa101d27…e3b6e1565`; ON is
`b70a9472fa64ac6b6b41b227ca64ddfcf61c6443879c9187331eae6d5e22c153`.
All three independent executable backings participate for each arm. Cases/arms
rotate and reverse under the frozen protocol; no extension, exclusion or relink
follows the result.

All39 fresh exported proofs pass independent Gnark verification;5160 SDK calls
include primes, with5043 timed records. The inventory advances60099→60138,
SHA `156cd5261fee7169f5c0cc07bdb79dd306cdfc1d8c49159ae1ebec570af3a2b2`.
Both raw readers agree on samples, proof bindings, source identity and decisions.
Both smoke reviews passed **before** the guarded main launcher ran. All capture
precedes late verification/analysis, without owned CPU cojobs. All SDK/export
commands succeeded on their first execution; the preparation compile failure
above remains separate. No cloud resources or new profiler were used.

Preparation took about305s; smoke and main capture spans were1.09s and64.87s.
Active SDK commands total65.93s and Gnark commands4.28s; final review finished
6.31s after the last verifier. Protocol creation to final review was393.25s.
These descriptive spans use command/exit/review file timestamps, while active
command durations come from xtask receipts; they are not instrumented monotonic
phase timers. Both declared time allowances were met. [Timing](artifacts/timing.json).

The next step is qualification of this exact binary across all eight shapes
and merge54, with current CPU diagnostics, portable checks, memory and latency
tradeoffs. Full CPU/Metal comparisons will use that campaign's results; accepted
comparisons remain in [196](../2026-11-17-certified-all196/README.md).

[Protocol](artifacts/protocol.md), [build review](artifacts/build-review.json),
[independent review](artifacts/independent-main.json), [sources/plans](artifacts/source-and-plans.tar.gz),
[checksums](SHA256SUMS). Raw keys, requests, proofs, full inventories and binaries
remain private. This report is committed locally; publication awaits approval.
