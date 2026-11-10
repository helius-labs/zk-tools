# Persistent quarter-scaled Windows: performance comparison

**The candidate fails the complete-Window screen and is not adopted.** Solo G1 improves, but the benefit disappears with more workers. At 32 workers G1 ON/OFF is **1.016719 [1.004700,1.041217]**, with 3/15 wins; G2 is **1.004137 [0.994473,1.010889]**, with 4/15 wins. All six fresh-OFF/accepted-source calibrations pass. The fixed implementation closes without an unchanged rerun or SDK trial. Accepted local 1×2 full-proof latency remains **7.783594 ms**.

This is the first timing of the [qualified complete G1/G2 integration](../2026-11-10-persistent-quarter-window176/README.md). Canonical X and opaque quarter-scaled Y persist between updates; incoming keys keep their original format. Long-prefix inverses, scaled scalar tails, exceptions, projective spills, queues, Phi and final decoding are included. Bucket payloads remain 64/128 bytes. No converted key, lookup table, solver plan or production pool is introduced. The IFMA product count remains unchanged.

Fresh OFF and ON use the same executable and core, selecting their original/candidate Window path by argument. REF links an API-only accepted-source core and original field1200; its 172 exact parents and four append-only helpers recover all 176 accepted sources. Qualified ON field/core libraries are reused without rebuilding. Only the missing REF core, two consumers and independent fixture helper are built.

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Each worker receives independently seeded descriptors for 15,045 ordinary points, width 12, column 0, batch 200. Points are shared and immutable. An offline Arkworks generator computes complete expected bucket vectors using its own bit decoder and explicit G2 Phi map. Three checked primes precede seventeen timed waves per latency process. Each wave includes fresh Window construction, G1 one-feed/G2 two-feed processing, queues, scalar paths, normalization, full output equality, a fixed two-point checksum and drop. Loading, pool construction and post-input hashing are excluded.

**A wave runs one complete Window per worker; it is not a complete MSM, proof or server-throughput measurement.** Worker i uses CPU i; solo uses CPU 2. Requested stacks are 512 KiB. Fifteen fixed rotated/reversed context/arm blocks are retained. Times below are medians of process medians; paired-ratio medians need not equal ratios of displayed times. Bounds use fixed ranks 4–12 of 15, nominal 96.484375% individual median coverage under independent exchangeable blocks, without a familywise guarantee.

|Context|REF ms/wave|OFF ms/wave|ON ms/wave|ON/OFF [bounds], wins|ON/REF [bounds], wins|
|---|---:|---:|---:|---|---|
|G1 / 1 worker|0.327678|0.326605|0.302480|0.927265 [0.922557,0.928517], 15/15|0.923506 [0.921861,0.927668], 15/15|
|G1 / 16 workers|0.405764|0.404492|0.401357|0.991400 [0.982794,1.007260], 8/15|0.992751 [0.985163,1.007604], 10/15|
|G1 / 32 workers|0.594941|0.590584|0.605651|1.016719 [1.004700,1.041217], 3/15|1.014837 [1.006939,1.039470], 2/15|
|G2 / 1 worker|1.228566|1.224928|1.218397|0.992795 [0.991409,0.994262], 15/15|0.990014 [0.988662,0.992842], 15/15|
|G2 / 16 workers|1.501370|1.500077|1.512120|1.009930 [1.004132,1.018126], 1/15|1.010926 [1.004967,1.016712], 2/15|
|G2 / 32 workers|2.401136|2.384404|2.397960|1.004137 [0.994473,1.010889], 4/15|0.998950 [0.990352,1.004958], 9/15|

The positive solo estimates are retained: G1 is about 7.27% faster than OFF and G2 about 0.72%, both winning all fifteen paired blocks. These observations do not establish a mobile result or offset the threaded failures. Equal-log G1/G2 32-worker ON/OFF is **1.011350 [1.004420,1.021227]**, 2/15 wins; ON/REF is **1.006962 [0.997043,1.017938]**, 4/15. No favorable worker subset is promoted.

The frozen screen fails **6/36 checks**: two G1 32-worker nonregression checks, two primary G2 32-worker gain checks and both aggregate gains. All six calibrations, all six RSS guards and all eight diagnostic-quality checks pass. The protocol requires calibration intervals inside factor 1.03, all ON/control nonregression upper ratios ≤1.02, primary/aggregate gain upper ratios <1 with ≥10 wins against both controls. There is no minimum percentage gain, post-capture exclusion, extension or parameter change.

|32-worker context / arm|Maximum process RSS MiB|Median CPU ms/Window|Minor faults/Window|
|---|---:|---:|---:|
|G1 OFF|25.75|0.605928|0.082721|
|G1 ON|25.50|0.616401|0.086397|
|G2 OFF|36.00|2.338825|0.270221|
|G2 ON|36.25|2.363301|0.240809|

