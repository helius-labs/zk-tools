# Euler FFT leaf fusion in the SDK on C7a

The SDK opt-in cleared the frozen release and same-build controls on both C7a inputs. The original request took **11.898 ms**, or **4.96× M5 CPU** and **2.52× Metal**. The 10× M5 CPU target for 2×2 is **5.9 ms**.

The option shares one expanded leaf between the inverse and forward transforms and applies the four Euler weights with a circulant matrix. It keeps the existing packing and column passes and adds no retained lookup tables. It applies to ordinary packed FFT inputs on native AVX-512 IFMA builds. Enable it with `Prover::set_euler_fft_leaf_fusion(true)`. The option defaults to off.

The C7a run pinned 64 main and 32 frontend Zopro workers on 96 physical cores. Prepared Aeglos used 96 workers with cache hits on all timed rounds. Each table entry is the median of 12 paired process means on C7a or eight on Ryzen 24+8. Peak MiB is maximum whole-process RSS. Paired changes use block-by-block ratios.

## C7a proof latency

| Request | Frozen ms | SDK off ms | SDK on ms | Prior shift ms | Prepared Aeglos ms | SDK on / off peak MiB |
|---|---:|---:|---:|---:|---:|---:|
| catalog | 12.113 | 12.043 | 11.898 | 11.876 | 11.614 | 74.3 / 76.7 |
| heldout | 12.124 | 12.113 | 11.887 | 11.984 | 11.443 | 74.4 / 76.7 |

| Request | SDK off vs frozen | SDK on vs frozen | SDK on vs off | SDK on vs prior shift | SDK on vs Aeglos |
|---|---:|---:|---:|---:|---:|
| catalog | -0.97% (8/12) | -1.88% (11/12) | -1.40% (9/12) | +0.06% (6/12) | +2.07% (0/12) |
| heldout | -0.01% (7/12) | -1.56% (12/12) | -1.44% (12/12) | -0.69% (9/12) | +3.73% (1/12) |

## Ryzen confirmation

| Request | Frozen ms | SDK off ms | SDK on ms | Prior shift ms | SDK on vs off |
|---|---:|---:|---:|---:|---:|
| catalog | 9.506 | 9.457 | 9.382 | 9.425 | -0.80% (7/8) |
| heldout | 9.480 | 9.474 | 9.397 | 9.406 | -0.89% (6/8) |

Ryzen also cleared both controls on both inputs.

All 23 native FFT tests passed. A native probe saw four calls to the fused transform with the option enabled, one for each of three primes and one timed proof, on each input. The disabled and packed-input routes made zero calls. The generic x86-64 build produced a verified proof with the option enabled. All 191 exported proofs were independently verified, including the probe and portable outputs. The first cloud attempt stopped at the Aeglos export path check. After correcting that path, the full panel ran again. AWS cleanup confirmed that both instances terminated, both buckets were removed, and zero tagged volumes or active benchmark instances remained.

## Eight supplied M5 references

Five of eight supplied shapes reach 10× against M5 CPU. The original 2×2 input uses the verified SDK result from this run. The other rows use the [earlier verified measurements](../2026-11-20-packed-final-c7a925/README.md#eight-supplied-m5-references).

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.752 | 37 / 24 | 4.77× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Results](artifacts/summary.json) · [C7a protocol](artifacts/cloud-protocol.json) · [C7a samples](artifacts/cloud-rows.json.gz) · [Native route checks](artifacts/routes.json) · [Portable proof check](artifacts/portable-verified.json) · [AWS cleanup](artifacts/cleanup.json)
