# Routing the canonical G1 A query to dedicated C7a workers

The trial runs the A query on an existing caller-attached pool while keeping K/H and B2 on the main pool. The two pools use disjoint CPU IDs in the split configurations. The leading number in each split label gives the main-worker count. An off arm reserves the same cores for the frontend and runs A on the main pool. The group arm preserves the ordinary A grouping decision, and raw runs A directly. The route checks an extra workspace reservation. The same-binary G2 bypass is a separate per-key SDK setting for the native exclusive G2 query. It uses the same raw arithmetic as the default query's fallback.

The strongest dedicated A configuration by mean of four relative medians was 80-raw-bypass, at 7.762 / 7.712 / 7.660 / 7.685 ms across catalog, heldout, v25 and v26. The G2 bypass on the full 96-worker main pool measured 7.893 / 10.230 / 10.239 / 10.200 ms on those inputs. On catalog it measured 4.69× the supplied M5 CPU speed and 3.04× the Metal speed.

All timings came from one c7a.24xlarge. Prepared Aeglos used 96 physical cores and reported cache hits for every timed request. The full Zopro control used 96 main workers with the established overlapping 16-core frontend. The split layouts keep access to the same 96 physical cores. Five blocks per input used three primes and fifteen timed proofs per process. Twelve verification-only processes checked the routing decision with attached, detached and zero-budget configurations. Sixteen local exports exercised the SDK before the AWS run. A native G2 generator-oracle test and generic SDK example compilation passed. Gnark verified all 308 distinct exports. The C7a instance and temporary resources were removed.

### catalog

| Setting | Proof ms | Paired vs full control | Paired vs same placement | Paired vs Aeglos | Peak MiB |
|---|---:|---:|---:|---:|---:|
| release | 8.066 | +0.25% (1/5) | +0.25% (1/5) | -1.61% (3/5) | 46.7 |
| off | 8.032 | +0.00% (0/5) | +0.00% (0/5) | -2.26% (3/5) | 46.8 |
| bypass | 7.893 | -1.84% (5/5) | -1.84% (5/5) | -3.22% (5/5) | 46.8 |
| aeglos | 8.155 | +2.31% (2/5) | +2.31% (2/5) | +0.00% (0/5) | 459.2 |
| 80-off | 8.770 | +9.09% (0/5) | +0.00% (0/5) | +6.10% (0/5) | 40.0 |
| 80-group | 8.766 | +9.02% (0/5) | +0.29% (2/5) | +7.69% (0/5) | 39.2 |
| 80-raw | 7.773 | -3.21% (5/5) | -11.11% (5/5) | -4.87% (5/5) | 40.8 |
| 80-raw-bypass | 7.762 | -3.66% (5/5) | -11.25% (5/5) | -4.71% (5/5) | 43.7 |
| 72-off | 8.892 | +10.71% (0/5) | +0.00% (0/5) | +9.91% (0/5) | 34.8 |
| 72-raw | 7.914 | -1.76% (5/5) | -11.26% (5/5) | -2.96% (5/5) | 43.0 |
| 72-raw-bypass | 7.857 | -2.01% (5/5) | -11.65% (5/5) | -4.07% (5/5) | 43.8 |
| 64-off | 9.203 | +14.58% (0/5) | +0.00% (0/5) | +13.98% (0/5) | 31.0 |
| 64-raw | 8.789 | +9.42% (0/5) | -4.84% (5/5) | +6.92% (0/5) | 46.0 |
| 64-raw-bypass | 8.623 | +7.35% (0/5) | -6.46% (5/5) | +5.78% (0/5) | 46.7 |

### heldout

| Setting | Proof ms | Paired vs full control | Paired vs same placement | Paired vs Aeglos | Peak MiB |
|---|---:|---:|---:|---:|---:|
| release | 11.053 | -0.81% (5/5) | -0.81% (5/5) | +35.87% (0/5) | 40.0 |
| off | 11.098 | +0.00% (0/5) | +0.00% (0/5) | +37.17% (0/5) | 40.9 |
| bypass | 10.230 | -8.08% (5/5) | -8.08% (5/5) | +25.24% (0/5) | 42.3 |
| aeglos | 8.128 | -27.10% (5/5) | -27.10% (5/5) | +0.00% (0/5) | 458.6 |
| 80-off | 11.050 | -0.76% (5/5) | +0.00% (0/5) | +36.25% (0/5) | 39.3 |
| 80-group | 7.740 | -30.27% (5/5) | -29.94% (5/5) | -5.11% (5/5) | 43.0 |
| 80-raw | 7.711 | -30.51% (5/5) | -29.94% (5/5) | -4.46% (5/5) | 39.2 |
| 80-raw-bypass | 7.712 | -30.52% (5/5) | -30.12% (5/5) | -4.78% (5/5) | 40.0 |
| 72-off | 11.146 | +0.01% (2/5) | +0.00% (0/5) | +37.18% (0/5) | 31.7 |
| 72-raw | 7.870 | -29.41% (5/5) | -29.41% (5/5) | -3.21% (5/5) | 41.5 |
| 72-raw-bypass | 7.822 | -29.57% (5/5) | -30.02% (5/5) | -3.64% (5/5) | 44.4 |
| 64-off | 10.946 | -1.36% (5/5) | +0.00% (0/5) | +33.92% (0/5) | 28.1 |
| 64-raw | 8.730 | -21.70% (5/5) | -20.81% (5/5) | +7.41% (0/5) | 45.2 |
| 64-raw-bypass | 8.591 | -22.78% (5/5) | -21.78% (5/5) | +5.21% (0/5) | 46.0 |

