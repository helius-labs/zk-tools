# Default borrowed-window bitmap reset

Enable the measured bitmap reset by default for native IFMA builds. After a borrowed G1/G2 batch completes, it clears the existing bitmap in one pass when that uses no more words than the pending-point loop; sparse tails keep the original loop. Primary32 measures **7.809034 ms**, a paired **0.56% gain** versus accepted REF, with **132.50 MiB** peak process RSS. The [full SDK comparison](../2026-11-09-busy-reset-sdk176/README.md) passes all 79 declared checks. The remaining functional qualification and default-build checks now pass, so this change is adopted.

The two Window files, including their targeted tests, are byte-exact to the measured and qualified source. The third change registers and emits `zopro_dense_busy_reset` from the core build script for x86_64 + ifma8 + AVX-512 IFMA/VL/DQ without no_ifma. This supplies the same flag used by the measured manual core build; it rewrites no runtime body. Non-IFMA, ARM and wasm defaults retain the original loop. GPU selection is preserved. No arithmetic, inversion, queue, solver/FFT, geometry, allocator, typed forecast, configuration lifetime, buffer, retained table/plan, key copy, pool or RAM cap is added.

Promotion adopts the exact already-measured native SDK artifact. Ten finite build-script contexts pass, and an actual offline Cargo native core check selects the default without an explicit dense-reset flag. That Cargo check is type/build-selection evidence, not a newly timed Cargo release SDK. The fixed-placement benchmark driver remains a harness setting; public SDK APIs are unchanged. The new [accepted manifest](artifacts/production-source-manifest.json) records all 176 source hashes, the three explicit changes, the build-script/source mapping, manual flags and exact native artifact.

The additional qualification links coherent generic consumers and native concurrent consumers against the frozen libraries. All 32 SDK processes and 53 Gnark checks pass:

- Twelve native cases cover primary/54, 1/4 workers, requested 512 KiB stacks, basis/raw images and denied optional preparation.
- Four native primary cases independently disable packed, direct, canonical-batch and vector-inverse paths.
- Thirteen generic/no-ASM/no-IFMA cases cover all eight shapes plus54, and primary/54 raw and denied paths.
- Two shared-image cases and one mixed-key case run four actual SDK callers with two proofs each in one global 32-worker Rayon pool. Loaded/idle TID sets confirm 37/33 threads; mixed callers use separate mapped images.

These are functional checks, not concurrent-throughput or physical ARM measurements. Existing field/core/Window suites and full performance are inherited without replay. Source review preserves allocation capacities and typed workspace forecasts. Optional preparation set to zero exercises the existing fallback; it is not a product memory cap. No new pressure, swap, stack-peak or minimum-RAM result is claimed. The earlier whole-process RSS checks pass, with mixed per-shape peaks.

This qualification adds 140 SDK requests and 53 fresh unique independently Gnark-verified exports, advancing the canonical inventory **41,739 → 41,792**. Across performance and qualification there are 44,350 SDK requests and 688 fresh verified exports. Every proof has the expected 164-byte length and image/public binding. Promotion adds no proof, profile or latency capture. One helper compilation initially failed on a moved path; a borrow-only correction preceded all SDK calls, and the initial source is retained. Helper compiler invocations have tool-transcript evidence; the five consumer links, 85 functional/verifier commands and four promotion commands have workflow receipts.

The post-promotion review checks 173 unchanged plus three explicit original files, exact measured runtime sources, native/generic artifacts, the earlier 90 qualification receipts, ten selector cases and actual Cargo selection. Historical pins are checked against retained pre-promotion copies of the three originals, rather than rewritten. The separate CLI build remains unverified; the standard SDK engine is qualified.

The gain remains small. ALL8 improves 0.22% against fresh OFF and 0.74% against REF under the declared equal-log stratified analysis. Merge54's 0.403% slowdown against OFF remains visible and within the 3% nonregression bound. Not every shape improves or has lower RSS. The prior reference was already below8ms, and historical 7.699867 ms used a different campaign/statistic; this is not a new sub-8-ms milestone or twofold breakthrough.

The following adopted figures are inherited from the successful exact-artifact SDK campaign. Supplied M5 CPU and Metal references are historical observations on different hardware, not remeasured devices. None of the eight CPU 10× targets is met.

|Shape|Adopted CPU ms|Peak RSS MiB|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|7.809034|132.50|37 / 4.738×|24 / 3.073×|3.7|
|2x2|10.940974|271.25|59 / 5.393×|30 / 2.742×|5.9|
|4x2|15.750370|319.25|109 / 6.920×|48 / 3.048×|10.9|
|8x2|23.951790|511.75|189 / 7.891×|74 / 3.090×|18.9|
|12x2|33.857141|713.00|284 / 8.388×|101 / 2.983×|28.4|
|16x2|39.726526|834.75|335 / 8.433×|124 / 3.121×|33.5|
|24x2|65.984808|1276.50|528 / 8.002×|176 / 2.667×|52.8|
|merge-24x1|65.660588|1195.76|507 / 7.722×|175 / 2.665×|50.7|

Merge54 is 163.167792 ms without a supplied M5 reference. The full CPU10× goal remains active and unmet, **0/8**. No new AWS/Aeglos, huge250/4999, phone/ARM/TEE, pressure/swap/minimum-RAM or concurrent-throughput result is added.

[Decision](artifacts/decision.json), [accepted manifest](artifacts/production-source-manifest.json), [post-adoption review](artifacts/post-adoption-review.json), [qualification](artifacts/qualification-analysis.json), [qualification reader](artifacts/qualification-review-result.json), [default selection](artifacts/build-selection.json), [continuation](artifacts/next.md), [source and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Binaries/libraries, unchanged full source snapshots, Cargo target output, key/image/witness/proof payloads and the full historical proof ledger are omitted. The archive retains the changed sources, prior three originals, helper sources, all new workflow receipts and selection evidence.
