# Starting shared 4×2 scalar preparation during the quotient FFT

Starting shared K/B2 preparation during the FFT fell short of the matched latency comparison. The frontend variant took **14.337 / 14.188 ms** and was **5.00% / 3.78% slower** in paired medians than same-build ordinary. The main-pool variant was **6.69% / 6.40% slower**. Both lost every block against the frozen, ordinary and gate controls on both inputs.

The earlier projection performed exact scalar classification once but delayed both K and B2 until it finished. The new modes start that same checked preparation concurrently with the FFT. `early-main` uses the FFT's existing pool, and `early-front` uses the existing frontend pool while the FFT stays on main. The early and late modes use the same consumer-width workspace forecast. The paired curve join is shared with the gate control. At zero optional workspace, route probes verified ordinary fallback. [Original projection experiment](../2026-11-20-four-by-two-k-b2-projection-ryzen919/README.md)

Ryzen uses 24 main and eight frontend hardware threads. Eight counterbalanced blocks per input prime three requests and time seventeen. Fresh ordinary calibrates within 2% against frozen on both inputs, at −0.04% / +0.45%. Each early mode requires lower paired medians and six wins out of eight against frozen, ordinary and gate on both inputs. All 136 exported proofs independently verified.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs gate | Paired vs late | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | frozen | 13.590 | +0.00% (0/8) | +0.04% (4/8) | -0.05% (4/8) | -4.92% (8/8) | 210.2 |
| catalog | ordinary | 13.641 | -0.04% (4/8) | +0.00% (0/8) | -0.46% (5/8) | -4.95% (8/8) | 214.5 |
| catalog | gate | 13.626 | +0.05% (4/8) | +0.46% (3/8) | +0.00% (0/8) | -4.56% (8/8) | 220.2 |
| catalog | late | 14.292 | +5.18% (0/8) | +5.20% (0/8) | +4.78% (0/8) | +0.00% (0/8) | 235.0 |
| catalog | early-main | 14.525 | +6.79% (0/8) | +6.69% (0/8) | +6.35% (0/8) | +1.54% (0/8) | 210.5 |
| catalog | early-front | 14.337 | +5.15% (0/8) | +5.00% (0/8) | +5.31% (0/8) | +0.14% (4/8) | 182.2 |
| heldout | frozen | 13.639 | +0.00% (0/8) | -0.44% (4/8) | +0.08% (3/8) | -4.76% (8/8) | 209.2 |
| heldout | ordinary | 13.633 | +0.45% (4/8) | +0.00% (0/8) | +0.61% (3/8) | -4.80% (8/8) | 215.0 |
| heldout | gate | 13.565 | -0.08% (5/8) | -0.61% (5/8) | +0.00% (0/8) | -5.41% (8/8) | 214.2 |
| heldout | late | 14.313 | +5.01% (0/8) | +5.04% (0/8) | +5.72% (0/8) | +0.00% (0/8) | 239.8 |
| heldout | early-main | 14.522 | +6.96% (0/8) | +6.40% (0/8) | +6.79% (0/8) | +0.98% (0/8) | 208.2 |
| heldout | early-front | 14.188 | +4.40% (0/8) | +3.78% (0/8) | +4.53% (0/8) | -0.81% (6/8) | 182.8 |

The frontend mode overlapped **2.35 / 2.40 ms**, but the prerequisites for the curve stage still took **2.76 / 2.87 ms**, up from **2.23 / 2.25 ms** with ordinary. FFT elapsed increased by **0.12 / 0.19 ms**. Projected preparation still outlasted the overlapping portion. The main placement delayed the curve start further. Clocked results come from three separate blocks per input and include any thread handoff in the preparation interval. Late preparation is inside the curve interval.

| Input | Mode | FFT ms | Shared preparation ms | Interval overlap ms | Before curves ms | Curves ms |
|---|---|---:|---:|---:|---:|---:|
| catalog | ordinary | 2.232 | — | — | 2.232 | 8.815 |
| catalog | gate | 2.177 | — | — | 2.177 | 8.563 |
| catalog | late | 2.257 | 3.896 | — | 2.257 | 9.400 |
| catalog | early-main | 2.372 | 3.082 | 2.371 | 3.097 | 8.882 |
| catalog | early-front | 2.351 | 2.750 | 2.350 | 2.764 | 8.811 |
| heldout | ordinary | 2.249 | — | — | 2.249 | 8.597 |
| heldout | gate | 2.237 | — | — | 2.237 | 8.830 |
| heldout | late | 2.254 | 3.979 | — | 2.254 | 9.357 |
| heldout | early-main | 2.373 | 3.079 | 2.372 | 3.093 | 8.789 |
| heldout | early-front | 2.434 | 2.755 | 2.399 | 2.866 | 8.817 |

Frontend projection reduced observed whole-process peak RSS from **214.5 / 215.0 MiB** to **182.2 / 182.8 MiB**. It increased median aggregate CPU time from **292.0 / 292.3 to 300.0 / 298.7 CPU-ms per proof**. RSS includes setup. This is a latency and memory tradeoff on the measured host. The trial stayed out of the default prover.

The [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) still shows five of eight original requests reaching 10× M5 CPU and includes all eight Metal comparisons. AWS SSO expiration kept the pending 2×2 run from starting.

[Protocol](artifacts/protocol.json) · [Samples and intervals](artifacts/rows.json.gz) · [Aggregates](artifacts/summary.json) · [Trial source](artifacts/source.patch) · [Verification](artifacts/verification.json)
