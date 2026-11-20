# Shorter G2 inversion chains in 4×2 proofs

The two-chain trial took **13.678 / 13.728 ms** for the original and held-out Ryzen 4×2 inputs. Paired changes versus ordinary were **-0.09% / +0.70%**. The optimized one-chain trial took **13.750 / 13.628 ms**, with paired changes of **+0.68% / +0.24%**. No mode improved both matched controls on both inputs at the required consistency. The patch stays experimental. AWS SSO expired, so the [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) still has five of the eight supplied shapes at 10× M5 CPU.

The native affine addition prefix path accumulates denominator norms for groups of eight G2 additions. The one-chain control skips multiplication by the initial identity, multiplication by the first prefix in reverse and the unused final inverse update. The two-chain candidate alternates groups between two products, inverts their product once and recovers both inverses. The one-chain and two-chain variants both use three fewer packed multiplies per complete batch after accounting for the extra product and recovered factors. Two chains also shorten the dependent product sequences. The candidates reuse the existing prefix records and do not allocate key tables. Batches with tails and short batches keep their ordinary path.

All three native prefix tests passed. The new arithmetic test compares both candidates with pairwise point addition for even and odd counts of full groups and both chain inversion settings. In separate route captures, both modes reached a 200-pair complete batch on each input. The ordinary mode and the mode with prefix records disabled bypassed the candidates. Gnark independently verified all 72 exports.

The test uses a Ryzen 9950X with 24 main and eight frontend hardware threads. Each cell uses eight counterbalanced blocks with three primes and seventeen timed calls. Latency is the median of process means. The same-build ordinary route calibrated at **+0.41% / +0.49%** against the frozen seeded SDK, with a 2% absolute limit. Each candidate requires lower paired medians and at least six wins versus both controls on both inputs. Preferring two chains additionally requires that result against one chain. CPU-ms measures process CPU time per timed proof. Peak RSS includes key loading.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs one chain | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | frozen | 13.596 | +0.00% (0/8) | -0.41% (6/8) | -1.02% (7/8) | 291.3 | 212.5 |
| catalog | ordinary | 13.678 | +0.41% (2/8) | +0.00% (0/8) | -0.67% (5/8) | 293.3 | 202.2 |
| catalog | single | 13.750 | +1.03% (1/8) | +0.68% (3/8) | +0.00% (0/8) | 292.6 | 209.5 |
| catalog | double | 13.678 | +0.49% (3/8) | -0.09% (4/8) | -0.54% (7/8) | 293.8 | 208.8 |
| heldout | frozen | 13.606 | +0.00% (0/8) | -0.49% (4/8) | +0.02% (4/8) | 291.8 | 213.2 |
| heldout | ordinary | 13.660 | +0.49% (4/8) | +0.00% (0/8) | -0.24% (5/8) | 293.3 | 223.5 |
| heldout | single | 13.628 | -0.02% (4/8) | +0.24% (3/8) | +0.00% (0/8) | 292.4 | 223.2 |
| heldout | double | 13.728 | +0.63% (3/8) | +0.70% (2/8) | +0.47% (1/8) | 294.7 | 204.5 |

[Reproducible patch](artifacts/source.patch) · [Build pins](artifacts/built.json) · [Protocol](artifacts/protocol.json) · [Timing and route data](artifacts/rows.json.gz) · [Proof verification](artifacts/verification.json)
