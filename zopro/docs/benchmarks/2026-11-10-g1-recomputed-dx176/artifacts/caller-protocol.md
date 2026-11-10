# Complete general-MSM caller protocol

Prerequisite PASS:130core tests,2,393,304 exact old-path slots and independent
Arkworks slots over4,924cases,122threaded/shared cases with53actual workers,
1280-byte/alignment64 record and checked-size tests; native OFF/ON and no_ifma
exclusion builds. This guard build used native field1200, not a physical ARM test.

This stage exercises public msm_g1/msm_g2 with immutable Fr inputs. Fixture
canonical-to-Fr decoding is outside the timer; internal canonical conversion,
scalar grouping, all window arithmetic, allocation/drop and final affine
normalization are timed. These are complete general MSMs, not SDK proof queries:
G2 does not use the proof-specific exclusive Phi2 entry. The unchanged G2 API is
a control. Do not compare its milliseconds directly with prior Phi2 callers or
call these full-proof observations. Every prime and timed output must match
retained independent Arkworks full-query hashes; all points/scalars unchanged.

REF reuses accepted busy-reset176 core, OFF/ON reuse just-qualified coherent
cores against exact original field1200 and original dependency libraries. Same
driver native/O3/fat-LTO/oneCGU, original allocator/environment/dense busy reset.
Default Rayon stacks, no explicit stack_size or RUST_MIN_STACK. Pin worker i to
CPUi, soloCPU2, taskset2/0-15/0-31. No successful kernel/build/oracle replay.

Six fixed contexts:1x2-A-w1,1x2-A-w16,1x2-A-w32,2x2-A-w32,1x2-B-w32,2x2-B-w32.
Preflight each of three arms once:3checked primes+1query,18processes. Then15
fixed rotated/reversed context/arm blocks,17queries+3checked primes per process,
270latency processes. Unit arithmetic mean of17integer ns, median of15process
means. Pair ON/OFF and ON/REF by block. Bounds sorted zero-based ranks3/11;
nominal96.484375%marginal coverage conditional on independent exchangeable
blocks, without familywise/stationarity guarantee. No exclusions/extensions.

Necessary full-SDK selection:all6OFF/REF intervals within factor1.03;all12
ON/control nonregression upper<=1.02;primary1x2-A32 and equal-log two-A32
aggregate upper<1 and>=10/15wins against both controls;all12RSS guards
ONmax<=1.05*controlmax+1MiB;all12diagnostic quality checks.46total checks.
No minimum positive gain floor. G2 controls need no gain. Passing only permits
full-SDK ALL8+54/primary1/16/32/fresh accepted control/Gnark/resource qualification.
No production adoption, full-proof claim or favorable-worker/size subset here.
Failure closes this fixed record layout; no field/record/threshold rescue or
unchanged rerun. Report all positive and negative evidence.

After all latency, FIFO-gated grouped cycles/instructions/cache misses/branch
misses,101queries, and fixed16M cycles:u profiles,1021queries, OFF/ON primaryA16,
primaryA32 and primaryB32. Chosen before any clock; at least1000samples,<=5%
unknown, PID/TID/CPU/executable mapping/event/period ownership and zeroLOST;
PMU>=99.9%scheduled.12diagnostic processes followed by30offline commands.
No absolute period coverage or causal cost attribution claimed.

Freeze producer, separately compiled rank reader, driver/input/core/helper
identities and this protocol before preflight. Their receipt/sample parsers are
shared; independent integer fold and rank selection are a same-agent cross-check,
not an external audit. Capture all before any decode/analysis/publication.
During functional preflight/recording ONLYpoll exactconfirmedlivehandleuntil
terminal; timeout notterminal. No owned helper/build/hash/analysis/report overlap.
Canonical45335 unchanged; no new proof exported. FullCPU10xALL8goalactive0/8.
