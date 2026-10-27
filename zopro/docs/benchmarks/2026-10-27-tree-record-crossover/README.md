# G2 tree-layer service crossover

Training threshold in actual ordinary pairs: **32**. Component advances: **true**. This is a serial arithmetic component, not whole-proof latency. No production dispatcher or accepted source changes; the all-eight CPU10× objective remains active and unmet.

Every selected heldout case passes against both original controls. Dense heldout layers at128/256/512/1024 ordinary pairs have prefix/A geometric ratios0.961672/0.954450/0.959927/0.937833, each with7/7 wins. At16and17 ordinary pairs, dense prefix layers are1.009860and1.018872 times originalA, with0/7 wins. These results support testing a structural choice between the kernels. They do not establish cache causality or a full-proof improvement.

The offline heldout rule has aggregate ratios0.989119/0.989031 againstA/B, with7/7 winning block aggregates. Those synthetic, equally weighted case aggregates include unselected ratios fixed at1 and omit runtime dispatch cost; they are not a predicted prover gain.

The [preceding full SDK study](../2026-10-27-g2-tree-records/README.md) retained24×2 and54→1 gains but failed its global gate. This experiment asks whether the original and prefix-record layer costs cross with ordinary-pair count, before proposing any structural dispatch. It does not reclassify that failed study or choose a circuit/core selector from its rows.

The previous candidate's arithmetic is factored through a compile-time boolean. One instantiation executes the original record body and one executes prefix/norm records. The default wrapper selects the same path as the prior candidate. Reversing the factor and native cfg hooks recovers that candidate; its existing audit recovers the accepted source. Field primitives, tail formulas, exceptional-pair handling, point order, reduction topology and scheduling remain unchanged. The new fixture module/reexport is native-cfg-only.

## Fixed component caller

CPU2, one serial thread, no Rayon pool. Requested pair counts1/4/7/8/9/15/16/17/32/64/128/256/512/1024, either one dense bucket or seven ragged mixed buckets with identity/doubling/opposite cases. Seeds0/1 use distinct generator offsets and strides. Actual ordinary-pair counts exclude exceptional pairs and are computed before timing. These are synthetic curve-valid fixtures, not captured key points or witness distributions.

All56 cases run seven paired blocks in one fresh process each, with identical original controlsA/B and prefix mode in rotated/reversed order. Three primes per stage;iterations=max(64,16384/requestedpairs). The same fixture, point addresses and scratch serve all modes. Reset occurs before each kernel timer; complete storage/count validation occurs after each stage. Per-call Instant bracketing adds observer cost, which is retained. Caller time includes reset/clock/loop work. Both record vectors are warmed and retained, so this caller does not measure per-mode peak memory savings. All JSON/output follows all21 stages.

