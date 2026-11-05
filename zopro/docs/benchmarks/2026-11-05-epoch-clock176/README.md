# Solver epoch timing diagnostic

Primary's measured 32-worker solver execution loop lasts a median 1.016 ms; after the first joined epoch, 0.767 ms remains. For 2×2 the corresponding values are 1.575 ms and 1.156 ms. These observations identify a limited interval for possible earlier query work; they are not removable proof time or a measured speedup. No runtime change is adopted.

The declared observer-overhead screen **fails overall**. Five checks fail in the 16-worker contexts, including both fresh-OFF/accepted calibrations. The two 32-worker contexts individually meet their bounds, but this does not turn the global screen into a pass. Full-proof controls bound overall observer effects only and do not isolate distortion within individual epochs. All observations and failed checks are retained.

## Method and validation

Ryzen 9950X, 16 physical / 32 logical CPUs. Two retained circuits, physical CPUs 0–15 or logical CPUs 0–31, with exact accepted SDK, fresh OFF and instrumented ON arms. Twelve one-round preflight processes precede 15 fixed rotated/reversed blocks of 31 measured proofs per process; each process primes three times. All latency processes finish before the 180 final verifiers. Total: 6,168 SDK requests, 192 distinct exported proofs independently verified by pinned Gnark, and 1,864 ON clock snapshots. The proof ledger increases from 29,255 to 29,447; exports are fresh across that ledger. All SDK requests include normal solve/prove/internal verification/serialization.

The observer records cumulative monotonic nanoseconds after each joined solver epoch. Direct and refined paths are instrumented; all observed proofs use the direct 16-epoch path. Three primes, key load and output formatting are unobserved. Snapshot allocation and publication remain inside ON's full-proof timer. OFF has no clocks in its execution loops. The epoch interval excludes solver allocation, selected-slot copying/canonicalization, quotient FFTs and later MSMs. It contains no overlapping query work.

The actual prepared schedules at 16 and 32 workers place every operation in the same outer epoch and produce identical readiness counts. After the first primary barrier, two A chunks and five B chunks of 1,024 points are ready. The preceding [parallel-readiness census](../2026-11-05-parallel-readiness176/README.md) supplies the complete counts and low-task-parallelism geometry; this diagnostic adds elapsed times. Task counts do not measure idle cores.

The private snapshot has 177 files: 174 of 176 originals are exact, two parents have reversible diagnostic edits, and one child supplies clocks. All original bytes recover. Field arithmetic/generator, scheduling order and production remain unchanged. Each OFF/ON arm passes 128 unfiltered core tests plus two prover and six cubic tests. Field qualification is inherited. Independent closeout reconstructs latency medians from integer nanoseconds, every epoch statistic and declared gate, checks 400 successful command receipts and their stream hashes, source/pins, schedule bindings, proof hashes and Gnark bindings. No successful capture, verifier or qualification run was repeated for bookkeeping.

## Full-proof diagnostic medians

These are medians of 15 process medians, not new accepted performance claims.

|Context|Accepted ms|OFF ms|ON ms|
|---|---:|---:|---:|
|1x2-w16|9.601849|9.535804|9.376485|
|1x2-w32|7.940026|7.923265|7.949935|
|2x2-w16|13.127345|13.006717|13.184222|
|2x2-w32|11.181547|11.202626|11.144106|

Paired ratios use matched block medians. Bounds are order statistics 4–12 of 15, nominal 96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. OFF/accepted must fit [1/1.03, 1.03]; ON/OFF and ON/accepted upper bounds must be at most 1.03. This is an observer-overhead screen, not an optimization-gain screen. No samples were excluded and no bounds or block count were changed.

