# Split REDC planes: complete-caller comparison

**Not adopted: the required 32-worker gain and nonregression gate fails.** The all-context aggregate improves by about 2.1% against fresh OFF, and every 1/16-worker context improves against both controls. Those gains are retained. They do not overcome the declared 32-worker failures or establish a full-proof improvement. Production remains unchanged.

## Decision

The fixed gate fails **21 of 76 checks**: four control calibrations, 15 nonregression checks and both LOGICAL32 gain checks. All four calibration failures are at 32 workers: G1-200, G1-352, G2-200 and G2-201. No observation, context, interval or threshold is replaced. This is not a globally calibrated campaign.

The larger G2 32-worker controls at 352 and 353 points do calibrate. ON loses all 15 paired blocks against both controls at each size; ON/OFF medians are 4.64% and 3.69% slower respectively. These losses cannot be dismissed as the unrelated control failures elsewhere. The implementation is closed without a topology-only selection, SDK trial, source adoption or unchanged retry.

| Declared aggregate | ON/control ratio [marginal interval], wins |
|---|---|
| ALL/OFF | 0.978911975 [0.972984123, 0.983126603], 14/15 |
| ALL/frozen | 0.986652275 [0.981335878, 0.989753695], 15/15 |
| LOGICAL32/OFF | 1.011700629 [0.990632907, 1.025630377], 5/15 |
| LOGICAL32/frozen | 1.036956242 [1.018759255, 1.046190991], 2/15 |

The ALL gain checks pass. Every individual 1/16-worker context also has its ON/OFF and ON/frozen interval below parity; these are real component improvements. The primary LOGICAL32 aggregate includes both curves and all four sizes, as declared for a global field change. No smaller successful subset becomes the adoption scope.

The following worker aggregates summarize the same complete data descriptively; they add no new measurements or decision gates. Each includes all eight curve/size contexts.

| Worker summary | ON/control ratio [marginal interval], wins |
|---|---|
| w1/ON/OFF | 0.959174127 [0.957886090, 0.960941571], 15/15 |
| w1/ON/frozen | 0.958556946 [0.957490589, 0.960997070], 15/15 |
| w16/ON/OFF | 0.967635244 [0.967339818, 0.968645442], 15/15 |
| w16/ON/frozen | 0.967978478 [0.966647888, 0.968750451], 15/15 |
| w32/ON/OFF | 1.011700629 [0.990632907, 1.025630377], 5/15 |
| w32/ON/frozen | 1.036956242 [1.018759255, 1.046190991], 2/15 |

## Complete timings

Times are median ns per complete batch wave, not proof latency or concurrent-proof throughput. Paired ratios use the same fixed block and need not equal ratios of displayed medians. All 24 contexts remain.

