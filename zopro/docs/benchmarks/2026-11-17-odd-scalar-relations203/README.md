# Small odd-multiple scalar relations

**The real inputs contain a small amount of additional scalar reuse; no runtime
implementation or speedup is established.** On the retained 1×2 and 2×2 B2
queries, relations `s=±3t,±5t,±7t` permit at most two and seven additional class
merges beyond free exact/sign grouping. The fixed full-map detector is not
selected as the next runtime build. This is a prioritization decision, not a
performance rejection or a minimum-gain rule.

The hypothesis is to combine cheaply related point contributions before scalar
window processing. It differs from small-factor digit recoding, point
endomorphism aliases and repeated digit patterns across windows. The bounded
history review found those existing families and leaves their results unchanged;
this report makes no broad novelty claim.

| Retained query | Input rows | Zero scalars | Distinct nonzero scalars | Sign components | Components with odd relations | Extra possible merges |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 30,089 | 1,600 | 26,142 | 26,139 | 26,137 | 2 |
| 2×2 | 52,274 | 2,763 | 28,852 | 28,845 | 28,838 | 7 |

These are connected-component bounds, **not production-admitted group counts**.
Exact and sign grouping are assumed free for this comparison; the current
grouping admission policy can decline a real input. Connected edges can compose
into large multipliers, so a component does not itself provide a cheap executable
point transformation. No bound on full-proof time follows from these counts.

Seven/20 distinct undirected labeled edges are found. Primary has only ±3 odd
relations; grouped 2×2 also has +5 and ±7. The corresponding odd-affected classes
number four/11 and cover 574/1,126 original rows. A subsequent input check confirms
that both fixtures contain no identity bases, so point filtering does not remove
these opportunities. It preserves the original scalar-only result unchanged.

The affected primary classes have centered bit lengths 1,1,2,253: one full-width
class occurs once, while the others have 568,2,3 original occurrences. Grouped
2×2 has eight affected classes of at most nine centered bits and three full-width
classes, with 24,2,12 occurrences. The opportunity is therefore neither zero nor
entirely tiny-scalar reuse. After grouping, occurrences and classes are different
quantities; do not interpret 574/1,126 rows as additional eliminated MSM classes.

The fixed detector makes six additional hash lookups per distinct nonzero class,
156,852/173,112 lookups in these views, plus field multiplication, sign and
construction work. This is a source-derived count for this detector, not a
lower bound for every possible algorithm or a timed cost. Point transformation,
memory admission, grouping interaction and complete-query latency are unmeasured.
The analysis maps/edges are temporary diagnostic data; no retained table, key
copy, solver plan, pool or production allocation is introduced.

## Checks and scope

Two focused Rust tests pass: a known signed/odd relation graph and the scalar
additive-group identity for combining contributions. They are not elliptic-curve
or full-proof qualification. The census uses the existing exact-production field
library. A separate same-agent reader rebuilds every edge using canonical
four-word repeated modular addition and checks component sizes by breadth-first
traversal, independently of the census's Montgomery arithmetic and disjoint-set
structure. All counts, edge digests and 179 current production source hashes
match. This is not an external audit.

The workflow records 11 commands: nine successes and two failures. The first
compile stops on a missing closing brace. After that repair, both tests and the
census compile pass; execution stops before the fixture census because the
manually transcribed ledger digest has an extra `e2`. Its correction changes only
that expected string. Only the affected compile/execution is repeated; successful
tests are inherited. Both original source versions, pins and failures are
retained. Review and the later point-input check succeed first execution. All
22 command stdout/stderr hashes check against their receipts.

The original protocol is frozen at 16:03:48 UTC; the first result is present by
16:08:11, within its 15-minute allowance. Earlier source/history research is
outside that interval. Summed command elapsed time is about 2.03 seconds,
including compilation and failures, not total research or preparation time.
No latency capture, profile, new SDK request, Gnark export or cloud resource is
created. The proof inventory stays **58,257**, digest
`7f4ec2097957bcc2f063d32778300edba00d8adc238787e44c0d3e9e2e2eab9e`.

The positive opportunity remains recorded. A runtime selection needs a concrete,
bounded transformation and evidence that its detection/preparation cost is
worth testing; the current count alone does not justify another full campaign.
No ratio-set/threshold sweep, favorable-shape promotion or unchanged baseline run
is queued. This result does not explain or resolve the C7a/Aeglos gap.

Production SDK194 and its [full C7a comparisons](../2026-11-17-accepted-c7a201/README.md)
remain unchanged: primary is 11.098/12.515 ms at 32/96 cores, compared with pinned
Aeglos 12.143/8.311 ms. Latest accepted local primary is 7.628251 ms at 32 workers,
with its gain inconclusive at that count. All-eight M5 CPU/Metal comparisons remain
in reports196/201; the verified CPU10× objective is active and unmet, **0/8**.
No new device, huge-circuit, pressure/swap, minimum-RAM or throughput claim follows.

[Protocol](artifacts/protocol.md), [census](artifacts/result.json),
[independent arithmetic/graph checks](artifacts/review.json),
[point-input check](artifacts/live.json), [evidence](evidence.tar.gz),
[checksums](SHA256SUMS). The archive includes own helpers, source pins, plans and
all command streams/receipts. It excludes raw points/scalars, keys, witnesses,
proof/public bytes, full ledgers, executables and libraries. No push is performed.

Private root: `/home/ubuntu/src/zopro-perf-work/odd-scalar-relations203`.
