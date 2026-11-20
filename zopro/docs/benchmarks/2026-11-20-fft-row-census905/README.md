# Whole-pack and per-lane pruning in the Euler quotient FFT

The folded quotient needs the Euler derivative of A where B is nonzero, and the derivative of B where A is nonzero. This study checks whether that asymmetry or exact equality in the inverse butterflies can remove enough field products to justify a specialized FFT kernel. It uses four public A/B captures for each supplied circuit. The captured rows include ordinary constraint and public rows. Zeros extend them to the image's exact FFT domain.

In inverse DIF, a difference of zero or ±1 can replace a twiddle product with zero or a signed root. The whole-pack column counts groups of four adjacent rows across both channels for which all eight differences meet that condition. The ideal-lane column assumes compaction is free and counts only non-unit twiddles, across all inverse layers. The forward DIT scan starts from the other input's actual nonzero positions and propagates the required derivative lanes backwards. A butterfly can omit its twiddle when both outputs on that channel are unused. The whole-pack columns count the first five outer layers, from the input side for inverse and from the output side for forward. All percentages are medians over the four public requests.

| Shape | Domain | Inverse trivial whole packs | Inverse removable lanes | Forward unused whole packs | Forward removable lanes |
|---|---:|---:|---:|---:|---:|
| 1×2 | 32,768 | 0.039% | 0.024% | 0.000% | 0.013% |
| 2×2 | 65,536 | 0.101% | 0.083% | 0.000% | 0.039% |
| 4×2 | 131,072 | 1.561% | 0.550% | 0.176% | 0.284% |
| 8×2 | 262,144 | 1.090% | 0.374% | 0.133% | 0.211% |
| 12×2 | 524,288 | 1.786% | 0.575% | 0.207% | 0.323% |
| 16×2 | 524,288 | 1.131% | 0.361% | 0.131% | 0.202% |
| 24×2 | 1,048,576 | 1.858% | 0.567% | 0.216% | 0.319% |
| merge-24×1 | 1,048,576 | 1.981% | 0.590% | 0.231% | 0.333% |

Across the twelve captures for the three shapes still short of the target, at most 0.550% of non-unit inverse products and 0.285% of non-unit forward products are eligible for elimination even with free lane compaction. The ordinary input already makes most outer butterflies necessary for both derivatives. The counts leave little room for this particular kernel change. They exclude any benefit from other ways to represent the quotient and any costs of testing or compacting lanes. Counts alone do not measure latency.

The reader checks 512 field outputs against direct DFT sums with roots from the pinned key images. A separate residue-class derivation reproduces all 576 forward per-lane layer counts. The combined-channel outer pack counts match all 160 corresponding records in the [prior output support scan](../2026-11-19-residual-support581/README.md). All 32 row captures match the archived independent verifier receipts and proof ledger. This analysis made no new proof or cloud calls. AWS authentication was expired at the start of the experiment.

[Reader](artifacts/census.rs) · [Counts and identities](artifacts/counts.json.gz) · [Summary](artifacts/summary.json) · [Reproduction details](artifacts/reproduction.json)

## Eight supplied M5 reference timings

The [recorded 96-core C7a profiles](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) reach 10× M5 CPU on **5/8** original requests.

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
