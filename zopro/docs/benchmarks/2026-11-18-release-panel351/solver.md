# Direct product assignments on C7a

These four shapes use the experimental solver executable and a same-binary control with the shortcut disabled. Each setting uses seven randomized processes. The shortcut preserves every R1CS output row and adds no per-key lookup table.

| Shape | Workers | Mode | Median ms | Paired change vs off | Faster blocks |
|---|---:|---|---:|---:|---:|
| 1x2 | 64 | off | 8.388 | +0.00% | 0/7 |
| 1x2 | 64 | direct | 8.299 | -0.79% | 5/7 |
| 1x2 | 64 | square | 8.294 | -1.34% | 5/7 |
| 2x2 | 48 | off | 14.001 | +0.00% | 0/7 |
| 2x2 | 48 | direct | 14.045 | +0.85% | 3/7 |
| 2x2 | 48 | square | 14.028 | -0.74% | 5/7 |
| 16x2 | 64 | off | 37.206 | +0.00% | 0/7 |
| 16x2 | 64 | direct | 36.915 | -0.87% | 6/7 |
| 16x2 | 64 | square | 36.902 | -0.62% | 5/7 |
| merge-24x1 | 64 | off | 53.750 | +0.00% | 0/7 |
| merge-24x1 | 64 | direct | 54.019 | +1.05% | 3/7 |
| merge-24x1 | 64 | square | 53.880 | +0.69% | 2/7 |

[Earlier Ryzen experiment and route checks](../2026-11-18-wire-product350/README.md) · [Primary report](README.md)
