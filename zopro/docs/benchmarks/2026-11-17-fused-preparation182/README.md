# Fused canonical conversion and query extraction

The local full-proof screen passes its advancement criteria. The candidate
converts each wire block and immediately gathers its A/B scalars, removing the
global conversion-to-gather barrier and separate gather iterator trees. It adds
no lookup table, persistent metadata or output buffer beyond the existing two
query vectors. Production is unchanged; profiling and release qualification
remain required.

| Local Ryzen 9950X case | Accepted REF, ms | Candidate OFF, ms | Candidate ON, ms | REF / ON peak RSS, MiB |
|---|---:|---:|---:|---:|
| 1x2, 16 workers | 8.948060 | 8.847901 | 8.815680 | 80.00 / 78.25 |
| 1x2, 32 workers | 7.721298 | 7.668980 | 7.566416 | 111.75 / 101.00 |
| 2x2, 32 workers | 10.751928 | 10.686846 | 10.561951 | 270.75 / 263.25 |

Times are medians of nine process medians. RSS is the largest whole-process
peak, including loading and primes; it is not minimum required RAM. This is a
local screen, not a C7a result or a new Aeglos comparison.

The paired ON/OFF median ratios are 0.991961, 0.985234 and 0.989692 respectively,
with 7/9, 9/9 and 8/9 winning blocks. Their descriptive rank-1-through-7 bounds
(zero-based sorted ranks) are [0.977751, 1.000392], [0.980674, 0.991862] and
[0.972767, 0.996148]. The 16-worker bounds include parity. These finite-sample
bounds assume independent blocks and are not simultaneous confidence bounds.
ON/OFF mean-CPU paired median ratios are 0.993201, 0.979920 and 0.981119.
All declared latency, CPU and RSS screen guards pass.

Three independent executable backings were balanced across nine rotated/reversed
blocks. The accepted reference's first backing was slower, particularly at 16
workers. Consequently the full ON/REF difference cannot all be attributed to
the algorithm. OFF and ON use the same candidate binary and backing; each of
their three backing medians improves in each case. No block or backing was
discarded. This screen advances the candidate; it does not establish adoption.

Both solver loaders already validate strictly increasing, in-range query indices.
The candidate partitions sorted indices and output slices at each wire midpoint;
leaves of at most 1024 wires convert once and fill disjoint spare-capacity slices.
Vector lengths are published only after all joined work returns. The leaf size
was fixed before capture. The existing canonical-wire eligibility and portable
fallback remain in place.

All 131 native core tests pass, including two added tests comparing converted
wires and selected outputs with a serial oracle across leaf boundaries, concurrent
calls, 1/4/16/32 workers and 512 KiB worker stacks, plus panic handling. All 83
fresh exported proofs pass independent Gnark verification and image/public-input
bindings. The panel made 2762 SDK calls, including primes and preflights.
Portable, all-shape and remaining resource qualification are still pending.

The exact candidate SDK SHA-256 is
`c285cbc88f783abac203c66db43ea86bda8207338b9fb209df7eb5d846126202`;
accepted REF is `b646c7b8c1d8a26a0db45fb3f67c453dd616bd9f02e25c36192721ae02855abe`.
Runtime selector: `ZOPRO_BENCH_FUSED_PREPARATION=0/1`.
Private experiment root: `/home/ubuntu/src/zopro-perf-work/fused-preparation182`.
The canonical proof ledger is now 55861 entries, SHA-256
`7c6fa1eeaace1afcf4736845278c8de7d03dce5a61f9794d9beaa30ddf4b7f3e`.

The archive preserves the frozen protocol, source overlay, build receipts,
all screen receipts, verification records and analysis. Binaries, proving keys
and large historical ledgers are retained privately. No AWS resources were
created and nothing was pushed. The all-eight-shape CPU 10x goal remains unmet;
retain the [accepted all-shape comparison](../2026-11-12-bundle-adoption178/README.md)
until this candidate completes qualification.
