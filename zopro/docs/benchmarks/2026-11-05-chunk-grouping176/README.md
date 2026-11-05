# Query grouping across chunks

Independent grouping of ready 1,024-point chunks would lose substantial global grouping on larger retained circuits. On 24×2, the A query has 43,958 global nonzero classes versus 322,633 chunk-local class instances; B has 66,221 versus 484,832. Even retaining the best 32 cross-chunk classes would recover less than 1% of those extra instances. A small class cache does not preserve most of the observed global savings. No runtime optimization is adopted.

Primary 1×2 is different: its observed A and B scalar populations decline global grouping under the existing policy. Consequently it does not have admitted global grouping savings to lose at that point. The preceding [epoch timing diagnostic](../2026-11-05-epoch-clock176/README.md) found about 0.767 ms of primary32 solver execution remaining after the first ready chunks, with a failed global observer-overhead screen. That is a limited descriptive interval, not a twofold path or removable proof time. These two findings argue against one universal chunk-local grouping policy.

## Measurement and coverage

This is a structural diagnostic, not a latency benchmark. On the Ryzen 9950X at 32 workers, immutable canonical A/B scalars are counted after normal solving/extraction. The observer reports anonymous class-size/chunk-span histograms and counts, without exporting scalar values. It calls the existing grouping admission policy on the same complete query and each chunk at the diagnostic point. Admission under another memory load or a different witness is not established. Temporary diagnostic maps/canonical copies are not a proposed production cache or retained plan. Existing arithmetic, scalars, points, solver and MSM results remain unchanged.

The first isolated source and capture passed 128 unfiltered native core tests and produced eight internally verified SDK proofs, all independently verified by pinned Gnark. The first finisher correctly failed its coverage assertion: only 1×2/2×2 entered the instrumented small canonical-wire path. Larger queries use another extraction path. All eight valid exports and the four observed query populations are retained.

A second isolated revision adds read-only observation at generic extraction, converting temporary canonical copies while preserving the original Montgomery inputs. Four focused observer/conversion tests pass, with 125 other core tests filtered out; the preceding full core qualification and unchanged field/prover qualification are inherited. Only the six previously unobserved larger shapes run under this revised diagnostic. Their six fresh exports independently pass Gnark. No successful initial observation, native suite or verifier is repeated for bookkeeping. The final report covers all eight shapes and 16 query populations, with 14 fresh exports and ledger 29,447→29,461. This coverage correction and the original failed assertion are [retained](artifacts/coverage-failure.md).

Each source revision contains 177 files, 174 of the 176 originals exact; all original bytes recover and production is unchanged. Independent closeout checks both pin sets, all 36 successful command receipts/stream hashes, both source recoveries, all population identities, complete coverage and the fresh-proof ledger union.

## Global versus chunk-local populations

A class is an exact nonzero scalar value. Repeated zeros are excluded from classes. Local instances sum distinct nonzero values separately in each fixed chunk, including the final partial chunk. These are population counts, not EC addition counts, an instruction model, latency ratios or a prediction for a new MSM. Global point grouping itself performs work that chunking would redistribute.

|Shape / query|Points|Zeros|Global classes|Global admitted values|Chunk-local instances|Extra instances|Best32 recovery of excess|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2/A|21006|1733|17454|declined|18046|592|11.99%|
|1x2/B|30089|1600|26142|declined|27311|1169|7.10%|
|2x2/A|36637|3067|19268|19268|32086|12818|1.15%|
|2x2/B|52274|2763|28852|28852|48016|19164|0.85%|
|4x2/A|67154|5767|20386|20386|57909|37523|0.73%|
|4x2/B|95537|5120|30544|30544|87163|56619|0.52%|
|8x2/A|129064|11198|25236|25236|111167|85931|0.59%|
|8x2/B|183347|9881|37850|37850|166757|128907|0.42%|
|12x2/A|190734|16719|30704|30704|164135|133431|0.58%|
|12x2/B|270757|14770|46062|46062|246335|200273|0.41%|
|16x2/A|252548|22312|34326|34326|216527|182201|0.56%|
|16x2/B|358343|19703|51597|51597|325319|273722|0.39%|
|24x2/A|376992|33639|43958|43958|322633|278675|0.56%|
|24x2/B|534619|29802|66221|66221|484832|418611|0.39%|
|merge-24x1/A|359220|31247|41272|41272|311704|270432|0.57%|
|merge-24x1/B|507895|26246|62005|62005|467562|405557|0.38%|

Best32 is an optimistic offline selection: retain the 32 classes appearing in the most chunks and count the class instances eliminated by perfect cross-chunk combination. It assumes future knowledge, ignores state/update costs, is unweighted by scalar digit density and does not implement a streaming cache. The report also retains best8/best16 counts, complete histograms, per-chunk distinct counts and actual per-chunk admission results. A production bounded scheme is not guaranteed to attain these numbers.

## Decision

Close the proposed universal chunk-local grouping plus tiny retained-class cache route. It discards widespread cross-chunk grouping on larger circuits; the savings are not concentrated in a handful of classes. Preserving global grouping would require a different streaming reduction design, not a larger unbounded cache. Primary-only early work remains a separate possible optimization, but the measured overlap interval is limited and a universal dispatch must handle other witnesses that do admit grouping. Any future prototype must preserve group/point lifetimes, original G2 high/low semantics, persistent bucket state with one final reduction, and complete workspace admission. This diagnostic does not justify adding a key table, copying a plan, creating a pool or imposing a fixed RAM cap.

## Accepted CPU and Metal comparisons, inherited

No latency is measured here. Accepted SDK SHA-256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. The following accepted local observations and supplied M5 references remain unchanged; all eight CPU10× targets remain unmet.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667x|24 / 3.027x|3.7|
|2x2|11.151932|59 / 5.291x|30 / 2.690x|5.9|
|4x2|16.086386|109 / 6.776x|48 / 2.984x|10.9|
|8x2|24.179453|189 / 7.817x|74 / 3.060x|18.9|
|12x2|33.884455|284 / 8.381x|101 / 2.981x|28.4|
|16x2|40.077381|335 / 8.359x|124 / 3.094x|33.5|
|24x2|65.850777|528 / 8.018x|176 / 2.673x|52.8|
|merge-24x1|65.060937|507 / 7.793x|175 / 2.690x|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference and is not observed in this census. No new AWS/Aeglos, RAM peak/minimum, pressure/swap, mobile/ARM/TEE, 250/4999 or concurrent-throughput result is added. Latest full-SDK latency evidence remains the epoch timing diagnostic; these counts do not replace it. Goal all-eight CPU10× remains active.

## Artifacts

[Design](artifacts/design.md), [combined counts](artifacts/combined-analysis.json), [independent review](artifacts/review.json), [reviewer](artifacts/close.rs), [initial observer](artifacts/census.rs), [revised observer](artifacts/missing-six/census.rs), [driver](artifacts/driver.rs), [first protocol](artifacts/plan.json), [missing-six protocol](artifacts/missing-six/plan.json), [coverage correction](artifacts/coverage-failure.md), [combined proof inventory](artifacts/combined-proof-inventory.json), [final ledger](artifacts/missing-six/proof-sha256-after.json), [source archive](artifacts/source.tar.gz), [public proofs](artifacts/public-proofs.tar.gz), [raw receipts](artifacts/receipts.json.gz), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). Source354/public-proof28 regular members match archive extraction hashes, and receipt JSON gzip roundtrips exactly. Private key/images/requests, witnesses, executables and libraries are excluded.
