# Zopro CPU benchmark report

29 October 2026. Snapshot of completed measurements at repository commit `c03bd71091808c0c982fd6b379e1fb16fe222e73`.

The accepted Zopro build proves **1×2 in 8.022 ms locally**, with **134 MiB observed peak RSS**. On AWS c7a.24xlarge, its best tested 1×2 result is **11.916 ms**, compared with **7.959 ms for Aeglos Sorted**. Across the nine AWS circuits, Zopro has lower best-count latency on six and lower observed peak RSS on all nine. Aeglos leads the three smallest circuits.

The latest arithmetic experiment improves an affine expression by **7.38%**, but also regresses two other expressions. It has not established a full-proof improvement or changed production. The CPU 10× objective against the supplied M5 references remains unmet on all eight reference circuits.

## Measurement conditions

Local measurements use a Ryzen 9 9950X, 16 physical cores, 32 logical CPUs and 128 GiB RAM. The accepted local table uses 32 workers. AWS measurements use one c7a.24xlarge with 96 physical cores and no SMT, testing 32, 64 and 96 workers. The AWS container has a 96-CPU quota, a 30 GiB memory limit, no swap and disabled networking; this memory setting is not an architecture requirement.

The latency tables report medians of 15 process medians after three priming proofs. Local processes measure 151 requests for 1×2 and 31 for other circuits; AWS processes measure 101 and 17 respectively. Each timed request includes fresh witness solving, fresh randomized proving, SDK verification and serialization. Loading and preparation are outside latency but included in lifetime process RSS. Peak RSS is the maximum high-water mark across the corresponding processes, not minimum RAM or incremental workspace.

AWS Zopro uses the accepted 175-file source compiled for Zen 4; the local executable targets the Ryzen machine. Aeglos uses the Sorted engine at commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, with prepared-key cache hits checked. Results apply to these configurations.

## Accepted local results

The M5 CPU and Metal values are supplied historical references. They were not remeasured in these runs. Speedup is reference latency divided by Zopro latency.

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

There is no supplied M5 reference for 54→1. Local 1×2 needs approximately 54% lower latency to reach its 3.7 ms CPU target.

The accepted build's original full performance gate remains **false** because its physical-16 fresh-control calibration interval exceeded the declared range. Adoption used an explicit engineering exception; all candidate gain/nonregression checks and the separate final-artifact gate passed. Later experiments do not inherit that exception. [Adoption decision](../2026-10-29-balanced-kh-adoption/README.md), [final-artifact qualification](../2026-10-29-balanced-kh-production/README.md).

## AWS comparison with Aeglos

Each displayed latency is that engine's fastest median among 32, 64 and 96 workers. RSS comes from the selected count. This is a descriptive selection across the measured matrix; it does not prescribe a universal worker count. The underlying report retains every same-worker comparison and paired interval.

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

For 54→1, this is 122.397 ms and 1.83 GiB peak RSS for Zopro versus 173.236 ms and 12.81 GiB for Aeglos. Lower RSS alone does not explain latency differences. [Complete AWS measurements, configuration and artifacts](../2026-10-29-proof-pgo-c7a175/README.md).

## CPU scaling

The AWS 1×2 sweep shows the principal remaining gap:

|Workers|Zopro latency ms|Aeglos latency ms|Zopro CPU ms per proof|Aeglos CPU ms per proof|
|---|---:|---:|---:|---:|
|32|11.916|11.868|256.128|328.248|
|64|11.995|8.774|420.399|430.446|
|96|13.778|7.959|704.247|513.394|

At 32 workers the measured medians are close. Aeglos latency falls as workers increase, while Zopro consumes more CPU time without a latency improvement. Zopro's sampled epoch/scheduler residence rises from 14.09% at 32 workers to 61.48% at 96. Those flat CPU sample categories are not exclusive wall-time costs or directly removable fractions.

The measurements support investigating useful parallel work, critical paths and coordination. They do not establish RAM capacity as the sole bottleneck. The container records zero memory/swap/OOM event deltas. Its 39 CPU-throttled periods cover the entire workflow and cannot be attributed to one timed engine. The tables do not establish minimum RAM, behavior under memory pressure, mobile/ARM/TEE performance or concurrent proof throughput.