| Context | Frozen ns/wave | OFF ns/wave | ON ns/wave | ON/OFF ratio [interval], wins | ON/frozen ratio [interval], wins | OFF/frozen ratio [interval], wins |
|---|---:|---:|---:|---|---|---|
| G1-200-w1 | 4258.020 | 4257.359 | 4002.540 | 0.940105625 [0.938903029, 0.942603733], 15/15 | 0.940116220 [0.938530486, 0.943289520], 15/15 | 1.000560926 [0.998514641, 1.001292136], 6/15 |
| G1-200-w16 | 4838.927 | 4845.926 | 4593.312 | 0.947870694 [0.945061445, 0.951379041], 15/15 | 0.949646962 [0.948727882, 0.953563635], 15/15 | 1.000904330 [0.999843658, 1.005375549], 4/15 |
| G1-200-w32 | 6932.685 | 6934.868 | 6950.958 | 1.000940159 [0.859830095, 1.007494642], 6/15 | 1.005777841 [0.996323399, 1.150581871], 4/15 | 1.006615246 [0.997159949, 1.181306628], 4/15 |
| G1-201-w1 | 4380.302 | 4377.961 | 4134.743 | 0.944724301 [0.939310034, 0.946923087], 15/15 | 0.941783524 [0.940307347, 0.948958372], 15/15 | 1.000828164 [0.998393858, 1.002188200], 5/15 |
| G1-201-w16 | 4972.807 | 4974.726 | 4728.360 | 0.950251740 [0.945136315, 0.954853945], 15/15 | 0.949808934 [0.940159343, 0.954406945], 15/15 | 0.999531867 [0.989379239, 1.002661837], 9/15 |
| G1-201-w32 | 7116.211 | 7124.348 | 7149.631 | 1.004037138 [0.876095310, 1.010614598], 6/15 | 1.004329694 [0.996991830, 1.011940370], 5/15 | 0.995758158 [0.991135964, 1.004411782], 8/15 |
| G1-352-w1 | 7004.303 | 7003.810 | 6565.917 | 0.936222209 [0.935375636, 0.938940192], 15/15 | 0.937548478 [0.934580832, 0.939598922], 15/15 | 0.999033611 [0.997258935, 1.000890663], 8/15 |
| G1-352-w16 | 7989.023 | 7998.884 | 7557.412 | 0.945241968 [0.942735244, 0.946151567], 15/15 | 0.946246968 [0.943592574, 0.947396556], 15/15 | 1.000784682 [0.997464076, 1.003384926], 7/15 |
| G1-352-w32 | 11631.460 | 11645.957 | 11629.430 | 0.995984096 [0.804932342, 1.011512973], 10/15 | 1.008647109 [0.994473638, 1.188571614], 6/15 | 1.002899600 [0.998445764, 1.267329564], 5/15 |
| G1-353-w1 | 7109.888 | 7112.987 | 6683.349 | 0.939603210 [0.938605740, 0.940147517], 15/15 | 0.940187881 [0.938623972, 0.940844140], 15/15 | 1.000470687 [0.999217628, 1.001161593], 7/15 |
| G1-353-w16 | 8124.726 | 8125.250 | 7682.134 | 0.944531599 [0.943411028, 0.947118575], 15/15 | 0.946431986 [0.942543111, 0.946708561], 15/15 | 0.999412390 [0.996937525, 1.002764908], 8/15 |
| G1-353-w32 | 11764.322 | 11721.482 | 11900.408 | 1.012190397 [0.994916664, 1.232880836], 4/15 | 1.004748041 [0.997000287, 1.235538060], 4/15 | 0.999108332 [0.993723848, 1.002986758], 8/15 |
| G2-200-w1 | 7753.744 | 7749.040 | 7600.100 | 0.980000558 [0.977491100, 0.981564690], 15/15 | 0.979942547 [0.976499996, 0.980773707], 15/15 | 0.998969020 [0.997357682, 1.000745565], 9/15 |
| G2-200-w16 | 8973.596 | 8972.320 | 8874.706 | 0.988926066 [0.985792131, 0.991342274], 15/15 | 0.989548939 [0.987850479, 0.990643303], 15/15 | 1.000041334 [0.999118008, 1.000514387], 7/15 |
| G2-200-w32 | 14380.522 | 14423.156 | 15041.132 | 1.042526965 [0.993397817, 1.098162469], 5/15 | 1.045170348 [1.039586768, 1.100270300], 1/15 | 1.003237087 [0.998737274, 1.051205397], 6/15 |
| G2-201-w1 | 8187.203 | 8193.681 | 8042.427 | 0.980567595 [0.978281015, 0.982950688], 15/15 | 0.980375774 [0.978736593, 0.983483798], 15/15 | 1.000465692 [0.998878078, 1.002763325], 6/15 |
| G2-201-w16 | 9472.994 | 9477.212 | 9351.342 | 0.987167428 [0.986278657, 0.988080867], 15/15 | 0.987061280 [0.983943864, 0.989088122], 15/15 | 0.999707253 [0.996304775, 1.003124547], 8/15 |
| G2-201-w32 | 14941.406 | 15056.533 | 15524.319 | 1.036527492 [0.988393529, 1.044160976], 5/15 | 1.038075069 [0.992028919, 1.043768031], 5/15 | 0.999751526 [0.968593312, 1.050399691], 8/15 |
| G2-352-w1 | 13121.157 | 13125.951 | 12808.755 | 0.975525333 [0.974527222, 0.977792076], 15/15 | 0.975457442 [0.974147970, 0.977711811], 15/15 | 0.999576499 [0.998545952, 1.001200664], 8/15 |
| G2-352-w16 | 15264.853 | 15242.171 | 15092.233 | 0.990179253 [0.989031792, 0.991384887], 15/15 | 0.988154274 [0.985938071, 0.989944208], 15/15 | 0.998234779 [0.995384131, 0.999323438], 13/15 |
| G2-352-w32 | 24914.846 | 24883.442 | 25841.776 | 1.046411823 [1.034085375, 1.052180886], 0/15 | 1.040253586 [1.034721370, 1.050460219], 0/15 | 0.999639024 [0.993109781, 1.002859330], 9/15 |
| G2-353-w1 | 13512.517 | 13514.912 | 13223.204 | 0.978767874 [0.978237758, 0.979358102], 15/15 | 0.977913209 [0.976926749, 0.978933057], 15/15 | 0.998719781 [0.998451256, 1.000420627], 11/15 |
| G2-353-w16 | 15701.807 | 15697.350 | 15538.171 | 0.989143893 [0.988443330, 0.992034407], 14/15 | 0.989297380 [0.988258654, 0.990835114], 14/15 | 0.999526900 [0.998071868, 1.001993454], 10/15 |
| G2-353-w32 | 25488.987 | 25507.254 | 26457.178 | 1.036937845 [1.032187645, 1.055109577], 0/15 | 1.039201370 [1.027842007, 1.046958730], 0/15 | 0.991669755 [0.988401522, 1.006795009], 9/15 |

