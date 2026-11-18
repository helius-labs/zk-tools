# Rounded high products during G2 scalar decomposition

The experimental split computes only the high multiplication diagonals needed to round each lattice quotient. A conservative guard reruns the full product when omitted carries could cross the half-limb threshold. The resulting signed components match the existing calculation. The candidate adds no key data or proof workspace.

The native suite compares 600,000 generic products, boundary inputs, component reconstruction and the complete G2 MSM against the existing oracles. A separate Rust checker explicitly constructs inputs where omitted carries change rounding. Its output records the exact number of such cases.

The isolated 4,096-scalar loop took 7.54 ns/scalar for the original split and 7.59 ns/scalar for the guarded split, a +0.77% paired change (1/41 blocks faster). These timings cover the compilation and call boundaries in the supplied harness.

| Shape | Setting | Median ms | Paired vs fresh off | Faster blocks | CPU-ms/proof | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | retained | 7.957 | -0.66% | 4/7 | 156.8 | 63 |
| 1x2 | off | 7.958 | +0.00% | 0/7 | 156.6 | 64 |
| 1x2 | high | 7.922 | -0.24% | 4/7 | 156.1 | 65 |
| 2x2 | retained | 12.725 | +16.44% | 2/7 | 213.2 | 115 |
| 2x2 | off | 10.952 | +0.00% | 0/7 | 210.8 | 113 |
| 2x2 | high | 10.952 | +0.12% | 3/7 | 210.4 | 112 |

All 46 exported proofs passed independent verification. The two enabled probes reached the shortened split. The off and denied-memory probes had zero entries. Measurements use 32 logical Ryzen threads including SMT. C7a timings apply to the preceding integrated release.

[Protocol](artifacts/protocol.md) · [Per-request timing](artifacts/processes.json.gz) · [Component check](artifacts/component-results.txt) · [Route probes](artifacts/routing.json) · [Source change](artifacts/source.patch)

On 1x2, the fresh disabled control differed from the retained executable by +0.01%. The same-binary shortened-quotient change was -0.24% with 4/7 block wins.

On 2x2, the fresh disabled control differed from the retained executable by -13.94%. The same-binary shortened-quotient change was +0.12% with 3/7 block wins.

The isolated loop and full-proof panel did not establish a speed gain for adoption. The 2×2 control discrepancy further limits comparisons across executables.
