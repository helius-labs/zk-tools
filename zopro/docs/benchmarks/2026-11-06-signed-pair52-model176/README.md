# Signed radix-52 arithmetic model

The fixed signed-difference multiplication recipe passed its exact arithmetic screen. This selects a native primitive implementation and code-generation check; it does not establish a speedup or change production.

The model checked **240,048 field reductions and 1,200,240 cancellation rows**: Fq and Fr, R256 and R260, ordinary products, fused sums, and differences implemented through the nonnegative `2p-c` complement. Each field/radix had 122 boundary values (14,884 ordered tuples), 4,096 deterministic full-width random tuples, and 1,024 changing-chain tuples, with three operations per tuple. Another 4,546 convolutions covered arbitrary full-width 52-bit limbs. Every modeled cancellation state matched independent integer division, the final Montgomery quotient matched an extended-Euclid reference, and the canonical result matched modular arithmetic using an independently computed radix inverse. No unreduced host field objects were constructed.

The signed difference identity preserves the existing radix and Montgomery factors. A shared diagonal convolution reduces repeated additions. This differs from the earlier rejected radix-51 pair-sum implementation; that result remains unchanged. The mathematical bounds and limitations are in [bounds.md](artifacts/bounds.md). Signed intermediate coefficients require arithmetic carry shifts in a native implementation. All modeled accumulators satisfied the declared signed-64-bit bounds. Finite checks supplement the bounds and do not qualify native SIMD code.

The source recipe uses **80 versus 100 IFMAs for an ordinary product**, and **110 versus 150 for a fused product**. It also requires 83/153 difference/diagonal vector or mask operations before common reduction and normalization costs. These are partial source-operation counts, not optimized instruction counts or a latency estimate. Additional arithmetic, coefficient dependencies, register pressure and spills may outweigh the saved IFMAs. No lookup table, key copy or retained execution plan is proposed. Actual native stack and runtime memory remain unmeasured.

The Rust model build, public-fixture preparation and model execution each succeeded on the first attempt through xtask. A separate receipt/ledger review verified 19 run pins, all three command receipts and unchanged production source identities (176 files). It did not rerun the model. The archive contains the exact public fixtures, source and receipts; no executable, library, key, witness or proof blobs are included.

The pairwise-product motivation is discussed in the authors' [June 2026 preliminary matrix-arithmetic paper, §3.5](https://www.texmacs.org/joris/amx/amx.html). Its matrix results do not imply a BN254 field or prover speedup. The [previous radix-51 experiment](../2026-10-23-pairwise-radix51-component/README.md) remains closed.

## Accepted full-proof baseline (inherited)

No new full proof, AWS comparison, Aeglos comparison, RSS peak or minimum-RAM result was produced in this stage. The accepted local 32-worker build remains unchanged. M5 CPU and Metal timings are supplied reference values on different hardware; these ratios do not establish equal hardware efficiency. The goal remains unmet on all eight shapes.

| Shape | Local CPU ms | M5 CPU ms | M5 Metal ms | CPU speedup | vs Metal | 10× CPU limit ms |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 4.67× | 3.03× | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.29× | 2.69× | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 6.78× | 2.98× | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 7.82× | 3.06× | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 8.38× | 2.98× | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 8.36× | 3.09× | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 8.02× | 2.67× | 52.8 |
| merge24 | 65.060937 | 507 | 175 | 7.79× | 2.69× | 50.7 |

Additional merge54 regression baseline: 161.883899 ms; no supplied M5 reference. The inherited accepted RSS is 132.75 MiB, not a new peak or a minimum-RAM measurement. Accepted SDK SHA-256: `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`; canonical proof ledger remains 30,846.

## Next stage

Implement this one fixed signed-difference recipe in an isolated field snapshot, preserving the accepted fallback and field representation. Qualify its actual SIMD outputs against independent field arithmetic, then inspect the final optimized primitive for IFMA counts, signed carries and stack traffic. Only a credible native result selects separately frozen matched caller and full-proof measurements. No width/layout/attribute sweep, favorable subset, old experiment rerun or production adoption is selected here.

Raw [model output](artifacts/model-result.json), [protocol](artifacts/protocol.json), [review](artifacts/review.json), [source pins](artifacts/source-pins.json) and [source/receipts archive](artifacts/source-and-receipts.tar.gz) are preserved. The full-limb convolution population checks exact products but does not maintain a separate output digest; its displayed digest is the empty-stream SHA-256.

The first receipt-review helper stopped because it compared the input plan with xtask's normalized protocol, which inserts default command fields. Only the reviewer was repaired to account for those defaults; the successful model, build and fixture stages were not rerun. The original helper source and observed-failure summary are retained in the archive, with its binary identity in the review.
