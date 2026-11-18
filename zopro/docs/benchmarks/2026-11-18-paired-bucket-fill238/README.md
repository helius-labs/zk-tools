# Paired independent G2 bucket fills

The paired-band native candidate **fails the complete-query screen and is not
adopted**. Primary32 latency is9.17% above current streaming, with0/3wins.
Grouped2x2/32 is24.89% slower, also0/3. All four threaded median nonregression
checks fail. There is no full-SDK follow-up or new proving-time claim.
The adopted paired weighted reducer remains unchanged.

## Mechanism and controls

The experiment combines adjacent original256-bucket routing bands into one
512-bucket staged-tree job. Every bucket retains its independent binary tree,
leaf order and mathematical joins. Affine pairs at matching levels share
batches/inversions; signed leaves, high-before-low Phi order, two partitions,
narrow top, merges and weighting retain their algorithms. Coalescing also
changes task granularity, temporary arrays and live scratch, so the measurement
does not isolate an inversion-only cause.

The [weighted reducer233](../2026-11-18-paired-weighted233/README.md) showed that
sharing inversions can help another phase. That new evidence motivates this
application to ordinary point fill; it does not predict its speedup. The
[closed tree200](../2026-11-17-bucket-tree-c7a200/README.md) is included only as
a mechanism control. Its result and the closed destination-round variant stay
unchanged. This fixed candidate changes only BAND256->512 in that tree source,
with no group/worker/compiler/layout sweep or rescue.

A single coherent current188-source native test artifact offers STREAM0
(current algorithm), TREE1(exact historical256 tree) and PAIR2(new512).
Three byte-identical independently backed executable files all participate,
one per matched block. New sources total191files:187originals exact, one
reversible G2 parent and three new children. All188 current originals recover;
field arithmetic and its source qualification are inherited unchanged.

## Matched complete-query screen

Ryzen9 9950X, Rust1.98.1/znver4, Cargo releaseO3/fatLTO/oneCGU. Three matched
rotated/reversed blocks cover1x2/2x2 at1/16/32workers and all three arms.
Three primes precede17timed queries and one checked postcall. Loading/oracle
hashing are outside the per-query timer; copying scalar words, grouping,split,
routing, allocation, fills, merging, weighted assembly and normalization are
inside. Each query output matches its retained independent Arkworks hash.
Input points are checked immutable and actual selected-route calls asserted.

Individual workers pin to CPU2 solo or CPUs0..15/0..31; coordinator uses the
matching process mask. Diagnostic worker stacks are512KiB. This is a complete
query caller, not SDK proof latency, default-SDK-stack or server-throughput
qualification. Every sample/backing remains. No owned CPU cojob overlaps
any smoke or main capture.

|Shape / workers|STREAM ms|TREE ms|PAIR ms|PAIR/STREAM paired median|Wins|PAIR/TREE paired median|Wins|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2 / 1|31.402257|37.951188|36.292793|1.155738351|0/3|0.956301877|3/3|
|1x2 / 16|3.228225|3.409018|3.487466|1.076613048|0/3|1.016146187|0/3|
|1x2 / 32|2.886850|2.948076|3.165197|1.091657862|0/3|1.084273977|0/3|
|2x2 / 1|33.762811|38.740226|40.205267|1.188733394|0/3|1.037651591|0/3|
|2x2 / 16|3.707413|4.079315|4.156484|1.121127740|0/3|1.018917206|0/3|
|2x2 / 32|3.355995|3.719524|4.216122|1.248938425|0/3|1.115427332|0/3|

Times are medians of process means; paired medians are not their ratios.
The directional rule requires primary32 PAIR/STREAM median<1 and>=2/3wins,
with all four threaded medians<=1.02. Neither part passes. TREE is a mechanism
control, not an adoption veto. There is no confidence interval or minimum gain
floor. PAIR's solo primary gain versus TREE(4.37%,3/3) remains recorded; it
does not improve on streaming or justify promoting a worker/shape subset.

|Shape / workers|PAIR/STREAM loop CPU ratio|Peak RSS MiB STREAM / TREE / PAIR|
|---|---:|---:|
|1x2 / 1|1.155747716|18.75 / 18.75 / 18.75|
|1x2 / 16|1.186561188|28.75 / 37.75 / 43.50|
|1x2 / 32|1.280941441|30.50 / 48.50 / 64.50|
|2x2 / 1|1.188725691|31.25 / 31.50 / 31.50|
|2x2 / 16|1.255008020|39.50 / 49.00 / 55.75|
|2x2 / 32|1.447570635|42.75 / 59.50 / 78.00|

CPU service includes between-query oracle hashing; it is not an exclusive
query-stage service metric. RSS includes loading, priming and process history.
Primary32 PAIR reaches64.5MiB versus30.5MiB for streaming; grouped32 reaches
78MiB versus42.75MiB. They are neither full-proof peaks nor minimum-RAM or
physical-pressure requirements. No cache/DRAM/stack/scheduler cause is proved.

The historical conservative extra_bytes formula uses the new BAND in routes,
20held windows, two compact intermediate buffers, borrowed pairs, adder scratch
and temporary stack-array charges. Reservation::analysis denies before tree
construction on unsupported dimensions/overflow/resources. The experiment
adds no retained key table, key copy, solver plan, production pool or fixed
memory cap. Larger proof-local batches and arrays remain real costs, despite
sharing inversions. Component success could not waive the separate10%+8MiB
full-proof RSS/adoption requirements; this screen already fails latency.

## Correctness and provenance

Two focused native tests pass, checking six full-query raw outputs across
stream/originaltree/pairedtree and two retained inputs, plus16generator-oracle
queries with identities/repeated/opposite points and scalar edges across
1/4/16/32-worker512KiB pools. Admission checks cover unsupported dimensions,
zero workers and overflow-sized lengths. Full generic noASM/noIFMA core
compilation passes. Unchanged field/split/oracle qualification is inherited;
no unchanged field/core suite is replayed.

Six smoke processes(1timed+3prime+1post) precede54main processes, totaling
**1164oracle-checked captured queries and924timed samples**. All60captures
and two build/test commands succeed first execution. Both semantic smoke
readers pass before guarded main dispatch. The separate reader reconstructs
60receipts/120stream hashes, means, ratios, calls, backing identities and source
recovery. A final publication check also binds both build receipts:62receipts/
124hashes. Oracle computations are asserted by the pinned native consumer;
this is not a new independent cryptographic verification run.

The first smoke semantic reader fails because xtask omits the harness's
SHIFTED_* variables from its command-environment allowlist. Its checked exit
prevents main dispatch. A new separately pinned reader skips only those
unavailable receipt fields; the full frozen plan and each printed SCREEN
record still bind arm/shape/workers/rounds. Other environment/argument/hash
checks remain, and the original independent reader stays unchanged. No native
source, executable, input, scientific plan/order or capture is changed/repeated.
The repaired review and both prerequisite readers pass before main.
[Receipt repair](artifacts/receipt-repair-note.md).

No new PMU/profile is needed to decide against this fixed recipe. No SDK proof,
Gnark export, new field tests, AWS or production change occurs. The62187-proof
inventory remains unchanged; current188 source hashes pass. This closes the
fixed paired-band fill composition without a band/worker/subset/layout rescue.

Preparation 230.56s; active captures 18.75s; protocol-to-independent-review
322.81s. Preparation/capture allowances pass. Descriptive file/xtask durations
exclude earlier history review and are not instrumented phase timers.

The adopted full-proof primary32 remains7.460937ms/128.75MiB, inherited from
[234's all-shape CPU/Metal comparison](../2026-11-18-paired-weighted-all234/README.md),
with the selector provenance in[236](../2026-11-18-paired-weighted-adoption236/README.md).
The full all-eight CPU10x goal remains active and unmet,0/8. Merge54 remains
163.135118ms without a supplied M5 reference. No freshAeglos/AWS, phone/ARM,
pressure/swap, huge-circuit or concurrent-throughput claim follows.

[Protocol](artifacts/protocol.md), [assessment](artifacts/assessment.json),
[independent review](artifacts/independent-main.json), [receipts](artifacts/receipts.json),
[timing](artifacts/timing.json), [sources/plans/raw captures](artifacts/source-and-plans.tar.gz),
[checksums](SHA256SUMS). Private point/scalar/key/witness/proof arrays, full
inventories and executable binaries are excluded. Local root:
`/home/ubuntu/src/zopro-perf-work/paired-bucket-fill238`. Committed locally;
publication awaits approval.
