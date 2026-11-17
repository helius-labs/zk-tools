# Central denominator validation in G2 filling

**The fixed query screen does not advance.** Primary32 has a favorable paired
median of0.997817, but improves in only2/3blocks. The declared rule requires all
three. All other directional nonregression checks pass. This short screen does
not establish either a repeatable gain or a slowdown. Production is unchanged.

The original feeder compares incoming and finite-bucket X before batch admission.
The candidate removes that comparison and preserves the existing scalar inverse's
`Option`: a zero product rejects the complete batch before any bucket write.
For Fq2, the denominator norm is zero exactly when the denominator is zero.
Rejected batches use complete signed curve additions to their distinct destinations.
Ordinary norm/prefix/inverse/reverse arithmetic stays the same.

Unlike the earlier [G2 packed filter](../2026-11-10-g2-packed-filter176/README.md),
this adds no per-vector classification or status array. The earlier
[G1 central-check result](../2026-10-31-deferred-equality176/README.md) also remains
closed. This G2 kernel retains its smaller prefix/norm record organization.
There is no new field primitive, point buffer, retained table or pool. Exceptional
batches pay speculative work and potentially many complete scalar additions;
adversarial-input performance is not qualified.

## Complete-query observations

Ryzen9950X, current179-file baseline, one OFF/ON test executable. Two retained
real queries,1/16/32workers,three rotated/reversed blocks across three separately
copied byte-identical executables. Twelve one-round preflights precede36main
processes. Main processes use3primes,17timed calls and1postcheck, with individual
CPU pinning and512KiB worker stacks. Reset,grouping,allocation,complete MSM and
affine normalization remain timed; loading,pools and oracle hashes are excluded.

|Query/workers|OFF ms|ON ms|Paired ratio|Wins/3|
|---|---:|---:|---:|---:|
|1×2/1|32.866285|32.924391|0.998465|2|
|1×2/16|3.355282|3.366830|1.004613|1|
|1×2/32|3.008365|3.002507|0.997817|2|
|2×2/1|35.742709|35.626332|0.996666|3|
|2×2/16|3.834535|3.875096|1.003200|0|
|2×2/32|3.486652|3.442737|0.984724|2|

Times are medians of process means; ratios pair blocks and need not equal ratios
of those medians. No confidence interval or adoption claim is made. Primary32
block ratios are0.962512/0.997817/1.012627. Every other context's median is within
the fixed1.02guard. No sample is discarded,extended or used to select a subset.
All816queries,including primes/postchecks,match retained independent Arkworks
MSM hashes; point inputs remain unchanged. These are query times,not proof times.

## Exact-consumer CPU diagnostic

After closing the screen, a separately frozen18-process diagnostic runs251timed
calls plus3primes/1postcheck in each plain/stat/cycles panel. It adds4,590checked
queries. All recording precedes decoding and analysis; the native executable is
unchanged. Whole-process PMUs include loading,primes,checks and teardown.

|Block|Instructions/call OFF/ON (whole process÷255)|Instruction ratio|Cycle ratio|
|---|---:|---:|---:|
|0|512.204M/509.139M|0.994016|0.994853|
|1|511.883M/509.459M|0.995265|1.006495|
|2|511.820M/508.824M|0.994147|0.998050|

Instruction reduction is about0.47–0.60%; cycles are mixed. This supports a small
reduction in aggregate caller work, not a major arithmetic saving or a proof
latency prediction. All four coarse observer guards and twelve counter/sample
quality checks pass: counters run100%, six profiles contain26,734samples, unknown
symbols stay below5%, and decoded streams contain no LOST records. Fixed period
is16Muser cycles with512mmap pages. CPU attributes were not recorded; there is no
sample CPU-placement,MMAP ownership,warm-only or exclusive critical-path claim.
Profiles do not reopen the failed progression rule.

Seven selected native tests pass,129others are filtered and one benchmark ignored.
New checks cover signs,tails,zero products without writes,exceptional complete
Windows and1/4/16/32-worker pools. Existing field qualification is inherited.
The private snapshot has176byte-exact originals,three changed parents and one new
child. Offline recovery restores original G2 arithmetic; a separate same-agent
reader recomputes raw means/wins,checks185pins and matches samples to recorder
counts. This is not an external audit or full release qualification.

Primary32 lifetime RSS is28.5MiB in both arms; grouped32 is42.5/43.0MiB. These
are whole-query process peaks,not minimum RAM or full-proof peaks. No portable,
pressure,phone,huge-circuit,concurrent-throughput or AWS result is added.

The native build/test command takes61.18s. Screen result follows protocol freeze
by about3min. Counter recording initially fails for permissions before executing
that panel; an existing noninteractive privilege wrapper completes the remaining
panels without changing system settings or repeating the two completed plain
panels. Decoder repairs add missing IP display and omit an unrecorded CPU field;
recordings are reused. Initial failures and source versions remain in evidence.

Close this exact recipe without a mask,layout,threshold or worker-count rescue.
Validation-check removal has not supplied the substantial complete-caller work
reduction needed for the target. No production code changes or new SDK/Gnark
exports occur; ledger58033 remains exact. Accepted194 primary remains7.628251ms
locally at32workers. All eight CPU/Metal comparisons remain in the
[accepted qualification](../2026-11-17-certified-all196/README.md); CPU10× goal is
active and unmet,0/8. Merge54 remains163.384864ms without an M5 reference.
No push is authorized or performed.

[Protocol](artifacts/protocol.md),[query results](artifacts/result.json),
[profile results](artifacts/profile-result.json),[review](artifacts/review.json),
[sources and receipts](evidence.tar.gz),[checksums](SHA256SUMS).
Private root:`/home/ubuntu/src/zopro-perf-work/g2-central-equality198`.
