# Packed final exponentiation in the SDK on C7a

The SDK opt-in cleared the frozen release and same-build controls on all three C7a inputs. The original 1×2 request took **7.736 ms**, or **4.78× M5 CPU** and **3.10× Metal**. The 10× M5 CPU target for this request is **3.7 ms**.

The verifier packs the cyclotomic square into AVX-512 IFMA lanes. It keeps the ordinary Miller arithmetic and uses no extra key tables. Enable it with `Prover::set_packed_final_exponentiation(true)`. The option defaults to off. The generic x86-64 build uses scalar arithmetic.

Zopro used 80 main and 16 frontend cores. Prepared Aeglos used 96 workers with cache hits on every timed proof. Each timing is the median of twelve counterbalanced process means after three primes. The controls are frozen Zopro, the new SDK with the opt-in off, and the exact experimental packed executable from the earlier C7a run.

| Input | Frozen ms | SDK off ms | SDK on ms | Prior packed ms | Prepared Aeglos ms | SDK on / off peak MiB |
|---|---:|---:|---:|---:|---:|---:|
| catalog | 7.820 | 7.809 | 7.736 | 7.761 | 8.040 | 43.8 / 41.5 |
| heldout | 7.820 | 7.803 | 7.749 | 7.702 | 8.069 | 40.8 / 43.0 |
| v27 | 7.778 | 7.801 | 7.736 | 7.724 | 8.045 | 40.8 / 43.6 |

| Input | SDK off vs frozen | SDK on vs frozen | SDK on vs off | SDK on vs prior packed | SDK on vs Aeglos |
|---|---:|---:|---:|---:|---:|
| catalog | -0.11% (6/12) | -1.04% (11/12) | -0.74% (11/12) | +0.32% (5/12) | -3.80% (12/12) |
| heldout | +0.04% (6/12) | -0.80% (10/12) | -1.23% (10/12) | +0.28% (5/12) | -4.34% (12/12) |
| v27 | +0.12% (4/12) | -0.80% (10/12) | -1.10% (11/12) | +0.06% (6/12) | -3.71% (11/12) |

All 183 exported proofs independently verified with Gnark. The 40 native BN254 tests and the request-bound verifier test passed. A native probe saw three packed exponentiations per proof with the option enabled and zero with it disabled. The generic x86-64 build verified a proof with the option enabled. The C7a instance terminated, the bucket was deleted, and the final check found zero tagged volumes or active benchmark instances.

## Eight supplied M5 references

Five of eight supplied shapes reach 10× against M5 CPU. The table uses the SDK result for the original 1×2 input and earlier verified results for the other shapes. [Prior reference table](../2026-11-20-euler-fft-sdk-c7a943/README.md#eight-supplied-m5-references)

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.736 | 37 / 24 | 4.78× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Route checks](artifacts/routes.json) · [Portable verification](artifacts/portable-verified.json) · [AWS cleanup](artifacts/cleanup.json)
