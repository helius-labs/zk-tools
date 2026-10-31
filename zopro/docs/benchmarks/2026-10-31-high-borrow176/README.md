# Guarded borrow from the top SIMD limb

The complete-caller gate is **false**. The candidate is **not adopted**, and no SDK timing trial follows. Production source 176 / SDK `6d83…` remains unchanged.

Solo G1 regresses by 0.77–1.00% and solo G2 by 1.43–1.57% against fresh OFF, with zero wins in all four cases. All four solo control calibrations pass. The 32-worker G2 aggregate also regresses against OFF; the G1 aggregate is inconclusive. Two threaded control calibrations fail and remain in the result. The failure is not attributed solely to those calibrations.

|Curve / control|32-worker aggregate ON/control ratio [bounds], wins|
|---|---|
|G1/OFF|1.089202 [0.973282, 1.106560], 5/15|
|G1/frozen|0.998254 [0.923656, 1.088961], 8/15|
|G2/OFF|1.013121 [1.001729, 1.032805], 2/15|
|G2/frozen|1.010107 [0.995773, 1.020967], 4/15|


The new operation returns the original general subtraction's exact lazy representation. With normalized radix-52 limbs, unequal top digits determine whether `a < b`. If every lane's top digits differ, one signed carry pass evaluates `a − b + [a < b]·2p`. For inputs in `[0,2p)`, the result remains in `[0,2p)` and matches the original digits. If any top digits tie, all eight lanes use the complete original subtraction. There is no estimate, probabilistic correctness assumption, widened representation or changed point formula.

Only the eight-lane general subtraction changes. Canonical add/sub shortcuts, CIOS products, four-lane/scalar/portable kernels, keys, solver, queues, schedules and resource forecasts remain original. There is no table, heap buffer, transformed key, cached plan or RAM cap. New unsafe code contains SIMD intrinsics within the existing feature-qualified module; it adds no unchecked pointer arithmetic. The existing field generator is updated and passes its consistency check; new runners and helpers are Rust.

Earlier [bounded subtraction](../2026-10-21-ryzen-research/r260-bounded-subtraction/README.md) used reciprocal quotient selection; [lazy linear arithmetic](../2026-10-26-lazy-linear/README.md) selected small multiples of the modulus. The [canonical final-Y trial](../2026-10-24-canonical-y-output/README.md) classified differences and exported canonical coordinates at selected callsites. Their failed gates remain unchanged. This experiment tests the general lazy primitive, exact output digits and an equality fallback across its existing consumers. It makes no novelty claim.

The original diagnostic caller is freshly built from accepted source, then frozen before timing. Fresh OFF and ON share exact source and use separate target directories/executables, differing by `cfg(zopro_high_borrow)`. Each executable checks its CLI label against its compiled cfg before execution. OFF/ON identities are distinct. Actual final links use optimization level 3, native CPU, fat LTO and one codegen unit. The original and fresh OFF linked borrowed G1/G2 bodies have identical sizes and instruction-site counts; this does not remove all placement or host effects.

Native qualification passes **37 field tests** (35 existing plus two new) and **127 core tests** (125 existing plus two diagnostic caller tests). New field checks compare all five result vectors with the exact original routine, independent four-word integer subtraction and canonical Arkworks values across both primes and Montgomery radices. Coverage includes all 256 top-limb tie masks, limb/modulus boundaries, loose values and changing multiply/subtract chains. Core and diagnostic checks cover changing signed curve points, tails, scratch reuse and 1/4/16/32-worker pools with 512 KiB stacks. These do not constitute a new full prover, portable or pressure qualification.

The control snapshot has 177 files: 174 original files exact, two reversible diagnostic core suffixes and one child. The candidate has 178 files: 172 originals exact, four reversible parents and two children. Independent reversal recovers all 176 accepted files in each snapshot. The original subtraction body remains exact behind the guard. Four-lane and scalar field sources remain exact. The source audit also binds the generated field child and caller wrappers to their reviewed input fragments.

