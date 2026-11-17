# Fixed chunk pair-cache count screen

The complete chunk pair-cache recipe has a poor memory/work tradeoff on the
retained large queries. For primary G2, **161.61 MiB of point payload removes
128 generic fill additions, or 0.0211%**. A theoretical **20.20 GiB** removes
7.0397%. No tables were allocated and no native implementation is selected.
Accepted227 and its small qualified gain remain unchanged.

The investigation follows Grigaitis's
[fixed-base MSM preprint](https://www.preprints.org/manuscript/202604.0045),
sections3.1/4.1: cache pair sums within chunks of a shifted-point array. Its
experiments target small MSMs with2–64 inputs. Here the query populations have
21,006–52,274 points; the paper's speedups are not transferred to this workload.
No external implementation is copied.

## Fixed construction and counts

This local signed extension uses width12,22 windows and point-major shifted
order from the earlier prepared-query family. Sum-only caches one point per
unordered pair and matches equal signed nonzero digits. Sum-and-difference
caches two points and matches equal nonzero magnitudes. Each entry can be used
once within its chunk/bucket. A population of k matching entries removes
floor(k/2) additions. The global bucket occupancy and weighting stay unchanged
in this generic formal-point model.

All fixed nonidentity entries contribute to preparation size, including entries
whose captured witness digit is zero. Pair sizing uses every unordered pair in
each chunk; no witness-dependent pruning, deduplication, identity cancellation
or cross-chunk pairing is assumed. Known key identities can be omitted; none
occur in these four fixtures. Calculated bytes include the original shifted
array and cached point payload only. They exclude setup intermediates, headers,
indexes, routing, reducer scratch and allocator overhead.

|Retained raw query|Points|Shifted entries|Nonzero contributions|Prepared fill additions|
|---|---:|---:|---:|---:|
|1×2 A|21,006|462,132|407,098|405,050|
|1×2 B2|30,089|661,958|609,901|607,853|
|2×2 A|36,637|806,014|706,536|704,488|
|2×2 B2|52,274|1,150,028|1,058,011|1,055,963|

These are **not current227 operation counts**. Its G1 balanced windows and G2
Phi2 decomposition differ. Both2×2 queries admit the original scalar-grouping
policy; raw2×2 rows are hypothetical and do not justify bypassing that benefit.
Existing [prepared local](../2026-11-17-shifted-identity188/README.md) and
[C7a](../2026-11-17-shifted-c7a197/README.md) caller regressions remain closed.

Selected primary G2 rows follow; all64 rows across four queries, eight chunk
sizes and both cache modes remain in [counts](artifacts/counts.json).

|Chunk entries|Cached values per pair|Total point MiB|Fill additions removed|Fill removed|
|---:|---|---:|---:|---:|
|2|Sum|121.21|61|0.0100%|
|2|Sum/difference|161.61|128|0.0211%|
|8|Sum/difference|646.44|1,043|0.1716%|
|16|Sum|686.84|1,111|0.1828%|
|32|Sum|1,333.28|4,586|0.7545%|
|128|Sum/difference|10,342.60|25,399|4.1785%|
|256|Sum/difference|20,684.79|42,791|7.0397%|

For primary A, the two-entry sum/difference cache needs56.41 MiB and removes
93 additions,0.0230%. Eight-entry chunks need225.65 MiB for693,0.1711%.
The largest modeled A cache needs7,220.17 MiB for7.0157%. Multi-GiB rows are
theoretical accounting, not architecture proposals. No fixed RAM admission cap
or minimum gain floor is introduced.

Counts describe generic additions saved in prepared fill, not total query or
proof CPU savings. Lookup/sign handling, preparation and memory accesses are
uncharged. Actual equal/opposite-point behavior could differ. These fixed witness
fixtures do not establish a hit rate for arbitrary future proofs or a lower
bound on every cache design. Positive counts alone do not establish a useful
complete caller.

## Validation and provenance

All four retained fixture hashes/provenance bindings pass before execution.
No new export, witness solve, proof, SDK request or Gnark execution occurs.
The original two model tests pass: exact signed digit reconstruction over
full-width/boundary integers, and exhaustive small signed populations with
identities, chunk tails, matching and payload checks. Every real row additionally
checks explicit disjoint pairing and complete weighted group outputs modulo
65,537 and1,000,000,007.

A separately implemented reader uses bit-by-bit extraction, preceding-high-bit
carry recoding and sorted runs. It agrees on all64 addition/payload rows, checks
206 pins and the five preceding command receipts, and verifies all186 unchanged
production source hashes and the unchanged canonical inventory. It does not
import the primary count/matching helpers. This is same-agent review, not an
external audit or native BN254 qualification.

The original numerical test values did not independently establish every formal
coefficient promised by the protocol. A separately selected **third new test**
adds positional basis values256^i with exact range/uniqueness bounds, covering
every signed population through four inputs and all identity masks. It passes;
the original two tests are filtered, not replayed. Original protocol, plans,
pins and six-step results remain intact. See the explicit
[validation note](artifacts/additional-validation.md). The final publication
checker verifies all **eight workflow receipts and16 stream hashes**, including
this later compile/test pair. No successful scientific execution is repeated.

Preparation and recording spans are retained in [timing](artifacts/timing.json).
Both declared allowances pass. Earlier paper/history research precedes protocol
creation; filesystem spans are descriptive, not instrumented monotonic timers.
The canonical inventory remains **61,143**, SHA-256
`ab290bf3ef3d535745621f21aa914423ff5d578abfb30f44ed80f2f6998104f8`.

Close this exact complete pair-cache construction without a native table build,
geometry sweep or unchanged reroll. Sparse/deduplicated caches and other MSM
algorithms remain distinct questions; this result does not refute the paper on
its target small instances. There is no new AWS, phone, huge-circuit, pressure,
minimum-RAM, throughput, PMU or full-proof latency claim. No cloud resource is
created and no production source changes.

## Accepted full-proof comparison, inherited

Exact accepted227 measurements from
[228](../2026-11-17-scaled-prefix-all228/README.md),32 workers; supplied M5
references use different hardware. These are unchanged full-proof observations,
not timings of this count model. All-eight CPU10× remains active and unmet,0/8.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.634234|37|24|3.7|
|2×2|10.695071|59|30|5.9|
|4×2|15.853243|109|48|10.9|
|8×2|24.089668|189|74|18.9|
|12×2|33.894323|284|101|28.4|
|16×2|39.850686|335|124|33.5|
|24×2|66.290215|528|176|52.8|
|Merge24×1|66.016778|507|175|50.7|

Accepted merge54 is164.152637ms, without a supplied M5 reference. Exact CPU and
Metal speedup ratios are retained in [comparison](artifacts/accepted-comparison.json).
Accepted SDK SHA-256 is
`b70a9472fa64ac6b6b41b227ca64ddfcf61c6443879c9187331eae6d5e22c153`.
The exact accepted SDK has no new AWS measurement here.

[Protocol](artifacts/protocol.md), [independent review](artifacts/review.json),
[decision](artifacts/decision.json), [receipts](artifacts/receipts.json),
[sources/plans](artifacts/source-and-plans.tar.gz), [checksums](SHA256SUMS).
Private input/point/scalar payloads, keys, binaries and complete proof inventories
are excluded. Local root:`/home/ubuntu/src/zopro-perf-work/chunk-pair-count231`.
Evidence is committed locally; publication awaits approval.
