# Full SDK qualification of balanced G2 windows

30 October2026. Complete verified requests across all nine circuits at32workers plus primary at16workers. The candidate follows a passed complete G2-query gate; its query gain is not assumed to transfer to proving. Production remains unchanged.

Declared full-request gate **false**; diagnostic qualification **true**. Failed checks: `["1x2-w16/ON/accepted","1x2-w16/ON/OFF","original8/ON/OFF/gain"]`.

The candidate is **not adopted**. At 32 workers, primary 1×2 measures **8.013709 ms and 133.75 MiB peak RSS**, versus accepted **8.042119 ms / 135.50 MiB** and fresh OFF **8.071193 ms / 138.25 MiB** in this run. Paired improvements are 0.416% versus accepted and 0.729% versus OFF; both primary gain checks pass. These are smaller full-proof gains than the earlier complete-query gains.

The original-eight aggregate improves 0.647% versus accepted, with its interval below parity. Versus fresh OFF, its estimate improves 0.305%, but its interval [0.994048990, 1.000201673] spans parity. Physical-16 primary estimates also improve, but upper bounds of 1.029952 and 1.030085 exceed the declared 1.02 nonregression tolerance. This is insufficient evidence for those checks, rather than an established 3% slowdown. All control-calibration, maximum-RSS and diagnostic checks pass. All observations are retained; the failed gate is not waived.

Production remains the accepted build at 8.022165 ms / 134 MiB in its qualification run. No fresh AWS comparison has been made for this candidate. The complete current comparison is in the [benchmark report](../2026-10-30-current-bench-report/README.md).

## Implementation and qualification

G2Phi2 uses eleven variable-width spans covering127bits: `[11,12,11,12,11,12,11,12,11,12,12]`, starts `[0,11,23,34,46,57,69,80,92,103,115]`. All use the existing ordinary borrowed Window, high component then phi then low, with original early-stopping LIFO queue and152/200entry batches. This replaces the narrow indexed top's routing, point-tile copies and reduction layers. The exact accepted K/H offset Booth reader avoids a digit array. Field arithmetic, scalar split, key operands and two-part geometry remain. No previous failed queue-scan, queue-drain, GLS4 or arithmetic candidate is mixed in.

The earlier count model reports15.26%fewer bucket output slots per part, but more ordinary affine/projective work; it does not model all original indexed-top arithmetic or inversion costs. In the complete-query trial primary32 improves3.110% and the two-shape32aggregate3.467% versus the API-only frozen baseline, both31/31wins. Primary16query latency is inconclusive despite lower CPU service. These are query results, not proof results. [Count screen](../2026-10-30-g2-balanced-count175/README.md), [complete-query trial](../2026-10-30-g2-balanced175/README.md).

SDK integration retains the exact qualified balanced child and two parent cfg/module changes, restoring the original lib.rs and removing the unused private query-probe flag/export. Native source176 has173exact originals; inverse edits recover all175. All17field files and workspace forecasts remain exact. Existing forecasts already charge eleven maximum-sized parents plus indexed-top extras; each new parent/leaf fits that envelope, without a new scalar vector, point/key table, duplicate plan, pool or retained state. Span descriptors request176stackbytes; compiler frames, minimum RAM and pressure behavior are not proven. Unsupported sizes/targets and disabled borrowed-prefix paths retain their original route.

Fresh nativeOFF/ON each pass127unfiltered core and8prover tests; portableON passes101core and8prover tests on generic x86-64/no_ifma. The three actual-curve tests retain whole sums, empty/full/tails, identities/doubling/opposites/sparse values, input point immutability and1/4/16/32custom pools. All33accepted field tests are inherited after exact source checks, without new field arithmetic or redundant execution. Original SDK driver and its perf helper are unchanged. [Source review](artifacts/final-source-review.json), [design](artifacts/design.md).

## Fixed measurement scope

