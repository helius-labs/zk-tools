# Canonical H coefficients at quotient emission on C7a

Both eager encodings fell short of the matched full-proof requirement.

The test uses the [same executable and arithmetic as the Ryzen trial](../2026-11-20-eager-h-canonical-ryzen953/README.md). The emitter writes canonical limbs into its existing buffer, which the H multiscalar multiplication consumes directly. Zopro uses 64 main and 32 frontend workers and the verified 16-core solver placement. The frozen control is the exact earlier 16-core solver program.

Each input has twelve counterbalanced timing blocks. Each process primes three proofs and measures seventeen. The table reports medians of the process means and paired changes. A path qualifies when ordinary calibrates within 2% of frozen and the path has a lower paired median and at least nine wins against both controls on each input.

| Input | Frozen ms | Ordinary ms | Eager scalar ms | Eager packed ms | Packed vs ordinary | Packed vs frozen | Packed peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 14.797 | 14.765 | 14.748 | 14.936 | +0.36% (5/12) | +0.21% (6/12) | 221.5 |
| heldout | 14.724 | 14.735 | 14.721 | 14.754 | -0.35% (7/12) | +0.31% (4/12) | 219.3 |

| Input | Scalar vs ordinary | Scalar vs frozen | Packed vs scalar | Ordinary vs frozen |
|---|---:|---:|---:|---:|
| catalog | -0.11% (6/12) | -0.58% (8/12) | +0.33% (5/12) | -0.37% (8/12) |
| heldout | -0.35% (8/12) | +0.32% (5/12) | -0.20% (7/12) | +0.44% (4/12) |

The packed path used 644.5 and 641.4 CPU-ms per proof on the original and held-out inputs, compared with 641.0 and 639.8 for same-build ordinary. Its median minor faults fell from 1,151 and 1,132 per proof to 1,075 and 1,051. The counters cover the complete timed proof loops.

All 96 C7a proof exports were independently verified after retrieval. The temporary instance and bucket were removed, with zero tagged volumes or active benchmark instances left in the test region.

## Eight supplied M5 references

Five of eight shapes reach 10× against M5 CPU. [Baseline measurements](../2026-11-20-quartet-placement-factorial-c7a950/README.md#eight-supplied-m5-references)

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.736 | 37 / 24 | 4.78× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.457 | 109 / 48 | 7.54× / 3.32× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup](artifacts/cleanup.json)
