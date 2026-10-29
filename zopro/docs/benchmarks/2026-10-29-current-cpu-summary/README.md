# Current CPU prover benchmarks

29 October 2026

The accepted Zopro build measures **8.022 ms locally for 1×2**. On the latest AWS c7a.24xlarge comparison, Zopro measures **11.916 ms versus 7.959 ms for pinned Aeglos Sorted**, using each engine's fastest tested worker count. Zopro has lower best-count latency on six of nine AWS circuits and lower observed peak process RSS on all nine. The supplied M5 CPU 10× target remains **unmet on all eight reference shapes**.

Production remains the accepted 175-file implementation. Recent PGO, G2 scheduling, arithmetic, grouping and GLV K/H experiments have not replaced it.

## Measurement scope

Local measurements use a Ryzen 9 9950X with 16 physical cores, 32 logical CPUs and 128 GiB RAM; the accepted table uses 32 workers. AWS measurements use one c7a.24xlarge with 96 physical cores and no SMT, testing 32, 64 and 96 workers. AWS runs use a network-disabled container with a 96-CPU quota, 30 GiB memory limit and no swap. That container limit is a benchmark setting, not a prover architecture cap.

Both tables report medians of 15 process medians after three priming proofs. Local primary processes measure 151 requests and other shapes 31; AWS primary processes measure 101 and other shapes 17. Timers include fresh witness solving, fresh randomized proving, SDK verification and serialization. Loading and preparation are outside latency and inside lifetime process RSS. RSS is the maximum process high-water mark across the corresponding blocks, in MiB; it is not minimum RAM or incremental proof workspace.

The AWS Zopro builds use the exact accepted source compiled for Zen 4; the local executable targets the Ryzen machine. These are different executables. Aeglos uses pinned commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, Sorted engine, with prepared-key cache hits checked. The comparison does not claim to represent every upstream configuration.

## Accepted local results and M5 references

M5 CPU and Metal numbers are supplied historical references, not new Apple measurements. Speedups are reference time divided by Zopro time. The final column gives the latency required for a CPU 10× result.

|Circuit|Zopro ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022|134.00|37|4.612×|24|2.992×|3.7|
|2x2|11.336|275.00|59|5.205×|30|2.646×|5.9|
|4x2|15.946|315.00|109|6.836×|48|3.010×|10.9|
|8x2|24.090|512.50|189|7.845×|74|3.072×|18.9|
|12x2|33.845|707.25|284|8.391×|101|2.984×|28.4|
|16x2|39.932|826.25|335|8.389×|124|3.105×|33.5|
|24x2|66.233|1328.75|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273|1175.50|507|7.767×|175|2.681×|50.7|
|54→1|162.250|1831.87|—|—|—|—|—|

No M5 reference was supplied for 54→1. Reaching 3.7 ms for local 1×2 requires approximately **54% lower latency** than the accepted 8.022 ms result.

The accepted build's full performance gate remains **false** because the physical-16 fresh-control/accepted calibration interval exceeded its declared range. Adoption was an explicit engineering exception; all candidate gain/nonregression checks and the separately declared final-artifact gate passed. This exception is retained here, and is not transferred to later experiments. [Adoption decision](../2026-10-29-balanced-kh-adoption/README.md), [complete final-artifact measurements](../2026-10-29-balanced-kh-production/README.md).

## Latest AWS comparison with Aeglos

Each engine's displayed value is its fastest median among 32, 64 and 96 workers. These selections describe this matrix; they are not a paired comparison across worker counts or a universal worker policy. Peak RSS comes from the selected worker count. The full report retains every same-worker comparison and its paired bounds.

