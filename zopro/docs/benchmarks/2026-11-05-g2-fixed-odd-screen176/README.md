# Fixed-window odd-digit recoding work screen

No native trial is selected. The main bucket work saves **460 modeled additions**, but **28,166 skew corrections** change the total from **634,405 to 662,111**, an increase of **4.37%**. These are generic-addition counts for explicit dense recipes, not measured latency or a lower bound on every recoding method. Production remains unchanged.

## Representation and correction

The independently written width-12 recipe represents each magnitude as eleven radix-4096 signed odd digits, minus a skew of zero or one. Every nonzero even magnitude receives skew one. One ascending pass folds a ±1 digit into an opposite-signed predecessor where the range remains valid, including the final position. Negative components negate both digits and correction. The recipe uses 2,048 positive odd-magnitude buckets per window, the same count as the balanced signed control.

The [audited libsecp256k1 source](https://github.com/bitcoin-core/secp256k1/blob/cc0e03ffd4ddfce3663dc807a10cc4e3d0132d79/src/ecmult_impl.h) describes fixed odd digits and skew correction. Its source hash and exact commit are retained; no external code is copied or benchmarked. Our final-window collapse and separate correction sum are explicit differences. This is distinct from the rejected [bitwise NAF screen](../2026-11-05-g2-wnaf-screen176/README.md), which needed up to 127 bit positions.

Corrections are modeled as a separate signed point-sum tree. With s leaves, s−1 tree joins and one final combination charge s generic additions. The model does not insert all corrections into bucket zero and assume the concentrated arrivals are free of queue collisions. A native implementation of the separate tree would still need curve, normalization, workspace and scheduling qualification. No correction sum is cached between proofs.

## Exact counts and selection

The retained primary B fixture has 30,089 points and 60,178 signed scalar components. The original two-part geometry and exact qualified scalar decomposition are retained. The full eleven-window dense balanced-radix model is reused after identity checks; it is not rescanned or replaced by an ordinary-only comparator. The accepted optimized indexed-top execution remains outside this dense counterfactual model.

| Count | Frozen balanced radix | Fixed odd digits |
|---|---:|---:|
| Main nonzero arrivals | 613891 | 613459 |
| Bucket first writes | 41072 | 41015 |
| Part merge slots | 20536 | 20507 |
| Dense fold additions | 41050 | 40994 |
| Main generic-addition model | 634405 | 633945 |
| Skew corrections | 0 | 28166 |
| Complete generic-addition model | 634405 | 662111 |
| Additional odd-weight doublings | 0 | 11 |

Only 116 adjacent unit collapses occur. Corrections split by part/component are 6,816 / 6,814 and 7,282 / 7,254. Fewer main arrivals do not compensate for their correction sums. The declared selection requires fewer complete modeled additions than the frozen full-radix recipe; it fails without a width sweep, favorable subset, changed threshold or reroll.

The main model is arrivals minus first writes, plus shared-part bucket merges, plus 2×(last used slot−1) for an eager dense weighted fold. An odd-weight shifted fold reuses its running total and adds one doubling when more than one slot is used. Doublings are reported separately, without an assumed field-cost conversion. The independent reviewer checks this exact integer fold on 4,096 cases. No full curve work or speed is inferred from it.

## Validation and limits

All actual components pass exact weighted-term and signed Horner reconstruction after skew correction, with oddness/range checks. Another 8,569 exhaustive/boundary and 20,000 random recoding cases pass. The pinned fixture retains its previous independent Arkworks complete-query binding; that oracle is not rerun. All 176 production source hashes remain exact. Scalar-decomposition mathematics and the original full-radix count qualification are inherited, not newly counted.

The two-step Rust/xtask workflow compiles and counts once. Offline review verifies input/source pins, both command/exit/stream hashes, every candidate row and correction total, the frozen comparator identity and the additional fold identities. It does not perform a second scalar scan. Sources and exact receipts retain the first successful run. No helper, test or capture fails or repeats.

The model leaves curve cancellations, equal-x cases, projective queues, SIMD tails, inversion/normalization, packing, allocation, memory pressure and scheduling unpriced. Counter arrays are temporary research storage, not runtime lookup tables. This result closes the fixed recipe at the work screen; it neither excludes all fixed recodings nor establishes the accepted indexed-top cost. Raw scalars, per-point digits and input points remain private. No executable, frozen library or external source text is published.

Zero new SDK requests, Gnark exports or timing measurements; canonical proof ledger remains **30,708**. No AWS/Aeglos, portable/mobile/TEE, 250/4999, pressure/swap, minimum-RAM or concurrent-proof result is added. The all-eight CPU10× goal stays active and unmet, **0/8**.

## Accepted CPU and Metal comparisons, inherited

Accepted 32-worker local timings and supplied historical M5 references, not measurements of this recipe. [Accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json).

| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 | 24 | 4.667× | 3.027× | 3.7 |
| 2x2 | 11.151932 | 59 | 30 | 5.291× | 2.690× | 5.9 |
| 4x2 | 16.086386 | 109 | 48 | 6.776× | 2.984× | 10.9 |
| 8x2 | 24.179453 | 189 | 74 | 7.817× | 3.060× | 18.9 |
| 12x2 | 33.884455 | 284 | 101 | 8.381× | 2.981× | 28.4 |
| 16x2 | 40.077381 | 335 | 124 | 8.359× | 3.094× | 33.5 |
| 24x2 | 65.850777 | 528 | 176 | 8.018× | 2.673× | 52.8 |
| merge-24x1 | 65.060937 | 507 | 175 | 7.793× | 2.690× | 50.7 |

Merge54 remains 161.883899 ms without a supplied M5 reference.

[Design](artifacts/design.md), [counts](artifacts/analysis.json), [review](artifacts/review.json), [recoder source](artifacts/screen.rs), [prior-art provenance](artifacts/prior-art.json), [exact receipts](artifacts/receipts.json), [checksums](SHA256SUMS). Reproduction requires screen.rs, split.rs, the exact pinned fixture and the command/tool/library identities in plan.json. All receipt members round trip exact UTF-8 bytes, lengths and SHA-256 hashes.
