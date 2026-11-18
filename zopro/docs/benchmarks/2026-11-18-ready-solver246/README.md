# Producer-ready solver: C7a screen

The prototype improves 2×2 and merge54 latency, but **does not pass the
96-core primary screen** and is not adopted. All90 exported proofs pass
independent Gnark verification on the server and after transport.

At96 cores, 1×2 is12.017ms versus12.544ms for the accepted reference and
11.866ms for the same new executable with readiness disabled. The disabled
control is itself about5.1% faster than the accepted build in paired blocks,
outside the declared calibration range. These observations are retained;
the primary improvement over the accepted build cannot be attributed to the
new dependency execution alone.

## Change

The private opt-in prototype removes global joins between solver dependency
bands. It keeps the existing arithmetic leaves, flattens local refinements,
and derives leaf edges from unique physical wire/slot producers. An existing
Rayon scope runs ready leaves, inlining one ready successor and spawning
others. There is no blocking collective barrier or new worker pool.
Per-proof predecessor counters publish producer writes before consumers run;
errors suppress subsequent work and the scope drains before storage drops.

Construction scratch, retained graph arrays and each concurrent proof's atomic
counts use the existing resource admission policy. Denied admission preserves
the previous solver. There is no copied operation plan, point array or key
table. Production source remains unchanged; this is an isolated source snapshot.

## Plain full-proof latency

Fresh c7a.24xlarge,96 physical cores without SMT. Fixed worker affinity,
three rotated/reversed blocks and three independent executable copies.
Each process primes three requests and times151 for1×2,51 for2×2 or17 for
merge54. All plain latency completes before profiling. No other owned CPU
work runs on the benchmark host during SDK capture.

REF is exact current SDKd523. OFF/ON use the same newly linked executable,
with a process-stable readiness flag. Times are medians of three process means.
Paired ratios are medians of within-block ratios and can differ from ratios
of the displayed times. These are descriptive screening observations, without
confidence intervals or full-suite adoption qualification.

|Circuit|Cores|REF ms|OFF ms|ON ms|Paired ON/REF|Paired ON/OFF|
|---|---:|---:|---:|---:|---:|---:|
|1×2|32|10.957765|10.866974|10.575763|0.970813|0.973996|
|1×2|96|12.543526|11.866462|12.017152|0.958036|1.012699|
|2×2|32|14.556649|14.507800|13.616938|0.933302|0.938594|
|2×2|96|16.353245|16.182577|15.206101|0.929852|0.936405|
|merge54|32|141.919793|142.233708|139.876368|0.967056|0.964922|
|merge54|96|124.058469|121.990973|116.436998|0.933984|0.954472|

The **25/30** screen checks pass. Failures are primary96 gain versus OFF on
two of three backings, primary96 CPU time versus OFF (+5.35%), primary96
OFF/REF latency calibration, and merge54/96 RSS. There is no minimum-gain
floor or favorable-subset adoption. Positive results on other contexts remain
eligible evidence for a distinct follow-up; they are not discarded because
the intended96-core fix failed.

The supplied historical M5 references are37/24ms CPU/Metal for1×2 and59/30ms
for2×2. New ON at32 cores is3.499×/2.269× those1×2 references and
4.333×/2.203× for2×2. These are different-machine reference ratios, not a
matched Apple measurement. Merge54 has no supplied M5 reference. The
[current all-eight comparison](../2026-11-18-paired-weighted-all234/README.md)
and [broader C7a comparison](../2026-11-18-full-catalog-c7a244/README.md) remain
separate evidence. No all-eight CPU10× success is established.

## Memory and activation

Readiness activates for all six ON shape/core contexts and never activates
for REF/OFF preflights. Graph structure is the same at32 and96 cores.

|Circuit|Leaves|Edges|Roots|Construction peak charge MiB|Retained graph bytes|
|---|---:|---:|---:|---:|---:|
|1×2|97|84|63|3.780|3832|
|2×2|140|143|77|6.568|5444|
|merge54|2694|4166|877|143.118|103264|