|Circuit|Zopro ms|Workers|Aeglos ms|Workers|Zopro peak RSS MiB|Aeglos peak RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2|11.916|32|7.959|96|120.97|480.14|
|2x2|15.895|32|11.822|96|230.36|811.11|
|4x2|19.993|32|18.492|96|276.21|1351.90|
|8x2|26.898|64|31.531|96|450.82|2491.09|
|12x2|34.862|64|48.580|96|648.76|3714.61|
|16x2|38.983|64|57.136|96|766.20|4612.36|
|24x2|54.034|64|86.667|96|1225.00|6758.14|
|merge-24x1|57.133|64|84.350|96|1109.38|6608.05|
|merge-54x1|122.397|64|173.236|96|1877.03|13115.29|

Aeglos leads 1×2, 2×2 and 4×2; Zopro leads the remaining six circuits in this descriptive selection. For 54→1, Zopro measures 122.397 ms with 1.83 GiB peak RSS, versus 173.236 ms with 12.81 GiB for Aeglos. These memory observations do not establish why the latency differs. [Full AWS matrix, paired comparisons and raw artifacts](../2026-10-29-proof-pgo-c7a175/README.md).

The corresponding Zopro speedups against the same supplied M5 references are:

|Circuit|AWS Zopro ms|M5 CPU speedup|M5 Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|11.916|3.105×|2.014×|3.7|
|2x2|15.895|3.712×|1.887×|5.9|
|4x2|19.993|5.452×|2.401×|10.9|
|8x2|26.898|7.027×|2.751×|18.9|
|12x2|34.862|8.146×|2.897×|28.4|
|16x2|38.983|8.593×|3.181×|33.5|
|24x2|54.034|9.772×|3.257×|52.8|
|merge-24x1|57.133|8.874×|3.063×|50.7|

## CPU scaling evidence

The AWS 1×2 worker sweep shows different scaling behavior:

|Workers|Zopro latency ms|Aeglos latency ms|Zopro CPU ms per proof|Aeglos CPU ms per proof|
|---|---:|---:|---:|---:|
|32|11.916|11.868|256.128|328.248|
|64|11.995|8.774|420.399|430.446|
|96|13.778|7.959|704.247|513.394|

Zopro's sampled epoch/scheduler residence rises from 14.09% at 32 workers to 61.48% at 96. Those flat CPU sample categories are not removable wall-time fractions. The evidence directs investigation toward useful work availability, critical paths and coordination, but does not isolate memory capacity, cache misses, clock behavior, syscalls or fences as the cause.

The container recorded zero memory/swap/OOM event deltas. It also recorded 39 CPU-throttled periods over the entire workflow, including preparation, verification and profiling; that is not a timed-only or per-engine attribution. The measurements do not establish performance under memory pressure, minimum RAM, phone/ARM/TEE performance, concurrent throughput, or forester 250/4999 behavior.

## Latest cached hash grouping constructor

The revision retains one u64 hash per distinct class in a separate proof-local vector, so table growth avoids representative-input hashing. Hashes and the dictionary are released before scalar materialization; original output-phase memory bounds remain. Full-value equality, stable class order and the original sample/cutoff policy remain unchanged.

The newly declared constructor gate **passes**. Nine native component tests cover whole outputs, collisions, original allocation forecasts and exactly one hash per nonzero input through forced growth. Thirty-one fixed blocks retain all 18 real A/B inputs against the source-extracted original and exact previous representative constructor; three zero-loss profiles follow latency. No proof timing or adoption follows from this constructor screen. [Full results](../2026-10-29-cached-hash-groups175/README.md).

|All 18 inputs versus|Ratio|Bounds|Wins / 31|
|---|---:|---|---:|
|original|0.948305731|[0.946040180, 0.952979918]|31|
|parent|0.949390753|[0.947929195, 0.951318569]|31|

## Previous representative grouping constructor

The new one-pass constructor retains compact representative-row/count records and row classes, copying full scalar values only after exact admission. It removes the previous candidate's second input hash pass, preserves query independence and adds no key table, execution plan or point copy.

The separately declared constructor gate **fails**. Thirty-one fixed blocks cover all 18 real A/B inputs against original and one-pass Swiss builders. Eight component tests pass, including full-output oracles and checked source-extracted allocation forecasts. Three gated profiles have zero lost records. No proof timing or adoption is inferred. [Full constructor report](../2026-10-29-representative-groups175/README.md).