### v25

| Setting | Proof ms | Paired vs full control | Paired vs same placement | Paired vs Aeglos | Peak MiB |
|---|---:|---:|---:|---:|---:|
| release | 11.147 | +0.37% (1/5) | +0.37% (1/5) | +34.23% (0/5) | 39.4 |
| off | 11.082 | +0.00% (0/5) | +0.00% (0/5) | +30.17% (0/5) | 41.5 |
| bypass | 10.239 | -8.06% (5/5) | -8.06% (5/5) | +23.22% (0/5) | 40.8 |
| aeglos | 8.309 | -23.17% (5/5) | -23.17% (5/5) | +0.00% (0/5) | 456.4 |
| 80-off | 11.117 | +0.55% (2/5) | +0.00% (0/5) | +33.38% (0/5) | 37.8 |
| 80-group | 7.737 | -30.10% (5/5) | -30.28% (5/5) | -6.85% (5/5) | 40.8 |
| 80-raw | 7.732 | -30.23% (5/5) | -30.21% (5/5) | -7.28% (5/5) | 40.7 |
| 80-raw-bypass | 7.660 | -30.85% (5/5) | -31.00% (5/5) | -7.98% (5/5) | 40.8 |
| 72-off | 11.068 | +0.30% (2/5) | +0.00% (0/5) | +32.76% (0/5) | 36.3 |
| 72-raw | 7.905 | -28.52% (5/5) | -28.62% (5/5) | -4.83% (5/5) | 45.2 |
| 72-raw-bypass | 7.818 | -29.66% (5/5) | -29.29% (5/5) | -5.88% (5/5) | 41.6 |
| 64-off | 11.041 | -0.37% (3/5) | +0.00% (0/5) | +31.40% (0/5) | 25.8 |
| 64-raw | 8.716 | -21.51% (5/5) | -21.05% (5/5) | +4.82% (0/5) | 47.6 |
| 64-raw-bypass | 8.531 | -23.21% (5/5) | -23.00% (5/5) | +1.17% (2/5) | 46.1 |

### v26

| Setting | Proof ms | Paired vs full control | Paired vs same placement | Paired vs Aeglos | Peak MiB |
|---|---:|---:|---:|---:|---:|
| release | 11.097 | +0.08% (2/5) | +0.08% (2/5) | +33.05% (0/5) | 41.5 |
| off | 11.088 | +0.00% (0/5) | +0.00% (0/5) | +32.29% (0/5) | 40.2 |
| bypass | 10.200 | -8.06% (5/5) | -8.06% (5/5) | +25.17% (0/5) | 42.3 |
| aeglos | 8.342 | -24.41% (5/5) | -24.41% (5/5) | +0.00% (0/5) | 457.0 |
| 80-off | 11.025 | -0.57% (4/5) | +0.00% (0/5) | +32.17% (0/5) | 37.7 |
| 80-group | 7.715 | -30.47% (5/5) | -29.92% (5/5) | -7.38% (5/5) | 42.3 |
| 80-raw | 7.707 | -30.56% (5/5) | -30.17% (5/5) | -7.05% (5/5) | 42.2 |
| 80-raw-bypass | 7.685 | -30.64% (5/5) | -30.30% (5/5) | -7.22% (5/5) | 44.6 |
| 72-off | 11.109 | +0.18% (1/5) | +0.00% (0/5) | +32.33% (0/5) | 32.6 |
| 72-raw | 7.883 | -28.73% (5/5) | -29.06% (5/5) | -5.71% (5/5) | 42.3 |
| 72-raw-bypass | 7.871 | -29.48% (5/5) | -29.17% (5/5) | -5.27% (5/5) | 43.7 |
| 64-off | 11.030 | -0.34% (5/5) | +0.00% (0/5) | +32.23% (0/5) | 26.5 |
| 64-raw | 8.698 | -21.29% (5/5) | -21.02% (5/5) | +4.27% (0/5) | 45.9 |
| 64-raw-bypass | 8.541 | -23.40% (5/5) | -22.56% (5/5) | +2.39% (1/5) | 46.7 |

[Samples and CPU masks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Implementation](artifacts/source.patch) · [Verification](artifacts/verification.json) · [Previous C7a timing experiments](../2026-11-19-kh-seed-c7a734/README.md)
