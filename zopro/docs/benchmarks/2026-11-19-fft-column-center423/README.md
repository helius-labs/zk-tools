# Combining FFT input packing and Euler weighting on 12×2

All three executables or modes used the qualified 32-thread Ryzen profile with the first inverse stages packed directly from the input. The candidate also performs the Euler weighting inside each local transform before the outer forward stages. Each process warmed up three times and timed 21 complete SDK requests. Percentages use seven paired blocks on the original request.

| Configuration | Median ms | Median CPU-ms | Peak MiB | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|
| Integrated release | 32.558 | 905.9 | 679.8 | 8.72× / 3.10× |
| Candidate, local weighting disabled | 32.736 | 907.0 | 675.8 | 8.68× / 3.09× |
| Candidate, local weighting enabled | 32.203 | 885.9 | 680.8 | 8.82× / 3.14× |

Against the identical disabled build, the paired change was -2.30% with 7/7 wins. Against the release, it was -1.33% with 7/7 wins. The candidate meets the criterion for a varied-witness confirmation.

A separate trace recorded three combined-transform entries on three timed enabled proofs and zero on the disabled arm. The arithmetic test compared the combined result with separate transforms at three domain sizes. All 23 exported proofs were independently verified.

[Protocol](artifacts/protocol.md) · [Prototype patch](artifacts/source.patch) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Trace symbol](artifacts/sites.json)