|Constructor aggregate versus|Ratio|Bounds|Wins / 31|
|---|---:|---|---:|
|original|0.996738665|[0.989779084, 0.997448289]|31|
|swiss|0.989996984|[0.988998289, 0.990332830]|31|


## Previous two pass grouping constructor

The deferred builder first determines exact admission using a compact dictionary of borrowed row indices. Only admitted inputs allocate class/value/count arrays; a second pass converts the same dictionary to class indices in place. It preserves query independence and adds no key table, plan or point copy.

The separately declared constructor gate **fails**. Thirty-one fixed blocks compare source-extracted original, one-pass Swiss and deferred builders on all 18 real A/B inputs. Every result matches an independent ordered-map oracle; eight component tests and three zero-loss profiles pass. This is constructor evidence, with no new proof timing or adoption. [Full constructor results](../2026-10-29-deferred-groups175/README.md).

|Constructor aggregate versus|Ratio|Bounds|Wins / 31|
|---|---:|---|---:|
|original|1.208738792|[1.206586580, 1.209804937]|0|
|swiss|1.198512259|[1.197358081, 1.199062747]|0|


## Latest arithmetic screen

Direct signed CIOS differences improve the affine arithmetic expression by 7.38% against fresh ordinary code, with 31/31 wins and paired ratio 0.926221229 [0.925461631, 0.927912346]. The four-pattern aggregate improves 0.45%, also 31 wins, but the dependent chain regresses 2.89% and product-plus-scaling 3.81%. Three nonregression checks fail, so the declared generic component gate remains false.

The routine has no new heap buffer or key table. Its BigInt model passes 30,084 exact cases, and two native tests pass 172,544 chained Arkworks lane comparisons. All 372 latency processes and four gated profiles complete; no new SDK proof is measured. This is an arithmetic-expression gain, not faster proving. A separately declared actual-G2-batch qualification follows below, retaining all synthetic regressions and the failed original gate. [Full component report](../2026-10-29-signed-difference175/README.md).

## Latest complete G2 caller experiment

The exact signed-difference helper is now measured in complete owned and borrowed G2 prefix batches, including inversion, packing, coordinate output and scatter. Eighteen contexts cover 32/200/352 pairs at 1/16/32 workers; 199/351-pair tails receive correctness preflights. Fifteen fixed blocks compare frozen original libraries, fresh OFF and candidate ON, with no new table, point copy or timed allocation.

The separately declared caller gate **fails**. The earlier generic arithmetic gate remains failed, and production remains unchanged. No SDK proof timing or independent proof is added. All 930 workflow steps complete, including six gated profiles with CPU IDs/timestamps and zero lost records. [Full caller results and failed checks](../2026-10-29-signed-g2-caller175/README.md).

|Caller aggregate|Candidate/control ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|all18/on/frozen|0.996913176|[0.996031448, 1.000000895]|11|
|all18/on/off|0.997421680|[0.995443811, 0.999687934]|13|
|threaded12/on/frozen|1.002934622|[1.000923399, 1.005559305]|2|
|threaded12/on/off|1.001960588|[0.999557705, 1.005031514]|5|


## Latest cached hash SDK experiment

The constructor stores hashes only during proof-local table construction, then drops them before admitted scalar materialization. The complete SDK follows its passing all-18 constructor screen. Earlier uncached gates remain failed.

The new full-request gate **fails**. Fifteen fixed blocks retain all nine circuits at 32 workers against accepted and fresh OFF. The trial makes 20,904 SDK requests and 441 fresh independent Gnark exports, with nine zero-loss primary/54 profiles after latency. Production remains unchanged pending any required further qualification. [Full results and failed checks](../2026-10-29-cached-hash-sdk175/README.md).

