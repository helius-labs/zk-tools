# Reusing Poseidon round outputs within one proof

The proof-local prototype matches the complete three-field state after the first ordinary round of recognized width-three Poseidon permutations. The fixed-size table uses the full state to check equality after indexing. A matching entry replays each S-box input, square, fourth power and fifth power into the destination task's own rows and wires. The remaining dense and sparse arithmetic can then be skipped. The proof releases the table at the end of the solver call. The largest trial configuration has 256 entries, with optional memory admission covering every entry's maximum trace.

First records on the first encounter and reuses completed traces. Second starts recording on a second encounter. Join allows concurrent workers to share one in-flight calculation. A slot collision with an unequal state computes normally. A differential test compared every output wire and A/B/C row to the ordinary interpreter for cache reuse, concurrent callers, forced collisions, compiled and ordinary addressing, both multiplication orientations, and omitted C rows.

Three randomized blocks per request each included three primes and eleven timed full proofs on 32 Ryzen workers. The release and disabled prototype are separate controls. The two varied requests per shape use scattered transfer leaves or populated merge trees. Eight additional route processes included zero optional preparation allowance. Route counters report the range across four calls of each instrumented process.

| Shape / request | Release / off / first / second / join ms | First / second / join vs off | Joined replay hits | Peak MiB release / join |
|---|---:|---:|---:|---:|
| 12x2 / original | 31.336 / 31.838 / 31.330 / 31.445 / 31.964 | -1.25% / -1.20% / +0.84% | 512–512 | 545.4 / 544.3 |
| 12x2 / 137 | 47.743 / 47.346 / 47.970 / 48.197 / 48.357 | +1.32% / +0.64% / +2.11% | 170–204 | 940.3 / 969.5 |
| 12x2 / 144 | 47.644 / 47.965 / 48.664 / 48.514 / 48.677 | +1.46% / +1.47% / +1.48% | 210–210 | 979.6 / 1007.4 |
| 24x2 / original | 64.179 / 64.415 / 65.766 / 63.627 / 63.796 | +0.81% / -0.42% / -0.96% | 1002–1002 | 1021.3 / 1019.2 |
| merge-24x1 / original | 63.540 / 63.233 / 63.287 / 64.895 / 63.922 | +0.09% / +2.63% / +1.09% | 1011–1011 | 1020.7 / 1011.0 |
| merge-24x1 / 129 | 66.625 / 65.574 / 66.910 / 66.456 / 65.606 | +2.41% / +1.35% / +0.42% | 731–736 | 1102.9 / 1064.2 |
| merge-24x1 / 136 | 72.704 / 73.319 / 74.395 / 72.927 / 73.226 | +1.28% / +0.54% / -0.13% | 670–673 | 1207.5 / 1171.0 |

All 113 exported proofs verified independently with Gnark. The split between fully completed and still-running duplicate permutations in the [state-count diagnostic](../2026-11-19-poseidon-reuse486/README.md) motivated the join comparison. These prototype results do not change the qualified profiles.

[Paired results and probes](artifacts/results.json) · [Full-proof samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Experimental implementation](artifacts/source.patch)