Major-fault medians are zero. RSS includes fixture loading and process mappings; these short observations are not minimum RAM, pressure/swap results or a full-proof peak. The preceding allocation qualification found identical requested live payload peaks. Neither that observation nor fewer G2 minor faults establishes that RAM explains this latency difference.

After every latency process, four FIFO-gated counter panels and four separate user-cycle profiles run 2,048 waves each. All grouped counters report 100% scheduled coverage. Profiles use a 16-million-cycle period and 512 pages; all four pass PID/TID/CPU/executable-MMAP checks, zero LOST records and unknown-symbol limits. They contain **104,869 samples**, with unknown shares below 0.47%.

|Per complete Window, 32-worker panel|G1 OFF|G1 ON|G2 OFF|G2 ON|
|---|---:|---:|---:|---:|
|Instructions|4,665,434|4,791,737|19,234,000|19,225,172|
|User cycles|2,609,668|2,644,961|10,203,073|10,221,294|
|Branch misses|4,709|7,772|7,206|13,464|
|Generic cache misses|3,969|3,894|22,264|25,477|

The G1 counter panel has more instructions and cycles; both panels have substantially more branch misses. Flat samples move from the arithmetic routine toward its surrounding Window work: G1 arithmetic 6,386→6,013 while its Window entry rises 2,018→2,790; G2 arithmetic 31,353→30,090, feed 3,949→4,741 and Window entry 1,815→2,626. These are separate diagnostic panels, not replicated causal estimates, exclusive wall-time fractions or attribution of branch misses to a particular source line. Generic misses identify neither a cache level nor DRAM bandwidth.

Source inspection identifies a concrete follow-up hypothesis: scalar `Stored::encode` quarters Y through two conditional modular halvings at initial bucket insertion and scalar boundaries. For canonical numerical Y and `p ≡ 3 mod 4`, the exact identity `quarter(Y) = floor(Y/4) + (Y mod 4)·((p+1)/4)` permits a distinct fixed-mask scalar codec. This may reduce boundary branches without a table or changed layout. Its complete cost and emitted code must be checked before a new experiment; this report does not establish that those branches caused the regression or that replacing them will help.

All **316 capture commands succeed once**: 18 preflights, 270 latency processes, four PMU panels, four profiles and twenty offline decodes/reviews. All **614,432 Window invocations** match their independent oracle; inputs remain unchanged. Frozen producer and separate same-agent raw reader both pass first execution, reconstructing all 36 decisions and resources. They share PMU/sample schema parsers. All 399 capture pins and 176 production source identities remain exact.

Across five build/fixture commands, three analysis-tool build attempts, capture and two post-analysis commands, **326 workflow commands comprise 325 successes and one retained compile failure**. The first analysis-tool compile moved a path before hashing it; the repair borrows that path. No core, driver, fixture, successful capture or numerical analysis is rebuilt/replayed for the repair. Its original source, command and stderr remain. A read-only JSON inspection query also needed its projection corrected; it changes no artifact or scientific result. Setup/publication helper calls are outside this workflow count.

No SDK request, Gnark export, AWS/Aeglos, phone/ARM, huge-circuit or full-proof result is added. Canonical proof inventory remains 44,265. Accepted local 32-worker full-proof observations and supplied M5 references below are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md).

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 / 4.754×|24 / 3.083×|3.7|
|2×2|10.932731|59 / 5.397×|30 / 2.744×|5.9|
|4×2|15.757088|109 / 6.918×|48 / 3.046×|10.9|
|8×2|23.973295|189 / 7.884×|74 / 3.087×|18.9|
|12×2|33.827269|284 / 8.396×|101 / 2.986×|28.4|
|16×2|39.900917|335 / 8.396×|124 / 3.108×|33.5|
|24×2|66.041083|528 / 7.995×|176 / 2.665×|52.8|
|merge-24×1|65.717994|507 / 7.715×|175 / 2.663×|50.7|

The full verified CPU 10× goal remains active and unmet, **0/8**. Accepted primary16 remains 9.016864 ms; merge54 remains 162.170158 ms without a supplied M5 reference. Historical primary proof RSS of 111.25/133.50 MiB belongs to different campaigns and is not remeasured here.

[Analysis summary](artifacts/analysis.json), [raw reader](artifacts/review.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.md), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [fixture identities](artifacts/fixture-identities.json), [command index](artifacts/receipts.json), [symbols](artifacts/symbols.json), [preserved failure](artifacts/failures.md), [continuation/autoresearch loop](artifacts/next.md), [raw/public bindings](artifacts/public-views.json), [source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Public analysis omits only duplicated per-job rows; every raw row and full command remains in the indexed archive. Fixture bytes, binaries/libraries, raw perf payloads, unchanged source snapshots and full proof ledgers are excluded.
