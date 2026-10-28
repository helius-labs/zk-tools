# Conclusions and next discriminating measurement

The exact adopted executable is now observed without a prover rebuild. The
previous goal turn was progress (source174 adoption); this turn adds complete
coarse wall phases and simultaneous sampled cycles,25verified exports and a
reusable checked trace decoder. It is diagnostic progress, not a new speedup.

Primary32 direct shared-MSM join median is5.546757ms. The median of per-request
SDK-minus-join durations within each of the three blocks is2.887880,
2.946079 and2.971778ms. This is measured from matched intervals, not subtraction
of independently reported medians. With that observed remainder unchanged,
halving MSM latency is insufficient for the3.7ms target. This conditional budget
is not a device-independent lower bound: tracing itself adds overhead and other
phases can be improved. Primary must reduce both its prefix/remainder and MSM
work; neither deleting a small sampled scheduler category nor G2 alone establishes
the requested jump. The original goal is retained in full,0/8achieved.

Primary32 phase medians: solver1.170829ms, gather/public/reclaim0.193304,
FFTproduct0.605934, post-product5.560433, SDK outside innercore0.778088.
The pre-solver phase is only0.090180ms; it is not a hidden millisecond-scale
allocation stage. These independent medians must not be summed as an exact
request decomposition. The solver has little aggregate sampled CPU but material
wall time; this directly warns against ranking targets solely by CPU shares.

An important new boundary observation is in remainders.json. The G2 raw routine
begins1.714265/1.545426/1.612232ms after the small join begins, and returns only
0.010179/0.016170/0.009748ms before that join ends. Its roughly4ms inclusive
duration therefore does not mean G2 is safely off the tail. At physical16 the
entry delay is0.752080/0.754043/0.953689ms. We have not separated when the G2
closure starts from its grouping/admission or other caller preparation. Nested
Rayon work inside callbacks further prevents assigning these spans as exclusive
G2 CPU. A simple statement that G1 alone finishes late would be unsupported.

Next concrete diagnostic: add coarse entry/return observations for the existing
g2_query_exclusive and scalar_groups/group_affine caller boundaries on the same
adopted binary, and relate them to this already measured join/raw boundary.
This distinguishes task-start delay from caller preparation before selecting a
CPU or scheduling change. Exact symbols exist at0x457590(g2_query_exclusive),
0x217790(scalar_groups),0x22b010(group_affineG2); recalculate file offsets from
ELF and retain disassembly before any new capture. It must have its own fixed
protocol and proof verification; no unmeasured speed claim or unchanged timing
reroll follows from these addresses.

History review prevents duplicate fixes: B/G2-first ordering already failed
2026-10-22-universal-primary-b-first. Empty-victim precheck, epoch-guard batching
and batch transfer also remain closed. Exact grouping prefix cutoff already
had a full failed SDK screen2026-10-27-grouping-prefix-cutoff; bitmap lower-bound
and parallel scalar grouping have separate negative records. A late G2 boundary
does not revive any of those patches or demonstrate the hash pass caused it.

For16x2, solver5.287848/FFT10.221869/post-product23.352205ms are material. For
merge54, solver20.461019/FFT65.119807/post-product72.856730ms show a different
budget. An all-shape solution therefore cannot presume every case is dominated
by the same microkernel. In the primary post-product sampled period, G1stream
is23.57%, borrowedG2apply19.63%, Fqinverse7.29%, G2reduce7.07%, G1pendingfeed6.74%,
G2feed6.48%, epoch6.09%, andG1reduce4.79%. These are exclusive top-symbol weights
inside a wall phase, not additive removable wall-time fractions.

ON/OFF paired median ratios are1.051619(primary32),1.058905(primary16),
1.025218(16x2),1.016425(54). The last physical16ON process is much slower than
the other two and remains included. Report all block ratios and switches/RSS;
no external-preemption, uprobe-only or cache cause is established. No exclusion,
extension, fastest-copy selection, repeated successful capture or profile-based
substitution for the accepted8.366931ms benchmark. OFF/ON both use root and the
same SDK inode; that diagnostic context differs from the accepted full matrix.

Four parser tests pass after the format repair; an earlier three-test pass is
retained. All13recordings have positive cycle samples, one sampled PID, zero
lost records and validated14traceperiod1+cyclesfreq199+metadata-dummy event
attributes. All25exports independently verify, match public hashes/164-byte
lengths and are fresh beyond15028identities. Ledger15053.820complete SDK calls
include primes. Final174source/SDK/helper/input pins pass; all14owned probes
removed. Sessions16329workflow and finish are terminal/reaped exit0. No owned
build/helper/hash/archive/second benchmark overlaps captures. No production,
dependency, cloud, phone/ARM/TEE, giant, swap, minimum-RAM or pressure change.

Setup repairs are retained separately from result interpretation: tee-a tracefs
seek failure, ineffective atomic-pipe workaround, direct restricted writer;
unsupported tracepoint-period term on perf6.8 (defaultperiod1 verified instead);
combinedPID/TID and fixed perf field ordering; expected metadata dummy event;
one compile-only PathBuf borrow repair. The one successful preflight/Gnark/three
decodes were reused after parser repair. No benchmark was run during those
builds. All earlier failed setup commands had zero completed SDK requests.
