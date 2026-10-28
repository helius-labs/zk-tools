# First-layer triples in unweighted reductions

Primary1x2 has **24,524 ordinary G1 triples in indexed top-window tiles**, but only **two in affine-group reduction**. Larger shapes have substantial grouped G1 work. All three observations per shape match and no guard fires in these fixtures. The next relevant test is a complete G1 unweighted reduction at these sizes; this is **not a measured speedup**. [Interpretation](artifacts/interpretation.md).

This diagnostic counts where the tested three-point formula could apply to real reduction inputs. It changes no proof arithmetic and establishes **no speedup or production adoption**. The three-map consistency condition is **true**; all conservation and independent-proof checks pass.

Two hooks observe only the initial input layer in group_affine_impl and each reduce_indexed_buckets input tile. The former includes scalar-group reduction and the final reduction of indexed tile sums. Weighted trees, streaming flushes and later binary layers are excluded. The earlier [MSM geometry observer](../2026-10-28-msm-geometry/README.md) counted reduce_layer across weighted trees too, so its triple totals did not isolate these sites. No ternary execution or later-layer topology is simulated here.

Within each group, disjoint consecutive triples are classified by identity, equal first-pair X, zero H=(Qx−Px)²(Px+Qx+Rx)−(Qy−Py)², or ordinary. These are the input-known denominator guards of the [tested three-point formula](../2026-10-28-triple-affine-screen/README.md). Observation performs extra field arithmetic but no inversion or point mutation. No coordinates, scalars or key points are exported. Binary slots include exceptional pairs; triple slots include guards. They are not counts of executed ordinary additions or saved inversions.

Twelve fresh processes run three repetitions of1x2,2x2,16x2 and merge54 at32workers/CPUs0–31, rotating/reversing case order. Each makes three unobserved primes then one observed, internally verified SDK request. All12 captures finish before independent verification and analysis. No owned build/helper/hash/archive/profile/verifier/second benchmark overlaps capture. Diagnostic latency, CPU and RSS are not measured as performance results.

| Case | Three arrays identical |
|---|---|
| 16x2-w32 | true |
| 1x2-w32 | true |
| 2x2-w32 | true |
| merge-54x1-w32 | true |

Counts below are per observed SDK request. Identical repetitions are shown once; if a case varies, every observation is shown.

| Case | Observation | Context | Input points | Binary slots | Triple slots | Ordinary triples | Identity | Equal X | H zero | Jobs ≥8 ordinary | Max triples/job |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2-w32 | 0 | affine_group/G1 | 571330 | 259871 | 160461 | 160461 | 0 | 0 | 0 | 152 | 5104 |
| 16x2-w32 | 0 | affine_group/G2 | 338658 | 153873 | 94926 | 94926 | 0 | 0 | 0 | 87 | 4530 |
| 16x2-w32 | 0 | indexed_tile/G1 | 315814 | 157906 | 105168 | 105168 | 0 | 0 | 0 | 158 | 682 |
| 16x2-w32 | 0 | indexed_tile/G2 | 17554 | 8777 | 5845 | 5845 | 0 | 0 | 0 | 18 | 341 |
| 1x2-w32 | 0 | affine_group/G1 | 204 | 5 | 2 | 2 | 0 | 0 | 0 | 0 | 2 |
| 1x2-w32 | 0 | affine_group/G2 | 448 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 1x2-w32 | 0 | indexed_tile/G1 | 73773 | 36834 | 24524 | 24524 | 0 | 0 | 0 | 203 | 682 |
| 1x2-w32 | 0 | indexed_tile/G2 | 55277 | 27525 | 18269 | 18269 | 0 | 0 | 0 | 443 | 54 |
| 2x2-w32 | 0 | affine_group/G1 | 83436 | 32417 | 1970 | 1970 | 0 | 0 | 0 | 9 | 482 |
| 2x2-w32 | 0 | affine_group/G2 | 49959 | 19198 | 1023 | 1023 | 0 | 0 | 0 | 5 | 407 |
| 2x2-w32 | 0 | indexed_tile/G1 | 84528 | 42263 | 28148 | 28148 | 0 | 0 | 0 | 46 | 682 |
| 2x2-w32 | 0 | indexed_tile/G2 | 57135 | 28460 | 18889 | 18889 | 0 | 0 | 0 | 442 | 54 |
| merge-54x1-w32 | 0 | affine_group/G1 | 1819089 | 838199 | 547376 | 547376 | 0 | 0 | 0 | 195 | 11991 |
| merge-54x1-w32 | 0 | affine_group/G2 | 1078251 | 496304 | 324043 | 324043 | 0 | 0 | 0 | 111 | 10212 |
| merge-54x1-w32 | 0 | indexed_tile/G1 | 903903 | 451949 | 301007 | 301007 | 0 | 0 | 0 | 446 | 682 |
| merge-54x1-w32 | 0 | indexed_tile/G2 | 38855 | 19427 | 12939 | 12939 | 0 | 0 | 0 | 38 | 341 |

