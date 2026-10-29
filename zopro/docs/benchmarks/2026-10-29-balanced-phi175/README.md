# Balanced G2 phi windows: SDK pilot

**The latency gain gate fails; no adoption.** Primary ON latency is 8.002155 ms versus 8.032553 ms for fresh OFF and 7.998378 ms for accepted. Paired savings are +13.336 µs against OFF and +7.153 µs against accepted, with 9/15 wins each; both intervals cross parity. OFF/accepted calibration passes. CPU service falls by about 6% despite nearly unchanged single-proof latency. This positive work reduction is retained, without replacing the declared latency gate or claiming a throughput improvement.

## Change

The current eligible G2 query splits each scalar into two signed components with magnitudes below 2^126. The candidate replaces ten full 12-bit windows and one narrow indexed top with eleven balanced signed spans covering 127 bits: five spans of width 11 and six of width 12. The Booth reader uses the preceding bit as carry and keeps component signs separate. Variable-width Horner recombination reconstructs the same scalar. Every span uses the existing borrowed pending/collision/exception state, high-component bucket endomorphism, two point partitions, bucket merge and weighted reduction.

The earlier uniform 11-bit/four-partition G2 trial remains rejected; it changed partition count and window count. The failed ordinary G1 balanced-window pilot remains closed. This candidate changes G2 digit boundaries and top reduction, retaining the current decomposition and field kernels. No result from accepted balanced K/H is transferred. [History audit](artifacts/history-audit.md).

The new per-partition sum of bucket capacities is 17,408, versus 20,480 ordinary slots plus the old narrow top. Capacity is not a count of executed additions: collisions, inversions and replacing the indexed top can change costs. The existing forecast already charges eleven full 2,048-bucket parents per two partitions, the maximum 200-entry batch and extra top scratch. New heap geometry fits that envelope; eleven spans add 176 bytes of query metadata on this 64-bit host, with compiler temporaries additional. There is no key table, point copy, new scalar/digit buffer, pool, unsafe code, dependency or retained plan. This is not minimum-RAM or memory-pressure qualification.

Two cfg-only parent edits and one child make a 176-file private snapshot: 173 originals are byte-exact, and reversing the edits recovers all 175 accepted files. K/H, A, FFT, solver, scheduler, workspace forecast and all 17 field files remain exact. OFF is byte-identical to accepted SDK `20fde19f…`; ON is `d343f8e1…`. The linked audit checks actual cfg/features, immutable libraries, compiled test dispatch and ON-only balanced text contexts. Defining and generic symbol contexts remain distinguished from exclusive profile attribution. [Source audit](artifacts/source-audit.json), [linked audit](artifacts/code-audit.json).

## Complete verified requests

Ryzen 9950X, 16 physical / 32 logical CPUs, 32 workers pinned to CPUs 0–31. All nine shapes × three variants pass native preflight. Fifteen fixed rotated/reversed primary blocks each measure 151 complete SDK requests after three primes. Loading is excluded; solving, fresh randomized proving, SDK verification and serialization are included. Displayed medians need not reproduce paired estimates.

|Variant|Median ms|CPU ms/request|Median/max RSS MiB|M5 CPU speedup|M5 Metal speedup|
|---|---:|---:|---:|---:|---:|
|accepted|7.998378|181.715|130.50/134.00|4.626×|3.001×|
|OFF|8.032553|181.843|131.50/134.25|4.606×|2.988×|
|ON|8.002155|170.318|129.75/133.50|4.624×|2.999×|

RSS is process high-water including loading/primes, not required RAM. The modest observed RSS differences do not establish low-memory behavior.

|Latency comparison|Paired ratio|Interval|Wins / 15|Paired saving µs|
|---|---:|---:|---:|---:|
|1x2-w32/OFF/accepted|1.001247734|0.997938583–1.005924578|6|-10.010|
|1x2-w32/ON/OFF|0.998339756|0.995168143–1.003111882|9|+13.336|
|1x2-w32/ON/accepted|0.999106686|0.995081228–1.007546354|9|+7.153|

The predeclared gate requires ON/control upper ratios below one with at least 10/15 wins against both controls, and OFF/accepted intervals within [1/1.03,1.03]. Sorted ranks 4–12 of 15 have nominal 96.484375% individual coverage conditional on independent exchangeable blocks, without a familywise guarantee. Both gain checks fail. There is no minimum positive-gain floor, exclusion, extension, successful rerun, width/partition sweep or subset promotion. [Fixed design](artifacts/design.md), [all observations](artifacts/analysis.json).

