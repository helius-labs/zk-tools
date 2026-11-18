# Integrated sequential G2 scalar splitting

The explicit serial setting applies only after four-range G2 passes its existing worker and memory checks. The same point-range and weighted reductions still run in parallel. All 143 native tests passed. Separate timed probes checked the range count and whether parallel scalar splitting executed on the integrated binary. The AWS figures in the linked study refer to the experimental executable.

| Shape | Workers | Setting | Median ms | Peak MiB |
|---|---:|---|---:|---:|
| 1x2 | 16 | retained | 8.820 | 58 |
| 1x2 | 16 | four | 8.824 | 59 |
| 1x2 | 16 | qualified-serial | 8.901 | 59 |
| 1x2 | 16 | serial | 8.834 | 58 |
| 1x2 | 32 | retained | 7.870 | 64 |
| 1x2 | 32 | four | 7.896 | 65 |
| 1x2 | 32 | qualified-serial | 8.036 | 64 |
| 1x2 | 32 | serial | 8.048 | 68 |
| 2x2 | 16 | retained | 12.173 | 104 |
| 2x2 | 16 | four | 12.016 | 105 |
| 2x2 | 16 | qualified-serial | 12.102 | 108 |
| 2x2 | 16 | serial | 12.204 | 104 |
| 2x2 | 32 | retained | 10.954 | 109 |
| 2x2 | 32 | four | 10.897 | 114 |
| 2x2 | 32 | qualified-serial | 11.128 | 113 |
| 2x2 | 32 | serial | 11.078 | 112 |

All 84 exported proofs verified against their public witnesses.

[C7a candidate and latency tails](../2026-11-18-range-start345/README.md) · [Calibration](artifacts/calibration.json) · [Timings](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
