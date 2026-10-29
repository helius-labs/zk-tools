# Current Zopro, transferred proof-PGO profile and pinned Aeglos on C7a

The transferred PGO profile does not pass C7a qualification. Across all eight reference circuits it is approximately flat at 32 workers and slower by 5.704% at 64 and 4.997% at 96; both larger-worker aggregates lose all 15 blocks. The primary32 paired ratio is 1.001256 [0.998103, 1.008667], with 5/15 wins: no established gain. Seventeen of 27 PGO/plain nonregression checks fail. The candidate remains unadopted and production is unchanged.

At their respective fastest tested worker counts, plain Zopro records 11.915591 ms for 1×2 versus pinned Aeglos Sorted's 7.959184 ms. For 54→1, plain Zopro records 122.396711 ms versus 173.236337 ms, with maximum RSS 1,877.03 versus 13,115.29 MiB. Plain Zopro has a lower displayed best-count median on six of nine circuits; Aeglos on 1×2, 2×2 and 4×2. Best-count selection is descriptive; same-worker paired results are retained below. Neither Zopro build meets any of the eight supplied CPU-10× targets.

## Builds and fixed scope

Both Zopro builds use the exact 175 accepted source files, Rust 1.98.1 / LLVM 22.1.8 and `target-cpu=znver4`. All other retained baseline flags and the standard SDK driver are shared. Plain SDK SHA is `53cfecd8f2353d3842d3eeb02c1e106d4d23a13a3fb28b328027ec93eaeb6f5a`; PGO SDK SHA is `0a7c564ae62e13a833569184147dd7a92110276c29aeea666a22ee7d9c33b41a`. These are fresh Zen4 builds of current accepted source, not the byte-identical local Zen5 accepted executable. Each build freshly passes 33 field, 124 core and 8 prover tests. No new arithmetic-case count is claimed for reuse of the existing tests.

PGO consumes the same proof-only profile `0f9dbf4b…` from the [local pilot](../2026-10-29-proof-pgo175/README.md), trained across all nine shapes at 32 workers on Ryzen 9950X. There is no C7a retraining, profile choice or new training capture. The consumer contains no LLVM profiling-runtime symbols; all compiler warnings are retained. The [broader local qualification](../2026-10-29-proof-pgo-qualification175/README.md) retains its aggregate gain and failed physical16 calibration check. This is an explicit architecture-transfer experiment; the local result is not universal compiler evidence.

Aeglos reuses pinned commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, its Sorted engine, existing preparation policy, qualified native archive and pool-aware SDK driver. It is not asserted to be the latest upstream. The adapter checks prepared-key cache hits and preserves the driver's verified-request timer. Current Zopro images and established underlying keys/fixture bindings are pinned. No per-key lookup expansion, source change, new memory cap or prover architecture change occurs.

One dedicated c7a.24xlarge provides 96 physical cores without SMT. Cases use 32/64/96 workers pinned to CPUs 0…N−1. The network-disabled container has CPU quota 96, memory 30 GiB and no swap. This is a benchmark container limit, not a Zopro architectural cap or minimum RAM requirement. Fifteen fixed blocks rotate/reverse both context and engine order across all nine circuits, including 54→1. Primary processes measure 101 requests after three primes; other circuits 17 after three. Loading/preparation/primes are excluded from latency but included in process lifetime RSS. Timers include fresh assignment/witness solving, fresh randomness, proving, internal verification and serialization.

All 1,215 latency processes finish before their independent Gnark verifications. Six separate 199 Hz user-cycle profiles follow for primary32/96 across all three engines, 251 requests after three primes with FIFO enable/ack. All six finish before their verification, decoding and raw-loss audits. No owned build, hash, archive or other CPU helper overlaps captures. All helpers are prepared before capture; post-capture analysis and report helpers do not run during measurement. No successful capture or test is repeated, extended or excluded.

## Complete latency and resources

Each cell is the median of 15 process medians. CPU columns are median per-process mean user+system CPU milliseconds per request. RSS is the maximum lifetime process RSS across the 15 processes, including load and preparation; it is not a cgroup peak, incremental workspace or minimum RAM.