For every cell, ordinary+identity+equalX+H0 equals triple_slots;3×triple_slots+remainder_points equals input points, and the group-size histogram sums to groups. Full histograms, guard counts, vector-opportunity jobs, maxima and all12 raw arrays remain in analysis.json. A job with eight ordinary triples has one native-width opportunity, not a qualified caller. Repacking, filtering, new records, scalar tails, exceptional fallback, later-layer changes and resource accounting remain unmeasured. Counts cannot establish a CPU saving or twofold proof gain.

The observer uses4×32AtomicU64 counters (1024bytes) plus an active gate and fixed32u64 local accumulation per hook. It allocates no maps or journals inside observation. JSON/export happens afterward. This is a diagnostic storage description, not total stack/RSS/minimum RAM. Begin/end occur at quiescent SDK boundaries after joined worker work.

Native validation passes124 unfiltered core and8 prover tests. Two added tests check real G1/G2 guard cases, disabling/reset, concurrent accumulation and slot/histogram conservation. Field source and qualification are inherited, not rerun. Some existing optional missing-fixture scenarios are not claimed. A175-file private copy changes two original files by observer hooks/module/reexport;172 originals remain exact and inverse edits recover174 originals. Existing reduction expressions and all production source remain unchanged. Coherent native libraries, proc macros, actual flags/features and closing pins are checked.

All48 SDK calls verify internally. Twelve distinct164-byte exports independently pass Gnark/public binding/freshness,24 proof/public members, ledger15541→15553. All captures precede all independent checks and hashes. There is no optimization or new benchmark binary adopted.

The [small native triple caller](../2026-10-28-triple-callers/README.md) retained useful component results, while the [mixed streaming integration](../2026-10-28-mixed-sdk/README.md) regressed and remains closed. This census concerns pre-grouped inputs and avoids claiming that queue-free integration will inherit those component gains. A future implementation needs actual caller timing, complete proof checks and existing memory admission; the current weighted binary recurrence cannot simply use three-way compaction.

Source review of pinned Aeglos029 confirms that its prepared Sorted engine stores shifted GLV bases and merges windows into one bucket set; Windowed is its fallback. The [existing crosshost ablation](../2026-10-23-aeglos-shifted-crosshost/README.md) already measures that table/algorithm tradeoff. The [G1 half-shift](../2026-10-23-shift128-g1-component/README.md), [G2 shift](../2026-10-23-shift64-g2-component/README.md) and [indexed follow-up](../2026-10-23-shift64-indexed-component/README.md) remain rejected. No new lookup table or Aeglos speed claim is introduced. Inspected primary sources: [MSM plan](https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm.hpp), [MSM implementation](https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm_impl.hpp). Only URLs and hashes are archived; full third-party source stays local.

The accepted all-eight table below is inherited, not measured by this observer. M5 values are supplied historical references. Accepted174/SDK81033 and primary8.366931ms remain; the all-eight10× CPU goal is active/unmet,0/8. Historical adoption exceptions remain in the [adoption report](../2026-10-28-g2-borrowed-adoption/README.md). There is no54 M5 reference, new AWS/Aeglos, phone/ARM/TEE, giant, concurrent-proof, pressure, swap or minimum-RAM qualification.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 12x2 | 34.858408 | 284.0 | 8.147× | 101.0 | 2.897× |
| 16x2 | 41.377043 | 335.0 | 8.096× | 124.0 | 2.997× |
| 1x2 | 8.366931 | 37.0 | 4.422× | 24.0 | 2.868× |
| 24x2 | 67.764001 | 528.0 | 7.792× | 176.0 | 2.597× |
| 2x2 | 11.539335 | 59.0 | 5.113× | 30.0 | 2.600× |
| 4x2 | 16.540233 | 109.0 | 6.590× | 48.0 | 2.902× |
| 8x2 | 25.136722 | 189.0 | 7.519× | 74.0 | 2.944× |
| merge-24x1 | 67.096516 | 507.0 | 7.556× | 175.0 | 2.608× |

[Protocol](artifacts/design.md), [all counts](artifacts/analysis.json), [metrics](artifacts/metrics.json), [source audit](artifacts/source-audit.json), [native build](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [closing pins](artifacts/pins.json).
