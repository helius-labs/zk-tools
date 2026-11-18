# C7a replication of direct FFT input and inline G2 grouping

On the primary 24×2 at 48 cores, direct input took 52.225 ms, disabled took 53.423 ms and accepted took 53.617 ms. The median paired change was -0.36% with 5/7 wins and control calibration of -0.26%. The C7a screen did not pass.
The independent inline grouping candidate measured 10.680 ms on 96-core 1×2 against 12.273 disabled and 12.541 accepted. Median paired change -13.11%, 7/7 wins, disabled/accepted calibration -2.30%. The screen passed.


| Shape | Physical cores | Mode | Median ms | Paired change vs off | Wins | Median CPU ms | Max RSS MiB | M5 CPU / Metal speedup |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| 12x2 | 48 | accepted | 32.650 | -1.49% | 4/7 | 1259 | 649 | 8.70× / 3.09× |
| 12x2 | 48 | direct | 32.690 | -0.47% | 4/7 | 1243 | 655 | 8.69× / 3.09× |
| 12x2 | 48 | off | 33.003 | +0.00% | 0/7 | 1262 | 654 | 8.61× / 3.06× |
| 12x2 | 48 | padded | 32.576 | -0.89% | 7/7 | 1240 | 656 | 8.72× / 3.10× |
| 16x2 | 48 | accepted | 37.220 | -0.14% | 4/7 | 1435 | 759 | 9.00× / 3.33× |
| 16x2 | 48 | direct | 36.435 | -1.99% | 6/7 | 1413 | 794 | 9.19× / 3.40× |
| 16x2 | 48 | off | 37.185 | +0.00% | 0/7 | 1440 | 785 | 9.01× / 3.33× |
| 16x2 | 48 | padded | 36.880 | -0.82% | 4/7 | 1408 | 801 | 9.08× / 3.36× |
| 1x2 | 64 | accepted | 11.118 | -0.01% | 4/7 | 402 | 120 | 3.33× / 2.16× |
| 1x2 | 64 | group-off | 11.046 | +0.00% | 0/7 | 387 | 114 | 3.35× / 2.17× |
| 1x2 | 64 | group-on | 9.430 | -14.24% | 7/7 | 373 | 64 | 3.92× / 2.54× |
| 1x2 | 96 | accepted | 12.541 | +2.35% | 1/7 | 692 | 112 | 2.95× / 1.91× |
| 1x2 | 96 | group-off | 12.273 | +0.00% | 0/7 | 627 | 112 | 3.01× / 1.96× |
| 1x2 | 96 | group-on | 10.680 | -13.11% | 7/7 | 621 | 58 | 3.46× / 2.25× |
| 24x2 | 48 | accepted | 53.617 | +0.26% | 3/7 | 2118 | 1258 | 9.85× / 3.28× |
| 24x2 | 48 | direct | 52.225 | -0.36% | 5/7 | 2069 | 1221 | 10.11× / 3.37× |
| 24x2 | 48 | off | 53.423 | +0.00% | 0/7 | 2125 | 1205 | 9.88× / 3.29× |
| 24x2 | 48 | padded | 52.321 | -3.81% | 5/7 | 2056 | 1262 | 10.09× / 3.36× |
| 24x2 | 96 | accepted | 57.599 | +2.60% | 2/7 | 3769 | 1272 | 9.17× / 3.06× |
| 24x2 | 96 | direct | 56.186 | -0.39% | 4/7 | 3593 | 1267 | 9.40× / 3.13× |
| 24x2 | 96 | off | 56.626 | +0.00% | 0/7 | 3643 | 1270 | 9.32× / 3.11× |
| 24x2 | 96 | padded | 56.034 | -1.10% | 6/7 | 3546 | 1281 | 9.42× / 3.14× |
| 2x2 | 48 | accepted | 14.186 | +0.38% | 2/7 | 412 | 300 | 4.16× / 2.11× |
| 2x2 | 48 | group-off | 14.105 | +0.00% | 0/7 | 409 | 300 | 4.18× / 2.13× |
| 2x2 | 48 | group-on | 14.071 | -0.21% | 4/7 | 410 | 289 | 4.19× / 2.13× |
| merge-24x1 | 48 | accepted | 55.913 | -0.96% | 4/7 | 2054 | 1082 | 9.07× / 3.13× |
| merge-24x1 | 48 | direct | 55.087 | -2.02% | 6/7 | 2022 | 1117 | 9.20× / 3.18× |
| merge-24x1 | 48 | off | 56.053 | +0.00% | 0/7 | 2060 | 1168 | 9.04× / 3.12× |
| merge-24x1 | 48 | padded | 55.238 | -1.99% | 6/7 | 2035 | 1110 | 9.18× / 3.17× |
| merge-54x1 | 96 | accepted | 123.186 | +0.59% | 2/7 | 6418 | 2011 | — |
| merge-54x1 | 96 | direct | 123.720 | +1.57% | 3/7 | 6143 | 1929 | — |
| merge-54x1 | 96 | off | 122.298 | +0.00% | 0/7 | 6227 | 1937 | — |
| merge-54x1 | 96 | padded | 122.260 | -0.03% | 4/7 | 6096 | 1930 | — |

Seven randomized blocks, three primes and 21 timed requests per large-shape process except merge54, which used 11. The two small shapes used 101 requests each. Every process exported a proof verified independently on the host and after retrieval. FFT and grouping candidates each have a same-binary disabled control.

[Source and local comparison](../2026-11-18-fft-input302/README.md) · [Runs](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