|Circuit|Accepted ms|OFF ms|ON ms|Accepted peak RSS MiB|OFF peak RSS MiB|ON peak RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.056177|8.041961|8.073931|132.75|132.25|86.00|
|2x2|11.336486|11.300638|11.279108|275.50|272.00|221.50|
|4x2|16.165656|16.081457|16.028418|309.50|316.75|289.50|
|8x2|24.331289|24.162864|24.388408|498.00|509.50|493.50|
|12x2|33.908997|33.890713|33.551914|721.50|727.50|683.25|
|16x2|40.177051|40.055373|39.853234|849.75|841.25|808.00|
|24x2|66.390604|66.198222|65.311431|1304.75|1284.25|1226.75|
|merge-24x1|65.599179|65.608057|64.783803|1165.41|1154.45|1160.50|
|merge-54x1|162.838936|161.561478|158.333945|1871.30|1837.06|1802.11|

## Previous representative SDK experiment

One-pass representative grouping is now measured across all nine circuits in complete fresh SDK requests. Its original constructor gate remains failed. This separately declared protocol compares exact accepted, fresh OFF and ON in 15 rotated/reversed blocks at 32 workers, including whole-request latency and lifetime RSS.

The new full-request performance gate **fails**. Calibration intervals are retained as diagnostics under this prospective protocol; both candidate/control gain and nonregression comparisons remain required, together with relative RSS bounds. No production adoption follows from this stage alone. [Complete SDK report and failed checks](../2026-10-29-representative-sdk175/README.md).

|Circuit|Accepted ms|OFF ms|ON ms|ON peak RSS MiB|
|---|---:|---:|---:|---:|
|1x2|8.152022|8.026144|8.033347|83.75|
|2x2|11.394626|11.246055|11.204142|217.50|
|4x2|16.055527|15.896628|15.852074|290.75|
|8x2|24.230379|24.140120|24.083721|479.00|
|12x2|33.869702|33.798232|33.531005|657.00|
|16x2|40.583149|40.040498|39.670771|787.00|
|24x2|66.708818|66.117965|66.813065|1221.75|
|merge-24x1|65.574555|64.976160|65.470053|1135.04|
|merge-54x1|162.474643|162.128890|172.728370|1897.42|


## Previous GLV full proof experiment

Borrowed signed GLV inside the fused K/H stream is slower. It reuses the accepted signed split with ten balanced component windows and two partitions, transforms temporary high-component buckets, and borrows the original K/H point operands. Earlier separate-query G1 GLV trials remain negative; this trial tests a materially revised caller.

|Variant|1×2 ms|CPU ms per request|Peak RSS MiB|
|---|---:|---:|---:|
|accepted|8.140269|182.861|133.75|
|OFF|8.037214|182.087|135.00|
|ON|8.481222|193.316|137.00|

ON/accepted paired ratio is 1.042733048 [1.036482844, 1.045527578], with zero wins in 31 blocks and a 346.494 µs slowdown estimate. ON/OFF is 1.055244019 [1.051573654, 1.057443162], also zero wins. Fresh OFF/accepted calibration passes; OFF is 1.24% faster by its paired estimate, a control observation rather than a candidate gain. Both candidate gain checks fail. Production remains unchanged.

The new implementation has no persistent key table or transformed point copy, but adds scalar scratch, partition merges and bucket transforms. Fewer weighted reductions did not offset these costs in the full request. CPU time rises and observed peak RSS is slightly higher. The captures do not isolate each contribution. Native suites pass 124/127 core and eight prover tests OFF/ON; both ON profiles sample the new route, with zero lost records. [Full GLV K/H pilot](../2026-10-29-glv-kh-borrowed175/README.md).

## Previous shared classification experiment

Shared A/B witness classification reduces modeled hashing visits by about 40%, but the actual native caller is slower. It builds one proof-local dictionary over existing witness values, reconstructs both stable local groupings, and overlaps that work with product FFT preparation.

|Variant|1×2 ms|CPU ms per request|Peak RSS MiB|
|---|---:|---:|---:|
|accepted|8.129776|182.753|136.75|
|OFF|8.077206|181.573|135.25|
|ON|9.401192|189.313|99.25|

