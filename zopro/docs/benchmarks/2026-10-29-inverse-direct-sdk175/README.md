# Direct inverse finish: full SDK pilot

**The pilot gain gate fails; no demonstrated proof-latency improvement and no adoption.** Candidate primary latency is 8.028322 ms versus 7.983257 ms for fresh OFF and 8.001953 ms for the accepted executable. The paired ON/OFF estimate is 0.114% slower, with ratio interval 0.999476–1.007180 and 5/15 wins. Against accepted it is 0.342% slower, interval 0.998736–1.006481 and 5/15 wins. Both intervals include small improvements and regressions; this is not proof of a definitive slowdown. The fresh/accepted calibration passes.

## Change and control identities

The exact prior direct-finish routine reduces the signed Euclidean linear form directly modulo the field prime. It removes the old radix64 cancellation and scale-restoring field multiplication after the unchanged one-word Euclidean loop. Supported-modulus guards, GCD matrices, representations, solver, MSM, scheduler and scratch remain unchanged. It adds no heap allocation, key table or point copy. [Bounds](artifacts/bounds.md), [source audit](artifacts/source-audit.json).

The earlier primitive comparison accidentally linked ON twice and remains unqualified. This is its first full SDK integration; that primitive timing schedule is not repeated. Separate build directories and immutable hashed libraries are audited for actual cfg/features. OFF and accepted SDKs are byte-identical (`20fde19f…`); ON is `899a0cb6…`. OFF and ON have different field/core/prover library hashes and SDK hashes. Actual linked inverse bodies shrink from 7,723 to 7,013 bytes for each Fq copy and 7,786 to 7,078 for Fr. These static sizes establish a code change, not faster execution. [Code audit](artifacts/code-audit.json).

## Complete verified requests

Ryzen 9950X, 16 physical / 32 logical CPUs, 32 workers pinned to CPUs 0–31. All nine shapes pass accepted/OFF/ON preflight, followed by 15 fixed rotated/reversed primary blocks, each with three primes and 151 measured requests. Loading is excluded; fresh witness solving, randomized proving, SDK verification and serialization are included. Displayed medians need not reproduce paired-ratio estimates.

|Variant|Median ms|CPU ms/request|Median/max RSS MiB|M5 CPU / measured|M5 Metal / measured|
|---|---:|---:|---:|---:|---:|
|accepted|8.001953|181.401|130.25/133.75|4.624×|2.999×|
|OFF|7.983257|180.902|130.25/134.25|4.635×|3.006×|
|ON|8.028322|180.941|130.75/135.75|4.609×|2.989×|

Process high-water RSS includes loading/primes; it is not minimum RAM or an allocation bound. No memory-saving claim follows from the smaller inverse functions.

Both ON/control upper paired ratios must be below one with at least 10/15 wins. The OFF/accepted interval must lie within [1/1.03, 1.03]. Sorted ranks 4–12 of 15 have nominal 96.484375% individual coverage under independent exchangeable blocks, without a familywise guarantee. There is no minimum positive-gain floor, exclusion, extension, successful rerun, flag/layout sweep or partial promotion. Both gain conditions fail. [Design](artifacts/design.md), [all observations and checks](artifacts/analysis.json).

Both coherent native builds pass all 34 field, 124 core and 8 prover tests. The 68,672 signed-residue field cases are inherited inputs freshly executed in both builds, not newly distinct cases. Original inverse source and the candidate helper match their prior inputs byte-for-byte. The 176-file private snapshot has 174 exact originals; reversing the one original edit recovers all 175 production files.

All 7,902 complete SDK requests verify internally. All 78 fresh 164-byte exports independently pass Gnark, exact image/public-input binding and uniqueness against 18,076 prior proof identities. The ledger is now 18,154; 156 proof/public members are archived. No successful test, capture or independent verification is repeated.

## CPU diagnostics

After all latency captures and their verification, three separately gated four-event stat runs measure 31 requests each, followed by three separately gated 199 Hz user-cycle profiles of 251 requests each. Every process primes three times; FIFO gates exclude loading/primes. All six captures precede diagnostic verification/decoding. All stat events run at least 99.9%; every profile has one sampled process and zero lost records. No owned build/helper/hash/archive/other CPU job overlaps captures. The host is shared; exclusive access is not claimed.

|Variant|Cycles/request, million|Instructions/request, million|Cache misses/request, million|Branch misses/request, million|
|---|---:|---:|---:|---:|
|OFF|829.489|1420.620|2.971|1.199|
|ON|828.489|1419.985|2.839|1.214|
|accepted|827.592|1420.120|2.987|1.194|

|Profile|Samples|Defining inverse-symbol period|
|---|---:|---:|
|cycles-1x2-w32-OFF|8968|7.00%|
|cycles-1x2-w32-ON|8922|8.00%|
|cycles-1x2-w32-accepted|8964|8.02%|

Raw/demangled symbol weights and original decoded streams are retained. These are sampled IP periods and one diagnostic run per variant; they are not causal stall attribution, exclusive inversion cost, removable wall time or a latency selection criterion. Raw perf files remain locally with hashes.

An initial reverse-source check consumed one original newline; the pattern was fixed before native builds. A controller-generator list anchor was corrected during static review before preparing or executing captures. Initial sources and notes are retained. Neither repair changed arithmetic, timing thresholds or successful observations. All 13 native and 165 capture-workflow steps finish. The archive checks 178 receipts and 712 original streams byte-for-byte, source recovery, proof/public hashes, compressed round trips and closing pins. [Closeout](artifacts/closeout.json).

## Inherited accepted all-eight CPU and Metal comparison

These historical accepted-source measurements use supplied M5 references; they are not fresh candidate all-eight latency measurements. Only the primary comparison above is newly timed. The accepted 1×2 peak RSS remains 134 MiB. Its full performance gate remains false with the recorded physical16 calibration engineering exception; the final artifact gate remains true.

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

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. No candidate portable/mobile/TEE, physical16, memory-pressure, concurrency, 250/4999, minimum-RAM, AWS or Aeglos qualification follows. Production source 175 / SDK20fde remains unchanged. The exact direct-finish SDK integration is closed by this failed pilot; it is not selected for all-eight qualification. The all-eight CPU 10× goal remains active and unmet (0/8).