Ryzen9 9950X,16physical cores/32logical CPUs,128GiB RAM, shared host. Nine circuits at32workers onCPUs0–31, primary also at16on0–15. Accepted is the retained exact20fde production executable; OFF/ON are coherent fresh SDK builds. Thirty-one fixed rotated/reversed blocks retain all10cases and all3variants. Each process primes three times, then times151primary or31other requests. Timers include fresh witness solve, randomized proving, SDK verification and serialization. Loading/preparation are outside latency and inside lifetime process RSS. Reported latency is the median of31process medians; CPU is median summed service/request, RSS the largest process high-water across blocks, not minimum RAM or incremental workspace.

Thirty preflight processes independently pass Gnark before latency. All930latency processes precede six grouped PMU panels and six fixed-period profiles, primary16/32across all3variants. All captures precede the remaining942independent proof verifications, ownership transfer, decode and analysis. Whole-process taskset includes the coordinator. Every helper, including this publisher/summary/source reviewer, was compiled and pinned before capture. No owned build/helper/hash/archive/analyzer or independent CPU workload overlaps captures.

|Case|Variant|Median ms|CPU ms/request|Peak RSS MiB|
|---|---|---:|---:|---:|
|12x2-w32|OFF|33.961610|938.909|716.25|
|12x2-w32|ON|33.908155|939.161|723.00|
|12x2-w32|accepted|33.908826|939.020|724.75|
|16x2-w32|OFF|40.110852|1121.942|836.75|
|16x2-w32|ON|40.052991|1122.227|864.00|
|16x2-w32|accepted|40.282697|1128.076|823.75|
|1x2-w16|OFF|9.735707|119.098|89.75|
|1x2-w16|ON|9.668965|113.316|91.75|
|1x2-w16|accepted|9.697928|120.431|89.50|
|1x2-w32|OFF|8.071193|182.428|138.25|
|1x2-w32|ON|8.013709|171.309|133.75|
|1x2-w32|accepted|8.042119|182.948|135.50|
|24x2-w32|OFF|65.961542|1889.391|1318.75|
|24x2-w32|ON|65.972846|1891.022|1312.61|
|24x2-w32|accepted|66.310026|1903.307|1311.30|
|2x2-w32|OFF|11.295129|262.642|272.50|
|2x2-w32|ON|11.225406|248.640|273.25|
|2x2-w32|accepted|11.319767|262.868|277.50|
|4x2-w32|OFF|15.955362|418.421|316.75|
|4x2-w32|ON|15.945477|419.657|317.25|
|4x2-w32|accepted|16.048913|420.261|318.00|
|8x2-w32|OFF|24.227034|667.411|518.25|
|8x2-w32|ON|24.096192|666.133|507.50|
|8x2-w32|accepted|24.300257|670.726|514.50|
|merge-24x1-w32|OFF|65.131764|1805.868|1169.55|
|merge-24x1-w32|ON|65.075009|1803.110|1177.45|
|merge-24x1-w32|accepted|65.245781|1805.156|1193.75|
|merge-54x1-w32|OFF|161.470804|4436.231|1874.37|
|merge-54x1-w32|ON|161.864048|4443.037|1924.79|
|merge-54x1-w32|accepted|162.256330|4456.229|1911.71|

