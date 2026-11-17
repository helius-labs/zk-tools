# Unshifted G2 bucket tree on C7a

**The tree improves 96-core query time but does not improve the fastest configuration.**
Primary 96-core G2 latency improves by a paired 23.90%, winning all nine blocks;
its best latency across 32/96 cores still loses to current streaming. Two of three
prospective progression checks fail. This version is not adopted.

This tests an existing locally slower algorithm on a different host. The exact
[178 staged tree](../2026-11-12-bucket-round178/README.md) is rebased onto current
179-file production sources. It routes signed indices into 256-bucket groups and
reduces borrowed leaves as balanced trees. This exposes more independent work
and uses temporary index/intermediate-point arrays. It retains two point
partitions, high-before-low Phi components, narrow top, merges and weighting.
There is no retained shifted-point table, duplicate key/solver plan or new pool.

## Complete-query results

One c7a.24xlarge with 96 physical cores and no SMT; fixed individual worker affinity,
512 KiB stacks, Rust 1.98.1/znver4, existing native IFMA arithmetic. Values are
medians of nine process means, each with 17 timed queries after three primes.
Paired ratios need not divide the displayed medians.

| Shape / workers | Streaming ms | Tree ms | Paired tree/stream | Wins /9 |
|---|---:|---:|---:|---:|
| 1×2 /32 | 4.870280 | 4.916313 | 1.012659 | 2 |
| 1×2 /96 | 6.649059 | 5.034480 | 0.760996 | 9 |
| 2×2 /32 | 6.534435 | 6.991978 | 1.070726 | 0 |
| 2×2 /96 | 8.413935 | 7.285387 | 0.864116 | 9 |

The best-primary paired ratio is 1.012659, with 2/9 wins; the grouped best-latency
ratio is 1.070315, above its 1.02 guard. Both fail. The 96-core primary improvement
passes. All three executable backings improve primary96 individually, while all
three lose primary32. There is no confidence-qualified small-gain or full-proof
claim, and no worker-count or parameter sweep follows.

| Shape / workers | Streaming/tree CPU ms/query | Streaming/tree peak RSS MiB |
|---|---:|---:|
| 1×2 /32 | 89.59 /102.13 | 20.53 /40.79 |
| 1×2 /96 | 259.09 /249.46 | 60.96 /90.30 |
| 2×2 /32 | 102.64 /121.49 | 28.73 /55.71 |
| 2×2 /96 | 300.48 /336.30 | 155.54 /189.10 |

CPU includes post-timing result hashing. RSS includes loading and warmup and is
not minimum RAM or full-prover memory. The 30 GiB/no-swap service ceiling supplies
infrastructure headroom; it is not an architectural cap. These results show a
high-core improvement without retained tables, while also showing why high-core
improvement alone does not meet the single-proof latency objective. No unique
cache, scheduler or arithmetic cause is established.

## Checks and decision

Two focused native tests pass (134 filtered, one ignored screen): raw complete
MSM outputs match retained independent Arkworks oracles, and synthetic generator
cases exercise identities, repeated/opposite points, scalar edges and 1/4/16/32
worker pools. Historical exact-child sign/padding and field qualification are
inherited. Actual grouping and dispatch are exercised by every remote query.
The harness asserts the selected route, output and point-input immutability.

Two local functional smoke processes precede launch. Eight remote preflights
precede 72 main processes, with three distinct byte-identical executable backings
and nine rotated/reversed blocks. All **1,552 remote queries**, including primes
and postchecks, match retained oracles. Timing includes word copying, grouping,
split, routing, intermediate allocation, fills, merges, weighting and normalization.
No build or analysis overlaps capture. Native build/tests take 59.41 seconds;
the remote measurement service completes in 15.17 seconds. Protocol preparation
to retrieved result takes less than ten minutes, within its 20-minute allowance.

The reader independently reconstructs process means and ratios from stdout,
checks 80 command receipts/160 stream hashes and the three backing inodes, and
matches the remote executable digest. Source recovery restores the original G2
parent exactly; 178 other production parents and the historical tree are byte
identical. All 179 production hashes and the 58,033-proof ledger remain unchanged.
This is a same-agent review, not an external cryptographic audit.

A preparation helper initially used a nonexistent historical runner filename;
using the actual repaired runner source fixes preparation before any capture.
All native tests, local smoke and remote workflows succeed once. No SDK proof,
Gnark export, Aeglos comparison or exact SDK194 AWS measurement is added.

The one owned instance and its volume are removed after retrieval; the temporary
transfer bucket and objects are deleted. No ECR image is created. Close this
fixed tree recipe on both hosts. Further work needs a different reduction in
complete-caller cost, rather than more tasks alone.

Production remains the adopted opt-in SDK194. Its local primary observation is
7.628251 ms; all supplied CPU/Metal comparisons and separate merge54 result stay
in [196 qualification](../2026-11-17-certified-all196/README.md). The CPU 10× goal
remains active and unmet, 0/8. This report and changes are local; nothing is pushed.

[Protocol](artifacts/protocol.md), [results](artifacts/analysis.json),
[review](artifacts/review.json), [source and receipts](evidence.tar.gz),
[checksums](SHA256SUMS).
Private root: `/home/ubuntu/src/zopro-perf-work/bucket-tree-c7a200`.
