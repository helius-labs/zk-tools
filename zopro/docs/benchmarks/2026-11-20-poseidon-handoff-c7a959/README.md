# Reusing completed Poseidon tails on C7a 4×2

Replay lowered the paired median against the ordinary path in all four comparisons. Neither solver width met the required nine wins out of twelve against the frozen build on both inputs. The 4×2 record remains 14.457 ms.

The run used the [same executable as the Ryzen trial](../2026-11-20-poseidon-handoff-ryzen958/README.md). The c7a.24xlarge used 64 main and 32 frontend workers, with either one or two eight-core L3 groups for solving. Each input has twelve counterbalanced blocks with three warm-up and seventeen timed proofs per process. The ordinary path from the same executable calibrated within 0.94% of the frozen build in every comparison. Changes are paired medians of process means.

| Solver cores | Input | Frozen ms | Ordinary ms | Replay ms | Replay vs ordinary | Replay vs frozen | Replay peak MiB |
|---:|---|---:|---:|---:|---:|---:|---:|
| 8 | catalog | 14.595 | 14.617 | 14.484 | −0.91% (8/12) | −0.79% (8/12) | 215.4 |
| 8 | heldout | 14.695 | 14.736 | 14.653 | −0.47% (11/12) | −0.35% (7/12) | 228.9 |
| 16 | catalog | 14.486 | 14.670 | 14.473 | −0.92% (9/12) | −0.66% (7/12) | 211.7 |
| 16 | heldout | 14.573 | 14.612 | 14.558 | −0.53% (8/12) | −0.24% (7/12) | 215.4 |

Eight-core replay was 0.21% and 0.38% slower than the matched frozen sixteen-core profile on the original and held-out requests. Separate probes measured a median of 10 reused tails per proof with eight solver cores and 13.5–18.5 with sixteen, below the 36 observed at seven Ryzen workers. The observed solve time at sixteen cores was 2.652/2.599 ms with replay and 2.708/2.536 ms with ordinary execution.

Gnark independently verified all 186 exported proofs after retrieval. The ten route probes compared all solver buffers against ordinary execution, including the late-source and deliberately mismatched cases and the materialized C buffer. The temporary instance and bucket were removed. The region has zero active tagged benchmark instances or associated volumes.

## Eight supplied M5 examples

| Shape | Best verified C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.736 | 37 / 24 | 4.78× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.457 | 109 / 48 | 7.54× / 3.32× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Earlier scorecard](../2026-11-20-eager-h-canonical-c7a954/README.md#eight-supplied-m5-references) · [Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup](artifacts/cleanup.json)
