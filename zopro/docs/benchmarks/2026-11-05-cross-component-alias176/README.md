# Cross-component G2 alias screen

**No additional cross-component reuse was found** on either retained query after signed full-scalar contraction. The necessary primary novelty gate fails. No native implementation, performance capture or adoption follows; production remains unchanged.

The [previous alias screen](../2026-11-05-component-alias-count176/README.md) creates a separate magnitude map for each Phi2 component and excludes cross-component grouping. This screen checks the remaining case: the same magnitude occurring in both the P and φ(P) streams. The comparator first receives maximal full-scalar grouping modulo sign within each of the two original point-range parts, plus any eligible within-component grouping. Existing reuse is therefore not counted as new. This is an optimistic structural comparator, not a claim that production admits that exact grouping route.

| Query | Raw rows | Signed full-scalar classes | Mixed component classes | Additional generic additions saved | Additional φ maps |
|---|---:|---:|---:|---:|---:|
| 1x2-B | 30089 | 26488 | 0 | 0 | 0 |
| 2x2-B | 52274 | 44086 | 0 | 0 | 0 |

The per-part full-scalar class counts are 13,156 / 13,332 on primary and 20,698 / 23,388 on 2×2. No cross-part or different-width opportunity is selected after observing the result. The raw 2×2 population is not its admitted grouped runtime population. These two retained inputs do not rule out component reuse on every other witness.

## Work charged

A shared magnitude occurring in both components needs a point sum combining signed P references with signed φ(P) references. After the comparator's within-component grouping, a mixed class with w nonzero width-12 Booth windows emits two bases. If w≥2, combining them adds one generic join and avoids one arrival in each window, giving w−1 additional generic additions saved. Single-window classes stay separate. This recipe preserves the original two point-range parts and eleven component windows.

Each selected mixed class also needs one φ map of its formed high-component point. The existing affine x map costs two Fq multiplications; these are reported separately, without treating them as free or assigning an unsupported conversion to generic additions. Original bucket-φ work is conservatively not credited as a saving. Here there are no mixed classes, so formation, avoided arrivals, additional φ work and net saving are all zero. A positive necessary count would still require complete curve-operation, metadata, normalization, scratch and admission analysis before any native trial.

## Correctness and review

Three new model tests pass on their first execution. Constructed mixed-component collisions include existing intra-component duplicates and verify that only new cross-component savings are counted. Other cases cover signs, zeros, one-window exclusion, cancellation, partial populations and changing multiwindow classes. Every signed bucket before and after grouping matches in two small-prime additive modules. Points have two coordinates and φ is represented as J(a,b)=(b,−a−b), a linear map satisfying J²+J+1=0. Thus P and φ(P) remain distinct in the model. These are algebraic model checks, not elliptic-curve or proof oracles.

All 70,574 normalized full-scalar classes reconstruct with the accepted Fr/lambda decomposition identity. The split code is shared with qualified production and is not an independent decomposition implementation. The original field library's exact SHA-256 is verified. All 176 production source hashes remain exact; existing field/core/prover qualification and complete Arkworks fixture bindings are inherited, not rerun.

A separately written reviewer uses manual modulus subtraction, sorted scalar/component records, per-bit Booth extraction and independent row sums. It imports no study code and reproduces every histogram, count, presence set and necessary-gate decision. It verifies 194 input/source/tool/library pins and all four successful Rust/xtask command receipts and stream hashes. The four-step workflow compiles the model tests, runs all three unfiltered, compiles the study and counts once. No command or test fails or repeats.

Temporary diagnostic maps are not a proposed retained lookup table or a native memory bound. A future implementation would need to charge metadata, formed points, normalization and reduction scratch in addition to key/prover workspace, and preserve admission fallback and borrowed lifetimes. Input scalars, per-point memberships, point arrays, keys, images, witnesses and executable/library blobs remain private.

Zero new SDK requests, Gnark proofs, latency or CPU profiles. The canonical proof ledger remains **30,708**. No AWS/Aeglos, phone/ARM/TEE, 250/4999, pressure/swap, minimum-RAM or concurrent-throughput result is added. This closes the fixed cross-component lead on these inputs. The all-eight CPU10× goal remains active and unmet, **0/8**.

## Accepted CPU and Metal comparisons, inherited

Accepted 32-worker local proof timings and supplied historical M5 references, not measurements from this screen. [Accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json).

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

Merge54 remains 161.883899 ms without a supplied M5 reference. The latest separate [reverse-loop caller trial](../2026-11-05-g2-reverse-leaf176/README.md) remains rejected; no result here changes accepted proof performance.

[Design](artifacts/design.md), [counts](artifacts/analysis.json), [independent review](artifacts/review.json), [model](artifacts/study.rs), [review source](artifacts/review.rs), [workflow](artifacts/plan.json), [exact receipts](artifacts/receipts.json), [checksums](SHA256SUMS). Private root: `/home/ubuntu/src/zopro-perf-work/cross-component-alias176`. Reproduction uses the pinned two input fixtures, original field library, scalar helper and explicit tool/compiler argv in the plan. Receipt members round trip exact UTF-8 bytes, lengths and SHA-256 hashes.
