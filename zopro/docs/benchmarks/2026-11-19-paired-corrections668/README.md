# Combine two sparse G2 corrections before weighting their buckets

The experimental mapped prover emits separate point and scalar lists for two levels of grouped bases. Each list is usually small. An optional proof-local slot allows whichever level finishes collecting its list second to compute both corrections together while the main query runs. The two lists retain separate buffers. Their matching window buckets merge before point weighting and the final doubling chain. The first list has a capacity-based memory reservation while it waits. A lone list completes at return and a denied reservation uses the separate-list path.

Eight and ten use the respective single-list window widths. Pair8 and pair10 use the corresponding shared reduction. Frozen is the original mapped executable. For each paired mode the middle comparison uses its same-width, same-binary separate-list control. For all other modes it uses off. Each original placement uses 30 hardware threads spanning sixteen physical cores on Ryzen. Two extra requests test sensitivity to the witness. The table reports medians of process means. Every process exported a proof that independently verified on the original circuit key.

| Input | Mode | Warm ms | Paired vs same-width separate or off | Paired vs frozen | Peak MiB | M5 CPU / Metal (10× CPU blocks) |
|---|---|---:|---:|---:|---:|---:|
| original0 | frozen | 28.609 | +0.09% (4/8) | +0.00% (0/8) | 540.3 | 9.93× / 3.53× (1/8) |
| original0 | off | 28.637 | +0.00% (0/8) | -0.08% (4/8) | 552.8 | 9.92× / 3.53× (3/8) |
| original0 | pair10 | 29.094 | +0.91% (1/8) | +0.78% (1/8) | 542.3 | 9.76× / 3.47× (0/8) |
| original0 | eight | 28.835 | +0.92% (3/8) | +0.55% (2/8) | 556.5 | 9.85× / 3.50× (0/8) |
| original0 | ten | 28.681 | +0.08% (4/8) | -0.04% (4/8) | 548.0 | 9.90× / 3.52× (3/8) |
| original0 | pair8 | 28.945 | +0.25% (4/8) | +1.92% (2/8) | 535.0 | 9.81× / 3.49× (0/8) |
| original4 | frozen | 28.679 | +0.23% (3/6) | +0.00% (0/6) | 549.9 | 9.90× / 3.52× (1/6) |
| original4 | off | 28.764 | +0.00% (0/6) | -0.22% (3/6) | 548.0 | 9.87× / 3.51× (0/6) |
| original4 | pair10 | 29.035 | +0.77% (3/6) | +0.90% (2/6) | 542.5 | 9.78× / 3.48× (0/6) |
| original4 | eight | 28.603 | -0.40% (4/6) | +0.23% (2/6) | 542.5 | 9.93× / 3.53× (1/6) |
| original4 | ten | 28.759 | +0.06% (3/6) | +0.34% (3/6) | 542.3 | 9.88× / 3.51× (1/6) |
| original4 | pair8 | 29.007 | +1.13% (1/6) | +1.30% (2/6) | 542.8 | 9.79× / 3.48× (0/6) |
| held58 | frozen | 28.472 | -0.76% (3/4) | +0.00% (0/4) | 559.3 | — |
| held58 | off | 28.677 | +0.00% (0/4) | +0.77% (1/4) | 528.8 | — |
| held58 | pair10 | 28.981 | +0.24% (2/4) | +0.32% (2/4) | 550.8 | — |
| held58 | eight | 28.615 | -0.40% (3/4) | +0.04% (2/4) | 547.6 | — |
| held58 | ten | 28.992 | +1.11% (1/4) | +0.66% (1/4) | 539.8 | — |
| held58 | pair8 | 28.860 | +0.88% (0/4) | +0.63% (1/4) | 543.0 | — |
| scattered139 | frozen | 44.337 | -1.70% (3/4) | +0.00% (0/4) | 952.8 | — |
| scattered139 | off | 45.156 | +0.00% (0/4) | +1.72% (1/4) | 1010.9 | — |
| scattered139 | pair10 | 46.082 | +1.15% (0/4) | +3.47% (1/4) | 958.5 | — |
| scattered139 | eight | 44.933 | -0.49% (2/4) | +0.38% (2/4) | 970.1 | — |
| scattered139 | ten | 45.413 | +0.47% (0/4) | +2.39% (1/4) | 1029.6 | — |
| scattered139 | pair8 | 44.836 | -0.04% (2/4) | +0.94% (2/4) | 996.3 | — |

Both original placements entered the combined correction on lists of 1,873 and 1,899 terms. Against separate lists at the same window width, the eight-bit pair won 5/14 blocks and the ten-bit pair won 4/14. Both combined modes had higher original medians than the frozen executable at both placements. The two combined modes also missed the 28.4 ms reference target in every original block. All 160 native core tests and 160 independent proof exports passed. Keep the separate-list implementation.

[Parent implementation](../2026-11-19-mapped-phi667/README.md) · [Incremental source](artifacts/source.patch) · [Dispatch evidence](artifacts/diagnostics.json) · [Configuration](artifacts/configs.json.gz) · [Timings and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
