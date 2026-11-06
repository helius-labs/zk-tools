# Window batch contract: full-proof results

**The combined candidate is not adopted.** The full-proof gate fails14/39 checks. The original eight32-worker shapes have a scoped0.29%aggregate improvement against the accepted SDK, while primary32 remains uncertain and merge54 regresses. Positive estimates are retained. Production and the all-eight CPU10× goal remain unchanged.

## Comparison and decision

This is the first full-proof endpoint selected by the [disclosed component stop-rule exception](../2026-11-06-window-batch-capture176/README.md). It uses the same combined G1/G2 producer-bounds candidate and exact [qualified SDKs](../2026-11-06-window-batch-sdk176/README.md), with no algorithm, width, layout, arithmetic, queue, key, table, plan, pool or acceptance change. Accepted SDK6d83 and fresh OFF are the controls. The failed component gate remains failed; no further exception, G1-only variant or favorable worker subset is selected.

The independent timing/proof review passes. One calibration, nine nonregression and four gain checks fail. All nine32-worker contexts and primarysolo calibrate; primary16 does not. Thus the original-eight aggregate is a calibrated scoped positive, not a globally calibrated all-context result. Original-eight improvements do not replace the additional regression contexts or primary gain requirement.

| Declared scope/control | ON/control ratio [marginal interval], wins |
|---|---|
| ORIGINAL8/ON/OFF | 0.995878639 [0.992846317, 0.998626002], 12/15 |
| ORIGINAL8/ON/accepted | 0.997100447 [0.994261639, 0.998583196], 12/15 |
| PRIMARY32/ON/OFF | 0.993842471 [0.975560513, 1.000169632], 11/15 |
| PRIMARY32/ON/accepted | 0.985587316 [0.980291672, 1.003649891], 11/15 |
| ALL11/ON/OFF | 0.995637298 [0.986208189, 1.008287210], 9/15 |
| ALL11/ON/accepted | 1.001113208 [0.994272589, 1.008447674], 7/15 |

Primary32's displayed medians are7.850794ms ON versus7.915095ms accepted, about64microseconds lower. Its paired gain estimates are0.62%againstOFF and1.44%againstaccepted, with11/15wins each; both intervals include parity. These are different summaries and are not interchangeable. Primarysolo improves1.60%againstaccepted with15/15wins and a parity-excluding interval. Merge54 loses14/15blocks againstaccepted:ratio1.008359021[1.002765335,1.016217987]. Its ON/OFF interval includes parity, so this does not prove the bounds-contract change itself caused that loss. Primary16 cannot support a calibrated algorithmic latency conclusion.

## All context timings

Each displayed latency is the median of15 process medians, each from31 verified requests after3checked primes. Ratios use paired blocks and need not equal ratios of displayed medians. Times are milliseconds.

| Context | Accepted | OFF | ON | ON/OFF [interval], wins | ON/accepted [interval], wins | OFF/accepted [interval], wins |
|---|---:|---:|---:|---|---|---|
| 1x2-w32 | 7.915095 | 7.899435 | 7.850794 | 0.993842471 [0.975560513, 1.000169632], 11/15 | 0.985587316 [0.980291672, 1.003649891], 11/15 | 0.998470882 [0.989533275, 1.011911498], 8/15 |
| 2x2-w32 | 11.169812 | 11.214486 | 11.072078 | 0.987961855 [0.978760666, 0.993275216], 12/15 | 0.993994446 [0.987367435, 0.998141817], 12/15 | 1.002449273 [1.000705909, 1.008647816], 3/15 |
| 4x2-w32 | 16.085882 | 16.008355 | 15.948292 | 0.993018862 [0.988723326, 1.002648568], 10/15 | 0.991646816 [0.981779203, 1.000798848], 11/15 | 0.998762849 [0.983449942, 1.008355237], 8/15 |
| 8x2-w32 | 24.517520 | 24.253332 | 24.186476 | 0.990804373 [0.978662912, 1.016880650], 9/15 | 0.985948131 [0.969070467, 1.012155962], 10/15 | 0.994303742 [0.977270893, 0.999451716], 12/15 |
| 12x2-w32 | 33.838626 | 33.848014 | 33.885744 | 0.998902467 [0.987224532, 1.006045111], 8/15 | 1.001636511 [0.996149375, 1.010180469], 4/15 | 1.001423775 [0.995650887, 1.012089737], 7/15 |
| 16x2-w32 | 40.083672 | 39.859419 | 39.929871 | 0.999636245 [0.996396221, 1.008222363], 8/15 | 0.996893501 [0.992426787, 1.004152551], 9/15 | 0.995971577 [0.993553051, 1.003359021], 10/15 |
| 24x2-w32 | 65.506198 | 65.761289 | 66.042299 | 1.000908316 [0.989598098, 1.011787140], 7/15 | 1.005715781 [0.998554943, 1.015966932], 5/15 | 1.003762326 [0.997723523, 1.019814362], 5/15 |
| merge-24x1-w32 | 65.200982 | 65.178700 | 65.082840 | 0.997375728 [0.993524442, 1.000064161], 11/15 | 1.001587463 [0.991859293, 1.005858763], 6/15 | 1.003745842 [0.992898429, 1.007653969], 6/15 |
| merge-54x1-w32 | 161.718956 | 161.911470 | 162.995143 | 1.006692997 [0.998360450, 1.015433549], 5/15 | 1.008359021 [1.002765335, 1.016217987], 1/15 | 1.005683736 [0.995163680, 1.013581938], 7/15 |
| 1x2-w1 | 93.923972 | 94.024092 | 92.409056 | 0.982747946 [0.980415675, 0.989691495], 15/15 | 0.983955323 [0.981237414, 0.989976942], 15/15 | 0.999994327 [0.997451714, 1.002823986], 8/15 |
| 1x2-w16 | 9.563112 | 9.549326 | 10.030263 | 0.992597100 [0.908551078, 1.130350855], 9/15 | 1.031960215 [0.970977033, 1.112373695], 7/15 | 1.002891348 [0.978218689, 1.074086619], 7/15 |

