# Radix grouping: full SDK comparison

**The full performance gate fails; this candidate is not adopted.** Primary ON latency is 8.012793 ms versus 8.045524 ms for fresh OFF and 8.025797 ms for the accepted executable. The paired savings estimates are +38.883 µs against OFF and +10.861 µs against accepted; both intervals cross parity and both win counts miss the fixed gain condition. The original-eight aggregate is 2.073% slower than OFF and 3.947% slower than accepted. Primary maximum observed RSS falls to 86 MiB from 135/134.5 MiB, respectively. These positive observations are retained without claiming a qualified speedup or minimum RAM.

## Implementation and controls

The proof-local builder replaces the owned full-scalar hash map with four stable byte-radix passes over packed 32-bit fingerprints and 32-bit original indices. Full 256-bit comparison resolves every fingerprint collision; no equality decision depends on the fingerprint. An all-equal run uses a linear scan, other runs use exact scalar/index sorting. Original input order determines class numbering and membership order. Sampling, 80/95% cutoffs, resource reservation and canonicalization remain unchanged. The original builder is retained when the checked workspace forecast cannot fit its existing reservation. There are no key lookup tables, point/key copies, added dependencies, new parallel work or persistent caches. [Source audit](artifacts/source-audit.json), [workspace review](artifacts/memory-review.md).

The explicit payload forecast covers both radix vectors, classes, returned metadata and scatter counters using the existing checked capacity helpers. It must fit the untouched old grouping-cost bound before allocation. This is an allocation-payload bound, not RSS or minimum RAM; compiler frames, allocator retention and other proof stages remain outside it. No low-memory or huge-circuit qualification is inferred.

Accepted SDK is `20fde19f…`, fresh OFF `e84d0701…`, ON `cd72bf0b…`. OFF is not byte-identical to accepted, although its grouping entry has the same 7,624-byte size; ON is 13,661 bytes. LLVM inlines the radix builder into that entry. ON-only standard-library sort specializations carry the radix comparison closure in their generic names; they are not standalone defining radix functions. Actual linked code, cfg/features and immutable library hashes are retained. Static code size is not speed evidence. [Corrected code audit](artifacts/code-audit-v2.json).

## Full verified latency matrix

Ryzen 9950X, 16 physical / 32 logical CPUs; 32 workers pinned to CPUs 0–31. All nine shapes have accepted/OFF/ON preflight and 15 fixed rotated/reversed blocks. Each process primes three times; primary blocks measure 151 requests and other shapes 31. Loading is excluded from latency; witness solving, randomized proving, SDK verification and serialization are included. Displayed cell medians need not reproduce paired estimates.

|Shape|Variant|Median ms|CPU ms/request|Median/max RSS MiB|
|---|---|---:|---:|---:|
|1x2|accepted|8.025797|182.195|130.75/134.50|
|1x2|OFF|8.045524|181.954|130.25/135.00|
|1x2|ON|8.012793|182.469|82.50/86.00|
|2x2|accepted|11.359725|262.165|264.00/273.25|
|2x2|OFF|11.328125|263.015|254.25/271.25|
|2x2|ON|11.249827|260.058|217.50/226.25|
|4x2|accepted|15.823534|416.050|305.00/316.00|
|4x2|OFF|16.398138|427.064|302.50/321.50|
|4x2|ON|15.937769|418.954|276.00/289.50|
|8x2|accepted|23.989775|666.604|491.50/507.00|
|8x2|OFF|24.973080|691.962|492.50/515.25|
|8x2|ON|25.269919|690.280|472.00/499.75|
|12x2|accepted|33.732439|931.879|689.50/705.50|
|12x2|OFF|34.769055|960.259|693.75/720.75|
|12x2|ON|35.404343|970.344|639.00/674.50|
|16x2|accepted|40.057277|1124.715|793.75/834.00|
|16x2|OFF|40.817292|1140.558|789.25/849.00|
|16x2|ON|42.759626|1174.766|783.25/815.25|
|24x2|accepted|66.552279|1899.325|1197.04/1291.50|
|24x2|OFF|66.993211|1912.855|1207.54/1283.31|
|24x2|ON|71.365649|1987.388|1196.02/1244.72|
|merge-24x1|accepted|65.123601|1801.237|1143.32/1198.88|
|merge-24x1|OFF|65.799019|1826.970|1137.59/1188.03|
|merge-24x1|ON|69.694215|1902.888|1104.39/1126.52|
|merge-54x1|accepted|161.973465|4436.256|1788.81/1903.81|
|merge-54x1|OFF|163.187834|4482.120|1742.90/1824.09|
|merge-54x1|ON|180.465275|4802.858|1751.61/1875.40|

RSS is process high-water memory including loading and primes. Lower primary RSS does not establish the required RAM, pressure behavior or a causal explanation for latency.

|Shape|Comparison|Paired ratio|Interval|Wins / 15|Paired saving µs|
|---|---|---:|---:|---:|---:|
|1x2|OFF/accepted|1.003275|1.001247–1.006238|3|-26.219|
|1x2|ON/OFF|0.995188|0.992856–1.001770|9|+38.883|
|1x2|ON/accepted|0.998644|0.993179–1.007422|8|+10.861|
|2x2|OFF/accepted|0.995356|0.990953–1.011524|8|+52.749|
|2x2|ON/OFF|0.991742|0.981998–1.000326|11|+93.677|
|2x2|ON/accepted|0.989368|0.978747–1.001082|11|+120.848|
|4x2|OFF/accepted|1.036550|1.012365–1.046650|1|-578.350|
|4x2|ON/OFF|0.974788|0.966569–0.987229|14|+415.073|
|4x2|ON/accepted|1.002766|0.993387–1.011763|7|-43.963|
|8x2|OFF/accepted|1.039289|1.020457–1.055956|0|-945.423|
|8x2|ON/OFF|1.025089|0.998591–1.027167|4|-618.687|
|8x2|ON/accepted|1.053487|1.046498–1.067529|0|-1293.659|
|12x2|OFF/accepted|1.025540|1.018293–1.035491|1|-859.872|
|12x2|ON/OFF|1.028081|1.014276–1.034049|0|-969.109|
|12x2|ON/accepted|1.052965|1.045693–1.056540|0|-1813.851|
|16x2|OFF/accepted|1.020647|1.009639–1.025568|1|-829.083|
|16x2|ON/OFF|1.052926|1.041953–1.058645|0|-2143.813|
|16x2|ON/accepted|1.073225|1.061439–1.082267|0|-2916.831|
|24x2|OFF/accepted|1.009887|1.001659–1.019134|3|-665.345|
|24x2|ON/OFF|1.059757|1.055685–1.074186|0|-4036.052|
|24x2|ON/accepted|1.073036|1.062553–1.081234|0|-4871.376|
|merge-24x1|OFF/accepted|1.012555|1.000909–1.021001|2|-812.993|
|merge-24x1|ON/OFF|1.059982|1.046573–1.068992|0|-3945.871|
|merge-24x1|ON/accepted|1.066686|1.061174–1.072801|0|-4425.476|
|merge-54x1|OFF/accepted|1.012457|1.005334–1.017669|2|-2022.885|
|merge-54x1|ON/OFF|1.102318|1.098576–1.109129|0|-16800.731|
|merge-54x1|ON/accepted|1.117475|1.108575–1.123230|0|-19027.847|

|Original-eight equal-log aggregate|Ratio|Interval|Wins / 15|
|---|---:|---:|---:|
|original8/ON/OFF|1.020733|1.019059–1.026054|0|
|original8/ON/accepted|1.039474|1.037095–1.041028|0|

Ranks 4–12 of 15 have nominal 96.484375% individual coverage under independent exchangeable blocks, without a familywise guarantee. The fixed gate requires OFF/accepted intervals inside [1/1.03, 1.03], ON upper nonregression ratios ≤1.03 against both controls for every shape, and primary plus original-eight aggregate upper ratios below one with at least 10/15 wins against both controls. OFF/accepted calibration fails on 4×2, 8×2 and 12×2. Both primary gain conditions, both aggregate gain conditions and several large-shape nonregression conditions fail. All 18 failed checks remain in the analysis. No threshold, exclusion, extension, successful rerun, fingerprint-width sweep or shape-subset promotion follows these results. [Fixed design](artifacts/design.md), [all observations and checks](artifacts/analysis.json).

## Correctness and audit history

Both fresh OFF and ON pass all 128 core and 8 prover tests. The four new core tests check stable radix ordering, exact grouping metadata against an independent ordered-map oracle across 256 cases including forced total fingerprint collisions, 522,288 checked workspace cases, and compiled OFF/ON dispatch. Existing G1/G2 weighted-generator and grouping cases remain passing. All 17 field files are byte-exact; 33 field-test results are inherited, not rerun. The private snapshot contains 176 files, with 174 unchanged originals; reversing one edit recovers all 175 production files.

All 20,142 complete SDK requests verify internally. All 438 fresh 164-byte exports pass independent Gnark verification, exact image/public-input binding and uniqueness against the 18,154 prior proof identities. The proof ledger is 18,592, with 876 proof/public members archived.

The native workflow completed its ten build/test/freeze/link steps, then failed its original code audit (exit 101). That audit incorrectly required a standalone defining radix child symbol. The corrected audit checks all 366 original build pins, immutable library selection, test/dispatch receipts and actual grouping code plus ON-only collision-sort specializations. It was executed directly and observed exit 0 before the first measurement; `audit.log` is retained, and this is not represented as an xtask receipt. Initial sources, the original failure and the corresponding pre-capture controller repair are archived. No implementation, measurement gate or successful test/build was changed or repeated. All 885 capture-workflow steps complete. [Audit review](artifacts/audit-review.json), [original failure](artifacts/native-failure.json).

## CPU diagnostics

All latency captures precede their 405 Gnark verifications. Three FIFO-gated four-event stat runs then measure 31 requests each, and three FIFO-gated 199 Hz user-cycle profiles measure 251 requests each. All diagnostic captures precede their verification and decoding; loading and three primes are outside each FIFO gate. All events run at least 99.9%; each profile has one sampled process and zero lost records. No owned build/helper/hash/archive/other CPU job overlaps captures; the shared host is not claimed exclusive.

|Variant|Cycles/request, million|Instructions/request, million|Cache misses/request, million|Branch misses/request, million|
|---|---:|---:|---:|---:|
|OFF|812.221|1420.973|2.813|1.191|
|ON|825.669|1425.321|2.973|1.235|
|accepted|836.825|1421.862|2.983|1.196|

|Profile|Samples|Defining grouping-entry sampled period|
|---|---:|---:|
|cycles-1x2-w32-OFF|9142|0.68%|
|cycles-1x2-w32-ON|9033|0.70%|
|cycles-1x2-w32-accepted|8953|0.62%|

Defining-entry attribution uses the demangled source-path prefix, not a radix name embedded inside a standard-library generic context. Raw/demangled symbols and original decoded streams remain available. These are sampled IP periods and single diagnostic runs, not causal stall attribution, exclusive grouping cost or removable wall time. Raw perf data remain local with hashes.

## Supplied CPU and Metal comparisons

The following accepted/OFF/ON speedups use the fresh full matrix above and the supplied M5 timings. They are comparisons across machines; no new M5 run is claimed. All eight ON CPU 10× conditions remain false.

|Shape|M5 CPU ms|Accepted/OFF/ON CPU speedup|M5 Metal ms|Accepted/OFF/ON Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|37|4.610× / 4.599× / 4.618×|24|2.990× / 2.983× / 2.995×|3.7|
|2x2|59|5.194× / 5.208× / 5.245×|30|2.641× / 2.648× / 2.667×|5.9|
|4x2|109|6.888× / 6.647× / 6.839×|48|3.033× / 2.927× / 3.012×|10.9|
|8x2|189|7.878× / 7.568× / 7.479×|74|3.085× / 2.963× / 2.928×|18.9|
|12x2|284|8.419× / 8.168× / 8.022×|101|2.994× / 2.905× / 2.853×|28.4|
|16x2|335|8.363× / 8.207× / 7.834×|124|3.096× / 3.038× / 2.900×|33.5|
|24x2|528|7.934× / 7.881× / 7.399×|176|2.645× / 2.627× / 2.466×|52.8|
|merge-24x1|507|7.785× / 7.705× / 7.275×|175|2.687× / 2.660× / 2.511×|50.7|

54→1 has no supplied M5 reference. The accepted historical production primary remains 8.022165 ms / 134 MiB maximum RSS; production source 175 and SDK `20fde19f…` are unchanged. Its full performance gate remains false with the recorded physical16 calibration engineering exception, and its final artifact gate remains true. No candidate portable/mobile/TEE, physical16, memory-pressure, concurrency, 250/4999, minimum-RAM, AWS or Aeglos qualification follows this matrix. The archive retains 896 receipts (including the failed original audit), 3,584 original streams, all private sources and proof/public members, with checked compression round trips and closing pins. [Closeout](artifacts/closeout.json).

This exact universal radix builder is closed by its failed full matrix. The lower primary RSS and small positive primary estimates remain evidence; they do not justify adoption or rescue a subset of the fixed experiment. The all-eight CPU 10× goal remains active and unmet (0/8).
