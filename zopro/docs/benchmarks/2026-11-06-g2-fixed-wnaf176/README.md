# Fixed-block odd recoding: indexed-top follow-up audit

**Not selected for native implementation.** With the indexed leading window and skew correction included, this fixed recipe increases modeled point-add calls by **4.37% on 1×2** and **4.34% on grouped 2×2**, while active doublings remain unchanged. This is an integer work model, not a measured slowdown or a rejection of every odd-digit algorithm. Production and accepted proof performance remain unchanged.

The [November5 fixed-window odd screen](../2026-11-05-g2-fixed-odd-screen176/README.md) had already rejected this recoding family on primary inputs. That report was missed when this work started. This is a follow-up audit, not a new optimization lead: it adds the actual Booth reader, indexed-top/weighted-tree model, exact low/skew cancellation and grouped2×2 population, and confirms the earlier negative decision. Frozen protocols and initial notes are preserved with a [history correction](artifacts/history-correction.md); they must not be read as evidence that fixed odd recoding was previously untested.

## Construction and comparator

The primary reference is [libsecp256k1's fixed-block recoder and Pippenger accumulator](https://github.com/bitcoin-core/secp256k1/blob/master/src/ecmult_impl.h), `secp256k1_wnaf_fixed` and `secp256k1_ecmult_pippenger_wnaf`. Our independent Rust construction uses eleven radix4096 positions for each existing 126-bit G2 component. Its nonzero digits are odd with magnitude at most4095; even nonzero magnitudes need a unit correction. This differs from the prior [bitwise NAF screen](../2026-11-05-g2-wnaf-screen176/README.md), which used 127 positions. No external runtime code or secp256k1 performance result is imported.

Odd digits do not halve this recipe's ordinary bucket capacity: magnitudes through 4095 still require 2,048 positive-magnitude buckets, the same count as the original radix12 path. Before counting, the selected recipe cancels a unit low digit against its own unit skew, removing both point entries exactly. The literal construction remains in the results as context. No further width, lattice, subset or layout variant is selected after seeing the counts.

The comparator uses the **exact production overlapping Booth reader**, not an ordinary balanced-radix approximation. Both recipes retain the existing Phi2 split, two point partitions and both component signs. Primary uses the pinned 30,089 raw points. The2×2 mathematical grouping contains 28,852 scalar classes derived from 52,274 raw points; its scalar digest and inherited zero-identity evidence match the prior qualified population. This data-only grouping does not replay resource or prepared-key admission.

## Complete counts

The [prospective protocol](artifacts/count-protocol.md) includes all eleven positions, first writes, partition merges, signed indexed leading reductions, negative folds and component merges. It models the current 256-bucket weighted tree's correction segments and group offsets. The original leading window has 64 magnitude positions; the fixed recipe has 32. Neither is charged as an ordinary dense2,048-bucket leading window.

The fixed weighted combiner can retain weighted sum W and total T, then use eleven doublings, add W, double and subtract T in Horner. This computes 4096R+2W−T with twelve invoked Horner doublings per position. The extra point additions and active leading doubling are counted; no independent extra doubling is incorrectly charged on top of that schedule.

| Population | Original point-add calls | Selected fixed recipe | Increase | Active doublings, both | Remaining skew entries | Exact low/skew cancellations |
|---|---:|---:|---:|---:|---:|---:|
|1×2|634,486|662,189|27,703 /4.37%|846|28,159|7|
|Grouped2×2|654,290|682,672|28,382 /4.34%|846|28,880|8|

| Population | Original total point arrivals | Fixed arrivals, including correction | Original leading fill/merge pairs | Fixed leading fill/merge pairs |
|---|---:|---:|---:|---:|
|1×2|613,891|641,611|55,221|54,783|
|Grouped2×2|633,695|662,094|57,079|56,616|

The smaller leading window saves some work, but the remaining correction dominates. Literal fixed recoding uses 662,203 /682,688 modeled point-add calls, so the declared cancellation saves 14 /16 calls. The selected recipe's active doubling count equals the original; invoked doublings are 865versus866because the smaller leading tree loses one level. That is not a cycle estimate.

The necessary screen requires fewer complete modeled point-add calls and no more active doublings on **both** populations. Both cases fail. No native kernel, query benchmark or full SDK follow-up is warranted by this count result. No threshold, successful stage or unchanged experiment is rerun.

## Qualification and limits

Two encoding tests pass, covering exhaustive small inputs, all component bit boundaries,50,000 random full-width magnitudes, carry patterns and both signs. Three new model tests check exact low/skew cancellation, weighted/total coefficients across sparse/dense masks and complete leading/correction conservation. The original two tests are filtered during that second suite. Six workflow commands complete on their first executions.

All 117,882 actual component magnitudes reconstruct independently in the original, literal and selected encodings. Original ordinary arrivals and first-write counts reproduce the prior independently qualified operation census. A separate checker imports neither the candidate recoder nor the reduction model: it uses quotient-parity recoding, a direct Booth formula and a closed block-graph count of correction trees. It re-reads the pinned scalar fixtures solely for independent review, reconstructs all 66 position rows, histogram hashes, costs and the failed selection. It verifies 22 pin occurrences, all six command argv/exits/stream hashes, and all 176 production source hashes unchanged. Review passes on its first execution; no original counter, capture or successful test is repeated.

The model assumes noncancelling bucket sums except the declared identical low/skew pair. It omits actual equal-X exceptions, queue/projective-spill behavior, inversion/normalization counts, SIMD tails, coordinate conversion, field-cycle weights and scheduling. Projective add-call counts do not specify exact curve formulas. Therefore this is neither a latency prediction nor a lower bound over other implementations. No new curve oracle, proof, native field/prover suite, CPU profile or memory-pressure qualification is claimed. Prior Phi2 and point/grouped-identity qualification is inherited.

Only temporary integer histograms and existing private fixture data are used. No runtime digit matrix, point lookup table, key copy, duplicate plan, pool or fixed RAM ceiling is proposed. No SDK/Gnark request or AWS resource is created; the verified proof ledger remains 30,846.

## Accepted comparisons remain unchanged

Inherited local 32-worker full-proof values and supplied M5 references, not timings of this screen. Accepted source 176/SDK `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` remains unchanged. Different hardware does not establish equal hardware efficiency.

| Shape | Accepted CPU ms | M5 CPU ms | CPU/local | M5 Metal ms | Metal/local | CPU10× limit ms |
|---|---:|---:|---:|---:|---:|---:|
|1×2|7.928660|37|4.667×|24|3.027×|3.7|
|2×2|11.151932|59|5.291×|30|2.690×|5.9|
|4×2|16.086386|109|6.776×|48|2.984×|10.9|
|8×2|24.179453|189|7.817×|74|3.060×|18.9|
|12×2|33.884455|284|8.381×|101|2.981×|28.4|
|16×2|40.077381|335|8.359×|124|3.094×|33.5|
|24×2|65.850777|528|8.018×|176|2.673×|52.8|
|merge24×1|65.060937|507|7.793×|175|2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. The all-eight CPU10× goal stays active and unmet, **0/8**. No new AWS/Aeglos, device, concurrent-prover, swap, huge-circuit or minimum-RAM result follows.

[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/count-protocol.md), [model](artifacts/model.rs), [recoder](artifacts/algebra.rs), [exact receipts/helpers](artifacts/receipts-and-helpers.json.gz) and [checksums](SHA256SUMS) retain the result. Private point/scalar/key/witness arrays, per-input digit streams and executables are excluded.