All failed checks:

- `G1-200-w32/OFF/frozen/calibration`
- `G1-200-w32/ON/frozen/nonregression`
- `G1-201-w32/ON/frozen/nonregression`
- `G1-201-w32/ON/OFF/nonregression`
- `G1-352-w32/OFF/frozen/calibration`
- `G1-352-w32/ON/frozen/nonregression`
- `G1-352-w32/ON/OFF/nonregression`
- `G1-353-w32/ON/frozen/nonregression`
- `G1-353-w32/ON/OFF/nonregression`
- `G2-200-w32/OFF/frozen/calibration`
- `G2-200-w32/ON/frozen/nonregression`
- `G2-200-w32/ON/OFF/nonregression`
- `G2-201-w32/OFF/frozen/calibration`
- `G2-201-w32/ON/frozen/nonregression`
- `G2-201-w32/ON/OFF/nonregression`
- `G2-352-w32/ON/frozen/nonregression`
- `G2-352-w32/ON/OFF/nonregression`
- `G2-353-w32/ON/frozen/nonregression`
- `G2-353-w32/ON/OFF/nonregression`
- `LOGICAL32-aggregate/ON/frozen/gain`
- `LOGICAL32-aggregate/ON/OFF/gain`

## What changed and what the counters show

The [qualified kernel](../2026-11-06-split-redc-native176/README.md) keeps product and reduction coefficients in two planes for the first four CIOS rows, merges before the original final row, and retains the final representation/corrections. Its source DAG and partial register-only chains shorten, but the actual [caller code](../2026-11-06-split-redc-callers176/README.md) adds vector additions and stack operands with the same IFMA counts. There is no new heap buffer, lookup table, key/point copy, extra plan or production pool.

| PMU context | ON/OFF retired instructions | ON/OFF cycles |
|---|---:|---:|
| G1-200-w1 | +5.398% | -6.332% |
| G1-200-w32 | +5.398% | -0.685% |
| G1-352-w32 | +5.777% | -1.130% |
| G2-200-w1 | +6.281% | -2.441% |
| G2-200-w32 | +6.282% | +3.416% |
| G2-352-w32 | +6.484% | +4.003% |

