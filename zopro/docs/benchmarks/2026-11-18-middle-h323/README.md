# Direct H queries for medium Euler images

The canonical variant skips scalar grouping for H and uses the existing canonical raw MSM. The Montgomery variant also skips conversion of each field element by treating its Montgomery words as an integer and correcting the resulting group element once. Eligibility is 32,768 through 262,143 terms. Each proof uses 16 physical Ryzen workers, with seven independent randomized processes per shape and setting, three primes and 21 timed full SDK requests.

| Shape | Setting | Median ms | Paired vs off | Wins / 7 | CPU ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 2x2 | reference | 11.954 | +1.64% | 2 | 150 | 180 |
| 2x2 | off | 11.798 | +0.00% | — | 149 | 173 |
| 2x2 | canonical | 11.797 | +0.22% | 3 | 149 | 171 |
| 2x2 | montgomery | 11.927 | -0.26% | 4 | 149 | 172 |
| 4x2 | reference | 16.984 | -0.76% | 4 | 237 | 228 |
| 4x2 | off | 17.131 | +0.00% | — | 239 | 234 |
| 4x2 | canonical | 17.110 | +0.62% | 3 | 237 | 232 |
| 4x2 | montgomery | 16.895 | -0.64% | 4 | 235 | 224 |
| 8x2 | reference | 26.728 | +1.43% | 2 | 392 | 384 |
| 8x2 | off | 26.543 | +0.00% | — | 388 | 383 |
| 8x2 | canonical | 26.987 | +1.22% | 2 | 391 | 380 |
| 8x2 | montgomery | 27.118 | +2.91% | 1 | 390 | 391 |

Each route probe recorded one call per enabled eligible proof and zero for disabled or ineligible 12×2. The generator oracle covers repeated scalars, full-width scalars, zeros and identity bases. All 96 proof exports passed independent verification. The patch applies to experiment 317.
