# Two-bank complete K/H query screen

Two temporary G1 accumulators per bucket improve the complete primary K/H query
in all **27 matched pairs**. The 32-worker paired median improvement is **3.31%**,
with peak process RSS increasing **32.0 → 49.25 MiB**. This selects a short
full-proof screen; it does not establish an SDK speedup or authorize adoption.

|Workers|OFF ms|ON ms|Paired improvement|OFF / ON peak RSS MiB|
|---:|---:|---:|---:|---:|
|1|34.587178|34.118467|1.28%|17.50 / 17.75|
|16|3.484379|3.408205|2.38%|28.25 / 40.75|
|32|2.468711|2.397340|3.31%|32.00 / 49.25|

Times are medians of nine arithmetic process means per arm/context, each from
17 calls after three primes. Paired improvements use median ON/OFF ratios and
therefore differ from ratios of displayed medians. Each context has nine wins;
all three independently written executable backings have favorable median ratios.
No confidence interval or release-qualification claim is made from this screen.
Whole-loop CPU ms/query falls 34.590→34.119, 38.787→37.762 and 54.867→53.250 at
1/16/32 workers. Major faults are zero; other resource counters remain in raw rows.

The [qualified two-bank Window](../2026-11-11-two-bank177/README.md) is unchanged.
A new feeder uses the original twenty balanced spans and digit extraction without
diagnostic counters. OFF calls the existing balanced fused implementation. ON
selects the first available bank, preserving the original arithmetic kernel.
The measured boundary includes allocation, digit reading, queue/spill handling,
normalization, final bank merge, weighted reduction, Horner fold, destruction,
pool entry, result normalization and equality checking. Extra final-merge scratch
is included. Key loading, fixture decoding, pool construction and input hashes
are outside timing. RSS includes fixture loading/primes, and is neither SDK RSS
nor minimum RAM. No key expansion, persistent table or production pool is added.

Inputs are 30,529 actual K points/scalars and the 30,922-point H prefix used by
this proof, recovered from the pinned Euler image and retained scalar cache.
All 61,451 canonical scalars, identity markers and point curve/subgroup checks
pass. An independent Arkworks MSM computes the full combined expected result;
all **1,104** priming/timed caller outputs match, with input digests unchanged.
Six one-call preflights precede 54 processes/918 main timed calls. Workers are
pinned to CPU2 for solo, CPU0–15 for16, CPU0–31 for32 on the local Ryzen9950X;
32 workers use SMT. Worker stacks are512KiB. One coherent native executable
contains both arms, across three byte-identical, inode-distinct files.

The reader checks204 source/input pins, backing identities, frozen command
arguments, receipt stream hashes, ordering, sample counts and worker placement.
Capture command time totals **16.262 seconds**. Fixture preparation completes
about203seconds after protocol creation; the initial native build takes44.949s,
and the corrected driver relink7.302s. This excludes earlier source inspection.
Initial fixture extraction incorrectly required the used H length to equal the
whole key domain; the repair selects production's prefix. The failed extraction
and an initial freezer type-inference error are retained. No timing protocol or
sample count changed after capture.

The next experiment is a short verified SDK comparison against the exact accepted
binary, with a fresh disabled control and enabled candidate. The memory increase
must be priced into admission/fallback before any production selection. There is
no fresh AWS/Aeglos/mobile/pressure/large-circuit result. Accepted primary full-proof
latency remains **7.821726ms**, versus the supplied M5 CPU/Metal37/24ms and the
3.7ms CPU goal. The inherited [all-eight comparison](../2026-11-11-cost-map177/README.md#accepted-full-proof-reference)
is unchanged; the goal remains active and unmet0/8. This caller screen creates no
proofs; the canonical ledger remains53,917. Nothing is pushed or published.

[Protocol](artifacts/protocol.md), [results and all pairs](artifacts/analysis.json),
[driver](artifacts/driver.rs), [reader](artifacts/analyze.rs),
[sources, plans and receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
Private fixture/oracle bytes, keys, witnesses, binaries and unchanged full source
snapshots are excluded. Reproduction requires the pinned local inputs/libraries.