|Circuit|Workers|Plain ms|PGO ms|Aeglos ms|Plain CPU ms|PGO CPU ms|Aeglos CPU ms|Plain max RSS MiB|PGO max RSS MiB|Aeglos max RSS MiB|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2|32|11.915591|11.900208|11.867991|256.128|255.186|328.248|120.97|124.11|426.57|
|1x2|64|11.994779|12.482933|8.773943|420.399|437.975|430.446|129.21|127.07|448.28|
|1x2|96|13.777708|14.290857|7.959184|704.247|761.710|513.394|120.29|121.06|480.14|
|2x2|32|15.895282|15.817342|18.578617|352.233|348.497|536.656|230.36|223.10|716.71|
|2x2|64|16.992010|17.220410|13.230513|588.806|614.233|679.635|234.27|223.75|757.38|
|2x2|96|18.748760|19.394289|11.822046|995.139|1058.785|831.085|222.27|221.46|811.11|
|4x2|32|19.992623|19.985021|30.724820|513.152|511.277|916.840|276.21|281.98|1254.26|
|4x2|64|20.129692|20.436394|21.020963|859.562|887.362|1140.264|272.58|278.61|1296.08|
|4x2|96|21.930211|22.262369|18.492036|1369.573|1443.307|1397.460|261.27|274.00|1351.90|
|8x2|32|27.524281|27.769358|54.899915|761.266|765.980|1656.760|470.83|464.25|2316.52|
|8x2|64|26.897505|26.951272|35.662293|1286.219|1298.314|2085.367|450.82|459.12|2345.90|
|8x2|96|28.780558|29.720680|31.530774|1955.338|2087.394|2573.850|458.83|458.24|2491.09|
|12x2|32|36.260757|36.272816|86.163804|1011.020|1015.953|2636.287|644.45|647.61|3434.15|
|12x2|64|34.861964|36.820206|56.200677|1624.744|1664.337|3339.684|648.76|645.98|3512.52|
|12x2|96|37.074308|38.968668|48.579665|2498.789|2613.789|4110.262|640.70|648.89|3714.61|
|16x2|32|42.275779|42.309120|105.108051|1203.084|1199.859|3170.738|766.91|812.57|4337.23|
|16x2|64|38.983221|43.198532|65.075319|1813.090|1873.117|3852.942|766.20|787.02|4406.80|
|16x2|96|43.283881|46.035874|57.135549|2824.982|2943.121|4887.969|785.29|775.73|4612.36|
|24x2|32|61.438334|61.220990|171.512323|1787.654|1786.841|5186.095|1235.95|1216.25|6394.04|
|24x2|64|54.034119|61.639738|107.006571|2571.207|2660.334|6249.930|1225.00|1225.88|6436.07|
|24x2|96|58.646533|63.934294|86.666616|3748.582|3907.228|7441.079|1254.74|1206.76|6758.14|
|merge-24x1|32|62.440127|62.700468|166.080733|1715.071|1731.410|5043.665|1096.54|1095.73|6171.44|
|merge-24x1|64|57.132869|63.619142|101.635962|2541.788|2635.583|5977.826|1109.38|1105.95|6248.05|
|merge-24x1|96|62.695917|67.902674|84.350321|3821.731|4023.742|7243.954|1112.32|1136.04|6608.05|
|merge-54x1|32|138.189311|138.251455|347.611126|3526.573|3524.751|10677.124|1826.39|1880.12|12512.38|
|merge-54x1|64|122.396711|125.814258|207.563258|4701.831|4860.606|12212.661|1877.03|1885.46|12584.70|
|merge-54x1|96|126.994039|131.047737|173.236337|6494.773|6774.422|14983.932|1910.71|1896.00|13115.29|

## Paired results and fixed checks

Ratios use all 15 paired blocks, median rank 8 and bounds at ranks 4 and 12. Nominal individual coverage is 96.484375% under independent, exchangeable blocks; these are not simultaneous/familywise guarantees. Paired ratios and savings need not equal ratios or differences of displayed marginal medians. In particular, primary32's displayed PGO median is slightly lower, while its paired estimate is slightly slower and uncertain.

PGO qualification requires primary32 upper ratio <1 and at least 10/15 wins, upper ratio ≤1.03 in all 27 PGO/plain contexts, and strict gain with at least 10 wins for each worker count's equal-log aggregate of all eight reference shapes. There is no positive-gain floor, post-capture threshold waiver or subset promotion. Same-worker Aeglos win checks use strict upper ratio <1 and at least 10/15 wins; best-count selection is kept separate. All fixed PGO gain conditions fail, and 17 nonregression conditions fail. Positive individual estimates remain in the table.

