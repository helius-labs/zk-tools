# SimdMSM paired/projective source audit

This read-only audit selects **no new runtime candidate**. SimdMSM shares routing for G1 and G2 terms with the same scalar digits, then calls separate arithmetic kernels for each group. Its affine-input wrappers return projective points. These mechanisms alone do not establish less work for Zopro's accepted BN254 path or explain the historical AWS difference.

Reference: [SimdMSM commit 5e28e01](https://github.com/JR-account/SimdMSM/tree/5e28e01f84f3c56e63952fd2099dabddc5b55809), directory AVX-MSM/demo/381. Four upstream files are hash-bound and remain private. One Rust/xtask audit verifies 196 pins, all 176 accepted source files, three active G2 formulas, two paired routes and five wrapper bodies. It builds no prover, imports no external C implementation and records no timing or proof. This revisits literature already considered in the [G1 kernel split](../2026-10-24-g1-kernel-split/README.md) and [PMNS study](../2026-10-23-pmns-field-component/README.md).

## Source findings

| Mechanism | Inspected behavior | Consequence |
|---|---|---|
| Paired routing | One scalar digit addresses a bucket containing G1 and G2 points | Shares control; arithmetic calls remain separate |
| Affine-input addition | Converts Montgomery representation, sets Z=one, returns x/y/z | The wrapper name does not imply an affine result or batch inversion |
| Pending work | Queues intermediate projective pairs and batches eight operations | Context and queue payload must accompany arithmetic costs |
| Mixed drain | Calls separate G1 and G2 mixed kernels | No shared cross-group field product is established |
| Weighted/window finish | Performs group-specific weighted and projective work | Pairing does not eliminate these operations |
| Accepted Zopro path | Omits B1 and uses native fused two-product Fq operations inside Fq2 multiplication | Pairing B1/B2 does not remove an existing accepted query; portable Karatsuba counts misprice the native path |

The primary [paired source](https://github.com/JR-account/SimdMSM/blob/5e28e01f84f3c56e63952fd2099dabddc5b55809/AVX-MSM/demo/381/src/pair_ifma.c#L657) shows separate G1/G2 calls. The [affine-input wrappers](https://github.com/JR-account/SimdMSM/blob/5e28e01f84f3c56e63952fd2099dabddc5b55809/AVX-MSM/demo/381/src/pair_ifma.c#L300) retain projective coordinates. Combining Zopro A and B2 instead would require checking their actual index sets, decomposition and bucket geometry; a shared scalar vector does not establish compatibility.

Active calls in [padd.c](https://github.com/JR-account/SimdMSM/blob/5e28e01f84f3c56e63952fd2099dabddc5b55809/AVX-MSM/demo/381/src/padd.c#L333), after excluding C comments and literals:

| G2 formula | Fq2 multiplications | Fq2 squares | Coefficient-specific calls |
|---|---:|---:|---:|
| Equal-Z projective | 5 | 2 | 0 |
| Mixed projective | 9 | 2 | 0 |
| Generic projective | 12 | 0 | 2 |

These are source call sites, **not dynamic costs, cycles or a speed prediction**. The generic formula's coefficient handling belongs to this BLS12-381 implementation. A BN254 derivation and exception handling would be required before treating it as a valid replacement. Zopro's ordinary affine fills amortize inversion over a batch; comparing an entire batch inverse with one projective addition would be misleading. No assertion that all projective algorithms are slower follows.

The upstream [types](https://github.com/JR-account/SimdMSM/blob/5e28e01f84f3c56e63952fd2099dabddc5b55809/AVX-MSM/demo/381/include/types.h) store nine field-coordinate arrays per projective G1/G2 pair, versus six for paired affine coordinates. Each array has HT_NWORDS u64 elements; its defining header was outside this audit. This coordinate count excludes queues, retained context capacity, input keys, temporaries and allocator overhead. No live-memory bound, RSS or minimum-RAM result is claimed.

## Decision and limits

No distinct work-removal mechanism is established that justifies an unchanged projective/paired implementation trial. The [G1 projective SDK](../2026-10-22-universal-g1-projective-reduction-sdk/README.md), [hybrid reduction component](../2026-10-23-hybrid-reduction-component/README.md), [two-chain trial](../2026-11-06-g2-two-chain-stream176/README.md) and [descriptor trial](../2026-11-06-g2-descriptor-chunks176/README.md) stay closed. The [operation model](../2026-10-25-operation-model/README.md) is historical rather than a current dynamic cost measurement. The [Aeglos source audit](../2026-11-06-aeglos-msm-mechanism176/README.md) likewise supplies no measured cause for the AWS gap.

The exact [FMA/RNE component](../2026-10-23-exact-fma-rne-component/README.md) already preserves Montgomery R=2^256 and failed its native eligibility tracks. Keeping that radix is therefore not a new FMA proposal, and no reroll follows. The [G1 code audit](../2026-11-06-g1-control-codegen176/README.md) remains a scoped code comparison without a causal alignment claim.

The full paper was unavailable behind the linked TCHES anti-bot page. Allocator/context headers, the complete exceptional-input path, subgroup assumptions, full call graph and BN254 port correctness were not audited. The helper verifies structural anchors and counts; no separate semantic reviewer is claimed. Source reading does not establish relative prover speed, CPU utilization or AWS scaling.

## Accepted comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references; no new measurement of any prover:

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No AWS resources, new Aeglos/Gnark comparison, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput measurements are added.

[Analysis](artifacts/analysis.json), [audit helper](artifacts/audit.rs), [design](artifacts/design.md), [reading scope](artifacts/reading.md), [pins](artifacts/pins.json), [workflow](artifacts/plan.json), [receipts](artifacts/receipts.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS). External source bytes and executables are excluded.