ON/accepted paired ratio is 1.159005632 [1.153296967, 1.159845819], with zero wins in 31 blocks and a 1.287 ms slowdown estimate. ON/OFF is 1.163638735 [1.159946487, 1.167238766], also zero wins. Fresh OFF/accepted calibration passes. The speed pilot fails, and production remains unchanged.

Observed maximum lifetime RSS falls from 136.75 MiB accepted to 99.25 MiB ON, 37.50 MiB or 27.42% lower in this capture. This memory observation does not establish minimum RAM or pressure behavior. Thirty-one fixed blocks use 151 requests after three primes. This fresh accepted 8.130 ms measurement is separate from the complete nine-shape qualification above. The change reaches the canonical-wire path used by 1×2 and 2×2; larger paths receive correctness preflights without a new latency qualification. [Full native caller pilot and profiles](../2026-10-29-shared-wire-sdk175/README.md).

## Previous grouped dictionary experiment

The index-only grouping candidate uses grouped hash-table probing and stores class indices instead of complete scalar keys. Its real-input constructor benchmark passes, but the full-proof pilot fails calibration and the gain check against accepted production. The fresh OFF control is 10.04% slower by its paired estimate.

|Variant|1×2 ms|CPU ms per request|Peak RSS MiB|
|---|---:|---:|---:|
|accepted|8.031194|182.108|133.50|
|OFF|8.822366|196.167|134.50|
|ON|8.011907|182.187|102.50|

ON/accepted paired ratio is 0.997222755 [0.995722920, 1.000449909], with 20/31 wins and a 22.313 µs saving estimate. Bounds include parity. ON/OFF is 0.907667496 [0.903037230, 0.909997208], with 31 wins, but OFF/accepted calibration fails. Production remains unchanged.

Observed ON peak RSS is 102.50 MiB versus 133.50 MiB accepted, 31 MiB or 23.22% lower in this primary capture. This memory observation is retained without claiming minimum RAM, pressure behavior or a universal reduction. Thirty-one fixed blocks use 151 requests after three primes. Their fresh accepted 8.031 ms is a separate capture from the earlier complete nine-shape 8.022 ms qualification above. [Full proof result](../2026-10-29-swiss-groups-sdk175/README.md), [constructor evidence](../2026-10-29-swiss-groups175/README.md).

## Diagnostics and implications

The earlier dictionary trial's control gap remains unresolved after an offline audit found identical selected G2/G1 kernel bytes. That earlier failure is retained. In contrast, fresh control calibration passes in the new shared-classification trial, whose candidate still loses every block. [Earlier control audit](../2026-10-29-swiss-control-audit175/README.md).

The [shared-wire screen](../2026-10-29-wire-union-census175/README.md) established exact metadata and fewer hashing visits across nine fixtures. The native trial demonstrates that this reduction is insufficient for a proof-speed gain. A global classification pass changes the dependency graph: both query MSMs await it and quotient preparation. Its latency regression rules out adoption of this implementation; the flat CPU profiles do not isolate the regression's cause. Further work should preserve query readiness and target arithmetic or data movement on the critical path.

Both grouping experiments retain lower observed primary RSS. A separately declared memory/latency qualification may investigate that tradeoff with a nonregression bound set before measurement, while preserving each original failed speed gate. Neither candidate has been adopted under such a tradeoff. No AWS measurement follows the matrix above.

## Earlier local full proof experiment

The early-X candidate moves an existing coordinate correction earlier in the batched affine addition. Its caller benchmark passes with a 0.320% aggregate gain versus fresh OFF, but this does not translate into a verified full-proof gain. Production remains unchanged.

The fixed 31-block primary comparison uses the same accepted executable as the local table, fresh OFF and candidate ON, with 151 requests per process after three primes. Its freshly measured accepted result is 8.040 ms; the 8.022 ms table above retains the earlier complete nine-shape qualification. These are separate captures.