|All-eight PGO/plain aggregate|Ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|all8-w32/PGO/plain|1.001184386|[0.995324066, 1.009458295]|7|
|all8-w64/PGO/plain|1.057039738|[1.043223751, 1.068515040]|0|
|all8-w96/PGO/plain|1.049965901|[1.037701326, 1.052830148]|0|

|Same-worker comparison|Ratio|Bounds|Wins / 15|Paired saving µs|
|---|---:|---|---:|---:|
|12x2-w32/PGO/aeglos|0.418113928|[0.414202990, 0.428098911]|15|50027.688|
|12x2-w32/PGO/plain|1.000332563|[0.993354904, 1.009978304]|7|-12.059|
|12x2-w32/plain/aeglos|0.417819439|[0.412280904, 0.425790898]|15|50099.685|
|12x2-w64/PGO/aeglos|0.651705404|[0.646017152, 0.657923128]|15|19564.751|
|12x2-w64/PGO/plain|1.053810397|[1.030410956, 1.069872108]|0|-1889.134|
|12x2-w64/plain/aeglos|0.618588874|[0.606849497, 0.628760612]|15|21310.221|
|12x2-w96/PGO/aeglos|0.805469657|[0.797987799, 0.814716718]|15|9471.173|
|12x2-w96/PGO/plain|1.056120157|[1.036228038, 1.064211274]|0|-2080.616|
|12x2-w96/plain/aeglos|0.761425456|[0.757097974, 0.781936675]|15|11547.967|
|16x2-w32/PGO/aeglos|0.405066554|[0.399015522, 0.412401206]|15|62659.246|
|16x2-w32/PGO/plain|0.993005998|[0.987519423, 1.004822088]|10|299.730|
|16x2-w32/plain/aeglos|0.404758545|[0.401134670, 0.414270682]|15|62173.431|
|16x2-w64/PGO/aeglos|0.663823592|[0.653800203, 0.672114682]|15|21876.787|
|16x2-w64/PGO/plain|1.097908858|[1.081397937, 1.116899689]|0|-3782.660|
|16x2-w64/plain/aeglos|0.600038200|[0.590599162, 0.610162991]|15|25901.021|
|16x2-w96/PGO/aeglos|0.805583578|[0.795459306, 0.815287555]|15|11108.089|
|16x2-w96/PGO/plain|1.067831952|[1.048452818, 1.084978401]|0|-2880.326|
|16x2-w96/plain/aeglos|0.755413563|[0.740783403, 0.761925872]|15|14019.663|
|1x2-w32/PGO/aeglos|1.003581061|[0.994196279, 1.007267612]|5|-42.500|
|1x2-w32/PGO/plain|1.001255813|[0.998103476, 1.008666837]|5|-14.970|
|1x2-w32/plain/aeglos|1.004010788|[0.992807639, 1.009670244]|7|-47.600|
|1x2-w64/PGO/aeglos|1.422202954|[1.396691377, 1.442282766]|0|-3664.196|
|1x2-w64/PGO/plain|1.031734663|[1.026111604, 1.058735889]|1|-373.281|
|1x2-w64/plain/aeglos|1.360080164|[1.347290722, 1.384309155]|0|-3177.234|
|1x2-w96/PGO/aeglos|1.797622880|[1.756748575, 1.822135202]|0|-6295.367|
|1x2-w96/PGO/plain|1.032710636|[1.015276852, 1.053213765]|0|-444.990|
|1x2-w96/plain/aeglos|1.724341951|[1.706624864, 1.744136037]|0|-5778.355|
|24x2-w32/PGO/aeglos|0.356111195|[0.351820007, 0.366758043]|15|109981.781|
|24x2-w32/PGO/plain|1.005068881|[0.986900670, 1.012940568]|6|-305.550|
|24x2-w32/plain/aeglos|0.360837931|[0.351033610, 0.365276504]|15|111180.030|
|24x2-w64/PGO/aeglos|0.576001637|[0.541493262, 0.581006315]|15|45391.844|
|24x2-w64/PGO/plain|1.133384636|[1.060826697, 1.150294575]|0|-7289.407|
|24x2-w64/plain/aeglos|0.508853872|[0.501022876, 0.511081850]|15|52651.027|
|24x2-w96/PGO/aeglos|0.736894770|[0.707392722, 0.743349488]|15|22998.919|
|24x2-w96/PGO/plain|1.081767739|[1.055478681, 1.104305127]|0|-4832.611|
|24x2-w96/plain/aeglos|0.674825620|[0.665440371, 0.680342344]|15|28290.173|
|2x2-w32/PGO/aeglos|0.850857255|[0.836052825, 0.861262179]|15|2731.588|
|2x2-w32/PGO/plain|0.994647995|[0.987803665, 1.012319569]|9|85.110|
|2x2-w32/plain/aeglos|0.854256886|[0.848040703, 0.860459957]|15|2697.696|
|2x2-w64/PGO/aeglos|1.290136323|[1.280676968, 1.313223741]|0|-3802.181|
|2x2-w64/PGO/plain|1.022767981|[0.993150705, 1.034334968]|4|-387.541|
|2x2-w64/plain/aeglos|1.276630590|[1.236313340, 1.322280430]|0|-3719.459|
|2x2-w96/PGO/aeglos|1.635131362|[1.616676246, 1.679743360]|0|-7547.017|
|2x2-w96/PGO/plain|1.041263112|[1.031530395, 1.048158492]|1|-765.292|
|2x2-w96/plain/aeglos|1.594349564|[1.558672681, 1.612063101]|0|-6913.908|
|4x2-w32/PGO/aeglos|0.654885111|[0.643073474, 0.660033151]|15|10538.514|
|4x2-w32/PGO/plain|0.994873767|[0.989089681, 1.010947750]|8|103.003|
|4x2-w32/plain/aeglos|0.649494388|[0.642488548, 0.660534211]|15|10773.028|
|4x2-w64/PGO/aeglos|0.974122573|[0.964799189, 0.981438950]|14|540.841|
|4x2-w64/PGO/plain|1.019988785|[0.998770536, 1.039805681]|4|-403.161|
|4x2-w64/plain/aeglos|0.953059767|[0.938655379, 0.966052022]|15|980.983|
|4x2-w96/PGO/aeglos|1.205905504|[1.172709157, 1.227981874]|0|-3807.612|
|4x2-w96/PGO/plain|1.015122525|[1.004717697, 1.039307823]|3|-331.311|
|4x2-w96/plain/aeglos|1.200243121|[1.146069231, 1.215428336]|0|-3702.903|
|8x2-w32/PGO/aeglos|0.510993383|[0.499721293, 0.514803799]|15|26781.120|
|8x2-w32/PGO/plain|1.007908308|[0.999194558, 1.017374598]|4|-219.091|
|8x2-w32/plain/aeglos|0.505159689|[0.499068518, 0.511400384]|15|27166.691|
|8x2-w64/PGO/aeglos|0.758784195|[0.745991701, 0.768319633]|15|8546.515|
|8x2-w64/PGO/plain|1.012246898|[0.991348495, 1.027058482]|5|-329.411|
|8x2-w64/plain/aeglos|0.750412975|[0.745407397, 0.760979707]|15|8926.724|
|8x2-w96/PGO/aeglos|0.944337977|[0.929364691, 0.956781594]|15|1754.894|
|8x2-w96/PGO/plain|1.024032733|[1.003889798, 1.049333577]|2|-692.101|
|8x2-w96/plain/aeglos|0.912528456|[0.897810185, 0.929803729]|15|2758.796|
|merge-24x1-w32/PGO/aeglos|0.377690137|[0.370357379, 0.385449015]|15|102917.729|
|merge-24x1-w32/PGO/plain|1.010008898|[0.997356362, 1.019926162]|4|-625.624|
|merge-24x1-w32/plain/aeglos|0.376288391|[0.369948064, 0.383466064]|15|102618.169|
|merge-24x1-w64/PGO/aeglos|0.625338651|[0.602148363, 0.637402581]|15|38192.940|
|merge-24x1-w64/PGO/plain|1.098821578|[1.063668569, 1.123264211]|0|-5818.522|
|merge-24x1-w64/plain/aeglos|0.565444074|[0.557344632, 0.573173492]|15|44384.740|
|merge-24x1-w96/PGO/aeglos|0.810340748|[0.778185059, 0.824674251]|15|16007.532|
|merge-24x1-w96/PGO/plain|1.083470285|[1.046647806, 1.088880748]|0|-5175.550|
|merge-24x1-w96/plain/aeglos|0.745282806|[0.738791869, 0.751592919]|15|21587.280|
|merge-54x1-w32/PGO/aeglos|0.398505924|[0.394929556, 0.402425893]|15|209086.033|
|merge-54x1-w32/PGO/plain|1.002221923|[0.993440035, 1.006401911]|7|-306.021|
|merge-54x1-w32/plain/aeglos|0.397365602|[0.394747138, 0.399865987]|15|208874.762|
|merge-54x1-w64/PGO/aeglos|0.606716626|[0.603485733, 0.608806845]|15|81459.936|
|merge-54x1-w64/PGO/plain|1.028380786|[1.019611403, 1.036823232]|0|-3475.750|
|merge-54x1-w64/plain/aeglos|0.590811008|[0.587461661, 0.594146299]|15|85107.272|
|merge-54x1-w96/PGO/aeglos|0.758308684|[0.754048462, 0.770542545]|15|41768.083|
|merge-54x1-w96/PGO/plain|1.032158164|[1.029520865, 1.036822795]|0|-4075.400|
|merge-54x1-w96/plain/aeglos|0.736450258|[0.732426595, 0.739073370]|15|45795.315|

