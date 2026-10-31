# Window-major signed digits in complete G1 fillers

**The caller gate fails; the candidate is not adopted.** At 32 workers, the two-shape equal-log aggregate is 17.07% slower than the same executable with the path disabled and 17.84% slower than the separate original control, with zero winning blocks against either. Both solo cases retain small positive ON/OFF estimates of about 0.20–0.24%. There is no full-proof gain or unchanged rerun.

Primary32 original/OFF/ON complete-filler medians are 0.857175/0.858457/1.026736 ms. Primary16 measures 1.062996/1.048839/1.206869 ms. All four threaded contexts regress against OFF, with paired lower bounds above parity. Four control calibrations also fail and remain failed; neither control drift nor a particular cache/scheduler mechanism is established as the cause of the candidate's regression.

The original loops extract a signed Booth digit from each 32-byte canonical scalar row in every window. The candidate first materializes one flat window-major matrix of 21×N i16 digits, then feeds the existing independent width12 windows using contiguous digit slices. The copied feeder changes only its digit source; the original Window/feed bodies, point access order, collision queue, batch size, field kernels, exceptions and finish are retained. There is no bucket sorting, segmented native-column representation, additional point merge, weighted-tree change or field formula.

The source-level inspiration is [TACEO ark-algebra0.1.0](https://github.com/TaceoLabs/circom-helpers/blob/9236cfeae662456b6db4def818303e3783722e6a/ark-algebra/src/msm.rs), which uses window-major digits in its generic MSM. No external code or performance result is imported. This candidate uses Zopro's existing Booth reader, i16 digits and resource policy. Package/source hashes and the VCS identity are retained in [fixture bindings](artifacts/fixture-bindings.json). This is a test of the layout within Zopro, not a comparison with TACEO.

The matrix adds 42N requested payload bytes: 882,252 bytes for raw 1×2 and 1,538,754 for raw 2×2, plus the existing 64-byte forecast charge. Checked layout and `Reservation::analysis` admit it before fallible allocation; the token lasts through the final consumer. Overflow, missing headroom or allocation failure retains the original path without modifying inputs. The buffer is proof-local scratch, not a retained key table, witness cache, duplicate solver plan, new pool or arbitrary universal RAM cap. It adds a decoder phase and a barrier before the original window fills. All those costs, initialization, allocation, admission and drop are inside the measurement.

Ryzen 9950X, 16 physical cores / 32 logical CPUs. Raw A inputs contain 21,006/36,637 points; all 21 ordinary width-12 windows are filled per pass. They are raw pre-group populations, not necessarily the final grouped proof queries. Narrow top, final weighting, solving and proof verification are outside this caller. Whole-process CPU affinity is CPU 2 solo or CPUs 0–15/0–31; worker stacks are 512 KiB and workers may migrate within the mask. OFF/ON share one executable; the original is compiled separately without the new cfg. Actual final optimization, native targeting, fat LTO and one codegen unit are checked, and both binaries retain native IFMA stream functions. Exact source bodies do not imply identical surrounding machine-code layout.

The fixed campaign has 18 preflights, then 15 rotated/reversed blocks of all 6 contexts × 3 arms, with 20 timed passes after 3 checked primes and 1 checked post-pass. All 300 caller processes compare every prime/post bucket with the previously independently Arkworks-checked fixture oracle. Every ON call must enter the admitted matrix path and return its exact capacity; no successful request silently falls back to make the candidate appear faster. No new solver or oracle construction is performed.

The 32-worker two-shape equal-log gain requires an upper rank 12/15 below 1 and at least 10 wins against both controls. Every context's ON/control upper bound must be at most 1.03; every OFF/original interval must fit factor 1.03. Rank 4–12 bounds have nominal 96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, excluded block, sample extension, parameter/layout sweep or favorable-core promotion is used.

Two-shape 32-worker ON/frozen: **1.178422 [1.126035, 1.189582], 0/15**.

Two-shape 32-worker ON/OFF: **1.170651 [1.149125, 1.183500], 0/15**.

|Raw context|Original / OFF / ON ms per complete fill pass|ON/OFF [bounds], wins|ON/original [bounds], wins|OFF/original calibration|
|---|---:|---|---|---|
|1x2-w1|10.465766 / 10.463304 / 10.439738|0.997959 [0.995125, 0.999824], 12/15|0.995570 [0.994432, 0.998449], 14/15|0.998384 [0.996430, 1.001179], 9/15|
|1x2-w16|1.062996 / 1.048839 / 1.206869|1.143403 [1.132970, 1.168998], 2/15|1.132900 [1.029976, 1.144593], 3/15|0.990814 [0.965747, 1.004368], 11/15|
|1x2-w32|0.857175 / 0.858457 / 1.026736|1.201536 [1.134640, 1.235233], 1/15|1.186219 [1.168799, 1.218159], 0/15|1.001495 [0.966219, 1.032486], 6/15|
|2x2-w1|16.582960 / 16.591798 / 16.526598|0.997598 [0.995658, 0.998440], 14/15|0.998203 [0.997618, 0.998566], 14/15|1.000164 [0.999602, 1.001766], 7/15|
|2x2-w16|1.645225 / 1.636608 / 1.872692|1.129073 [1.116921, 1.157422], 2/15|1.137399 [1.078247, 1.148429], 1/15|1.003229 [0.979333, 1.089128], 7/15|
|2x2-w32|1.352106 / 1.346913 / 1.546069|1.159019 [1.116417, 1.183201], 0/15|1.146676 [1.118873, 1.177905], 0/15|0.999146 [0.963359, 1.030289], 8/15|


All 14 failed checks are retained: `["1x2-w16/OFF/frozen/calibration","1x2-w16/ON/frozen/nonregression","1x2-w16/ON/OFF/nonregression","1x2-w32/OFF/frozen/calibration","1x2-w32/ON/frozen/nonregression","1x2-w32/ON/OFF/nonregression","2x2-w16/OFF/frozen/calibration","2x2-w16/ON/frozen/nonregression","2x2-w16/ON/OFF/nonregression","2x2-w32/OFF/frozen/calibration","2x2-w32/ON/frozen/nonregression","2x2-w32/ON/OFF/nonregression","two-shapes-w32/ON/frozen/gain","two-shapes-w32/ON/OFF/gain"]`.

|Context|Original / OFF / ON CPU ms/pass|Original / OFF / ON max-RSS counter MiB (diagnostic)|Original / OFF / ON minor faults/pass|Added digit payload KiB|
|---|---:|---:|---:|---:|
|1x2-w1|10.466 / 10.463 / 10.440|14.25 / 14.25 / 15.00|0.15 / 0.15 / 0.35|861.574|
|1x2-w16|11.765 / 11.710 / 12.062|18.25 / 19.00 / 27.25|12.05 / 10.40 / 103.75|861.574|
|1x2-w32|15.966 / 15.897 / 17.083|19.50 / 19.50 / 27.75|20.35 / 22.05 / 68.80|861.574|
|2x2-w1|16.583 / 16.592 / 16.526|17.00 / 17.00 / 18.50|0.10 / 0.10 / 0.10|1502.689|
|2x2-w16|18.003 / 17.976 / 18.584|20.75 / 21.00 / 36.25|8.40 / 10.40 / 172.95|1502.689|
|2x2-w32|23.882 / 23.747 / 25.278|18.75 / 18.75 / 31.50|15.75 / 18.35 / 141.95|1502.689|


CPU service and minor faults increase in all four threaded contexts, while solo CPU is slightly lower. Primary32 OFF/ON CPU service is 15.897/17.083 ms per pass. Recorded max-RSS counters rise from 19.50 to 27.75 MiB and minor faults from 22.05 to 68.80 per pass. These are descriptive observations. Counters include allocator/worker behavior beyond the matrix's payload and do not isolate its cost. Previous audits found max-RSS counters below page-table snapshots on this host; they are diagnostic, not precise resident peaks or minimum RAM. No fresh smaps/pressure measurement is claimed. Minor faults are resolved without disk page-ins; their counts do not measure latency cost. The driver does not export major faults.

After all 270 latency processes, six grouped counter panels and six separate fixed-16M-user-cycle profiles measure primary16/32 across all arms, 200 passes after FIFO-excluded primes. No owned build, hashing, helper, archive, analyzer or other workload overlaps latency/profiles. All PMU events run at least 99.9%; all 4,333 sampled events pass binary mmap/PID/FORK/TID/CPU/period/zero-loss checks. Profiles contain 635–831 events, all below 1,000: they pass the declared 400-event floor but remain thin descriptive evidence. Unknown share is at most 0.8424%. Flat samples, including inlined work, are not exclusive critical-path fractions, dynamic spills or validated absolute-period coverage.

|Stat panel|Cycles M/pass|Instructions M/pass|Generic cache misses M/pass|Branch misses M/pass|Profile samples|Unknown share|
|---|---:|---:|---:|---:|---:|---:|
|stat-1x2-w16-OFF|60.229|147.719|0.113532|0.114140|661|0.7564%|
|stat-1x2-w16-ON|61.089|142.577|0.164069|0.118837|635|0.6299%|
|stat-1x2-w16-frozen|60.236|147.724|0.109274|0.116529|662|0.4532%|
|stat-1x2-w32-OFF|75.903|151.776|0.120244|0.124635|784|0.3827%|
|stat-1x2-w32-ON|81.048|147.945|0.162772|0.131827|831|0.8424%|
|stat-1x2-w32-frozen|76.574|151.766|0.121045|0.127043|760|0.0000%|

|Workers|Descriptive ON/OFF cycles ratio|ON/OFF instructions ratio|
|---|---:|---:|
|16|1.014277|0.965189|
|32|1.067790|0.974755|


The extra decode/allocation stage and changed readiness could outweigh the simpler digit loop; this study does not isolate those effects from field, allocator, cache or scheduler costs. The result closes this exact full-matrix implementation. It does not establish that digit extraction is expensive, that memory bandwidth is saturated, or that a global per-key table is needed.

Native qualification passes 128 unfiltered core tests: 125 original plus 3 new tests covering independent bit-boundary digit values/layout, complete generator-bucket references and repeated 1/4/16/32-worker pools, and checked overflow/empty fallback. All 35 accepted field tests are inherited through byte-exact field files; no new field suite is run. Both 177-file snapshots have 174 originals exact; reversing two append-only parent edits recovers all 352 original copies. Field, generator, original Window/point functions and the shared driver bytes are independently checked. There is no production hook or standalone prover/portable qualification at this boundary.

All 330 capture commands and 300 caller processes completed: 9,018 complete filler passes including primes/post-checks, 189,378 individual window fills, and 51,609,600 checked prime/post bucket outputs. Independent review recomputes all 270 latency observations / 18 cells, paired statistics/aggregates/gates, capacities, source recovery, counter values and every profile bin/ownership check. The 456 original pins remain unchanged. There are zero fresh SDK proofs, solver calls or Gnark exports; canonical ledger 28,847 remains unchanged. All-eight CPU 10× remains active and unmet.

Preparation failures are preserved. The first standalone control wrapper failed final compilation because its copied perf support lacked a direct libc dependency. Adding that existing dependency only to the two wrapper manifests reused successful cached libraries; the128core suite and kernel/source snapshots were not repeated or modified. The first analysis-helper builds needed a missing format argument and an explicit map-key type; its offline pass-count assertion was corrected before capture. The original independent review then stopped on exact JSON equality of a one-bit rounded ratio; a separate v2 reviewer uses the existing1e-12 float comparator for that array, keeping all integer decisions exact. [Preparation repair](artifacts/helper-repair.md) and [review repair](artifacts/review-repair.md) retain details; no successful measurement, proof, test or gate was repeated or altered.

Inherited accepted whole-proof comparisons against supplied historical M5 references follow. These are not candidate filler-to-proof extrapolations or fresh M5 measurements. The accepted physical16 calibration exception and original failed full-performance gate remain recorded.

|Shape|Accepted whole-proof ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37.0|4.667×|24.0|3.027×|3.7|
|2x2|11.151932|59.0|5.291×|30.0|2.690×|5.9|
|4x2|16.086386|109.0|6.776×|48.0|2.984×|10.9|
|8x2|24.179453|189.0|7.817×|74.0|3.060×|18.9|
|12x2|33.884455|284.0|8.381×|101.0|2.981×|28.4|
|16x2|40.077381|335.0|8.359×|124.0|3.094×|33.5|
|24x2|65.850777|528.0|8.018×|176.0|2.673×|52.8|
|merge-24x1|65.060937|507.0|7.793×|175.0|2.690×|50.7|


Accepted source176/SDK6d83 remains unchanged. It reaches 0/8 CPU 10× targets: primary 7.928660 ms requires 3.7 ms. Accepted merge54 is 161.883899 ms without a supplied M5 reference. No new AWS/Aeglos, phone/ARM/TEE, huge 250/4999, pressure/swap, concurrent-proof or minimum-RAM result follows. No AWS resource was created.

[Protocol](artifacts/design.md), [history](artifacts/history.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [source audit](artifacts/source-audit.json), [linked code](artifacts/codegen.json), [symbols](artifacts/profile-symbols.json), [publication audit](artifacts/publication.json). Archives: [source snapshots](artifacts/source.tar.gz), [Rust workflows and protocol](artifacts/workflow.tar.gz), [all receipts](artifacts/receipts.tar.gz), [counters](artifacts/counters.tar.gz), [selected disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS). Keys, private scalar/witness/oracle payloads, external package source, executables, libraries and raw perf.data stay local with hashes. Root:`/home/ubuntu/src/zopro-perf-work/digit-planes176`.
