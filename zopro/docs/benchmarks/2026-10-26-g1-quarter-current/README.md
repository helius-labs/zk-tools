# Current-backend G1 quarter-slope rebase

The declared local gate **fails**. Production remains unchanged and no candidate is adopted. This is a fresh integration of previously tested raw-domain quarter-slope algebra on the current backend; older positive observations below their old minimum-gain thresholds are retained as history, not reused as current qualification.

Primary32 takes 8.554 ms versus accepted 8.594 ms in this series. Its paired ratio is 0.994617 against accepted (0.54% lower, four of seven wins) and 0.993797 against fresh OFF (0.62% lower, six wins). The original-eight aggregate is 0.32% lower than accepted and 0.47% lower than OFF. All paired nonregression checks pass, and the 32-worker control calibration passes.

Three physical-16 per-block control calibration checks fail. All observations remain included, and the declared complete gate therefore fails. The physical-16 result is not selected as a speedup. The small 32-worker signal is retained as an observation; it does not establish a general speedup or justify production adoption. This is not evidence of a 2× improvement.

Raw R256 point-coordinate words represent X=x/16,Y=y/16 in uniform R260 vectors. Scale the shared inverse by 1/4, yielding q=slope/4; evaluate X=q²−(AX+BX), Y=4q(AX−X)−AY. Raw output words recover the original host coordinates. The multiplier count is unchanged; the hypothesis is less mixed-radix packing work in current CIOS operations. New helpers use the current cancel-carry backend. This is not a new point-addition formula or an Aeglos port.

The original G1 group Vec and scalar-tail Vec are shared across both kernels. Bit bridges preserve stored bits and add no field arithmetic; no additional point buffer is introduced. The group remains 1,600 bytes and scratch remains 56 bytes on the native target; switching kernels reuses the same allocation. There is no new Vec, table, key conversion, pool or workspace forecast. Only explicitly enabled native direct/canonical/vector-inverse/early-store streaming selects the new route; other configurations retain current kernels.

## Verified request results

Ryzen 9950X, 16 physical cores and 32 logical CPUs. All nine shapes use 32 workers on CPUs 0–31; primary also uses 16 physical workers on CPUs 0–15. Accepted, fresh OFF and ON run seven rotated/reversed fresh-process blocks, with 31 timed requests after three primes. OFF/ON share one executable. Loading/preparation is excluded; witness solving, fresh randomness, proving, SDK verification and serialization are timed. All latency runs finish before profiles. Times are medians of process medians; ratios are paired geometric means.

|Case|Accepted ms|OFF ms|ON ms|ON/OFF|ON/accepted|ON/accepted wins|ON RSS MiB|ON CPU ms/request|
|---|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2-w32|8.593896|8.599606|8.554091|0.993797|0.994617|4/7|114.00|198.412|
|1x2-w16|11.489336|11.395629|11.033937|0.949437|0.945385|4/7|81.50|128.744|
|2x2-w32|11.606487|11.686168|11.668644|1.001646|1.004161|3/7|261.50|270.036|
|4x2-w32|16.179431|16.081585|15.963834|0.991715|0.990947|5/7|315.75|420.661|
|8x2-w32|24.772574|24.583470|24.807339|1.002726|0.988537|4/7|526.00|676.742|
|12x2-w32|34.467557|34.799252|34.356977|0.987663|0.995556|4/7|714.75|951.777|
|16x2-w32|40.553543|40.985076|40.493993|0.991825|0.998900|4/7|857.25|1136.187|
|24x2-w32|68.268380|69.036326|68.081392|0.986947|0.996532|4/7|1297.11|1952.981|
|merge-24x1-w32|67.225732|67.291500|67.628824|1.006044|1.005578|1/7|1192.52|1876.067|
|merge-54x1-w32|176.423318|177.837778|178.548536|1.001857|1.011041|2/7|1872.84|4890.665|

Whole-process RSS includes loading/primes and is not incremental scratch or minimum RAM. Summed CPU includes coordination and is not an arithmetic operation count. Full per-mode resource values and raw observations are in the analysis.

## Supplied M5 comparisons

