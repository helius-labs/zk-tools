# Default G2 vector signs: functional and resource qualification

The exact default artifact passes the additional functional and resource checks below. Its [final-artifact performance gate](../2026-10-30-g2-vector-sign-default175/README.md) remains **true**; the original private-candidate full gate remains **false** for the historical 16-core fresh-control calibration. Production remains unchanged and the candidate is not adopted. This study adds no new latency or profile measurement.

All 176 source files are byte-exact to the measured default snapshot. Its six fresh unfiltered suites are inherited after source/receipt/flag checks: native field/core/prover 35/125/8, baseline-x86 with IFMA disabled 11/101/8. No successful field/core/prover suite or SDK build is repeated. Warm and concurrent driver sources are byte-exact to the previous qualified drivers. Accepted consumers are inherited by exact binary/link receipts; only the two candidate consumers are freshly linked against coherent frozen default libraries.

- 13 portable processes cover nine basis shapes and primary/54→1 raw and preparation-denied paths.
- 24 native candidate processes cover 1/4 workers, 512 KiB/1 MiB configured stacks, primary/54→1 and basis/raw/denied paths.
- Eight processes independently disable packed/direct/canonical-batch/vector-inverse paths for accepted/candidate.
- Four concurrent processes use four actual SDK callers, two proofs each, one global 32-worker pool and a shared mapped key. They cover accepted/candidate × primary/54→1, requiring 37 loaded/33 idle TIDs. These are functional checks, not throughput measurements.
- Eight owned cgroup cases cover accepted/candidate × cold/warm × default/retained allocator settings at 32 workers, with swap disabled.

All 248 fresh SDK requests verify internally. All 89 new 164-byte exports independently pass Gnark with exact image/key/public bindings and uniqueness against 27,264 prior identities. The verified ledger advances to 27,353. All 178 proof/public members are retained. Raw loader checks use the matching Euler verifying image.

## Memory observations

Test limits derive from the final capture's accepted primary peak RSS of 135 MiB: 1.25× rounded upward to 64 MiB gives a 192 MiB maximum; 0.9× rounded upward to 8 MiB gives a 128 MiB warm high limit. Warm cases prime at 512 MiB, touch 32 MiB ballast, then lower high/max to 128/192 MiB before two proofs. Each owned service charges a private image copy. These are test controls, not an architectural memory cap or minimum-RAM estimate.

|Case|Cgroup peak MiB|High events|Max events|OOM/kill|Swap limit|
|---|---:|---:|---:|---|---:|
|cold-accepted-default|59.96|0|0|0/0|0|
|cold-accepted-retained|61.38|0|0|0/0|0|
|cold-on-default|61.62|0|0|0/0|0|
|cold-on-retained|62.96|0|0|0/0|0|
|warm-accepted-default|103.18|0|0|0/0|0|
|warm-accepted-retained|102.98|0|0|0/0|0|
|warm-on-default|104.49|0|0|0/0|0|
|warm-on-retained|104.59|0|0|0/0|0|

All cases exit normally with zero max/OOM/kill events. Zero high events establish operation under the configured limits, without establishing active throttling. Peaks include the private image, child controller, SDK and any ballast. Shared mappings can be charged to another cgroup, so these peaks are not standalone RAM requirements and should not be compared directly with process RSS. Only uniquely prefixed owned units/control directories are removed; global memory/kernel/cache settings are unchanged.

The first attempt to start the consumer-link workflow was rejected before any step ran because its output directory already held inherited receipts. The prelaunch log is retained. The receipts were moved aside, the empty output path was used, and both link steps completed. An early read-only preparation review also ran before preparation finished and found its plan missing; that log is retained, followed by the successful review after preparation completed. No successful test, proof capture, verifier or link step was repeated.

## Supplied CPU and Metal references

These are inherited 32-worker timings from the exact measured default artifact, with supplied historical M5 references. Primary was 7.928660 ms with a paired 1.54% improvement over production and 30/31 wins. The interval against the private implementation includes parity. All historical accepted local/AWS rows remain in [accepted rows](accepted-rows.json).

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

The original objective remains active and unmet: 0/8 shapes meet the supplied M5 CPU 10× targets. Primary requires 3.7 ms; 54→1 has no supplied M5 reference. This study adds no AWS/Aeglos, ARM/phone/TEE, large-circuit 250/4999, swap, minimum-RAM or concurrent-throughput result. The implementation adds no heap allocation, per-key lookup table, persistent point copy or second execution plan. Passing these checks permits a separate engineering/adoption review; it does not override the original failed calibration or itself adopt the candidate.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [source reuse](artifacts/source-reuse.json), [resources](artifacts/resource-result.json), [qualification review](artifacts/qualification-review.json), [archive review](artifacts/post-publication-review.json), [review](artifacts/review.json), [source](artifacts/source.json.gz), [helpers](artifacts/helpers.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [checksums](SHA256SUMS).
