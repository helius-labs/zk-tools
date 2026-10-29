# Compact index dictionary for scalar grouping

Component experiment on accepted production source175, prior commit cf4f5e7684.
The grouping HashMap currently repeats every 32-byte scalar key alongside a
class index. This candidate uses one eight-byte fingerprint/class-index slot
and compares all four words in the already-owned output values on a matching
fingerprint. Linear probing, at most 7/8 occupancy and geometric growth replace
the standard HashMap. Hashes use the same AHash RandomState family. A fingerprint
is never sufficient for equality. No scalar cutoff, zero handling, stable
class ordering, point arithmetic, key representation or persistent table changes.

This differs from borrowed-group-keys (25 October: 16-byte reference/index
entries in the standard HashMap) and radix-groups175 (29 October: radix sorting
packed fingerprint/input indices). Both earlier negative decisions remain.
There is no new third-party implementation. SRE arXiv2610.27452 and existing
FFT, GLV and inverse alternatives were checked and have prior experiments.

First qualify the pure builder from the original map allocation through its
final output, including temporary-map destruction. Sampling and resource
admission precede both builders and are excluded here. Returned metadata is
checked and destroyed outside the constructor timer, as the real caller needs
to consume it after construction. Full-output equality against an independent
BTreeMap oracle is checked after every priming and latency invocation. Declines
remain declines. Profile outputs receive the same checked primes and a checked
post-profile invocation; individual profile-loop outputs are not retained.
The exact original body is source-extracted with only its signature, result
type and reservation field adapted; arithmetic libraries remain frozen from
accepted production. The candidate changes only dictionary construction and
insertion in that extracted body.

Real A/G1 and B/G2 scalar vectors for all nine existing fixtures are produced by
the frozen accepted solver; cached/raw solver outputs must agree. Raw K is not
included because it is not the folded K/H production path. Scalars and witness
arrays are never persisted. Preparation occurs before CPU2-pinned single-thread
capture; no owned build/hash/archive/other CPU workload overlaps measurement.

Thirty-one fixed blocks rotate/reverse all 18 contexts and alternate original/
candidate order. Three priming calls per context/variant precede eight measured
constructors per cell. Identical state seeds are shared by matching variants
per block/round; no favorable seed or layout selection. All checks remain.
Primary A and B each require paired ratio upper bound below 1 and >=21/31 wins.
Every context upper bound must be <=1.03; the equal-log 18-context aggregate
must improve with upper<1 and >=21/31 wins. Bounds use sorted paired ranks
10..22, nominal 97.055% per comparison under independent/exchangeable blocks.
No gain floor, exclusion, extension, threshold adjustment or unchanged reroll.

All latency precedes two separate FIFO-gated 199Hz cycles:u profiles, one per
variant on primary B, 8192 builders after three primes. Oracle validation stays
outside the enabled interval. Profiles complete before loss audits and owner-
user decoding. No profiler timings enter the latency gate. All helper builds,
tests, input/source/library pins and profile control preparation precede capture.

The tested dictionary payload including old/new tables at growth must fit the
existing conservative grouping-map forecast at all input/capacity edges. This
does not establish allocator overhead, whole-prover RSS, stack or minimum RAM.
No full-proof, AWS, portable/device, pressure or concurrency speedup follows.
Only a passing component qualifies a later private full-SDK implementation.
The full all-eight CPU10x objective remains unmet; accepted CPU/Metal results
remain in the current-cpu-summary report. No production edit occurs here.
