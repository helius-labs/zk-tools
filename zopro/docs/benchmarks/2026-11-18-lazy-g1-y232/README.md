# Compact lazy G1 Y coordinates in the current pipeline

The exact candidate passes correctness checks but **does not improve the
primary full proof**. Paired ON/accepted latency medians are **1.78% slower at
16 workers** and **0.40% slower at 32**, with 0/3 and 1/3 wins. Both primary
comparisons also lose against fresh OFF. The composition is closed without
adoption; accepted scaled-prefix227 remains unchanged. There is no minimum
gain floor.

Merge54 has a positive descriptive paired median, 2.24% below accepted and 1.34%
below OFF. That observation is retained. It is not a qualified all-shape gain or
a reason to promote a shape subset after the primary rule fails.

## Change and arithmetic

Ordinary borrowed G1 buckets retain canonical X and a separate typed numerical
Y representative below2p. Vector updates skip the Y export reduction; scalar
tails, equal-X comparisons and final output normalize Y. First insertions,
projective spills, signs, collision handling, window geometry, scalar grouping,
weighted reduction and key inputs retain their original semantics.

The signed numerator forms2p+by−ay or3p−by−ay in[0,3p], then immediately
multiplies by a prefix below2p using current full-R260 REDC. The result is below
p+6p²/2²⁶⁰<2p. All limbs stay normalized,3p fits four words, and the general field
API keeps its original bounds. Independent full schoolbook-product/REDC and
Arkworks checks cover22,528 returned numerator lanes, including inclusive3p
inputs and output-fed state. Raw Y boundary import/export and zero
representatives also pass.

This is distinct from the closed
[G2 lazy-Y Window](../2026-11-07-g2-lazy-y-callers176/README.md), whose fused
R256 arithmetic needed different bounds/correction and was slower. That result,
the raw-R260 key/bucket trial and late-coordinate reload negatives remain closed.
No new mathematical algorithm is claimed.

The private snapshot has 192 files: 186 accepted files, four edited parents and
six new children. 182 original files remain exact; inverse edits recover all 186.
G1 buckets remain 64 bytes and records 1,600 bytes. No lookup table, key copy,
additional scratch field or pool is introduced. Safe terminal conversion retains
the pointer/capacity in16 tested length/capacity cases through2,048 elements.
That is evidence for this build, not a portable allocation or peak-memory bound.

## Matched full-proof screen

Ryzen 9 9950X, Rust 1.98.1, znver4, O3, fat LTO and one codegen unit. REF is exact
accepted227. OFF is a fresh coherent link with the private cfg absent; ON enables
it. Three independent executable backings per arm participate, one per matched
block. Cases/arms rotate and reverse under the frozen plan.

The SDK interval includes solving, fresh randomized proving, internal
verification and serialization. Loading and export are outside it. Each process
makes three primes, then251 timed calls for each primary,51 for2×2 or seven for
merge54. Default worker stacks, fixed worker/main affinities, the three accepted
opt-ins and allocator settings match both controls.

|Case|REF ms|OFF ms|ON ms|ON/REF paired median|Wins|ON/OFF paired median|Wins|
|---|---:|---:|---:|---:|---:|---:|---:|
|1×2,16 workers|8.585662|8.571027|8.738635|1.017817|0/3|1.016929|0/3|
|1×2,32 workers|7.523867|7.478262|7.520579|1.004004|1/3|1.004460|0/3|
|2×2,32 workers|10.402727|10.468521|10.601270|1.014054|0/3|1.008503|0/3|
|Merge54,32 workers|165.888300|166.201096|162.171369|0.977594|2/3|0.986565|3/3|

Times are medians of process means. Paired medians summarize matched ratios;
they differ from ratios of the displayed marginal medians. Thus32-worker ON's
slightly lower marginal median does not contradict its slower paired estimate.

Individual ON/REF primary ratios are:

- 16 workers:1.023258418,1.017817185,1.011346132.
- 32 workers:1.013078223,1.004003825,0.998089507.

Merge54 ON/REF ratios are0.940229782,1.012079820,0.977593773. All blocks remain,
including the large first-block difference. No scheduler, cache, spill, frequency
or memory-capacity cause is established. Three blocks do not establish precise
general penalties or a reliable merge54 speedup.

|Case|ON/REF paired CPU time|Peak RSS MiB REF / OFF / ON|
|---|---:|---:|
|1×2,16 workers|1.022258|82.00 /82.75 /81.25|
|1×2,32 workers|1.009813|130.00 /130.50 /132.25|
|2×2,32 workers|1.017247|292.75 /285.25 /285.00|
|Merge54,32 workers|0.996388|1659.73 /1664.89 /1675.02|

