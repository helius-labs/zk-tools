# Weighting G2 point ranges independently on Ryzen

The experiment flattens G2 window and point-range scheduling. Each job uses the existing bucket fill and weighted reduction, returns one projective point and releases its affine buckets. The final window fold adds the partials directly. The ordinary schedule merges affine buckets across ranges before weighting. Weighting is linear, so both schedules compute the same query. Both experiment modes reserve optional proof workspace before entry. The base count and range count bound the reservation. The ordinary schedule handles zero optional workspace.

The 1×2 and 2×2 same-build controls use the existing exclusive G2 endomorphism path. The 4×2 same-build control enables grouped Phi. Its release arm uses the ordinary released B2 path. The 2×2 arms use medium fused K/H. Ryzen uses 24 main and eight frontend logical CPUs. Each of six matched process blocks per input primes three proofs and measures thirteen. The native generator-sum test covers both widths, both range counts, full-width scalars, identities, repeated bases and one and eight workers. All 312 SDK proof exports independently verified, including the zero optional-memory routes. The first capture also traced candidate timings because the native flag defaults to enabled. Its data is retained separately. The scored timings use the same binary with tracing explicitly off.

Neither range layout consistently improved both controls across both inputs of any tested shape. The SDK default is unchanged.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.932 | +0.22% (3/6) | +0.00% (0/6) | 134.0 | 103 | 57.2 |
| 1×2 | catalog | ordinary | 6.932 | +0.00% (0/6) | -0.22% (3/6) | 133.8 | 130 | 57.8 |
| 1×2 | catalog | weighted2 | 6.795 | -1.50% (6/6) | -1.83% (6/6) | 133.9 | 94 | 56.2 |
| 1×2 | catalog | weighted4 | 7.224 | +4.78% (0/6) | +4.53% (0/6) | 144.0 | 137 | 60.0 |
| 1×2 | heldout | release | 6.802 | +1.12% (2/6) | +0.00% (0/6) | 128.7 | 137 | 58.2 |
| 1×2 | heldout | ordinary | 6.735 | +0.00% (0/6) | -1.11% (4/6) | 128.7 | 131 | 58.8 |
| 1×2 | heldout | weighted2 | 6.752 | -0.38% (3/6) | -0.73% (5/6) | 129.8 | 124 | 56.5 |
| 1×2 | heldout | weighted4 | 7.184 | +6.18% (1/6) | +5.28% (1/6) | 140.1 | 130 | 58.0 |
| 2×2 | catalog | release | 9.524 | -0.07% (3/6) | +0.00% (0/6) | 181.6 | 247 | 92.2 |
| 2×2 | catalog | ordinary | 9.512 | +0.00% (0/6) | +0.07% (3/6) | 181.4 | 199 | 93.2 |
| 2×2 | catalog | weighted2 | 9.464 | -0.09% (3/6) | -0.52% (5/6) | 183.1 | 263 | 96.2 |
| 2×2 | catalog | weighted4 | 9.861 | +3.82% (0/6) | +3.61% (0/6) | 191.2 | 220 | 96.0 |
| 2×2 | heldout | release | 9.501 | -0.01% (4/6) | +0.00% (0/6) | 181.5 | 326 | 93.8 |
| 2×2 | heldout | ordinary | 9.532 | +0.00% (0/6) | +0.01% (2/6) | 181.4 | 208 | 91.2 |
| 2×2 | heldout | weighted2 | 9.495 | -0.16% (4/6) | +0.15% (2/6) | 182.6 | 222 | 94.5 |
| 2×2 | heldout | weighted4 | 9.868 | +3.62% (0/6) | +3.92% (0/6) | 191.6 | 287 | 100.8 |
| 4×2 | catalog | release | 13.821 | +2.92% (0/6) | +0.00% (0/6) | 294.1 | 949 | 206.8 |
| 4×2 | catalog | ordinary | 13.395 | +0.00% (0/6) | -2.84% (6/6) | 282.7 | 948 | 201.0 |
| 4×2 | catalog | weighted2 | 13.264 | -0.98% (5/6) | -3.79% (6/6) | 283.0 | 971 | 208.8 |
| 4×2 | catalog | weighted4 | 13.719 | +2.41% (0/6) | -0.53% (6/6) | 293.9 | 890 | 193.0 |
| 4×2 | heldout | release | 13.699 | +2.66% (1/6) | +0.00% (0/6) | 293.4 | 869 | 201.2 |
| 4×2 | heldout | ordinary | 13.354 | +0.00% (0/6) | -2.59% (5/6) | 283.4 | 886 | 202.2 |
| 4×2 | heldout | weighted2 | 13.279 | -0.80% (4/6) | -3.07% (6/6) | 283.7 | 984 | 200.0 |
| 4×2 | heldout | weighted4 | 13.728 | +2.62% (1/6) | +0.22% (2/6) | 293.6 | 942 | 195.5 |

The route probes report the forecast charged before entering the candidate. The forecast is an allowance for concurrent scratch and differs from measured process RSS.

| Shape | Input | Query points | Window bits | Ranges | Reserved MiB |
|---|---|---:|---:|---:|---:|
| 1×2 | catalog | 30,089 | 12 | 2 | 54.2 |
| 1×2 | catalog | 30,089 | 12 | 4 | 107.4 |
| 1×2 | heldout | 30,089 | 12 | 2 | 54.2 |
| 1×2 | heldout | 30,089 | 12 | 4 | 107.4 |
| 2×2 | catalog | 27,218 | 12 | 2 | 54.1 |
| 2×2 | catalog | 27,218 | 12 | 4 | 107.3 |
| 2×2 | heldout | 27,267 | 12 | 2 | 54.1 |
| 2×2 | heldout | 27,267 | 12 | 4 | 107.3 |
| 4×2 | catalog | 29,738 | 12 | 2 | 54.2 |
| 4×2 | catalog | 29,738 | 12 | 4 | 107.3 |
| 4×2 | heldout | 29,961 | 12 | 2 | 54.2 |
| 4×2 | heldout | 29,961 | 12 | 4 | 107.4 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [First capture with tracing](artifacts/trace-enabled-rows.json.gz) · [Verification](artifacts/verification.json)
