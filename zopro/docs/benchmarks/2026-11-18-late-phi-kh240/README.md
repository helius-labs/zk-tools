# Final-only G1 endomorphism in the fused K/H query

The new complete-query screen is **negative**. At32 workers, independent GLV
components with a final-only endomorphism are9.28% slower in paired medians,
with0/3wins. The declared primary progression rule fails; there is no SDK
follow-up or adoption. The qualified paired weighted reducer remains adopted.

## Execution change

The current balanced fused K/H stream covers255 bits in20 windows. The prototype
splits scalars into two signed components and processes20 independent
(component,span) jobs. Each component gets its own weighted reductions and
127-bit Horner sum; the G1 map is applied once to the final high sum.

This differs from the closed [G1 GLV attempt175](../2026-10-29-glv-kh-borrowed175/README.md),
which used10 spans and two point-range partitions, transformed temporary buckets,
and merged bucket arrays. That negative result remains closed. The new plan
eliminates those transformations/merges relative to that historical algorithm;
relative to current production, it adds splitting and uses69632 versus71680
weighted bucket slots. Both current and new plans have20 fill jobs. No2× gain
was predicted or inferred from operation counts.

The new route adds32 bytes per K/H scalar, about1.88MiB for the actual61451 rows.
It does not add a transformed key, lookup table, point-range partition, pool or
shared mutable cache. This diagnostic has no production resource admission or
universal memory forecast; it would require both before any adoption.

Source has189 files:187 of188 originals are exact, one balanced-module parent
only appends a test module, and one child contains the new plan. Original
function bodies and field arithmetic are unchanged. No production source changes.
The test-only route is not a newly linked production SDK.

## Correctness and fixed measurement

An independent Arkworks full variable-base MSM uses the exact Euler-image K/H
points and retained canonical scalars. It decodes Montgomery coordinates
explicitly, checks every point on the curve, and binds identity flags to the
retained scalar cache. K has30529 points; H uses the actual30922-point prefix,
without padding it to32768. No witness solve or FFT is repeated.

Two focused native test functions pass: both routes match that real-input
oracle;1021 scalar cases check signed digits and the G1 endomorphism relation;
12 generator-query cases cover empty/tail/sign/identity/opposite cases at
1/4/16/32 workers. The generic whole-core compile passes, but the test-only new
route is not a portable native-arithmetic implementation qualification. Field
and scalar-split qualification is inherited by unchanged source.

One coherent Rust1.98.1/znver4 release test executable contains both routes.
Three byte-identical independently written backings all participate, one per
matched block. Primary K/H is measured at1/16/32 workers over three fixed
rotated/reversed blocks, with alternating arm order. Workers pin individually
to CPU2 solo or CPUs0–15/0–31, with512KiB diagnostic stacks.

The timer includes splitting/allocation, digit decoding, fills, weighted
reductions, Horner assembly, final mapping/combination, normalization and temporary
drops. Loading/pool creation and oracle checks are outside each query timer;
aggregate CPU includes between-query output checks. All points and scalar words
are checked unchanged after each process. This is a complete K/H query, not a
full proof or critical-path measurement.

Two four-worker smokes precede18 main processes. Each has three checked primes,
one or17 timed queries, and one checked postcall: **388 checked queries and
308 timed records** total. All captures run without owned CPU cojobs.

## Results

|Workers|Current ms|Final-only phi ms|Paired ratio|Wins/3|Paired CPU ratio|Peak RSS current/new MiB|
|---|---:|---:|---:|---:|---:|---:|
|1|32.568514|32.700636|1.004927|0|1.004934|18.75/20.75|
|16|3.339042|3.382532|1.013025|0|1.040732|29.00/29.50|
|32|2.358584|2.572734|1.092821|0|1.105776|32.50/33.50|

Times are medians of process means; ratios use within-block pairs. All three
32-worker ratios exceed1:1.064424,1.092821,1.113576. No confidence interval or
precise production penalty is claimed from three blocks. The16-worker median
nonregression guard and threaded RSS guards pass; the primary32 gain criterion
does not. Solo is diagnostic and retained. There is no minimum gain floor,
sample extension, exclusion, parameter sweep or promotion of a favorable subset.

Fewer mapped buckets and slightly fewer weighted slots did not improve this
caller. Splitting, digit handling, collision distribution, and execution
dependencies change together; no isolated cause follows. No new PMU recording
was selected after the negative screen. Existing235 profiles motivated inspecting
G1 work but do not supply removable wall-time fractions.

## Execution record and decision

Both native smokes succeeded, but the first semantic parser then failed because
libtest printed its test-name prefix before the first LATE record on the same
line. Main was not launched. Separately pinned reader-v2 helpers locate the
record marker within a line and preserve exactly-one-record and all binding
checks. A review-only entry consumes the successful smokes without replaying them.
Both semantic and separate smoke readers pass before guarded main execution.
No native source, consumer, input, plan, criterion or capture changed.

All actual native tests, oracle generation and capture processes succeed first
attempt. The separate reader reconstructs20 capture receipts,40 stream hashes,
the full sample counts, all paired statistics, backing identities and source
recovery. The final audit also checks34 preparation/capture workflow receipts
and68 stream hashes. This is a separate same-agent implementation, not an
external audit.

Protocol to first complete screen is483.82seconds, within1200seconds; active
capture is5.525seconds, within120seconds. Reporting preparation reaches572.37seconds.
Earlier research/history review is outside these clocks; no authentication wait
is subtracted. No successful scientific step was repeated.

The current C7a [package239](../2026-11-18-adopted-c7a239/README.md) stays ready.
This turn's actual STS check again failed expired SSO/refresh; no AWS resources
were created. The last two goal turns both saw that failure, but this turn made
independent research progress. No C7a result is claimed.

Canonical proof inventory remains62191, SHA256
`c2407f7fd7d43e32a685db92664fc94facac7c2608ba7843419be07629700cf3`.
This experiment adds no SDK calls or Gnark exports. Current full-proof CPU/Metal
comparisons remain [234](../2026-11-18-paired-weighted-all234/README.md), primary32
7.460937ms/128.75MiB, with236 adoption provenance. The all-eight verified CPU10×
goal remains active and unmet0/8. No minimum-RAM, pressure, phone, huge-circuit or
concurrent-proof throughput claim follows.

Related primary literature reviewed before this prototype:
[Hrabar's 2026 thesis](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403)
studies scalar factoring/clustering, overlapping earlier retained work;
[Longa's fused field-products paper](https://eprint.iacr.org/2022/367) concerns
arithmetic already represented in existing field/history checks. No published
speedup is transferred to this execution plan.

[Assessment](artifacts/assessment.json), [protocol](artifacts/protocol.md),
[reader](artifacts/independent-main.json), [repair](artifacts/reader-repair.json),
[timing](artifacts/timing.json), [sources/plans/raw commands](artifacts/sources-plans-raw.tar.gz).
Private point/scalar/identity data, raw oracle output, proofs, full inventories,
executables and PID/TID streams are excluded. No push is performed.
