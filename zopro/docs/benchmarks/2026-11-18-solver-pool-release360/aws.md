# C7a qualification of the optional interpreter pool

All rows use the same binary for the 96-core idle and active configurations. Both attach the same smaller pool. The active configuration opts in to routing the general CPU solver instructions to it. The integrated comparison binary predates this option. The primary merge24 and secondary 16×2 measurements each use seven process blocks. Other shapes use three. Each process times complete SDK requests after three primes. Shown are medians of per-process means.

| Shape | Exact prior 64 ms | Exact prior 96 ms | New idle 96 ms | New active 96 ms | Paired active vs idle | Faster blocks | Active peak MiB | Active vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.536 | 8.573 | 8.550 | 8.515 | -0.50% | 2/3 | 45 | 4.35× / 2.82× |
| 2x2 | 14.473 | 14.806 | 14.912 | 14.959 | +0.10% | 1/3 | 103 | 3.94× / 2.01× |
| 4x2 | 18.121 | 20.260 | 20.078 | 19.702 | +0.28% | 1/3 | 225 | 5.53× / 2.44× |
| 8x2 | 24.606 | 26.534 | 26.305 | 26.984 | +3.08% | 1/3 | 376 | 7.00× / 2.74× |
| 12x2 | 32.797 | 33.915 | 33.533 | 35.343 | +5.56% | 0/3 | 567 | 8.04× / 2.86× |
| 16x2 | 34.126 | 36.389 | 36.616 | 36.497 | +0.51% | 1/7 | 735 | 9.18× / 3.40× |
| 24x2 | 47.188 | 50.259 | 51.877 | 51.273 | -1.17% | 2/3 | 1011 | 10.30× / 3.43× |
| merge-24x1 | 49.085 | 52.913 | 54.202 | 51.183 | -5.44% | 7/7 | 1019 | 9.91× / 3.42× |

The small-circuit configurations attach 16 frontend workers, and 1×2 uses two G2 ranges at 64 and four with serial decomposition at 96. Other active and idle 96-core configurations attach 48 workers. Integrated large-circuit controls have no auxiliary pool. Medium and large shapes use eight epochs except 16×2, which uses six in every cell.

| Shape | Main cores | Setting | Wall ms | Aggregate CPU-ms | Peak MiB | Blocks at 10× M5 CPU |
|---|---:|---|---:|---:|---:|---:|
| 16x2 | 64 | release | 34.126 | 1672.5 | 771 | 1/7 |
| 16x2 | 64 | active | 34.751 | 1519.3 | 724 | 0/7 |
| 16x2 | 96 | release | 36.389 | 2455.0 | 790 | 0/7 |
| 16x2 | 96 | idle | 36.616 | 2427.0 | 788 | 0/7 |
| 16x2 | 96 | previous | 36.443 | 2182.4 | 720 | 0/7 |
| 16x2 | 96 | active | 36.497 | 2205.3 | 735 | 0/7 |
| merge-24x1 | 64 | release | 49.085 | 2298.9 | 1102 | 7/7 |
| merge-24x1 | 64 | active | 48.255 | 2107.6 | 986 | 5/7 |
| merge-24x1 | 96 | release | 52.913 | 3400.3 | 1107 | 0/7 |
| merge-24x1 | 96 | idle | 54.202 | 3449.7 | 1151 | 0/7 |
| merge-24x1 | 96 | previous | 50.180 | 2961.4 | 1050 | 6/7 |
| merge-24x1 | 96 | active | 51.183 | 3024.2 | 1019 | 2/7 |

[Earlier confirmation](../2026-11-18-epoch-team357/confirmation.md) · [Protocol](artifacts/aws-protocol.md) · [AWS measurements](artifacts/aws.json.gz)