These fixed post-latency diagnostics support the measured tradeoff: solo cycle improvements coexist with more instructions and G2 32-worker cycle regressions. They do not isolate execution ports, spill latency, cache levels, store forwarding or a cause for the control variability. Generic cache-miss/branch counters and absolute counts remain in the JSON; a large ratio from a low baseline is not a cache-capacity conclusion. The 32-worker G2 cases keep roughly 31–32 CPUs busy and have low timed minor-fault rates. This is utilization of the loaded caller test, not the whole proof pipeline, and does not establish a RAM-limit explanation.

## Protocol and qualification

All 1,279 workflow commands succeed once: 72 preflights plus checker, 1,080 latency processes in 15 paired blocks, 18 PMU calls, 18 cycle profiles and 90 offline ownership/decode steps. All 1,188 caller processes pass complete generator/output-hash, unchanged-capacity and zero-timed-allocation checks. Campaign and independent Rust reviewer were frozen before clocks; each independently reconstructs all 76 checks from the saved observations. Sorted zero-based ranks 3/7/11 of 15 are marginal intervals, without family-wise coverage. There is no minimum positive-gain floor.

Each process uses three 128-round primes. Latency rounds per worker are 32,768 for 200/201 and 16,384 for 352/353 points. Workers use CPU 2 solo, CPUs 0–15 at 16, or 0–31 at 32, with a nonexclusive host. No owned build/test/second benchmark/hash/archive overlaps latency. One small checkpoint write was made during correctness preflights (progress still at preflight 31), as permitted by the protocol.

After all latency, grouped user cycles/instructions/cache-miss/branch-miss counters have at least 90% running coverage. Profiles use fixed 16,000,000-cycle periods with FIFO timed-region control. All recordings precede ownership/hash/decode work; each stream is decoded once. Executable/PID/TID/FORK/MMAP/CPU/period checks pass, and no successful stage is repeated.

All **18 profiles qualify with 119,477 samples**, zero lost events and all worker samples on assigned workers. Minimum native-kernel/inverse attribution is 99.8712% and maximum unknown attribution 0.0649%. Flat samples have skid and are not exclusive costs or removable proof time.

## Resources

Scratch capacities match across all arms and before/after timing; no timed heap allocations occur. Candidate maximum observed process RSS is **18.75 MiB** in this small caller matrix, including fixture setup and threads. This is not full-proof peak RAM, forced-memory evidence or minimum RAM. CPU-time/wall ratios and timed minor faults are diagnostic only.

