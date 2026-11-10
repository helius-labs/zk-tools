# Linear relations in validation constraints

The 1×2 and 2×2 solver images contain 83 and 121 independent linear relations among already computed wires. Restricting relations to the existing A/B query support identifies fewer than 0.23% as many private pivots as query points in all four cases. This establishes a small opportunity for this restricted transformation; it does not establish proof-time savings. No key transformation was implemented or adopted.

This examines a gap in the earlier assignment-only affine census: constraints without an assignment can relate wires that are already computed. Original wires remain independent formal variables, and stored sums expand exactly. Each validation equation is expanded as `A(w) B(w) − C(w)`. Only outputs of the solver's bit hint admit `b² = b`; every other quadratic coefficient must cancel exactly. No sampled witness values or unknown-divisor cancellation are used.

|Circuit|Validation rows|Nonzero linear relations / private rank|Bit identities reducing to tautologies|Matched bit-hint reconstruction relations|
|---|---:|---:|---:|---:|
|1×2|2,421|83 / 83|1,753|17|
|2×2|4,395|121 / 121|3,166|27|

All nonzero relations are already literally linear. Applying certified bit identities additionally identifies the tautologies above, without adding a nonzero linear relation. Public wires are excluded as elimination pivots.

|Circuit|Query|Original points|Fully supported relations / private rank|Rank / query points|
|---|---|---:|---:|---:|
|1×2|A|21,006|31 / 31|0.1476%|
|1×2|B|30,089|67 / 67|0.2227%|
|2×2|A|36,637|49 / 49|0.1337%|
|2×2|B|52,274|97 / 97|0.1856%|

These support-restricted ranks count the extracted relations wholly contained in each query. They are not an exhaustive bound on arbitrary combinations of excluded relations, nonlinear implications, transformations that expand support, other hint semantics, or larger circuits. Implementing a transformed key would also need to establish correctness, memory cost and end-to-end benefit. The counts alone do not justify presenting this as a large work reduction; any small benefit remains unmeasured.

The Rust 1.98.1 producer reads both immutable version-2 solver images completely. Three new tests pass: sparse native-field rank versus independent dense Arkworks elimination on 256 matrices; exact Boolean reduction with cross terms retained; and a complete synthetic image with every truncated prefix rejected. An Arkworks reviewer independently multiplies all 204 selected equations, checks 5,179 coefficient products, and computes ranks by column elimination rather than the producer's row elimination. Both reviews pass. Extraction and bit provenance are shared; this is not an independent full-image decoder or an audit of omitted constraints.

The serialized xtask workflows contain nine executed commands: eight pass and one original test-build command fails. That failure lacked a test-only trait import and vector type annotation. The repair changes only test code; the successful original producer build is reused, and neither census nor successful tests are replayed. The original failure and repaired source are retained. Helper builds and plan preparation are separate from that command count.

There are zero new proof requests, exports, latency measurements or CPU profiles. All 176 accepted production source files remain unchanged, and the canonical proof ledger remains at 46,167. Raw solver images and relation coefficient certificates remain private; their hashes are recorded. This experiment adds no production allocation or key table. It makes no minimum-RAM, mobile or large-circuit claim.

The accepted timings below are inherited observations, not measurements of this census. Local results use the accepted Zen5 build at 32 workers. AWS results use the accepted Zen4 build at each circuit's fastest tested count among 32/64/96. Supplied M5 CPU and Metal references are historical observations on different hardware. The CPU AVX single-proof 10× goal remains active and unmet: 0/8 targets on either host.

|Circuit|Local accepted ms|AWS accepted ms|M5 CPU ms|M5 Metal ms|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1×2|7.772288|11.243892|37|24|3.7|
|2×2|10.929386|15.263659|59|30|5.9|
|4×2|15.731643|19.121785|109|48|10.9|
|8×2|23.965774|25.894916|189|74|18.9|
|12×2|33.742953|33.778290|284|101|28.4|
|16×2|39.704242|37.977699|335|124|33.5|
|24×2|66.034077|53.297583|528|176|52.8|
|24→1|65.387417|56.324672|507|175|50.7|

The completed AWS comparison has Zopro at 11.243892 ms versus pinned Aeglos at 11.870012 ms for 1×2 with 32 workers. Aeglos reaches 7.902802 ms at 96 workers. At each engine's fastest tested count, Zopro has lower displayed latency on six of nine circuits; Aeglos leads on the three smallest. The AWS report contains the paired comparisons, resource observations, profiles and limitations. Its instance and temporary resources have been removed. No AWS work was needed for this census.

Evidence: [1×2 counts](artifacts/1x2.json), [2×2 counts](artifacts/2x2.json), [1×2 independent arithmetic/rank review](artifacts/1x2.review.json), [2×2 review](artifacts/2x2.review.json), [design](artifacts/design.md), [source and command receipts](artifacts/source-receipts.json), [publication checks](artifacts/quality.json), [checksums](SHA256SUMS), [accepted AWS comparison](../2026-11-10-accepted-current-c7a176/README.md), [accepted local measurements](../2026-11-09-busy-reset-adoption176/README.md).
