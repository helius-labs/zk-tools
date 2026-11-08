# Complete DFT16 caller protocol, declared before any caller execution

Objective remains verified CPU AVX single-proof >=10x supplied M5 CPU on ALL8
plus CPU/Metal comparisons. This component experiment can only select the next
real FFT/Euler stage. Production176/SDK6d83/canonical35171 unchanged.

Fixed implementation: byte-exact qualified parent kernel.rs and original-within
fragment. No alternative DIT transpose, direction/root subset, arithmetic,
compiler/attribute/layout/worker/host rescue. Two independent16-point sequences,
64 changing fixtures per logical worker, four Pair vectors per fixture. Root
construction, gold parsing, pool creation and validation are outside timers;
all transform loads/shuffles/add/sub/corrections/four vector products/six dummy
lanes/stores/blackboxes and both DIT bit reversals remain inside.

48contexts = roots1,3,5,7,9,11,13,15 x DIF/DIT x workers1/16/32. Three arms:
FROZEN original-only diagnostic newly built from accepted field/source,
fresh OFF and ON. FROZEN is NOT a historical captured full-FFT/SDK artifact.
All link identical field1200 with -O/native/fatLTO/oneCGU/debug1. OFF/ON compile
same source with only compile-time candidate selection. FROZEN's tag cfg selects
original validation/label only. Same fixture/root type, allocator and pool setup.

Each process has three128-call primes, every fixture checked and reset each time.
Latency rounds32768 per worker. One preflight64 per context/arm (144processes),
then15 rotated/reversed blocks x48contexts x3arms (2160processes). No pilot,
exclusions, extension, reroll, wins subset or percentage gain floor. Every trial
finishes before offline analysis, regardless of apparent performance.

Pool: existing Rayon,512KiB stacks; solo pinned CPU2,16workers CPUs0..15,
32workers CPUs0..31. Input fixture index is logical, not guaranteed execution
thread. Record every execution worker TID/CPU and verify affinity after work.
Pool and retained fixture capacity remain fixed. Require zero allocation requests
in timed regions. Driver/global allocator is diagnostic-only, not prover policy.

Independent Arkworks16x16 matrices bind the exact root and order, then exponentiate
by rounds/64 for each fixture visit count. Generate hashes for ALL32output fields
of EACHfixture, EACHlogical worker0..31, eachroot/order and rounds64/128/32768/
65536/262144. Snapshot hash inputs canonical little-endian words. No vector/new
kernel executes in gold generation. Gold readback, primes and final checks compare
all64fixture hashes per worker. Initial raw512bit inputs share only a deterministic
byte generator, reduced independently by each field implementation.

Timing gates use paired per-process joined ns/round (one worker's transform count;
also report total throughput separately, never call this single-proof latency).
Sorted paired ratio ranks3/7/11 of15 give the stated marginal interval/median;
no simultaneous-coverage claim. ALL48 OFF/FROZEN intervals must lie within
[1/1.03,1.03]. All96 ON/control upper ratios <=1.02. Aggregate equal-log ALL48
must have upper<1 and >=10/15 block wins against BOTH controls. Also report
three worker aggregates and two direction aggregates without selecting subsets.
All outputs, priming, allocations, ownership/affinity/rounds/capacity checks pass.
Any failed qualification/calibration/nonregression/aggregate check rejects the
full recipe without native timing repeat or SDK selection. No synthetic RAM cap.
Caller peak RSS is recorded, NOT prover RAM/minimum RAM or mobile qualification.

After ALL latency recordings, representative diagnostics use root1,DIF/DIT,
workers1/16/32,all3arms:18 grouped PMU65536 and18 cycles profiles262144 calls/worker.
Same warm/check/reset then FIFOACK enables only work. Profile cycles:u period100000,
mmap512, zeroLOST,>=1000samples each,unknown<=5%,assigned worker sample share>=95%,
kernel/loop plus field correction/inverse share>=95%. Explicit allowed mangled
symbols come from actual compiled artifact inventories before clocks. No unknown
period/run length is selected after seeing profiles. PMU cycles/instructions/
cache-misses/branch-misses running>=99.9%; retain actual scheduling fractions.
Single panels describe observations, not replicated causal uncertainty or proof
speed fractions. No generic-symbol threshold or sparse oldDFT8 profile inheritance.

Compile and qualify all actual callers/goldens and inspect NEW actual loop codegen
ONCE before recording. Compile/freeze full plan/analyzer/offline reviewer and
quality decoder before ANYpreflight. Capture metadata+alllatency+allPMU/profiles
before ANYdecoding/hash/helper/analysis/report/push. Serialize xtask and do not
run other owned CPU work during ANYrecording including preflights. Do not modify
qualified kernel, table layout or loop code after codegen/recording for rescue.

Passing this complete caller only selects a separate real packed whole-FFT/Euler
operator (all boundaries/resources/fallback/permutations), then actual verified
SDK/all8+54/primary16/Gnark/resource/portable before adoption. Publish/commit AND
push finished caller experiment positive or negative, politely, with inherited
all8 CPU/Metal comparisons. No active goal completion from this component result.
