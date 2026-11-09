# Batched double-add for weighted bucket folds

The standalone native G1 kernel passes three new correctness tests and a generic guard compile. It computes the actual weighted-fold recurrence `S <- 2S-Q` across eight independent groups. **Performance is unmeasured; no optimization is adopted.** The complete core consumer, resource accounting and all-shape proof qualification remain to be implemented.

The final `weighted_sum` recurrence has a repeated input, unlike arbitrary bucket `P+Q`. This permits the double-add identity from [Eisenträger, Lauter and Montgomery](https://arxiv.org/html/math/0208038), sections 3–3.1: form the first sum's X coordinate, then compute the second slope without its unused Y coordinate. The local history audit found the paper in general stream/triple studies; those failed recipes are not rerun. The scalar identities are established arithmetic, not claimed as a new formula or a transferred paper speedup.

The new fixed-eight kernel uses the unchanged accepted Fq/IFMA8 library and the exact existing lane-inverse tree. Identity inputs, equal first X and a zero second denominator fall back elementwise to the original complete projective double plus affine add. Inactive lanes use unit denominators; all-inactive blocks skip inverse work. Inputs remain immutable. The kernel has fixed arrays, zero heap-allocation expressions and no unsafe block, table, key copy, persistent plan, new pool or memory cap. This source observation is not a full stack/RSS bound.

An all-ordinary block uses five vector point products and two five-product inverse bridges: **15 eight-lane products, two scalar root products and two scalar inversions**. The original ordinary scalar Jacobian double and mixed addition use 9 multiplications plus 9 squares per lane. These different execution types are not interchangeable cycle costs. Coordinate packing, middle-X canonical export/repack, fallback normalization, both inversions, output form and reduced task parallelism can erase the opportunity. No timing or IFMA-retirement saving is claimed.

All three Rust/xtask workflow commands succeed on their first execution: build independent native qualification, run the three tests, and compile the no-IFMA/no-ASM generic guard. The exact accepted field library `1200e635…` and prior generic field are reused without rebuilding or replaying their suites. The generic result is a compile check, not ARM execution. Two unused-import warnings are retained; no warning-only rebuild is performed.

|New qualification|Checked stage point outputs|Coverage|
|---|---:|---|
|Changing packets|2,304|All active prefixes 0–8; signed full-width points; identity, equal, opposite and Q=-2P cases|
|Weighted recurrence|4,608|All eight steps and final weighted dot product for 256 buckets; cancellation and zero patterns|
|Threaded recurrence|3,392|Every actual worker in 1/4/16/32-worker Rayon pools; requested 512 KiB stacks|

The 10,304 stage comparisons use independent Arkworks group arithmetic. Additional final/control comparisons and repeated native executions are not added to that count. Packet routing has 1,092 ordinary results, 939 first guards and 273 second guards. These are synthetic test frequencies, not actual proof branch counts. Sixteen weighted fixtures include repeated zero/cancellation populations, rather than sixteen wholly independent random populations. Each fixture constructs nine correction roots independently from bucket coefficients and checks every intermediate recurrence, final weighted sum, original projective fold, inactive destinations and input immutability. Test allocations are outside the proposed kernel and establish no production memory figure.

The separate same-agent reader checks all 189 pins, three exact command/stream receipts, result counts, both source guards and the original `weighted_sum` dispatch recurrence. An independently written integer reconstruction checks all 256 bucket coefficients. It shares the cryptographic libraries and does not replay the native tests or independently implement field arithmetic. [Review](artifacts/review-result.json).

Next is the actual complete weighted consumer. It must preserve **both** the weighted sum and the unweighted chunk total used for group-offset corrections; root0 already contains that total. The prior [weighted-transpose screen](../2026-11-07-weighted-transpose176/README.md) showed why dropping it gives an invalid comparison. A possible bounded job reuses one existing 511-slot leaf array and scratch while collecting at most eight sets of nine roots, then folds them together. That layout is not implemented here and may reduce available parallelism. Typed admission, arbitrary sparse/exceptional inputs, partial group tiles, original fallbacks, concurrency and complete caller timings must precede any SDK selection.

Production's 176 sources, accepted SDK and 41,104-export private proof ledger remain unchanged. This stage adds zero SDK requests or Gnark exports. The previous short-forward SDK candidate remains closed and unadopted with two failed selection checks; its supported small primary gain remains evidence. No new AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result is supplied.

The accepted CPU/Metal comparison below is inherited from the [latest full SDK experiment](../2026-11-09-negative-phi-sdk176/README.md), using its unchanged accepted fixed-placement REF and median of process means. M5 timings are supplied references from different hardware. All eight CPU10x targets remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10x target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.832708|37|24|4.724x|3.064x|3.7|
|2x2|11.065989|59|30|5.332x|2.711x|5.9|
|4x2|15.913588|109|48|6.849x|3.016x|10.9|
|8x2|24.254933|189|74|7.792x|3.051x|18.9|
|12x2|34.007120|284|101|8.351x|2.970x|28.4|
|16x2|40.086324|335|124|8.357x|3.093x|33.5|
|24x2|66.467587|528|176|7.944x|2.648x|52.8|
|merge-24x1|65.569872|507|175|7.732x|2.669x|50.7|

[Design](artifacts/design.md), [kernel](artifacts/kernel.rs), [independent qualification](artifacts/qualification.rs), [plan](artifacts/native-plan.json), [pins](artifacts/native-pins.json), [continuation](artifacts/next.md), [sources and receipts](evidence.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Libraries, binaries, raw proving inputs/keys/scalars/proofs, raw perf and full private ledgers are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/weighted-double-add176`.
