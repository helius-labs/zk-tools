# Compact quadratic-ring Fq: arithmetic feasibility

The representation and range bounds pass **36,488 field/codec comparisons**, but the specified generic arithmetic recipe increases nominal IFMA work: **123 versus 100** for multiplication and **174 versus 150** for a fused two-product operation. Conversion, normalization and other costs are additional. Close this recipe before native implementation; no prover speedup or slowdown was measured.

This investigates the BN254 **base field Fq**, rather than the earlier [Gaussian Fr FFT representation](../2026-10-27-gaussian-fr/README.md) or [six-coefficient PMNS bridge](../2026-11-09-pmns-point-bridge176/README.md). It preserves a logical 32-byte field encoding using two signed 128-bit coefficients. The proposal adds no expanded key, point table or solver plan. A complete native point/key format was not implemented.

## Construction and range

For BN parameter `u = 4965661367192848881`, set

```
a = 6u² + 3u + 1 = 147946756881789319005730692170996259610
p = a² + 3u²
τ² = −3,       τ = −a/u mod p
π = a + uτ
```

The exact norm equals the current BN254 base-field prime. In the quadratic integer ring, π maps to zero in Fq. A coordinate pair `(x,y)` represents `(x+yτ)/R mod p`, with `R=2^131` and `H=2^127`. Host coefficients stay in `[-H,H)`.

For a raw quadratic product T, or a sum/difference of two products, compute

```
q = −T·π⁻¹ mod R, with each q coefficient centered in [−R/2,R/2)
C = (T + qπ)/R
```

The inverse exists because the norm p is odd. Both numerator components divide exactly by R. With input coefficient magnitudes at most H, fused-two-product bounds give

```
|C.x| ≤ 8H²/R + (a+3u)/2 < H
|C.y| ≤ 4H²/R + (a+u)/2 < H
```

The reader checks these strict inequalities by exact integer cross-multiplication. Thus even fused sum/difference outputs fit the signed 128-bit storage domain without relying on random-case behavior. Expanded radix-52 arithmetic needs six vectors per field batch, versus the current five; preserving host bytes does not preserve register or scratch costs.

Import and recentering use the lattice matrix `[[a,−3u],[u,a]]`, whose determinant is p. Rounding the inverse-matrix coordinates and subtracting the corresponding lattice vector bounds the result by `(a+3u)/2` and `(a+u)/2`, respectively. The model implements exact BigInt division. An efficient native normalizer, canonical equality and inversion bridge remain unqualified and unpriced.

## Complete generic multiplication ledger

Raw multiplication uses three Karatsuba products; low quotient formation uses three truncated products; constant-modulus multiplication uses two 3×3-limb products and one 3×2-limb product. Known-zero highest product halves are omitted. Truncated products count low/high IFMA contributions only where their bit positions are below 131.

|Specified stage|Ordinary product|Fused sum/difference of two products|
|---|---:|---:|
|Raw quadratic products|51|102|
|Low quotient|27|27|
|Multiply by constant π|45|45|
|Total nominal IFMA sites|123|174|
|Current generic template|100|150|
|Increase|23%|16%|

This is deliberately favorable arithmetic accounting. It omits signed lowering, small constant factors, carries, linear operations, canonical recentering, packing, import/export, inversion conversion, memory and scheduling. These are source-recipe counts, not emitted native instructions, throughput estimates or a lower bound on every quadratic-ring algorithm. Specialized squares and alternative multiplication/cancellation schedules are not a measured rescue. The current [stream cost model](../2026-11-06-stream-work-census176/README.md) supplies the accepted template comparison.

Splitting a 254-bit value into two smaller coordinates does not itself remove raw multiplication work: each smaller coefficient still needs three radix-52 limbs. The small second modulus coefficient helps one constant product but does not compensate for quotient and reduction work in this recipe. No native build is selected without a different work-removal mechanism.

## Validation and experiment cost

The optimized, overflow-checked Rust model uses independent ordinary BigInt modular arithmetic as its decoded field oracle. Coverage includes 100 boundary coefficient pairs, 4,096 deterministic full-width random pairs, all sixteen endpoint orientations, repeated changing chains and Fq2 products/norms:

|Check|Comparisons|
|---|---:|
|Ordinary products, including endpoint combinations|4,212|
|Fused products|8,392|
|Import/export and recentered add/subtract|12,588|
|Fused endpoint cases|32|
|Changing chains|8,192|
|Fq2 product/norm components|3,072|
|Total|36,488|

These are algebra-model checks, not native SIMD, curve, proof, portability or memory-pressure qualification. Test populations are reused across operations; the count does not imply that many independent field inputs.

The first compilation fails on an ambiguous integer literal in JSON output formatting. Its source, command and stderr remain. An explicit BigInt literal fixes only formatting; the original algebra and protocol remain unchanged. The corrected build takes **0.755 seconds** and the model run **0.057 seconds**; preparation and analysis are excluded. No successful build or numerical run is repeated.

A separate same-agent reader independently checks the norm/map identity, exact range inequalities, cardinalities and bit-position-based partial-product ledger. It verifies original/corrected command receipts, stream/source hashes and the unchanged proof ledger without replaying the numerical model. No external audit is claimed.

The broader history review also found prior native outcomes for ordinary-window routing, pending triples, ring queue trees, four-lane SIMD, blocked inputs and uniform prefixes; those were not rerun. A newly inspected [concurrent-MSM disclosure](https://patents.google.com/patent/WO2025125771A1/en) reorganizes weighted bucket aggregation for parallel execution and explicitly does not reduce total point additions. It does not supply a twofold ordinary-stream work reduction; no source implementation or performance claim is imported.

Production and accepted performance remain unchanged. The primary local full proof is **7.821726 ms**, against the **3.7 ms** target. All eight supplied CPU/Metal comparisons remain in the [current cost-map table](../2026-11-11-cost-map177/README.md#accepted-full-proof-reference); the verified CPU 10× goal is **active and unmet, 0/8**. Canonical proof inventory remains **53,917**, SHA-256 `a033132ee87cf5d5260c396acf1894bbb1afc95c07d8ba22c55ed9c8e5a85873`. No new proof, SDK/AWS/Aeglos result, production edit, push or publication occurs.

[Protocol](artifacts/protocol.md), [model](artifacts/model-fixed.rs), [results](artifacts/result.json), [reader](artifacts/review.rs), [review](artifacts/review.json), [receipts and original source](artifacts/evidence.tar.gz), [checksums](SHA256SUMS). Reproduction requires the pinned local Rust toolchain and retained dependencies; compiled binaries and private proving inputs are excluded.
