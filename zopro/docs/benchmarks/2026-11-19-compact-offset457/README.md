# Small-workspace B2 correction setup

Sixty-four independent four-bit windows compute the same fixed correction point from the public key. Each active leaf uses 15 projective buckets. The compact calculation reserves 155,136 bytes of temporary workspace at 32 workers. The general batched kernel reserved 49,685,752 bytes. Each retains the same 42,048-byte vector of public offsets plus a correction point. A diagnostic probe compared each active setup path against ordinary serial scalar multiplication. Compact setup ran under a 1 MiB process-wide optional preparation budget. With zero optional budget the ordinary route produced a valid proof.

Two fresh fixtures from experiment 456 were repeated in nine matched randomized blocks. The first proof starts after key loading. Each later sample averages 21 complete warm SDK calls. First-proof differences and warm percentages use paired blocks against the disabled mode under the same optional budget.

| Input | Mode | First proof ms | Extra first-proof ms | Warm ms | Warm paired change | Wins / 9 | Peak RSS MiB |
|---:|---|---:|---:|---:|---:|---:|---:|
| 9 | off | 59.30 | +0.00 | 6.973 | +0.00% | — | 62.8 |
| 9 | batch | 61.19 | +1.69 | 6.892 | -1.75% | 7 | 62.2 |
| 9 | compact | 61.24 | +1.92 | 6.886 | -1.72% | 9 | 60.5 |
| 9 | limited-off | 59.91 | +0.00 | 7.468 | +0.00% | — | 56.0 |
| 9 | limited-compact | 61.32 | +1.63 | 7.401 | -1.81% | 7 | 56.0 |
| 16 | off | 59.18 | +0.00 | 6.988 | +0.00% | — | 63.0 |
| 16 | batch | 60.37 | +1.09 | 6.929 | -0.84% | 7 | 62.0 |
| 16 | compact | 61.38 | +1.96 | 6.926 | -0.85% | 8 | 62.8 |
| 16 | limited-off | 59.39 | +0.00 | 7.509 | +0.00% | — | 55.8 |
| 16 | limited-compact | 61.86 | +2.37 | 7.481 | -0.01% | 5 | 55.8 |

Traced setup took 3.81 ms in the general kernel, 4.82 ms in the compact kernel and 4.58 ms in the compact kernel under the 1 MiB budget. These single observations include key digest validation. The temporary reservation covers algorithm workspace. Maximum RSS includes the key and all process allocations. All 96 exported proofs passed independent Gnark verification.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Incremental patch](artifacts/changes.patch) · [Eight-input warm comparison](../2026-11-19-batched-offset456/README.md)
