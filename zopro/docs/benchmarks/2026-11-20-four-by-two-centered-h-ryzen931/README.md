# Centering 4×2 quotient scalars before window filling

Centered H took **13.883 / 13.938 ms** across the original and held-out proofs. Its paired changes were **+1.42% / +2.53%** against same-build ordinary and **+0.89% / +0.97%** against the explicit unsigned-width-12 control. Keep the ordinary H path.

The [work-count screen](../2026-11-20-four-by-two-h-representatives930/README.md) found that centering cuts leading-window arrivals by about half. The CPU trial transforms each scalar during the existing canonical conversion, using a spare bit for its sign. The 21 full windows keep 12-bit magnitudes, the same 4,096 bucket positions and two ranges of original points. The signed route folds negative coefficients into point selection. The leading window has magnitude zero or one and reduces the positive and negative index lists before subtraction. There is no retained point or digit table.

The exact indexed leading totals match the model at **32,836 / 32,869** on the two inputs. The prior ordinary totals were **65,087 / 65,037**. The separate H-span captures measured **8.305 / 8.393 ms** with ordinary and **8.322 / 8.372 ms** with centered. Those spans can include waiting and nested work.

The Ryzen run uses the pinned seeded 4×2 profile on 24 main and eight frontend hardware threads. Eight counterbalanced blocks per input prime three proofs and measure seventeen. Advancement requires a lower paired median and six wins or more against frozen, same-build ordinary and explicit unsigned12 on both inputs. Both same-build ordinary controls calibrated within 2%. The H spans come from a separate three-block capture with seven measured proofs per process. Peak RSS covers key loading and proving.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs explicit unsigned12 | H span ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | frozen | 13.629 | +0.00% (0/8) | +0.11% (4/8) | -0.91% (5/8) | — | 209.2 |
| catalog | ordinary | 13.636 | -0.11% (4/8) | +0.00% (0/8) | -0.61% (7/8) | 8.305 | 207.5 |
| catalog | unsigned12 | 13.725 | +0.92% (3/8) | +0.62% (1/8) | +0.00% (0/8) | 7.906 | 219.5 |
| catalog | centered | 13.883 | +1.77% (0/8) | +1.42% (0/8) | +0.89% (1/8) | 8.322 | 216.8 |
| heldout | frozen | 13.633 | +0.00% (0/8) | +0.12% (4/8) | -1.58% (7/8) | — | 232.2 |
| heldout | ordinary | 13.629 | -0.12% (4/8) | +0.00% (0/8) | -1.18% (7/8) | 8.393 | 214.5 |
| heldout | unsigned12 | 13.758 | +1.60% (1/8) | +1.20% (1/8) | +0.00% (0/8) | 8.489 | 262.2 |
| heldout | centered | 13.938 | +1.87% (1/8) | +2.53% (1/8) | +0.97% (2/8) | 8.372 | 218.0 |

The native tests check the field-order boundary and complete G1 weighted sums with signed, zero, identity and wide coefficients. Initial compile checks caught mismatched reference and curve types before any capture. Workspace denial verified with the existing fallback. All 88 proof exports independently passed Gnark verification.

The C7a score remains **5/8 at 10× M5 CPU**. The recorded scorecard includes CPU and Metal references for all eight shapes. [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings)

[Protocol](artifacts/protocol.json) · [Samples and route checks](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Trial source](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
