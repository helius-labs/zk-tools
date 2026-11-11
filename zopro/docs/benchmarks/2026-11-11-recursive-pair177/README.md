# Recursive reuse of point sums: scalar opportunity screen

Recursive reuse increases the earlier single-generation saving from **2.08% / 2.11% to 4.23% / 4.40%** of nonzero digit arrivals after charging point formation, beyond exact scalar grouping. It does not expose a twofold ordinary-MSM work reduction in this construction. No native implementation or prover timing follows; production is unchanged.

The [previous screen](../2026-11-08-global-window-pair-bound176/README.md) paired each input at most once and explicitly left recursive reuse open. This extension retains residual digit rows and lets newly formed sums participate in later generations. Each generation visits all 231 signed Phi2 window pairs in fixed lexicographic order, pairs consecutive matching node indices, removes every compatible common nonzero column, and creates one sum/difference node. Each existing node participates at most once per generation. The process stops when a complete generation makes no pair. There is no parameter or ordering sweep.

## Results

Both retained fixtures reach a fixed point after six productive generations. A full terminal census checks that no two remaining rows share two nonzero columns with the same relative sign.

|Quantity|1×2 B2|2×2 B2|
|---|---:|---:|
|Original rows|30,089|52,274|
|Exact nonzero scalar classes|26,142|28,852|
|Arrivals after exact grouping|574,356|633,695|
|Final arrivals|525,924|578,162|
|Additional point formations|24,134|27,645|
|Net arrivals saved, formations charged|24,298|27,888|
|Net saving beyond exact grouping|4.2305%|4.4009%|
|Additional net saving after first generation|12,346|14,490|
|Formations involving an already generated node|842|1,019|
|Final nonempty rows|49,340|55,366|
|Maximum DAG depth|3|4|
|Hypothetical added affine-point payload, bytes|3,089,152|3,538,560|

Most later formations reuse residual original rows; only 3.5–3.7% involve generated nodes. Recursive opportunities do exist, but this recipe mostly buys another modest saving through repeated classification. The number of nonempty rows to route nearly doubles. Storing all generated G2 affine points would have the payload shown; liveness-based storage is unimplemented. Model node-vector capacity is 4,600,992 / 5,077,952 bytes, excluding sorting/maps, other arrays, field work and allocator overhead. Neither figure is native peak RSS or minimum RAM.

Including hypothetical full-scalar grouping gives 61,486 / 438,438 net saved arrivals, or 10.02% / 41.17% of raw arrivals. **The second figure is largely grouping already available in production**, not a new improvement. Primary production declines that grouping. This study does not replay its admission policy or prove that grouping plus recursive reuse pays for itself.

Arrivals are not complete point-add counts or cycles. First writes, partition/component merges, weighted reductions, exceptions/cancellation, inversion batches, SIMD occupancy, classification and routing costs remain unpriced. A fixed point of this greedy construction is not a bound on all recursive reuse, other orders or recodings. The small remaining opportunity and added discovery/routing work deprioritize this recipe for the requested jump; they do not establish a measured slowdown or a minimum gain rule for adoption. A practical implementation would first need a cheaper discovery schedule and complete-query evidence.

## Checks and cost

The model propagates each final DAG's signed coefficients backwards, column by column, and exactly reconstructs all **1,209,868 original scalar-class/window coefficients** with checked integer arithmetic. Every input also receives inherited Phi2/Booth integer and Fr reconstruction checks. The first-generation counts exactly reproduce the earlier study. These checks establish scalar linear conservation, not native point addition or proof correctness.

Two new tests pass: all **19,683** three-row/three-column matrices over −1/0/1 compare the map-based pairing selector against a separately expressed sorted-run selector and check complete coefficient recovery; a crafted four-row case requires signed recursive reuse. Transformation code is shared between selector variants. The retained input curves/subgroups and complete MSM outputs inherit their earlier independent Arkworks qualification; no curve test is replayed.

Four Rust/xtask commands pass on their first execution: test build **0.380 s**, tests **0.028 s**, model build **0.533 s**, and the two-fixture screen **6.711 s**. These exclude preparation and analysis and are not prover performance measurements. A separate same-agent reader checks aggregate conservation, generation counts, first-generation anchors, ratios, 20 source/input pins, exact command/stream receipts and the unchanged proof ledger. It does not replay the private DAG or independently verify curve arithmetic. No successful test or screen was repeated.

Accepted local primary proof remains **7.821726 ms / 127.5 MiB peak RSS**, against a **3.7 ms** CPU target. The [current all-eight CPU and Metal comparison](../2026-11-11-cost-map177/README.md#accepted-full-proof-reference) is unchanged; the verified CPU 10× goal remains **active and unmet, 0/8**. Canonical verified exports remain **53,917**, SHA-256 `a033132ee87cf5d5260c396acf1894bbb1afc95c07d8ba22c55ed9c8e5a85873`. No SDK/AWS/Aeglos timing, new proof, production edit, push or publication occurs.

[Protocol](artifacts/protocol.md), [model](artifacts/model.rs), [results](artifacts/result.json), [reader](artifacts/review.rs), [review](artifacts/review.json), [plans/pins/receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS). Private scalars, digit rows, DAGs, points, keys, witnesses, proofs and compiled binaries are excluded. Reproduction requires the pinned local dependencies and retained private fixtures.
