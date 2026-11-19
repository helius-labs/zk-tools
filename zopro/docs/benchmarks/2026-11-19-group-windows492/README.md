# H window selection with sampled scalar groups on 12×2

The same experimental executable used sampled groups for A, K and B2 with 32 pinned Ryzen workers. This screen varies only the requested dense H layout. Each of three randomized blocks compares the ordinary setting, 18 signed windows, and 19 unsigned windows, all configured for one range. Each process primed three proofs, timed eleven, and exported a proof for independent Gnark verification. Values are medians of process means. Percent changes compare within blocks. Peak RSS includes setup. Request 57 has adjacent leaves and 137 has scattered leaves. These three requests provide a screen for a wider comparison.

| Request | H setting | Proof ms | Paired vs off | Faster blocks | Peak MiB |
|---|---|---:|---:|---:|---:|
| Original | off | 29.419 | reference | — | 607.8 |
| Original | 18s | 30.412 | +3.23% | 0/3 | 624.8 |
| Original | 19 | 29.875 | +2.12% | 1/3 | 612.3 |
| 57 | off | 29.559 | reference | — | 629.9 |
| 57 | 18s | 30.313 | +2.44% | 0/3 | 636.9 |
| 57 | 19 | 30.280 | +2.44% | 0/3 | 635.1 |
| 137 | off | 48.931 | reference | — | 969.0 |
| 137 | 18s | 49.482 | -0.02% | 2/3 | 969.7 |
| 137 | 19 | 48.094 | -2.05% | 2/3 | 997.4 |

On the original request both alternatives increased paired latency, by 2.12–3.23%. On the single scattered request the 19-window setting reduced paired latency by 2.05%. The scattered result needs an independent witness panel before it can support input-dependent dispatch. All 27 exports verified.

[Process measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Sampled group implementation and controls](../2026-11-19-sampled-groups487/README.md)
