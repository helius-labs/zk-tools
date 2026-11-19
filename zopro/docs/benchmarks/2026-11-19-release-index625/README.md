# Indexed scalar grouping in the ordinary SDK

**Every latency pair ran the parent first.** The runner rotated two variants and reversed them on odd blocks, canceling the rotation. All 156 proofs verified, and the candidate showed lower maximum RSS on all 13 inputs. For latency, run position and implementation are confounded. The follow-up uses the same binaries and inputs with equal numbers of parent-first and indexed-first blocks. The earlier four-input experimental comparison independently randomized its process order. [Experimental measurements](../2026-11-19-indexed-transfer622/README.md)

Times are medians of five process means. Each process warms three requests and measures eleven. Differences use paired blocks. Peak RSS is the largest lifetime peak among the five latency processes. Each variant and input also has an independent CPU profile.

| Shape | Request | Parent / indexed ms | Indexed versus parent | Peak parent / indexed MiB | Indexed vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|
| 1x2 | Original | 7.024 / 7.090 | +1.30% (0/5) | 62.2 / 60.0 | 5.22× / 3.39× |
| 2x2 | Original | 9.697 / 9.628 | -0.86% (4/5) | 112.8 / 104.5 | 6.13× / 3.12× |
| 4x2 | Original | 15.457 / 15.205 | -1.32% (5/5) | 258.2 / 252.5 | 7.17× / 3.16× |
| 8x2 | Original | 24.351 / 24.230 | -0.61% (3/5) | 435.0 / 427.8 | 7.80× / 3.05× |
| 12x2 | Original | 31.105 / 30.667 | -1.48% (4/5) | 575.5 / 510.7 | 9.26× / 3.29× |
| 16x2 | Original | 39.787 / 39.224 | -1.94% (4/5) | 665.6 / 640.5 | 8.54× / 3.16× |
| 24x2 | Original | 65.420 / 64.003 | -4.43% (4/5) | 1025.8 / 980.8 | 8.25× / 2.75× |
| merge-24x1 | Original | 63.780 / 63.138 | -0.73% (5/5) | 1063.0 / 1008.2 | 8.03× / 2.77× |
| merge-54x1 | Original | 156.089 / 152.357 | -2.34% (4/5) | 1790.2 / 1760.3 | — |
| 4x2 | 153 | 15.651 / 15.647 | -0.03% (3/5) | 273.0 / 244.8 | — |
| 4x2 | 154 | 15.642 / 15.323 | -1.92% (5/5) | 254.0 / 251.2 | — |
| 4x2 | 156 | 16.094 / 15.467 | -2.45% (5/5) | 262.2 / 260.2 | — |
| 12x2 | 139 | 48.206 / 45.333 | -6.07% (5/5) | 958.0 / 799.2 | — |

All 156 exports independently verified. [Protocol](protocol.md) · [Samples and profiles](artifacts/rows.json.gz) · [Exact xtask configurations](artifacts/configs.json.gz) · [Build evidence](artifacts/build.json) · [Verification](artifacts/verification.json)