|Variant|1×2 ms|CPU ms per request|Peak RSS MiB|
|---|---:|---:|---:|
|accepted|8.039747|182.204|134.75|
|OFF|8.083209|181.949|134.50|
|ON|8.067820|181.764|134.25|

|Paired comparison|Latency ratio|Bounds|Wins / 31|
|---|---:|---|---:|
|ON/OFF|0.999244496|[0.997223650, 1.002353813]|17|
|ON/accepted|1.004017497|[1.001188080, 1.007720711]|8|

The paired saving versus OFF is 6.021 µs with bounds spanning parity. Versus accepted, ON is 32.431 µs slower by the paired estimate. Both prospective gain checks fail; control calibration passes. Bounds have nominal 97.055% individual median coverage under independent/exchangeable blocks. The candidate is not adopted. [Full experiment and verification receipts](../2026-10-29-early-x-sdk175/README.md), [caller measurements](../2026-10-29-early-x175/README.md).

## Recent experiment decisions

|Experiment|Measured result|Decision|
|---|---|---|
|[Local proof-only PGO](../2026-10-29-proof-pgo-qualification175/README.md)|Primary pilot 0.828% faster versus accepted; newly measured seven-shape aggregate 0.771% faster. Full qualification failed physical-16 control calibration.|Not adopted; positive local estimates retained.|
|[Same PGO profile on AWS](../2026-10-29-proof-pgo-c7a175/README.md)|All-eight aggregate approximately flat at 32 workers, 5.704% slower at 64 and 4.997% slower at 96. Seventeen nonregression checks failed.|Not adopted.|
|[G2 partition readiness](../2026-10-29-g2-part-ready175/README.md)|Local 1×2 ON 8.066 ms, OFF 8.071 ms, accepted 7.991 ms. Paired ON/OFF interval spans parity; primary gain, 8×2 nonregression and all-eight gain checks failed.|Not adopted.|
|[Five-product Fq6 arithmetic](../2026-10-29-fq6-toom175/README.md)|Correct field results, but complete final exponentiation 0.625% slower versus fresh OFF. Component aggregate/gain checks failed.|No prover integration.|
|[Early-X caller and full proof](../2026-10-29-early-x-sdk175/README.md)|Caller aggregate 0.320% faster versus OFF; full proof ON 8.068 ms, OFF 8.083 ms, accepted 8.040 ms. Full-proof gain checks fail.|Not adopted; component gain retained.|
|[Index-only grouped dictionary](../2026-10-29-swiss-groups-sdk175/README.md)|Constructor aggregate 55.93% faster in its compiled context; full proof ON 8.012 ms versus accepted 8.031 ms, with uncertainty spanning parity. Peak RSS 102.50 versus 133.50 MiB.|Not adopted; accepted-relative gain and control calibration checks fail.|
|[Shared witness classification](../2026-10-29-shared-wire-sdk175/README.md)|ON 9.401 ms versus accepted 8.130 ms; paired regression 15.90%, zero wins. Peak RSS 99.25 versus 136.75 MiB. Control calibration passes.|Not adopted; both gain checks fail.|
|[Borrowed GLV fused K/H](../2026-10-29-glv-kh-borrowed175/README.md)|ON 8.481 ms versus accepted 8.140 ms; paired regression 4.27%, zero wins. CPU time 193.316 versus 182.861 ms/request; peak RSS 137.00 versus 133.75 MiB.|Not adopted; both gain checks fail and control calibration passes.|
|[Direct signed CIOS difference](../2026-10-29-signed-difference175/README.md)|Affine expression 7.38% faster, aggregate 0.45% faster; dependent chain and scaled-product regressions fail three nonregression checks.|Generic component gate fails; actual-G2-batch qualification selected separately.|

The current prover combines vector field arithmetic, batched affine MSM additions, borrowed point operands, scalar endomorphism decomposition and balanced fused K/H windows. These experiments add no adopted per-key lookup expansion. Further work should target the dominant MSM arithmetic and its critical path; the latest component gains have not established another full-proof improvement.

