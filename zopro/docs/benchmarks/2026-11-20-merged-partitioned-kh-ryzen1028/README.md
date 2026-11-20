# Merging fused K/H range buckets on Ryzen

Merging each window’s buckets before weighted reduction improved the four-range fusion prototype. The merged profiles still increased proof latency by 5.52–8.93% versus separate K and H, with zero wins in six paired blocks on either request. The trial remains isolated.

The prototype groups K scalars and streams multiple ranges of K and H. The merged modes retain each range’s affine buckets until they can combine them for one weighted reduction per window. The independent modes reduce each range first. The existing scratch budget admits each fused call.

| Shape | Request | Profile | Proof ms | Paired vs separate K/H | Paired vs independent ranges | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | Separate K and H | 15.069 | +0.00% (0/6) | — | 221.8 |
| 4×2 | catalog | Two independent ranges | 16.057 | +6.73% (0/6) | — | 158.2 |
| 4×2 | catalog | Merge two ranges | 15.895 | +5.52% (0/6) | -1.25% (5/6) | 146.0 |
| 4×2 | catalog | Four independent ranges | 16.288 | +8.41% (0/6) | — | 155.8 |
| 4×2 | catalog | Merge four ranges | 15.970 | +6.29% (0/6) | -1.86% (5/6) | 151.8 |
| 4×2 | heldout | Separate K and H | 15.082 | +0.00% (0/6) | — | 222.0 |
| 4×2 | heldout | Two independent ranges | 16.036 | +6.39% (0/6) | — | 141.5 |
| 4×2 | heldout | Merge two ranges | 15.839 | +5.62% (0/6) | -0.87% (6/6) | 144.8 |
| 4×2 | heldout | Four independent ranges | 16.327 | +7.13% (0/6) | — | 148.2 |
| 4×2 | heldout | Merge four ranges | 15.890 | +5.67% (0/6) | -2.14% (5/6) | 159.5 |
| 8×2 | catalog | Separate K and H | 24.394 | +0.00% (0/6) | — | 378.8 |
| 8×2 | catalog | Two independent ranges | 26.836 | +9.87% (0/6) | — | 264.0 |
| 8×2 | catalog | Merge two ranges | 26.710 | +8.84% (0/6) | -0.86% (4/6) | 293.8 |
| 8×2 | catalog | Four independent ranges | 26.877 | +9.45% (0/6) | — | 264.5 |
| 8×2 | catalog | Merge four ranges | 26.099 | +6.99% (0/6) | -3.01% (6/6) | 275.8 |
| 8×2 | heldout | Separate K and H | 24.813 | +0.00% (0/6) | — | 365.8 |
| 8×2 | heldout | Two independent ranges | 27.277 | +10.37% (0/6) | — | 263.5 |
| 8×2 | heldout | Merge two ranges | 26.973 | +8.93% (0/6) | -1.35% (5/6) | 270.2 |
| 8×2 | heldout | Four independent ranges | 26.986 | +8.60% (0/6) | — | 266.5 |
| 8×2 | heldout | Merge four ranges | 26.322 | +6.59% (0/6) | -2.53% (5/6) | 271.0 |

One native generator test covers signed and odd-length segments for both range counts. Six timing blocks each prime three proofs and time thirteen on 24 main and eight frontend Ryzen logical workers. Diagnostic probes check grouping and the selected reduction mode. Four more checks verify the fallback with zero scratch allowance. Gnark independently reverified all 144 exported proofs.

[Summary](artifacts/summary.json) · [Samples and route checks](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
