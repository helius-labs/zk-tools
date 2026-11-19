# Where time goes in verified Ryzen proofs

Both shapes used 32 hardware threads on a Ryzen 9950X. Five shuffled blocks compared plain and traced copies of the same executable. Each process timed 21 requests after three warmups. The 1×2 configuration used solver task packing and claiming. The 12×2 configuration used compact Poseidon output addresses.

| Shape | Plain ms | Traced ms | Paired trace overhead | Within ±5% |
|---|---:|---:|---:|---|
| 1x2 | 7.229 | 7.361 | +1.24% | yes |
| 12x2 | 32.905 | 33.338 | +1.29% | yes |

Median observed wall time in milliseconds across 105 traced requests per shape. The first three rows partition each request. For 1×2 the boundary is canonical core return. On 12×2 the separate curve query function returns after H and G2 complete. The canonical core symbol had no calls on that path, so the table uses curve query return as its boundary. Their separate medians can differ slightly from the median total. Later rows can overlap. A dash indicates that the instrumented symbol did not run. Magnitudes for a shape outside the ±5% calibration range describe the traced execution only.

| Interval | 1×2 | 12×2 |
|---|---:|---:|
| SDK entry to FFT return | 1.719 | 13.901 |
| FFT return to phase boundary | 4.962 | 18.542 |
| Phase boundary to SDK return | 0.610 | 0.797 |
| Witness solver | 0.842 | 2.753 |
| FFT product | 0.554 | 9.404 |
| Curve query function | — | 18.529 |
| Ordinary H query | — | 16.564 |
| Union of observed G2 calls | 4.636 | 4.503 |
| Concurrent H and G2 time | — | 3.914 |
| Last observed H or G2 return to phase boundary | — | 0.012 |
| Final exponentiation | 0.227 | 0.247 |

The H probe covers the ordinary Montgomery G1 function. The G2 measurement takes the union of intervals from the three named G2 entry points. Unobserved routes and work on concurrent threads limit attribution to individual kernels.

[Protocol](artifacts/protocol.md) · [Function names and offsets](artifacts/sites.json) · [Coverage and medians](artifacts/stages.json) · [Per-request intervals](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
