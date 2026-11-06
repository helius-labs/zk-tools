# Joint signed-pair complex arithmetic model

The fixed jointly accumulated complex-product recipe passes its exact coefficient and signed-Montgomery-reduction screen. This selects one native feasibility implementation and focused independent SIMD/code-generation qualification. **No speedup or adoption is established.** Production remains unchanged.

## Arithmetic

Three unreduced products share work between both output polynomials. Normal multiplication uses S=a0+a1,T=b0+b1,D=b1-b0 (normalized modulo2p), P=S*b0,Q=a1*T,U=a0*D, then(real,imag)=(REDC(P-Q),REDC(P+U)). Conjugation instead uses T=b0-b1,D=b0+b1 and(real,imag)=(REDC(P-Q),REDC(P-U)). The model matches direct complex identities modulo p and the input Montgomery radix. No invalid unreduced host field objects are constructed.

Each raw product uses five diagonal and ten oriented signed limb-difference products. The diagonals mix directly into two outputs, and the P cross pieces are computed once and shared. This is a joint dataflow, not three separately materialized full convolutions. Signed real outputs, and both conjugated outputs, require arithmetic carries and a signed top until conditional addition of2p. Ordinary field multiplication and Fr/R260 FFT arithmetic are outside the proposed API change.

## Model evidence

The new model checked **324,160 reduced outputs and1,620,800 exact cancellation rows** over Fq/Fr, R256/R260, normal/conjugated products, lazy endpoints, full-width random inputs and changing chains. Each field/radix has14,884boundary tuples,4,096random tuples,256endpoint-cartesian tuples and1,024chain tuples, with four outputs per tuple. Another6,144joint full52-bit polynomial calls check12,288output polynomials without assuming field bounds. The inherited public fixture is reapplied to a different arithmetic algorithm; no old model is rerun and none of these component results is a new proof.

Every joint polynomial matches independent integer products. Every cancellation state is checked for divisibility and its exact signed quotient; full REDC matches an independent extended-Euclid quotient, and canonical values match direct complex arithmetic with a modular-power radix inverse. The model exercises **7537negative reduced representatives** and **16unsigned subtract2p corrections**. Each field/radix exercises negative-top correction; the larger radix needs no unsigned correction.

The general bounds and source recipe are in [bounds.md](artifacts/bounds.md). The two polynomials start below32B in coefficient magnitude, cancellation stays below64B, and final scaling/correction stays below1026B<2^63, B=2^52. Observed maxima supplement these arguments. Signed raw reductions lie in(-p,2p), while a normal unsigned imaginary reduction is below3p. Conditional2p corrections restore normalized[0,2p) outputs. These are mathematical bounds and finite checks, not native machine-code or memory qualification.

## Work and native obligations

The fixed source proxy has **190 IFMAs versus300** for two retained fused products:90input plus100reduction. It adds296input vector/mask operations before three field-linear operations, reducer/carry/correction ALU, comparisons, loads, moves and spills. The 190count is not an optimized instruction count or a latency bound. A native result can lose despite fewer IFMAs.

Twenty live output coefficients represent1,280logical vector bytes at eight lanes, before operands and scratch. This is not a frame, concurrent-lifetime bound, actual spill traffic, RSS peak or minimum RAM. The native implementation must retain joint accumulation, qualify actual signed corrections, and inspect final compiler code. It introduces no proposed heap/table/key-copy/plan/pool expansion.

This differs from the [closed250-IFMA Gauss caller](../2026-10-29-positive-gauss-caller175/README.md) and from three independently reduced products: the limb-product count, joint dataflow and signed correction differ. The [global ordinary signed-pair replacement](../2026-11-06-signed-pair52-screen176/README.md) remains closed with all gains/regressions retained; this is not a representation subset adoption of that build. The [Toom-5 screen](../2026-11-06-toom5-work176/README.md) also remains closed.

## Execution and next stage

The normal Rust build and new model run both succeed first attempt. Separate review checks22pins, two successful receipts, all176unchanged production-source identities, row/output totals, exercised correction paths and the reconstructed source ledger. It does not rerun the model. Archive member/extraction hashes and final checksums pass.

Next is one fixed native complex implementation with both normal and conjugated paths, independent raw-output/export oracles and optimized final-consumer inspection. Only credible native code selects separately frozen complete G2 callers (200/352, tails,1/16/32workers, both controls and automatic profiles), followed by all-eight plus merge54 full SDK, resource/portable and independent Gnark proof qualification before adoption. No width/layout/attribute sweep, old-kernel reroll or favorable subset is selected.

Accepted production176/SDK6d83 remain unchanged: local32 1×2 **7.928660ms**, merge54 **161.883899ms**; inherited132.75MiB RSS is not a new peak or minimum. Canonical proof ledger remains30,846. The inherited [all-eight M5 CPU and Metal comparison](../2026-11-06-signed-pair52-model176/README.md#accepted-full-proof-baseline-inherited) is unchanged. The all-eight10× CPU goal remains active and unmet. No new native/profile/proof/AWS/Aeglos/large/mobile/pressure/swap/throughput result exists.

[Model output](artifacts/result.json), [protocol](artifacts/protocol.json), [review](artifacts/review.json), [pins](artifacts/pins.json), [bounds](artifacts/bounds.md) and [source/receipts](artifacts/source-and-receipts.tar.gz) are preserved. The inherited public fixture remains in its parent archive; executable/library/key/witness/proof blobs are excluded.