## Best tested worker counts

These selections describe this complete matrix and are not a generally optimal worker policy or a new paired comparison across different worker counts. All same-worker observations remain above.

|Circuit|Plain ms (workers)|PGO ms (workers)|Aeglos ms (workers)|Plain max RSS MiB|Aeglos max RSS MiB|
|---|---:|---:|---:|---:|---:|
|1x2|11.915591 (32)|11.900208 (32)|7.959184 (96)|120.97|480.14|
|2x2|15.895282 (32)|15.817342 (32)|11.822046 (96)|230.36|811.11|
|4x2|19.992623 (32)|19.985021 (32)|18.492036 (96)|276.21|1351.90|
|8x2|26.897505 (64)|26.951272 (64)|31.530774 (96)|450.82|2491.09|
|12x2|34.861964 (64)|36.272816 (32)|48.579665 (96)|648.76|3714.61|
|16x2|38.983221 (64)|42.309120 (32)|57.135549 (96)|766.20|4612.36|
|24x2|54.034119 (64)|61.220990 (32)|86.666616 (96)|1225.00|6758.14|
|merge-24x1|57.132869 (64)|62.700468 (32)|84.350321 (96)|1109.38|6608.05|
|merge-54x1|122.396711 (64)|125.814258 (64)|173.236337 (96)|1877.03|13115.29|

