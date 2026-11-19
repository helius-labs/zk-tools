# Time the packed Euler transform inside complete proofs

Four blocks per shape and mode use the same candidate executable and placement as study 676, with packed Euler enabled. A separate byte-identical executable records Linux function entry and return timestamps. The SDK controls capture around its nine timed requests after three primes. For every observed proof the reader checks that the first input layer lies within the packed transform, which lies within the Euler product, which lies within the proof. The packed interval after subtracting the input layer includes the inner transforms and final forward columns. The last column compares that interval with the same block's ordinary schedule. Medians summarize process means. All 120 process exports independently verified against the original key and expected public inputs.

| Shape | Mode | Plain ms | Paired vs off | Paired trace effect | Euler product / packed transform / input layer ms | Post-input packed change ms (wins) |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | off | 7.291 | +0.00% (0/4) | +1.14% | 0.637 / 0.594 / 0.225 | +0.000 (0/4) |
| 1x2 | direct | 7.271 | -0.37% (2/4) | -0.35% | 0.650 / 0.609 / 0.240 | -0.002 (2/4) |
| 1x2 | four | 7.282 | +0.11% (2/4) | -0.42% | 0.624 / 0.583 / 0.230 | -0.018 (4/4) |
| 12x2 | off | 28.420 | +0.00% (0/4) | +1.30% | 8.946 / 8.117 / 3.123 | +0.000 (0/4) |
| 12x2 | direct | 28.531 | +0.92% (1/4) | -0.03% | 8.913 / 8.090 / 3.130 | -0.014 (2/4) |
| 12x2 | four | 28.424 | -0.09% (2/4) | +1.36% | 8.751 / 7.914 / 3.152 | -0.200 (3/4) |
| 24x2 | off | 59.551 | +0.00% (0/4) | -0.61% | 20.488 / 18.178 / 6.162 | +0.000 (0/4) |
| 24x2 | direct | 59.450 | -0.50% (2/4) | -0.83% | 20.461 / 18.149 / 6.093 | +0.054 (2/4) |
| 24x2 | four | 59.878 | +1.10% (2/4) | -0.45% | 20.336 / 18.007 / 6.045 | -0.038 (2/4) |
| merge-24x1 | off | 61.540 | +0.00% (0/4) | +0.53% | 20.196 / 18.101 / 5.883 | +0.000 (0/4) |
| merge-24x1 | direct | 62.471 | +2.21% (0/4) | +1.54% | 20.880 / 18.710 / 6.278 | +0.540 (0/4) |
| merge-24x1 | four | 61.485 | -1.93% (3/4) | +0.89% | 20.223 / 17.730 / 5.932 | +0.004 (2/4) |
| merge-54x1 | off | 153.013 | +0.00% (0/4) | -0.97% | 48.938 / 41.853 / 12.099 | +0.000 (0/4) |
| merge-54x1 | direct | 154.134 | +0.17% (1/4) | -0.62% | 48.633 / 41.666 / 11.946 | -0.184 (2/4) |
| merge-54x1 | four | 152.631 | +0.12% (2/4) | -0.19% | 48.176 / 41.238 / 11.951 | -0.684 (4/4) |

On merge54, four reduced the measured post-input packed interval on all four blocks by a paired median of 0.684 ms. The plain full-proof comparison split two wins to two losses. On merge24, the four-quad plain process won three of four pairs, yet the post-input interval split two to two with a +0.004 ms paired change. The first screen's whole-proof advantage on merge24 therefore cannot be attributed confidently to a shorter post-input transform. With the ordinary kernel, the merge54 input layer alone took 12.099 ms and the complete Euler product took 48.938 ms. All temporary Linux probes were removed.

[Candidate and exact configurations](../2026-11-19-quad-euler676/README.md) · [Symbols and entry points](artifacts/sites.json) · [Per-proof phase clocks](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Verification](artifacts/verification.json)
