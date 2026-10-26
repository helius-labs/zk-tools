# Preparation stages and pool handoffs

The diagnostic passes both declared observer-calibration checks on both measured shapes. No optimization is adopted, and production remains unchanged. This follows the rejected [preparation pool](../2026-10-26-prefix-pool/README.md), whose reduced summed CPU did not improve latency.

The rejected pool implementation is unchanged apart from bounded coarse tracing. Local Ryzen9950X,32workers/CPUs0–31,1×2 and8×2 only. Three rotated/reversed fresh-process blocks,31timed SDK requests after3primes. OFF has no active session; TRACE records the ordinary path; POOL records the eight-worker preparation route where eligible. No new hardware profiles are added: this experiment measures wall spans and calling-thread CPU, supplementing the previous profiles.

The32KiB recorder and per-round observation vectors add memory. Session reset/collection occur outside individual SDK timers, while span clocks and first-use thread-ID calls remain timed. Summed process CPU includes outside-timer collection; calling-thread stage CPU excludes other workers. Pool startup is primed, and the existing17MiB lifetime forecast/pressure limitations remain. RSS includes load/setup/primes and is not minimum RAM.

The8×2 control uses a different solver path and emits the common SDK/prover/external-entry/assembly spans, without the small-path preparation/worker spans. An original analyzer assumption incorrectly required those small-path spans on the control. The reader was corrected after preflight and before latency capture; the prover binary and all six preflight exports were retained unchanged. [Analyzer repair](artifacts/analyzer-repair.md) records the exact scope and preserved original reader. No SDK command was rerun.

## Findings and next action

The observed caller/worker handoffs are small: ordinary dispatch+return medians are about9µs, versus about17µs across the two pool handoffs. These independently summarized values are not a precise attribution of the request difference, but they do not support a millisecond-scale handoff explanation in this run.

The pool's solver span rises1.151→1.257ms and FFT0.605→0.724ms; conversion/gathers improve slightly. A [per-proof wall decomposition](artifacts/wall-decomposition.json) computes preparation-stage sums before taking process and block medians: recorded preparation work is1.960366ms ordinary versus2.119878ms pooled. External entry→query start is2.062590ms versus2.229965ms. Query join stays near5.70–5.74ms. These are observed local spans, not proof of a universal cause;8×2 is a calibration control, and96-coreC7a is unmeasured here.

The primary supplied CPU target is37/10=3.7ms for the entire proof. The current query phase alone takes about5.7ms. Keeping that phase unchanged cannot meet the target by removing preparation alone. The next optimization must materially shorten the query critical path, through less arithmetic or a demonstrably better execution strategy. Another blind pool-width sweep is not justified by this diagnostic. The previous uninstrumented candidate rejection stands.

## SDK latency and supplied references

Each displayed time is a median of three process medians. Supplied M5 references are historical, not same-host measurements.

|Shape|Mode|ms|Max RSS MiB|Summed CPU ms/request|M5 CPU/Metal ms|CPU/Metal speedup|
|---|---|---:|---:|---:|---:|---:|
|1x2|accepted|8.557506|104.50|196.816|37/24|4.32×/2.80×|
|1x2|off|8.528412|112.25|196.283|37/24|4.34×/2.81×|
|1x2|trace|8.604896|109.50|196.638|37/24|4.30×/2.79×|
|1x2|pool|8.662685|75.25|174.761|37/24|4.27×/2.77×|
|8x2|accepted|24.574447|499.50|681.040|189/74|7.69×/3.01×|
|8x2|off|24.120602|513.25|675.805|189/74|7.84×/3.07×|
|8x2|trace|24.706036|512.25|686.229|189/74|7.65×/3.00×|
|8x2|pool|24.354222|515.00|672.356|189/74|7.76×/3.04×|

## Primary wall spans

For every timed trace, first take the median within each process, then the median across its three blocks. Independent medians do not add to a median proof. Spans may nest or overlap; they are not exclusive CPU fractions or removable wall time.

|Stage|Ordinary TRACE ms|Preparation POOL ms|
|---|---:|---:|
|core.solve_wires|1.151361|1.257421|
|core.public_correction|0.001773|0.001583|
|core.canonical_wires|0.071084|0.050294|
|core.gather_queries|0.111470|0.074841|
|core.product_fft|0.604970|0.723875|
|core.query_join|5.703765|5.739964|
|core.A_query|5.647779|5.695109|
|core.KH_query|5.690380|5.700329|
|core.B2_query|5.685902|5.706049|
|core.encode_parts|0.005801|0.005450|
|core.assemble_randomized|0.140124|0.138661|
|prover.verify|0.482239|0.474916|

Caller start→worker start and worker end→caller end are directly checked nonnegative intervals. The ordinary entry includes initial routing overhead. Prefix/math submit spans enclose their worker bodies; these are handoff observations with recorder overhead, not isolated scheduler syscall costs.

|Route|Dispatch ms|Return ms|Worker body ms|Caller span ms|
|---|---:|---:|---:|---:|
|ordinary|0.004639|0.004087|7.866744|7.875100|
|prefix|0.004950|0.003216|2.120668|2.128594|
|math|0.005511|0.003136|5.746465|5.754551|

## Observer calibration

OFF/accepted and TRACE/OFF must each stay within factor1.05perblock and1.03pairedmedian. POOL/TRACE is descriptive and has no speed/adoption gate in this diagnostic.

|Case|Comparison|Block ratios|Paired median|Paired geometric mean|
|---|---|---|---:|---:|
|1x2-w32|off_accepted|1.042856 / 0.991409 / 1.000141|1.000141|1.011221|
|1x2-w32|trace_off|0.959167 / 1.014251 / 1.011211|1.011211|0.994551|
|1x2-w32|pool_trace|1.011349 / 1.006716 / 0.999424|1.006716|1.005818|
|8x2-w32|off_accepted|0.996045 / 1.021798 / 0.977428|0.996045|0.998258|
|8x2-w32|trace_off|1.029729 / 0.973908 / 1.024271|1.024271|1.008986|
|8x2-w32|pool_trace|0.980658 / 0.995881 / 1.040509|0.995881|1.005365|

Calibration failures:

- None.

All840SDK calls completed, with30distinct independently Gnark-verified exports/60proof-public members,164-byte length,fixture public hashes and freshness against5928prior identities. Next prior5958. There are372latency and4preflight trace snapshots; every trace checks span completion, ordering, required stages, pool route, nested same-thread bounds and caller-worker containment. Full116nativecore/raw2/cubic6/generated IFMA and4analyzer tests pass. Field math is unchanged and prior qualification is inherited. Two source-edit assertion repairs occurred before build/measurement and are retained in [repair note](artifacts/instrument-repair.md). All measurement/analysis/report tools compiled before capture; no successful capture is repeated or excluded.

This diagnostic does not replace the previous uninstrumented candidate qualification or establish a speedup. No AWS,Aeglos,phone,TEE,memory-pressure,concurrent-throughput or giant-circuit measurement is added. Two measured shapes do not qualify all nine; all-eight CPU10× remains active and unmet.

[Design](artifacts/design.md), [analysis with every trace](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json) and [artifact index](artifacts/artifact-index.json) retain the evidence. Binaries and large keys stay local, pinned by hashes.
