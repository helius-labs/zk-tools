# Where the extra C7a workers spend CPU time

Each sample is linked to a worker using the Linux thread ID captured inside the benchmark process. The worker CPU masks were identical before and after each run. Three original trace processes recorded 11 verified SDK requests each per shape and width. Sampling at 500 Hz per running thread gives an estimate of CPU time. The CPUs column groups main-pool workers by their pinned logical CPU IDs. Other threads include the caller and auxiliary pools when present. The hardware exposed 96 physical cores and one thread per core. The three groups of 32 describe worker placement only.

## During the solver

CPU-ms per proof is summed over the indicated workers. Interpreter samples fall inside named interpreter functions. Samples inside their callees can appear under other. The final column is the share of each group’s sampled CPU time in Crossbeam.

| Shape | Workers | CPU IDs | Total CPU-ms | Crossbeam CPU-ms | Interpreter CPU-ms | Other Rayon CPU-ms | Other CPU-ms | Crossbeam share |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | 0–31 | 123.2 | 80.4 | 33.2 | 4.6 | 5.1 | 65.2% |
| 16x2 | 64 | 32–63 | 108.2 | 68.3 | 33.1 | 3.2 | 3.6 | 63.1% |
| 16x2 | 64 | other threads | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — |
| 16x2 | 96 | 0–31 | 153.8 | 118.8 | 24.8 | 5.2 | 5.0 | 77.3% |
| 16x2 | 96 | 32–63 | 143.6 | 114.2 | 22.7 | 3.3 | 3.5 | 79.5% |
| 16x2 | 96 | 64–95 | 135.2 | 104.9 | 23.7 | 3.8 | 2.7 | 77.6% |
| 16x2 | 96 | other threads | 0.1 | 0.0 | 0.0 | 0.0 | 0.1 | 0.0% |
| merge-24x1 | 64 | 0–31 | 0.5 | 0.3 | 0.0 | 0.1 | 0.2 | 55.6% |
| merge-24x1 | 64 | 32–63 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — |
| merge-24x1 | 64 | other threads | 138.4 | 37.8 | 86.8 | 6.0 | 7.8 | 27.3% |
| merge-24x1 | 96 | 0–31 | 1.0 | 1.0 | 0.0 | 0.0 | 0.0 | 100.0% |
| merge-24x1 | 96 | 32–63 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — |
| merge-24x1 | 96 | 64–95 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — |
| merge-24x1 | 96 | other threads | 215.2 | 97.5 | 95.6 | 9.7 | 12.3 | 45.3% |

The original 64 workers account for 84.3 of the 189.2 additional sampled Crossbeam CPU-ms per proof when the pool grows to 96. The 32 added workers account for the remaining 104.9. Interpreter samples shift across all three groups, reaching 24.8, 22.7 and 23.7 CPU-ms respectively. This supports testing whether the schedule exposes enough work to keep a larger team productive. The attribution does not measure how much wall time a schedule change would recover.

## Across phases on 16×2 at 96 cores

Each cell gives total CPU-ms per proof followed by the percentage of that CPU time in Crossbeam. Phase intervals can overlap.

| Interval | CPUs 0–31 | CPUs 32–63 | CPUs 64–95 | Other threads |
|---|---:|---:|---:|---:|
| interp | 153.8 / 77.3% | 143.6 / 79.5% | 135.2 / 77.6% | 0.1 / 0.0% |
| fft | 204.1 / 40.2% | 207.9 / 38.9% | 207.7 / 42.0% | 0.1 / 0.0% |
| msms | 483.5 / 25.0% | 423.9 / 30.0% | 412.4 / 30.2% | 0.0 / 0.0% |
| sdk | 886.8 / 38.9% | 815.9 / 42.0% | 795.5 / 42.6% | 0.2 / 0.0% |

## Crossbeam symbols sampled during the 16×2 solve

The pin column counts samples attributed to the pinning closure in with_handle, and advance counts samples attributed to Global::try_advance. These are symbol samples, so inlined callees can contribute.

| Cores | CPU IDs | Pin CPU-ms | Advance CPU-ms | Other Crossbeam CPU-ms |
|---:|---|---:|---:|---:|
| 64 | 0–31 | 53.5 | 23.2 | 3.6 |
| 64 | 32–63 | 50.3 | 14.9 | 3.1 |
| 96 | 0–31 | 69.7 | 44.4 | 4.7 |
| 96 | 32–63 | 74.1 | 36.2 | 3.9 |
| 96 | 64–95 | 79.5 | 21.5 | 3.9 |

The solver sample totals reconcile by category with the original five-interval analysis for all four shape/width combinations. All twelve archived exports passed independent Gnark verification again. These are the same requests and proofs as in experiment 363.

[Per-worker counts](artifacts/worker-samples.json.gz) · [Input hashes](artifacts/inputs.json) · [Original solver profiles](../2026-11-18-wave-claims373/quintiles.md)
