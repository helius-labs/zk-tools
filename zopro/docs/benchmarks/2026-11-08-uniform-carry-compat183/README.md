# Default combined arithmetic: complete-request and memory-budget qualification

The exact default native/generic SDKs pass complete-request and memory-budget qualification: **284SDKrequests and98fresh independent Gnark exports**, including all nine shapes, fallback paths, small-stack/low-worker calls and concurrent clients. All eight192MiB cgroup cases exit normally with swap0 and no max/OOM/kill events. No high events or memory-pressure stalls occur, so this proves operation under the declared limits, **not active reclaim, minimum RAM or a speed improvement**. Production176 stays unchanged; the default artifact still needs its own prospective performance measurement. The original CPU10×goal remains active/unmet0/8.

## Requests and consumers

This stage reuses the exact [default native/generic SDKs](../2026-11-08-uniform-carry-default182/README.md), their183-file source,297successful test executions, generator/cfg/body audits and frozen8-library bundles. None of those builds, suites or decoders is repeated. Only two new consumer drivers are linked for the changed native artifact. Their eight direct libraries match its frozen bundle; transitive dependency inputs are pinned. Warm/concurrent sources remain byte-exact to the known drivers. Reused accepted consumers match all eight libraries of the current accepted SDK6d83; superseded consumers elsewhere in the old templates are not used.

|Scope|Processes|Requests including primes|Fresh Gnark exports|
|---|---:|---:|---:|
|Native basis, all9shapes/32workers|9|36|9|
|Generic basis all9, primary/54 raw+denied|13|52|13|
|Native1/4workers,512KiB/1MiB configured stacks, primary/54 basis/raw/denied|24|96|24|
|Accepted/candidate independent opt-outs|8|32|8|
|Accepted/candidate × primary/54, four concurrent callers|4|32|32|
|Accepted/candidate cold/warm × allocator policies|8|36|12|
|Total|66|284|98|

Each functional process primes three full SDK requests and exports one internally verified request. The four concurrent processes have four actual callers, two proofs each, a shared mapped key and one existing32-worker global pool; loaded/idle thread counts are37/33. All58functional/concurrent processes finish before their86independent Gnark verifiers. Each resource case finishes before its verification and next case. No owned build/helper/hash/analysis/decoder/report/push or competing benchmark overlaps SDK capture; the resource controller's monitoring is part of its test.

The opt-outs independently disable packed, direct, canonical-batch or vector-inverse behavior. Raw and preparation-denied cases use zero preparation allowance. Stacks are configured values, not a measured whole-call-stack bound. Generic x86 disables assembly and IFMA; it is not execution on ARM, a phone or a TEE. Printed few-request correctness times are not used as latency estimates or compared against the performance references.

## Memory budgets and observed charge

Limits are derived prospectively from the accepted primary pilot's135.75MiB maximum processRSS: ceil-to64MiB of1.25×peak gives192MiB max; ceil-to8MiB of0.9×peak gives128MiB warm high. These are test settings, not architecture-wide product limits or minimum physical RAM. Each owned service copies and charges its private input image, then starts the SDK. Cold cases enforce192MiB from launch; warm cases prime at512MiB, touch32MiB ballast, then apply high/max128/192MiB and prove two more requests. Only uniquely prefixed owned units and control directories are eligible for cleanup; no shared image cache or kernel setting is changed.

|Case|Cgroup lifetime peak MiB|High / max / OOM / kill events|Swap peak bytes|
|---|---:|---:|---:|
|cold-accepted-default|63.718750|0 / 0 / 0 / 0|0|
|cold-accepted-retained|63.140625|0 / 0 / 0 / 0|0|
|cold-on-default|60.347656|0 / 0 / 0 / 0|0|
|cold-on-retained|63.031250|0 / 0 / 0 / 0|0|
|warm-accepted-default|105.253906|0 / 0 / 0 / 0|0|
|warm-accepted-retained|96.472656|0 / 0 / 0 / 0|0|
|warm-on-default|103.312500|0 / 0 / 0 / 0|0|
|warm-on-retained|103.929688|0 / 0 / 0 / 0|0|


