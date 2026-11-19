# Outer-worker idle policy on held-out 24×2 proofs

The 24×2 original was the one 32-worker shape with four paired wins for the outer-only policy in the preceding experiment. This confirmation uses five new randomized blocks on the same original and eight different public requests. The selected policy allows five failed search rounds in the outer worker loop and retains the upstream 32 rounds while a worker waits inside a task. At each setting the same executable runs either with ordinary queries or with the fixed prepared A/K/B2 point groups from the earlier larger-transfer study. The official release is a separate control. Each process primes three calls and times eleven. Peak RSS includes preparation.

| Request | Release / ordinary / outer / grouped / grouped outer ms | Outer vs ordinary | Grouped outer vs grouped | Peak MiB ordinary / outer / grouped / grouped outer |
|---|---:|---:|---:|---:|
| Original | 63.448 / 63.194 / 63.792 / 60.765 / 58.903 | +1.06% (1/5) | -2.03% (4/5) | 1036 / 856 / 1101 / 874 |
| 97 | 63.501 / 63.037 / 63.437 / 59.293 / 59.525 | -0.20% (4/5) | +0.43% (2/5) | 1045 / 856 / 1120 / 914 |
| 98 | 64.136 / 63.804 / 63.733 / 58.867 / 58.129 | +0.40% (2/5) | +0.37% (2/5) | 1019 / 868 / 1130 / 865 |
| 99 | 64.266 / 64.886 / 63.149 / 58.432 / 57.880 | -2.20% (4/5) | -1.30% (5/5) | 1011 / 844 / 1153 / 847 |
| 100 | 64.525 / 63.035 / 63.933 / 58.237 / 58.095 | +1.42% (1/5) | +1.08% (1/5) | 1015 / 873 / 1099 / 870 |
| 101 | 64.516 / 63.822 / 64.743 / 59.144 / 58.175 | +0.36% (2/5) | -2.22% (4/5) | 1030 / 893 / 1165 / 835 |
| 102 | 63.889 / 63.788 / 63.312 / 59.023 / 58.387 | -0.79% (4/5) | -0.68% (3/5) | 1006 / 857 / 1112 / 882 |
| 103 | 64.658 / 63.453 / 64.267 / 58.460 / 58.539 | +1.32% (1/5) | -1.02% (3/5) | 1030 / 857 / 1111 / 869 |
| 104 | 65.101 / 63.483 / 63.548 / 59.558 / 58.207 | -0.06% (3/5) | +1.23% (2/5) | 1039 / 851 / 1129 / 913 |

The original grouped outer setting reached 58.903 ms, or 8.96× M5 CPU and 2.99× M5 Metal. 0/5 process means reached 52.8 ms. All 225 process exports verified. [Samples](artifacts/processes.json.gz) · [Plans and fixtures](artifacts/inputs.json) · [Verification](artifacts/verification.json)
