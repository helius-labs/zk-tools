# Extending prepared groups to small and offset-corrected scalars

Previous groups used varying wide scalars on eight public adjacent-leaf training requests. This experiment keeps those groups, subtracts the existing public wire offsets in the training vectors, then groups the remaining B2 rows by their eight exact residual scalar values. New groups contain 3 to 128 members. Points map into A and K as before. Each proof validates the sampled plan and corrects all mismatches on the compact route. No user witness is retained.

Five randomized blocks compare the 32-worker release with the two grouped configurations on the same 28-worker placements. Each process uses three primes and eleven timed calls. Request 0 is the original, 59 and 63 use adjacent leaves, and 140 and 144 have scattered leaves.

| Request | Release 32 / original groups 28 / extended groups 28, ms | Extended vs original groups | Extended vs release | Peak MiB original / extended groups |
|---|---:|---:|---:|---:|
| Original | 31.755 / 29.516 / 32.326 | +10.11% (0/5) | +0.80% (0/5) | 615.3 / 503.5 |
| 59 | 31.190 / 29.522 / 32.363 | +8.73% (0/5) | +3.84% (1/5) | 610.2 / 510.1 |
| 63 | 31.523 / 29.720 / 32.554 | +9.37% (0/5) | +3.28% (0/5) | 615.8 / 502.5 |
| 140 | 47.702 / 46.814 / 47.008 | +0.26% (2/5) | -1.12% (4/5) | 972.3 / 964.0 |
| 144 | 47.718 / 47.819 / 46.982 | -2.04% (4/5) | -1.54% (4/5) | 1022.8 / 965.1 |

All 80 exported proofs passed independent Gnark verification, including five route probes. [Measurements](artifacts/processes.json.gz) · [Group dimensions](artifacts/layouts.json) · [Route probes](artifacts/probes.json) · [Verification](artifacts/verification.json)
