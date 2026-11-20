# Scoped G2 SDK transfer on 96-core C7a

The SDK port qualified on 2×2, 2×4, 2×8.

This run compares the native-tested SDK port with the archived scoped profile, pinned catalog build and prepared Aeglos. 2×2 and 2×8 use 48+48 cores while 2×4 uses 64+32. The fused arm enables Euler leaf fusion and packed final exponentiation. The matched timing panel is separate from the ELF diagnostics.

| Shape | Request | Profile | Proof ms | Paired vs archive | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 2×2 | catalog | Pinned | 11.930 | +4.46% (0/8) | +0.94% (3/8) | 76.8 |
| 2×2 | catalog | Archived early | 11.372 | +0.00% (0/8) | -3.86% (8/8) | 76.9 |
| 2×2 | catalog | SDK early | 11.313 | -0.28% (5/8) | -4.42% (7/8) | 84.4 |
| 2×2 | catalog | SDK early, fused | 11.200 | -1.90% (8/8) | -5.34% (8/8) | 82.0 |
| 2×2 | catalog | Prepared Aeglos | 11.845 | +4.01% (0/8) | +0.00% (0/8) | 816.6 |
| 2×2 | heldout | Pinned | 12.000 | +4.89% (0/8) | +0.21% (4/8) | 77.6 |
| 2×2 | heldout | Archived early | 11.382 | +0.00% (0/8) | -4.83% (8/8) | 80.5 |
| 2×2 | heldout | SDK early | 11.353 | -0.50% (6/8) | -4.71% (8/8) | 80.6 |
| 2×2 | heldout | SDK early, fused | 11.254 | -1.01% (7/8) | -4.59% (8/8) | 81.4 |
| 2×2 | heldout | Prepared Aeglos | 11.977 | +5.08% (0/8) | +0.00% (0/8) | 807.2 |
| 2×4 | catalog | Pinned | 13.976 | +22.42% (0/8) | +13.71% (0/8) | 142.6 |
| 2×4 | catalog | Archived early | 11.417 | +0.00% (0/8) | -7.05% (8/8) | 185.4 |
| 2×4 | catalog | SDK early | 11.293 | -1.22% (7/8) | -8.16% (8/8) | 192.2 |
| 2×4 | catalog | SDK early, fused | 11.161 | -1.83% (8/8) | -9.92% (8/8) | 186.1 |
| 2×4 | catalog | Prepared Aeglos | 12.325 | +7.58% (0/8) | +0.00% (0/8) | 821.6 |
| 2×4 | heldout | Pinned | 13.904 | +22.26% (0/8) | +12.43% (0/8) | 147.9 |
| 2×4 | heldout | Archived early | 11.323 | +0.00% (0/8) | -8.27% (8/8) | 189.2 |
| 2×4 | heldout | SDK early | 11.350 | +0.27% (3/8) | -7.97% (8/8) | 187.0 |
| 2×4 | heldout | SDK early, fused | 11.186 | -0.92% (6/8) | -8.89% (8/8) | 191.5 |
| 2×4 | heldout | Prepared Aeglos | 12.335 | +9.02% (0/8) | +0.00% (0/8) | 828.4 |
| 2×8 | catalog | Pinned | 15.527 | +18.58% (0/8) | +1.37% (2/8) | 130.6 |
| 2×8 | catalog | Archived early | 13.090 | +0.00% (0/8) | -14.94% (8/8) | 180.9 |
| 2×8 | catalog | SDK early | 13.185 | +1.11% (2/8) | -14.38% (8/8) | 173.6 |
| 2×8 | catalog | SDK early, fused | 12.934 | -1.36% (6/8) | -15.04% (8/8) | 183.4 |
| 2×8 | catalog | Prepared Aeglos | 15.268 | +17.56% (0/8) | +0.00% (0/8) | 1026.9 |
| 2×8 | heldout | Pinned | 15.532 | +18.01% (0/8) | +1.12% (1/8) | 132.3 |
| 2×8 | heldout | Archived early | 13.204 | +0.00% (0/8) | -14.53% (8/8) | 184.1 |
| 2×8 | heldout | SDK early | 13.087 | -0.69% (6/8) | -14.15% (8/8) | 186.9 |
| 2×8 | heldout | SDK early, fused | 12.923 | -1.85% (6/8) | -14.88% (8/8) | 180.2 |
| 2×8 | heldout | Prepared Aeglos | 15.208 | +17.02% (0/8) | +0.00% (0/8) | 1026.6 |