|Split|Requested pairs|Mixed|Ordinary pairs|A kernel µs|B kernel µs|Prefix kernel µs|Prefix/A|Prefix/B|Wins/A|Wins/B|
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
|training|1|false|1|0.934|0.934|0.893|0.956383|0.956692|7|7|
|training|1|true|0|0.655|0.655|0.655|0.999529|0.999892|5|3|
|training|4|false|4|1.652|1.651|1.582|0.958053|0.958031|7|7|
|training|4|true|0|0.659|0.659|0.663|1.006821|1.005742|0|0|
|training|7|false|7|2.241|2.239|2.172|0.969437|0.970862|7|7|
|training|7|true|1|2.191|2.191|2.186|0.997592|0.996772|7|7|
|training|8|false|8|0.941|0.940|0.943|1.001925|1.002532|1|1|
|training|8|true|0|4.846|4.848|4.836|0.998115|0.997415|7|7|
|training|9|false|9|1.281|1.282|1.288|1.006051|1.005976|1|1|
|training|9|true|0|4.991|4.990|4.986|0.999311|0.999517|6|6|
|training|15|false|15|2.571|2.575|2.548|0.991958|0.990738|7|7|
|training|15|true|3|6.460|6.464|6.222|0.963118|0.963209|7|7|
|training|16|false|16|1.239|1.239|1.254|1.014180|1.014136|0|0|
|training|16|true|4|6.540|6.536|6.479|0.990652|0.990847|7|7|
|training|17|false|17|1.599|1.597|1.628|1.021425|1.023423|0|0|
|training|17|true|5|6.689|6.693|6.680|0.998739|0.998139|6|6|
|training|32|false|32|1.904|1.905|1.892|0.992529|0.991933|7|7|
|training|32|true|20|7.183|7.187|7.157|0.994868|0.994626|6|7|
|training|64|false|64|3.184|3.189|3.093|0.972187|0.972058|7|7|
|training|64|true|40|12.009|12.029|11.785|0.980521|0.979410|7|7|
|training|128|false|128|5.893|5.892|5.643|0.959269|0.958128|7|7|
|training|128|true|80|23.467|23.459|23.379|0.996912|0.997074|7|7|
|training|256|false|256|11.213|11.215|10.741|0.957922|0.956519|7|7|
|training|256|true|160|46.013|46.024|45.543|0.989851|0.989521|7|7|
|training|512|false|512|21.934|21.898|20.887|0.950965|0.952213|7|7|
|training|512|true|320|91.761|91.765|90.324|0.983421|0.983503|7|7|
|training|1024|false|1024|43.796|43.776|41.281|0.943035|0.944687|7|7|
|training|1024|true|640|183.162|183.105|180.893|0.988292|0.988296|7|7|
|heldout|1|false|1|0.924|0.924|0.871|0.943336|0.943362|7|7|
|heldout|1|true|0|0.652|0.652|0.654|1.002906|1.003258|0|0|
|heldout|4|false|4|1.610|1.612|1.624|1.009932|1.010157|1|1|
|heldout|4|true|0|0.659|0.659|0.660|0.998162|1.001721|3|1|
|heldout|7|false|7|2.222|2.222|2.169|0.975629|0.975483|7|7|
|heldout|7|true|1|2.174|2.174|2.168|0.997180|0.997226|7|7|
|heldout|8|false|8|0.928|0.928|0.930|1.003361|1.003041|0|1|
|heldout|8|true|0|4.784|4.786|4.776|0.998425|0.998431|6|6|
|heldout|9|false|9|1.271|1.270|1.265|0.998905|0.999516|5|5|
|heldout|9|true|0|4.961|4.961|4.946|0.996945|0.997007|7|7|
|heldout|15|false|15|2.582|2.579|2.580|0.998675|0.999264|4|3|
|heldout|15|true|3|6.237|6.238|6.182|0.991168|0.991181|7|7|
|heldout|16|false|16|1.245|1.244|1.257|1.009860|1.009178|0|0|
|heldout|16|true|4|6.422|6.421|6.411|0.998444|0.998615|7|6|
|heldout|17|false|17|1.607|1.605|1.637|1.018872|1.019343|0|0|
|heldout|17|true|5|6.653|6.654|6.655|0.999978|0.999829|4|5|
|heldout|32|false|32|1.866|1.867|1.847|0.992536|0.991815|6|7|
|heldout|32|true|20|7.162|7.156|7.141|0.997885|0.997275|6|6|
|heldout|64|false|64|3.223|3.229|3.114|0.964628|0.962348|7|7|
|heldout|64|true|40|11.975|11.980|11.827|0.987495|0.987391|7|7|
|heldout|128|false|128|5.874|5.860|5.642|0.961672|0.963073|7|7|
|heldout|128|true|80|23.391|23.388|23.078|0.986152|0.986200|7|7|
|heldout|256|false|256|11.226|11.223|10.758|0.954450|0.955956|7|7|
|heldout|256|true|160|46.665|46.692|45.603|0.976444|0.975801|7|7|
|heldout|512|false|512|21.956|21.942|21.028|0.959927|0.959415|7|7|
|heldout|512|true|320|91.907|91.923|91.061|0.991066|0.990046|7|7|
|heldout|1024|false|1024|43.814|43.837|41.169|0.937833|0.937185|7|7|
|heldout|1024|true|640|183.557|183.592|181.334|0.987378|0.987890|7|7|