## Recent experiments

All results below remain experimental; the accepted implementation is unchanged. Full-proof comparisons use their own fresh accepted controls, so their medians should not be substituted into the earlier nine-circuit table.

|Experiment|Measured result|Decision|
|---|---|---|
|[Direct signed CIOS difference](../2026-10-29-signed-difference175/README.md)|Affine expression 7.38% faster versus fresh ordinary code, 31/31 wins; four-expression aggregate 0.45% faster. Dependent chain regresses 2.89%, scaled product 3.81%.|Generic component gate fails three nonregression checks. No SDK proof timing or adoption.|
|[Borrowed GLV fused K/H](../2026-10-29-glv-kh-borrowed175/README.md)|Full-proof candidate 8.481 ms versus accepted 8.140 ms; paired regression 4.27%, zero wins in 31 blocks. Peak RSS 137.00 versus 133.75 MiB.|Not adopted; both gain checks fail, calibration passes.|
|[Shared witness classification](../2026-10-29-shared-wire-sdk175/README.md)|Candidate 9.401 ms versus accepted 8.130 ms; paired regression 15.90%. Peak RSS 99.25 versus 136.75 MiB.|Not adopted for speed; lower observed RSS retained as a memory tradeoff lead.|
|[Index-only grouped dictionary](../2026-10-29-swiss-groups-sdk175/README.md)|Candidate 8.012 ms versus accepted 8.031 ms; paired gain interval spans parity. Peak RSS 102.50 versus 133.50 MiB.|Not adopted; accepted-relative gain and fresh-control calibration checks fail.|
|[Early-X arithmetic](../2026-10-29-early-x-sdk175/README.md)|Caller aggregate improves 0.320%; full proof is 8.068 ms versus accepted 8.040 ms.|Not adopted; full-proof gain checks fail.|
|[Proof PGO on AWS](../2026-10-29-proof-pgo-c7a175/README.md)|All-eight aggregate approximately flat at 32 workers, 5.704% slower at 64 and 4.997% slower at 96.|Not adopted; 17 nonregression checks fail.|

The signed-difference helper adds no heap buffer, persistent key table or point copy. Its BigInt model passes 30,084 exact cases, and two native tests pass 172,544 chained lane comparisons against Arkworks. The affine-expression paired candidate/fresh ratio is 0.926221229 with bounds [0.925461631, 0.927912346]. These expression measurements omit inversion, packing, bucket construction and scheduling, so their gain cannot be applied directly to proof latency.

## Conclusions and next experiment

Zopro's accepted approach combines vector field arithmetic, batched affine MSM additions, borrowed point operands, scalar endomorphism decomposition and balanced fused K/H windows. Its observed AWS memory advantage is useful, while Aeglos retains the small-circuit latency lead in the measured matrix. No recent experiment has established another full-proof improvement.

The next selected experiment integrates the exact signed-difference helper only into native IFMA8 G2 callers. It must measure complete owned and borrowed batches, including inversion, packing, output handling, tails and threaded execution, under a separately declared protocol. The earlier generic component gate remains failed. A successful caller result would still need full SDK latency, all-circuit nonregression, resource and portable qualification before adoption. The design constraint remains bounded temporary memory without large per-key lookup expansions.

## Verification and source records

The accepted final-artifact qualification passes 124 native core and eight prover tests, 101 portable core and eight prover tests, 28,000 SDK requests and 505 distinct independent Gnark proof exports. The latest AWS comparison adds 37,272 SDK requests and 1,248 distinct independent exports across its three engines. The latest arithmetic component adds no proofs. The historical distinct-proof ledger is **21,259**; this report adds no measurements or verification samples. All benchmark-owned resources from the latest AWS run were removed.

The [detailed current summary](../2026-10-29-current-cpu-summary/README.md) retains the wider experiment history. Its [full-precision data](../2026-10-29-current-cpu-summary/data.json) and [source hashes](../2026-10-29-current-cpu-summary/source-hashes.json), as stored at the snapshot commit above, identify the supporting records. Each linked experiment preserves its declared checks, failed checks, compiler flags, source/binary identities, process results and verification receipts.