All failed checks:

- `12x2-w32/ON/accepted/nonregression`
- `1x2-w16/OFF/accepted/calibration`
- `1x2-w16/ON/OFF/nonregression`
- `1x2-w16/ON/accepted/nonregression`
- `24x2-w32/ON/OFF/nonregression`
- `24x2-w32/ON/accepted/nonregression`
- `8x2-w32/ON/OFF/nonregression`
- `8x2-w32/ON/accepted/nonregression`
- `ALL11/ON/OFF/gain`
- `ALL11/ON/accepted/gain`
- `PRIMARY32/ON/OFF/gain`
- `PRIMARY32/ON/accepted/gain`
- `merge-54x1-w32/ON/OFF/nonregression`
- `merge-54x1-w32/ON/accepted/nonregression`

## Memory and runtime observations

The source adds no retained allocation, point/key copy, table, plan or pool. These are maximum observed whole-process RSS values across the15latency processes, including load/prime history; they are not minimum RAM, exact allocation-lifetime peaks or pressure tests. Differences do not establish a memory optimization.

| Context | Accepted RSS MiB | OFF RSS MiB | ON RSS MiB | Accepted CPU ms/request | OFF CPU ms/request | ON CPU ms/request |
|---|---:|---:|---:|---:|---:|---:|
| 1x2-w32 | 112.500 | 110.750 | 111.500 | 179.272 | 178.595 | 176.924 |
| 2x2-w32 | 266.250 | 273.000 | 272.750 | 259.199 | 258.835 | 256.722 |
| 4x2-w32 | 309.250 | 310.000 | 308.500 | 421.336 | 418.538 | 418.926 |
| 8x2-w32 | 507.750 | 501.000 | 517.750 | 674.466 | 672.106 | 669.003 |
| 12x2-w32 | 711.750 | 713.500 | 715.500 | 939.417 | 945.005 | 942.124 |
| 16x2-w32 | 848.250 | 826.000 | 842.750 | 1122.233 | 1122.557 | 1123.035 |
| 24x2-w32 | 1268.652 | 1336.500 | 1306.000 | 1879.048 | 1887.555 | 1891.200 |
| merge-24x1-w32 | 1163.000 | 1163.531 | 1189.234 | 1810.332 | 1810.946 | 1805.537 |
| merge-54x1-w32 | 1853.883 | 1899.016 | 1876.539 | 4422.166 | 4444.336 | 4457.858 |
| 1x2-w1 | 34.500 | 34.500 | 34.500 | 93.939 | 94.020 | 92.412 |
| 1x2-w16 | 81.000 | 80.500 | 82.500 | 115.309 | 114.736 | 112.690 |

Fault and context-switch summaries below are medians per timed request over the15 latency processes. Minor faults are page mappings/first touches, not measured storage I/O or fault service time. Counts alone do not explain elapsed differences.

