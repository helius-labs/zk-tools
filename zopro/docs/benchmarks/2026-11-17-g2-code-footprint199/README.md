# G2 instruction footprint and host cache capacity

**The complete function's size does not establish a C7a-specific cache-capacity
bottleneck.** Both retained C7a inventory and current Ryzen sysfs give 32 KiB of
L1 instruction cache per physical core. In the retained native query consumer,
the 51,862-byte G2 symbol has two arithmetic-loop components totaling 27,796 bytes.
Those components contain 2,047 of 2,049 G2 samples in one existing profile.

This closes a proposed capacity-only explanation before another outlining or
loop-bound variant. It does not rule out frontend costs: fetch/decode throughput,
branch behavior, callees, replacement and execution resources can differ.
[AMD's Zen 5 presentation](https://hc2024.hotchips.org/assets/program/conference/day2/24_HC2024.AMD.Cohen.Subramony.final.pdf)
documents 32 KB L1I and changes to fetch/decode and vector execution; equal cache
capacity does not imply equal processor performance. No new performance result
or optimization is selected here.

## Exact offline observations

Reuse the unchanged original G2 kernel in the
[198 consumer](../2026-11-17-g2-central-equality198/README.md), one existing OFF
cycle recording and audited CFG/SCC helpers. No prover rebuild, benchmark rerun,
proof export or AWS instance is needed. The consumer is experimental, not the
accepted SDK binary or a new C7a capture.

| Static object | Bytes | 64-byte lines |
|---|---:|---:|
| Complete G2 symbol | 51,862 | 811 |
| Two IFMA-containing loop components | 27,796 | 436 |
| Those loops plus entire inverse and diagnostic feeder | — | 583 (37,312 bytes) |

The two loop components contain all 1,450 static IFMA sites. Whole-symbol size also
includes large scalar-tail paths and other code outside those loops. One retained
profile supplies 4,458 samples in total; every selected-symbol offset maps to an
exact decoded instruction start. G2's 2,047/2,049 loop concentration is about 99.90%
of its sampled residence, not a wall-time fraction or a cache-miss rate.

The 583-line set includes the entire 6,979-byte inverse and 2,296-byte diagnostic
feeder. The feeder contains 198's OFF/ON selector code; it is not an exact accepted
SDK feeder. This static union exceeds 512 lines and has at most 10 lines in one of
the local 64 L1I sets. It is neither an observed resident working set nor an upper
or lower bound on actual cache demand: it includes unvisited helper branches,
omits transitive callees and ignores temporal order. It cannot establish misses,
their latency, op-cache occupancy or the C7a/Ryzen performance reversal.

## Verification and decision

Four xtask decode steps complete once. A separate same-agent reader checks 10,203
instruction rows, byte/line conservation and 1,686 SCC groups using bidirectional
reachability plus an acyclic condensation graph. It also checks sample counts,
input hashes, all 179 unchanged production sources and the unchanged 58,033-proof
ledger. No fresh arithmetic tests or cryptographic verification are claimed.

The initial symbol selector retained crate disambiguators; alternate demangling
repairs selection before disassembly. An initial analyzer compile needs an
explicit usize line-set type. Both sources are retained; successful decoding and
analysis are not replayed. The original parser and SCC algorithm are reused;
this is not an independent instruction decoder or external audit.

Do not justify a compact-code rewrite by comparing the whole 51.9 KB function to
one host's L1I size. No compact-code candidate follows from this audit alone.
The earlier outlining failures remain closed. Future work still needs a distinct
reduction in complete-caller arithmetic or dependency cost, with actual consumer
measurements. There are zero new queries, SDK proofs or cloud resources.

Accepted local 194 primary remains 7.628251 ms at 32 workers. The complete supplied
M5 CPU/Metal comparisons remain in the
[196 qualification](../2026-11-17-certified-all196/README.md); the all-eight CPU 10×
goal remains active and unmet, 0/8. Merge54 stays 163.384864 ms without an M5 reference.
No memory saving, phone/ARM, pressure, throughput or new Aeglos comparison follows.
Nothing is pushed.

[Protocol](artifacts/protocol.md), [analysis](artifacts/analysis.json.gz),
[review](artifacts/review.json), [local cache](artifacts/local-l1i.json),
[sources and exact decode receipts](evidence.tar.gz), [checksums](SHA256SUMS).
Private root: `/home/ubuntu/src/zopro-perf-work/g2-code-footprint199`.