| Context | OFF / ON peak RSS MiB | OFF / ON median busy CPUs | OFF / ON minor faults per wave |
|---|---|---|---|
| G1-200-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000061035 / 0.000061035 |
| G1-200-w16 | 3.75 / 3.75 | 15.4758 / 15.5381 | 0.000091553 / 0.000091553 |
| G1-200-w32 | 5.25 / 5.25 | 31.4110 / 31.4893 | 0.000091553 / 0.000122070 |
| G1-201-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000061035 / 0.000061035 |
| G1-201-w16 | 3.75 / 3.75 | 15.5318 / 15.5433 | 0.000061035 / 0.000061035 |
| G1-201-w32 | 5.0 / 5.25 | 31.4956 / 31.5306 | 0.000061035 / 0.000061035 |
| G1-352-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000122070 / 0.000122070 |
| G1-352-w16 | 5.5 / 5.25 | 15.5304 / 15.5681 | 0.000183105 / 0.000122070 |
| G1-352-w32 | 8.5 / 8.5 | 31.3396 / 31.4953 | 0.000183105 / 0.000183105 |
| G1-353-w1 | 3.0 / 3.0 | 1.0000 / 1.0000 | 0.000122070 / 0.000122070 |
| G1-353-w16 | 5.5 / 5.5 | 15.5319 / 15.5489 | 0.000122070 / 0.000122070 |
| G1-353-w32 | 8.5 / 8.5 | 31.5513 / 31.4098 | 0.000122070 / 0.000122070 |
| G2-200-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000061035 / 0.000061035 |
| G2-200-w16 | 5.0 / 5.0 | 15.6723 / 15.6462 | 0.000061035 / 0.000061035 |
| G2-200-w32 | 7.5 / 7.75 | 31.6310 / 31.7066 | 0.000061035 / 0.000061035 |
| G2-201-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000061035 / 0.000061035 |
| G2-201-w16 | 5.0 / 5.25 | 15.6208 / 15.6586 | 0.000061035 / 0.000061035 |
| G2-201-w32 | 7.75 / 7.75 | 31.3084 / 31.7333 | 0.000061035 / 0.000091553 |
| G2-352-w1 | 3.0 / 3.0 | 1.0000 / 1.0000 | 0.000122070 / 0.000122070 |
| G2-352-w16 | 9.5 / 9.0 | 15.6693 / 15.6437 | 0.000122070 / 0.000122070 |
| G2-352-w32 | 15.25 / 16.25 | 31.3717 / 31.5700 | 0.000122070 / 0.000122070 |
| G2-353-w1 | 3.0 / 3.0 | 1.0000 / 1.0000 | 0.000122070 / 0.000122070 |
| G2-353-w16 | 10.5 / 10.5 | 15.6451 / 15.6565 | 0.000122070 / 0.000122070 |
| G2-353-w32 | 18.75 / 18.75 | 31.2447 / 31.4743 | 0.000122070 / 0.000183105 |

## Unchanged full-proof baseline

These are inherited accepted-176 local 32-worker proof times, not this candidate’s proof results. The accepted physical-16 calibration engineering exception and prior false full gates remain documented; this study does not reopen them. SDK6d83 and canonical proof ledger 30,846 stay current. The inherited 132.75 MiB RSS is not a new peak or minimum RAM.

| Shape | Accepted CPU ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | CPU 10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 | 4.667× | 24 | 3.027× | 3.7 |
| 2x2 | 11.151932 | 59 | 5.291× | 30 | 2.690× | 5.9 |
| 4x2 | 16.086386 | 109 | 6.776× | 48 | 2.984× | 10.9 |
| 8x2 | 24.179453 | 189 | 7.817× | 74 | 3.060× | 18.9 |
| 12x2 | 33.884455 | 284 | 8.381× | 101 | 2.981× | 28.4 |
| 16x2 | 40.077381 | 335 | 8.359× | 124 | 3.094× | 33.5 |
| 24x2 | 65.850777 | 528 | 8.018× | 176 | 2.673× | 52.8 |
| merge24 | 65.060937 | 507 | 7.793× | 175 | 2.690× | 50.7 |

Inherited merge54 remains 161.883899 ms without a supplied M5 reference. The original all-eight CPU 10× goal remains active and unmet (0/8). No new SDK/Gnark proof, AWS/Aeglos, portability, huge-circuit, pressure, swap, minimum-RAM or concurrent-proof measurement is claimed.

## Artifacts and continuation

The [decision](artifacts/decision.md) closes this exact implementation while preserving its positive lower-thread results. The [next step](artifacts/next.md) requires distinct useful-work removal with a full-proof impact argument, not a topology/layout/annotation retry or another generic profile sweep.

[Analysis and all ratios](artifacts/analysis.json), [independent review](artifacts/review.json), [worker summaries](artifacts/worker-topology-descriptive.json), [protocol](artifacts/protocol.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [identities](artifacts/identities.json), [profile symbols](artifacts/profile-symbols.json), [host metadata](artifacts/host-public.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded stream index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json) and [checksums](SHA256SUMS). Raw recordings/streams remain locally under `/home/ubuntu/src/zopro-perf-work/split-redc-capture176`. The selected UTF-8 archive excludes executable/library/key/witness/proof blobs; the public host file omits the unrelated process list.
