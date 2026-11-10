# Packed ternary complete-query protocol

The signed-digit/storage tests and native complete-query generator/carry tests
must pass before capture. No production change. This is a new one-time recode
and constant-time digit decode; the fixed prior ternary geometry remains closed
as a standalone candidate and is reused only as a mechanism comparator.

Four arms: OLD_BASE and RECIPROCAL use the qualified reciprocal consumer
28bce6f68ae92bf79acfc22161f9a59c17c6aaf34e432dd389cf26d2cceeb35a, B/T.
FRESH_BASE and PACKED use the new byte-identical-driver consumer, B/T. One source
child changes and one is added to that older diagnostic snapshot. This is not
a current accepted-production SDK build: current proof latencies are inherited.
Allocation/reservation, scalar reset, grouping, Phi, one-time recoding, feeds,
reductions, pool install and normalization are inside the complete-query timer.
Input loading, hashing, pool creation and output oracle checks are outside.
Every output, including primes and postcheck, must match its existing independent
Arkworks complete MSM result; point bytes and worker affinity must be preserved.

Retain original 1x2 and2x2 fixtures and1/16/32 workers, CPU2 solo or0..workers-1,
512KiB stacks. Exactly24 correctness preflights with one observed query each;
their clocks cannot select performance. After validating them, run31 blocks,
six contexts and four arms. Rotate cases block%6, reverse odd blocks; rotate
arms (block+originalcaseindex)%4, reverse odd sums. Three primes, seventeen
observed queries and one postcheck per latency process. Unit elapsed_ns/17.
All744 processes remain. Paired31-block ratios use sorted ranks10/16/22
(zero-based9/15/21), nominal97.0550626516% marginal sign coverage conditional
on exchangeability, without familywise/stationarity guarantee.

Freeze gates before preflight: all6 FRESH_BASE/OLD_BASE intervals contained
within[1/1.03,1.03]; PACKED vs both base arms upper<=1.03 in all6 contexts;
primary32 and equal-log two-population32 aggregate upper<1 and>=21/31 wins
against both base arms. PACKED/RECIPROCAL is reported in all6 contexts as a
mechanism comparator and cannot by itself qualify adoption. All6 peak-process
RSS guards require PACKED<=1.10*FRESH_BASE+8MiB and
PACKED<=1.05*RECIPROCAL+1MiB. These are comparative experiment regression
bounds, not an architectural RAM cap or minimum-RAM claim. 28 total decisions.
No exclusions, extensions, favorable groups, rerolls or threshold changes.

After all latency, record one201-query grouped PMU panel and one251-query cycles
profile per arm at primary32. FIFO gates cover only observed queries. Group
cycles:u/instructions:u/cache-misses:u/branch-misses:u must run100%. Fixed16M
sample period,512pages, >=1000samples, <=5%unknown, zeroLOST, own executable
MMAP/CPU/TID/event/gates must pass. One-shot counters/samples do not identify
exclusive costs or a critical-path cause. Decode each new recording once;
reuse the qualified existing loss reader. Profiling is part of this workflow.

24preflight processes/120queries;744latency processes/15624queries;
8diagnostic caller processes/1840queries;total17584queries and zero new proofs.
Freeze controls/source/compiler/deps/inputs/oracles/order/gates/producer and
separate raw reader before preflight. During capture only poll the exact live
handle; no overlapping helper/build/hash/analysis/report/push. Analyze only
after recording is terminal; retain every failure and slower case. Passing
this diagnostic still requires actual production resource admission and current
accepted-source SDK ALL8+54/fresh verification before adoption. Full goal
remains >=10x supplied M5 CPU ALL8 with CPU+Metal reporting, active and0/8.
