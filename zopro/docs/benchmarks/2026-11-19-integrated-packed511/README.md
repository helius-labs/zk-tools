# Adaptive G1 groups with direct packed B2 extraction

Five configurations use the same binary and 32 Ryzen workers. Ordinary uses the original image. Linear adds adaptive A and K groups. Adaptive uses prepared groups for A, K and B2 on the ordinary image. Packed uses the mapped B2 section from experiment 506 and direct scalar extraction from 509. Integrated combines packed B2 with adaptive A and K groups. At 12×2 the broad A/K file is fixed across both requests and the packed B2 uses the wide groups used in 506. Four randomized blocks per request used three warmups and eleven timed proofs. Three more probes denied optional preparation memory and checked the exact dense fallback. Peak RSS includes setup.

| Shape | Request | Ordinary / linear / adaptive / packed / integrated ms | Integrated vs adaptive | Integrated vs ordinary | Peak MiB ordinary / adaptive / integrated |
|---|---|---:|---:|---:|---:|
| 8x2 | Original | 24.110 / 23.701 / 24.144 / 24.725 / 24.247 | +0.34% (2/4) | +1.27% (1/4) | 431.8 / 501.5 / 449.8 |
| 8x2 | 153 | 24.577 / 24.236 / 24.499 / 24.633 / 23.999 | -1.93% (4/4) | -2.29% (4/4) | 426.5 / 524.5 / 465.2 |
| 12x2 | Original | 32.006 / 30.666 / 30.884 / 33.631 / 34.507 | +12.00% (0/4) | +7.34% (1/4) | 555.3 / 525.5 / 570.5 |
| 12x2 | 141 | 47.382 / 47.202 / 48.602 / 51.622 / 51.623 | +5.91% (1/4) | +8.63% (0/4) | 953.3 / 975.0 / 983.6 |
| 24x2 | Original | 63.462 / 62.243 / 58.707 / 61.548 / 59.342 | +1.25% (2/4) | -6.77% (4/4) | 1009.0 / 1133.7 / 1081.1 |
| 24x2 | 97 | 64.787 / 61.517 / 58.120 / 60.942 / 58.337 | +0.43% (1/4) | -9.98% (4/4) | 1041.3 / 1141.8 / 1040.8 |
| 24x2 | 103 | 63.961 / 62.177 / 58.148 / 60.530 / 58.467 | +0.55% (2/4) | -9.35% (4/4) | 1039.1 / 1085.3 / 1049.8 |

All 150 exported proofs verified independently against the original key. [Process samples](artifacts/processes.json.gz) · [Execution routes](artifacts/probes.json) · [Incremental source](artifacts/incremental.patch) · [Verification](artifacts/verification.json)
