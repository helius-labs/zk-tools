# Sparse production query deadlines226

In the retained exploratory traces, A and G2 usually return almost together.
At32 workers the median G2-minus-A return gap is3.1–4.5microseconds, followed
by9.3–11.0microseconds to canonical completion. At16 workers G2 usually
returns later, by24–45microseconds, with14.7–16.7microseconds after the later
return. These observations do not expose a large trailing K/H interval or
justify treating G2's entire inclusive span as removable proof time.

**Full protocol qualification is false.** A smoke reader incorrectly required
canonical solving to run on the SDK's main thread. Main capture was started
before that failed reader result was checked. Canonical solving actually runs
on an owned Rayon worker. The reader was repaired after capture, and both
independent raw readers subsequently agree on all spans and derived values.
The fixed observer-overhead and semantic boundary checks pass, but that does
not retroactively satisfy the announced smoke-first sequence. All observations
remain exploratory; no timing or source adoption follows.

|Workers|Block|A span ms|G2 span ms|G2−A return ms|Canonical after later return ms|G2 returns later /251|
|---:|---:|---:|---:|---:|---:|---:|
|16|0|6.378358|6.493855|0.037530|0.015209|220|
|16|1|6.425868|6.520375|0.045005|0.014728|214|
|16|2|6.439985|6.538820|0.024206|0.016731|213|
|32|0|4.981817|5.004288|0.003547|0.009318|168|
|32|1|4.981095|4.990914|0.003116|0.009899|162|
|32|2|4.993980|5.005782|0.004479|0.011000|160|

Values are separate per-request medians and need not add or subtract to the
displayed medians. G2 returns after A in647/753 observed16-worker requests
and490/753 at32. The canonical residual exceeds0.10ms in0/753 and7/753
respectively. The threshold is a predeclared descriptive scale, not a selected
performance gate. The residual includes any remaining K/H completion, join
and point export; it is not exclusive K/H CPU time. Source mapping identifies
the single G1 canonical call as A in this fused-query branch.

Rayon callers can execute helped work while waiting for descendants. A return
timestamp is not the time when all that query's arithmetic first became ready,
and its inclusive duration is not exclusive CPU service. Near-coincident
returns do not prove one CPU, memory or scheduling bottleneck. Together with
the [225 bypass result](../2026-11-17-g2-group-counterfactual225/README.md), they
argue against assuming that removing one observed grouping wall span saves
the same amount of complete-proof latency. Shared MSM work remains relevant.

|Workers|Block|A/G2 starts from SDK ms|Canonical ends from SDK ms|SDK after canonical ms|
|---:|---:|:---|---:|---:|
|16|0|1.689152 /1.691707|8.200330|0.550978|
|16|1|1.725751 /1.728226|8.263329|0.544346|
|16|2|1.715362 /1.717957|8.275974|0.546410|
|32|0|1.895071 /1.898948|6.923995|0.589591|
|32|1|1.891975 /1.894440|6.917624|0.582117|
|32|2|1.897626 /1.899499|6.936871|0.576376|

The roughly1.7–1.9ms prefix through query entry and0.54–0.59ms SDK tail remain
part of the measured request. They are not attributed to one source operation,
assumed serial CPU, or declared removable. This is a local Ryzen9950X study;
it establishes no C7a explanation or twofold improvement.

The exact accepted194 executable is unchanged:
`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.
Three existing REF backings from222 are reused. Only SDKprove, canonical-wire
solve, G1 canonical MSM and G2 exclusive query receive entry/return uprobes.
Eight events per backing, bound to exact native symbols and ELF file offsets,
preserve the prover's emitted code. There is no relink, private timestamp
module, frequent feed probe, PMU or software sampling event. No native/field/
portable suite is replayed for the unchanged consumer.

Two4-worker smoke processes precede12 main processes: three rotated/reversed
matched blocks ×16/32workers ×plain/traced. Main processes have251 timed
requests after three primes. FIFO excludes loading, primes and export, using
the same monotonic clock for all probe events. The14 SDK captures finish before
main decoding and Gnark checks, without owned CPU cojobs during recording.
Smoke decoding was completed before main capture; the failed smoke analysis
and successful repaired analysis have the sequencing limitation stated above.

|Workers|Plain SDK means ms, blocks0/1/2|Traced means ms|Paired observer median|
|---:|:---|:---|---:|
|16|8.715931 /8.755038 /8.792024|8.750309 /8.804443 /8.819175|1.003944|
|32|7.567446 /7.577736 /7.523155|7.585779 /7.560517 /7.582595|1.002423|

Both observer medians pass the predeclared factor1.05 guard; all individual
paired ratios are retained in the result. Three blocks give descriptive
observer estimates without confidence or unbiased-scheduling claims. Each
traced request has exactly one complete SDK/canonical/A/G2 span, with complete
entry/return stacks, expected containment, recorded PID/TID/CPU ownership and
executable MMAP binding. All1,506 main traced requests and six blocks pass
these checks, with zero LOST/throttle records. No sample or backing is removed.

All3,056 SDK calls internally verify. All14 fresh164-byte exports independently
pass pinned Gnark and public/image/freshness checks. The3,014 timed records,
two separate boundary/median/observer readers,63 SDK/offline receipts and126
stream hashes agree. This is a same-agent cross-check, not an external audit.
The repaired reader changes only canonical-thread membership, replacing the
incorrect main-PID equality with membership in the already recorded worker
set. Original source, pins, failed-analysis context and all recordings remain.
No successful capture or verifier is replayed and no threshold changes.

The reusable manager now refuses a main launch until both smoke semantic
quality and independent review pass. Dependent orchestration must inspect
command exits before invoking a next stage; a successful launch does not
imply its prerequisite passed. This prevents recurrence but does not repair
this run's history. All24 owned probe definitions were removed after each
capture phase; final tracefs inspection is empty. No other probes are touched.

Canonical ledger is now **60,099**:
`/home/ubuntu/src/zopro-perf-work/query-deadlines226/proof-sha256-after-main.json`,
SHA256 `cec1b30046897101546b87a053f7194f6d0e07d523c2237e33ff6d501ef86ef3`.
Raw TID/IP/time streams, perf data, keys, witnesses, proof bytes, full ledgers
and binaries remain private under that root. Published request metrics use
relative times and omit raw ownership records. No AWS or push occurs.
Production194/196 remains accepted. The all-eight CPU10× objective remains
active/unmet0/8, with unchanged [local](../2026-11-17-certified-all196/README.md)
and [C7a](../2026-11-17-accepted-c7a201/README.md) CPU/Metal comparisons.
