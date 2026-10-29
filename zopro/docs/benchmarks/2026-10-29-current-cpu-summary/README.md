# Current CPU prover benchmarks

29 October 2026

The accepted Zopro build measures **8.022 ms locally for 1×2**. On the latest AWS c7a.24xlarge comparison, Zopro measures **11.916 ms versus 7.959 ms for pinned Aeglos Sorted**, using each engine's fastest tested worker count. Zopro has lower best-count latency on six of nine AWS circuits and lower observed peak process RSS on all nine. The supplied M5 CPU 10× target remains **unmet on all eight reference shapes**.

Production remains the accepted 175-file implementation. Recent PGO, G2 scheduling, arithmetic and grouping experiments have not replaced it.

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

## Latest local full proof experiment

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

The current prover combines vector field arithmetic, batched affine MSM additions, borrowed point operands, scalar endomorphism decomposition and balanced fused K/H windows. These experiments add no adopted per-key lookup expansion. Further work should target the dominant MSM arithmetic and its critical path; the latest component gains have not established another full-proof improvement.

## Verification and reproducibility

The accepted final-artifact qualification passed 124 native core and eight prover tests, 101 portable core and eight prover tests, plus 28,000 SDK requests and 505 distinct independent Gnark proof exports. The latest AWS comparison adds 37,272 SDK requests and 1,248 distinct Gnark-verified exports across its three engines; all 1,221 cloud exports were also reverified after transport, without counting that repeat as new proofs. The G2 partition-readiness experiment adds 13,824 SDK requests and 246 distinct independent exports. Fq6 and early-X component work add no proofs. The early-X SDK experiment adds 15,954 requests and 126 distinct independent Gnark exports. The grouped-dictionary SDK pilot adds a further 15,954 requests and 126 independent exports. The shared-classification SDK pilot adds another 15,954 internally verified requests and 126 fresh independent exports. The historical distinct-proof ledger is **21,133**.

All of the latest AWS benchmark resources were removed. Source reports retain exact compiler flags, binary identities, keys and fixture bindings, proof bytes, process blocks, raw profiles, verification receipts and failed checks. This summary consolidates completed runs; it does not introduce new timing or verification samples. Full precision and selected worker counts are in [data.json](data.json), with [source hashes](source-hashes.json).