All four OFF/REF median calibrations pass factor1.03. All prospective RSS guards
pass ON≤each control×1.10+8MiB. RSS includes loading/primes and is not minimum
RAM, allocation accounting, structural savings or pressure/swap qualification.
The primary signal requires median<1 and≥2/3 wins against both controls at
either worker count. Neither primary meets it. No confidence interval, sample
exclusion, extension, threshold/geometry rescue or unchanged reroll follows.

## Qualification, emitted code and provenance

Native field (41) and core (141) unfiltered tests pass. Three additional selected complete
Window executions cover the missing inverse/canonical configurations0/1,0/0,
1/0 on the same pinned test binary; native defaults1/1 are in the unfiltered
suite. New Window tests include generator-coefficient bucket oracles, signs,
identities/equal/opposite points, tails, two feeds, terminal conversion and actual
1/4/16/32-worker pools with 512 KiB stacks. Generic noASM/noIFMA field tests (12)
and a complete generic core compile pass. Total 197 successful test executions; no new portable
native-arithmetic, phone, shared-proof throughput or physical-pressure claim.

Two preparation failures remain explicit. The initial snapshot audit reads a
binary constants member as UTF-8 and exits before builds; a byte-safe finalizer
checks the existing snapshot. Native core then fails compilation on concrete
reference Borrow calls and a diagnostic comparison of typed Y to Fq. Direct
dereferences and canonical diagnostic Y repair those adapters. No core tests
execute in that failed compile. Successful field tests are inherited without
replay; the continuation runs only remaining core/build/link stages.
[Preparation note](artifacts/preparation-repair.md),
[native repair](artifacts/native-repair.md), [build review](artifacts/build-review.json).
The first preparer failure is a tool-transcript observation; native compile
failure receipts and original source copies are retained in the archive.

Linked G1 true/signed borrowed kernel grows26,188→26,257 bytes and4,433→4,450
static instruction sites; both have800 static IFMA sites across their branches.
REF/OFF G1 bytes match exactly. G2 remains50,804 bytes/8,477 sites/1,450 IFMA
sites, but REF/ON raw symbol bytes differ; binary identity is not claimed.
G2 source bodies are unchanged. Static sites are not retired instructions,
dynamic spills or speed predictions. No new performance profile is recorded
after this negative screen. [Codegen](artifacts/codegen.json).

All 39 fresh exported proofs pass independent Gnark verification, with 5,160 SDK
calls/5,043 timed records, 78 receipts and 156 stream hashes. Both readers agree
on source recovery, counts and decisions. Both smoke readers pass before the
guarded main launcher. Capture precedes late verification/analysis; no owned CPU
cojob overlaps any SDK recording. SDK, verifier and raw-reader executions pass
first attempt. No unchanged successful test configuration is repeated.

```text
REF b70a9472fa64ac6b6b41b227ca64ddfcf61c6443879c9187331eae6d5e22c153
OFF 9de535513754b1331c3e6f1a08613b31c0a0c45721809bfbd78eb11cde1eeedb
ON  9b2f92e54a866d455151821445d137815c30fac8657c3ad8aaa35e4c55e479c8
```

The canonical inventory advances61,143→61,182; final SHA-256 is
`b13f8652c75305401c54c2240014100aea4e9e33c4da60b86fc56fdeefdb2090`.
Preparation takes889.93s; smoke/main capture spans are1.13s/64.57s. Active SDK
commands total65.68s and Gnark4.38s. Protocol-to-final-review is978.40s; both
allowances pass. These are descriptive filesystem spans and xtask durations,
not instrumented monotonic timers. Research precedes protocol creation.

Accepted all-eight CPU/Metal comparisons remain
[228](../2026-11-17-scaled-prefix-all228/README.md); the all-eight CPU 10× goal
stays active and unmet, 0/8. Exact accepted227 still has no new AWS measurement.
No cloud resources or production changes occur. Next work needs a distinct
reduction of main-path arithmetic/work, not another unchanged lazy-coordinate
or conversion-layout trial.

[Assessment](artifacts/assessment.json), [protocol](artifacts/protocol.md),
[independent review](artifacts/independent-main.json), [timing](artifacts/timing.json),
[sources/plans/receipts](artifacts/source-and-plans.tar.gz), [checksums](SHA256SUMS).
Private keys/requests/scalars/points/proofs/full inventories/binaries and raw
disassembly are excluded. Local root:`/home/ubuntu/src/zopro-perf-work/lazy-g1-y232`.
Evidence is committed locally; publication awaits approval.