Charges are requested-capacity accounting, not measured allocation or RAM
minima. The implementation reserves a temporary edge vector for the total
possible read count before deduplication. That is unnecessarily large for
merge54's actual4,166 edges. Its96-core ON process peak is2247.65MiB versus
2022.32MiB REF, exceeding the declared2232.56MiB guard. Lifetime RSS includes
loading, preparation, primes and allocator retention; the increase cannot be
equated one-for-one with the vector's requested capacity.

The next memory change is explicit: count unique edges before allocating
their output, then fill an exact-sized adjacency array in a second pass.
This changes construction, not the runtime readiness rule. It requires a new
qualified build and fresh comparison; no result here is retroactively repaired.

## Automatic profiles and limits

After latency, primary96 OFF/ON each run three matched observer, two-event
counter and dual-sample panels. Both events use fixed16,000,000 periods and
FIFO control around the warmed SDK interval. All six recordings pass event,
sample-count, unknown-symbol, zero-LOST and SDK process/thread/CPU/mapping
checks. All12 counter readings report100% scheduled.

Overall observer qualification **fails**: OFF stat/plain is1.072598 and
dual/plain1.088573, exceeding factor1.05. ON stat/plain1.036001 and
dual/plain1.048913 pass. Counted ON/OFF instructions rise2.33% and cycles4.03%
in these perturbed runs. These are not accepted unprofiled cost estimates.
Flat samples include inlining/skid and are not critical-path or removable
wall-time fractions. All profile rows and failed observer checks are retained.

## Qualification and provenance

The first native build passes145 unfiltered core tests. Review then finds a
tag mask that would preserve the existing Poseidon TASK marker and reject
tagged graphs into fallback. The corrected build passes five focused tests:
dependency publication/exactly-once/concurrent reuse; error suppression and
reuse; panic propagation; solver-vector equivalence across refinements and
1/4/16/96workers; and tagged Poseidon vector equivalence for all12 widths.
Generic no-ASM/no-IFMA compilation passes. This is not mobile/ARM execution.

All corrections precede the first SDK call. An initial workflow schema error,
missed test initializer, generic-command argument error and tooling borrow
error are retained with the tag correction. No successful SDK capture,
verification or decoder is replayed. The actual tagged/untagged operation
layout and physical outputs are checked before graph construction proceeds.

The candidate modifies solver.rs and a Poseidon test file, adds ready.rs,
and leaves186 other original source files exact. All188 actual production
files remain unchanged. Candidate SDK SHA-256:
`1e0cdbfa3b0ee307b74fa670d93ef15c3c685825e287adcaaf3a405326061549`.
Reference SDK: `d523f49e8548620c4373a1ab4be2712d705709fdd7a590c5ed73e9aff97424e3`.

The run completes8748 SDK calls,8478 timed records and90 fresh exports.
All90 pass server and host Gnark checks. Canonical inventory advances to64301,
SHA-256 `626a63f74bda9e3ec1e916e8da9d6836e6a035db3b1c20b1305ccdd0b314a19e`.
Host re-verification adds no distinct proofs. The server command runs3m48.843s.
Perf7.0.14 tool hashes match before/after. The30GiB/no-swap unit is benchmark
headroom, not an architectural cap. Its console512KiB peak is not used as
prover RSS. No pressure/swap, minimum-RAM or concurrent SDK throughput result
is claimed; concurrency tests cover dependency execution correctness.

Cleanup confirms instance termination, absent tagged EBS, deleted private
bucket/objects/multipart uploads, and removed secret parameter files.
No owned benchmark resource remains. The full goal stays active and unmet.
This report and prototype are committed locally; not pushed.

[Protocol](artifacts/protocol.md), [screen](artifacts/decision.json),
[profiles](artifacts/analysis.json.gz), [qualification](artifacts/native-qualification.json),
[checksums](SHA256SUMS). Source patches and tooling are included. Private
keys, requests, proofs, binaries and raw PID/TID/address/profile streams are
excluded.
