# Known-unit prefix boundaries in complete MSM update batches

Caller gate: **false**. No production change or adoption follows from this experiment. It tests the complete G1/G2 borrowed update kernels, including packing, inversion and output scatter, rather than a field-only operation. No new SDK proof or AWS measurement is claimed.

All four solo contexts improve against OFF with 15/15 wins, with paired savings of about 0.37–0.80%. At 32 workers, G1's two-size ON/OFF aggregate is 0.994397 [0.884217, 1.074659], 9/15 wins; G2 is 1.009333 [0.995979, 1.028817], 5/15 wins. Neither establishes an aggregate gain. Six checks fail: one G2 control calibration, one G2 ON/OFF non-regression, and all four curve/control aggregate gains. These results close this exact implementation without an unchanged rerun or favorable-topology promotion.

The first prefix is one. The G1 candidate seeds its first numerator with signed dy and its chain with dx, omitting two unit multiplications. The final reverse group skips the unused inverse update. G2 seeds its norm chain directly, then uses the final inverse directly without multiplying by prefix one or updating the unused inverse. Each eligible batch removes three eight-lane field multiplications; mathematical affine-addition and inversion counts stay unchanged. Explicit first/last peeling can enlarge code, change allocation-check placement and affect the optimizer, so the source count is not a guaranteed cycle saving.

The original G1 stream and G2 borrowed functions remain byte-exact. A private child supplies the changed variants. Existing field, representation, input encoding, batch size, queue, schedule and scratch types are unchanged. The first unscaled differences/norm remain within the accepted normalized lazy bounds. Short batches with fewer than eight points retain the original function. Tests cover ordinary stream-domain batches; exception admission remains outside these kernels. There is no table, key copy, solver plan, new pool, new input-sized allocation or arbitrary RAM cap.

The [history audit](artifacts/history.md) distinguishes this from previous independent-prefix-chain, norm-heap and balanced-denominator-tree designs. Their failed gates remain failed. The preceding weighted-SDK result also remains unadopted. No old positive component estimate or external paper gain is transferred to this experiment.

Ryzen 9950X, 16 physical cores / 32 logical CPUs. Fixed G1/G2 sizes 200/352 run at 1/16/32 workers. Original, candidate-OFF and candidate-ON use 15 rotated/reversed blocks. OFF/ON share one optimized native/fat-LTO/one-codegen-unit executable; original is separately compiled without the new child. Per-worker CPU pinning uses CPU 2 solo or CPUs 0–15/0–31, with 512 KiB stacks. Each process has three checked 128-update primes, then 4,096 updates per worker. Whole-wave wall includes dispatch/join and the slowest worker. Each worker owns different synthetic generator inputs, not a recorded witness stream. All outputs are checked against generator-scalar sums outside the timer, and capacity/zero-timed-allocation checks are enforced. These are batch-wave timings, not proof latency.

Each curve's 32-worker two-size equal-log ON/control aggregate requires upper rank 12/15 < 1 and at least 10 wins against both controls. Every context upper ON/control bound must be at most 1.03, and OFF/original bounds must lie within factor 1.03. Ranks 4–12 have nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise coverage. There is no minimum-gain floor, excluded result, sample extension, layout sweep or unchanged reroll. Full-proof translation would require its own proof/compatibility/resource qualification.

|Curve / control|32-worker aggregate ON/control ratio [bounds], wins|
|---|---|
|G1/OFF|0.994397 [0.884217, 1.074659], 9/15|
|G1/frozen|0.957910 [0.897583, 1.006683], 10/15|
|G2/OFF|1.009333 [0.995979, 1.028817], 5/15|
|G2/frozen|0.998741 [0.987364, 1.019850], 8/15|