Ryzen 9950X, 16 physical cores / 32 logical CPUs, shared host. Solo pins CPU 2; 16/32 workers pin individually to CPUs 0–15/0–31, with 512 KiB stacks. Fixtures can be stolen between workers. Each process performs three reset, checked 128-batch primes, resets, then times 4,096 changing batches per fixture. Reference assembly, pool dispatch, arithmetic and waiting are timed; construction, reset and generator checks are outside the clock. Every final bucket after each prime and timed loop matches its independent generator reference. Intermediate timed states are not individually checked. All outputs agree across arms, with zero timed allocations and equal, stable scratch capacities. These are complete batch-caller measurements, not proof latency.

|Context|Original / OFF / ON µs per wave|ON/OFF [bounds], wins|ON/original [bounds], wins|OFF/original calibration|
|---|---:|---|---|---|
|G2-200-w1|7.662 / 7.655 / 7.768|1.014251 [1.013175, 1.015032], 0/15|1.013005 [1.012487, 1.014004], 0/15|0.999266 [0.998314, 1.000140], 10/15|
|G2-200-w16|8.911 / 8.920 / 9.019|1.011205 [1.009827, 1.013391], 0/15|1.012936 [1.009563, 1.013550], 0/15|1.001638 [0.997838, 1.002573], 7/15|
|G2-200-w32|14.357 / 14.284 / 14.969|1.045115 [1.004355, 1.069539], 2/15|1.025189 [0.997019, 1.053507], 4/15|0.992696 [0.963478, 1.001802], 11/15|
|G2-352-w1|12.982 / 12.967 / 13.170|1.015713 [1.015015, 1.015856], 0/15|1.014503 [1.013336, 1.014821], 0/15|0.998761 [0.998121, 0.999686], 13/15|
|G2-352-w16|15.181 / 15.267 / 15.625|1.023710 [1.017193, 1.025605], 1/15|1.027468 [1.022573, 1.030237], 0/15|1.005694 [1.001447, 1.009344], 1/15|
|G2-352-w32|25.069 / 24.949 / 24.968|0.997333 [0.992934, 1.007675], 9/15|0.995768 [0.980115, 1.005102], 9/15|0.996051 [0.986108, 0.999835], 12/15|
|G1-200-w1|4.210 / 4.207 / 4.239|1.007698 [1.006521, 1.008610], 0/15|1.007349 [1.006532, 1.008419], 0/15|1.000428 [0.996380, 1.001206], 7/15|
|G1-200-w16|4.761 / 4.766 / 4.771|1.003816 [0.996821, 1.005133], 6/15|1.002514 [0.998418, 1.005013], 4/15|1.000613 [0.998067, 1.004109], 7/15|
|G1-200-w32|6.854 / 6.881 / 6.932|1.006753 [0.982962, 1.165011], 4/15|1.015574 [1.002099, 1.164249], 2/15|1.001721 [0.990821, 1.024266], 6/15|
|G1-352-w1|6.930 / 6.921 / 6.990|1.009996 [1.008902, 1.010695], 0/15|1.009108 [1.007789, 1.009434], 0/15|0.998447 [0.997681, 1.000188], 11/15|
|G1-352-w16|7.931 / 7.912 / 7.964|1.006883 [1.004755, 1.009822], 1/15|1.004656 [1.000910, 1.005885], 2/15|0.995458 [0.994522, 0.999646], 12/15|
|G1-352-w32|14.567 / 11.736 / 14.005|1.027165 [0.804433, 1.220133], 6/15|0.970797 [0.796914, 1.018537], 11/15|0.981103 [0.789048, 0.998022], 12/15|


Each curve's equal-log aggregate over both 32-worker sizes must have upper ON/control ratio below one and at least 10/15 wins against both controls. Every context must have an upper ratio at most 1.03; every OFF/original interval must fit factor 1.03. Rank 4–12 of 15 gives nominal 96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. No gain floor, exclusions, extensions, favorable subset, changed threshold, parameter sweep or unchanged reroll is used. Positive estimates and uncertain comparisons remain in the tables.