|Context / ratio|Median|Lower|Upper|Pass|
|---|---:|---:|---:|---|
|1x2-w16/OFF/accepted|0.991372|0.846703|1.009171|false|
|1x2-w16/ON/OFF|0.986447|0.972357|1.118269|false|
|1x2-w16/ON/accepted|1.001939|0.935174|1.041948|false|
|1x2-w32/OFF/accepted|0.998593|0.991425|1.003551|true|
|1x2-w32/ON/OFF|1.003133|0.995966|1.010180|true|
|1x2-w32/ON/accepted|1.003397|0.991057|1.009787|true|
|2x2-w16/OFF/accepted|0.990811|0.862576|1.001359|false|
|2x2-w16/ON/OFF|1.011957|1.002224|1.038192|false|
|2x2-w16/ON/accepted|1.002220|0.974392|1.023195|true|
|2x2-w32/OFF/accepted|1.001885|0.997590|1.008337|true|
|2x2-w32/ON/OFF|0.994478|0.982697|0.999348|true|
|2x2-w32/ON/accepted|0.992427|0.990702|0.996652|true|

## Cumulative epoch observations

Each context contains 465 measured ON proof snapshots. Remaining time is computed within each proof before taking its median; it is not the difference of the displayed medians. All contexts remain descriptive because the global observer screen failed.

|Context|Epochs joined|Cumulative median µs|Remaining solver median µs|
|---|---:|---:|---:|
|1x2-w16|1|215.126|701.642|
|1x2-w16|4|439.198|480.586|
|1x2-w16|8|606.623|315.655|
|1x2-w16|15|890.448|31.499|
|1x2-w16|16|921.988|0.000|
|1x2-w32|1|246.775|767.226|
|1x2-w32|4|490.604|525.280|
|1x2-w32|8|677.968|340.411|
|1x2-w32|15|983.604|33.473|
|1x2-w32|16|1016.326|0.000|
|2x2-w16|1|371.911|1044.989|
|2x2-w16|4|668.911|745.816|
|2x2-w16|8|943.278|474.896|
|2x2-w16|15|1378.419|39.986|
|2x2-w16|16|1420.578|0.000|
|2x2-w32|1|419.581|1155.959|
|2x2-w32|4|748.942|827.219|
|2x2-w32|8|1044.198|531.241|
|2x2-w32|15|1527.569|43.872|
|2x2-w32|16|1574.798|0.000|

## Implication

A bounded early-query prototype could test whether useful work fits into the measured interval without delaying solver completion. It must preserve pending MSM bucket state across chunks and reduce once, account for scalar grouping across chunks, preserve G2 high/low ordering, and bound scalar staging and point lifetimes. Final Montgomery wires remain readable in their existing representation. No duplicated key/plan, large lookup table, additional pool or fixed RAM cap is proposed. These observations do not establish a twofold path. Earlier post-solver overlap failures remain closed.

## Accepted comparisons, inherited

The accepted SDK remains SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. The following local observations and supplied M5 references are inherited; this diagnostic does not replace them. All eight CPU10× targets remain unmet.

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

Merge54 remains 161.883899 ms without a supplied M5 reference. No new AWS/Aeglos, peak/minimum RAM, pressure/swap, phone/ARM/TEE, 250/4999 or concurrent-throughput measurement is added. The all-eight10× objective remains active.

## Artifacts

[Design](artifacts/design.md), [binding protocol](artifacts/binding-protocol.md), [analysis](artifacts/analysis.json), [raw rows](artifacts/rows.json), [independent review](artifacts/review.json), [reviewer](artifacts/review.rs), [observer](artifacts/clock.rs), [driver](artifacts/driver.rs), [campaign](artifacts/campaign.rs), [source audit](artifacts/source-audit.json), [protocol](artifacts/plan.json), [pins](artifacts/pins.json), [proof inventory](artifacts/proof-inventory.json), [ledger](artifacts/proof-sha256-after.json), [public proof archive](artifacts/public-proofs.tar.gz), [source archive](artifacts/source.tar.gz), [raw receipts](artifacts/receipts.json.gz), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). Source177/public-proof384 regular members match extraction hashes; receipt JSON gzip roundtrips exactly. Private images, keys, schedules, requests, witnesses, executables and library binaries are excluded.