A port profile qualifies when both requests beat Aeglos in at least six paired blocks with a lower paired median and stay within 2% of the archived profile on paired medians. The table below records each outcome.

| Shape | Qualifying profiles |
|---|---|
| 2×2 | SDK early, SDK early, fused |
| 2×4 | SDK early, SDK early, fused |
| 2×8 | SDK early, SDK early, fused |

## ELF phase diagnostics

Trace spans are measured from the outer SDK proof call. The output uses medians of the fifteen traced proofs per request and profile. Optional inner calls are shown where the executable entered their exact symbols. Grouping measures entry to G2 through entry to the decomposition function. Scalar splitting measures entry to decomposition through entry to window processing. Time after generation includes the remaining inline verification and SDK work.

| Shape | Request | Profile | G2 start | G2 | Grouping | Split scalars | G2 windows | Quotient | Overlap | Last observed G1 | After generation | Proof |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 2×2 | catalog | SDK early | 3.604 | 7.706 | 3.224 | 0.193 | 4.407 | 1.191 | 1.154 | 9.248 | 0.593 | 12.288 |
| 2×2 | catalog | SDK early, fused | 3.707 | 7.780 | 3.213 | 0.186 | 4.353 | 1.242 | 1.198 | 9.390 | 0.508 | 12.343 |
| 2×2 | catalog | SDK late | 5.086 | 7.884 | 3.514 | 0.237 | 4.114 | 1.279 | 0.000 | 8.294 | 0.581 | 13.392 |
| 2×2 | catalog | SDK zero budget | 12.635 | 10.784 | 0.783 | 4.121 | 5.845 | 1.346 | 0.000 | 23.289 | 0.592 | 24.102 |
| 2×2 | heldout | SDK early | 3.796 | 7.802 | 3.293 | 0.201 | 4.184 | 1.265 | 1.216 | 9.222 | 0.592 | 12.499 |
| 2×2 | heldout | SDK early, fused | 3.788 | 7.940 | 3.170 | 0.240 | 4.493 | 1.269 | 1.225 | 9.787 | 0.508 | 12.439 |
| 2×2 | heldout | SDK late | 5.119 | 8.304 | 3.469 | 0.213 | 4.116 | 1.191 | 0.000 | 8.767 | 0.569 | 14.244 |
| 2×2 | heldout | SDK zero budget | 12.532 | 10.544 | 0.796 | 4.129 | 5.652 | 1.342 | 0.000 | 23.225 | 0.593 | 24.196 |
| 2×4 | catalog | SDK early | 3.500 | 8.137 | 4.020 | 0.173 | 3.917 | 2.520 | 2.508 | 12.313 | 0.595 | 13.409 |
| 2×4 | catalog | SDK early, fused | 3.526 | 8.076 | 4.006 | 0.157 | 3.901 | 2.088 | 1.850 | 11.888 | 0.516 | 12.698 |
| 2×4 | catalog | SDK late | 5.284 | 8.806 | 4.721 | 0.152 | 3.800 | 1.511 | 0.000 | 11.571 | 0.568 | 14.857 |
| 2×4 | catalog | SDK zero budget | 12.973 | 9.320 | 0.761 | 0.730 | 7.074 | 1.616 | 0.000 | 22.810 | 0.587 | 23.642 |
| 2×4 | heldout | SDK early | 3.459 | 8.056 | 3.693 | 0.163 | 4.348 | 2.258 | 2.249 | 12.812 | 0.595 | 14.402 |
| 2×4 | heldout | SDK early, fused | 3.530 | 8.070 | 3.940 | 0.146 | 3.918 | 1.928 | 1.920 | 11.892 | 0.518 | 12.934 |
| 2×4 | heldout | SDK late | 5.218 | 8.666 | 4.361 | 0.122 | 3.755 | 1.514 | 0.000 | 11.151 | 0.576 | 14.485 |
| 2×4 | heldout | SDK zero budget | 12.894 | 10.486 | 0.918 | 1.257 | 6.584 | 1.658 | 0.000 | 21.689 | 0.593 | 24.060 |
| 2×8 | catalog | SDK early | 4.115 | 8.907 | 3.375 | 0.816 | 4.060 | 2.516 | 2.483 | 13.452 | 0.599 | 14.472 |
| 2×8 | catalog | SDK early, fused | 4.069 | 9.030 | 3.573 | 0.805 | 4.096 | 2.738 | 2.567 | 13.704 | 0.530 | 14.483 |
| 2×8 | catalog | SDK late | 6.622 | 9.419 | 4.476 | 0.786 | 4.043 | 2.622 | 0.000 | 13.215 | 0.576 | 16.978 |
| 2×8 | catalog | SDK zero budget | 15.641 | 12.327 | 0.800 | 4.374 | 7.069 | 2.348 | 0.000 | 28.167 | 0.592 | 28.991 |
| 2×8 | heldout | SDK early | 4.060 | 9.266 | 3.828 | 0.796 | 4.164 | 2.562 | 2.530 | 13.420 | 0.597 | 14.412 |
| 2×8 | heldout | SDK early, fused | 4.022 | 8.542 | 3.074 | 0.875 | 4.154 | 2.505 | 2.474 | 13.777 | 0.516 | 14.535 |
| 2×8 | heldout | SDK late | 6.748 | 9.190 | 4.425 | 0.789 | 3.968 | 2.656 | 0.000 | 13.655 | 0.588 | 16.874 |
| 2×8 | heldout | SDK zero budget | 15.703 | 12.565 | 0.784 | 4.744 | 7.020 | 2.427 | 0.000 | 28.024 | 0.593 | 28.870 |
| 1×16 | catalog | SDK early | 4.320 | 8.805 | 3.554 | 0.911 | 4.332 | 1.308 | 1.273 | 13.236 | 0.625 | 14.363 |
| 1×16 | catalog | SDK early, fused | 4.171 | 9.247 | 3.748 | 0.913 | 4.410 | 1.338 | 1.291 | 13.318 | 0.538 | 14.378 |
| 1×16 | catalog | SDK late | 5.770 | 9.703 | 4.342 | 0.915 | 4.278 | 1.224 | 0.000 | 13.106 | 0.591 | 16.463 |
| 1×16 | catalog | SDK zero budget | 13.796 | 12.094 | 0.883 | 4.843 | 6.486 | 1.590 | 0.000 | 25.555 | 0.593 | 26.362 |
| 1×16 | heldout | SDK early | 4.188 | 9.132 | 3.707 | 0.916 | 4.366 | 1.367 | 1.339 | 12.936 | 0.615 | 14.284 |
| 1×16 | heldout | SDK early, fused | 4.294 | 9.095 | 3.942 | 0.920 | 4.238 | 1.291 | 1.262 | 13.342 | 0.542 | 14.437 |
| 1×16 | heldout | SDK late | 5.669 | 10.276 | 4.339 | 0.915 | 4.401 | 1.439 | 0.000 | 13.253 | 0.591 | 16.881 |
| 1×16 | heldout | SDK zero budget | 13.624 | 11.555 | 0.837 | 4.527 | 6.843 | 1.451 | 0.000 | 24.144 | 0.601 | 25.388 |

All durations are milliseconds. Three trace blocks use five timed proofs per process. Eight timing blocks prime three proofs and time seventeen. All 336 C7a and eight local preflight exports passed independent verification. The temporary AWS resources were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.864 | 37 / 24 | 4.70× / 3.05× | [Scoped G2 transfer](../2026-11-20-scoped-g2-shape-transfer-c7a998/README.md) |
| 2×2 | 11.200 | 59 / 30 | 5.27× / 2.68× | This panel |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | [Grouped SDK widths](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 8×2 | 18.810 | 189 / 74 | 10.05× / 3.93× | [Solver widths](../2026-11-20-grouped-solver-widths-c7a1006/README.md) |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

Five catalog shapes meet 10× M5 CPU. The eight Metal comparisons remain below 10×. The selected profiles span the linked matched C7a runs.

[Summary](artifacts/summary.json) · [Samples and spans](artifacts/rows.json.gz) · [SDK patch](artifacts/source.patch.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