Failed checks: `["G2-200-w32/OFF/frozen/calibration","G2-200-w32/ON/frozen/nonregression","G2-200-w32/ON/OFF/nonregression","G2-352-w16/ON/frozen/nonregression","G1-200-w32/ON/frozen/nonregression","G1-200-w32/ON/OFF/nonregression","G1-352-w32/OFF/frozen/calibration","G1-352-w32/ON/OFF/nonregression","G1-32aggregate/ON/frozen/gain","G1-32aggregate/ON/OFF/gain","G2-32aggregate/ON/frozen/gain","G2-32aggregate/ON/OFF/gain"]`.

|Context|Original / OFF / ON CPU ms per wave|Original / OFF / ON maximum-RSS counter MiB|Record / tail bytes per fixture, all arms|
|---|---:|---:|---:|
|G2-200-w1|0.007663 / 0.007656 / 0.007769|2.75 / 2.75 / 2.75|`[16000,0]`|
|G2-200-w16|0.138359 / 0.138478 / 0.139959|5.00 / 5.00 / 5.00|`[16000,0]`|
|G2-200-w32|0.450128 / 0.448500 / 0.451201|7.50 / 7.75 / 7.75|`[16000,0]`|
|G2-352-w1|0.012982 / 0.012968 / 0.013171|3.00 / 3.00 / 3.00|`[28160,0]`|
|G2-352-w16|0.236152 / 0.236418 / 0.239455|9.50 / 9.25 / 9.25|`[28160,0]`|
|G2-352-w32|0.777001 / 0.776630 / 0.775616|15.75 / 15.50 / 15.25|`[28160,0]`|
|G1-200-w1|0.004211 / 0.004207 / 0.004240|2.75 / 2.75 / 2.75|`[40000,0]`|
|G1-200-w16|0.073471 / 0.073454 / 0.073798|3.75 / 3.75 / 3.75|`[40000,0]`|
|G1-200-w32|0.214450 / 0.214288 / 0.216474|5.25 / 5.00 / 5.00|`[40000,0]`|
|G1-352-w1|0.006931 / 0.006921 / 0.006990|2.75 / 2.75 / 2.75|`[70400,0]`|
|G1-352-w16|0.122418 / 0.122373 / 0.123235|5.50 / 5.50 / 5.75|`[70400,0]`|
|G1-352-w32|0.363343 / 0.361133 / 0.364213|8.75 / 8.50 / 8.50|`[70400,0]`|


Scratch payloads are identical across all arms: G1 records use 40,000 / 70,400 bytes at 200 / 352 points; G2 norm records use 16,000 / 28,160 bytes. The timed sizes have no scalar-tail payload. Other fixture arrays, compiler stack, allocator overhead and inputs are additional. This is not a total RAM bound. Maximum-RSS counters are diagnostic because the earlier page-table audit found undercounts on this host. No fresh page-table, minimum-RAM or pressure qualification is claimed.

All 540 latency processes finish before twelve grouped four-event counter panels and twelve separate fixed-16M-user-cycle profiles. Counter panels measure 16,384 batches per worker and profiles 65,536, after three checked 128-batch primes excluded by FIFO acknowledgements. No owned build, helper, hashing, archive, analyzer or second workload overlaps latency/profiles. Recording ownership changes occur only after capture, with identical before/after hashes.

|Counter panel|Cycles per wave|Instructions per wave|Cache misses per wave|Branch misses per wave|Profile samples|
|---|---:|---:|---:|---:|---:|
|stat-G1-200-w32-OFF|950693|1678187|10.487|438.921|3883|
|stat-G1-200-w32-ON|967178|1639813|10.832|437.977|3934|
|stat-G1-200-w32-frozen|953934|1678231|8.207|440.204|3880|
|stat-G1-352-w32-OFF|1558976|2725746|44.901|440.444|6390|
|stat-G1-352-w32-ON|1590899|2658177|123.733|480.536|6502|
|stat-G1-352-w32-frozen|1564873|2725796|102.946|438.720|6391|
|stat-G2-200-w32-OFF|1882870|3557163|13.450|460.119|7697|
|stat-G2-200-w32-ON|1894283|3475744|38.674|487.003|7759|
|stat-G2-200-w32-frozen|1885693|3557254|8.720|455.461|7701|
|stat-G2-352-w32-OFF|3246466|6033192|135.759|468.674|13262|
|stat-G2-352-w32-ON|3267362|5889569|80.948|502.429|13328|
|stat-G2-352-w32-frozen|3248294|6033143|51.291|467.997|13263|