## CPU work and diagnostics

The following paired CPU comparisons were computed after capture as descriptive evidence. They are not additional selection gates and do not rescue the failed latency result. CPU time sums process user and system service per measured request.

|CPU comparison|Paired ratio|Rank interval|Wins / 15|Paired CPU saving ms|
|---|---:|---:|---:|---:|
|OFF/accepted|0.999015|0.997395–1.002996|8|+0.180|
|ON/OFF|0.937487|0.934631–0.938703|15|+11.385|
|ON/accepted|0.935329|0.932596–0.937844|15|+11.788|

The lower CPU service is consistent with less aggregate work. It does not isolate a field, bucket, scheduling or cache cause. The prior accepted-source service trace placed both G2 and K/H on the finishing path, but this candidate has no fresh per-query critical-path trace; unchanged wall time alone cannot prove which query limits it. No throughput, energy or AWS scaling benefit is measured. [Descriptive CPU observations](artifacts/descriptive-CPU-pairs.json).

Three FIFO-gated four-event stat runs measure 31 requests each, and three separate 199 Hz user-cycle profiles measure 251 requests each, after three excluded primes. All 45 latency captures precede their Gnark verifications. All six diagnostics precede their verification/decoding; loading/primes are outside their gates. Every stat event runs 100%; profiles have one process each and zero lost records. No owned build/helper/hash/archive/other workload overlaps capture; the shared host is not claimed exclusive.

|Variant|Cycles/request, million|Instructions/request, million|Cache misses/request, million|Branch misses/request, million|
|---|---:|---:|---:|---:|
|OFF|829.166|1418.549|2.790|1.183|
|ON|781.515|1356.075|2.477|1.155|
|accepted|824.904|1418.261|2.745|1.185|

|Profile|Samples|Defining borrowed G2 apply sampled period|
|---|---:|---:|
|cycles-1x2-w32-OFF|9098|17.54%|
|cycles-1x2-w32-ON|8516|22.44%|
|cycles-1x2-w32-accepted|9192|18.85%|

Raw/demangled symbols and original decoded streams are retained. Defining-path shares exclude generic-context name matches and do not include every inlined/callee operation. Single stat/profile runs are descriptive, not exclusive costs, causal stalls or removable wall time. Raw perf files remain locally with hashes.

## Correctness and archive

Both fresh native builds pass all 127 core and 8 prover tests on their first executions. Three new core tests check 196,988 independent digit checks and 17,908 signed-component reconstructions; 48 known-generator query cases across 1/4 workers, 1/2 partitions and lengths 0/1/17/1,024/4,097/30,089; and compiled OFF=0/ON=1 dispatch. Fixtures include full-width and adversarial scalars, identity, repeated and opposite points. Unchanged field files inherit 33 field-test results; no field suite is rerun.

All 7,902 complete SDK requests verify internally. All 78 fresh 164-byte exports independently pass pinned Gnark, exact image/public binding and uniqueness against 18,592 prior proof identities. Ledger 18,670; 156 proof/public members archived. Ten native and 165 capture steps complete; the separate linked audit is a directly observed exit-zero invocation with `audit.log`, not a fabricated xtask receipt. There are no failed builds/tests/captures or successful repeats in this experiment. The archive checks 175 receipts / 700 original streams, source recovery, proof/public hashes, compression round trips and closing pins. [Closeout](artifacts/closeout.json).

## Inherited all-eight CPU and Metal comparison

These accepted-source measurements use supplied historical M5 references. Only the primary row above is newly timed in this pilot; native all-nine preflights do not qualify other-shape latency.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted primary remains 8.022165 ms / 134 MiB maximum RSS; 54→1 remains 162.250464 ms without a supplied M5 reference. The accepted full performance gate remains false with its recorded physical16 calibration engineering exception; final artifact gate remains true. No candidate physical16, portable/mobile/TEE, pressure/swap/minimum-RAM, concurrent-proof, 250/4999, AWS or Aeglos qualification is claimed. Production source175/SDK20fde is unchanged.

The exact balanced-phi layout is closed by this failed latency pilot. Its positive CPU-work and small uncertain latency estimates remain evidence. Any subsequent complementary algorithm needs its own declared controls and complete-proof test; no adoption or unchanged rerun follows here. The all-eight CPU 10× goal remains active and unmet (0/8).
