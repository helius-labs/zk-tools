# Mixed H window widths for 4×2 on Ryzen

The [uniform-width C7a trial](../2026-11-20-four-by-two-h-width-c7a897/README.md) reduced H point additions with signed-14 digits and increased proof latency. This trial keeps the 854 solver, quotient and curve placement profile. It changes the H scalar windows and keeps two point ranges. The two unsigned modes cover the 254 scalar bits in 21 passes. `merged-top` combines the leading two-bit and 12-bit windows into one 14-bit window. `rebalanced-top` uses two 13-bit leading windows instead of the original two 12-bit windows and the leading short window. The signed mode uses eight 14-bit and eleven 13-bit windows. All modes use the existing bucket fill, merge and weighted reduction, with no retained point table or digit buffer.

Ten route probes check the mixed layouts on both inputs and ordinary fallback at zero optional workspace. A native test checks all three layouts against a generator-weighted scalar oracle with signed, wide, zero and identity cases. All 162 full-proof exports independently verified. AWS authentication was expired, so the measurements use 24 main and eight frontend Ryzen logical CPUs.

The frozen profile took 13.641 / 13.615 ms on the original and held-out inputs. The lowest candidate median on the original input was `rebalanced-top` at 13.633 / 13.682 ms, with paired changes of +0.59% / +1.00% against ordinary. Every candidate fell short of the paired comparisons on at least one input. Each process warms three proofs and times 17. Eight paired timing blocks per input compare the original frozen executable, the 897 executable with its trial off, the new executable with the trial off, the uniform signed alternatives and the three new layouts. Advancement requires a lower paired median and six wins against each of the three ordinary controls on each input.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs parent | Paired vs ordinary | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | frozen | 13.641 | +0.00% (0/8) | -1.03% (6/8) | +0.69% (2/8) | 210.2 |
| catalog | parent | 13.692 | +1.04% (2/8) | +0.00% (0/8) | +1.04% (1/8) | 220.0 |
| catalog | ordinary | 13.564 | -0.68% (6/8) | -1.03% (7/8) | +0.00% (0/8) | 208.0 |
| catalog | signed13 | 13.570 | -0.40% (5/8) | -1.17% (6/8) | +0.11% (4/8) | 212.0 |
| catalog | signed14 | 14.003 | +2.87% (0/8) | +2.26% (1/8) | +3.29% (0/8) | 222.2 |
| catalog | merged-top | 13.747 | +0.92% (2/8) | +0.45% (2/8) | +1.24% (0/8) | 218.0 |
| catalog | rebalanced-top | 13.633 | -0.18% (5/8) | -0.20% (4/8) | +0.59% (1/8) | 215.2 |
| catalog | mixed-signed | 13.787 | +1.45% (2/8) | +0.21% (2/8) | +1.62% (0/8) | 214.5 |
| heldout | frozen | 13.615 | +0.00% (0/8) | +0.14% (3/8) | -0.06% (5/8) | 209.2 |
| heldout | parent | 13.607 | -0.14% (5/8) | +0.00% (0/8) | -0.09% (4/8) | 211.5 |
| heldout | ordinary | 13.562 | +0.06% (3/8) | +0.09% (4/8) | +0.00% (0/8) | 217.0 |
| heldout | signed13 | 13.589 | -0.33% (7/8) | -0.35% (5/8) | -0.46% (5/8) | 215.8 |
| heldout | signed14 | 14.055 | +3.19% (0/8) | +3.32% (0/8) | +3.23% (0/8) | 218.5 |
| heldout | merged-top | 13.654 | +0.72% (1/8) | +0.69% (2/8) | +0.66% (2/8) | 211.5 |
| heldout | rebalanced-top | 13.682 | +0.43% (1/8) | +0.58% (1/8) | +1.00% (3/8) | 224.2 |
| heldout | mixed-signed | 13.656 | +0.21% (3/8) | +0.17% (2/8) | +0.08% (4/8) | 222.0 |

The separate observer replays A, K and B2 only on exact coefficient matches. It counts the unchanged H computation in two processes per input and mode. Reported additions sum streaming updates, merge joins and weighted-tree pairs. The total excludes other projective work and the different costs of the operations. Reserved MiB is the experimental scratch forecast at the tested worker count. Peak RSS in the timing table covers the whole process.

| Input | Mode | Windows | Streaming | Merge | Tree | Counted adds | Paired-input change from ordinary | Reserved MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | ordinary | 22 | 1,868,495 | 85,993 | 233,738 | 2,188,226 | +0.00% | — |
| catalog | signed13 | 20 | 1,690,697 | 77,821 | 249,389 | 2,017,907 | -7.78% | 43.5 |
| catalog | signed14 | 19 | 1,455,192 | 146,705 | 370,362 | 1,972,259 | -9.87% | 81.4 |
| catalog | merged-top | 21 | 1,852,466 | 93,768 | 184,903 | 2,131,137 | -2.61% | 131.7 |
| catalog | rebalanced-top | 21 | 1,856,182 | 92,135 | 180,795 | 2,129,112 | -2.70% | 73.5 |
| catalog | mixed-signed | 19 | 1,627,505 | 109,243 | 214,902 | 1,951,650 | -10.81% | 75.5 |
| heldout | ordinary | 22 | 1,869,905 | 85,992 | 233,688 | 2,189,585 | +0.00% | — |
| heldout | signed13 | 20 | 1,691,990 | 77,823 | 249,493 | 2,019,306 | -7.78% | 43.5 |
| heldout | signed14 | 19 | 1,456,597 | 146,680 | 370,283 | 1,973,560 | -9.87% | 81.4 |
| heldout | merged-top | 21 | 1,853,909 | 93,755 | 184,896 | 2,132,560 | -2.60% | 131.7 |
| heldout | rebalanced-top | 21 | 1,857,622 | 92,139 | 180,794 | 2,130,555 | -2.70% | 73.5 |
| heldout | mixed-signed | 19 | 1,628,891 | 109,209 | 214,902 | 1,953,002 | -10.80% | 75.5 |

[Source](artifacts/source.patch) · [Observer](artifacts/meter.patch) · [Samples and counts](artifacts/samples.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md) still shows **5/8** original requests achieving 10× M5 CPU.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.858 | 37 / 24 | 4.71× / 3.05× |
| 2×2 | 12.029 | 59 / 30 | 4.90× / 2.49× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |
