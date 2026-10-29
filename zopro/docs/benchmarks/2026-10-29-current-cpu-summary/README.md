# Current CPU prover benchmarks

29 October 2026

The accepted Zopro build measures **8.022 ms locally for 1×2**. On the latest AWS c7a.24xlarge comparison, Zopro measures **11.916 ms versus 7.959 ms for pinned Aeglos Sorted**, using each engine's fastest tested worker count. Zopro has lower best-count latency on six of nine AWS circuits and lower observed peak process RSS on all nine. The supplied M5 CPU 10× target remains **unmet on all eight reference shapes**.

Production remains the accepted 175-file implementation. Recent PGO, G2 scheduling and Fq6 arithmetic experiments have not replaced it.

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

## Recent experiment decisions

|Experiment|Measured result|Decision|
|---|---|---|
|[Local proof-only PGO](../2026-10-29-proof-pgo-qualification175/README.md)|Primary pilot 0.828% faster versus accepted; newly measured seven-shape aggregate 0.771% faster. Full qualification failed physical-16 control calibration.|Not adopted; positive local estimates retained.|
|[Same PGO profile on AWS](../2026-10-29-proof-pgo-c7a175/README.md)|All-eight aggregate approximately flat at 32 workers, 5.704% slower at 64 and 4.997% slower at 96. Seventeen nonregression checks failed.|Not adopted.|
|[G2 partition readiness](../2026-10-29-g2-part-ready175/README.md)|Local 1×2 ON 8.066 ms, OFF 8.071 ms, accepted 7.991 ms. Paired ON/OFF interval spans parity; primary gain, 8×2 nonregression and all-eight gain checks failed.|Not adopted.|
|[Five-product Fq6 arithmetic](../2026-10-29-fq6-toom175/README.md)|Correct field results, but complete final exponentiation 0.625% slower versus fresh OFF. Component aggregate/gain checks failed.|No prover integration.|

The current prover combines vector field arithmetic, batched affine MSM additions, borrowed point operands, scalar endomorphism decomposition and balanced fused K/H windows. These experiments add no adopted per-key lookup expansion. Further work should target the dominant MSM arithmetic and its critical path; the latest verification-only arithmetic result does not support another proof-speed claim.

## Verification and reproducibility

The accepted final-artifact qualification passed 124 native core and eight prover tests, 101 portable core and eight prover tests, plus 28,000 SDK requests and 505 distinct independent Gnark proof exports. The latest AWS comparison adds 37,272 SDK requests and 1,248 distinct Gnark-verified exports across its three engines; all 1,221 cloud exports were also reverified after transport, without counting that repeat as new proofs. The G2 partition-readiness experiment adds 13,824 SDK requests and 246 distinct independent exports. Fq6 component work adds no proofs. The historical distinct-proof ledger is **20,755**.

All of the latest AWS benchmark resources were removed. Source reports retain exact compiler flags, binary identities, keys and fixture bindings, proof bytes, process blocks, raw profiles, verification receipts and failed checks. This summary consolidates completed runs; it does not introduce new timing or verification samples. Full precision and selected worker counts are in [data.json](data.json), with [source hashes](source-hashes.json).