## Verification and reproducibility

The accepted final-artifact qualification passed 124 native core and eight prover tests, 101 portable core and eight prover tests, plus 28,000 SDK requests and 505 distinct independent Gnark proof exports. The latest AWS comparison adds 37,272 SDK requests and 1,248 distinct Gnark-verified exports across its three engines; all 1,221 cloud exports were also reverified after transport, without counting that repeat as new proofs. The G2 partition-readiness experiment adds 13,824 SDK requests and 246 distinct independent exports. Fq6 and early-X component work add no proofs. The early-X SDK experiment adds 15,954 requests and 126 distinct independent Gnark exports. The grouped-dictionary SDK pilot adds a further 15,954 requests and 126 independent exports. The shared-classification SDK pilot adds another 15,954 internally verified requests and 126 fresh independent exports. The GLV K/H SDK pilot adds a further 15,954 internally verified requests and 126 fresh independent exports. The new all-nine representative SDK qualification adds 20,802 internally verified requests and 438 fresh independent exports. The historical distinct-proof ledger is **22,144**.

All of the latest AWS benchmark resources were removed. Source reports retain exact compiler flags, binary identities, keys and fixture bindings, proof bytes, process blocks, raw profiles, verification receipts and failed checks. This summary consolidates completed runs; it does not introduce new timing or verification samples. Full precision and selected worker counts are in [data.json](data.json), with [source hashes](source-hashes.json).

## Frozen 54 to 1 CPU diagnostic

The exact accepted, OFF and ON SDK executables are profiled on 54→1 at 32 workers, with no source rebuild or new latency qualification. Each engine has one gated four-event PMU capture and one separate 199 Hz cycles profile, each covering 31 fresh requests after three primes. All six captures precede offline analysis, and all six exported proofs pass independent Gnark verification. The original all-nine candidate gate remains failed and production is unchanged. [Full diagnostic and counters](../2026-10-29-representative-54-diagnostic175/README.md).

|Executable|Retired instructions per proof|Cycles per proof|IPC|Profile samples|Lost records|
|---|---:|---:|---:|---:|---:|
|accepted|18014458225|21405284917|0.842|26551|0|
|OFF|18015861802|21289826939|0.846|26119|0|
|ON|18128207687|22184905006|0.817|27071|0|

These single captures locate sampled work; they do not supply paired speed bounds or assign exclusive wall time to flat symbols. Raw counters retain event running percentages and unavailable or multiplexed events. This diagnostic adds 204 SDK requests and six fresh independent exports, taking the distinct-proof ledger to **21,703**. The all-eight CPU 10× objective remains unmet; the accepted CPU and Metal comparisons above are unchanged.

## Cached packed X coordinates in complete G2 batches

The new native streaming path retains packed Ax/Bx in six-vector per-batch prefix records, eliminating reverse input-X loads/packing while preserving all field formulas. reserve_exact keeps typed capacity below the existing nine-vector workspace allowance. Actual cache bytes are reported rather than requiring them equal to the smaller control records. There is no key-sized cache.

The separately declared caller gate **fails**. Fifteen blocks retain all 18 owned/borrowed G2 contexts across 1/16/32 workers and 32/200/352 pairs, plus 199/351 tails in preflight. All outputs match generator oracles and all timed allocation counts are zero. This is a complete batch-wave measurement, not SDK latency. [Full caller results](../2026-10-29-g2-cached-x175/README.md).

|Aggregate|Ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|all18/on/frozen|1.017859445|[1.015400839, 1.018940953]|0|
|all18/on/off|1.017241273|[1.015096934, 1.019725284]|0|
|threaded12/on/frozen|1.018575126|[1.014903964, 1.020916122]|0|
|threaded12/on/off|1.017953293|[1.015005397, 1.020129986]|0|

The component adds no SDK requests, independent proof exports or adoption. The distinct-proof ledger remains 22,144, and the accepted CPU/Metal and AWS comparisons above are unchanged.
