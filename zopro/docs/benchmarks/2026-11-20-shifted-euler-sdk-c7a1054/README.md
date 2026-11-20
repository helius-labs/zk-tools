# Shifted Euler inputs with the current 12×2 SDK on C7a

The current SDK loaded the archived shifted image with a read-only FFT input reference and matching K and public query adjustments. The image adds 482,944 bytes to the standard Euler key. The G2 grouping arm uses the same two-level plan under the existing preparation budget. The standard and shifted configurations use the original verifying key and expected public inputs.

The shift met the paired criterion on catalog, heldout, v58. The shifted and grouped configuration took **27.724 ms** on the catalog request and **27.946 ms** held out, against the **28.4 ms** target for 10× M5 CPU. The catalog result is **10.24×** relative to M5 CPU and **3.64×** relative to M5 Metal. The 10× Metal target is **10.1 ms**.

| Request | Frozen / standard / shifted / standard grouped / shifted grouped warm ms | Shifted vs standard | Shifted grouped vs standard grouped | Shifted grouped vs frozen | Standard / shifted grouped cold ms | Standard / shifted grouped peak MiB |
|---|---:|---:|---:|---:|---:|---:|
| catalog | 29.568 / 29.685 / 28.172 / 29.305 / 27.724 | -4.27% (7/8) | -5.23% (8/8) | -6.35% (8/8) | 317.8 / 339.0 | 544.6 / 529.0 |
| heldout | 29.911 / 29.932 / 28.382 / 29.300 / 27.946 | -4.80% (8/8) | -4.56% (8/8) | -6.57% (8/8) | 317.0 / 338.2 | 558.2 / 528.2 |
| v58 | 29.670 / 29.319 / 28.341 / 29.664 / 27.958 | -3.25% (8/8) | -5.68% (8/8) | -6.07% (8/8) | 318.4 / 339.9 | 549.9 / 532.7 |
| v139 | 40.136 / 39.751 / 39.988 / 40.552 / 39.855 | +0.43% (4/8) | -1.66% (7/8) | -1.28% (7/8) | 330.1 / 351.6 | 814.7 / 743.4 |

One c7a.24xlarge used the pinned 64 main and 32 frontend workers. Eight order-balanced blocks measure thirteen warm proofs after three primes. Warm time is the median of process means. Cold time includes startup and the first verified response in a fresh process. Processes share the host file cache. Peak RSS is the maximum process high-water mark across blocks. The paired criterion requires at least seven wins and a lower paired median in all three reported comparisons, with the same-build standard control within 2% of the frozen binary.

The zero-budget route completed valid proofs with the optional group plan declined. A local test compared the standard and shifted keys on three sets of independent A, B, C, and scalar inputs and recovered identical Groth16 parts. The prover and benchmark examples also passed a generic x86-64 build check. Gnark independently reverified all 360 first and final warm exports after retrieval. The temporary instance, volume, and bucket were removed.

## All eight supplied M5 references

The 12×2 row uses this run. The other seven rows use the verified unified C7a baseline. Together these measurements reach 10× on 5/8 shapes against M5 CPU and 0/8 against M5 Metal.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.842 | 37 / 24 | 4.72× / 3.06× |
| 2×2 | 10.329 | 59 / 30 | 5.71× / 2.90× |
| 4×2 | 14.132 | 109 / 48 | 7.71× / 3.40× |
| 8×2 | 18.678 | 189 / 74 | 10.12× / 3.96× |
| 12×2 | 27.724 | 284 / 101 | 10.24× / 3.64× |
| 16×2 | 33.365 | 335 / 124 | 10.04× / 3.72× |
| 24×2 | 47.470 | 528 / 176 | 11.12× / 3.71× |
| merge-24×1 | 46.193 | 507 / 175 | 10.98× / 3.79× |

[Summary](artifacts/summary.json) · [Samples and route checks](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Independent parts test](artifacts/parts-test.rs) · [Local trial and source](../2026-11-20-shifted-euler-sdk-ryzen1053/README.md) · [Unified C7a baseline](../2026-11-20-c7a-three-provers/README.md#supplied-m5-references)
