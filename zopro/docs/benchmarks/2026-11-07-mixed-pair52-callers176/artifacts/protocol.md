# Hybrid R256 arithmetic: complete caller comparison

Frozen before the first timing observation on 7 November 2026. This is an
explicit hybrid integration trial, selected in the caller design before builds.
The earlier global signed-pair52 gate remains failed. The separate joint-complex
algorithm remains closed. Neither old experiment is replayed or promoted.

ON uses the exact qualified single-output signed-pair kernel for all eight-lane
R256 ordinary/fused products in both fields. Uniform R260 retains CIOS. OFF uses
accepted arithmetic. Both have coherently rebuilt field/core/driver inputs with
actual O3/native/fat-LTO/one-CGU driver flags. Qualification passes 36 field and
126 core tests, two new OFF tests, raw dispatch comparisons and generic
no-ASM/no-IFMA compilation. Kernel mathematical qualification is inherited by
exact runtime source hashes. Production176 and SDK6d83 are unchanged.

The frozen control389307b9 is the audited accepted-source diagnostic caller,
distinct from the production SDK. New native code reduces actual G1 IFMA sites
800 to640 and G2 1450 to1110. G2 instructions increase8590 to9938 and vector
stack-addressed rows603 to1080. These are static counts, not measured cycles,
dynamic spill counts, memory peaks or a predicted speedup.

Compare all24 contexts: G1/G2 x200/201/352/353 points x1/16/32 workers. These
public-generator fixtures check changing values, signs, tails, reuse and an
independent complete generator oracle. They do not represent arbitrary
full-width coordinates or a full SDK proof. Each invocation creates an existing
diagnostic Rayon pool with512KiB stacks and pins workers to CPU2 solo, CPUs0-15
for16, or CPUs0-31 for32. No production pool/table/key copy/retained plan is added.
Three checked128-round priming passes and a reset precede the measured interval.
Timing includes dispatch and one borrowed-reference array construction, followed
by output checking and JSON outside the interval. Require zero timed allocations
and equal scratch capacities/output hashes in every arm.

Run72 preflights of64 rounds and one checker, then15 fixed matched blocks across
all contexts and three arms. Latency uses8192 rounds per worker for every size.
This shorter fixed screen is selected before clocks; no duration pilot, adaptive
extension or reroll is allowed. Rotate context and arm order by block/context
and reverse odd blocks. Run processes serially under MALLOC_ARENA_MAX=32,
MALLOC_MMAP_THRESHOLD_=1073741824 and MALLOC_TRIM_THRESHOLD_=-1. Capture host,
topology/governor/kernel/tool observations; frequency is not locked. No owned
build/helper/analysis/hash/report/push overlaps any latency or profile collection.

For paired ratios use sorted zero-based ranks3/7/11 of15 as low/median/high,
reporting every raw ratio. These marginal rank intervals are not simultaneous
family-wise guarantees. All24 fresh-OFF/frozen intervals must lie within
[0.97,1.03]. All48 ON/OFF and ON/frozen nonregression intervals require high<=1.01.
For EACH curve, an equal-log aggregate of its four sizes at32 workers requires
high<1 and at least11/15 wins against BOTH controls. The all24 equal-log aggregate
requires the same. There are78 timing checks. No minimum positive-gain floor,
discarded blocks, favorable curve/size/thread subset or amended threshold.

After all1080 latency processes, collect18 grouped PMU calls and18 cycle profiles
across each curve at200w1,200w32 and352w32, in all three arms. Profile rounds are
2097152 solo or65536 at32; stat uses one quarter. Use cycles:u fixed period
16000000 and512 mmap pages. FIFO enable/disable brackets the existing measured
interval. Group cycles:u,instructions:u,cache-misses:u,branch-misses:u; require
numeric counts and at least99.9% running coverage. PMU panels are diagnostic,
not replicated causal cache-level measurements.

Each profile requires at least1000 samples, at most5% unknown, zero lost events,
the exact period, executable MMAP ownership, TIDs from FORK metadata, assigned
worker CPUs and legal taskset CPUs for other owned threads. Require at least95%
worker samples and95% attribution to native G1/G2, vector inverse or scalar
inverse functions. Flat whole-function attribution is not critical-path time.
Inadequate profiles remain failed evidence without extension.

Only after recordings, chown each exact raw path to the campaign UID/GID while
preserving its hash. Decode each recording once for samples, metadata,
attributes and loss. Total1279 commands:72 preflights+1checker+1080 latency+
18stat+18profiles+90offline;1188 caller processes. Freeze source/binary/protocol,
input/source/library identities, host and serialized xtask before execution.
An independently implemented reader reconstructs intervals, timing checks,
ownership, resource summaries and PMU coverage from raw command receipts.

A positive caller gate selects a fresh full SDK all-eight+merge54 comparison,
resource/fallback checks and independent Gnark qualification before adoption.
This campaign adds no SDK proof. Canonical count34929 and latest full SDK
fused-epoch-sdk176 remain authoritative. Its accepted local32-worker1x2 was
8.018344ms; earlier7.915095/7.928660ms used the same accepted SDK. No slower build
was adopted. There is no new AWS/Aeglos/mobile/huge/pressure/minimum-RAM claim.
The verified CPU10x goal remains active and unmet on all eight shapes; final
comparisons must include supplied M5 CPU and Metal.
