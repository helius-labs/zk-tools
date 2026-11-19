# Isolating scalar grouping from the global prover option

The C7a 1×2 global deduplication switch sped up the second input and slowed the original. It also changes the workspace forecast. This study uses the source of the byte-identified C7a server executable and bypasses the canonical and exclusive G2 grouping entries independently. Skipping both retains the conservative enabled workspace forecast. The global-off arm runs the normal application-wide setting. A second disabled arm and the earlier binary provide separate controls. Three distinct public transactions each ran in six blocks of twenty-one timed full proofs at 32 Ryzen logical CPUs. All 108 process exports passed independent verification.

| Input | Configuration | Median ms | Paired vs same binary off | Peak MiB |
|---|---|---:|---:|---:|
| catalog | release | 7.234 | +0.12% (3/6) | 61.2 |
| catalog | off | 7.212 | +0.00% (0/6) | 63.8 |
| catalog | skip-a | 7.702 | +6.76% (0/6) | 56.8 |
| catalog | skip-b2 | 7.229 | +0.04% (3/6) | 63.0 |
| catalog | skip-both | 7.696 | +6.63% (0/6) | 56.0 |
| catalog | global-off | 7.724 | +7.00% (0/6) | 58.2 |
| heldout | release | 7.002 | +0.12% (2/6) | 58.0 |
| heldout | off | 6.988 | +0.00% (0/6) | 58.8 |
| heldout | skip-a | 7.522 | +7.67% (0/6) | 56.2 |
| heldout | skip-b2 | 7.034 | +0.61% (1/6) | 58.5 |
| heldout | skip-both | 7.486 | +6.92% (0/6) | 56.5 |
| heldout | global-off | 7.470 | +6.90% (0/6) | 57.2 |
| v27 | release | 7.014 | +0.74% (1/6) | 58.2 |
| v27 | off | 6.986 | +0.00% (0/6) | 58.2 |
| v27 | skip-a | 7.516 | +7.46% (0/6) | 56.8 |
| v27 | skip-b2 | 7.089 | +1.10% (1/6) | 58.8 |
| v27 | skip-both | 7.595 | +7.66% (0/6) | 58.0 |
| v27 | global-off | 7.516 | +7.67% (0/6) | 57.2 |

[Implementation](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [AWS comparison](../2026-11-19-varied-phases716/README.md)