|Context|Original / OFF / ON µs per wave|ON/OFF [bounds], wins|ON/original [bounds], wins|OFF/original calibration|
|---|---:|---|---|---|
|G2-200-w1|7.665 / 7.658 / 7.598|0.991955 [0.990070, 0.993219], 15/15|0.991229 [0.989816, 0.992476], 15/15|0.999458 [0.997996, 1.000756], 9/15|
|G2-200-w16|8.912 / 8.910 / 8.867|0.995573 [0.992371, 0.998368], 14/15|0.994737 [0.990789, 0.996235], 13/15|1.000170 [0.994254, 1.003175], 6/15|
|G2-200-w32|14.730 / 14.372 / 14.524|1.008478 [0.971040, 1.036765], 6/15|0.975018 [0.956040, 1.027163], 10/15|0.993429 [0.966987, 1.001697], 11/15|
|G2-352-w1|12.990 / 12.984 / 12.900|0.993675 [0.993405, 0.994542], 15/15|0.992896 [0.992045, 0.993595], 15/15|0.998807 [0.998183, 0.999283], 14/15|
|G2-352-w16|15.192 / 15.255 / 15.136|0.992839 [0.990770, 0.997368], 14/15|0.996803 [0.995446, 1.000473], 11/15|1.004335 [1.000828, 1.006762], 3/15|
|G2-352-w32|25.020 / 25.085 / 25.403|1.009741 [0.994472, 1.021904], 5/15|1.012589 [0.997330, 1.028970], 4/15|1.006411 [0.991502, 1.013140], 5/15|
|G1-200-w1|4.220 / 4.214 / 4.186|0.993259 [0.991196, 0.994404], 15/15|0.992407 [0.990485, 0.993465], 15/15|0.998647 [0.997421, 1.000016], 11/15|
|G1-200-w16|4.778 / 4.803 / 4.781|0.991184 [0.987213, 1.003038], 10/15|0.999915 [0.990604, 1.003509], 8/15|1.002242 [0.999526, 1.009691], 4/15|
|G1-200-w32|6.906 / 6.951 / 6.852|0.992278 [0.981598, 1.019525], 11/15|0.997301 [0.982094, 1.023600], 8/15|1.000623 [0.974002, 1.012730], 7/15|
|G1-352-w1|6.942 / 6.933 / 6.912|0.996334 [0.994917, 0.997281], 15/15|0.995096 [0.994389, 0.995734], 15/15|0.998584 [0.997415, 1.000151], 11/15|
|G1-352-w16|7.934 / 7.920 / 7.863|0.992231 [0.989993, 0.993855], 15/15|0.991396 [0.990246, 0.992820], 15/15|0.999906 [0.998160, 1.000910], 8/15|
|G1-352-w32|11.674 / 11.775 / 11.558|0.987859 [0.794724, 1.022516], 9/15|0.991998 [0.793960, 1.018959], 10/15|1.007772 [0.984612, 1.024456], 6/15|


Retained failed checks: `["G2-200-w32/OFF/frozen/calibration","G2-200-w32/ON/OFF/nonregression","G1-32aggregate/ON/frozen/gain","G1-32aggregate/ON/OFF/gain","G2-32aggregate/ON/frozen/gain","G2-32aggregate/ON/OFF/gain"]`.

|Context|Original / OFF / ON CPU ms per wave|Original / OFF / ON maximum-RSS counter MiB|Record / tail bytes per fixture, all arms|
|---|---:|---:|---:|
|G2-200-w1|0.007666 / 0.007659 / 0.007599|2.75 / 2.75 / 2.75|`[16000,0]`|
|G2-200-w16|0.138368 / 0.138391 / 0.137540|5.00 / 5.25 / 5.25|`[16000,0]`|
|G2-200-w32|0.450728 / 0.450671 / 0.453708|7.50 / 7.75 / 7.75|`[16000,0]`|
|G2-352-w1|0.012991 / 0.012985 / 0.012901|3.00 / 3.00 / 3.00|`[28160,0]`|
|G2-352-w16|0.236074 / 0.236324 / 0.235765|9.50 / 9.50 / 9.25|`[28160,0]`|
|G2-352-w32|0.776372 / 0.777036 / 0.785093|15.50 / 15.50 / 15.00|`[28160,0]`|
|G1-200-w1|0.004221 / 0.004214 / 0.004187|2.50 / 2.75 / 2.75|`[40000,0]`|
|G1-200-w16|0.073629 / 0.073706 / 0.072996|3.75 / 3.75 / 3.75|`[40000,0]`|
|G1-200-w32|0.214170 / 0.214562 / 0.212584|5.00 / 5.00 / 5.00|`[40000,0]`|
|G1-352-w1|0.006943 / 0.006933 / 0.006913|2.75 / 2.75 / 2.75|`[70400,0]`|
|G1-352-w16|0.122498 / 0.122488 / 0.121457|5.50 / 5.25 / 5.50|`[70400,0]`|
|G1-352-w32|0.361702 / 0.362364 / 0.359458|8.50 / 8.25 / 8.75|`[70400,0]`|


All arms reuse the same record and tail capacities, with zero timed allocation calls. Recorded process RSS counters include setup/primes and are diagnostic; previous page-table audits found counter undercounts on this host. No fresh smaps/pressure or minimum-RAM qualification is inferred. Equal requested scratch does not bound compiler stacks or total prover memory.

All 540 latency processes precede twelve grouped CPU-counter panels and twelve separate user-cycle profiles, covering both sizes and all arms at 32 workers. Counters use 16,384 updates/worker; profiles use 65,536, after excluded primes via FIFO. No owned build/helper/hash/archive/analyzer or other workload overlaps latency/profiles. PMU running, binary mmap/PID/FORK/registered-worker TID and CPU, fixed 16M period and zero-loss evidence are checked. Profiles require at least 400 events and at most 5% unknown; thin or failed diagnostics remain reported. All 93509 sampled events are independently reconstructed. All twelve profiles qualify: at least 3,816 samples, at most 0.1568% unknown, and zero lost records. Flat samples, including inlined work, are not exclusive critical-path time, dynamic spills or validated absolute-period coverage.

