# H-only three-point affine algebra screen

Cancelling the first slope from the earlier three-point formula removes denominator factors and field products. **The revised algebra passes exact checks and justifies a separate native G1 caller experiment. It is not a measured speedup, native implementation or production change.** G2's work tradeoff remains unfavorable against binary updates at larger batches.

The earlier [three-point model](../2026-10-28-triple-affine-screen/README.md), [small callers](../2026-10-28-triple-callers/README.md), [mixed SDK trial](../2026-10-28-mixed-sdk/README.md) and [complete unweighted reducer](../2026-10-28-unweighted-triple-caller174/README.md) remain closed. This changes their arithmetic: it inverts H instead of dH and eliminates recovery of the first slope. It does not retry their layouts, parameters or failed gates.

## Formula

For finite P=(a,b), Q=(c,e), R=(r,s):

```text
d = c-a                    n = e-b
S = a+c+r                  H = d² S-n²
U = d(b+s)+n(r-a)           q = U/H
w = d q                    v = n q
x = w²-2v+a+c-r
y = w(r-x-2S+v)+b+e+s
```

When H is nonzero, the result is P+Q+R. Infinity inputs or H=0 request fallback before inversion. A first pair P=−Q with nonzero H now returns R directly; equal first points have H=0 and request fallback.

The old recipe formed D=dH, A=nH and B=d²U, recovered lambda=A/D and w=B/D, then used both slopes to form the result. In the final X coordinate, the lambda² terms cancel. Substituting n²q=d²Sq−U cancels the remaining 1/d terms in Y. Thus D, A, B and lambda recovery are unnecessary. This is a local derivation without a novelty claim; see the [full derivation](artifacts/derivation.md).

The [Eisenträger–Lauter–Montgomery paper](https://arxiv.org/html/math/0208038v2), sections 3–3.1, supplies background for removing unused intermediate coordinates in 2P+Q. Its two-division scalar-multiplication result does not establish this general three-input formula or its prover performance. No published speed estimate is transferred.

## Exact qualification

Two focused tests pass on their first executions. The toy curve has 18 points including infinity; all **5,832 triples** are checked against complete addition. The formula computes 4,128 ordinary results, including 272 opposite-first-pair cases; 919 identity and 785 H-zero cases request fallback.

The BN254 study uses boundary scalars, a fixed full-width scalar point pool and changing chains for both G1/Fq and G2/Fq2. All **11,264 attempts** are retained: 10,592 ordinary results agree with independent complete Arkworks generator-scalar sums, including 84 opposite-first-pair results. The 672 fallback requests comprise 448 identity and 224 H-zero cases. Guards perform no inversion. A further **496 triples** in fixed batches of 8/16/32/64/128 agree across the new formula, original dH recipe and two binary stages. These are scalar curve checks, not proofs or native SIMD execution.

The original PointField/Ops/Cost model is reused byte-exact. Its three old tests are filtered and inherited. No prover, native field/core suite, SDK, Gnark or performance capture is rerun. The first formatting attempt occurred before copying the original module and stopped on that missing file; copying it and disabling recursive child formatting repaired preparation before any build/test execution. All four subsequent Rust/xtask commands succeed first time.

Independent review checks 232 pins, the unchanged model and all command receipts. A separate sparse-polynomial implementation with checked integer coefficients expands the old and new X expressions over a common d²H² denominator and Y expressions over dH³. **Both polynomial differences are exactly zero.** This proves equality where d,H are nonzero; the opposite-first-pair extension is checked separately by the derivation and curve oracles. Review also reconstructs all case totals and both fields' work ledgers, without rerunning the successful tests or study.

All 176 production-source hashes remain exact. There is no native arithmetic candidate, source adoption or new proof. The canonical verified-proof ledger remains 30,846.

## Work model

Counts include the scalar prefix-inversion work but exclude the fixed eight-lane inversion bundle. Fused calls contain two elementary products and share a reduction. The Fq2 lowering is counted separately.

| Recipe | Ordinary base products | Base squares | Fused two-product calls | Elementary products | Central inversions |
|---|---:|---:|---:|---:|---:|
| G1 new H-only | 7 | 2 | 2 | **13** | 1 |
| G1 old dH | 8 | 3 | 3 | 17 | 1 |
| G1 two binary | 10 | 2 | 0 | 12 | 2 |
| G2 new H-only | 11 | 0 | 15 | **41** | 1 |
| G2 old dH | 13 | 0 | 21 | 55 | 1 |
| G2 two binary | 14 | 0 | 10 | 34 | 2 |

G1 retains more elementary products than two binary stages, but two fused calls lower its chosen variable IFMA-template count to the same total, with one fewer central inversion bundle. G1 explicit linear-operation counts rise from 12 to 18 per triple against binary; correction, carry and register costs may offset the apparent opportunity. The following are **template counts, not newly emitted instruction counts or native timings**. They use the previously inspected 100/150-site ordinary/fused templates plus 500 sites per fixed inversion bundle; scalar product/inverse, shuffles, codecs, memory, fallback, admission and scheduling remain separate.

| Curve | Triples | New H-only sites | Old dH sites | Two binary sites |
|---|---:|---:|---:|---:|
| G1/Fq | 8 | 1700 | 2050 | 2200 |
| G1/Fq | 16 | 2900 | 3600 | 3400 |
| G1/Fq | 32 | 5300 | 6700 | 5800 |
| G1/Fq | 64 | 10100 | 12900 | 10600 |
| G1/Fq | 128 | 19700 | 25300 | 20200 |
| G2/Fq2 | 8 | 3850 | 4950 | 3900 |
| G2/Fq2 | 16 | 7200 | 9400 | 6800 |
| G2/Fq2 | 32 | 13900 | 18300 | 12600 |
| G2/Fq2 | 64 | 27300 | 36100 | 24200 |
| G2/Fq2 | 128 | 54100 | 71700 | 47400 |

## Next stage and limits

A native **complete unweighted G1 reduction** screen is justified. The proposed two-vector prefix/H record must be implemented and tested for exceptions, tails, scratch reuse, resource bounds and input lifetime. Reverse results must go to their original disjoint input slots; compaction follows all reverse stores. This is essential if original coordinates are reread. It is not yet a qualified memory saving. Weighted binary reductions retain their original identity and cannot simply become ternary. See the [native continuation](artifacts/next-native.md).

No native IFMA kernel, timings, CPU profiles, process peak/minimum RAM, pressure/swap, phone/ARM/TEE, concurrency, huge-circuit, AWS/Aeglos or new proof result is added. The model establishes no new2×path or all-eight10×achievement. Passing algebra does not permit production adoption.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.




[Analysis](artifacts/analysis.json), [independent symbolic/receipt/work review](artifacts/review.json), [derivation](artifacts/derivation.md), [design](artifacts/design.md), [new formula](artifacts/cancel.rs), [unchanged original models](artifacts/original.rs), [curve study](artifacts/study.rs), [native next stage](artifacts/next-native.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [exact source/receipts archive](artifacts/source-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). No external paper text, private inputs, keys/images/witnesses/proofs or executable/library blobs are included.
