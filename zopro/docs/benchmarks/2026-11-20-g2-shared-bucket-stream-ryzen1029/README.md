# Shared G2 component stream on Ryzen

Sharing the point scan and inversion queue increased paired full-proof latency by 2.76–4.54%. The ordinary split queue remains selected.

The prototype visits each point once per window and submits both signed components to disjoint bucket ranges in one inversion queue. The second range receives its existing endomorphism before the ranges are combined. An optional scratch reservation covers the larger active window.

| Shape | Request | Control / shared ms | Paired shared change | Control / shared peak MiB |
|---|---|---:|---:|---:|
| 1×2 | catalog | 7.614 / 7.848 | +3.24% (0/6) | 58.0 / 61.8 |
| 1×2 | heldout | 7.578 / 7.929 | +4.43% (1/6) | 61.5 / 70.8 |
| 2×2 | catalog | 11.469 / 11.754 | +2.76% (0/6) | 86.0 / 88.2 |
| 2×2 | heldout | 11.510 / 11.850 | +2.94% (0/6) | 88.0 / 91.0 |
| 1×4 | catalog | 8.543 / 8.882 | +3.71% (0/6) | 67.2 / 70.8 |
| 1×4 | heldout | 8.568 / 8.895 | +3.87% (0/6) | 66.5 / 82.8 |
| 1×16 | catalog | 13.018 / 13.609 | +4.54% (0/6) | 94.5 / 100.2 |
| 1×16 | heldout | 12.974 / 13.416 | +3.42% (0/6) | 96.9 / 95.7 |

One native test compares colliding signed windows with the ordinary kernel. Eight runtime probes confirm dispatch, and eight zero-scratch probes confirm fallback. Six timing blocks on each request used 24 main and eight frontend Ryzen logical workers. Gnark independently reverified all 120 exported proofs.

[Summary](artifacts/summary.json) · [Samples and probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
