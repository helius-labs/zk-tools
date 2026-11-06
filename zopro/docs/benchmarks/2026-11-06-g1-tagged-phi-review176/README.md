# G1 per-input endomorphism work model

**Close this proposal without a native algorithm or timing campaign.** On the accepted primary K/H input population, fewer GLV windows save dense weighted-tree work but increase generic fill joins and add 585,007 point-transform field multiplications. The simple six-multiplications-per-join proxy rises by 369,193, or **4.96%**. This is an arithmetic model, not measured CPU work or a latency bound. Production and accepted proof results remain unchanged.

The proposal would interleave signed G1 endomorphism components in ten borrowed windows, applying beta to tagged input X coordinates on demand. It would try to reuse exclusive scalar storage instead of allocating another scalar vector. Earlier implementations instead filled high-component buckets, transformed those buckets, then filled low components. The proposed placement could remove that phase boundary, but must pay input transforms and preserve all first-arrival, equality, opposite-point, queue, projective and tail semantics. No such runtime implementation was built.

## Actual primary inputs and modeled work

The pinned Euler-v2 image and existing primary request feed the original accepted solver and product FFT. The result has 30,529 K scalars and **30,922 used H scalars**, not the full 32,768-point domain. Production already selects that product-point prefix. Both point populations contain zero identities. Of these rows, 27,981 K and 30,531 H rows have a nonzero high GLV component.

The original route uses twenty balanced windows covering 255 bits. The proposed model uses ten covering 127 bits, one point partition and the unchanged accepted signed decomposition. Every scalar, signed component and digit reconstruction passes independent field/integer checks. There are **61,451 rows**, not proofs. Exact inputs, witness values, key/image bytes and scalar cache remain private; only their hashes and aggregate counts are published.

| Modeled quantity | Original | Tagged proposal | Difference |
|---|---:|---:|---:|
| Windows | 20 | 10 | −10 |
| Bucket capacity across windows | 71,680 | 34,816 | −36,864 |
| Nonzero input arrivals | 1,170,733 | 1,170,653 | −80 |
| Occupied buckets | 70,681 | 34,282 | −36,399 |
| Generic finite fill joins | 1,100,052 | 1,136,371 | **+36,319** |
| Dense weighted-tree affine joins | 140,560 | 68,272 | −72,288 |
| Dense weighted-tree layer invocations | 2,240 | 1,088 | −1,152 |
| Added beta field multiplications | 0 | 585,007 | **+585,007** |

Fewer occupied buckets also mean fewer free first insertions. Thus halving windows does not preserve the original fill-addition count. The six-multiplications-per-generic-join proxy combines fill and dense weighted joins, then charges one multiplication per tagged high arrival: 7,443,672 → 7,812,865. It excludes scalar inversions, root products, tails, projective operations, Horner assembly, recoding, admission, memory and scheduling. Its 4.96% increase is **not a predicted proof slowdown**, nor an upper bound on possible latency gains.

For comparison with the same-geometry phase placement, high arrivals touch 34,282 distinct buckets. That implementation transforms both affine and projective arrays. Even allowing one of each per occupied high bucket gives an upper bound of **68,564 transforms**, versus 585,007 input transforms here: at least **516,443 extra transforms** under this placement model. The analysis JSON's single-affine occupancy difference of 550,725 is not the complete phase comparison; the independent review explicitly charges both arrays. Queue/exception trajectories are not modeled as actual operations.

The weighted model follows the existing 256-bucket reduction structure. Its dense generic case has 502 pair joins, 496 full-vector lane pairs, six scalar-tail pairs and eight layer invocations per chunk. Identities and exceptional pairs change actual execution. These counts are not source-level byte traffic, cache behavior, CPU utilization or exclusive critical-path costs.

## Source, history and validation

Current `Window::add` checks incoming X before queueing and directly copies first arrivals. A transform tag only in the packer would therefore be incorrect. An implementation would need transformed equality/first-arrival/projective/tail behavior and could not retain a reference to a temporary transformed point. Scalar reuse also requires exclusive ownership; current borrowed/shared slice APIs cannot simply be mutated.

The [earlier G1 bucket-Horner trial](../2026-10-22-universal-g1-bucket-horner/README.md), [arena32 followup](../2026-10-22-universal-g1-horner-arena32/README.md), [borrowed fused K/H GLV trial](../2026-10-29-glv-kh-borrowed175/README.md), [deferred equality trial](../2026-10-31-deferred-equality176/README.md) and [four-word export screen](../2026-11-06-word-export176/README.md) remain closed. The borrowed GLV trial regressed against fresh OFF in all 31 blocks. Older preparation notes also reference prior per-point transform variants; their original temporary sources were unavailable in this workspace, so an exact historical match or exhaustive novelty audit is not claimed. No old width/partition/allocator experiment is rerun.

Two diagnostic snapshots each contain 177 files: 174 originals byte-exact, two append-only parents and one child. Reversing the suffixes recovers all 176 accepted files in each snapshot. Every original arithmetic/function body and field file stays exact. The accepted field and support libraries are reused without rebuilding; a new diagnostic core exports existing solve/product inputs and the unchanged split, without changing proving dispatch. The loader copies the accepted basis parser through Key construction, retaining its format/range/padding/digest/slot checks.

The initial diagnostic incorrectly asserted that used H length equaled the domain. Four preparation stages succeeded before the model stopped after one solve and FFT. The repair preserves those stages, witness bytes, source and receipts; it corrects only the diagnostic product-point prefix and model length accounting. Changed diagnostic code and its dependent executable are rebuilt. The repaired model necessarily performs a second solve/FFT because the failed process retained no inputs. Totals are **two solves and two product FFTs: one aborted model and one completed model**, one witness preparation, zero proof/MSM requests and zero new field/core test-suite runs. No successful unchanged build, witness stage, capture or review is repeated.

Independent review succeeds first time. It checks all pins, recovers 352 original source copies, validates all eight workflow receipts (seven successes and the retained failure), and independently reconstructs the private cache's scalar/decomposition/digit identities, every model row, dense weighted counts and both-array phase comparison. It does not regenerate the witness or rerun the solve/FFT.

This screen does not prove that every GLV algorithm is slower. It provides no credible dominant arithmetic reduction for this input-transform placement, while showing substantial added transform work relative to the earlier phase design. That is insufficient reason to spend another native campaign on it. No new 2× path, proof timing, memory peak/minimum RAM, pressure/swap, concurrent throughput, Aeglos/AWS, phone/ARM/TEE or huge-circuit qualification follows. The all-eight CPU10× objective remains active and unmet.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

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

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.



[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [design](artifacts/design.md), [question and history](artifacts/question.md), [source audit](artifacts/source-audit.json), [fixed diagnostic audit](artifacts/source-fixed-audit.json), [initial workflow](artifacts/plan.json), [repair workflow](artifacts/repair-plan.json), [pins](artifacts/repair-pins.json), [repair scope](artifacts/repair.md), [six exact changed-source members](artifacts/source-deltas.json.gz), [exact helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Unchanged base files are bound by their accepted-source manifest hashes; the two full source snapshots remain local. No private input-cache/witness/key/image bytes, executable/library blobs or performance recordings are included.