Medians summarize stage averages; ratios pair blocks geometrically and need not equal ratios of displayed medians. All stage values, caller cost, process CPU/counters and retained record capacities are in [rows](artifacts/rows.json) and [analysis](artifacts/analysis.json). None is a predicted proof speedup.

## Training fit and heldout decision

Calibration requires every case's identical-control medianB/A within factor1.03 and geometricB/A within factor1.05. All blocks remain included; no individual block veto or minimum positive-gain floor. The frozen training algorithm selects the smallest threshold from8/16/32/64/128/256/512/1024 ordinary pairs for which both layouts are represented and every training case at or above it improves against both controls with at least4/7 wins. If none qualifies, no universal crossover is claimed.

Heldout data cannot change the fitted threshold. Every selected heldout case must improve against both controls in at least4/7 blocks and stay within2% against both. The offline rule's aggregate over all heldout cases must improve against both with at least4/7 winning block aggregates. Unselected cases use the corresponding original control ratio1 in that offline model; actual dispatch/branch cost is unmeasured. The rule is not implemented in the prover.

Calibration failures: `[]`.

Heldout/speed failures: `[]`.

[Training-only rule](artifacts/training-rule.json) retains all threshold candidates and training failures. All heldout outcomes and offline aggregates remain in analysis. There are no repeated captures, dropped cases, threshold reselections or partial promotions. A passing result supports only a subsequent distinct caller experiment; a failure does not erase individually positive measurements.

## Validation, profiles and scope

All112 native core tests, two raw tests, six cubic tests, generator and portable/no-IFMA consumer checks pass. Two new tests perform448 complete-storage comparisons across56 fixtures, two inverse modes and four mode orders, plus reset/address/geometry checks. Expected live outputs use independent complete scalar curve formulas; every physical point-storage slot is also checked against the original layer. The prior candidate's independent tree oracle is retained. Field source and accepted qualification are inherited unchanged.

Every one of1,180 stages passes complete storage/count checks and zero timed allocation calls. Process affinity and TID remain fixed. Usage counters cover the caller, not isolated field instructions. Whole-process RSS includes fixture/reference copies, both retained record buffers, allocator and executable mappings; it is not prover scratch, per-mode savings or minimum RAM.

All56 latency processes precede four separate warm-gated999Hz cycle profiles: dense seed1, requested16and1024, originalA/prefix. Profiles cover the complete caller, including resets and timing-loop work. Every capture has one own PID/TID and zero lost records. Sampled-cycle shares are descriptive and are not removable wall-time fractions.

|Profile|Samples|Lost|
|---|---:|---:|
|profile-p1024-control-a|357|0|
|profile-p1024-prefix|332|0|
|profile-p16-control-a|157|0|
|profile-p16-prefix|157|0|

Maximum whole-process RSS across these component processes: **4.00MiB**. This includes all diagnostic storage. Raw root-owned recordings remain local and hashed; decoded records/receipts are archived.

No helper, build, hash or archive overlaps CPU captures. Sources, libraries, executable, tools and protocol pins are checked before and after. **Zero SDK requests and zero Gnark exports** are generated; the proof ledger remains **7,560**. All170 production files and the exact accepted SDK are unchanged. No new M5 CPU/Metal, AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, pressure, concurrency or throughput qualification is claimed. The preceding full SDK report remains authoritative for those measured shape comparisons.

Component executable: `/tmp/zopro-tree-record-crossover/native/probe-sdk`; SHA-256 `08bed2e16676a2f0776191184f67dec9eab2cb572296f59c17206b6f9513e4d2`.

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [native tests](artifacts/native/core/tests.json), [affinity receipts](artifacts/affinity.json), [raw hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).
