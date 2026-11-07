# Shared-product common-Z tree

The algebra passes for G1 and G2. This recipe replaces inter-level inversions with a shared projective scale, at the cost of more multiplications. **There is no timing result or production adoption.** The work screen selects a small SIMD-finish follow-up; it does not select a whole-tree replacement.

## Construction

Each level starts with a common Jacobian Z. For pair i, let d_i=X2_i−X1_i and e_i=Y2_i−Y1_i. Form D=product(d_i), q_i=product of all other denominators, S=D², T=D³, u_i=e_i q_i. Then

    X'_i = u_i² − (X1_i + X2_i) S
    Y'_i = u_i (X1_i S − X'_i) − Y1_i T
    Z'   = Z D

Prefix/suffix products compute the cofactors without inversions. Odd carried points rescale by S/T. The singleton-pair case uses the ordinary equal-Z formula. Identity inputs or zero denominators decline the speculative path and complete original projective addition reduces the original input. Later failures discard all speculative state. Inputs remain immutable.

The [EFD Jacobian page](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html) supplies the coordinate convention and existing equal-Z formula. The shared-product lift is independently derived here; no mathematical novelty or published CPU speed claim is asserted. The earlier co-Z review required a shared scale but did not implement this construction. Earlier projective and frontier experiments retain their separate negative results.

## Exact abstract costs

M/S/I count operations in the curve's coordinate field. These are **not executed SIMD counts, native timings or a model of the accepted G2 norm-inversion kernel**. The explicit affine comparator uses one conventional batch inversion per binary level. Additions, checks, packing, shuffles, allocation and exceptional work are unpriced.

| Points | Abstract affine M/S/I | Common-Z, affine output M/S/I | Inversions saved | Added M/S |
|---|---|---|---:|---|
|8|26 / 7 / 3|49 / 11 / 1|2|23 / 4|
|16|63 / 15 / 4|110 / 20 / 1|3|47 / 5|
|32|140 / 31 / 5|235 / 37 / 1|4|95 / 6|
|128|614 / 127 / 7|997 / 135 / 1|6|383 / 8|
|256|1,251 / 255 / 8|2,018 / 264 / 1|7|767 / 9|
|512|2,528 / 511 / 9|4,063 / 521 / 1|8|1,535 / 10|
|4,096|20,439 / 4,095 / 12|32,726 / 4,108 / 1|11|12,287 / 13|

For n=2^k, affine output adds exactly (3n−1)M+(k+1)S while removing k−1 inversions. Projective output adds (3n−4)M+kS and removes k inversions. Final affine normalization is charged explicitly. At n=16 the abstract break-even is 3I>47M+5S; at n=256 it is 7I>767M+9S. No fixed I/M exchange rate is assumed.

The growing multiplication burden does not support treating inversion removal as a free general speedup. A separate SIMD finish for **at most sixteen live points (two times the existing eight lanes)** remains a credible question: it can remove several small-tail inversions without paying the product lift through large levels. This limit comes from single-vector pair geometry, before native measurements; no threshold sweep is selected. That implementation, its real caller coverage, complete exceptions, memory behavior and latency remain unqualified. This is a selection for the next experiment, not an adoption or rejection of every common-Z algorithm.

## Qualification and review

One Rust/xtask workflow compiles the new model and runs it once; both commands succeed on their first executions. It links the unchanged accepted field library `1200e635…` with O3, native CPU, fat LTO and one codegen unit. It introduces no AVX kernel or core/prover change, and does not replay the accepted field/core/prover suites.

- 204 complete outputs per curve cover four generator walks, odd/even lengths through512 and three initial scales, including non-unit Fq2 scales. Generator-weighted sums and complete original additions agree.
- 1,068 instrumented level-cost checks per curve match the separately written formulas. Fifty declined cases cover empty/singleton, identity, equal/opposite, late equal/opposite and zero-scale inputs with complete fallback.
- An independent F1019 rational implementation checks1,824 outputs/carry identities. It uses direct all-other products rather than the candidate prefix/suffix routine and imports no native field or curve implementation.
- Offline review reconstructs all19 cost rows from ceil(n/2^level) geometry and checks power-of-two closed forms, source pins and both command/stream receipts. It does not rerun either arithmetic oracle.

Both native curve oracles share the accepted field implementation. The small-prime oracle independently checks the algebra, not BN254 field code or SIMD representation. The model is a research implementation, not portable/pressure/concurrent-proof qualification.

Its requested temporary payload is at most (2n+2 floor(n/2)) coordinate-field elements, excluding borrowed input points, Vec headers, allocator overhead and stack temporaries. That is24KiB for256 G1 points or48KiB for G2 in this scalar prototype; it is not prover RSS or minimum RAM. No lookup table, key copy, duplicated solver plan or production pool is added.

## Accepted comparisons, unchanged

These are inherited latest matched32-worker local full-proof observations and supplied historical M5 references, not this recipe's timings. [Accepted source manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json), [latest full-proof experiment and controls](../2026-11-06-window-batch-proof-capture176/README.md).

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU/local|Metal/local|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1×2|7.915095|37|24|4.675×|3.032×|3.7|
|2×2|11.169812|59|30|5.282×|2.686×|5.9|
|4×2|16.085882|109|48|6.776×|2.984×|10.9|
|8×2|24.517520|189|74|7.709×|3.018×|18.9|
|12×2|33.838626|284|101|8.393×|2.985×|28.4|
|16×2|40.083672|335|124|8.357×|3.094×|33.5|
|24×2|65.506198|528|176|8.060×|2.687×|52.8|
|merge24|65.200982|507|175|7.776×|2.684×|50.7|

Accepted merge54 is161.718956ms, without a supplied M5 reference. The all-eight CPU10× goal remains active and unmet,0/8. No new SDK request, proof export, Gnark/Aeglos/AWS comparison or memory-pressure result is produced; the canonical distinct-proof ledger remains31,387. The previous packed final-exponentiation experiment remains rejected.

[Prospective design](artifacts/design.md), [model](artifacts/model.rs), [counts and checks](artifacts/analysis.json), [independent review](artifacts/independent-review.json), [source/input identities](artifacts/pins.json), [workflow](artifacts/plan.json), [artifact checksums](SHA256SUMS).
