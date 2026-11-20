# Prefetching coordinates in borrowed G2 batches on Ryzen

The borrowed B2 kernel reads X coordinates for the forward denominator pass, then reads both coordinates for affine completion in reverse order. The candidate issues L1 prefetch hints for the bucket point and incoming point one or two full eight-lane groups ahead in each pass. The forward pass visits X only. The reverse pass covers both X and Y. Incomplete groups and the scalar tail keep the ordinary path. Both distances use the same field operations, bucket order and key storage. The hints require no additional point buffer or table.

The native checks compare the exact affine coordinates for both prefetch distances against the ordinary implementation and retain the independent scalar-addition checks. They exercise short batches, tails, mixed signs and scratch reuse. Route probes confirm both distances on every input and run the ordinary G2 path with borrowed prefix records disabled.

All 207 exported proofs independently verified. Ryzen uses 24 main and eight frontend logical CPUs. On 4×2 every timed mode uses the grouped Phi option so that the borrowed route is comparable. Eight blocks on each input time 17 proofs after three primes. Advancement requires a lower paired median and six wins against both Zopro controls on both inputs. AWS authentication was expired before this trial.

- 1×2 paired changes against ordinary were `one` +0.28% / +0.85% and `two` +0.29% / -0.27%. Neither profile qualified.
- 2×2 paired changes against ordinary were `one` +0.41% / +0.30% and `two` +0.22% / +0.02%. Neither profile qualified.
- 4×2 paired changes against ordinary were `one` +0.79% / +0.10% and `two` +1.10% / +0.31%. Neither profile qualified.

| Shape | Input | Profile | Proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.889 | +0.00% (0/8) | -1.80% (7/8) | 57.8 |
| 1x2 | catalog | ordinary | 7.045 | +1.83% (1/8) | +0.00% (0/8) | 58.0 |
| 1x2 | catalog | one | 7.011 | +2.17% (0/8) | +0.28% (3/8) | 57.8 |
| 1x2 | catalog | two | 7.017 | +2.04% (0/8) | +0.29% (3/8) | 58.2 |
| 1x2 | heldout | release | 6.777 | +0.00% (0/8) | -1.71% (8/8) | 58.2 |
| 1x2 | heldout | ordinary | 6.929 | +1.74% (0/8) | +0.00% (0/8) | 58.2 |
| 1x2 | heldout | one | 6.929 | +2.33% (0/8) | +0.85% (2/8) | 58.8 |
| 1x2 | heldout | two | 6.925 | +2.04% (1/8) | -0.27% (5/8) | 58.2 |
| 2x2 | catalog | release | 9.505 | +0.00% (0/8) | -0.65% (6/8) | 98.8 |
| 2x2 | catalog | ordinary | 9.558 | +0.65% (2/8) | +0.00% (0/8) | 96.2 |
| 2x2 | catalog | one | 9.589 | +0.85% (1/8) | +0.41% (2/8) | 100.0 |
| 2x2 | catalog | two | 9.575 | +0.73% (2/8) | +0.22% (2/8) | 94.5 |
| 2x2 | heldout | release | 9.472 | +0.00% (0/8) | -0.95% (5/8) | 96.0 |
| 2x2 | heldout | ordinary | 9.575 | +0.96% (3/8) | +0.00% (0/8) | 97.8 |
| 2x2 | heldout | one | 9.584 | +0.82% (0/8) | +0.30% (2/8) | 94.0 |
| 2x2 | heldout | two | 9.576 | +0.97% (0/8) | +0.02% (4/8) | 96.0 |
| 4x2 | catalog | release | 13.194 | +0.00% (0/8) | -1.45% (8/8) | 210.5 |
| 4x2 | catalog | ordinary | 13.365 | +1.47% (0/8) | +0.00% (0/8) | 217.0 |
| 4x2 | catalog | one | 13.411 | +2.00% (0/8) | +0.79% (2/8) | 213.2 |
| 4x2 | catalog | two | 13.442 | +2.16% (1/8) | +1.10% (3/8) | 205.8 |
| 4x2 | heldout | release | 13.268 | +0.00% (0/8) | -1.52% (6/8) | 221.2 |
| 4x2 | heldout | ordinary | 13.434 | +1.55% (2/8) | +0.00% (0/8) | 211.0 |
| 4x2 | heldout | one | 13.450 | +1.62% (1/8) | +0.10% (4/8) | 203.5 |
| 4x2 | heldout | two | 13.421 | +1.47% (0/8) | +0.31% (4/8) | 211.8 |

[Source](artifacts/source.patch) · [SDK samples and route traces](artifacts/samples.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md) reaches 10× M5 CPU on **5/8** original requests.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.858 | 37 / 24 | 4.71× / 3.05× |
| 2×2 | 12.029 | 59 / 30 | 4.90× / 2.49× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |
