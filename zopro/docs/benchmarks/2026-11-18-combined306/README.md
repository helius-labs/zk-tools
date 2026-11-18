# Testing small- and large-circuit optimizations in combination

The native prover was built from the accepted source with the inline G2 diversity check, the admitted four-range G2 geometry, direct inverse FFT input and the previously measured dense H-window option. Each cell toggles features in the same executable. Small shapes used 32 logical Ryzen workers and 51 timed requests. Large shapes used 16 physical cores and 11 timed requests. Each of five fresh randomized blocks included an archived production comparison.

| Shape | Mode | Median ms | Paired vs disabled | Wins | Median CPU ms | Peak MiB | M5 CPU / Metal speedup |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x2 | accepted | 7.496 | +0.69% | 0/5 | 164 | 141 | 4.94× / 3.20× |
| 1x2 | off | 7.464 | +0.00% | 0/5 | 163 | 112 | 4.96× / 3.22× |
| 1x2 | group | 8.393 | +12.75% | 2/5 | 168 | 98 | 4.41× / 2.86× |
| 1x2 | split | 7.659 | +2.61% | 0/5 | 173 | 117 | 4.83× / 3.13× |
| 1x2 | group-split | 7.539 | +1.01% | 1/5 | 173 | 97 | 4.91× / 3.18× |
| 2x2 | accepted | 10.431 | -16.52% | 4/5 | 238 | 290 | 5.66× / 2.88× |
| 2x2 | off | 12.509 | +0.00% | 0/5 | 242 | 293 | 4.72× / 2.40× |
| 2x2 | group | 10.458 | -16.40% | 4/5 | 240 | 284 | 5.64× / 2.87× |
| 2x2 | split | 10.667 | -16.08% | 3/5 | 250 | 294 | 5.53× / 2.81× |
| 2x2 | group-split | 10.593 | -14.98% | 3/5 | 247 | 294 | 5.57× / 2.83× |
| 16x2 | accepted | 46.567 | -0.41% | 3/5 | 686 | 602 | 7.19× / 2.66× |
| 16x2 | off | 46.666 | +0.00% | 0/5 | 693 | 604 | 7.18× / 2.66× |
| 16x2 | direct | 46.135 | -0.40% | 4/5 | 683 | 615 | 7.26× / 2.69× |
| 16x2 | h13 | 45.655 | -2.17% | 4/5 | 682 | 580 | 7.34× / 2.72× |
| 16x2 | direct-h13 | 45.337 | -2.85% | 5/5 | 671 | 595 | 7.39× / 2.74× |
| 24x2 | accepted | 75.124 | -1.77% | 5/5 | 1132 | 987 | 7.03× / 2.34× |
| 24x2 | off | 76.503 | +0.00% | 0/5 | 1140 | 985 | 6.90× / 2.30× |
| 24x2 | direct | 73.903 | -3.36% | 5/5 | 1111 | 960 | 7.14× / 2.38× |
| 24x2 | h13 | 74.761 | -3.13% | 5/5 | 1130 | 984 | 7.06× / 2.35× |
| 24x2 | direct-h13 | 72.527 | -5.35% | 4/5 | 1089 | 969 | 7.28× / 2.43× |
| merge-24x1 | accepted | 73.858 | -0.67% | 4/5 | 1082 | 864 | 6.86× / 2.37× |
| merge-24x1 | off | 74.153 | +0.00% | 0/5 | 1094 | 860 | 6.84× / 2.36× |
| merge-24x1 | direct | 72.561 | -2.15% | 4/5 | 1059 | 868 | 6.99× / 2.41× |
| merge-24x1 | h13 | 73.121 | -1.73% | 5/5 | 1075 | 898 | 6.93× / 2.39× |
| merge-24x1 | direct-h13 | 71.259 | -4.24% | 5/5 | 1051 | 922 | 7.11× / 2.46× |

The combined local screen passed on 16x2, 24x2, merge-24x1. A pass required a lower median than each component and the accepted executable, a paired improvement of at least 2% over disabled in four of five blocks, and control calibration within 5%. All 125 process exports and two independent preflight proofs verified. The integrated native suite passed 144 tests.

[Source patch](artifacts/source.patch) · [Process data](artifacts/rows.json.gz) · [Checks](artifacts/checks.json) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
