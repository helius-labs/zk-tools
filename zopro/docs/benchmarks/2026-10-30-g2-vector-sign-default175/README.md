# Default G2 vector-sign integration

The changed default artifact measures 7.928660 ms on primary at 32 workers, a paired 1.54% improvement over production: ON/accepted ratio 0.984643470 [0.981084195, 0.989242556], with 30/31 wins. The final-artifact performance gate is **true**. Production remains unchanged and the candidate is not adopted.

The [original private-candidate full gate](../2026-10-30-g2-vector-sign-sdk175/README.md) remains **false** because its 16-core fresh-control calibration exceeded the declared range. This changed-artifact study does not relabel that gate or repair it by rerunning unchanged code. OFF below is the previously measured private vector-sign implementation, an algorithm reference rather than an identical control. ON is the handwritten-module/default-dispatch artifact; accepted is the frozen production SDK.

Generated arithmetic and generator scripts remain unchanged. Three existing source files change and one handwritten field module is added: 176 files, 172 accepted originals byte-exact. Arithmetic and reduction bodies match the measured implementation after helper renaming. Fresh unfiltered suites pass: native 35 field / 125 core / 8 prover; baseline-x86 with IFMA disabled 11 field / 101 core / 8 prover. These runs do not establish ARM execution.

The fixed protocol uses 31 rotated/reversed blocks, all nine shapes at 32 workers and primary at 16 physical cores. All 55,788 SDK requests verify internally; 972 fresh 164-byte exports independently pass Gnark and extend the actual distinct proof ledger from 26,292 to 27,264. All 1,944 proof/public members are retained. Paired bounds are order statistics 10–22 of 31, nominal marginal coverage 97.055%, without a family-wise guarantee. No observations are removed or added after capture.

|Case|Accepted ms|Private reference ms|Default candidate ms|Candidate CPU ms|Candidate max RSS MiB|
|---|---:|---:|---:|---:|---:|
|1x2-w32|8.041215|7.905330|7.928660|179.413|132.75|
|2x2-w32|11.332764|11.197325|11.151932|259.360|275.00|
|4x2-w32|16.058698|16.146193|16.086386|422.670|312.25|
|8x2-w32|24.119536|24.132635|24.179453|670.190|517.25|
|12x2-w32|33.934147|33.791759|33.884455|941.563|706.50|
|16x2-w32|40.123600|40.131746|40.077381|1122.984|842.00|
|24x2-w32|66.048424|65.844004|65.850777|1888.740|1300.50|
|merge-24x1-w32|65.636306|65.286100|65.060937|1811.215|1168.19|
|merge-54x1-w32|161.820930|162.328796|161.883899|4441.470|1876.96|
|1x2-w16|9.784834|9.430276|9.418625|116.804|90.25|

Failed final-artifact checks: `[]`. Six perf-stat panels and six fixed-period profiles have combined qualification status **true**. Full counts, per-pair bounds, RSS, counter qualification and weighted sample attribution are in the analysis. Absolute period coverage and causes of wall-time changes are not established.

Against the private reference, the primary32 paired ratio is 1.002793442 [0.998432049, 1.005750446]; the interval includes parity and passes the declared 2% nonregression bound. Across the original eight shapes, the equal-log paired improvement over accepted is 0.377%, with ratio 0.996230217 [0.992822966, 0.998524922] and 24/31 wins. The default artifact retains an improvement over production; this study does not establish an improvement over the private implementation.

The initial static source reviewer rejected optional trailing call commas inserted by `rustfmt`. Its original source and failure log are archived. Only the reviewer's formatting normalization was corrected; no prover code, successful test, timing, profile or verifier run was repeated.

## Supplied M5 references

|Shape|Default ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2x2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4x2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8x2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12x2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16x2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24x2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge-24x1|65.060937|507 / 7.793×|175 / 2.690×|50.7|
|merge-54x1|161.883899|—|—|—|

The original all-eight CPU objective remains active: 0/8 meet the supplied CPU 10× targets. The 54→1 shape has no supplied M5 reference. No new AWS/Aeglos, phone/ARM/TEE, large-circuit, resource-limit, minimum-RAM, swap or concurrent-throughput result is claimed. The implementation adds no heap allocation, per-key table, persistent point copy or second execution plan. Functional/resource checks and a distinct engineering decision remain necessary before adoption. Historical accepted local/AWS results are retained in [accepted rows](accepted-rows.json).

[Protocol](artifacts/capture-design.md), [analysis](artifacts/analysis.json), [source review](artifacts/final-source-review.json), [review](artifacts/review.json), [archive and profile review](artifacts/post-publication-review.json), [source](artifacts/source.json.gz), [helpers](artifacts/helpers.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [checksums](SHA256SUMS).
