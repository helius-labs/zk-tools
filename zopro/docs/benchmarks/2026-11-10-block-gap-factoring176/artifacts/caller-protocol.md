# Fixed complete-query follow-up protocol

Prerequisite: all 128 native core tests passed, including 16,695 new suffix
comparisons. Original OFF/ON libraries are retained; the test-only Affine.double
compile error was repaired without replaying successful library builds. The
non-test child prefix is byte-identical before and after the repair.

Use coherent original-field OFF/ON complete-query drivers, native/O3/fat-LTO/one
CGU, allocator and environment matching the accepted campaign. Private export
calls the original g2_query_exclusive path. The candidate inserts only the fixed
block transform after Phi splitting. Original production source remains exact.
Default Rayon worker stacks, no explicit stack_size or RUST_MIN_STACK. Pin worker
i to CPUi; solo CPU2. Whole process taskset masks2/0-15/0-31 as appropriate.

Four contexts: 1x2-w1,1x2-w16,1x2-w32,2x2-w32. Preflight each OFF/ON once (three
checked primes and one query). Freeze both producer and separate same-agent
reader, all binary/input identities and this protocol before first preflight.
Then 15 fixed blocks, rotate/reverse context order and alternate OFF/ON order
by block+context. Each process has three checked primes plus17 timed queries.
No exclusions, extension, optional stop, changed geometry or unchanged rerun.
Each timed sample includes scalar reset copy, grouping, Phi split, sorting,
suffix construction, correction packing, allocations/drops, all original bucket
and weighted/Horner stages, result normalization. Parse/setup/oracle reads and
output checks are excluded. Every prime and timed output must match retained
independent Arkworks complete-query hashes; original points remain unchanged.

Unit is arithmetic mean of17 integer nanosecond samples. Report median of15
means. Pair ON/OFF by block; median and zero-based rank3/11 bounds. Bounds have
nominal96.484375% marginal median coverage conditional on independent exchangeable
blocks, no familywise/stationarity guarantee. Primary32 and equal-log aggregate
of the two32-worker shapes require upper<1 and>=10/15wins. All4 nonregression
upper bounds<=1.02. All4 RSS guards ONmax<=1.05*OFFmax+1MiB. No positive gain floor.
Passing selects separate full-SDK/fresh-control ALL8+54/Gnark/resource admission
qualification, never adoption. Failure closes this fixed representation without
worker/block/width/part/threshold rescue. Memory peaks are process maxima, not
minimum RAM. There is no fresh proof timing or proof export in this stage.

After all latency, automatic FIFO-gated grouped user cycles/instructions/cache
misses/branch misses (31 queries) and fixed16M cycles:u profiles (251queries)
cover OFF/ON primary16/32 and2x232. All recordings finish before ownership,
decode, analysis or publication. Require>=99.9% scheduled PMU coverage, >=1000
profile samples, <=5%unknown samples, correct PID/TID/CPU ownership, cycles:u,
fixed period and zeroLOST. Absolute period coverage remains unvalidated; flat
sample counts are descriptive, not exclusive removable costs or causal CPI.

No owned build/helper/hash/analysis/report work overlaps functional preflight or
recording; only poll the exact confirmed live handle until terminal. Timeout is
not terminal. All rows/failures retained. Canonical45,335 unchanged. Full goal
remains >=10x supplied M5 CPU ALL8 with CPU+Metal comparisons, active and unmet.
