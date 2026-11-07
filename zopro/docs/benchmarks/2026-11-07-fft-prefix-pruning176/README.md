# Euler FFT output-prefix dependency screen

**Simple natural-output prefix pruning removes no twiddle products on any of the nine supplied geometries.** It can remove final add/sub outputs, but not the multiplication work in the existing radix2 DIT dependency graph. No native kernel or new performance campaign is selected. This does not rule out other transform factorizations or output orders.

The current Euler path performs a full forward transform before consuming the first constraint-count values. Saved geometry receipts show every requested prefix exceeds half the padded domain. At the final butterfly level, that prefix includes every lower output. Each such output needs both previous-stage inputs and the shared twiddle product, so every preceding stage remains live. This explains why discarding a large output tail is not a proportional multiplication saving.

## Model results

Counts are per single scalar transform, not the compiled paired SIMD kernel. Unit-root products are excluded consistently; actual kernels may execute them. No measured CPU work, latency ceiling, memory saving or AWS comparison is inferred. Geometry counts are reused from the prior derivative study; no key/solver/FFT capture was rerun.

| Shape | Domain | Required outputs | Discarded | Nonunit products: full / prefix | Removed add/sub nodes | Share of full add/sub nodes |
|---|---:|---:|---:|---:|---:|---:|
| 12x2 | 524288 | 280242 | 244046 | 4456449 / 4456449 | 244046 | 2.450% |
| 16x2 | 524288 | 371122 | 153166 | 4456449 / 4456449 | 153166 | 1.538% |
| 1x2 | 32768 | 30922 | 1846 | 212993 / 212993 | 1846 | 0.376% |
| 24x2 | 1048576 | 554226 | 494350 | 9437185 / 9437185 | 494350 | 2.357% |
| 2x2 | 65536 | 53874 | 11662 | 458753 / 458753 | 11662 | 1.112% |
| 4x2 | 131072 | 98674 | 32398 | 983041 / 983041 | 32398 | 1.454% |
| 8x2 | 262144 | 189618 | 72526 | 2097153 / 2097153 | 72526 | 1.537% |
| merge-24x1 | 1048576 | 525759 | 522817 | 9437185 / 9437185 | 522817 | 2.493% |
| merge-54x1 | 2097152 | 1178199 | 918953 | 19922945 / 19922945 | 918953 | 2.087% |

The graph checks all 518 prefixes across domains 2–256. A separate forward evaluator checks 8379 requested values against direct polynomial DFTs over F65537, with checked root orders, full-width finite-field inputs, a last-coefficient basis input and near-modulus inputs. A one-output positive control does prune products, so the model is not hardcoded to report a full transform. These checks qualify a finite-field dependency model, not a BN254 runtime implementation.

Both inspected FFT sources match the accepted manifest. One compile, prepare, xtask model command and publication succeed once. There is no separate independent parser/reviewer. The actual prefix-product opportunity check fails without a percentage threshold; saved add/sub nodes are retained in the table. A transformed-output ordering, sparse-value specialization or alternative FFT algorithm would need a distinct algebra/cost/consumer study.

Accepted production176/SDK6d83, the 31,387-proof ledger and [all eight supplied M5 CPU and Metal comparisons](../2026-11-07-nonbinary-radix176/README.md#accepted-proof-baseline-unchanged) remain unchanged. Local accepted 1×2 is 7.915095 ms and merge54×1 is 161.718956 ms. The all-eight CPU10× goal is active and unmet (0/8). No new proof, native curve/field test, profile, device/pressure/minimum-RAM or throughput measurement is claimed.

[Design](artifacts/design.md), [model](artifacts/model.rs), [analysis](artifacts/analysis.json), [pins](artifacts/pins.json), [workflow](artifacts/plan.json), [command](artifacts/command.json), [exit receipt](artifacts/exit.json) and [checksums](SHA256SUMS).