|Counter panel|Cycles per wave|Instructions per wave|Cache misses per wave|Branch misses per wave|Profile samples|
|---|---:|---:|---:|---:|---:|
|stat-G1-200-w32-OFF|951734|1678194|21.068|444.850|3869|
|stat-G1-200-w32-ON|935285|1663445|52.285|442.852|3816|
|stat-G1-200-w32-frozen|953278|1678179|16.656|452.016|3863|
|stat-G1-352-w32-OFF|1566136|2725715|46.500|446.420|6378|
|stat-G1-352-w32-ON|1553773|2714672|63.534|440.764|6332|
|stat-G1-352-w32-frozen|1553697|2725858|30.399|439.042|6364|
|stat-G2-200-w32-OFF|1885611|3557188|15.372|464.984|7707|
|stat-G2-200-w32-ON|1881556|3531960|13.879|458.611|7686|
|stat-G2-200-w32-frozen|1888596|3557216|23.576|457.817|7702|
|stat-G2-352-w32-OFF|3250366|6033103|125.419|469.676|13271|
|stat-G2-352-w32-ON|3245749|6004252|206.726|473.710|13256|
|stat-G2-352-w32-frozen|3247831|6033141|123.925|470.854|13265|


|Compiled context / function|Bytes|
|---|---:|
|control / `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|26198|
|control / `zopro_core::msm::g2_borrowed::apply::<zopro_bn254::curve::G2Config, true>`|51830|
|candidate / `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, false, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|25197|
|candidate / `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|26198|
|candidate / `zopro_core::msm::unit_prefix::g1::<zopro_bn254::curve::G1Config, true, false, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|27777|
|candidate / `zopro_core::msm::unit_prefix::g1::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|28743|
|candidate / `zopro_core::msm::g2_borrowed::apply::<zopro_bn254::curve::G2Config, true>`|51830|
|candidate / `zopro_core::msm::unit_prefix::g2::<zopro_bn254::curve::G2Config, true>`|75589|


Linked functions contain native IFMA. Static byte sizes show code footprint only; they do not measure dynamic multiplication savings, stack peaks or cycle cost. The separate original functions and candidate OFF can differ in code layout/inlining despite exact source bodies.

The defining G2 boundary function grows from the original's 51,830 bytes to 75,589 bytes. Boundary peeling exposes more code to the optimizer, even though the source executes fewer products. Its higher 32-worker CPU service accompanies the uncertain/slower elapsed result; no exclusive instruction-cache, register, memory or scheduler cause is established. G1 CPU service falls in all six contexts, while its 32-worker elapsed ratios remain variable. Retained solo and 16-worker positive estimates do not change the full gate.

Native qualification passes all 128 core tests: 125 existing plus three new tests covering signed/unsigned complete generator references, sizes 0–512 including tails, repeated OFF/ON transitions, equal retained capacities and 1/4/16/32 workers on 512 KiB stacks. All field source files and 35-test accepted qualification are inherited byte-exact; no field suite is rerun and no new field formula is introduced. Control/candidate snapshots contain 177/178 files, with 174 originals exact in each. Reversing two append-only parents recovers all 352 original copies. Original stream bodies and complete fixture driver bytes are independently checked. No standalone prover suite, portable or deployment qualification is claimed at this boundary.

All 660 workflow commands and 600 complete caller processes retain command/exit/stream receipts. Independent review recomputes all 540 latency-process observations and 36 cells, paired intervals/aggregates/gates, capacity/digest equality, PMU values and every profile bin/ownership check, and rechecks source/binary/protocol pins. No successful test, caller or capture is rerun for analysis. No fresh SDK proof, independent export or solver request occurs; the canonical verified ledger remains 28,847.

Inherited accepted whole-proof results against supplied historical M5 references follow. They are unchanged and are not candidate full-proof timings. The accepted source's original physical16 calibration exception and failed full-performance gate remain recorded.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37.0|4.667×|24.0|3.027×|3.7|
|2x2|11.151932|59.0|5.291×|30.0|2.690×|5.9|
|4x2|16.086386|109.0|6.776×|48.0|2.984×|10.9|
|8x2|24.179453|189.0|7.817×|74.0|3.060×|18.9|
|12x2|33.884455|284.0|8.381×|101.0|2.981×|28.4|
|16x2|40.077381|335.0|8.359×|124.0|3.094×|33.5|
|24x2|65.850777|528.0|8.018×|176.0|2.673×|52.8|
|merge-24x1|65.060937|507.0|7.793×|175.0|2.690×|50.7|


The all-eight verified CPU 10× objective remains active and unmet: 0/8. Primary accepted 7.928660 ms still requires 3.7 ms. Accepted merge54 is 161.883899 ms without a supplied M5 reference. Historical AWS/Aeglos observations predate source176 and are not refreshed here. No AWS resource, phone/ARM/TEE, huge 250/4999, pressure/swap, concurrent-proof or minimum-RAM result follows.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [source audit](artifacts/source-audit.json), [linked code](artifacts/codegen.json), [symbols](artifacts/profile-symbols.json), [publication audit](artifacts/publication.json). Archives: [source snapshots](artifacts/source.tar.gz), [Rust workflow and protocol](artifacts/workflow.tar.gz), [all receipts](artifacts/receipts.tar.gz), [counters](artifacts/counters.tar.gz), [selected disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS). Keys, private witnesses/inputs, executables, libraries and raw perf.data remain local. Root:`/home/ubuntu/src/zopro-perf-work/unit-prefix176`.