| Context/arm | Minor faults/request | Voluntary switches/request | Involuntary switches/request |
|---|---:|---:|---:|
| 1x2-w32/accepted | 441.323 | 392.323 | 267.097 |
| 1x2-w32/OFF | 405.290 | 382.742 | 284.355 |
| 1x2-w32/ON | 417.613 | 395.871 | 219.065 |
| 2x2-w32/accepted | 1262.839 | 612.806 | 496.387 |
| 2x2-w32/OFF | 1261.290 | 625.710 | 548.065 |
| 2x2-w32/ON | 1323.871 | 629.806 | 461.097 |
| 4x2-w32/accepted | 1098.452 | 1036.452 | 596.194 |
| 4x2-w32/OFF | 1136.774 | 1057.226 | 616.194 |
| 4x2-w32/ON | 1099.419 | 1054.355 | 644.452 |
| 8x2-w32/accepted | 1639.806 | 1007.613 | 867.452 |
| 8x2-w32/OFF | 1736.613 | 1032.032 | 945.258 |
| 8x2-w32/ON | 1689.516 | 1018.097 | 965.452 |
| 12x2-w32/accepted | 2126.484 | 843.290 | 815.065 |
| 12x2-w32/OFF | 2256.710 | 882.806 | 924.161 |
| 12x2-w32/ON | 2314.484 | 873.710 | 951.161 |
| 16x2-w32/accepted | 2617.581 | 1007.323 | 1062.065 |
| 16x2-w32/OFF | 2525.194 | 1033.355 | 1004.226 |
| 16x2-w32/ON | 2392.677 | 1027.419 | 1045.226 |
| 24x2-w32/accepted | 6605.000 | 1115.774 | 1305.258 |
| 24x2-w32/OFF | 4836.290 | 1130.000 | 1329.065 |
| 24x2-w32/ON | 6243.097 | 1134.613 | 1241.935 |
| merge-24x1-w32/accepted | 3749.677 | 1185.677 | 1523.355 |
| merge-24x1-w32/OFF | 4000.581 | 1223.516 | 1563.484 |
| merge-24x1-w32/ON | 3330.484 | 1211.710 | 1585.871 |
| merge-54x1-w32/accepted | 21544.581 | 1784.194 | 1801.323 |
| merge-54x1-w32/OFF | 21323.806 | 1817.710 | 1916.871 |
| merge-54x1-w32/ON | 21582.516 | 1808.645 | 1815.387 |
| 1x2-w1/accepted | 0.000 | 3.677 | 4.065 |
| 1x2-w1/OFF | 0.000 | 3.677 | 4.032 |
| 1x2-w1/ON | 0.000 | 3.677 | 4.000 |
| 1x2-w16/accepted | 217.645 | 268.000 | 671.903 |
| 1x2-w16/OFF | 214.935 | 272.484 | 754.516 |
| 1x2-w16/ON | 226.871 | 269.613 | 997.516 |

## CPU counters and profiles

All12 PMU panels qualify with numeric counters and at least90% running coverage. These are separate single panels, not confidence intervals or matched latency replicates. Primary retired-instruction reductions coexist with small/uncertain elapsed gains; instruction count alone does not predict proof latency.

| PMU context | ON/OFF instructions | ON/OFF user cycles | ON/accepted instructions | ON/accepted user cycles |
|---|---:|---:|---:|---:|
| 1x2-w32 | 0.976081668 | 0.992709863 | 0.975989333 | 0.986929571 |
| merge-54x1-w32 | 1.000293189 | 1.015148868 | 0.996930594 | 1.009105494 |
| 1x2-w1 | 0.974296434 | 0.987196443 | 0.974293402 | 0.984448832 |
| 1x2-w16 | 0.976211643 | 1.004677335 | 0.977753225 | 1.024000860 |

All12profiles qualify:197,640samples,zeroLOST,maximum unknown share1.8598%. Fixed16Mcycles:u periods,512pages,warm FIFO gates and actual executableMMAP/PID/FORK-TID/allowedCPU/period are checked. Primarysolo usesCPU2. Threads may migrate inside the declared affinity; no kernel-only attribution gate is imposed. Perf resolves symbols; this is not independent ASLR decoding, absolute cycle coverage or exclusive removable wall time.

