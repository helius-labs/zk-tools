# Prepared point pairs on canonical 2×2 queries

Eight public 2×2 training fixtures identify B2 rows with equal canonical residual scalar vectors after subtracting the existing public offsets. Groups contain two to 128 members. The same wire indices produce A and private K plans. The query path accepts canonical words directly. It uses full scalar equality and computes every necessary field difference when the sampled plan qualifies. The bounded-memory fallback and independent Gnark verification were exercised separately. Six held-out transactions and the original used the same 32 Ryzen workers with twelve frontend workers on one L3. Five randomized blocks compare the release, the candidate with groups disabled, with B2 only, and with all three plans. Each process primed three proofs and timed 31.

| Request | Release / off / B2 / all, ms | All vs off | All vs release | Peak MiB release / all |
|---|---:|---:|---:|---:|
| Original | 9.581 / 9.628 / 9.802 / 10.259 | +5.97% (0/5) | +6.42% (0/5) | 114.8 / 113.2 |
| 41 | 9.611 / 9.562 / 9.755 / 10.213 | +6.82% (0/5) | +6.51% (0/5) | 114.2 / 116.0 |
| 42 | 9.604 / 9.587 / 9.705 / 10.131 | +6.49% (0/5) | +5.96% (0/5) | 114.2 / 113.2 |
| 43 | 9.629 / 9.632 / 9.594 / 10.114 | +6.66% (0/5) | +6.90% (0/5) | 112.5 / 115.0 |
| 44 | 9.565 / 9.585 / 9.817 / 10.324 | +5.88% (0/5) | +7.67% (0/5) | 112.2 / 112.5 |
| 47 | 9.568 / 9.571 / 9.737 / 10.144 | +6.94% (0/5) | +6.02% (0/5) | 114.2 / 113.8 |
| 48 | 9.620 / 9.614 / 9.799 / 10.280 | +6.93% (0/5) | +5.44% (0/5) | 122.8 / 114.8 |

All 144 exported proofs verified. [Measurements](artifacts/processes.json.gz) · [Checked routes](artifacts/probes.json) · [Source changes from sampled groups](artifacts/incremental.patch) · [Verification](artifacts/verification.json)
