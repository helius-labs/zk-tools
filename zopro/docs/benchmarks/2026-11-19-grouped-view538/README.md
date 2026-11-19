# Nested scalar access and outer worker sleep on 12×2

The fixed prepared groups use two B2 levels and one A/K plan. This experiment crosses the first-level scalar view with the outer worker idle policy in the same executable. Release and frozen grouped builds serve as separate controls. Five randomized blocks use 30 pinned Ryzen workers and three warmups followed by eleven timed calls. The original request and four further requests cover adjacent and scattered leaf patterns. Peak RSS includes setup.

| Request | Release / frozen grouped / ordinary / view / outer / both ms | Paired view vs ordinary / both vs outer / both vs frozen / both vs release | Peak MiB in mode order |
|---|---:|---|---|
| Original | 31.315 / 29.945 / 29.763 / 29.674 / 30.120 / 30.223 | -0.35% (3/5) / +0.21% (2/5) / +0.66% (2/5) / -4.27% (5/5) | 563 / 576 / 579 / 541 / 476 / 462 |
| 58 | 31.156 / 29.289 / 29.773 / 29.651 / 30.225 / 30.063 | -0.40% (4/5) / -0.33% (3/5) / +3.11% (1/5) / -3.51% (5/5) | 560 / 587 / 583 / 526 / 469 / 458 |
| 63 | 31.230 / 29.491 / 29.824 / 29.797 / 30.434 / 30.129 | -0.36% (3/5) / -0.71% (4/5) / +2.98% (0/5) / -3.78% (5/5) | 586 / 566 / 584 / 531 / 474 / 454 |
| 139 | 48.175 / 45.838 / 45.952 / 45.380 / 46.785 / 46.947 | -1.78% (5/5) / +1.83% (2/5) / +2.05% (1/5) / -1.66% (4/5) | 990 / 1007 / 960 / 993 / 700 / 660 |
| 143 | 49.171 / 45.843 / 45.879 / 46.106 / 45.519 / 45.207 | +0.37% (2/5) / -1.04% (4/5) / -1.52% (4/5) / -5.63% (5/5) | 958 / 1007 / 978 / 993 / 691 / 659 |

The original combined mode reached 9.40× M5 CPU and 3.34× M5 Metal. 0/5 process means met 28.4 ms. All 150 exports verified. [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json) · [View implementation](../2026-11-19-lazy-representatives536/README.md) · [Worker policy](../2026-11-19-idle-context530/README.md)
