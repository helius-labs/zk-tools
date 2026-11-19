# Signed H windows across eight 12×2 transfer witnesses

A 16-physical-core Ryzen process verified each complete SDK request. Each witness changes the blinding seed and redistributes value between the two outputs. The fixture generator recalculated commitments and the public input, and Gnark independently produced the expected public bytes. All three H configurations use compact solver outputs. Each witness has three randomized blocks with 21 timed requests after three primes.

| Witness | Ordinary ms | Signed one range ms | Signed two ranges ms | Paired vs ordinary | Paired vs two ranges |
|---:|---:|---:|---:|---:|---:|
| 1 | 38.420 | 36.975 | 38.350 | -3.74% | -3.50% |
| 2 | 38.482 | 36.906 | 38.149 | -3.54% | -2.44% |
| 3 | 38.341 | 36.957 | 38.220 | -3.38% | -3.30% |
| 4 | 38.247 | 37.253 | 37.996 | -3.19% | -1.96% |
| 5 | 38.575 | 37.502 | 38.031 | -3.47% | -1.69% |
| 6 | 38.773 | 37.884 | 38.059 | -2.39% | -0.54% |
| 7 | 38.407 | 37.384 | 38.100 | -3.31% | -1.75% |
| 8 | 38.219 | 37.080 | 38.183 | -3.57% | -1.96% |

| H layout | Median ms across witnesses | Median CPU-ms | Maximum peak MiB | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|
| ordinary | 38.414 | 567.5 | 541.5 | 7.39× / 2.63× |
| 18s1 | 37.166 | 543.9 | 546.2 | 7.64× / 2.72× |
| 18s2 | 38.125 | 559.1 | 553.2 | 7.45× / 2.65× |

The single-range setting improved the median paired result on 8/8 witnesses against ordinary H and 8/8 against the two-range setting. It qualified for the 16-core Ryzen profile. The eight-witness comparison uses a single shape.

[Protocol](artifacts/protocol.md) · [Per-witness input hashes](artifacts/inputs.json) · [Process samples](artifacts/processes.json.gz) · [Decision](artifacts/decision.json) · [Verification](artifacts/verification.json)
