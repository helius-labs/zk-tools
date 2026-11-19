# Packing two polynomial inputs directly into eight native field lanes

The FFT consumes four consecutive field elements from A and four from B. The direct mode reads each complete four-element slice in two unaligned 64-byte loads and transposes their 64-bit words. The shifted mode also places the factor of sixteen for the native radix change directly into the radix-52 split. Both use the existing final reduction. Partial input groups take the padded packing route. The change adds no tables or key preparation.

A native lane test checks every combination of ten A and B prefix lengths, including unaligned lengths and complete rows, against the original field values. The complete inverse and direct coefficient oracles also ran in all three modes. Five randomized blocks compare the frozen executable, the new executable using the ordinary packer, direct loads, and direct loads with the fused radix split. Each uses its original shape profile, 32 Ryzen workers and the same request, key and verifier.

| Shape | Frozen / ordinary / direct / shifted ms | Direct vs ordinary | Shifted vs ordinary | Shifted vs direct | Shifted vs frozen | Peak MiB ordinary / shifted | Shifted vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.063 / 7.148 / 7.133 / 7.109 | -0.19% (3/5) | -0.47% (3/5) | -0.26% (3/5) | +0.66% (0/5) | 63.0 / 63.2 | 5.20× / 3.38× |
| 2x2 | 9.655 / 9.622 / 9.633 / 9.649 | +0.19% (1/5) | +0.38% (2/5) | -0.21% (3/5) | -0.10% (3/5) | 114.2 / 112.5 | 6.11× / 3.11× |
| 4x2 | 15.517 / 15.495 / 15.521 / 15.469 | +0.06% (2/5) | -0.52% (3/5) | +0.06% (2/5) | -0.65% (4/5) | 269.8 / 264.5 | 7.05× / 3.10× |
| 8x2 | 23.760 / 24.418 / 23.960 / 24.568 | -1.26% (4/5) | +1.05% (1/5) | +1.89% (1/5) | +3.17% (0/5) | 424.2 / 438.2 | 7.69× / 3.01× |
| 12x2 | 31.431 / 32.117 / 31.805 / 31.232 | +0.12% (2/5) | -4.08% (4/5) | -1.70% (4/5) | -0.63% (4/5) | 571.0 / 569.9 | 9.09× / 3.23× |
| 16x2 | 39.186 / 39.419 / 39.328 / 39.227 | +0.09% (2/5) | -0.38% (3/5) | -0.08% (3/5) | -0.12% (3/5) | 653.7 / 681.0 | 8.54× / 3.16× |
| 24x2 | 63.485 / 63.099 / 63.364 / 63.248 | +0.01% (2/5) | +0.85% (2/5) | +0.83% (2/5) | -0.80% (3/5) | 1031.5 / 1040.1 | 8.35× / 2.78× |
| merge-24x1 | 63.524 / 63.931 / 63.758 / 63.322 | -0.50% (3/5) | -1.33% (4/5) | -1.87% (4/5) | -0.20% (3/5) | 1063.5 / 1069.8 | 8.01× / 2.76× |

All 160 process exports verified independently. [Process timings](artifacts/processes.json.gz) · [Source and test](artifacts/pack.patch) · [Verification](artifacts/verification.json)