|Paired comparison|Ratio|Bounds|Wins /31|Pass|
|---|---:|---|---:|---|
|12x2-w32/OFF/accepted|1.001392714|[0.994390538, 1.007784990]|15|true|
|12x2-w32/ON/OFF|0.998386555|[0.991873771, 1.003949853]|19|true|
|12x2-w32/ON/accepted|0.998778716|[0.993044466, 1.006158985]|16|true|
|16x2-w32/OFF/accepted|0.995443289|[0.989643536, 1.000327800]|21|true|
|16x2-w32/ON/OFF|0.999698266|[0.996174922, 1.004484280]|16|true|
|16x2-w32/ON/accepted|0.997155427|[0.985678788, 1.003205811]|18|true|
|1x2-w16/OFF/accepted|1.000939975|[0.983159258, 1.026766542]|15|true|
|1x2-w16/ON/OFF|0.993878085|[0.977024397, 1.030084544]|16|false|
|1x2-w16/ON/accepted|0.990701533|[0.974113289, 1.029951524]|16|false|
|1x2-w32/OFF/accepted|1.001119803|[0.997271708, 1.005788636]|11|true|
|1x2-w32/ON/OFF|0.992710208|[0.989573279, 0.998194757]|24|true|
|1x2-w32/ON/accepted|0.995843881|[0.994439313, 0.999216146]|23|true|
|24x2-w32/OFF/accepted|0.993660892|[0.988675832, 0.999557215]|22|true|
|24x2-w32/ON/OFF|0.998411344|[0.994786440, 1.005654287]|17|true|
|24x2-w32/ON/accepted|0.994306932|[0.986368885, 1.000171517]|20|true|
|2x2-w32/OFF/accepted|0.999749070|[0.993475746, 1.004643932]|16|true|
|2x2-w32/ON/OFF|0.991256243|[0.986012621, 0.997352441]|23|true|
|2x2-w32/ON/accepted|0.987761848|[0.984674194, 0.996855014]|24|true|
|4x2-w32/OFF/accepted|0.996933673|[0.986540852, 1.005020044]|17|true|
|4x2-w32/ON/OFF|0.996943449|[0.990975318, 1.005211504]|19|true|
|4x2-w32/ON/accepted|0.993961233|[0.985579573, 0.998899322]|22|true|
|8x2-w32/OFF/accepted|0.998070724|[0.986396999, 1.005521423]|17|true|
|8x2-w32/ON/OFF|0.992619388|[0.987500045, 1.000474151]|20|true|
|8x2-w32/ON/accepted|0.994349939|[0.986068919, 0.999284859]|22|true|
|merge-24x1-w32/OFF/accepted|0.998076807|[0.993244043, 1.001779259]|18|true|
|merge-24x1-w32/ON/OFF|0.998897548|[0.990142077, 1.005054182]|16|true|
|merge-24x1-w32/ON/accepted|0.995563465|[0.991751336, 1.002446657]|20|true|
|merge-54x1-w32/OFF/accepted|0.995520400|[0.990606555, 1.001329366]|21|true|
|merge-54x1-w32/ON/OFF|1.003254168|[0.996768515, 1.009041514]|14|true|
|merge-54x1-w32/ON/accepted|0.996342734|[0.993891319, 1.003683321]|19|true|

|Original-eight aggregate versus|Ratio|Bounds|Wins /31|Pass|
|---|---:|---|---:|---|
|OFF|0.996951509|[0.994048990, 1.000201673]|21|false|
|accepted|0.993530048|[0.991543018, 0.997731795]|27|true|

Primary32and equal-log original-eight aggregate require upper ON/control ratios below1 and≥21/31wins versus BOTHaccepted/OFF. Every other case, including16and54→1, requires upper≤1.02. All ten OFF/accepted calibration intervals must fit factor1.03; all ten maximum RSS ON/control ratios must be≤1.05versus both controls. This relative guard is not an architecture cap. Sorted ranks10–22give nominal97.055%individual median coverage under independent/exchangeable blocks, without a familywise guarantee. Ratios pair blocks and need not equal ratios of displayed medians. No minimum gain floor, exclusions, extensions, changed thresholds, favorable subsets or unchanged reroll. Diagnostic qualifications also remain required. Passing performance selects separate resource/admission and final-artifact checks, not adoption or objective completion. The accepted build's historical physical16calibration exception is not inherited.

## Automatic counters and profiles

Each grouped PMU panel has31requests after3primes and requires≥99.9%event running coverage. Profiles use251requests after3primes with fixed16million cycles:u period, explicit CPU/time/period and executable/task metadata. FIFO gates exclude setup/primes and final exports/checking. OnePID, valid affinity/mappings/events, zero loss,≥1000samples and≤5%unknown period are required. These single panels are diagnostic, not repeated-trial confidence intervals or isolated causes. The known absolute-period coverage deficit remains unresolved: flat shares are qualitative, not exclusive costs, local CPI or removable proof wall time.