ON reduces instructions by about 2.3–2.5% against OFF in all four panels, while cycles increase by about 0.61–2.05%. These are single descriptive panels, not paired confidence intervals. The high-digit decision can add a dependency before the carry pass, but the measurements do not isolate that as the cause. Neither fewer instructions nor a shorter source expression establishes lower latency. Generic cache events do not identify a cache level or bandwidth bottleneck.

All twelve counter panels run 100%. The twelve profiles contain **93,990 samples**, zero lost records, at least 3,880 samples per profile and unknown share below 0.1%. Expected process, registered worker TID, FORK, mapping, allowed CPU and fixed-period evidence is checked. Flat attribution includes inlined work; it does not establish exclusive stalls, removable wall time or absolute period coverage.

|Linked function|Bytes|Instruction sites|IFMA sites|RSP operand lines|Literal stack subtraction sum|
|---|---:|---:|---:|---:|---:|
|frozen / G1-borrowed|26198|4425|800|574|4288|
|frozen / G2-borrowed|51830|8590|1450|1450|5952|
|OFF / G1-borrowed|26198|4425|800|574|4288|
|OFF / G2-borrowed|51830|8590|1450|1450|5952|
|ON / G1-owned|25803|4323|800|576|4096|
|ON / G1-borrowed|26739|4511|800|596|4160|
|ON / G2-borrowed|53040|8782|1450|1527|6016|


Static site and RSP-operand counts are not executed instruction counts, measured spills or complete stack bounds. The ON binary also retains an unsigned G1 specialization that is dormant in this borrowed-caller benchmark. All seven actual selected disassemblies are hash-bound; four initial original/OFF disassemblies remain separately preserved.

All **660 capture-workflow commands and 600 caller processes** succeed once. Independent review checks all 413 pins, source recovery, raw command/exit/stream hashes, all latency/CPU/counter-memory statistics and gates, exact scratch equality, every profile symbol bin and sample identity. Review reruns no native test, caller or verifier. No SDK request, proof export or Gnark verification is added; the canonical ledger remains **28,349** from the preceding full-SDK experiment.

Two helper failures occurred before capture. A review-generator raw-string delimiter was repaired; the initial source and available failure evidence are retained, with the missing full initial compiler log stated explicitly. A code observer's two-symbol assertion was corrected to retain the actual two/two/three specialization sets. Native arithmetic source, its successful qualification and the measured executables did not change. [Repair details](artifacts/helper-repair.md).

The following accepted source-176 proof comparisons are inherited. They are not measurements of this caller trial. M5 CPU/Metal numbers are the supplied historical references. The accepted adoption retains its documented physical16 calibration engineering exception and false original full-performance gate.

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


The all-eight CPU 10× objective remains active and unmet: **0/8**. Accepted primary is 7.928660 ms against a 3.7 ms target. Historical merge54 is 161.883899 ms without a supplied M5 reference. The experimental compact-SDK primary result was not adopted. AWS/Aeglos comparisons predate source 176 and are not refreshed; no AWS resource was created. No new mobile/ARM/TEE, huge250/4999, pressure/swap, concurrent-throughput or minimum-RAM result follows.

[Protocol](artifacts/design.md), [source audit](artifacts/source-audit.json), [full results](artifacts/analysis.json), [independent review](artifacts/review.json), [profile symbols](artifacts/profile-symbols.json), [linked code](artifacts/codegen.json), [workflow plan](artifacts/plan.json), [input pins](artifacts/pins.json), [publication review](artifacts/publication.json). Archives: [both source snapshots](artifacts/source.tar.gz), [Rust workflows and protocol](artifacts/workflow.tar.gz), [command receipts](artifacts/receipts.tar.gz), [counter panels](artifacts/counters.tar.gz), [disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS). Executables, libraries and raw perf.data stay local with hashes. Root: `/home/ubuntu/src/zopro-perf-work/high-borrow176`.
