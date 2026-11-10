# Fixed-mask scalar quarter conversion

**The scalar codec removes branches, but the complete candidate fails 8/48 checks and is not adopted.** G2 at 32 workers improves against the accepted-source control by about 0.8%; the combined G1/G2 result does not qualify. One G1 calibration also fails. This fixed implementation closes without a parameter rescue, unchanged rerun or SDK trial. Accepted local 1×2 full-proof latency remains **7.783594 ms**.

The change replaces only `Stored::encode` in the [quarter-scaled Window implementation](../2026-11-10-persistent-quarter-performance176/README.md). For canonical numerical `Y<p`, `p≡3 mod4` and `r=Y mod4`, modular quartering is exactly `floor(Y/4)+r·((p+1)/4)`. Two fixed masks and a four-word carry chain replace two conditional modular halvings. There is no lookup table, multiplication/division, allocation, representation/layout/scheduling change or expanded key/solver plan. The IFMA arithmetic is unchanged.

Native old/new outputs agree with independent BigUint results on **99,580 cases**: 1,276 boundary cases, 32,768 deterministic random inputs and 65,536 changing output-fed inputs. Canonical bounds and decoding are checked. The four existing prefix, complete-Window, worker and layout tests pass against the new coherent field/core: 1,695,744 independent Arkworks prefix point comparisons, 5,018,112 Window bucket comparisons and 220 threaded Windows on 15,045-point inputs, including every worker at 1/16/32 threads and twelve simultaneous callers through one eight-worker pool. This is new-code qualification; unchanged fixture oracles and controls are reused. Portable fallback qualification is inherited: the single changed method belongs to the native IFMA module, and its guards and all other source bytes remain exact.

|Externally visible scalar probe, native fat LTO|Old codec|New codec|
|---|---:|---:|
|Static instructions|105|63|
|Conditional jumps|2|0|
|Calls / integer multiply or divide|0 / 0|0 / 0|
|Register push bytes|48|32|

The probe proves the emitted branch change for this build. Its static instruction count is not a dynamic instruction count, cycle measurement or caller-speed prediction. Whole-Window counters below still show slightly more instructions with the new codec.

Four arms separate the mechanism from the accepted baseline:

|Arm|Implementation|
|---|---|
|REF|Retained API-only accepted-source core and original field1200|
|BASE|New coherent field/core, original Window path|
|OFF|Retained old quarter-scaled field/core and candidate Window|
|ON|New fixed-mask codec and quarter-scaled Window|

BASE and ON share an executable. BASE/REF checks equivalent original algorithms; OFF intentionally differs from the accepted baseline and is a mechanism control. Its driver argument selects the old candidate path. All role-to-selector mappings are frozen and checked.

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Six contexts use G1/G2, 15,045 points, width 12, column 0, batch 200 and 1/16/32 workers. Exact independent-Arkworks fixtures from the prior campaign are reused without regeneration. Each process runs three checked primes and seventeen measured waves. A wave includes fresh Window construction, G1 one-feed/G2 two-feed processing, queues, scalar paths, normalization, complete output equality, checksum and drop. Loading, pool setup and input hashing are excluded. **Each wave runs one Window per worker; it is not a whole MSM, proof or server-throughput measurement.** Worker i is pinned to CPU i, solo to CPU 2; requested stacks are 512 KiB.

Fifteen fixed rotated/reversed context/four-arm blocks are retained. Times are medians of process medians; paired ratios need not equal ratios of displayed times. Bounds are fixed ranks 4–12 of 15, nominal 96.484375% individual median coverage under independent exchangeable blocks, without a familywise guarantee.

|Context|REF ms|BASE ms|OFF ms|ON ms|ON/REF [bounds], wins|ON/OFF [bounds], wins|
|---|---:|---:|---:|---:|---|---|
|G1 / 1 worker|0.327948|0.326676|0.302811|0.308241|0.939848 [0.938482,0.941988], 15/15|1.016776 [1.014337,1.020328], 0/15|
|G1 / 16 workers|0.405294|0.402388|0.403220|0.389224|0.958029 [0.953359,0.974478], 15/15|0.969446 [0.956161,0.976356], 15/15|
|G1 / 32 workers|0.600492|0.590994|0.605411|0.598178|1.007402 [0.981794,1.030870], 7/15|0.989255 [0.974237,1.016109], 10/15|
|G2 / 1 worker|1.229759|1.240258|1.220220|1.219007|0.991263 [0.990103,0.992885], 15/15|1.001096 [0.996616,1.002098], 7/15|
|G2 / 16 workers|1.499577|1.504807|1.515647|1.502062|0.999779 [0.998700,1.004487], 8/15|0.992467 [0.979385,0.999879], 12/15|
|G2 / 32 workers|2.399483|2.424370|2.396858|2.383062|0.991967 [0.987327,0.997543], 12/15|0.992201 [0.984591,1.003425], 11/15|

G2 32-worker ON/BASE is **0.983470 [0.976918,0.992162]**, 15/15 wins. Its ON/REF improvement is retained, while the codec-only ON/OFF interval includes parity. G1 at 16 workers improves against all three controls, but its solo ON/OFF loses every block. Complete comparisons against BASE, OFF and REF remain in the analysis.

Equal-log G1/G2 32-worker ON/BASE is **1.001049 [0.993549,1.013401]**, 7/15 wins; ON/OFF **0.993395 [0.983136,1.000087]**, 11/15; ON/REF **1.003262 [0.988281,1.019617]**, 7/15. All three aggregate gain checks fail. No favorable worker or group subset is promoted.

