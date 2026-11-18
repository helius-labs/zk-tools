# Full G2 and FFT overlap on C7a

The FFT uses a caller-attached frontend pool. The selected setting starts G2, or both G2 and A, before the FFT returns. The wait control uses the same binary and frontend width and runs the FFT first. Five screening blocks select a mode and width, then seven new randomized blocks compare it against both controls. Each small-shape process times 61 requests after three warmups.

| Shape | Cores | Selected / front | Release ms | Wait ms | Overlap ms | Paired vs release | Faster blocks | Paired vs wait | Faster blocks | Peak MiB wait / overlap | M5 CPU / Metal speedup |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | g2 / 24 | 8.187 | 8.278 | 11.737 | +43.35% | 0/7 | +41.11% | 0/7 | 81 / 57 | 3.15× / 2.04× |
| 1x2 | 96 | g2a / 16 | 8.260 | 8.284 | 11.320 | +37.29% | 0/7 | +36.36% | 0/7 | 70 / 53 | 3.27× / 2.12× |
| 2x2 | 48 | g2 / 16 | 12.960 | 13.061 | 15.184 | +17.37% | 0/7 | +17.13% | 0/7 | 144 / 143 | 3.89× / 1.98× |
| 2x2 | 96 | g2 / 24 | 13.172 | 13.183 | 16.395 | +25.51% | 0/7 | +24.38% | 0/7 | 157 / 153 | 3.60× / 1.83× |

On 96 cores the matched Aeglos median was 7.943 ms with 469 MiB peak RSS. The selected Zopro overlap setting measured +42.19% in paired comparisons with 0/7 lower process means.

## Early verification on 1×2

At 96 cores, starting the request’s public-input Miller loop during generation took **8.060 ms** versus **8.260 ms** for the integrated release and **7.943 ms** for Aeglos. The paired improvement over the release was 2.45%, with seven wins in seven blocks. Aeglos was faster in five of those seven blocks. Maximum process RSS was 48.2 MiB for this Zopro candidate and 468.6 MiB for Aeglos. Each process timed 61 complete verified requests. [Verification schedules](caller.md)

## 16×2 selection

The screen chose release on 60 cores at 33.735 ms. Fresh confirmation measured 33.852 ms against the fixed 64-core release at 33.918 ms (-0.20%, 5/7 pairs faster). 0/7 selected process means cleared the 33.5 ms CPU target.

## Prespecified integrated release scorecard

| Shape | Cores | SDK ms | Peak MiB | M5 CPU / Metal ms | CPU / Metal gain | Blocks at 10× CPU |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 96 | 8.254 | 48 | 37 / 24 | 4.48× / 2.91× | 0/5 |
| 2x2 | 48 | 12.980 | 102 | 59 / 30 | 4.55× / 2.31× | 0/5 |
| 4x2 | 48 | 17.190 | 251 | 109 / 48 | 6.34× / 2.79× | 0/5 |
| 8x2 | 48 | 23.657 | 431 | 189 / 74 | 7.99× / 3.13× | 0/5 |
| 12x2 | 48 | 30.740 | 611 | 284 / 101 | 9.24× / 3.29× | 0/5 |
| 16x2 | 64 | 34.494 | 740 | 335 / 124 | 9.71× / 3.59× | 0/5 |
| 24x2 | 64 | 46.269 | 1163 | 528 / 176 | 11.41× / 3.80× | 5/5 |
| merge-24x1 | 64 | 48.182 | 980 | 507 / 175 | 10.52× / 3.63× | 5/5 |

[Caller verification](caller.md) · [Complete screen](screen.md) · [Stage traces](stages.md) · [Protocol](artifacts/protocol.md) · [Individual timings](artifacts/processes.json.gz)
