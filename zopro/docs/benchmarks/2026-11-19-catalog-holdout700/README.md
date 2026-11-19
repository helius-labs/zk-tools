# Frozen 96-core configurations on a second transaction for every shape

The same Zopro configurations selected using the catalog transaction ran on one different transaction for each of the 41 shapes. Prepared Aeglos ran on each identical key, request and 96-core mask. Zopro recorded the lower median on **26/41** new transactions and won all four new paired blocks on **24/41**. It led on both the catalog and second transactions on **25/41** shapes.

Each new transfer has a separately generated owner, amounts, blindings and authenticated paths. The three merge fixtures change the input amounts, blinding values and external data hash. Gnark independently serialized all 41 public witnesses. Every new public hash differs from its corresponding catalog hash. Both provers first generated a locally verified proof for each new input. On the c7a.24xlarge, four fresh blocks alternated engine order per shape, used three warmups and timed seven sequential proofs per process. Local reverification checked all 328 AWS exports. Each timing is a median of process means and peak RSS covers setup. A second transaction per key samples one additional point in the input distribution.

| Shape | Fixed configuration | Catalog Zopro / Aeglos ms | Second transaction Zopro / Aeglos ms | Paired change (Zopro wins) | Peak MiB Zopro / Aeglos |
|---|---|---:|---:|---:|---:|
| 1x2 | pair-on | 8.001 / 8.384 | 11.035 / 8.292 | +33.70% (0/4) | 40.0 / 449.7 |
| 1x4 | bands-tree2 | 11.808 / 9.761 | 11.787 / 9.906 | +20.19% (0/4) | 109.6 / 582.0 |
| 1x8 | wide32 | 13.810 / 10.741 | 13.066 / 10.915 | +20.52% (0/4) | 73.0 / 683.1 |
| 1x16 | runtime-off | 16.741 / 12.343 | 16.879 / 12.569 | +36.01% (0/4) | 99.9 / 815.4 |
| 2x2 | bands-tree2 | 12.619 / 11.761 | 12.322 / 11.858 | +4.79% (0/4) | 122.4 / 793.5 |
| 2x4 | bands-tree2 | 13.896 / 12.357 | 13.802 / 12.318 | +12.81% (0/4) | 130.0 / 815.1 |
| 2x8 | wide32 | 15.585 / 15.297 | 15.356 / 15.081 | +0.19% (0/4) | 118.6 / 1012.6 |
| 2x16 | coeff-packed | 22.060 / 16.985 | 21.439 / 16.996 | +25.66% (0/4) | 210.1 / 1221.8 |
| 3x2 | runtime-off | 17.549 / 16.136 | 17.562 / 15.996 | +9.71% (0/4) | 178.8 / 1159.9 |
| 3x4 | coeff | 17.835 / 16.644 | 18.289 / 16.609 | +9.23% (0/4) | 175.7 / 1191.0 |
| 3x8 | runtime-off | 19.974 / 17.519 | 20.190 / 17.743 | +13.66% (0/4) | 200.4 / 1268.6 |
| 4x2 | coeff | 18.289 / 18.632 | 18.713 / 18.580 | +0.56% (2/4) | 199.1 / 1339.8 |
| 4x4 | coeff | 19.220 / 18.902 | 19.094 / 19.219 | +0.18% (1/4) | 197.5 / 1357.0 |
| 4x8 | front16 | 21.476 / 19.861 | 21.275 / 20.121 | +5.45% (0/4) | 226.8 / 1447.8 |
| 4x16 | front16 | 23.566 / 21.463 | 23.722 / 21.339 | +11.81% (0/4) | 282.2 / 1574.9 |
| 5x2 | coeff | 19.754 / 20.692 | 19.918 / 20.841 | -4.65% (4/4) | 223.0 / 1556.4 |
| 5x4 | coeff-packed | 20.994 / 21.003 | 21.443 / 21.146 | +0.82% (0/4) | 234.1 / 1561.4 |
| 5x8 | release | 24.895 / 26.261 | 24.811 / 26.316 | -5.20% (4/4) | 297.4 / 1932.5 |
| 5x16 | coeff-packed | 26.466 / 27.639 | 26.062 / 27.489 | -5.02% (4/4) | 305.4 / 2032.4 |
| 6x2 | release | 24.168 / 27.277 | 24.628 / 27.507 | -11.18% (4/4) | 316.3 / 2004.4 |
| 6x4 | release | 25.074 / 27.469 | 25.183 / 27.424 | -7.42% (4/4) | 326.5 / 2012.8 |
| 6x8 | release | 25.992 / 28.091 | 26.329 / 28.164 | -6.66% (4/4) | 334.1 / 2166.9 |
| 8x2 | release | 25.794 / 30.747 | 26.192 / 30.997 | -15.39% (4/4) | 364.4 / 2449.4 |
| 8x4 | release | 27.552 / 31.238 | 27.722 / 31.699 | -12.09% (4/4) | 378.8 / 2477.7 |
| 8x8 | release | 28.629 / 32.375 | 28.954 / 32.213 | -10.36% (4/4) | 404.0 / 2560.6 |
| 8x16 | release | 30.909 / 34.431 | 31.337 / 34.322 | -8.86% (4/4) | 430.7 / 2711.4 |
| 12x2 | release | 32.603 / 48.611 | 32.494 / 49.294 | -34.09% (4/4) | 503.5 / 3705.2 |
| 12x4 | release | 35.924 / 48.765 | 35.441 / 48.880 | -27.76% (4/4) | 524.5 / 3654.7 |
| 12x8 | release | 36.099 / 49.532 | 36.523 / 50.280 | -27.45% (4/4) | 561.3 / 3781.6 |
| 16x2 | release | 34.278 / 56.964 | 34.799 / 56.992 | -39.09% (4/4) | 596.7 / 4579.8 |
| 16x4 | release | 41.742 / 57.549 | 42.218 / 57.466 | -26.53% (4/4) | 682.3 / 4614.6 |
| 16x8 | release | 42.113 / 58.327 | 42.265 / 58.358 | -27.64% (4/4) | 687.2 / 4680.8 |
| 24x2 | release | 47.046 / 87.628 | 46.962 / 88.524 | -46.69% (4/4) | 925.3 / 6764.9 |
| 24x4 | release | 57.174 / 88.177 | 57.279 / 88.947 | -35.79% (4/4) | 1081.2 / 6751.3 |
| 32x2 | release | 67.100 / 102.224 | 66.861 / 102.403 | -35.07% (4/4) | 1302.5 / 8388.7 |
| 40x2 | release | 78.764 / 117.909 | 79.831 / 118.855 | -33.22% (4/4) | 1581.2 / 9700.9 |
| 48x2 | release | 103.680 / 166.687 | 106.217 / 167.224 | -36.48% (4/4) | 1866.0 / 12344.2 |
| 49x2 | release | 106.354 / 168.649 | 106.525 / 168.572 | -37.00% (4/4) | 1660.1 / 12596.7 |
| merge-8x1 | release | 28.546 / 29.924 | 28.999 / 29.848 | -2.66% (3/4) | 356.9 / 2349.6 |
| merge-24x1 | release | 51.427 / 85.223 | 48.707 / 85.197 | -42.69% (4/4) | 946.4 / 6587.8 |
| merge-54x1 | release | 102.684 / 173.714 | 95.739 / 173.307 | -44.83% (4/4) | 1704.8 / 12993.4 |

[Original confirmation](../2026-11-19-loss-routes689/README.md) · [Input identities](artifacts/inputs.json) · [Pinned workflow](artifacts/workflow.json.gz) · [Per-block observations](artifacts/rows.json.gz) · [Proof verification](artifacts/verification.json)