The eight failed checks comprise three nonregressions, one calibration, one primary gain and three aggregate gains. G1 32-worker BASE/REF is **0.987951 [0.961383,1.002002]**; its lower bound lies outside the declared factor-1.03 calibration range. This limits attribution of that context's exact differences solely to the new algorithm. Five other calibrations pass. All six RSS guards and all twelve diagnostic-quality checks pass. No bound, observation, population, worker configuration or gate changes after capture; there is no minimum gain floor or extension.

|32-worker context / arm|Maximum process RSS MiB|Median CPU ms/Window|Minor faults/Window|
|---|---:|---:|---:|
|G1 BASE|26.00|0.598879|0.080882|
|G1 OFF|25.75|0.618840|0.086397|
|G1 ON|25.50|0.614985|0.095588|
|G2 BASE|36.00|2.388803|0.281250|
|G2 OFF|35.75|2.362471|0.246324|
|G2 ON|36.00|2.351178|0.238971|

Major-fault medians are zero. These are short process high-water observations including fixtures and mappings, not minimum RAM, allocator overhead, pressure/swap or full-proof memory peaks. Bucket/record byte sizes and allocation sites remain unchanged; no new process-memory saving is established.

After every latency process, six FIFO-gated counter panels and six separate user-cycle profiles each run 2,048 waves. All counters report 100% scheduled coverage. Profiles use a 16-million-cycle period and 512 pages; all six pass PID/TID/CPU/executable-MMAP checks, zero LOST records and unknown-symbol limits. They contain **157,710 samples**, with unknown shares below 0.47%.

|Per complete Window, 32-worker panel|G1 BASE|G1 OFF|G1 ON|G2 BASE|G2 OFF|G2 ON|
|---|---:|---:|---:|---:|---:|---:|
|Instructions|4,662,211|4,792,561|4,804,836|19,206,677|19,225,231|19,339,629|
|User cycles|2,600,834|2,644,918|2,616,287|10,435,553|10,222,305|10,126,043|
|Branch misses|4,795|7,774|5,734|7,231|13,393|9,214|
|Generic cache misses|4,151|3,953|3,971|22,107|25,423|25,173|

Against old quarter-scaled OFF, branch misses fall about 26%/31% for G1/G2, while instructions rise slightly. The scalar probe therefore does not predict the complete caller's instruction count. These are separate single diagnostic panels, not replicated causal estimates, exclusive wall-time fractions or identification of a DRAM/cache-level bottleneck. They support retaining the branch reduction as evidence, without promoting the complete candidate.

All **426 capture commands succeed once**: 24 preflights, 360 latency processes, six PMU panels, six profiles and thirty offline steps. All **906,752 Window invocations** match the independent oracle and preserve inputs. Frozen producer and separate same-agent raw reader pass first execution, reconstructing all 48 decisions and resource rows; PMU/sample schema parsers are shared. The native 13-command and consumer/tool five-command workflows also pass. Including two post-analysis commands, **all 446 workflow commands succeed**. All 607 capture pins and 176 production source identities remain exact.

One setup generator fails before producing a build plan: an earlier count replacement changes a later expected progress-message string. Its one-string repair changes only generation bookkeeping. Original generator source/binary and transcript failure are retained; successful native builds/tests are reused. This setup call is outside the 446 workflow receipts. No successful recording is replayed.

The next distinct hypothesis is to defer initial bucket conversion into vector processing. Since `2p<2²⁵⁵`, an unused high bit in the existing opaque Y storage could distinguish original canonical Y from converted quarter-Y, without enlarging a bucket or touching keys. Every scalar/vector load, zero check and final decode would need to honor that state; tagged words must never enter canonical `Fq`. This is a proposed lifetime change, not an implemented or measured improvement. Its extra tag tests and conversion paths must be priced before another experiment.

No SDK request, fresh proof, Gnark export, AWS/Aeglos, phone/ARM, huge-circuit or full-proof result is added. Canonical proof inventory remains 44,265. Accepted local 32-worker full-proof observations and supplied M5 references below are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md).

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 / 4.754×|24 / 3.083×|3.7|
|2×2|10.932731|59 / 5.397×|30 / 2.744×|5.9|
|4×2|15.757088|109 / 6.918×|48 / 3.046×|10.9|
|8×2|23.973295|189 / 7.884×|74 /3.087×|18.9|
|12×2|33.827269|284 / 8.396×|101 /2.986×|28.4|
|16×2|39.900917|335 / 8.396×|124 /3.108×|33.5|
|24×2|66.041083|528 / 7.995×|176 /2.665×|52.8|
|merge-24×1|65.717994|507 / 7.715×|175 /2.663×|50.7|

The full verified CPU 10× goal remains active and unmet, **0/8**. Accepted primary16 remains 9.016864 ms; merge54 remains 162.170158 ms without a supplied M5 reference. Historical primary proof RSS of 111.25/133.50 MiB belongs to different campaigns and is not remeasured here.

[Analysis summary](artifacts/analysis.json), [raw reader](artifacts/review.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.md), [native arithmetic](artifacts/arithmetic.json), [emitted-code qualification](artifacts/structural.json), [source audit](artifacts/source-audit.json), [codec](artifacts/encode.rs), [prospective freeze](artifacts/prospective-review.json), [identities](artifacts/identities.json), [command index](artifacts/receipts.json), [symbols](artifacts/symbols.json), [setup failure](artifacts/failures.md), [continuation/autoresearch loop](artifacts/next.md), [raw/public bindings](artifacts/public-views.json), [source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Public analysis omits only duplicated per-job rows. Every raw row and full command remains archived. Fixture bytes, binaries/libraries, raw perf payloads, unchanged source snapshots and full ledgers are excluded.