## Supplied M5 CPU and Metal references

These are supplied historical M5 figures, not new Apple measurements. Every original reference shape is retained. No M5 reference was supplied for 54→1. Both Zopro builds remain at 0/8 CPU-10× targets, so the original goal remains active and unmet. Speedups below use each engine's descriptive fastest tested count. Full PGO/Aeglos CPU+Metal ratios are also retained in analysis.json.

|Shape|M5 CPU ms|M5 Metal ms|Plain ms|Plain CPU speedup|Plain Metal speedup|PGO ms|PGO CPU speedup|PGO Metal speedup|CPU-10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|12x2|284|101|34.861964|8.146×|2.897×|36.272816|7.830×|2.784×|28.4|
|16x2|335|124|38.983221|8.593×|3.181×|42.309120|7.918×|2.931×|33.5|
|1x2|37|24|11.915591|3.105×|2.014×|11.900208|3.109×|2.017×|3.7|
|24x2|528|176|54.034119|9.772×|3.257×|61.220990|8.624×|2.875×|52.8|
|2x2|59|30|15.895282|3.712×|1.887×|15.817342|3.730×|1.897×|5.9|
|4x2|109|48|19.992623|5.452×|2.401×|19.985021|5.454×|2.402×|10.9|
|8x2|189|74|26.897505|7.027×|2.751×|26.951272|7.013×|2.746×|18.9|
|merge-24x1|507|175|57.132869|8.874×|3.063×|62.700468|8.086×|2.791×|50.7|

## Profiles, scaling and limits

All six profiles contain one SDK PID and zero decoded lost records, confirmed by separate raw perf record audits. The following cycle-residence percentages are flat symbol classifications, not exclusive work, idle-time measurements or removable wall-time fractions. PGO can change inlining and symbol attribution, so epoch and scheduler categories should be considered together; their split alone does not establish a mechanism. Aeglos matches `other` in the inherited generic classifier; its complete raw symbols remain available.

