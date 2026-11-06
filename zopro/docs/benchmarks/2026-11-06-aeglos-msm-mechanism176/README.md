# Pinned Aeglos MSM mechanism audit

This read-only audit selects **no new Zopro implementation**. Aeglos's prepared Sorted engine combines shifted-window contributions before one weighted reduction. Its Windowed engine keeps separate window buckets and merges chunk results. Those are different work/memory tradeoffs. The audit does not measure their current performance or establish the cause of the AWS scaling difference.

Reference: [Aeglos commit 0290613](https://github.com/helius-labs/aeglos/tree/0290613e8eeca17a4ce88f6f2d726e8e06a5385d), the existing comparison reference. Four upstream files are hash-bound. All 176 accepted Zopro source files match the production manifest. The source-pinned Rust/xtask audit checks 195 pins, ten unique source anchors and four illustrative plan geometries once. It adds no runtime candidate, timing, profile, SDK request or proof.

## Work and storage

| Mechanism | Aeglos source behavior | Implication for this prover |
|---|---|---|
| Prepared bases | Stores the original and endomorphism image at every selected shift | Removes online shifts/window folds using retained expanded point data |
| Sorted accumulation | Counts/scatters descriptors, then reduces bucket-local pairs in rounds | Exposes independent bucket-block tasks; also needs descriptor and intermediate storage |
| Sorted weighted finish | All shifted windows share a bucket set | A single weighted reduction is enabled by the shifted basis; this saving cannot be transferred while omitting its preparation |
| Shared jobs | One pool pass spans all supplied MSM jobs, degree two first | Broadly overlaps the earlier shared-phase scheduling studies |
| Shared recoding/items | Eligible calls lend descriptors or share wire-prefix codes | Scalar/index/domain/window/infinity conditions matter; this does not share point sums or proofs |
| Windowed fallback | Codes each window, streams unshifted bases into chunk buckets, merges occupied buckets, then performs window weighting/Horner finish | Avoids all shifted rows but retains the window/chunk merge work and temporary workspace |
| Zopro small G2 | Two signed Phi components, 10 ordinary windows plus a narrow indexed top; two point partitions | Transforms bucket sums instead of retaining endomorphism/shifted key copies |

The [plan](https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm.hpp) chooses a width using its own cost model. The [constructor](https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm.cpp) normally limits shifted rows to the smaller of 8 GiB and one eighth of the native context budget; an explicit nonzero per-MSM limit replaces that default. Failure to fit selects the unshifted layout. That does not bound every host allocation or all simultaneously live preparation data.

For an **illustrative degree-two plan** with 30,089 rows, the formula selects width 13, ten windows and 4,096 buckets. The shifted base payload is 77,027,840 bytes (73.46 MiB), versus 7,702,784 bytes for its two unshifted base images. Unshared codes request 2,407,120 bytes; maximum item payload is 4,814,240 bytes; bucket sums request 524,288 bytes. These are source-derived requested payloads, not observed allocations, process RSS or minimum RAM. Original key ownership and other preparation/workspaces are additional.

| Illustrative rows | Workers | Sorted bucket-block tasks | Weighted SIMD tasks, assuming eight lanes | Shifted base payload MiB |
|---|---:|---:|---:|---:|
|30,089|32|256|8|73.46|
|30,089|96|512|24|73.46|
|52,274|32|256|8|127.62|
|52,274|96|512|24|127.62|

The row counts are published raw query lengths, **not captures of Aeglos's actual admitted/grouped plans**. The source formulas explain available task geometry, not utilization, imbalance, critical-path service time or speedup. In particular, more independent blocks alone do not establish why AWS is faster.

In [Sorted::accumulate](https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm_impl.hpp#L716), thread-local arena requests scale with that block's item count times affine-point size. The analysis's all-block arena quantity bounds the sum of these resize requests for the illustrative input. It is **not a maximum live-memory bound**: blocks need not overlap, and retained vector capacities depend on prior calls. Plan-owned codes/items/sums also retain capacity across calls.

The corresponding Zopro small-G2 geometry has 20 ordinary fill bucket sets of 2,048 slots, four signed indexed-top component reductions and 11 weighted calls covering 20,544 final bucket slots. Those counts describe its fixed structure, not an estimate that weighting dominates latency. Aeglos's projective range weighting and Zopro's affine weighted trees do not have interchangeable per-addition costs.

## History and research decision

The following earlier results remain closed; none is rerun or renamed as a new optimization:

- [Matched local profiles](../2026-10-22-aeglos-zopro-local-profiles/README.md) already identified common phase scheduling and recoding as leads.
- [Shifted-table ablation](../2026-10-22-aeglos-windowed-ablation/README.md) measured a material latency/retained-memory tradeoff. Its old timings are not current-source results.
- [Flat reductions](../2026-10-22-universal-flat-window-reduction-sdk/README.md) and [joint MSM phases](../2026-10-24-joint-msm-phases/README.md) failed their screens.
- [Shared witness classification](../2026-10-29-shared-wire-sdk175/README.md) reduced observed memory but lost full-proof speed. It is not identical to Aeglos wire-code sharing, and neither result proves the other implementation's cost.
- [Index-only G2 gathering](../2026-10-26-g2-index-gather/README.md), [joint weighted frontiers](../2026-10-31-weighted-frontier176/README.md) and [two streaming prefix chains](../2026-11-06-g2-two-chain-stream176/README.md) also remain unadopted.

The primary [SRE paper](https://arxiv.org/html/2610.27452v1) was rechecked in sections 4–6. It searches repeated scalar-window patterns before bucket insertion, and evaluates a single-core Python implementation. It is previously reviewed work, not a new discovery or a CPU speed prediction. Related bounded cross-window reuse already has a [current-fixture negative count screen](../2026-10-30-partial-windows175/README.md).

[Noël–Plantard](https://link.springer.com/article/10.1007/s13389-026-00393-z) was previously reviewed mainly through its abstract. This audit also visually checks the publisher's public Algorithm 4: each window traverses descending bands of at most d buckets, rescans input terms per band and retains its running weighted total across bands. This supplies a bounded-bucket memory tradeoff; it does not remove window contributions using shifted points. The full article prose was not accessible and the ePrint PDF returned 403. No reported paper gain is transferred to Zopro, and neither paper is implemented here.

The [Pool header](https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/pool.hpp) documents nonnested passes, but its implementation was not fetched in this audit. Source inspection supplies no new syscall, idle-time or CPU-utilization measurement. This audit closes without selecting an unchanged scheduling, inversion, gathering or expanded-key recipe.

## Accepted comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references. These are not new measurements of either prover.

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

Accepted merge54 remains 161.883899 ms with no supplied M5 reference; primary RSS132.75 MiB is inherited. The all-eight CPU10× objective remains active/unmet, **0/8**. The historical source175 c7a.24xlarge comparison remains Zopro11.915591 ms at32 workers versus Aeglos7.959184 ms at96. It does not measure accepted source176. No AWS resource, new proof, phone, pressure, swap, minimum-RAM or concurrent-throughput result is added; the distinct-proof ledger remains30,846.

[Analysis](artifacts/analysis.json), [source audit helper](artifacts/audit.rs), [design](artifacts/design.md), [reading scope](artifacts/reading.md), [input pins](artifacts/pins.json), [workflow](artifacts/plan.json), [receipts](artifacts/receipts.json), [publication identities](artifacts/publication.json), [checksums](SHA256SUMS). External source files and paper images stay private and are represented by hashes/primary links.