|Panel|Instructions/request M|Cycles/request M|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|1377.399|564.461|true|
|stat-1x2-w16-ON|1318.265|540.069|true|
|stat-1x2-w16-accepted|1380.807|585.781|true|
|stat-1x2-w32-OFF|1419.524|834.857|true|
|stat-1x2-w32-ON|1358.163|780.023|true|
|stat-1x2-w32-accepted|1419.892|839.782|true|

|Profile|Samples|Balanced-namespace period|Qualified|Lost|
|---|---:|---:|---|---:|
|cycles-1x2-w16-OFF|8920|0|true|0|
|cycles-1x2-w16-ON|8571|5696000000|true|0|
|cycles-1x2-w16-accepted|8906|0|true|0|
|cycles-1x2-w32-OFF|12624|0|true|0|
|cycles-1x2-w32-ON|11797|6480000000|true|0|
|cycles-1x2-w32-accepted|12574|0|true|0|

## CPU and Metal reference comparisons

M5 timings are supplied historical references, not new matched-hardware Apple measurements. ON remains experimental. There is no M5reference for54→1.

|Circuit|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× achieved|
|---|---:|---:|---:|---:|---:|---|
|12x2-w32|33.908155|284.0|8.376×|101.0|2.979×|false|
|16x2-w32|40.052991|335.0|8.364×|124.0|3.096×|false|
|1x2-w32|8.013709|37.0|4.617×|24.0|2.995×|false|
|24x2-w32|65.972846|528.0|8.003×|176.0|2.668×|false|
|2x2-w32|11.225406|59.0|5.256×|30.0|2.673×|false|
|4x2-w32|15.945477|109.0|6.836×|48.0|3.010×|false|
|8x2-w32|24.096192|189.0|7.844×|74.0|3.071×|false|
|merge-24x1-w32|65.075009|507.0|7.791×|175.0|2.689×|false|

## Verification and retained records

All1974steps complete. All55,788SDKrequests verify internally, including primes. Each process exports its last timed proof after the clock/FIFO gate. All972fresh164-byte exports independently pass Gnark with public/image bindings, common per-shape verification-key identities and uniqueness against23,971prior proof hashes. The persisted union is24,943; proof/public archives retain1944members. Repeated verification is not counted as fresh proof output. Source/receipt/proof/symbol archives round-trip byte-exactly; original source/helper/binary/input pins close. Raw perf and ELF files stay local with hashes.

The accepted original nine local/AWS rows remain in [accepted-rows.json](accepted-rows.json), copied without decimal reserialization from the pre-capture summary. Accepted local primary remains8.022165ms/134MiB and54→1remains162.250464ms/1831.87MiB. Latest AWSprimary remains Zopro11.915591ms at32workers versus pinned AeglosSorted7.959184ms at96; no new AWSmeasurement. These local results do not establish phone/ARM/TEE, forester250/4999, memory pressure/swap, minimum RAM or concurrent proof throughput.

A separate [offline artifact review](artifacts/offline-review.json) verifies all 718 current pins, all 175 production files, reversal of both candidate parent edits, every archived member and all 972 proof/verification receipts. It independently recomputes the 30 paired comparisons and two aggregates from retained observations, reproducing the three failed checks. [Reviewer source](artifacts/offline-review.rs) and [review notes](artifacts/review-notes.md) are retained. This review executes no benchmark, proof or verifier again.

[Analysis](artifacts/analysis.json), [observations](artifacts/rows.json), [source review](artifacts/final-source-review.json), [design](artifacts/design.md), [diagnostic policy](artifacts/analysis-diagnostics.md), [capture plan](artifacts/plan.json), [pins](artifacts/pins.json), [sources](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proof bytes](artifacts/proofs.json.gz), [proof inventory](artifacts/proof-inventory.json), [new union ledger](artifacts/proof-sha256-after.json), [symbols](artifacts/profile-symbols.json.gz), [inherited summary](artifacts/inherited-summary.json.gz), [archive checks](artifacts/publication-review.json).
