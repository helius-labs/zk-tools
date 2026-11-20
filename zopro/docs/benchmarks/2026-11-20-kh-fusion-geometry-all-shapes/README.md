# K/H fusion and peak memory across 41 C7a circuit shapes

**Six shapes fit the existing small or medium fusion path.** Five now use it in the selected Zopro scorecard profile. The sixth, 2×4, has a measured trade-off between latency and peak memory.

At 32+64 workers, 1×8 peak memory fell from **139.7 to 78.3 MiB** on the primary input. Paired latency fell 3.47% and 3.51% on the two inputs. At 40+56 workers, the fused profile peaked at 68.5 MiB on the primary input. The scorecard uses the matched 32+64 result. [1×8 timing and memory](../2026-11-20-one-input-kh-fusion-c7a1036/README.md)

At 64+32 workers, 2×4 peak memory fell from **186.2 to 163.0 MiB** on the primary input while paired latency rose 6.09% and 8.04% across the two inputs. With 48+48 workers, fusion cut the primary peak from 136.8 to **78.3 MiB**. That profile ran slower than the 64+32 latency control. The scorecard keeps its earlier latency profile. [2×4 timing and memory](../2026-11-20-two-input-kh-fusion-c7a1038/README.md)

## How the prover decides

K and H are two G1 sums that contribute to the same proof component. Fusion lets their points use one bucket set per window. The CPU route checks the key dimensions and the proof call. Both folded sums must have 1,024–65,535 terms, H must have unit scale, and the proof mode must consume the combined result. The canonical wire path also requires each original key point array to be smaller than 65,536 points. It uses the small path when K and H total fewer than 65,536 terms. Above that total, the medium path requires its opt-in and a successful scratch reservation. Large shapes use the general route.

The census reads all 41 SHA-256-pinned proving-image headers from the exact selected workflows. K includes folded solver slots. The Euler H query uses the original constraint rows. Ten additional local proofs verified the selected fusion calls on 1×2, 1×4, 1×16 and 2×2 and the general route on 2×8 with the medium flag enabled. The separate 1×8 and 2×4 runs confirmed actual fusion on both inputs. [Verifier and route data](artifacts/routes.json)

## All 41 shapes

Peak memory is the recorded whole-process RSS and includes key setup. The two peak columns show the same Zopro profiles as the latency scorecard. Geometry is computed from the selected key. The 35 large shapes require investigation in the general route for related optimizations.