Historical supplied references; new local candidate times include the complete verified SDK request. Merge54 has no supplied reference. Ratios below describe the unadopted candidate.

|Shape|ON ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|
|---|---:|---:|---:|---:|---:|
|1x2|8.554091|37|24|4.33×|2.81×|
|2x2|11.668644|59|30|5.06×|2.57×|
|4x2|15.963834|109|48|6.83×|3.01×|
|8x2|24.807339|189|74|7.62×|2.98×|
|12x2|34.356977|284|101|8.27×|2.94×|
|16x2|40.493993|335|124|8.27×|3.06×|
|24x2|68.081392|528|176|7.76×|2.59×|
|merge-24x1|67.628824|507|175|7.50×|2.59×|
|merge-54x1|178.548536|—|—|—|—|

0/8 reach the supplied CPU10× threshold. The original all-eight objective remains active and unproven.

## Declared checks

OFF/accepted calibration: factor 1.05 each block and 1.03 paired median in all ten cases. Primary32 must improve against both controls with at least 4/7 wins and no minimum gain floor. Every case, including physical16, must remain within 1.02 against both controls. The original-eight 32-worker aggregate must be <=1.00 against both controls. No successful capture is excluded or repeated.

calibration_failures:

- 1x2-w16 b0 OFF/accepted=0.9303441904736707
- 1x2-w16 b2 OFF/accepted=0.9386307734961444
- 1x2-w16 b5 OFF/accepted=1.0701266568570988

primary_failures:

- None.

regressions:

- None.

Original-eight ON/OFF geometric ratio: 0.995272; ON/accepted: 0.996838.

## Correctness and profiles

Fresh 36 field tests (33 existing plus three new), 112 core tests (109 plus three), raw2/cubic6, generated-IFMA and portable/native-noIFMA consumer checks pass. New independent Ark tests cover normalized widened operands at lazy boundaries for both fields. Additional raw-domain, scalar, random, sign and negative-zero checks and signed/unsigned tail/reuse/current-wrapper tests pass. Shared-allocation and unchanged native ABI sizes are asserted. The current generated field arithmetic body is unchanged.

All-nine OFF/ON preflight completes 72 SDK requests and 18 Gnark exports, with ON selection traces and no OFF selection. The full study completes 7,374 SDK requests and 231 distinct independently Gnark-verified exports / 462 proof-public members. Every export has the pinned public hash, 164-byte length and fresh identity against 6,331 prior proofs. The next prior is 6,562; repeated verification is not counted as new proof evidence.

Three primary32 profiles follow all 210 latency processes, each with 51 timed calls after three primes, gated after preparation, cycles:u at 199 Hz with `--no-buildid-cache`. There are 9,674 samples, zero lost records and one sampled process each. Categories are aggregate sampled user cycles and can include inlined work; they do not isolate stage wall time. Much of the new kernel can appear under its caller's G1 symbols, so the small named quarter category is not its full CPU share.

|Variant|Samples|Lost|Sampled processes|Quarter category %|
|---|---:|---:|---:|---:|
|accepted|3214|0|1|0.000|
|off|3315|0|1|0.000|
|on|3145|0|1|0.451|

All helpers compile before capture. Native binaries, source pins, input keys, fixtures, verifier and protocol hashes are retained. Raw profiles, binaries and large keys remain local with hashes. No AWS, Aeglos, phone, TEE, giant, swap or throughput measurement is added. A local pass alone would not authorize adoption; remaining resource/consumer/target qualification would still be required. The [latest C7a comparison](../2026-10-26-g2-tree-sdk/README.md) remains unchanged.

This closes the current-backend rebase. It reduces neither the field multiplier count nor the number of point additions. The small observation does not support further inline-attribute, packing or parameter sweeps to rescue the failed complete gate. A larger improvement needs a mechanism that reduces substantial arithmetic work or critical-path cost, with prior negative results checked first.

[Design](artifacts/design.md),[implementation audit](artifacts/implementation-audit.json),[analysis](artifacts/analysis.json),[request/profile rows](artifacts/performance/rows.json),[proof inventory](artifacts/proof-inventory.json),and[artifact index](artifacts/artifact-index.json) retain the evidence.