|Profile|Samples|G1 %|G2 %|Epoch %|Scheduler %|Epoch + scheduler %|
|---|---:|---:|---:|---:|---:|---:|
|cycles-1x2-w32-PGO|14067|36.99|27.99|1.96|12.67|14.63|
|cycles-1x2-w32-aeglos|17402|unclassified|unclassified|unclassified|unclassified|unclassified|
|cycles-1x2-w32-plain|15395|37.13|28.48|8.94|5.15|14.09|
|cycles-1x2-w96-PGO|45702|13.77|11.29|18.07|45.76|63.83|
|cycles-1x2-w96-aeglos|27439|unclassified|unclassified|unclassified|unclassified|unclassified|
|cycles-1x2-w96-plain|54063|14.11|12.51|48.59|12.89|61.48|

Plain Zopro's combined epoch/scheduler residence rises from 14.09% at 32 workers to 61.48% at 96; PGO shows 14.63% and 63.83%. Plain primary CPU service rises from 256.128 to 704.247 ms/request while latency worsens from 11.915591 to 13.777708 ms. Aeglos's corresponding latency falls from 11.867991 to 7.959184 ms. This points further investigation toward work availability, critical paths and coordination at larger pools. It does not isolate a clock, memory-capacity, cache, syscall or fence cause, or predict a gain from removing a symbol's samples. Previous scheduling trials retain their outcomes; these profiles do not justify repeating unchanged rejected variants.

Whole-container workflow CPU counters record 39 throttled periods and 521,975 throttled microseconds. This scope includes loading, preparation, priming, independent verification, latency and profiling; it is not a per-variant or timed-only attribution. All memory/swap event deltas are zero, including OOM. Nonzero throttling is retained and no timing is repeated to remove it. Cgroup events and process RSS have different scopes. These measurements do not establish minimum RAM, concurrent throughput, phones, TEE overhead, portable builds or 250/4999-circuit performance.

## Verification and cleanup

Exact-container local preflight completes 108 SDK requests and 27 independent exports, all nine circuits across three engines at four workers. Cloud completes 37,164 SDK requests and 1,221 independent exports. Combined: **37,272 requests and 1,248 distinct Gnark-verified exports**, 2,496 proof/public members, ledger 19,261→20,509. Every retained proof is 164 bytes, fixture-bound and disjoint from all prior/new identities. All 1,221 cloud exports are independently reverified locally after SHA-256/size-checked retrieval; that declared repeat verification adds no new proofs. The first local verifier invocation found no proofs in the nested layout and verified zero; hash-checked flat copies then pass once. The original failure is retained.

The initial Docker packaging attempt failed before any SDK request or VM launch because `FROM sha256:localID` was treated as a registry reference. A continuation uses the existing local base tag after exact image-ID verification; native builds/tests are not rerun. Original helper source, failing logs, Dockerfile and repair evidence remain archived. The old benchmark subnet/profile no longer existed; a separate launch-template resolution uses the existing current qualification subnet, no-ingress security group and instance profile, preserving the original helper metadata. No network or IAM resource is created or modified.

Exactly one owned c7a.24xlarge (`i-019acb8ff0bdc76cc`) uses a 90-minute termination timer and encrypted delete-on-termination storage. The instance is confirmed terminated, its tagged volume absent, the unique ECR tag deleted, and the private transfer object/bucket and local URL files removed. No regional TemporaryProverBenchmark instance remains. No production resource is changed.

## Artifacts

[Analysis](artifacts/analysis.json), [fixed protocol](artifacts/design.md), [source/build audit](artifacts/source-audit.json), [all native/preflight/cloud receipts](artifacts/workflow-receipts.json.gz), [original driver streams](artifacts/raw-driver-streams.json.gz), [all proof/public bytes](artifacts/proof-public-members.json.gz), [cloud samples and profile symbols](artifacts/cloud-rows.json.gz), [raw perf data](artifacts/raw-perf-data.json.gz), [source snapshot](artifacts/source-snapshot.json.gz), [post-transport verification](artifacts/post-transport-gnark.log.json), [closing pins](artifacts/closing-pins.json), [closeout](artifacts/closeout.json). Each encoded archive member includes its SHA-256; decompression round-trips are checked. Artifact index and SHA256SUMS cover the report.