All cases use swap0, retain zero max/OOM/kill events, and exit normally. Warm limits are verified from the actual cgroup after application. All warm pre/post worker-TID sets match. The largest candidate charged peak is103.929688MiB, including the32MiB warm ballast; this is not a phone's RAM requirement or a new arithmetic memory-saving claim. Cgroup lifetime peak includes the warm priming phase; it differs from whole-processRSS and from the longer performance campaign's allocator history.

Memory high events and PSI totals remain zero. The fixed budget test therefore does not demonstrate actual reclaim or sustained long-run memory behavior. No extra ballast, lower cap or larger-cap retry is added after seeing the result. This short request scope cannot establish an allocator's long-run plateau; later performance captures retain their own RSS observations. Both default and retained allocator environments are tested, with mixed peaks between accepted/candidate.

## Verification and provenance

All98exports are164-byte proofs with44-byte public witnesses, fresh against the35,863-proof inventory and one another. Independent verification binds original image, public input and expected VK, including raw-input cases verified against the canonical image. All other requests are internally verified; they are not counted as independent exports. Inventory advances **35,863→35,949functional→35,961final**. Resource additions are persisted after each successfully verified case. Copies or reader passes are not new exports.

Both new consumer links, all144functional/verification commands and the eight-case resource-controller command pass on first execution. The independent reader checks source/library/binary pins, command/stream receipts, all98bindings and exact inventory unions, declared scope/limits and zero max/OOM/kill events. It shares the established crypto verifier and controller implementation, rather than claiming another crypto implementation. No successful request, verifier, test, build, decoder or analysis is rerun to repair bookkeeping.

All owned services and runtime control overrides are gone after the test. Actual key/image/request/witness/proof/public payloads, executables/libraries and full proof ledgers remain private. Sources/helpers, frozen plans/pins, raw logs, limits, thread/cgroup observations, receipts and verification hashes are retained. No production source, new pool, large lookup table, retained solver plan, key representation or fixed product memory cap is introduced.

## CPU and Metal reference comparisons

The default artifact has no qualified latency measurement yet. These are the **measured private parent's** local CPU observations against the supplied M5 references on different hardware; primary32 is its unchanged pilot and the other shapes its full-shape study. They do not describe the new default artifact's speed. That parent's real roughly1% all-eight aggregate gain and failed primary16 calibration/nonregression checks remain unchanged.

|Shape|Private-parent local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.869400|37 / 4.702×|24 / 3.050×|3.7|
|2x2|11.018191|59 / 5.355×|30 / 2.723×|5.9|
|4x2|15.922784|109 / 6.846×|48 / 3.015×|10.9|
|8x2|24.003699|189 / 7.874×|74 / 3.083×|18.9|
|12x2|33.729083|284 / 8.420×|101 / 2.994×|28.4|
|16x2|39.706864|335 / 8.437×|124 / 3.123×|33.5|
|24x2|65.792319|528 / 8.025×|176 / 2.675×|52.8|
|merge-24x1|64.546908|507 / 7.855×|175 / 2.711×|50.7|


Private-parent merge54 is161.197772ms without a supplied M5 reference. The previously measured7.859756ms uniform-only candidate remains valid history; no new best-time claim is made here. No fresh AWS/Aeglos, huge-circuit, phone/ARM/TEE, swap-enabled, minimum-RAM, throughput or full performance result is added. Next: measure the exact default native SDK under a new fixed prospective full-shape protocol with automatic diagnostics and coherent contemporaneous controls. No source adoption or10×goal completion follows from this correctness/resource qualification.

[Protocol](artifacts/protocol.json), [preparation review](artifacts/preparation-review.json), [functional review](artifacts/functional-review.json), [analysis](artifacts/analysis.json), [limits](artifacts/resource-limit-derivation.json), [resource results](artifacts/resource-result.json), [review](artifacts/review.json), [cleanup](artifacts/cleanup.json), [publication](artifacts/publication.json), [evidence archive](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
