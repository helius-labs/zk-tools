# Independent solver groups across adjacent epoch pairs on Ryzen

Parent tasks sharing a wire dependency inside two adjacent epochs belong to the same component. Components execute in parallel and join before the next pair. Each component runs its left parents before its right parents, with the original inner waves and arithmetic. The key retains parent indices and group boundaries. The all setting groups every adjacent pair. The gain setting groups only pairs where the modeled span falls. The release, disabled and two candidate arms all have solver task packing enabled.

| Workers | Mode | Release ms | Disabled ms | Candidate ms | Paired vs release (wins / 7) | Paired vs disabled (wins / 7) | CPU-ms release / candidate | Peak MiB release / candidate | Retained group bytes | M5 CPU / Metal speedup |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16 | all | 44.726 | 44.678 | 44.595 | -0.56% (4) | -0.51% (5) | 660.2 / 660.2 | 636.5 / 619.0 | 5376 | 7.51× / 2.78× |
| 16 | gain | 44.726 | 44.678 | 44.803 | -0.09% (4) | -0.09% (5) | 660.2 / 661.2 | 636.5 / 639.2 | 4224 | 7.48× / 2.77× |
| 32 | all | 40.570 | 40.559 | 40.415 | -0.48% (5) | +0.37% (3) | 1169.4 / 1159.8 | 791.2 / 776.2 | 5376 | 8.29× / 3.07× |
| 32 | gain | 40.570 | 40.559 | 40.419 | -0.87% (5) | +0.17% (3) | 1169.4 / 1163.9 | 791.2 / 780.2 | 4224 | 8.29× / 3.07× |

Neither grouping policy met the prespecified criterion at either width. At 32 workers the all-pairs median was 40.415 ms against 40.559 ms disabled, yet it won only three of seven matched blocks against disabled. At 16 workers it won five against disabled and four against the release. The solver retained 5,376 extra bytes with all pairs. The experiment leaves the integrated schedule unchanged. All 150 native core tests passed in the build and all 29 solver tests passed with pairing and packing requested.

Seven fresh blocks per worker count each timed 21 verified 16×2 SDK requests after three warmups. The 16-worker rows used physical cores, and the 32-worker rows used both SMT threads on the same 16 cores. Each of the eight supplied shapes exported a verified proof with the all setting. A zero optional preparation allowance also produced a verified proof through the fallback.

[Protocol](artifacts/protocol.md) · [Decision](artifacts/decision.json) · [Process samples](artifacts/processes.json.gz) · [Implementation](artifacts/source.patch.gz) · [Verification](artifacts/verification.json) · [Span model](../2026-11-19-solver-components401/README.md)