| Shape | K terms | H terms | FFT domain | Zopro primary peak MiB | Zopro held-out peak MiB | Selected route |
|---|---:|---:|---:|---:|---:|---|
| 1×2 | 30,529 | 30,922 | 32,768 | 45.2 | 48.2 | Small fused K/H |
| 1×4 | 34,195 | 34,586 | 65,536 | 54.2 | 57.3 | Medium fused K/H |
| 1×8 | 42,763 | 43,162 | 65,536 | 78.3 | 74.6 | Medium fused K/H |
| 1×16 | 59,131 | 59,546 | 65,536 | 91.0 | 89.5 | Medium fused K/H |
| 2×2 | 53,079 | 53,874 | 65,536 | 78.9 | 74.5 | Medium fused K/H |
| 2×4 | 56,762 | 57,554 | 65,536 | 186.1 | 191.5 | Separate K/H |
| 2×8 | 65,362 | 66,162 | 131,072 | 183.4 | 180.2 | General |
| 2×16 | 81,794 | 82,610 | 131,072 | 238.6 | 241.6 | General, direct tiles |
| 3×2 | 75,069 | 76,266 | 131,072 | 183.1 | 181.7 | General, direct tiles |
| 3×4 | 78,769 | 79,962 | 131,072 | 213.1 | 195.1 | General, direct tiles |
| 3×8 | 87,401 | 88,602 | 131,072 | 218.5 | 220.7 | General, direct tiles |
| 4×2 | 97,075 | 98,674 | 131,072 | 211.7 | 217.6 | General |
| 4×4 | 100,792 | 102,386 | 131,072 | 216.2 | 218.4 | General, direct tiles |
| 4×8 | 109,456 | 111,058 | 131,072 | 241.0 | 236.4 | General, direct tiles |
| 4×16 | 126,016 | 127,634 | 131,072 | 263.4 | 281.6 | General, direct tiles |
| 5×2 | 119,673 | 121,674 | 131,072 | 220.6 | 218.4 | General |
| 5×4 | 123,407 | 125,402 | 131,072 | 256.0 | 243.2 | General |
| 5×8 | 132,103 | 134,106 | 262,144 | 309.4 | 304.0 | General |
| 5×16 | 148,727 | 150,746 | 262,144 | 327.1 | 327.2 | General |
| 6×2 | 141,711 | 144,114 | 262,144 | 298.7 | 311.0 | General |
| 6×4 | 145,462 | 147,858 | 262,144 | 320.7 | 314.5 | General |
| 6×8 | 154,190 | 156,594 | 262,144 | 351.5 | 334.9 | General |
| 8×2 | 186,411 | 189,618 | 262,144 | 360.3 | 371.4 | General |
| 8×4 | 190,196 | 193,394 | 262,144 | 377.7 | 371.8 | General |
| 8×8 | 198,988 | 202,194 | 262,144 | 436.4 | 399.0 | General |
| 8×16 | 215,804 | 219,026 | 262,144 | 460.6 | 444.0 | General |
| 12×2 | 275,427 | 280,242 | 524,288 | 453.9 | 492.4 | General |
| 12×4 | 279,280 | 284,082 | 524,288 | 548.5 | 528.5 | General |
| 12×8 | 288,200 | 293,010 | 524,288 | 574.1 | 559.2 | General |
| 16×2 | 364,699 | 371,122 | 524,288 | 610.8 | 615.3 | General |
| 16×4 | 368,620 | 375,026 | 524,288 | 681.0 | 689.3 | General |
| 16×8 | 377,668 | 384,082 | 524,288 | 701.0 | 698.5 | General |
| 24×2 | 544,587 | 554,226 | 1,048,576 | 921.0 | 933.2 | General |
| 24×4 | 548,644 | 558,258 | 1,048,576 | 1039.7 | 1066.4 | General |
| 32×2 | 724,391 | 737,250 | 1,048,576 | 1335.7 | 1327.6 | General |
| 40×2 | 901,159 | 917,250 | 1,048,576 | 1636.1 | 1532.8 | General |
| 48×2 | 1,079,271 | 1,098,594 | 2,097,152 | 1771.0 | 1814.2 | General |
| 49×2 | 1,101,373 | 1,121,100 | 2,097,152 | 1598.0 | 1685.3 | General |
| merge-8×1 | 175,481 | 178,623 | 262,144 | 358.4 | 361.5 | General |
| merge-24×1 | 516,297 | 525,759 | 1,048,576 | 975.3 | 944.5 | General |
| merge-54×1 | 1,156,887 | 1,178,199 | 2,097,152 | 1846.0 | 1801.6 | General |

## Larger-shape findings

- **2×8 is the first shape outside the size guard.** Its H query has 66,162 terms and its FFT domain has 131,072 entries. A follow-up needs a general-route implementation with a measured scratch bound. On 4×2 and 8×2, the previous prototype that merged point-range buckets before weighted reduction raised proof latency by 5.52–8.93% locally. [Large K/H experiment](../2026-11-20-merged-partitioned-kh-ryzen1028/README.md)
- **Direct tiles already provide a similar win on six large shapes.** At fixed placement they reduced primary peak RSS from 248.4 to 238.6 MiB on 2×16, 189.1 to 183.1 on 3×2, 228.1 to 218.5 on 3×8, 224.4 to 216.2 on 4×4, 252.1 to 241.0 on 4×8, and 289.8 to 263.4 on 4×16. Peak RSS also fell on the held-out requests and paired latency improved in all six cases. The 3×4 tile result improved speed but increased primary peak memory from 206.4 to 213.1 MiB. [3× and 4× tile comparisons](../2026-11-20-grouped-transfers-c7a988/README.md) · [2×16, 3×8 and 4×16 tile comparisons](../2026-11-20-medium-profile-transfer-c7a985/README.md)
- **48×2 and 49×2 show another cross-shape peak reversal.** Primary peak memory is 1,771.0 MiB and 1,598.0 MiB respectively, despite the same FFT domain and more terms on 49×2. The peak images do not identify the source of that difference. It needs a same-shape allocation or layout comparison before crediting a specific optimization. [Full-catalog timing](../2026-11-20-all-shapes-c7a979/README.md)

The remaining general-route entries are geometry and measured RSS observations. Timing benefit from a different partition or tile layout still needs a same-shape comparison.

[Zopro vs Aeglos scorecard](../2026-11-20-c7a-41-shape-latency-memory/README.md) · [Pinned key geometry](artifacts/geometry.json) · [Shape census CSV](artifacts/geometry.csv) · [M5 CPU and Metal references](../2026-11-20-geometry-placement-c7a1027/README.md#all-eight-supplied-m5-references)
