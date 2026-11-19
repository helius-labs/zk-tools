# A four-point circulant for Euler differentiation

At the center of the conjugated Euler transform the local coefficient indices are q, q+n/4, q+n/2 and q+3n/4. For the four input values x, the inverse transform, weighting and forward transform together produce output j equal to (4q+2n)x[j] + (n/2)(i(x[j−1]−x[j+1])−sum(x)). Indices wrap modulo four and i is the inverse transform's fourth root of unity. The experimental code evaluates this directly in each half of an eight-lane vector. It computes the fourth root once per transform, uses bounded integer multiplications through domain size 2²⁶ and retains the ordinary butterflies for larger domains. No tables are added.

Four randomized blocks compare the frozen nested-group build, the candidate with ordinary butterflies and the candidate using the circulant. Two additional modes record just the inner transform and weighting interval inside each proof. All requests use 32 Ryzen workers, three warmups and eleven timed SDK calls. The groups are fixed for 12×2 and 24×2. Merge uses its qualified Euler profile. Separate tests compare the native result with the unfused inverse/weight/forward transform and the direct coefficient oracle.

| Shape | Request | Frozen / ordinary / circulant ms | Circulant vs ordinary | Circulant vs frozen | Timed inner interval off / on ms | Peak MiB off / on |
|---|---|---:|---:|---:|---:|---:|
| 12x2 | Original | 29.610 / 29.640 / 29.866 | +0.94% (1/4) | +0.84% (1/4) | 3.673 / 3.705 | 563.2 / 580.6 |
| 24x2 | Original | 58.400 / 59.753 / 59.804 | +0.91% (2/4) | +1.28% (0/4) | 7.880 / 8.101 | 1073.5 / 1149.2 |
| merge-24x1 | Original | 64.417 / 64.130 / 63.654 | -0.06% (2/4) | -0.96% (4/4) | 8.439 / 8.457 | 1045.0 / 1059.2 |
| 12x2 | 142 | 46.521 / 45.367 / 46.001 | +1.40% (1/4) | -0.57% (4/4) | 3.372 / 3.575 | 1027.6 / 1026.7 |
| 24x2 | 103 | 58.452 / 58.472 / 59.107 | +0.92% (2/4) | +0.86% (2/4) | 7.996 / 8.018 | 1089.2 / 1158.5 |

Original speedups against M5 CPU / Metal were 12x2 9.51× / 3.38×, 24x2 8.83× / 2.94×, merge-24x1 7.96× / 2.75×. All 100 exported proofs verified. [Samples](artifacts/processes.json.gz) · [Source](artifacts/center.patch) · [Verification](artifacts/verification.json)
