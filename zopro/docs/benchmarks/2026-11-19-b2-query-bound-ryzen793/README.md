# Full-query B2 replay on Ryzen

The benchmark builds the B2 result normally on its first warmup. It reuses only that one curve query result, after comparing the full witness coefficients. Each timed call still solves the witness, computes the other queries and builds a fresh proof. A changed witness or denied optional reservation takes the ordinary route. One loaded immutable key and one benchmark process bound the lifetime of every replay. The smaller circuits use the exclusive G2 route. The 4×2 and 8×2 panels also replay only scalar classes and point sums to compare preparation against the remaining curve work.

Five matched blocks per input use the same 24 main and eight frontend logical CPUs. The release and same-build ordinary controls use the same per-shape options. On 8×2 these include Poseidon origins and serial class conversion. Each process discards three priming calls and times thirteen requests. All 164 exported proofs independently verified. The route checks confirm reuse for both inputs on all four shapes, an exact miss with a different warmup input, and fallback under a zero optional-memory allowance.

The original / held-out medians are below.

- 1×2 took 7.040 / 6.888 ms normally and 5.021 / 4.701 ms with the B2 result replayed.
- 2×2 took 9.860 / 9.815 ms normally and 7.196 / 7.218 ms with the B2 result replayed.
- 4×2 took 14.542 / 14.519 ms normally and 12.328 / 12.239 ms with the B2 result replayed.
- 8×2 took 22.450 / 22.508 ms normally and 18.383 / 18.517 ms with the B2 result replayed.

These fixed-request observations measure the net effect of removing a concurrent curve query, with the scalar comparison still in the timed path. The values do not update the [C7a scorecard](../2026-11-19-b2-repeat-bound-c7a791/README.md).

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.008 | -2.35% (4/5) | +0.00% (0/5) | 57.8 |
| 1×2 | catalog | ordinary | 7.040 | +0.00% (0/5) | +2.41% (1/5) | 57.8 |
| 1×2 | catalog | result | 5.021 | -27.86% (4/5) | -27.34% (4/5) | 54.2 |
| 1×2 | heldout | release | 6.854 | -1.26% (3/5) | +0.00% (0/5) | 57.2 |
| 1×2 | heldout | ordinary | 6.888 | +0.00% (0/5) | +1.28% (2/5) | 57.8 |
| 1×2 | heldout | result | 4.701 | -31.00% (5/5) | -31.36% (5/5) | 53.5 |
| 2×2 | catalog | release | 9.777 | -0.58% (5/5) | +0.00% (0/5) | 99.2 |
| 2×2 | catalog | ordinary | 9.860 | +0.00% (0/5) | +0.59% (0/5) | 99.0 |
| 2×2 | catalog | result | 7.196 | -27.02% (5/5) | -26.32% (5/5) | 102.5 |
| 2×2 | heldout | release | 9.826 | -0.20% (4/5) | +0.00% (0/5) | 103.2 |
| 2×2 | heldout | ordinary | 9.815 | +0.00% (0/5) | +0.20% (1/5) | 101.2 |
| 2×2 | heldout | result | 7.218 | -26.63% (5/5) | -26.54% (5/5) | 100.0 |
| 4×2 | catalog | release | 14.484 | -0.42% (3/5) | +0.00% (0/5) | 209.0 |
| 4×2 | catalog | ordinary | 14.542 | +0.00% (0/5) | +0.42% (2/5) | 211.0 |
| 4×2 | catalog | result | 12.328 | -15.06% (5/5) | -14.88% (5/5) | 150.2 |
| 4×2 | catalog | affine | 14.018 | -4.17% (5/5) | -3.76% (5/5) | 163.5 |
| 4×2 | heldout | release | 14.493 | -0.04% (3/5) | +0.00% (0/5) | 197.2 |
| 4×2 | heldout | ordinary | 14.519 | +0.00% (0/5) | +0.04% (2/5) | 200.8 |
| 4×2 | heldout | result | 12.239 | -15.37% (5/5) | -15.69% (5/5) | 152.5 |
| 4×2 | heldout | affine | 14.095 | -3.05% (5/5) | -2.99% (5/5) | 161.8 |
| 8×2 | catalog | release | 22.408 | -0.76% (3/5) | +0.00% (0/5) | 397.8 |
| 8×2 | catalog | ordinary | 22.450 | +0.00% (0/5) | +0.77% (2/5) | 416.2 |
| 8×2 | catalog | result | 18.383 | -18.39% (5/5) | -17.96% (5/5) | 347.8 |
| 8×2 | catalog | affine | 20.978 | -6.49% (5/5) | -6.09% (5/5) | 366.2 |
| 8×2 | heldout | release | 22.633 | +0.50% (1/5) | +0.00% (0/5) | 400.0 |
| 8×2 | heldout | ordinary | 22.508 | +0.00% (0/5) | -0.50% (4/5) | 403.2 |
| 8×2 | heldout | result | 18.517 | -17.25% (5/5) | -17.97% (5/5) | 357.0 |
| 8×2 | heldout | affine | 21.295 | -5.44% (5/5) | -5.91% (5/5) | 369.8 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