Accepted primary32 has67.00%MSM/curve samples,7.81%inverse and9.80%crossbeam-epoch category samples. Primary16's epoch category is2.68%,solo0%. Accepted merge54 instead has44.36%FFT and37.52%MSM/curve samples. These are descriptive categories with inlined ownership limitations, not proof that scheduler, memory or any individual function causes the differences. The current recorded traces can support a distinct source-level question without another profile campaign.

## Protocol, correctness and scope

The full-proof protocol predates SDK correctness execution. ALL8plusmerge54at32workers andprimary1/16workers are retained.15fixed rotating/reversing matched blocks,three immutable arms,31requests and3primes per latency process give495latency processes. All latency precedes12PMU and12profiles; all recording precedes519Gnark checks and60offline operations. The1098-command workflow,finish and independent reviewer each pass on their first execution. Existing22fresh preflight exports are reused as bound qualification, not executed or counted as new.

A gated launcher waits while the actual exec handle and checkpoint are saved, then starts collection only after launch-armed is written. No unrelated owned compute or checkpoint write overlaps timing/recording. No successful native build,test,link,preflight,capture or decoder is replayed.651source/input/library/binary/tool/protocol/receipt/reviewer bindings are frozen before launch.

All19,326new SDK requests,including primes,pass internal verification. The519exported final proof/public pairs independently pass pinned Gnark verification, exact image/public/VK binding and uniqueness against the30,868-export prior ledger and each other. The successor ledger has31,387exports. This is519independent verifications,not19,326. The independent reviewer reconstructs all39timing checks and checks every command stream and fresh proof binding. PMU/profile parsing has one implementation and is not independently reparsed.

39predeclared checks comprise11OFF/accepted calibration intervals within[.97,1.03],22ONvsboth nonregression upper bounds<=1.01,and6aggregate gain checks requiring high<1and at least11/15wins. Intervals use ranks3/7/11of15 and are marginal,without a familywise guarantee. No gain floor, removed observations, favorable subset, new block, parameter change or reroll is applied. Portable and pressure qualification remains unperformed for this rejected candidate. No AWS,Aeglos,phone,huge-circuit,swap,pressure or concurrent-proof-throughput result/resource is introduced.

## Supplied M5 comparisons

These are new matched local32-worker CPU results for the exact accepted SDK and experimental candidate, compared with the supplied M5 references. Ratios do not establish equal hardware efficiency. No candidate shape meets its CPU10× point target; the all-eight verified goal remains unmet0/8 and production remains unchanged.

| Shape | Accepted CPU ms | ON CPU ms | M5 CPU ms | M5 Metal ms | M5 CPU/ON | M5 Metal/ON | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.915095 | 7.850794 | 37 | 24 | 4.713× | 3.057× | 3.7 |
| 2x2 | 11.169812 | 11.072078 | 59 | 30 | 5.329× | 2.710× | 5.9 |
| 4x2 | 16.085882 | 15.948292 | 109 | 48 | 6.835× | 3.010× | 10.9 |
| 8x2 | 24.517520 | 24.186476 | 189 | 74 | 7.814× | 3.060× | 18.9 |
| 12x2 | 33.838626 | 33.885744 | 284 | 101 | 8.381× | 2.981× | 28.4 |
| 16x2 | 40.083672 | 39.929871 | 335 | 124 | 8.390× | 3.105× | 33.5 |
| 24x2 | 65.506198 | 66.042299 | 528 | 176 | 7.995× | 2.665× | 52.8 |
| merge-24x1 | 65.200982 | 65.082840 | 507 | 175 | 7.790× | 2.689× | 50.7 |

Merge54 is162.995143msON versus161.718956msaccepted,without a supplied M5 reference. Historical adopted local1×2=7.928660ms andmerge54=161.883899ms remain labeled historical; the accepted implementation is identical. Historical1×2RSS132.75MiB is not substituted for the new capture's RSS observations.

[Analysis/all ratios](artifacts/analysis.json),[independent review](artifacts/review.json),[prospective design](artifacts/design.md),[collection details](artifacts/collection.md),[decision](artifacts/decision.md),[identities](artifacts/identities.json),[profile symbols](artifacts/profile-symbols.json),[proof inventory](artifacts/proof-inventory.json),[archive index](artifacts/archive-index.json),[local raw/bulk hashes](artifacts/raw-and-bulk-index.json). The UTF-8 helper/receipt archive round-trips exact bytes and hashes. Native executables/libraries,key images,private witnesses and full ledger payloads are excluded. Raw perf and bulk decoded streams remain local with explicit hashes. Local root:`/home/ubuntu/src/zopro-perf-work/window-batch-proof-capture176`.
