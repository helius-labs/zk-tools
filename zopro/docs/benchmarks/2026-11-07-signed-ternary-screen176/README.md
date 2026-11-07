# Signed-ternary G2 work screen

The fixed radix-6561 representation reduces modeled generic G2 joins by **6.85% on 1×2** and **6.92% on 2×2**. A separate random-access digit reader passes exact integer checks while retaining the original 32-byte scalar storage. This selects an isolated native caller study; it establishes no measured speedup, resource qualification or adoption. Accepted production remains unchanged.

## Paper and comparison

[Haddaji et al., ePrint2024/038](https://eprint.iacr.org/2024/038), revised February2026, presents a signed-ternary Pippenger method without point-table precomputation. Its binary comparator is unsigned, while our production path already uses signed binary digits. The paper's full-runtime Table7 is slower with recoding included in all13rows; its BLS application can reuse scalar encodings. Those results do not predict fresh-proof latency. The small-d exponential-table method is not selected.

All19pages of the authored PDF were read through [a mirror](https://eprint.iacr.asia/2024/038.pdf). Its creation date and abstract match current official metadata; the official PDF fetch returned403, so a second official copy was not checked for byte identity. The pinned543,851-byte PDF has SHA256 `24c5644e65f1b354e5e5487641ac46a6011a1fcb50163428ab7521d4b103180b`. Full text/PDF are not republished. The original2024abstract's50%claim is not substituted for this revision.

## Fixed real-input model

One parameter is declared before running the model: eight trits per window, radix6561, ten magnitude-3280 windows and one magnitude-1 final carry. The paper's2^15/2^16parameter rows bracket the retained populations. There is no radix, worker or window sweep. The existing signed Phi2 split and unchanged zero-aware grouping policy are reused. Primary remains30,089rows;2×2 groups52,274rows into28,852classes. No scalar/point fixture or witness is regenerated.

Both binary and ternary representations exactly reconstruct every component. Arrivals, occupied buckets, generic fill/partition merges, high-component transforms and identity-skipping weighted joins are counted on the actual inputs, assuming generic noncancelling point sums. Grouped-point cancellation is unknown. Existing256-bucket weighted subtrees require powers of two: ordinary ternary storage pads3,280logical buckets to3,328, leaving48zero buckets. The carry is counted explicitly, not discarded.

|Shape|Quantity|Accepted binary|Ternary|
|---|---|---:|---:|
|1x2|Nonzero arrivals|613891|558731|
|1x2|Fill joins|572819|493144|
|1x2|Partition merges|20536|32786|
|1x2|Weighted affine joins|40265|64320|
|1x2|Total modeled joins|633620|590250|
|1x2|Endomorphism Fq products|82044|128854|
|1x2|Logical buckets, all windows|20544|32801|
|1x2|Weighted subtrees|81|131|
|2x2|Nonzero arrivals|633695|576658|
|2x2|Fill joins|592623|511066|
|2x2|Partition merges|20536|32791|
|2x2|Weighted affine joins|40265|64320|
|2x2|Total modeled joins|653424|608177|
|2x2|Endomorphism Fq products|82072|129528|
|2x2|Logical buckets, all windows|20544|32801|
|2x2|Weighted subtrees|81|131|

Primary has68nonzero final carries;2×2has55. The necessary strictly-positive join-saving check passes on both shapes, without a minimum percentage floor. The prior radix3072candidate and regular differential ladder remain closed. Only their unchanged input population/qualified split and binary reference counts are reused.

Fewer arrivals save fill work, while larger bucket sets increase weighted work and endomorphism transforms. Projective Horner work is separate:120doublings become80triplings; the basic BN254 construction is80doublings plus80additions. No tripling-friendly curve model or free tripling is assumed. Queue fallbacks, SIMD batch topology, inversions, normalization, recoding instructions, projective corrections, allocation, scheduling and memory traffic are excluded. Joins and Fq products are different units and are not combined into a CPU-time estimate.

## Codec and memory limits

For odd radixB and D=B^j, the balanced quotient is floor((m+(D−1)/2)/D). Center its residue moduloB and restore the component sign. Eleven compile-time-divisor branches read the original signed magnitude, with no recoded digit array, heap, key table, persistent state, unsafe code or new pool. The supported magnitude is below2^126. This is source-level storage behavior, not a native instruction or performance claim.

A literal eleven-i16-per-component codec would use44bytes/row rather than32. Random access avoids that extra361,068/346,224byte payload on the two populations. It still has integer division/extraction cost to measure in the actual caller. Two-part affine bucket payload increases from5,259,264to8,397,056logical bytes, or8,519,936with declared padding. These are incomplete layout formulas: projective fallbacks, scratch, live windows, allocator, stacks, key and other proof work are additional. No peak/minimum-RAM, admission, pressure/swap, phone or large-circuit claim follows. Existing bounded-memory fallbacks remain required for integration.

## Validation and next stage

The new model test passes140,877integer codec checks and282weighted masks/shapes, including signs, radix/half-radix boundaries, full magnitude limits, final carry and padded groups. The separate random-access test passes139,561checks and an expected caught invalid-index panic. Counts include repeated boundary values; they are not claims of distinct inputs. Neither test repeats prior field or curve qualification.

An independent offline reader uses per-bucket multiplicities and a separate block-root weighted-tree count, checks the random-access reconstruction of all117,882actual components, all176unchanged production hashes,391pin occurrences and three successful xtask commands/stream hashes. It reuses the previously qualified Phi2 routine and original grouping admission; it is not an independent field/Phi/curve oracle. All new tests, count capture and review pass on first execution. No successful model, proof, decoder or performance run is repeated.

Next implement the fixed G2 caller in an isolated accepted snapshot, qualify ordinary/top windows, signs, exceptional points and subgroup/oracle results, inspect actual consumer flags/codegen, and measure complete real queries against frozen and fresh controls. Larger bucket geometry and resource admission need explicit qualification before SDK integration. A positive count screen does not adopt code or establish the all-eight goal.

No new proof or SDK request is made. The canonical ledger remains32,196. No new AWS/Aeglos,250/4999,ARM/phone,concurrent-proof or full-proof timing is claimed.

## Accepted CPU and M5 references, inherited

The accepted implementation remains production176 / SDK `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. Local values below are the preceding [half-constant complete-build comparison's accepted observations](../2026-11-07-pairing-half-constant176/README.md), not timings of this integer model. M5CPU/Metal are supplied historical references; cross-machine ratios do not establish equal hardware efficiency.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1x2|8.041470|37|24|3.7|
|2x2|11.237839|59|30|5.9|
|4x2|16.143230|109|48|10.9|
|8x2|24.397442|189|74|18.9|
|12x2|33.736405|284|101|28.4|
|16x2|40.082192|335|124|33.5|
|24x2|65.765454|528|176|52.8|
|merge-24x1|65.350546|507|175|50.7|

Merge54remains161.495859ms, without a supplied M5 reference. The all-eight CPU10×goal remains **active and unmet,0/8**.

[Artifacts](artifacts/) include designs, source provenance, both new codecs, model/reviewer/publisher sources, exact counts, plans, pins and raw command receipts. The full paper, private scalar/point/key/witness inputs, executable/library blobs and build targets are excluded. Every archive member is hashed and a single-pass tar comparison passes. Reproduction requires the pinned toolchain, accepted library and private fixtures.
