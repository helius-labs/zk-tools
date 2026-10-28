# Three-point affine addition screen

A three-point addition can expose both affine denominators before inversion, replacing two dependent batched inversion stages with one. The exact model passes its checks, but increases multiplication work. **Only a small-batch native caller experiment is justified; no proving speedup or production change is established.** Accepted local 1x2 remains 8.324853 ms at 134.50 MiB peak RSS.

## Formula and scope

For finite P=(a,b), Q=(c,e), R=(r,s), set d=c-a, n=e-b and H=d²(a+c+r)-n². Define U=d(b+s)+n(r-a), D=dH, A=nH and B=d²U. Inverting D once yields the first slope l=A/D and the sum of the two slopes w=B/D. With k=w-l, reconstruct:

```text
xS = l²-a-c
x  = k²-xS-r
y  = w(xS-x)+l(x-a)+b
```

H=d²(r-xS); the expanded final expression equals the ordinary second chord formula. This is a local derivation, without a novelty claim. The [derivation](artifacts/derivation.md) specifies the field lowering, work accounting and storage exclusions.

Infinity, d=0 and H=0 return a typed fallback request before inversion. The scalar model does not implement complete native fallback. A future integration must preserve original inputs, handle exceptional results using complete rules, and defer compaction until reverse writes finish. For G2, inversion uses an Fq prefix chain over norms and shares the recovered Fq2 inverse between the two slope numerators.

The review used the [EFD affine formulas](https://www.hyperelliptic.org/EFD/g1p/auto-shortw.html), [EFD XYZZ formulas](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-xyzz.html), and selected sections of the [2011 co-Z author paper](https://dspace.jaist.ac.jp/dspace/bitstream/10119/10293/1/17117.pdf). Co-Z addition requires a common projective coordinate; arbitrary MSM inputs require additional work to obtain that relation. XYZZ mixed addition costs 8M+2S before normalization. Neither count establishes an advantage against the actual shared-inversion affine baseline. The paper PDF stays local with a pinned hash and is not redistributed here. The earlier [complete projective reduction trial](../2026-10-22-universal-g1-projective-reduction-sdk/README.md) remains separate negative evidence.

## Exact checks

Three new tests pass once: exhaustive addition/guard checks on an 18-point toy curve (5,832 triples), 1,024 Fq2 lowering checks against Arkworks, and explicit operation-ledger checks. The separate BN254 study performs 11,264 triple attempts across G1 and G2, using boundary scalars, a fixed pool with full-width random scalars, and changing chains. All 10,508 ordinary results agree with independent generator scalar sums; 756 attempts request fallback. Each curve has 224 infinity, 110 first-denominator and 44 second-denominator guards. Guard checks perform no inversion and do not claim a computed result.

An additional 496 triples cover batch sizes 8/16/32/64/128 across both curves. Direct and two-stage binary outputs agree with independent scalar-sum references. These are not included in the 11,264-attempt total. These algebra checks do not qualify a SIMD implementation or a complete prover.

The initial test build lacked an Arkworks trait import; the initial study build used an incompatible dependency search path. Failed sources and logs remain archived. Both were repaired before their respective successful execution; successful tests and study were not repeated.

## Work and memory

Per triple, excluding the fixed native inversion bundle:

| Curve and recipe | Ordinary Fq products | Fq squares | Fused two-product calls | Elementary products | Central inversion stages |
|---|---:|---:|---:|---:|---:|
| G1 direct | 8 | 3 | 3 | 17 | 1 |
| G1 two binary | 10 | 2 | 0 | 12 | 2 |
| G2 direct | 13 | 0 | 21 | 55 | 1 |
| G2 two binary | 14 | 0 | 10 | 34 | 2 |

The inspected eight-lane inversion bundle costs five vector products plus one scalar product and one scalar inverse, as well as conversions and shuffles. Using the accepted 100-IFMA ordinary and 150-IFMA fused templates gives:

| Triples | G1 direct / binary IFMA sites | G2 direct / binary IFMA sites |
|---|---:|---:|
| 8 | 2050 / 2200 | 4950 / 3900 |
| 16 | 3600 / 3400 | 9400 / 6800 |
| 32 | 6700 / 5800 | 18300 / 12600 |
| 64 | 12900 / 10600 | 36100 / 24200 |
| 128 | 25300 / 20200 | 71700 / 47400 |

Direct additionally saves one scalar product and inverse at every size. Counts exclude linear/carry/correction instructions, packing, loads/stores, shuffles, spills, compaction, masks, fallback, admission and scheduling. They are neither cycle estimates nor native measurements. Variable multiplication work grows substantially, especially for G2; removing an inversion stage alone does not establish a win.

Proposed minimal records use four Fq words per G1 lane or eight per G2 lane, with original coordinates reread in reverse. Retaining coordinates instead increases those records. No native layout, packing advantage, peak RSS or minimum RAM is established. There is no proposed per-key point table or duplicated execution plan.

Next: compare a direct triple reduction with **two actual existing binary layers** in a bounded small-batch caller, including exceptional fallback, real geometry and all data movement. Require a caller gain before any SDK trial. No native timing, profile, new proof export, AWS/Aeglos comparison, pressure, phone, TEE, concurrency or large-circuit qualification occurred in this screen. The independent proof ledger remains 12,524. All 172 accepted production files remain unchanged.

## Inherited CPU and Metal comparison

The [accepted report](../2026-10-27-clean-fft-adoption/README.md), including its full-performance-gate failure and engineering exception, remains authoritative. M5 figures are supplied historical references. The original all-eight CPU 10x goal remains active and unmet, 0/8.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445x | 24 | 2.883x |
| 2x2 | 11.460675 | 59 | 5.148x | 30 | 2.618x |
| 4x2 | 15.854365 | 109 | 6.875x | 48 | 3.028x |
| 8x2 | 24.084626 | 189 | 7.847x | 74 | 3.072x |
| 12x2 | 33.693397 | 284 | 8.429x | 101 | 2.998x |
| 16x2 | 39.981636 | 335 | 8.379x | 124 | 3.101x |
| 24x2 | 66.010670 | 528 | 7.999x | 176 | 2.666x |
| Merge24x1 | 65.086624 | 507 | 7.790x | 175 | 2.689x |

Accepted merge54 remains 161.341835 ms without a supplied M5 reference.

[Protocol](artifacts/design.md), [results](artifacts/analysis.json), [tests](artifacts/tests.log), [source and dependency pins](artifacts/pins.json), [closeout](artifacts/closeout.json), [artifact hashes](artifact-index.json).
